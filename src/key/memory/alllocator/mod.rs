
use std::alloc::{Allocator, Layout, AllocError};
use std::os::raw::c_void;
use std::ptr;
use region::{self, Allocation, Protection};

use super::CryptoSecureAllocator;

/// This is a secure memory allocator that will call mprotect and mlock
/// on the memory page to prevent swapping and secure access to sensitive data.
/// Note that if we do access protection then we need to support 
pub struct SecurePoolAllocator {
    pool: Vec<Allocation>, 
    used: u64, 
    total: u64,
}

pub struct SecureMemory {
    pub allocation: Allocation, 
    pub protection: Protection, 
}

impl SecurePoolAllocator {
    pub fn new() -> Result<Self, AllocError> {
        // Step 1: Allocate a large block of memory (memory-mapped region)
            // Memory-mapped region with read-write protection
        let allocation_size = region::page::size(); 
        let mmap = region::alloc(allocation_size, Protection::READ_WRITE)
                .map_err(|_| AllocError)?;


        // Lock the memory into RAM (prevent swapping)
        region::lock(mmap.as_ptr::<c_void>(), mmap.len()).map_err(|_| AllocError)?;

        // Optionally set the protection to be read-only for secure memory handling
        unsafe {
            region::protect(mmap.as_ptr::<c_void>(), mmap.len(), Protection::READ);
        }

        dbg!("SecurePoolAllocator initialized with {} bytes", allocation_size);

        // Step 2: Return the pool allocator
        Ok(SecurePoolAllocator {
            pool: mmap,
            used: 0,
            total: todo!(),  // Initially, nothing is used
        })
    }

    fn allocate_from_pool(&mut self, layout: Layout) -> *mut u8 {
        // Step 3: Check if there is enough space left in the pool
        if self.used + layout.size() > region::page::size() {
            dbg!(
                "Allocation failed: not enough space. Requested: {} bytes, Available: {} bytes",
                layout.size(),
                region::page::size() - self.used
            );
            return ptr::null_mut();  // Not enough space in the pool
        }

        let ptr = unsafe {
            self.pool.add(self.used)  // Get the next available chunk in the pool
        };
        self.used += layout.size();  // Update used memory tracker

        dbg!(
            "Allocated {} bytes from pool. Total used: {} bytes",
            layout.size(),
            self.used
        );
        ptr
    }

    fn protect_memory(protection: region::Protection) {

    }
}

unsafe impl Allocator for SecurePoolAllocator {
    fn allocate(&self, layout: Layout) -> Result<std::ptr::NonNull<[u8]>, AllocError> {
        // Step 4: Try to allocate memory from the pool
        let ptr = self.allocate_from_pool(layout);
        if ptr.is_null() {
            return Err(AllocError);  // Allocation failed, pool is exhausted
        }

        // Return the memory as NonNull
        Ok(unsafe { std::ptr::NonNull::new_unchecked(ptr) })
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: Layout) {
        // Deallocation is handled in a simple way here, but you can implement more sophisticated free lists
        // For now, we do nothing since we're using a pool.
        dbg!(
            "Deallocated {} bytes from pool (Note: deallocation not implemented).",
            layout.size()
        );
    }
    

}

impl CryptoSecureAllocator for SecurePoolAllocator {

}