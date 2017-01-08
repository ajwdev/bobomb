use std::ops::{Index,IndexMut};

use nes::ppu;

pub const ROM_BANK_SIZE: u16 = 16 * 1024;

#[derive(Debug,Copy,Clone)]
pub struct Address(pub u16);

impl Address {
    pub fn new(hi: u8, lo: u8) -> Address {
        Address((hi as u16) << 8 | lo as u16)
    }

    pub fn new_zeropage(lo: u8) -> Address {
        Address(lo as u16)
    }

    pub fn from_bytes(buf: &[u8]) -> Address {
        // Assert the correct length
        Address((buf[1] as u16) << 8 | buf[0] as u16)
    }


    pub fn high(&self) -> u8 {
        ((self.0 & 0xFF00) >> 8) as u8
    }

    pub fn low(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub fn to_u16(&self) -> u16 {
        self.0
    }
}


pub struct Bank {
    pub data: [u8; ROM_BANK_SIZE as usize]
}

impl Bank {
    pub fn new(src: &[u8]) -> Self {
        let mut b = Bank{
            data: [0; ROM_BANK_SIZE as usize],
        };
        b.data.clone_from_slice(src);
        b
    }
}

impl Index<usize> for Bank {
    type Output = u8;

    fn index(&self, idx: usize) -> &u8 {
        &self.data[idx]
    }
}

impl IndexMut<usize> for Bank {
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        &mut self.data[idx]
    }
}
