#![allow(unused)]
use crate::types::{Addressable, Cause};
use super::RegionRef;


/// Page-aligned fixed-sized dynamically allocated memory.
pub struct Memory {
    ptr: *mut u8,
    len: u32
}
impl Memory {
    const PAGE_SIZE: u32 = 4096;
    pub fn new(pages: u32) -> Self {
        assert!(Self::PAGE_SIZE != 0);
        assert!(Self::PAGE_SIZE.is_power_of_two());
        
        if pages == 0 {
            return Self {
                ptr: core::ptr::null_mut(),
                len: 0
            }
        }
        let len = pages * Self::PAGE_SIZE;
        if usize::try_from(len).unwrap() > isize::MAX as usize {
            panic!("the memory required to allocate {pages} pages overflows isize")
        }
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(len as usize, Self::PAGE_SIZE as usize);
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout)
            }
            for i in 0..len {
                ptr.add(i as usize).write(i as u8);
            }
            Self {
                ptr,
                len
            }
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
            address,
            ptr: self.ptr,
            len: self.len
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
        todo!()
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
