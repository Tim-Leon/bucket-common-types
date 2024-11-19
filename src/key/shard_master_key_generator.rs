use argon2::password_hash::Salt;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use vsss_rs::{combine_shares, shamir::split_secret, Gf256};
use rand::thread_rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerifiableSecretSharingSchemeAlgorithm {
    ShamirSecret,
}

/// Parameters for Verifiable Secret Sharing Scheme
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VerifiableSecretSharingSchemeParams {
    pub algorithm: VerifiableSecretSharingSchemeAlgorithm,
    /// Number of shares required to reconstruct the secret
    pub threshold: u32,
    /// Total number of shares to generate
    pub limit: u32,
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
pub struct MasterKeyBuilder<'a> {
    pub split_secret_params: Option<VerifiableSecretSharingSchemeParams>,
    pub password_hash: Option<PasswordHash<'a>>,
}

pub struct SplitSecret {
    pub shares: Vec<Gf256>,
}

pub struct GeneratedMasterKeys {
    pub master_key: MasterKey256,
    pub split_keys: Vec<SplitSecret>,
}

impl<'a> MasterKeyBuilder<'a> {
    /// Add secret sharing parameters
    pub fn with_split_secret(mut self, params: VerifiableSecretSharingSchemeParams) -> Self {
        if params.validate() {
            self.split_secret_params = Some(params);
        } else {
            panic!("Invalid secret sharing parameters");
        }
        self
    }

    /// Add the password hash
    pub fn with_password_hash(mut self, password_hash: PasswordHash<'a>) -> Self {
        self.password_hash = Some(password_hash);
        self
    }

    /// Build and return generated master keys
    pub fn build(self) -> GeneratedMasterKeys {
        let params = self.split_secret_params.unwrap_or_default();
        let password_hash = self.password_hash.expect("Password hash is required");

        // Convert password hash to bytes for secret sharing
        let secret_bytes = password_hash.to_string().into_bytes();
        let secret_field = Gf256::try_from(secret_bytes.as_slice()).expect("Invalid secret for Gf256");

        // Split the secret into shares
        let mut rng = thread_rng();
        let shares = split_secret(params.threshold as usize, params.limit as usize, &secret_field, &mut rng)
            .expect("Failed to split secret");

        // Verify and reconstruct the secret to ensure correctness
        let reconstructed_secret = combine_shares(&shares).expect("Failed to combine shares");
        assert_eq!(secret_field, reconstructed_secret);

        // Wrap shares into `SplitSecret`
        let split_keys = shares
            .into_iter()
            .map(|share| SplitSecret { shares: vec![share] })
            .collect();

        GeneratedMasterKeys {
            master_key: MasterKey256::from(secret_field.to_bytes()),
            split_keys,
        }
    }
}
