use argon2::password_hash::Salt;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use generic_array::GenericArray;
use p256::elliptic_curve::bigint::Encoding;
use p256::elliptic_curve::{Curve, PrimeField};
use p256::{NistP256, U256};
use vsss_rs::elliptic_curve::Scalar;
use vsss_rs::{combine_shares, shamir::split_secret, Gf256};
use rand::{thread_rng, CryptoRng, RngCore, SeedableRng};

use crate::key::memory::secure_generic_array::SecreteGenericArray;

use super::master_key::MasterKey256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerifiableSecretSharingSchemeAlgorithm {
    ShamirSecret,
}

/// Parameters for Verifiable Secret Sharing Scheme
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VerifiableSecretSharingSchemeParams {
    pub algorithm: VerifiableSecretSharingSchemeAlgorithm,
    /// Number of shares required to reconstruct the secret
    pub threshold: usize,
    /// Total number of shares to generate
    pub limit: usize,
}

impl Default for VerifiableSecretSharingSchemeParams {
    fn default() -> Self {
        Self {
            algorithm: VerifiableSecretSharingSchemeAlgorithm::ShamirSecret,
            threshold: 2,
            limit: 3,
        }
    }
}

impl VerifiableSecretSharingSchemeParams {
    pub fn validate(&self) -> bool {
        self.threshold > 0 && self.threshold <= self.limit
    }
}

/// Struct for building master keys with verifiable secret sharing
#[derive(Clone)]
pub struct MasterKeyBuilder {
    pub params: VerifiableSecretSharingSchemeParams,
}


pub struct SecreteShare {
    pub share: Gf256,
}

pub struct Secretes {
    pub master_key: MasterKey256,
    pub secrete_shares: Vec<SecreteShare>,
    pub threshold: usize, // When we reconstruct the secrete we must meet the threshold.
}

impl MasterKeyBuilder {
    pub fn new(params: VerifiableSecretSharingSchemeParams) -> Self  {
        if params.validate() {
            return Self {
                params,
            }
        } else {
            panic!("Invalid secret sharing parameters");
        }
    }

    pub fn combine(&self, secrete_shares: Vec<Gf256>) -> MasterKey256 {
        assert!(
            secrete_shares.len() >= self.params.threshold,
            "Not enough shares to meet the threshold"
        );


        // Reconstruct the secret
        let reconstructed_secret = combine_shares(&secrete_shares.as_slice()).expect("Failed to combine shares");

        // Wrap the reconstructed secret into a `MasterKey256`
        MasterKey256 {
            key: SecreteGenericArray::new(reconstructed_secret.to_bytes()),
        }
    }

    /// Build and return generated master keys
    pub fn build<'a>(&self, kdf: PasswordHash<'a>) -> Secretes {
        let kdf_output = kdf.hash.expect("Password hash is missing");
        assert_eq!(
            kdf_output.len(),
            32,
            "Password hash must be 256 bits (32 bytes)"
        );

        // Convert password hash to scalar
        let modulus = NistP256::ORDER;
        let mut value = U256::from_be_slice(&kdf_output.as_bytes());
        value = value % modulus; // Reduce modulo p
        let secrete_scalar = Scalar::from_repr(value.to_be_bytes().into())
            .expect("Failed to create Scalar from password hash");


        // Split the secret into shares
        let mut deterministic_rng = DeterministicRng::seed_from_u64(102312343859310);
        
        let shares = split_secret(self.params.threshold.clone(),
             self.params.limit.clone(),
              secrete_scalar,
             deterministic_rng)
            .expect("Failed to split secret");



        // Wrap shares into `SplitSecret`
        let split_keys = shares
            .into_iter()
            .map(|share| SecreteShare { share: share })
            .collect();
        let a = generic_array::GenericArray::<u8, generic_array::typenum::U32>::from_slice(kdf_output.as_bytes());
        Secretes {
            master_key: MasterKey256 { key: SecreteGenericArray::new( *a ) },
            secrete_shares: split_keys,
            threshold: self.params.threshold,
        }
    }
}


pub struct DeterministicRng(rand_xorshift::XorShiftRng);

impl Default for DeterministicRng {
    fn default() -> Self {
        Self::from_seed([7u8; 16])
    }
}

impl SeedableRng for DeterministicRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        Self(rand_xorshift::XorShiftRng::from_seed(seed))
    }
}

impl CryptoRng for DeterministicRng {}

impl RngCore for DeterministicRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(dest)
    }
}
