use http::uri::Scheme;
use url::Url;

use crate::{bucket, search::{lexer::token::SerachTerm, query::model::SearchQuery}, share::fully_qualified_domain_name::FullyQualifiedDomainName};

pub struct UrlEncoder {
    pub scheme: Scheme,
    pub fqd: FullyQualifiedDomainName,
}


impl UrlEncoder {
        // Should be in the format of "bucketdrive.co/search/:user_id/:bucket_id/#desc=:desc#tag={:tag..}"
    pub fn encode(query: &mut SearchQuery) -> Result<Url, url::ParseError> {
        let mut url = Url::parse("https://bucketdrive.co/search")?;
        query.sort_terms();
        // Build path segments
        let mut path_segments = Vec::new();
        let mut query_params = Vec::new();
        for term in &query.terms {
            match term {
                SerachTerm::Bucket(bucket_id) => {
                    match bucket_id {
                        crate::search::lexer::token::BucketIdentifier::Name(name) =>  {
                            query_params.push(format!("?bucket={}", name));                            
                        }
                        crate::search::lexer::token::BucketIdentifier::Uuid(uuid) =>  {
                            path_segments.push(format!("/{}", uuid));
                        },
                    }
                }
                SerachTerm::User(user_id) => {
                    match user_id {
                        crate::search::lexer::token::UserIdentifer::Name(name) =>  {
                            query_params.push(format!("?user={}", name));
                        }
                        crate::search::lexer::token::UserIdentifer::Uuid(uuid) =>  {
                            path_segments.push(format!("/{}", uuid));
                        },
                    }
                },
                SerachTerm::Tag(tag) => {
                    query_params.push(format!("?tag={}", tag.clone()));
                }
                SerachTerm::Description(desc) => {
                    query_params.push(format!("?desc={}", desc.clone()));
                }
            }
        }

        url.set_path(path_segments.join("/").as_str());
        url.set_query(Some(query_params.join("&").as_str()));
        

        Ok(url)
    }
}