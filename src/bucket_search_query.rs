// Searching query params

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use logos::Logos;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use url::Url;
use uuid::Uuid;

// https://bucketdrive.co/search/query=
//TODO: use logos for tokens.

#[derive(Logos, Debug)]
pub enum SearchTokenKey {
    #[token("bucket:")]
    #[token("/")]
    Bucket,
    #[token("user:")]
    #[token("@")]
    User,
    #[token("desc:")]
    #[token("description:")]
    #[token("!")]
    Description,
    #[token("tag:")]
    #[token("#")]
    Tag,
    #[regex(r#"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"#, |lex| uuid::Uuid::parse_str(lex.slice()).unwrap() )]
    UuidValue(Uuid),
    #[regex(r#"[a-zA-Z0-9]+"#, |lex| lex.slice().to_owned())]
    Word(String),
    #[regex(r#""[^"]*""#, |lex| lex.slice().to_owned())]
    MultiWord(String),
}

pub enum UuidStringUnion {
    String(String),
    Uuid(Uuid),
}

pub enum SearchTokensValue {
    // either name of user or bucket.
    User(UuidStringUnion),
    // either name of bucket or uuid of bucket.
    Bucket(UuidStringUnion),
    // Tag is a String only of one word.
    Tag(String),
    // description is a String of one or many words.
    Description(String),
}

pub fn parse_search(search: &str) -> Vec<SearchTokensValue> {
    let mut token_values = Vec::<SearchTokensValue>::new();
    let mut lexer = SearchTokenKey::lexer(search);
    while let Some(token) = lexer.next() {
        match token {
            Ok(SearchTokenKey::Bucket) => {
                if let Some(token_val) = lexer.next() {
                    match token_val {
                        Ok(SearchTokenKey::UuidValue(id)) => {
                            token_values.push(SearchTokensValue::Bucket(UuidStringUnion::Uuid(id)));
                        }
                        Ok(SearchTokenKey::Word(word)) => {
                            token_values.push(SearchTokensValue::Bucket(UuidStringUnion::String(word.to_string())));
                        }
                        _ => {}
                    }
                }
            }
            Ok(SearchTokenKey::User) => {
                if let Some(token_val) = lexer.next() {
                    match token_val {
                        Ok(SearchTokenKey::UuidValue(id)) => {
                            token_values.push(SearchTokensValue::User(UuidStringUnion::Uuid(id)));
                        }
                        Ok(SearchTokenKey::Word(word)) => {
                            token_values.push(SearchTokensValue::User(UuidStringUnion::String(word.to_string())));
                        }
                        _ => {}
                    }
                }
            }
            Ok(SearchTokenKey::Description) => {
                if let Some(token_val) = lexer.next() {
                    match token_val {
                        Ok(SearchTokenKey::MultiWord(multi_word)) => {
                            token_values.push(SearchTokensValue::Description(multi_word.to_string()));
                        }
                        Ok(SearchTokenKey::Word(word)) => {
                            token_values.push(SearchTokensValue::Description(word.to_string()));
                        }
                        _ => {}
                    }
                }
            }
            Ok(SearchTokenKey::Tag) => {
                if let Some(token_val) = lexer.next() {
                    match token_val {
                        Ok(SearchTokenKey::Word(word)) => {
                            token_values.push(SearchTokensValue::Tag(word.to_string()));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    token_values
}

pub enum SearchValue {}

#[derive(EnumIter, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Clone, Hash)]
pub enum SearchFlags {
    //#[strum(serialize = "/", serialize = "bucket_id", serialize = "id")]
    BucketId(uuid::Uuid),
    //#[strum(serialize = ">", serialize = "user_id", serialize = "user")]
    UserId(uuid::Uuid),
    //#[strum(serialize = "description", serialize = "desc")]
    Description(String),
    //#[strum(serialize = "#", serialize = "tag")]
    Tag(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseSearchFlagError {
    #[error("Search flag must a valid key:value")]
    InvalidSearchFlagFormat,
    #[error("Search flag key unknown")]
    UnknownSearchFlagKey,
    #[error(transparent)]
    InvalidUuidValue(#[from] uuid::Error),
}

impl TryFrom<&str> for SearchFlags {
    type Error = ParseSearchFlagError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((arg, val)) = value.split_once(':') {
            match arg.to_lowercase().as_str() {
                "/" | "bucket_id" | "id" => Ok(Self::BucketId(uuid::Uuid::try_parse(val)?)),
                ">" | "user_id" | "user" => Ok(Self::UserId(uuid::Uuid::try_parse(val)?)),
                "desc" | "description" => Ok(Self::Description(val.to_string())),
                "#" | "tag" => Ok(Self::Tag(val.to_string())),
                _ => Err(ParseSearchFlagError::UnknownSearchFlagKey),
            }
        } else {
            Err(ParseSearchFlagError::InvalidSearchFlagFormat)
        }
    }
}

impl fmt::Display for SearchFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SearchFlags::BucketId(id) => {
                write!(f, "bucket_id:{id}")
            }
            SearchFlags::UserId(id) => {
                write!(f, "user_id:{id}")
            }
            SearchFlags::Description(desc) => {
                write!(f, "description:{desc}")
            }
            SearchFlags::Tag(tag) => {
                write!(f, "tag:{tag}")
            }
        }
    }
}

pub struct BucketSearchQuery {
    pub query: String,
    pub flags: Vec<SearchFlags>,
}

#[derive(thiserror::Error, Debug)]
pub enum BucketSearchInputQueryParsingError {
    #[error("Empty")]
    Empty,
    #[error("TooManyBucketIdFlags")]
    TooManyBucketIdFlags,
    #[error("TooManyUserIdFlags")]
    TooManyUserIdFlags,
    #[error("TooManyDescriptionFlags")]
    TooManyDescriptionFlags,
}

impl FromStr for BucketSearchQuery {
    type Err = BucketSearchInputQueryParsingError;
    // Note this function is for user input.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut query = s.to_string();
        let fragments: Vec<&str> = s.split(';').collect();

        if fragments.len() == 0 {
            return Ok(BucketSearchQuery {
                query: s.to_string(),
                flags: Vec::new(),
            });
        }

        let mut flags: Vec<SearchFlags> = vec![];
        let mut counts: HashMap<SearchFlags, i32> = HashMap::new();
        for fragment in fragments {
            match SearchFlags::try_from(fragment) {
                Ok(x) => {
                    *counts.entry(x.clone()).or_insert(0) += 1;
                    if matches!(x, SearchFlags::BucketId(_) | SearchFlags::UserId(_))
                        && counts[&x] > 1
                    {
                        return Err(match x {
                            SearchFlags::BucketId(_) => {
                                BucketSearchInputQueryParsingError::TooManyBucketIdFlags
                            }
                            SearchFlags::UserId(_) => {
                                BucketSearchInputQueryParsingError::TooManyUserIdFlags
                            }
                            _ => unreachable!(),
                        });
                    }
                    flags.push(x);
                    query = query.replacen(fragment, "", 1); // Remove fragment from string.
                }
                Err(_) => continue,
            }
        }
        flags.sort();

        Ok(BucketSearchQuery { query, flags })
    }
}

impl TryFrom<url::Url> for BucketSearchQuery {
    type Error = ();

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let _domain = value.domain();

        let _path = value.path();

        todo!()
    }
}

impl BucketSearchQuery {
    // Convert struct into a search URL
    // Should be in the format of "bucketdrive.co/search/:user_id/:bucket_id/#desc=:desc#tag={:tag..}"
    pub fn to_search_url(&self) -> Url {
        let mut url = Url::parse("https://bucketdrive.co/search").unwrap();
        let mut path_segments: Vec<String> = Vec::new();
        let mut tag_vec: Vec<String> = Vec::new();
        let mut desc = String::new();

        for flag in &self.flags {
            match flag {
                SearchFlags::BucketId(bucket_id) => {
                    path_segments.push(format!("bucket_id/{}", bucket_id));
                }
                SearchFlags::UserId(user_id) => {
                    path_segments.push(format!("user_id/{}", user_id));
                }
                SearchFlags::Description(description) => {
                    desc = description.to_string();
                }
                SearchFlags::Tag(tag) => {
                    tag_vec.push(tag.to_string());
                }
            }
        }

        let tag_string = tag_vec.join(";");

        // Append path_segments to url
        for segment in path_segments {
            url.path_segments_mut().unwrap().push(&segment);
        }

        url.set_fragment(Some(&format!("{}#tag={}", desc, tag_string)));

        url
    }
}

#[cfg(test)]
mod tests {
    use crate::bucket_search_query::BucketSearchQuery;
    use crate::bucket_search_query::SearchFlags;
    use std::str::FromStr;
    use url::Url;
    use uuid::Uuid;
    #[test]
    fn parsing_user_input_test() {
        let test_input = "bucket_id:123e4567-e89b-12d3-a456-426614174000;user_id:123e4567-e89b-12d3-a456-426614174000;";
        let query = BucketSearchQuery::from_str(test_input).unwrap();
        assert_eq!(query.flags.len(), 2);
    }

    #[test]
    fn parsing_url_input_test() {
        let test_query = Url::parse("https://bucketdrive.co/123e4567-e89b-12d3-a456-426614174000/123e4567-e89b-12d3-a456-426614174000#desc=description#tag=tag1;tag2").unwrap();
        let query = BucketSearchQuery::try_from(test_query).unwrap();

        let expected_bucket_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
        let expected_user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
        assert_eq!(query.flags.len(), 4);
        assert!(matches!(query.flags[0], SearchFlags::BucketId(id) if id == expected_bucket_id));
        assert!(matches!(query.flags[1], SearchFlags::UserId(id) if id == expected_user_id));
        assert!(
            matches!(query.flags[2], SearchFlags::Description(ref desc) if desc == "description")
        );
        assert!(matches!(query.flags[3], SearchFlags::Tag(ref tag) if tag == "tag1,tag2"));
    }

    #[test]
    fn formatting_test() {
        let test_query = "bucket_id/123e4567-e89b-12d3-a456-426614174000:user_id/123e4567-e89b-12d3-a456-426614174000#desc=description#tag=tag1;tag2";
        let query = BucketSearchQuery::from_str(test_query).unwrap();
        let url = query.to_search_url();
        assert_eq!(url.as_str(), "https://bucketdrive.co/search/bucket_id/123e4567-e89b-12d3-a456-426614174000/user_id/123e4567-e89b-12d3-a456-426614174000#desc=description#tag=tag1;tag2");
    }
}
