use argon2::{Argon2, PasswordHash};
use vsss_rs::{*, feldman};
use elliptic_curve::ff::Field;
use vsss_rs::elliptic_curve::Scalar;
use vsss_rs::shamir::split_secret;
use crate::key::CryptoMasterKey;
use crate::key::master_key::MasterKey256;

pub enum VerifiableSecretSharingSchemeAlgorithm
{
    ShamirSecrete,
}


/// https://docs.rs/vsss-rs/latest/vsss_rs/
pub struct VerifiableSecretSharingSchemeParams {
    pub algorithm: VerifiableSecretSharingSchemeAlgorithm,
    /// How many shares of the secrete is needed in order for it to be reconstructed.
    pub threshold: u32,
    /// How many secretes to generate.
    pub limit: u32,
}

impl Default for VerifiableSecretSharingSchemeParams {
    fn default() -> Self {
        Self {
            algorithm: VerifiableSecretSharingSchemeAlgorithm::ShamirSecrete,
            threshold: 2,
            limit: 3,
        }
}
}

/// Creates a series of master keys
/// We split it into 5 parts,
/// 1. password
/// 2. server-secrete
/// 3. recover-code
///
/// We require at-least 2 of the secretes, password is used to create the secretes so it is responsible for creating the keys.
/// We use the 1 and 2nd for normal devices.
/// When user loses the password they can use recover-code to gain the secrete back.
/// If they don't have the recovery code they can if they want to store a backup in devices such as
pub struct MasterKeyShareGenerator {

}


/// WE take the master key, split it into multiple parts
impl MasterKeyShareGenerator {
    pub fn split(&self, password_hash: PasswordHash, params: VerifiableSecretSharingSchemeParams) -> Box<[impl CryptoMasterKey]> {
        let secret = password_hash.to_string();
        let a = 
        let g = Gf256::try_from(secret).unwrap();

        let res = split_secret::<G1Projective, u8, Vec<u8>>(params.threshold, params.limit, g, None, &mut rng);
        assert!(res.is_ok());
        let (shares, verifier) = res.unwrap();
        for s in &shares {
            assert!(verifier.verify_share(s).is_ok());
        }
        let res = combine_shares(&shares);
        assert!(res.is_ok());
        let secret_1: Scalar = res.unwrap();
        assert_eq!(secret, secret_1);
    }

    pub fn combine() -> MasterKey256 {

        combine_shares()
    }
}