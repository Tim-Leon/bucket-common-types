
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
//use hex_literal::hex;
use sha3::{Digest, Sha3_256};
use digest::typenum;
use pkcs8::ObjectIdentifier;
use super::memory::secure_generic_array::SecreteGenericArray;

pub struct MasterKey256 {
    pub key: SecreteGenericArray<u8, typenum::U32>,
}

#[derive(thiserror::Error, Debug)]
pub enum MasterKey256Error {
    #[error("Failed to encode salt as base64")]
    SaltEncodingError,
    #[error("Argon2 error")]
    PasswordHashError(argon2::password_hash::Error),
}

impl MasterKey256 {
    fn new(argon2: &Argon2, password: impl AsRef<[u8]>, salt: SaltString)
        -> Result<Self, MasterKey256Error>
    where
        Self: Sized
    {
        let mut hasher = Sha3_256::new();
        hasher.update(salt.as_salt().as_str().as_bytes());
        let mac = hasher.finalize();
        
        let salt_string = SaltString::encode_b64(&mac)
            .map_err(|_| MasterKey256Error::SaltEncodingError)?;
        let password_hash = argon2
            .hash_password(password.as_ref(), &salt_string)
            .map_err(MasterKey256Error::PasswordHashError)?;
            
        debug_assert_eq!(password_hash.hash.unwrap().as_bytes().len(), 32);
        let key = SecreteGenericArray::new(
            generic_array::GenericArray::from_slice(password_hash.hash.unwrap().as_bytes()).to_owned()
        );
        Ok(MasterKey256 { key })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MasterKey256ParseError {

}

//impl TryInto<pkcs8::PrivateKeyInfo<'_>> for MasterKey256 {
//    type Error = Infallible;
//    fn try_into(self) -> Result<pkcs8::PrivateKeyInfo<'static>, Self::Error> {
//        Ok(PrivateKeyInfo {
//            algorithm: AlgorithmIdentifier {
//                oid: (),
//                parameters: None,
//            },
//            private_key: self.key.0.expose_secret(),
//            public_key: None,
//        })
//    }
//}

impl MasterKey256 {
    pub fn oid() -> Option<ObjectIdentifier> {
        // Example OID for the master key (custom or private)
        Some(
            ObjectIdentifier::new("1.3.6.1.4.1.99999.1.1").unwrap()
        )
    }
}