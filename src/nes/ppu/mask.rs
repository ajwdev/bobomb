
use std::cmp::PartialEq;
use std::ops::BitAnd;

#[derive(Default,Debug)]
pub struct MaskRegister {
    pub grayscale: bool,
    pub show_left_background: bool,
    pub show_left_sprites: bool,
    pub show_background: bool,
    pub show_sprites: bool,
    pub red_tint: bool,
    pub green_tint: bool,
    pub blue_tint: bool,
}

impl MaskRegister {
    pub fn new() -> MaskRegister {
        MaskRegister {
            ..Default::default()
        }
    }

    pub fn from_u8(word: u8) -> Self {
        let mut r = Self::new();
        r.write_register(word);
        r
    }

    pub fn to_u8(&self) -> u8 {
        let mut result: u8 = 0;

        if self.grayscale {
            result |= 1;
        }

        if self.show_left_background {
            result |= 1 << 1;
        }

        if self.show_left_sprites {
            result |= 1 << 2;
        }

        if self.show_background {
            result |= 1 << 3;
        }

        if self.show_sprites {
            result |= 1 << 4;
        }

        if self.red_tint {
            result |= 1 << 5;
        }

        if self.green_tint {
            result |= 1 << 6;
        }

        if self.blue_tint {
            result |= 1 << 7;
        }

        result
    }

    pub fn write_register(&mut self, word: u8) {
        self.grayscale = (word & 1) > 0;
        self.show_left_background = ((word >> 1) & 1) > 0;
        self.show_left_sprites = ((word >> 2) & 1) > 0;
        self.show_background = ((word >> 3) & 1) > 0;
        self.show_sprites = ((word >> 4) & 1) > 0;
        self.red_tint = ((word >> 5) & 1) > 0;
        self.green_tint = ((word >> 6) & 1) > 0;
        self.blue_tint = ((word >> 7) & 1) > 0;
    }
}

impl BitAnd for MaskRegister {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        MaskRegister::from_u8(self.to_u8() & rhs.to_u8())
    }
}

impl BitAnd<u8> for MaskRegister {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Self {
        MaskRegister::from_u8(self.to_u8() & rhs)
    }
}

impl PartialEq for MaskRegister {
    fn eq(&self, rhs: &Self) -> bool {
        self.to_u8() == rhs.to_u8()
    }

    fn ne(&self, rhs: &Self) -> bool {
        self.to_u8() != rhs.to_u8()
    }
}

impl PartialEq<u8> for MaskRegister {
    fn eq(&self, rhs: &u8) -> bool {
        self.to_u8() == *rhs
    }

    fn ne(&self, rhs: &u8) -> bool {
        self.to_u8() != *rhs
    }
}

// impl fmt::Debug for MaskRegister {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:#X}", self.to_u8())
//     }
// }
