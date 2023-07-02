#![allow(unused)]
use super::{mem::Memory, RegionRef};

use crate::types::Cause;

pub struct Bus {
    dram: Memory,
}
impl Bus {
    pub fn new(pages: u32) -> Self {
        Self {
            dram: Memory::new(pages),
        }
    }
    pub fn regions(&mut self) -> Vec<RegionRef> {
        vec![self.dram.as_region(Self::DRAM_START)]
    }
    pub fn memory(&mut self, address: u32) -> RegionRef {
        match address {
            Self::DRAM_START..=Self::DRAM_END => self.dram.as_region(Self::DRAM_START),
            _ => RegionRef::none(address, 0),
        }
    }

    pub const BEFORE_DRAM_END: u32 = Self::DRAM_START - 1;
    pub const DRAM_START: u32 = 0xF000_0000;
    pub const DRAM_END: u32 = 0xFFFF_FFFF;
    pub const DRAM_LEN: u32 = (Self::DRAM_END - Self::DRAM_START) + 1;
    pub const DRAM_MB: u32 = Self::DRAM_LEN / (1024 * 1024);
}
impl rysk::Addressable<u32> for Bus {
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
        match address {
            Self::DRAM_START..=Self::DRAM_END => {
                self.dram.write_u8(address - Self::DRAM_START, byte)
            }
            _ => Err(Cause::STORE_FAULT),
        }
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
