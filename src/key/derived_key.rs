use std::{default, ops::DerefMut};

use generic_array::GenericArray;
use secrecy::{ExposeSecret, ExposeSecretMut};
use sha3::Sha3_256;

use crate::bucket::bucket_guid::BucketGuid;

use super::{master_key::MasterKey256, memory::secure_generic_array::SecreteGenericArray};
pub struct DerivedKey {
    pub key: SecreteGenericArray<u8, generic_array::typenum::U32>,
}


pub struct DeriveKeyParams {
    pub rounds: u32,
}

impl DerivedKey {
    /// Creates a derive key from the master key using KDF function.
    pub fn new(master_key: &MasterKey256, bucket_guid: &BucketGuid, params: &DeriveKeyParams) -> Self{
        let mut secrete = SecreteGenericArray::new(GenericArray::<u8, generic_array::typenum::U32>::default());
        pbkdf2::pbkdf2_hmac::<Sha3_256>(&master_key.key.expose_secret(), &bucket_guid.to_bytes(), params
            .rounds , secrete.expose_secret_mut()); 

        Self {
            key: secrete,
        }
    }
}