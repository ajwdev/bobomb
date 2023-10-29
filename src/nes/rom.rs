use std::ops::{Index};

const ROM_BANK_SIZE: u16 = 16 * 1024;

const ROM_LOWER_START: u16 = 0x8000;
const ROM_LOWER_END: u16 = ROM_LOWER_START + 0x3FFF;
const ROM_UPPER_START: u16 = ROM_LOWER_START + ROM_BANK_SIZE; // 0xc000
const ROM_UPPER_END: u16 = ROM_UPPER_START + 0x3FFF;

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


pub struct Rom {
    lower_bank: Option<Bank>,
    upper_bank: Option<Bank>,
}

impl Rom {
    pub fn new_double_bank(lower_bank: Bank, upper_bank: Bank) -> Self {
        Rom {
            lower_bank: Some(lower_bank),
            upper_bank: Some(upper_bank),
        }
    }

    pub fn new_single_bank(bank: Bank) -> Self {
        Rom {
            lower_bank: Some(bank),
            upper_bank: None,
        }
    }
}

impl Index<u16> for Rom {
    type Output = u8;

    fn index(&self, addr: u16) -> &u8 {
        match addr {
            ROM_LOWER_START..=ROM_LOWER_END => {    // Lower Bank
                let reladdr: u16 = addr - ROM_LOWER_START;
                &self.lower_bank.as_ref().unwrap()[reladdr as usize]
            }
            ROM_UPPER_START..=ROM_UPPER_END => {    // Upper Bank
                let reladdr: u16 = addr - (ROM_LOWER_START + ROM_BANK_SIZE);
                if self.upper_bank.is_some() {
                    &self.upper_bank.as_ref().unwrap()[reladdr as usize]
                } else {
                    &self.lower_bank.as_ref().unwrap()[reladdr as usize]
                }
            }
            _ => { panic!("outside ROM address range: {:#X}", addr); }
        }
    }
}
