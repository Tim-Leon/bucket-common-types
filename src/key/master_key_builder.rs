use argon2::PasswordHash;
use generic_array::GenericArray;
use p256::elliptic_curve::bigint::{Encoding, NonZero};
use p256::{NistP256, U256};
use vsss_rs::{combine_shares, shamir::split_secret};
use rand::{CryptoRng, RngCore, SeedableRng};
use crate::key::memory::secure_generic_array::SecreteGenericArray;
use p256::Scalar;
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
    pub share: Vec<u8>,
}

pub struct Secretes {
    pub master_key: MasterKey256,
    pub secrete_shares: Vec<SecreteShare>,
    pub threshold: usize, // When we reconstruct the secrete we must meet the threshold.
}

impl MasterKeyBuilder {
    pub fn new(params: VerifiableSecretSharingSchemeParams) -> Self  {
        if params.validate() {
            Self {
                params,
            }
        } else {
            panic!("Invalid secret sharing parameters");
        }
    }

    pub fn combine(&self, secrete_shares: Vec<SecreteShare>) -> MasterKey256 {
        assert!(
            secrete_shares.len() >= self.params.threshold,
            "Not enough shares to meet the threshold"
        );

        // Convert SecreteShare to Vec<Gf256> for reconstruction
        let shares: Vec<Vec<u8>> = secrete_shares.iter()
            .map(|s| s.share.clone()) 
            .collect();
            
        // Reconstruct the secret
        let reconstructed_secret: p256::Scalar = combine_shares::<Scalar, u8, Vec<u8>>(&shares)
        .expect("Failed to combine shares");
        
        // Wrap the reconstructed secret into a `MasterKey256`
        MasterKey256 {
            key:SecreteGenericArray::new(*GenericArray::from_slice(&reconstructed_secret.to_bytes())),
        }
    }

    /// Build and return generated master keys
    pub fn build(&self, kdf: PasswordHash<'_>) -> Secretes {
        let kdf_output = kdf.hash.expect("Password hash is missing");
        assert_eq!(
            kdf_output.len(),
            32,
            "Password hash must be 256 bits (32 bytes)"
        );

        // Convert password hash to scalar securely
        let modulus = <NistP256 as p256::elliptic_curve::Curve>::ORDER;
        let mut value = U256::from_be_slice(kdf_output.as_bytes());
        let non_zero_modulus = NonZero::new(modulus).expect("Modulus cannot be zero");
        value = value.rem(&non_zero_modulus); // Reduce modulo p
        let secret_scalar = p256::NonZeroScalar::from_repr(value.to_be_bytes().into())
            .expect("Failed to create Scalar from password hash");
        
        // Create a deterministic seed from the KDF output
        let seed = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(kdf_output.as_bytes());
            hasher.update(b"rng_seed"); // Domain separation
            let hash = hasher.finalize();
            let mut seed = [0u8; 16];
            seed.copy_from_slice(&hash.as_bytes()[0..16]);
            seed
        };
        
        let mut deterministic_rng = DeterministicRng::from_seed(seed);
        
        // Split the secret into shares
        //let scalar_bytes = secret_scalar.to_repr().as_ref().to_vec();
        let shares = split_secret::<Scalar, u8, Vec<u8>>(
            self.params.threshold,
            self.params.limit,
            *secret_scalar.as_ref(),
            &mut deterministic_rng
        ).expect("Failed to split secret");

        // Wrap shares into SecretShare
        let secret_shares: Vec<SecreteShare> = shares
            .into_iter()
            .map(|share| SecreteShare { share })
            .collect();

        Secretes {
            master_key: MasterKey256 { 
                key: SecreteGenericArray::new(GenericArray::from_slice(kdf_output.as_bytes() ).to_owned() )
            },
            secrete_shares: secret_shares,
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
