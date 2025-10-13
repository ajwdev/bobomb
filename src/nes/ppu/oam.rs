use super::palette;

#[derive(Debug, PartialEq)]
pub enum OamPriority {
    Foreground,
    BehindBackground,
}

#[derive(Debug, PartialEq)]
pub(crate) struct OamAttributes {
    pub palette: u8,
    pub priority: OamPriority,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl OamAttributes {
    pub fn new(
        palette: u8,
        priority: OamPriority,
        flip_horizontal: bool,
        flip_vertical: bool,
    ) -> Self {
        OamAttributes {
            palette,
            priority,
            flip_horizontal,
            flip_vertical,
        }
    }

    pub fn from_u8(word: u8) -> Self {
        OamAttributes {
            palette: word & 0x03,
            priority: if (word & (1 << 5)) == 0 {
                OamPriority::Foreground
            } else {
                OamPriority::BehindBackground
            },
            flip_horizontal: (word & (1 << 6)) != 0,
            flip_vertical: (word & (1 << 7)) != 0,
        }
    }
}

impl From<u8> for OamAttributes {
    fn from(t: u8) -> OamAttributes {
        OamAttributes::from_u8(t)
    }
}
//
// impl From<OamAttributes> for u8 {
//     fn from(t: StatusRegister) -> u8 {
//         t.to_u8()
//     }
// }

#[derive(Debug, PartialEq)]
pub(crate) struct OamEntry {
    pub pos_y: u8,
    pub tile: u8,
    pub attrs: OamAttributes,
    pub pos_x: u8,
}

impl OamEntry {
    pub fn new(pos_y: u8, tile: u8, attrs: OamAttributes, pos_x: u8) -> Self {
        // TODO Add bounds checking and panics
        OamEntry {
            pos_y,
            tile,
            attrs,
            pos_x,
        }
    }

    pub fn from_bytes(input: &[u8]) -> Self {
        let bytes: &[u8; 4] = input
            .try_into()
            .expect("OAM entry requires exactly 4 bytes");

        bytes.into()
    }
}

impl From<&[u8; 4]> for OamEntry {
    fn from(t: &[u8; 4]) -> Self {
        Self::new(t[0], t[1], t[2].into(), t[3])
    }
}

#[derive(Debug)]
pub(crate) struct Oam<'a> {
    mem: &'a [u8; 256],
}

impl<'a> Oam<'a> {
    pub fn new(mem: &'a [u8; 256]) -> Oam<'a> {
        Oam { mem }
    }

    pub fn from_bytes(mem: &'a [u8]) -> Oam<'a> {
        let array: &[u8; 256] = mem
            .try_into()
            .expect("OAM requires exactly 256 bytes of memory");

        Oam { mem: array }
    }

    pub fn iter(&self) -> OamIter<'a> {
        OamIter {
            mem: self.mem,
            idx: 0,
        }
    }
}

pub(crate) struct OamIter<'a> {
    mem: &'a [u8; 256],
    idx: usize,
}

impl<'a> Iterator for OamIter<'a> {
    type Item = OamEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx * 4;
        if i > 255 {
            return None;
        }

        self.idx = i;
        Some(OamEntry::from_bytes(&self.mem[i..i + 4]))
    }
}
