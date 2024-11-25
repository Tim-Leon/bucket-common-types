
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
use pkcs8::{ObjectIdentifier, PrivateKeyInfo};
use pkcs8::spki::AlgorithmIdentifier;
use super::memory::secure_generic_array::SecreteGenericArray;

pub struct MasterKey256 {
    pub key: SecreteGenericArray<u8, typenum::U32>,
}


impl MasterKey256 {

    fn new(argon2: &Argon2, password: impl AsRef<[u8]>, salt: SaltString) -> Result<Self, Infallible>
    where
        Self: Sized
    {
        let mut hasher = Self::CryptoHasher::new();
        hasher.update(salt);
        let mac = hasher.finalize();
        let password_hash = argon2
            .hash_password(password.as_ref(),  mac.as_slice())?;
        debug_assert_eq!(password_hash.hash.unwrap().as_bytes().len(), 32);
        let key = SecreteGenericArray::new(password_hash.hash.unwrap().as_bytes()); 
        Ok(MasterKey256 {
            key,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MasterKey256ParseError {

}

impl TryInto<pkcs8::PrivateKeyInfo<'_>> for MasterKey256 {
    type Error = Infallible;
    fn try_into(self) -> Result<pkcs8::PrivateKeyInfo<'static>, Self::Error> {
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

impl MasterKey256 {
    pub fn oid() -> Option<ObjectIdentifier> {
        // Example OID for the master key (custom or private)
        Some(
            ObjectIdentifier::new("1.3.6.1.4.1.99999.1.1").unwrap()
        )
    }
}