use std::ops::{Index,IndexMut};

pub const ROM_BANK_SIZE: u16 = 16 * 1024;

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
