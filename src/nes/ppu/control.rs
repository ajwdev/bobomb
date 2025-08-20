use std::cmp::PartialEq;
use std::fmt;
use std::ops::BitAnd;

use crate::nes::address::Address;

pub enum VramIncrement {
    AcrossOne,
    DownThirtyTwo,
}

pub enum SpriteSize {
    EightByEight,
    EightBySixteen,
}

pub struct ControlRegister {
    pub base_nametable_address: Address,       // 0-1
    pub vram_address_increment: VramIncrement, // 2
    pub sprite_address: Address,               // 3
    pub background_address: Address,           //4
    pub sprite_size: SpriteSize,               //5
    pub output_to_ext: bool,                   //6
    pub nmi_during_vblank: bool,               // 7
}

impl ControlRegister {
    pub fn new() -> ControlRegister {
        ControlRegister {
            base_nametable_address: Address(0x2000),
            sprite_address: Address(0),
            background_address: Address(0),
            vram_address_increment: VramIncrement::AcrossOne,
            sprite_size: SpriteSize::EightByEight,
            output_to_ext: false,
            nmi_during_vblank: false,
        }
    }

    pub fn from_u8(word: u8) -> Self {
        let mut r = Self::new();
        r.write_register(word);
        r
    }

    pub fn to_u8(&self) -> u8 {
        let mut result: u8 = 0;

        match self.base_nametable_address.to_u16() {
            0x2000 => {}
            0x2400 => {
                result |= 0x01;
            }
            0x2800 => {
                result |= 0x02;
            }
            0x2C00 => {
                result |= 0x03;
            }
            _ => {
                panic!(
                    "invalid nametable address: {:#X}",
                    self.base_nametable_address
                );
            }
        }

        match self.vram_address_increment {
            VramIncrement::DownThirtyTwo => {
                result |= 0b00000100;
            }
            VramIncrement::AcrossOne => {}
        }

        match self.sprite_address.to_u16() {
            0x1000 => {
                result |= 0b00001000;
            }
            0x0000 => {}
            _ => {
                panic!("invalid sprite address: {:#X}", self.background_address);
            }
        }

        match self.background_address.to_u16() {
            0x1000 => {
                result |= 0b00010000;
            }
            0x0000 => {}
            _ => {
                panic!("invalid background address: {:#X}", self.background_address);
            }
        }

        match self.sprite_size {
            SpriteSize::EightBySixteen => {
                result |= 0b00100000;
            }
            SpriteSize::EightByEight => {}
        }

        result |= (self.output_to_ext as u8) << 6;
        result |= (self.nmi_during_vblank as u8) << 7;

        result
    }

    pub fn write_register(&mut self, word: u8) {
        self.base_nametable_address = match word & 0b00000011 {
            0x00 => Address(0x2000),
            0x01 => Address(0x2400),
            0x02 => Address(0x2800),
            0x03 => Address(0x2C00),
            _ => {
                panic!("we should not be here");
            }
        };

        self.vram_address_increment = if (word & 0b00000100) > 0 {
            VramIncrement::DownThirtyTwo
        } else {
            VramIncrement::AcrossOne
        };

        self.sprite_address = if (word & 0b00001000) > 0 {
            Address(0x1000)
        } else {
            Address(0)
        };

        self.background_address = if (word & 0b00010000) > 0 {
            Address(0x1000)
        } else {
            Address(0)
        };

        self.sprite_size = if (word & 0b00100000) > 0 {
            SpriteSize::EightBySixteen
        } else {
            SpriteSize::EightByEight
        };

        self.output_to_ext = (word & 0b01000000) > 0;
        self.nmi_during_vblank = (word & 0b10000000) > 0;
    }
}

impl BitAnd for ControlRegister {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        ControlRegister::from_u8(self.to_u8() & rhs.to_u8())
    }
}

impl BitAnd<u8> for ControlRegister {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Self {
        ControlRegister::from_u8(self.to_u8() & rhs)
    }
}

impl PartialEq for ControlRegister {
    fn eq(&self, rhs: &Self) -> bool {
        self.to_u8() == rhs.to_u8()
    }

    fn ne(&self, rhs: &Self) -> bool {
        self.to_u8() != rhs.to_u8()
    }
}

impl PartialEq<u8> for ControlRegister {
    fn eq(&self, rhs: &u8) -> bool {
        self.to_u8() == *rhs
    }

    fn ne(&self, rhs: &u8) -> bool {
        self.to_u8() != *rhs
    }
}

impl fmt::Debug for ControlRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO Make this more useful
        write!(f, "{:#X}", self.to_u8())
    }
}
