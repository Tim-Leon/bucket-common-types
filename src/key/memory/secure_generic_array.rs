use core::slice::SlicePattern;
use std::alloc::{Allocator, Layout};
use generic_array::{ArrayLength, GenericArray};
use secrecy::{ExposeSecret, SecretBox};
use zeroize::Zeroize;

use super::alllocator::CryptoSecureAllocator;

/// A secure wrapper around a generic array of secrets.
/// Memory is protected against swapping and unauthorized access.
#[derive(Zeroize)]
pub struct SecureGenericArray<T, TLength: ArrayLength>(pub SecretBox<GenericArray<T, TLength>>)
where
    GenericArray<T, TLength>: Zeroize;

/// Implement `SlicePattern` to expose the secret array as a slice.
impl<T, TLength> SlicePattern for SecureGenericArray<T, TLength>
where
    TLength: ArrayLength,
    GenericArray<T, TLength>: Zeroize,
{
    type Item = T;

    /// Returns a slice of the underlying secret data.
    fn as_slice(&self) -> &[Self::Item] {
        self.0.expose_secret().as_slice()
    }
}

impl<T, TLength> SecureGenericArray<T, TLength>
where
    T: Zeroize,
    TLength: ArrayLength,
{
    /// Creates a new `SecureGenericArray` from a `GenericArray` using the given allocator.
    pub fn new<TAllocator: Allocator + CryptoSecureAllocator>(allocator: &TAllocator, array: GenericArray<T, TLength>) -> Result<Self, ()> {
        // Calculate memory layout for the GenericArray
        let layout = Layout::new::<GenericArray<T, TLength>>();
        dbg!("Layout for GenericArray", &layout);

        // Allocate memory using the allocator
        let memory = allocator.allocate(layout).map_err(|_| ())?;

        // Initialize the memory block with the given array
        unsafe {
            let ptr = memory.as_ptr() as *mut GenericArray<T, TLength>;
            ptr.write(array);
        }

        // Wrap the memory block in a SecretBox for secure handling
        let secret_box = unsafe { SecretBox::new(Box::from_raw(memory.as_ptr() as *mut GenericArray<T, TLength>)) };

        Ok(Self(secret_box))
    }
}

impl<T, TLength> ExposeSecret<T> for SecureGenericArray<T, TLength> {
    fn expose_secret(&self) -> &T {
        self.0.expose()
    }
}