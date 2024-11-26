use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum::Display;

pub struct Argon2IdParams {
    pub iterations: u32,
}

impl Default for Argon2IdParams {
    fn default() -> Self {
        todo!()
    }
}

pub struct PBKDF2Params {
    pub iterations: u32,
}

impl Default for PBKDF2Params {
    fn default() -> Self {
        todo!()
    }
}


#[derive(Display, Default, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum KeyDeriveFunction {
    /// Argon2Id the most secure KDF currently available.
    #[default]
    Argon2id,
    /// Uses less memory than Argon2Id, consider if you want to use Argon2Id
    PBKDF2
}