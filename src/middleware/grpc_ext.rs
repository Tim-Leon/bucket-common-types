use std::convert::Infallible;
use std::str::FromStr;
use http::HeaderName;
use mime::Mime;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::{Request, Response};
use crate::account::authentication::token::ApiToken;
use crate::client::middleware::metadata::{IdempotencyToken, RequestBuilderAuthorizationMetadataExt, RequestBuilderContentTypeMetadataExt, RequestBuilderIdempotencyTokenMetadataSetterExt, ResponseRatelimitHeaderExtractorExt, ResponseUserAgentHeaderExtractorExt};
use crate::client::middleware::ratelimit::RatelimitMetadata;
use crate::client::middleware::user_agent::UserAgent;
use crate::token::access_token::AccessToken;
use crate::token::idempotency_token::IdempotencyToken;
use super::{RequestBuilderAuthorizationMetadataExt, RequestBuilderAuthorizationMetadataSetterExt, RequestBuilderContentTypeMetadataExt, RequestBuilderContentTypeMetadataSetterExt, RequestBuilderIdempotencyTokenMetadataSetterExt, ResponseRatelimitHeaderExtractorExt, ResponseUserAgentHeaderExtractorExt};


impl<T> RequestBuilderAuthorizationMetadataExt for Request<T> {
    type Error = Infallible;
    fn with_authorization_metadata(mut self, api_token: &AccessToken) -> Result< Self, Self::Error>
    where
        Self: Sized
    {
        let meta = self.metadata_mut();
        let mut token: String = "Bearer ".to_string();
        token.push_str(api_token.to_string().as_str());
        let meta_data = MetadataValue::<Ascii>::from_str(token.as_str()).unwrap();
        meta.append(Self::AUTHORIZATION_KEY, meta_data);
        Ok(self)
    }


}

impl <T> RequestBuilderAuthorizationMetadataSetterExt for Request<T> {
    type Error = Infallible;
    fn set_authorization_metadata(&mut self, api_token: &AccessToken) -> Result<(), Self::Error> {
        let meta = self.metadata_mut();
        let meta_data = MetadataValue::<Ascii>::from_str(api_token.to_bearer_token().as_str()).unwrap();
        meta.append(Self::AUTHORIZATION_KEY, meta_data);
        Ok(())
    }
}

impl<T> RequestBuilderContentTypeMetadataExt for Request<T> {
    type Error = Infallible;
    fn with_content_type(mut self, content_type: &Mime) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        let meta = self.metadata_mut();
        let meta_data = MetadataValue::<Ascii>::from_str(content_type.to_string().as_str()).unwrap();
        meta.append( Self::CONTENT_TYPE_KEY, meta_data);
        Ok(self)
    }


}

impl <T> RequestBuilderContentTypeMetadataSetterExt for Request<T> {
    type Error = Infallible;
    fn set_content_type(&mut self, content_type: &Mime) -> Result<(), Self::Error> {
        let metadata = MetadataValue::<Ascii>::from_str(content_type.to_string().as_str())?;
        self.metadata_mut().insert(Self::CONTENT_TYPE_KEY,metadata).unwrap();
        Ok(())
    }
}

impl <T> RequestBuilderIdempotencyTokenMetadataSetterExt for Request<T> {
    type Error = Infallible;
    const IDEMPOTENCY_TOKEN: &'static HeaderName = &();

    fn set_idempotency_token(&mut self, idempotency_token: &IdempotencyToken) -> Result<(), Self::Error> {
        todo!()
    }

}
pub mod IntoMetadataKey = tonic::metadata::map;
impl IntoMetadataKey for HeaderName {

}

impl <T> ResponseRatelimitHeaderExtractorExt<T> for Response<T> {
    type Error = Infallible;

    type OutputType = RatelimitMetadata;
    const RATE_LIMIT_KEY: HeaderName = ();

    fn extract(&self) -> Result<Self::OutputType, Self::Error> {
        Ok(self.metadata().get("ratelimit").unwrap().try_into().unwrap())
    }
}

impl <T> ResponseUserAgentHeaderExtractorExt<T> for Response<T> {
    type Error = Infallible;
    type OutputType = UserAgent;
    const USER_AGENT_KEY: HeaderName = http::header::USER_AGENT;

    fn extract(&self) -> Result<Self::OutputType, Self::Error> {
        Ok(self.metadata().get(Self::USER_AGENT_KEY.as_str()).unwrap().try_into().unwrap())
    }
}