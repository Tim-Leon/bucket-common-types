use rand::{CryptoRng, RngCore};

/// A secrete share link, storing the token.
pub struct SecretShareLinkToken {
    pub token: [u8; 32],
}

impl SecretShareLinkToken {
    pub fn new(token: [u8; 32]) -> Self {
        Self {
            token
        }
    }

    pub fn generate<TCSPRNG: RngCore + CryptoRng>(cspring :&mut TCSPRNG) -> Self {
        let mut token = [0u8; 32];
        cspring.fill_bytes(&mut token);
        Self {
            token,
        }
    }
}

impl TryInto<Url> for SecretShareLinkToken {
    type Error = ();

    fn try_into(self) -> Result<Url, Self::Error> {
        todo!()
    }
}