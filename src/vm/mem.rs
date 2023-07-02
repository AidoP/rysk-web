#![allow(unused)]
use super::RegionRef;

use crate::types::{Addressable, Cause};

/// Page-aligned fixed-sized dynamically allocated memory.
pub struct Memory {
    ptr: *mut u8,
    len: u32,
}
impl Memory {
    const PAGE_SIZE: u32 = 4096;
    pub fn new(pages: u32) -> Self {
        assert!(Self::PAGE_SIZE != 0);
        assert!(Self::PAGE_SIZE.is_power_of_two());

        if pages == 0 {
            return Self {
                ptr: core::ptr::null_mut(),
                len: 0,
            };
        }
        let len = pages * Self::PAGE_SIZE;
        if usize::try_from(len).unwrap() > isize::MAX as usize {
            panic!("the memory required to allocate {pages} pages overflows isize")
        }
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(
                len as usize,
                Self::PAGE_SIZE as usize,
            );
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout)
            }
            const DEFAULT_DATA: &str = "🚀 Blast off!\n🦀 - Hello, Ferris!";
            ptr.copy_from(DEFAULT_DATA.as_ptr(), DEFAULT_DATA.len());
            Self { ptr, len }
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len as usize) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len as usize) }
    }

    pub fn as_region(&mut self, address: u32) -> RegionRef {
        RegionRef {
            ty: super::RegionType::Dram,
            ptr: self.as_ptr(),
            address,
            len: self.len,
        }
    }
}
impl Addressable<u32> for Memory {
    fn read_u8(&self, address: u32) -> u8 {
        todo!()
    }

    fn read_u16(&self, address: u32) -> u16 {
        todo!()
    }

    fn read_u32(&self, address: u32) -> u32 {
        todo!()
    }

    fn read_u64(&self, address: u32) -> u64 {
        todo!()
    }

    fn write_u8(&self, address: u32, byte: u8) -> Result<(), Cause> {
        if address >= self.len() {
            return Err(Cause::STORE_FAULT);
        }
        unsafe { self.ptr.offset(address as isize).write(byte) }
        Ok(())
    }

    fn write_u16(&self, address: u32, halfword: u16) -> Result<(), Cause> {
        todo!()
    }

    fn write_u32(&self, address: u32, word: u32) -> Result<(), Cause> {
        todo!()
    }

    fn write_u64(&self, address: u32, doubleword: u64) -> Result<(), Cause> {
        todo!()
    }
}
