use crate::authentication::BearerToken;

pub enum AccessToken {
    PersonalToken(BearerToken),
    ApiToken(BearerToken),
}
