use crate::token::bearer_token::BearerToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessToken {
    PersonalToken(BearerToken),
    ApiToken(BearerToken),
}
