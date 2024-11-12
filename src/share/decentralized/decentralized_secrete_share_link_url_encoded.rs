use core::slice::SlicePattern;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

use aes_gcm;
use aes_gcm::Aes256Gcm;
use base64::{Engine, engine::general_purpose};
use digest::generic_array::GenericArray;
use digest::OutputSizeUser;
use ed25519_compact::Noise;
use sha3::{Digest, Sha3_224};
use time::OffsetDateTime;
use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::encryption::{BucketEncryptionScheme, EncryptionAlgorithm};
use crate::key::derived_key::CryptoHashDerivedKeySha3_256;
use crate::region::RegionCluster;
use crate::share::versioning::SharingApiPath;
use crate::util::{DOMAIN_NAME, DOMAIN_URL, SECRET_SHARE_PATH_URL};

// https:eu-central-1.1.bucketdrive.co/share/0#user_id#bucket_id#bucket_encryption#bucket_key#permission#expires#signature


// The Only difference between ShareLink and SecretShareLink is
// that SecretShareLink encode the key for decrypting the bucket in an url such as Aes256Gcm.
// And that SecretShareLink use
#[derive(Debug,  Clone)]
pub struct DecentralizedSecretShareLink {
    pub version: SharingApiPath,
    pub region_cluster: Option<RegionCluster>,
    pub bucket_guid: BucketGuid,
    // Depending on what encryption used, the bucket_key might be different.
    // Note that the encryption algorithm chosen should have built in integrity check such as AES256-GCM to be considered fully secure or need an external source of integrity check.
    // Only the official supported bucket encryption can be used on the website,
    // any encryption that fal under custom will only be supported by client
    // that has the implementation necessary.
    pub bucket_encryption: BucketEncryptionScheme,
    // Currently we limit the key size to at most 4096-bit encryption keys.
    pub bucket_key: CryptoHashDerivedKeySha3_256,
    /// The permission associated with the url.
    pub permission: BucketPermissionFlags,
    /// For how long the signature is going to be valid
    pub expires: OffsetDateTime,
    pub signature: ed25519_compact::Signature, // The signature is stored in the link. This makes sure that the link is not tampered with.
}






// Hash the secret share link to get a unique identifier that is then signed with an ed22219 key to create the signature.
// Does not include the signature in the hash.
// https://github.com/RustCrypto/hashes
fn hash_secret_share_link<D: Digest + OutputSizeUser>(
    region_cluster: RegionCluster,
    user_id: uuid::Uuid,
    bucket_id: uuid::Uuid,
    bucket_key: aes_gcm::Key<Aes256Gcm>,
    permission: BucketPermissionFlags,
    expires: OffsetDateTime,
    output: &mut GenericArray<u8, <D as OutputSizeUser>::OutputSize>, //[u8;64],
) {
    let mut hasher = D::new();
    hasher.update(region_cluster.to_string());
    hasher.update(user_id.as_bytes());
    hasher.update(bucket_id.as_bytes());
    hasher.update(bucket_key.as_slice());
    hasher.update(permission.bits().to_be_bytes());
    hasher.update(bincode::serialize(&expires).unwrap());
    hasher.finalize_into(output)
}

impl Display for DecentralizedSecretShareLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}.{}/{}/{}#{}#{}#{}#{}",
            "https://",
            self.region_cluster.to_string(),
            DOMAIN_URL,
            SECRET_SHARE_PATH_URL,
            general_purpose::URL_SAFE_NO_PAD.encode(self.bucket_guid.as_slice()),
            self.bucket_key.as_slice(),
            general_purpose::URL_SAFE_NO_PAD.encode(self.permission.bits().to_be_bytes()),
            general_purpose::URL_SAFE_NO_PAD
                .encode(bincode::serialize(&self.expires).unwrap().as_slice()),
            general_purpose::URL_SAFE_NO_PAD.encode(self.signature.as_slice()),
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecretShareLinkParsingError {
    #[error("Invalid host")]
    InvalidHostDomain,
    #[error("Invalid version format")]
    InvalidVersionFormat,
    #[error(transparent)]
    Base64Decoding(#[from] base64::DecodeError),
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

impl TryFrom<url::Url> for DecentralizedSecretShareLink {
    type Error = SecretShareLinkParsingError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let domain = value.domain().ok_or(Self::Error::InvalidHostDomain)?;
        let subdomains = domain.split(".").collect::<Vec<&str>>();
        if subdomains.len() != 3 {
            return Err(Self::Error::InvalidHostDomain);
        }
        let (subdomain, domain_name, tld) = (
            subdomains[0],
            subdomains[1],
            subdomains[2],
        );
        if domain_name != DOMAIN_NAME {
            return Err(Self::Error::InvalidHostDomain);
        }

        let region_cluster = RegionCluster::from_str(subdomain).unwrap();

        let path = value.path();
        let parts = path.split('/').take(1).collect::<Vec<&str>>(); // First element should be empty.
        let user_id = parts[0].parse::<uuid::Uuid>().unwrap();
        let bucket_id = parts[1].parse::<uuid::Uuid>().unwrap();
        let fragments = parts[3].split('#').take(1).collect::<Vec<&str>>(); // Guessing first part is just the path.
        let bucket_key = *aes_gcm::Key::<Aes256Gcm>::from_slice(
            general_purpose::URL_SAFE_NO_PAD
                .decode(fragments[1].as_bytes())
                .unwrap()
                .as_slice(),
        );
        let permission = BucketPermissionFlags::from_bits(u32::from_be_bytes(
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(fragments[2].as_bytes())
                .unwrap()
                .try_into()
                .unwrap(),
        ))
            .unwrap();
        let has_expires_field = fragments.len() == 4;
        let expires: OffsetDateTime =  {
            bincode::deserialize(
                base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode(fragments[3])
                    .unwrap()
                    .as_slice(),
            ).unwrap()
        };
        let mut signature_index = 5;
        if !has_expires_field {
            signature_index -= 1;
        }
        let signature = ed25519_compact::Signature::from_slice(
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(fragments[signature_index])
                .unwrap()
                .as_slice(),
        )
            .unwrap();
        Ok(Self {
            version: 0,
            region_cluster,
            user_id,
            bucket_id,
            bucket_encryption: EncryptionAlgorithm::None,
            bucket_key,
            permission,
            expires,
            signature,
            bucket_guid: BucketGuid {},
        })
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SecretShareLinkVerifySignatureError {
    #[error("Invalid signature")]
    InvalidSignature(#[from] ed25519_compact::Error),
}

impl DecentralizedSecretShareLink {


    // Verify the signature against the signature file with special identifier.
    pub fn verify_signature(
        &self,
        public_signing_key: ed25519_compact::PublicKey,
    ) -> Result<(), SecretShareLinkVerifySignatureError> {
        let mut hash_output = GenericArray::default(); //[0; 64];
        self.compute_hash::<Sha3_224>(
            self.region_cluster,
            self.bucket_guid,
            self.bucket_key,
            self.permission,
            self.expires,
            &mut hash_output,
        );
        assert_eq!(hash_output.len(), 32);
        Ok(public_signing_key.verify(hash_output, &self.signature)?)
    }


    pub fn new(
        region_cluster: RegionCluster,
        bucket_guid: BucketGuid,
        bucket_key: aes_gcm::Key<Aes256Gcm>,
        permission: BucketPermissionFlags,
        expires: OffsetDateTime,
        secret_key: &ed25519_compact::SecretKey,
    ) -> Self {
        let hash_output = Self::hash::<Sha3_224>(
            region_cluster,
            bucket_guid.clone(),
            bucket_key,
            permission,
            expires,
        );

        let noise = Noise::from_slice(bucket_guid.as_slice()).unwrap(); // Do we even need it?
        let signature = secret_key.sign(hash_output, Some(noise));
        Self {
            version: 0,
            region_cluster,
            user_id,
            bucket_id,
            bucket_encryption: EncryptionAlgorithm::None,
            bucket_key,
            permission,
            expires,
            signature,
        }
    }
    // TODO: There is no way for the server to invalidate a secret share link.
    /*
    Generate a token that is used by the server to identify the link.
    */
    pub fn get_token(&self) -> [u8; 32] {
        let mut hash_output = GenericArray::default();
        let hash_output = self.compute_hash::<Sha3_224>(
            self.region_cluster,
            self.bucket_guid,
            self.bucket_key,
            self.permission,
            self.expires,
            &mut hash_output,
        );

        let mut output: [u8; 32] = [0; 32];
        output.clone_from_slice(&hash_output);
        output
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecretShareLinkFormatError {
    #[error(transparent)]
    SecretShareLinkFormatError(#[from] SecretShareLinkParsingError),
}

impl TryInto<url::Url> for DecentralizedSecretShareLink {
    type Error = SecretShareLinkFormatError;

    fn try_into(self) -> Result<url::Url, Self::Error> {
        let res: String = self.to_string();
        Ok(url::Url::parse(&res).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use rand::{random, Rng};

    use super::*;

    #[test]
    fn create_secret_share_link() {
        //Generate pseudo random bytes for the base64 conversion
        let bucket_key_bytes = rand::random::<[u8; 32]>();
        let bucket_key = aes_gcm::Key::<Aes256Gcm>::from_slice(&bucket_key_bytes);

        let permission = BucketSharePermissionFlags::VIEW;

        //Create a dummy secret key for the signing process
        let mut secret_key_bytes: [u8; 64] = [0; 64]; // Initialize with zeroes
        rand::thread_rng().fill(&mut secret_key_bytes);
        let secret_key = ed25519_compact::SecretKey::from_slice(&secret_key_bytes).unwrap();

        let region_cluster = RegionCluster::from_str("central-eu-1:1").unwrap();

        let ssl = SecretShareLink::new(
            region_cluster,
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            *bucket_key,
            permission,
            OffsetDateTime::now_utc(),
            &secret_key,
        );
        assert!(ssl.bucket_key != *aes_gcm::Key::<Aes256Gcm>::from_slice(&[0u8; 32]));
        assert!(ssl.permission == permission);
    }

    #[test]
    fn secret_share_link_to_and_from_url() {
        // Set up values for a SecretShareLink
        let user_id = uuid::Uuid::new_v4();
        let bucket_id = uuid::Uuid::new_v4();
        let bucket_key_bytes = [0u8; 32];
        let bucket_key = aes_gcm::Key::<Aes256Gcm>::from_slice(&bucket_key_bytes);
        let permission = BucketSharePermissionFlags::VIEW; //You need to replace ValorA
        let expires = OffsetDateTime::now_utc();
        let secret_key = ed25519_compact::SecretKey::from_slice(&[0u8; 64]).unwrap();
        let region_cluster = RegionCluster::from_str("eu-central-1-1").unwrap();

        // Create a SecretShareLink
        let original_link = SecretShareLink::new(
            region_cluster,
            user_id,
            bucket_id,
            *bucket_key,
            permission,
            expires,
            &secret_key,
        );

        // Convert it to a URL and back to a SecretShareLink
        let url: url::Url = original_link.clone().try_into().unwrap();
        let parsed_link: SecretShareLink = url.try_into().unwrap();

        // Assert that both links are equivalent
        assert_eq!(original_link.user_id, parsed_link.user_id);
        assert_eq!(original_link.bucket_id, parsed_link.bucket_id);
        assert_eq!(original_link.bucket_key, parsed_link.bucket_key);
        assert_eq!(original_link.permission, parsed_link.permission);
        assert_eq!(
            original_link.clone().expires.date(),
            parsed_link.expires.date()
        );
    }

    #[test]
    fn signature_verification() {
        let user_id = uuid::Uuid::new_v4();
        let bucket_id = uuid::Uuid::new_v4();
        // Create a SecretKey and corresponding PublicKey for the signing process
        let bytes = random::<[u8; 32]>();
        let key_pair = ed25519_compact::KeyPair::from_slice(&bytes).unwrap();

        let bucket_key_bytes = rand::random::<[u8; 32]>();
        let bucket_key = aes_gcm::Key::<Aes256Gcm>::from_slice(&bucket_key_bytes);
        let permission = BucketPermissionFlags::VIEW; //You need to replace ValorA
        let expires = OffsetDateTime::now_utc();
        let region_cluster = RegionCluster::from_str("").unwrap();
        let link = SecretShareLink::new(region_cluster,
                                        user_id,
                                        bucket_id,
                                        *bucket_key,
                                        permission,
                                        expires,
                                        &key_pair.sk,
        );

        assert_eq!(link.verify_signature(key_pair.pk), Ok(()));
    }
}
