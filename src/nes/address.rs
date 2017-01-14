use std::fmt;
use std::ops::{Sub,Add};

pub trait Addressable:
        Sized +
        Sub<Self, Output = Self> +
        Add<Self, Output = Self> +
        fmt::LowerHex + fmt::UpperHex
{
    fn nes_address(&self) -> u16;

    #[inline]
    fn nes_usize(&self) -> usize {
        self.nes_address() as usize
    }
}

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

impl Sub for Address {
    type Output = Address;

    fn sub(self, other: Address) -> Address {
        Address(self.to_u16() - other.to_u16())
    }
}

impl Sub<u16> for Address {
    type Output = Address;

    fn sub(self, other: u16) -> Address {
        Address(self.to_u16() - other)
    }
}

impl Add for Address {
    type Output = Address;

    fn add(self, other: Address) -> Address {
        Address(self.to_u16() + other.to_u16())
    }
}

impl Add<u16> for Address {
    type Output = Address;

    fn add(self, other: u16) -> Address {
        Address(self.to_u16() + other)
    }
}

impl fmt::UpperHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl fmt::LowerHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}


impl Addressable for Address {
    #[inline]
    fn nes_address(&self) -> u16 {
        self.to_u16()
    }
}

impl Addressable for u16 {
    #[inline]
    fn nes_address(&self) -> u16 {
        *self
    }
}
