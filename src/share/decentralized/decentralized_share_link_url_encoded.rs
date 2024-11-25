use std::fmt::{Display, Formatter};

use aes_gcm;
use base64::{Engine, engine::general_purpose};
use http::uri::Scheme;
use secrecy::ExposeSecret;
use time::OffsetDateTime;
use crate::bucket::bucket_feature_flags::{self, BucketFeaturesFlags};
use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::bucket::encryption::BucketEncryptionScheme;
use crate::key::derived_key::DerivedKey;
use crate::region::DatacenterRegion;
use crate::share::fully_qualified_domain_name::FullyQualifiedDomainName;
use crate::share::versioning::SharingApiPath;
use crate::util::DOMAIN_NAME;

use super::decentralized_share_token::{DecentralizedSecretShareToken, DecentralizedSecreteShareTokenError};
use super::decentralized_share_token_signature::{DecentralizedShareTokenSignature, DecentralizedShareTokenSignatureError};


// https:eu-central-1.1.bucketdrive.co/share/0#user_id#bucket_id#bucket_encryption#bucket_key#permission#expires#signature


// The Only difference between ShareLink and SecretShareLink is
// that SecretShareLink encode the key for decrypting the bucket in an url such as Aes256Gcm.
// And that SecretShareLink use
pub struct DecentralizedSecretShareLink {
    pub scheme: Scheme,
    pub region_cluster: Option<DatacenterRegion>,
    pub fqdn: FullyQualifiedDomainName,
    pub path: DecentralizedSecretesPath,
    // Token, is not encoded into urls, not needed, signature is enough.
    pub token: DecentralizedSecretShareToken,
}

pub struct DecentralizedSecretesPath {
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
    /// Signature of the token.
    pub token_signature: DecentralizedShareTokenSignature,
}


impl Display for DecentralizedSecretesPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "{}{}/{}#{}#{}", 
        self.version,
        self.bucket_guid, 
        general_purpose::URL_SAFE_NO_PAD.encode(&self.expires.to_string()), 
        general_purpose::URL_SAFE_NO_PAD.encode(&self.permission.bits().to_be_bytes()), 
        general_purpose::URL_SAFE_NO_PAD.encode(&self.token_signature.0)
        )?;

        match &self.secrete_key {
            Some(secrete_key) => {
                write!(f, "#{}", 
                general_purpose::URL_SAFE_NO_PAD.encode(secrete_key.key.expose_secret())
             )?
            },
            None => {},
        }
        Ok(())
    }
}


impl Display for DecentralizedSecretShareLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://",self.scheme)?;
        match &self.region_cluster {
            Some(region_cluster) => { write!(f, "{}.", region_cluster)? },
            None => {},
        }

        write!(
            f,
            "{}{}",
            self.fqdn,
            self.path
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

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SecretShareLinkVerifySignatureError {
    #[error("Invalid signature")]
    InvalidSignature(#[from] ed25519_compact::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DecentralizedSecreteShareLinkError {
    #[error(transparent)]
    DecentralizedSecreteShareTokenError(#[from] DecentralizedSecreteShareTokenError),
    #[error(transparent)]
    DecentralizedShareTokenSignatureError(#[from] DecentralizedShareTokenSignatureError),
}

impl DecentralizedSecretShareLink {
    const VERSION: SharingApiPath = SharingApiPath::V1;
    pub fn new<TKeyLength: generic_array::ArrayLength>(
        region_cluster: Option<DatacenterRegion>,
        path: DecentralizedSecretesPath,
        bucket_feature_flags: &BucketFeaturesFlags,
    ) -> Result<Self, DecentralizedSecreteShareLinkError> {
        
        let token = DecentralizedSecretShareToken::new::<TKeyLength>(&region_cluster, &path.bucket_guid, &path.secrete_key, &path.permission, &path.expires, &bucket_feature_flags)?;
        let token_signature = DecentralizedShareTokenSignature::new(&token, &path.secrete_key, &path.bucket_guid)?;
        
        let subdomain = match region_cluster {
            Some(region_cluster) => Some(region_cluster.to_string().into_boxed_str()),
            None => None,
        };

        let fqdn = FullyQualifiedDomainName {
            subdomain: subdomain,
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
