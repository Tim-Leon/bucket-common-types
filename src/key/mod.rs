use argon2::password_hash::SaltString;
use argon2::Argon2;
use core::slice::SlicePattern;
use std::hash::Hash;
use digest::{Digest, FixedOutput};
use generic_array::typenum::IsGreaterOrEqual;
use generic_array::{ArrayLength, GenericArray};
use secrecy::ExposeSecret;
use std::fmt::Debug;
use zeroize::Zeroize;

pub mod derived_key;
pub mod master_key;
pub mod nonce_generator;
pub mod master_key_builder;
pub mod memory;

pub struct Nonce<TNonceLength: ArrayLength>(GenericArray<u8, TNonceLength>);

/// A trait for generating nonce's of a specified length,
/// This trait is for non-deterministic nonce generation only.
pub trait NonceGenerator<TNonceLength>
where
    TNonceLength: ArrayLength,
    TNonceLength: IsGreaterOrEqual<generic_array::typenum::U8>, // Minimum supported output for nonce is 64 bits, for security reasons. 96 bits or more preferred.
{
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


#[derive(thiserror::Error, Debug)]
pub enum MasterKeyErrors {

}

