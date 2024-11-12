use digest::{generic_array, typenum};
use digest::generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::IsGreaterOrEqual;
use secrecy::Zeroize;

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

impl<TCryptoRng, TNonceLength> NonceGenerator<TCryptoRng, TNonceLength> for RandomSequential92BitNonceGenerator<TCryptoRng>
where TCryptoRng: rand::CryptoRng,
TCryptoRng: rand::RngCore, TNonceLength: generic_array::ArrayLength<T> + IsGreaterOrEqual<generic_array::typenum::U8>
{
    //TODO: Seed
    fn new<T>(csprng: TCryptoRng, seed:T) -> Self {
        Self {
            counter: 0,
            csprng,
        }
    }

    fn next(&mut self) -> Nonce<TNonceLength> {
        use rand::CryptoRng;
        use rand::RngCore;
        let mut nonce = GenericArray::default();
        // Fill fist 4 bytes with the counter
        nonce[0..4].copy_from_slice(&self.counter.to_be_bytes());
        // Fill the last 8 bytes with the random u64
        nonce[4..12].copy_from_slice(&self.csprng.next_u64().to_be_bytes());
        self.counter += 1;
        nonce
    }
}

pub struct DeterministicSequential64BitNonceGenerator<TCryptoRng> where
TCryptoRng: rand::CryptoRng,
TCryptoRng: rand::RngCore{
    pub counter: u64
}

impl< TNonceLength> DeterministicNonceGenerator<TNonceLength> for DeterministicSequential64BitNonceGenerator<TNonceLength>
    where
TNonceLength: generic_array::ArrayLength<T> + IsGreaterOrEqual<typenum::U8>
{
    fn new<TLength: ArrayLength<T>>(seed: GenericArray<u8, TLength>) -> Self {
        Self {
            counter: seed.try_into().unwrap()
        }
    }
    fn next(&mut self) -> Nonce<TNonceLength> {

    }
}