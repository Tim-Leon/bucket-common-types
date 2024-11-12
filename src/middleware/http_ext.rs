use std::convert::Infallible;
use std::io::Bytes;
use http::HeaderName;
use mime::Mime;
use reqwest::RequestBuilder;
use url::Url;
use crate::Encoding;
use crate::middleware::{RequestBuilderAuthorizationMetadataExt, RequestBuilderContentEncodingMetadataExt, RequestBuilderContentTypeMetadataExt};
use crate::token::access_token::AccessToken;

impl RequestBuilderAuthorizationMetadataExt for RequestBuilder {
    type Error = Infallible;

    fn with_authorization_metadata(mut self, api_token: &AccessToken) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        Ok(self.header(
            Self::AUTHORIZATION_KEY,
            format!("Bearer {0}", api_token.to_string()).as_str(),
        ))
    }


}

impl RequestBuilderContentTypeMetadataExt for RequestBuilder {
    type Error = Infallible;

    fn with_content_type(mut self, content_type: &Mime) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        Ok(self.header(Self::CONTENT_TYPE_KEY.as_str(), content_type.to_string().as_str()))
    }


}

impl RequestBuilderContentEncodingMetadataExt for RequestBuilder {
    type Error = Infallible;

    fn with_content_encoding(mut self, content_encoding: &[Encoding]) -> Result<Self, Self::Error> {
        let encoding_str = content_encoding
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        Ok(self.header(Self::CONTENT_ENCODING_KEY.as_str(), encoding_str.as_str()))
    }

}



//impl HttpUploadClientExt for HttpClient {
//    type Error = Infallible;
//    type Request = ();
//
//    async fn put(&self, url: Url,body: &[u8], api_token: &ApiToken, content_type: Mime, content_encoding: Option<Encoding>) -> Result<(), Self::Error> {
//        let val = JsValue::from_str(std::str::from_utf8(
//            body,
//        ).unwrap());
//        let resp = gloo::net::http::Request::put(url.as_str()).with_content_type(content_type).with_content_encoding(&content_encoding).body(val).send().await.map_err(|e| Self::Error::HttpPutError(e))?;
//        Ok(())
//    }
//}
//
//
//
//impl HttpDownloadClientExt for HttpClient {
//    type Error = Infallible;
//    type Request = ();
//
//    async fn get(&self, url: Url, body: String) -> Result<(), Self::Error> {
//        let request_builder = RequestBuilder;
//        self.get((), &(), None)
//        let req = RequestBuilder::new();
//        let resp = gloo::net::http::Request::get(url.as_str()).send().await.map_err(|e| Self::Error::HttpGetError(e))?;
//        Ok(())
//    }
//}

#[derive(thiserror::Error, Debug)]
pub enum HttpUploadError {

}

impl HttpUploadClientExt for  HttpClient{
    type Error = HttpUploadError;
    type Request = ();

    async fn put(&self, url: Url, body: &[u8], api_token: &ApiToken, content_type: &Mime, content_encoding: Option<Encoding>) -> Result<(), Self::Error> {
        let mut rb = self.put(url).body(body).with_authorization_metadata(api_token).unwrap().with_content_type(&content_type).unwrap();
        rb = match content_encoding {
            None => { rb },
            Some(x) => { rb.with_content_encoding(&[x]).unwrap() }
        };
        rb.send().await.map_err(|e| Self::Error::HttpDownloadError(e))?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum HttpDownloadError {

}

impl HttpDownloadClientExt for HttpClient {
    type Error = HttpDownloadError;

    async fn get(&self, url: Url, api_token: &ApiToken, content_encoding: Option<Encoding>) -> Result<Bytes, Self::Error> {
        use RequestBuilderContentEncodingMetadataExt;
        let mut rb = self.get(url.as_str()).with_authorization_metadata(api_token).unwrap();
        rb  = match content_encoding    {
            None => { rb }
            Some(content_encoding) => {
                rb.with_content_encoding(&[content_encoding]).unwrap()
            }
        };
        let resp = rb.send().await.map_err(|e| Self::Error::HttpDownloadError(e))?;
        let binary = resp.bytes().await.unwrap();
        Ok(binary)
    }
}