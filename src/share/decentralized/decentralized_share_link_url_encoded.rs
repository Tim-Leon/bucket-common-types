use std::fmt::{Display, Formatter};

use crate::bucket::bucket_feature_flags::{BucketFeaturesFlags};
use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::bucket::encryption_scheme::BucketEncryptionScheme;
use crate::key::derived_key::DerivedKey;
use crate::region::DatacenterRegion;
use crate::share::fully_qualified_domain_name::FullyQualifiedDomainName;
use crate::share::versioning::SharingApiPath;
use crate::util::DOMAIN_NAME;
use base64::{engine::general_purpose, Engine};
use ed25519_compact::SecretKey;
use http::uri::Scheme;
use secrecy::ExposeSecret;
use sha3::Sha3_256;
use time::OffsetDateTime;

use super::decentralized_share_token::{
    DecentralizedSecretShareToken, DecentralizedSecretShareTokenError,
};
use super::decentralized_share_token_signature::{
    DecentralizedShareTokenSignature, DecentralizedShareTokenSignatureError,
};

// https:eu-central-1.1.bucketdrive.co/share/0#user_id#bucket_id#bucket_encryption#bucket_key#permission#expires#signature

// The Only difference between ShareLink and SecretShareLink is
// that SecretShareLink encode the key for decrypting the bucket in an url such as Aes256Gcm.
// And that SecretShareLink use
pub struct DecentralizedSecretShareLink {
    pub scheme: Scheme,                           
    pub region_cluster: Option<DatacenterRegion>, 
    pub fqdn: FullyQualifiedDomainName,           
    pub path: DecentralizedShareParams, // acctual params for the specific bucket, encoded is stored in fragment port of uri as to not leak it to the server.
    // Token, is not encoded into urls, not needed, signature is enough.
    pub token: DecentralizedSecretShareToken, // token of path
}

pub struct DecentralizedShareParams {
    // Depending on what encryption used, the bucket_key might be different.
    // Note that the encryption algorithm chosen should have built in integrity check such as AES256-GCM to be considered fully secure or need an external source of integrity check.
    // Only the official supported bucket encryption can be used on the website,
    // any encryption that fal under custom will only be supported by client
    // that has the implementation necessary.
    pub version: SharingApiPath,
    pub bucket_guid: BucketGuid,
    // Depending on what encryption used, the bucket_key might be different.
    // Note that the encryption algorithm chosen should have built in integrity check such as AES256-GCM to be considered fully secure or need an external source of integrity check.
    // Only the official supported bucket encryption can be used on the website,
    // any encryption that fal under custom will only be supported by client
    // that has the implementation necessary.
    pub bucket_encryption: BucketEncryptionScheme,
    // Currently we limit the key size to at most 4096-bit encryption keys.
    pub secrete_key: Option<DerivedKey>,
    /// The permission associated with the url.
    pub permission: BucketPermissionFlags,
    /// For how long the signature is going to be valid
    pub expires: OffsetDateTime,
    /// Signature of the token, will be set by the decentralzied secrete share link so don't worry about it.
    pub token_signature: Option<DecentralizedShareTokenSignature>,
}

impl Display for DecentralizedShareParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}/{}#{}",
            self.version,
            self.bucket_guid,
            general_purpose::URL_SAFE_NO_PAD.encode(self.expires.to_string()),
            general_purpose::URL_SAFE_NO_PAD.encode(self.permission.bits().to_be_bytes()),
        )?;

        match &self.token_signature {
            Some(token_signature) => {
                write!(
                    f,
                    "#{}",
                    general_purpose::URL_SAFE_NO_PAD.encode(token_signature.0)
                )?;
            }
            None => {}
        }
        match &self.secrete_key {
            Some(secrete_key) => write!(
                f,
                "#{}",
                general_purpose::URL_SAFE_NO_PAD.encode(secrete_key.key.expose_secret())
            )?,
            None => {}
        }
        Ok(())
    }
}


impl Display for DecentralizedSecretShareLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://", self.scheme)?;
        match &self.region_cluster {
            Some(region_cluster) => write!(f, "{}.", region_cluster)?,
            None => {}
        }

        write!(f, "{}{}", self.fqdn, self.path)
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

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SecretShareLinkVerifySignatureError {
    #[error("Invalid signature")]
    InvalidSignature(#[from] ed25519_compact::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DecentralizedSecreteShareLinkError {
    #[error(transparent)]
    DecentralizedSecretShareTokenError(#[from] DecentralizedSecretShareTokenError),
    #[error(transparent)]
    DecentralizedShareTokenSignatureError(#[from] DecentralizedShareTokenSignatureError),
}

impl DecentralizedSecretShareLink {
    const VERSION: SharingApiPath = SharingApiPath::V1;
    pub fn new<TKeyLength: generic_array::ArrayLength>(
        region_cluster: Option<DatacenterRegion>,
        mut path: DecentralizedShareParams,
        bucket_feature_flags: &BucketFeaturesFlags,
        secrete_signing_key: &SecretKey,
    ) -> Result<Self, DecentralizedSecreteShareLinkError> {
        let token = DecentralizedSecretShareToken::new::<Sha3_256, digest::typenum::U32>(
            &region_cluster,
            &path.bucket_guid,
            &path.secrete_key,
            &path.permission,
            &path.expires,
            bucket_feature_flags,
        )?;
        let token_signature =
            DecentralizedShareTokenSignature::new(&token, secrete_signing_key, &path.bucket_guid)?;
        path.token_signature = Some(token_signature);
        let subdomain = region_cluster.as_ref().map(|region_cluster| region_cluster.to_string().into_boxed_str());

        let fqdn = FullyQualifiedDomainName {
            subdomain,
            domain: Box::from(DOMAIN_NAME),
            top_level_domain: Box::from(".co"),
        };

        Ok(Self {
            scheme: Scheme::HTTPS,
            region_cluster,
            fqdn,
            path,
            token,
        })
    }

    pub fn get_token(&self) -> &DecentralizedSecretShareToken {
        &self.token
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
