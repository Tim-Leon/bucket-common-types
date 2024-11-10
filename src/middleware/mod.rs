use std::fmt::Debug;
use mime::Mime;
use crate::Encoding;
use crate::token::access_token::AccessToken;
use crate::token::idempotency_token::IdempotencyToken;

pub mod grpc_ext;
pub mod http_ext;

pub const AUTHORIZATION_VALUE_MAX_LENGTH: usize = 120;
pub const CONTENT_TYPE_VALUE_MAX_LENGTH: usize = 12;
pub const CONTENT_ENCODING_VALUE_MAX_LENGTH: usize = 16;
pub const IDEMPOTENCY_TOKEN_VALUE_MAX_LENGTH: usize = 32;
pub const USER_AGENT_VALUE_MAX_LENGTH: usize = 16;
pub const RATE_LIMIT_VALUE_MAX_LENGTH: usize = 32;
pub const SIGNATURE_VALUE_MAX_LENGTH: usize = 2048;

/// Note this is for HTTP request
pub trait RequestBuilderAuthorizationMetadataExt {
    type Error: Debug;
    const AUTHORIZATION_KEY: &'static http::HeaderName = &http::header::AUTHORIZATION;
    const AUTHORIZATION_VALUE_MAX_LENGTH: usize = AUTHORIZATION_VALUE_MAX_LENGTH;
    fn with_authorization_metadata(self, api_token: &AccessToken) -> Result<Self, Self::Error> where Self: Sized;
}

pub trait RequestBuilderAuthorizationMetadataSetterExt {
    type Error: Debug;
    const AUTHORIZATION_KEY: &'static http::HeaderName = &http::header::AUTHORIZATION;

    const AUTHORIZATION_VALUE_MAX_LENGTH: usize = AUTHORIZATION_VALUE_MAX_LENGTH;
    fn set_authorization_metadata(&mut self, api_token: &AccessToken) -> Result<(), Self::Error>;
}

pub trait RequestBuilderContentTypeMetadataExt {
    type Error : Debug;
    /// Define the "content-type" key as a constant
    const CONTENT_TYPE_KEY: &'static http::HeaderName = &http::header::CONTENT_TYPE;
    fn with_content_type(self, content_type: &Mime) -> Result<Self, Self::Error> where Self: Sized;

}

pub trait RequestBuilderContentTypeMetadataSetterExt {
    type Error: Debug;
    const CONTENT_TYPE_KEY: &'static http::HeaderName = &http::header::CONTENT_TYPE;
    fn set_content_type(&mut self, content_type: &Mime) -> Result<(), Self::Error>;
}

pub trait RequestBuilderContentEncodingMetadataExt {
    type Error : Debug;
    const CONTENT_ENCODING_KEY: &'static http::HeaderName = &http::header::CONTENT_ENCODING;
    fn with_content_encoding(self, content_encoding: &[Encoding]) -> Result<Self, Self::Error> where Self: Sized;

}

pub trait RequestBuilderContentEncodingMetadataSetterExt {
    type Error : Debug;
    const CONTENT_ENCODING_KEY: &'static http::HeaderName = &http::header::CONTENT_ENCODING;
    fn set_content_encoding(&mut self, content_encoding: &[Encoding]) -> Result<(), Self::Error>;
}



pub trait RequestBuilderIdempotencyTokenMetadataExt {
    type Error : Debug;
    const IDEMPOTENCY_TOKEN_KEY: &'static http::HeaderName;
    fn with_idempotency_token(self, idempotency_token: &IdempotencyToken) -> Result<Self, Self::Error> where Self: Sized;

}

pub trait RequestBuilderIdempotencyTokenMetadataSetterExt {
    type Error : Debug;
    const IDEMPOTENCY_TOKEN_KEY:  &'static http::HeaderName;
    fn set_idempotency_token(&mut self, idempotency_token: &IdempotencyToken) -> Result<(), Self::Error>;
}

pub enum Signature {

}

pub trait RequestBuilderSignatureMetadataExt {
    type Error : Debug;
    const SIGNATURE_KEY: &'static http::HeaderName;
    fn with_signature(self, signature: &Signature) -> Result<Self, Self::Error> where Self: Sized;

}

pub trait RequestBuilderSignatureMetadataSetterExt {
    type Error : Debug;
    const SIGNATURE_KEY:  &'static http::HeaderName;
    fn set_signature(&mut self, signature: &Signature) -> Result<(), Self::Error>;
}


pub trait RequestBuilderWebhookIdMetadataSetterExt {
    t
}

pub trait ResponseRatelimitHeaderExtractorExt<T>  {
    type Error : Debug;
    type OutputType;
    const RATE_LIMIT_KEY: &'static http::HeaderName;
    fn extract(&self) -> Result<Self::OutputType, Self::Error>;
}

pub trait ResponseUserAgentHeaderExtractorExt<R> {
    type Error : Debug;
    type OutputType;
    const USER_AGENT_KEY: &'static http::HeaderName;
    fn extract(&self) -> Result<Self::OutputType, Self::Error>;
}
