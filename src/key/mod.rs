use argon2::password_hash::SaltString;
use argon2::Argon2;
use core::slice::SlicePattern;
use digest::{Digest, FixedOutput};
use generic_array::typenum::IsGreaterOrEqual;
use generic_array::{ArrayLength, GenericArray};
use secrecy::ExposeSecret;
use std::fmt::Debug;
use zeroize::Zeroize;

pub mod derived_key;
pub mod master_key;
pub mod nonce_generator;
mod shard_master_key_generator;

/// TODO: implement DER encoding.

/// A secure wrapper around a generic array of secrets. TODO: disable memory swap for this memory in linux and windows.
#[derive(Zeroize)]
pub struct SecureGenericArray<T, TLength: ArrayLength>(pub Secret<GenericArray<T, TLength>>)
where
    generic_array::GenericArray<T, TLength>: Zeroize;


// TODO: Maybe remove cause it kinda defeats the purpose of Secret struct specifically the traits it implements to easily audit code for exposed secretes.
impl<T, TLength: ArrayLength> SlicePattern for SecureGenericArray<T, TLength>
where
    generic_array::GenericArray<T, TLength>: Zeroize,
{
    type Item = T;

    /// Returns a slice of the underlying secret data.
    fn as_slice(&self) -> &[Self::Item] {
        self.0.expose_secret().as_slice()
    }
}
pub struct Nonce<TNonceLength: ArrayLength>(GenericArray<u8, TNonceLength>);

/// A trait for generating nonce's of a specified length,
/// This trait is for non-deterministic nonce generation only.
pub trait NonceGenerator<TCryptoRng, TNonceLength>
where
    Self: Zeroize,
    TCryptoRng: rand::CryptoRng,
    TCryptoRng: rand::RngCore,
    TNonceLength: ArrayLength,
    TNonceLength: IsGreaterOrEqual<generic_array::typenum::U8>, // Minimum supported output for nonce is 64 bits, for security reasons. 96 bits or more preferred.
{

    /// Creates a new nonce generator which will use the provided cryptographically secure random number generator.   
    fn new<T>(csprng: TCryptoRng, seed: T) -> Self;

    /// Generates the next nonce.
    fn next(&mut self) -> Nonce<TNonceLength>;
}


pub trait DeterministicNonceGenerator<TNonceLength>
where
    Self: Zeroize,
    TNonceLength: ArrayLength,
    TNonceLength: IsGreaterOrEqual<generic_array::typenum::U8> {
    fn new<TLength: ArrayLength>(seed: GenericArray<u8, TLength>) -> Self;

    fn next(&mut self) -> Nonce<TNonceLength>;
}

/// Marker trait
pub trait UniqueNonce {

}


/// A trait representing a master key for cryptographic operations.
pub trait CryptoMasterKey
where
    Self: Sized,
    Self: SlicePattern,
{
    /// The type of error returned by methods in this trait.
    type Error: Debug = MasterKeyErrors;

    /// The length of the key used in cryptographic operations.
    type KeyLength: ArrayLength;

    /// The type of cryptographic hasher used to derive keys.
    type CryptoHasher: digest::Digest + FixedOutput<OutputSize = Self::KeyLength>;

    /// Generates a new master key using the provided cryptographically secure RNG.
    fn generate<TCryptoRng: rand::CryptoRng + rand::RngCore>(csprng: &mut TCryptoRng) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Creates a new master key from the given Argon2 parameters.
    ///
    /// # Arguments
    ///
    /// * `argon2` - A reference to an Argon2 instance.
    /// * `password` - The password to use for key generation.
    /// * `salt` - A salt string to enhance security.
    fn new(
        argon2: &Argon2,
        password: impl AsRef<[u8]>,
        salt: SaltString,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// A trait for deriving keys from a master key and nonce.
pub trait CryptoHashDerivedKeyType<TKeyLength>
where
    TKeyLength: ArrayLength,
{
    /// The type of error returned by methods in this trait.
    type Error: Debug;

    /// The type of cryptographic hasher used to derive keys.
    type CryptoHasher: digest::Digest + FixedOutput<OutputSize =TKeyLength>;

    /// Derives a key from the master key and nonce.
    ///
    /// # Arguments
    ///
    /// * `master_key` - A reference to a master key implementing the `CryptoMasterKey` and `SlicePattern` traits.
    /// * `nonce` - A reference to a nonce array.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the derived key or an error if the operation fails.
    fn from_key_and_nonce<TInputNonceLength>(
        master_key: &(impl CryptoMasterKey + SlicePattern),
        nonce: &GenericArray<u8, TInputNonceLength>,
    ) -> Result<Self, Self::Error> 
    where
        TInputNonceLength: ArrayLength,
        Self: Sized;
        
}

#[derive(thiserror::Error, Debug)]
pub enum MasterKeyErrors {

}


mod tests {
    use rand::rngs::StdRng;

    use super::CryptoHashDerivedKeyType;

    #[test]
    fn master_key_to_derived_key_test() {
        let mut rng = StdRng::from_rng(OsRng::default()).unwrap();
        let master = MasterKey256::generate(&mut rng).unwrap();
        
        let derived = Sha3_256CryptoHashDerivedKey::from_key_and_nonce(&master, &()).unwrap();

    }
}