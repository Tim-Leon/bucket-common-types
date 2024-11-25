use generic_array::{typenum::IsGreaterOrEqual, ArrayLength, GenericArray};
use zeroize::Zeroize;

use crate::bucket::bucket_guid::BucketGuid;

use super::{Nonce, NonceGenerator};

/// 96-bit random sequential generator.
/// Generates a random cryptographically secure nonce.
#[derive(Zeroize)]
pub struct RandomSequential92BitNonceGenerator<TCryptoRng> where
TCryptoRng: rand::CryptoRng,
TCryptoRng: rand::RngCore
{
    pub counter: u32,
    pub csprng: TCryptoRng,
}

impl<TCryptoRng, TNonceLength> NonceGenerator<TNonceLength> for RandomSequential92BitNonceGenerator<TCryptoRng> where 
TCryptoRng: rand::CryptoRng, 
TCryptoRng: rand::RngCore,
TNonceLength: ArrayLength,
TNonceLength: IsGreaterOrEqual<generic_array::typenum::U8>
{
    fn next(&mut self) -> Nonce<TNonceLength> {
        let mut nonce = GenericArray::default();
        // Fill fist 4 bytes with the counter
        nonce[0..4].copy_from_slice(&self.counter.to_be_bytes());
        // Fill the last 8 bytes with the random u64
        nonce[4..12].copy_from_slice(&self.csprng.next_u64().to_be_bytes());
        self.counter += 1;
        Nonce (nonce)
    }
}



pub struct DeterministicHashSequentialNonceGenerator {
    pub seed: u128,
}

impl DeterministicHashSequentialNonceGenerator {
    pub fn new(val: &BucketGuid) -> Self {
        let bytes = val.to_bytes();
        Self {
            seed: u128::from_be_bytes(bytes[..16].try_into().unwrap()),
        }
    }
}

impl<TNonceLength> NonceGenerator<TNonceLength> for DeterministicHashSequentialNonceGenerator 
where 
TNonceLength: ArrayLength,
TNonceLength: IsGreaterOrEqual<generic_array::typenum::U8> {
    
    fn next(&mut self) -> Nonce<TNonceLength> {
        let mut nonce = GenericArray::default();
        // Use first 8 bytes of seed for first part of nonce
        nonce[0..8].copy_from_slice(&(self.seed as u64).to_be_bytes());
        // Use second 8 bytes of seed for second part
        nonce[8..16].copy_from_slice(&((self.seed >> 64) as u64).to_be_bytes());
        // Increment seed for next nonce
        self.seed = self.seed.wrapping_add(1);
        Nonce(nonce)
    }
}