use std::{alloc::{AllocError, Allocator, Global, Layout}, convert::Infallible, default, ops::{Deref, DerefMut}};
use generic_array::{ArrayLength, GenericArray};
use rand::{CryptoRng, RngCore};
use secrecy::{ExposeSecret, ExposeSecretMut, SecretBox};
use zeroize::{Zeroize, ZeroizeOnDrop};

use super::CryptoSecureAllocator;

/// A secure wrapper around a generic array of secrets.
/// Support custom allocator for Memory protected against swapping and unauthorized access, if nesassary?
/// When running the application we will just disable swap for the linux system probably, so there is no way for the encryption key to leek in that sense. 
#[derive(ZeroizeOnDrop)]
pub struct SecreteGenericArray<T, TLength: ArrayLength>(SecretBox<GenericArray<T, TLength>>)
where
    T: Zeroize,
    GenericArray<T, TLength>: Zeroize;

#[derive(thiserror::Error, Debug)]
pub enum SecreteGenericArrayError {
    #[error(transparent)]
    AllocationFail(#[from] AllocError),
}

impl<T, TLength> SecreteGenericArray<T, TLength>
where
    T: Zeroize,
    TLength: ArrayLength,
{
    /// Creates a new `SecureGenericArray` with global allocator, no memory specific protection.
    pub fn new(inner: GenericArray<T, TLength>) -> Self {
        Self::new_in(inner, &Global).unwrap()
    }

    /// Creates a new `SecureGenericArray` using a custom allocator.
    pub fn new_in<TAllocator>(
        inner: GenericArray<T, TLength>,
        allocator:&TAllocator,
    ) -> Result<Self, SecreteGenericArrayError>
    where
        TAllocator: Allocator + ?CryptoSecureAllocator, // Might be crpyot secure allocator, pls mark it if it's required but also need support for global allocator.
    {
        // Wrap the allocated memory in a `SecretBox`.
        let boxed = unsafe { Box::<T, &TAllocator>::from_raw_in( inner.as_mut_ptr(), allocator) };
        Ok(Self(SecretBox::new(boxed)))
    }

    /// Generates a new `SecureGenericArray` filled with random bytes.
    /// Use global if you want to use the global allocator.
    pub fn generate_with_rng<TCryptoRng, TAllocator>(
        rng: &mut TCryptoRng,
        allocator: &TAllocator
    ) -> Result<Self, SecreteGenericArrayError>
    where
        TCryptoRng: RngCore + CryptoRng,
        T: Default,
        TAllocator: Allocator
    {
        let mut slice = GenericArray::<u8, TLength>::default(); 
        rng.fill_bytes(&mut slice);
        Ok(
            Self::new_in(slice, allocator)?
        )
    }
}



impl<T, TLength> From<GenericArray<T, TLength>> for SecreteGenericArray<T, TLength> 
where
T: Zeroize,
TLength: ArrayLength, {
    fn from(value: GenericArray<T, TLength>) -> Self {
       Self (SecretBox::new(Box::new(value)))
    }
}


/// Implement `ExposeSecret` for read-only access to the secret.
impl<T, TLength> ExposeSecret<GenericArray<T, TLength>> for SecreteGenericArray<T, TLength>
where
    T: Zeroize,
    TLength: ArrayLength,
{
    fn expose_secret(&self) -> &GenericArray<T, TLength> {
        self.0.expose_secret()
    }
}

/// Implement `ExposeSecretMut` for mutable access to the secret.
impl<T, TLength> ExposeSecretMut<GenericArray<T, TLength>> for SecreteGenericArray<T, TLength>
where
    T: Zeroize,
    TLength: ArrayLength,
{
    fn expose_secret_mut(&mut self) -> &mut GenericArray<T, TLength> {
        self.0.expose_secret_mut()
    }
}