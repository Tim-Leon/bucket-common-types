
use argon2::password_hash::{Salt, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher};
use rand::CryptoRng;
use core::slice::SlicePattern;
//use hex_literal::hex;
use digest::generic_array::{ArrayLength, GenericArray};
use secrecy::ExposeSecret;
use sha3::{Digest, Sha3_256};
use std::convert::Infallible;
use digest::typenum;
use pkcs8::PrivateKeyInfo;
use pkcs8::spki::AlgorithmIdentifier;
use crate::key::{CryptoMasterKey, SecureGenericArray};

pub struct MasterKey256 {
    pub secrete: SecureGenericArray<u8, typenum::U32>,
}

impl SlicePattern for MasterKey256 {
    type Item = u8;

    fn as_slice(&self) -> &[Self::Item] {
        self.secrete.0.expose_secret().as_slice()
    }
}

impl CryptoMasterKey for MasterKey256 {
    type KeyLength = typenum::U32;
    type CryptoHasher = Sha3_256;

    fn generate<TCryptoRng: rand::CryptoRng + rand::RngCore>(csprng: &mut TCryptoRng) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        let mut secrete: [u8; 32] = [0; 32];
        csprng.fill_bytes(&mut secrete);
        Ok(Self {
            secrete: SecureGenericArray {
                0: GenericArray::from_slice(secrete.as_slice()).try_into()?,
            },
        })
    }

    fn new(argon2: &Argon2, password: impl AsRef<[u8]>, salt: SaltString) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        let mut hasher = Self::CryptoHasher::new();
        hasher.update(salt);
        let mac = hasher.finalize();
        let master_key = argon2
            .hash_password(password.as_ref(),  mac.as_slice())?;
        Ok(MasterKey256 {
            secrete: SecureGenericArray::from(GenericArray::from_slice(
                master_key.hash.unwrap().as_bytes(),
            ).try_into()?),
        })
    }
}



impl TryFrom<&PasswordHash<'_>> for MasterKey256 {
    type Error = Infallible;

    fn try_from(value: &PasswordHash) -> Result<Self, Self::Error> {
        let output = match value.hash {
            None => { return Err(()); }
            Some(x) => { x }
        };
        Ok(Self {
            secrete: SecureGenericArray::from(GenericArray::from_slice(output.as_bytes()).try_into()?),
        })
    }
}
impl<T, TArrayLength> From<SecureGenericArray<T, TArrayLength>> for MasterKey256 {
    fn from(value: SecureGenericArray<T, TArrayLength>) -> Self {
        Self {
            secrete: value,
        }
    }
}



impl TryInto<pkcs8::PrivateKeyInfo<'_>> for MasterKey256 {
    type Error = Infallible;
    fn try_into(self) -> Result<pkcs8::PrivateKeyInfo<'_>, Self::Error> {
        Ok(PrivateKeyInfo {
            algorithm: AlgorithmIdentifier {
                oid: (),
                parameters: None,
            },
            private_key: &self.secrete.0,
            public_key: None,
        })
    }
}

impl TryFrom<pkcs8::PrivateKeyInfo> for MasterKey256 {
    type Error = Infallible;
    fn try_from(value: pkcs8::PrivateKeyInfo) -> Result<Self, Self::Error> {
        value.algorithm.oid
        Ok(
            Self {
                secrete: SecureGenericArray::from(value.private_key),
            }
        )
    }
}


impl TryFrom<pkc1::key> for MasterKey256 {
    type Error = Infallible;
    fn try_from(value: pkc1::key) -> Result<Self, Self::Error> {

    }
}

#[cfg(test)]
mod tests {
    use crate::module::encryption::key::master_key::MasterKey256;
    use argon2::password_hash::SaltString;
    use argon2::Argon2;

    fn create_master_key_tests() {
        let argon2 = Argon2::default();
        let nonce = "";
        let password = "";
        let salt = SaltString::from();
        let master_key_from_plaintext = MasterKey256::new(&argon2, nonce, password, salt ).unwrap();
        let master_key_from_phc = MasterKey256::from_phc_string();
    }
}