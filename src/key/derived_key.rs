use aes_gcm::KeyInit;
use generic_array::ArrayLength;
use core::slice::SlicePattern;
use std::convert::Infallible;
use digest::{Digest, FixedOutput};
use digest::generic_array::{typenum, GenericArray, ArrayLength};
use secrecy::ExposeSecret;
use sha3::digest;
use sha3::digest::Update;
use sha3::Sha3_256;
use crate::key::{CryptoHashDerivedKeyType, CryptoMasterKey, SecureGenericArray};

/// 256-bit key
pub type CryptoHashDerivedKeySha3_256 = Sha3_256CryptoHashDerivedKey<typenum::U32>;


/// Represents a derived encryption key generated from a master key and nonce.
#[derive(Clone, Debug)]
pub struct Sha3_256CryptoHashDerivedKey<TKeyLength>
where
    TKeyLength: generic_array::ArrayLength,
{
    /// The securely derived secret key.
    secret: SecureGenericArray<u8, TKeyLength>, // Match the key length dynamically
}

// Implementing SlicePattern to expose the secret key as a slice of bytes.
impl<TKeyLength> SlicePattern for Sha3_256CryptoHashDerivedKey<TKeyLength> where TKeyLength: ArrayLength<T>
{
    type Item = u8;

    fn as_slice(&self) -> &[Self::Item] {
        self.secret.0.expose_secret().as_slice() // Expose the secret as a slice
    }
}

impl<TKeyLength> CryptoHashDerivedKeyType<TKeyLength> for Sha3_256CryptoHashDerivedKey<TKeyLength> where TKeyLength: ArrayLength<T>
{
    type Error = Infallible;
    type CryptoHasher = Sha3_256;


    /// Generates a `HashDerivedKey` from a master key and a nonce.
    ///
    /// # Parameters
    /// - `master_key`: The master key to derive from.
    /// - `nonce`: A unique nonce value to ensure the derived key is unique.
    ///
    /// # Returns
    /// A new `HashDerivedKey` instance derived from the provided master key and nonce.
    fn from_key_and_nonce<TInputNonceLength: ArrayLength>(master_key: &(impl CryptoMasterKey + SlicePattern), nonce: &generic_array::GenericArray<u8, TInputNonceLength>) -> Result<Self, Infallible> {
        let mut hasher = Self::CryptoHasher::default();
        hasher.update(master_key.as_slice());
        hasher.update(nonce);
        // Create a SecureGenericArray from the finalized hash
       Ok( Self {
            secret: SecureGenericArray {
                0: GenericArray::from_slice(&hasher.finalize()),
            },
        })
    }
}

impl<TArrayLength> TryInto<aes_gcm::Aes256Gcm> for Sha3_256CryptoHashDerivedKey<TArrayLength> {
    type Error = aes_gcm::aes::cipher::InvalidLength;
    fn try_into(self) -> Result<aes_gcm::Aes256Gcm, Self::Error> {
        Ok(aes_gcm::Aes256Gcm::new_from_slice(self.as_slice())?)
    }
}

impl<TArrayLength> TryInto<chacha20poly1305::ChaCha20Poly1305> for Sha3_256CryptoHashDerivedKey<TArrayLength> {
    type Error = digest::InvalidLength;
    fn try_into(self) -> Result<chacha20poly1305::ChaCha20Poly1305, Self::Error> {
        Ok(chacha20poly1305::ChaCha20Poly1305::new_from_slice(self.as_slice())?)
    }
}

