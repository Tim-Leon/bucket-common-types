use generic_array::{typenum::IsGreaterOrEqual, ArrayLength, GenericArray};
use vsss_rs::elliptic_curve::bigint;
use zeroize::Zeroize;

use crate::bucket::bucket_guid::BucketGuid;

use super::{DeterministicNonceGenerator, Nonce, NonceGenerator};

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
        
        Self {
            seed: todo!(),
        }
    }
}

impl<TNonceLength> NonceGenerator<TNonceLength> for DeterministicHashSequentialNonceGenerator 
where 
TNonceLength: ArrayLength,
TNonceLength: IsGreaterOrEqual<generic_array::typenum::U8> {
    
    fn next(&mut self) -> Nonce<TNonceLength> {
        
    }
} 