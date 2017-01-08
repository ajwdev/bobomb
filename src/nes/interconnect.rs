use std::ops::{Index,IndexMut};

use nes::ppu;
use nes::address::{Bank};

const SYSTEM_RAM: usize = 2 * 1024;

// http://wiki.nesdev.com/w/index.php/CPU_memory_map
// These constants ignore mirrors
const SYSTEM_RAM_START: u16 = 0x0;
const SYSTEM_RAM_END: u16 = 0x07FF;

const PPU_REGISTER_SIZE: usize = 8;
const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x2007;

const ROM_BANK_SIZE: u16 = 16 * 1024;

const ROM_LOWER_START: u16 = 0x8000;
const ROM_LOWER_END: u16 = ROM_LOWER_START + 0x3FFF;
const ROM_UPPER_START: u16 = ROM_LOWER_START + ROM_BANK_SIZE; // 0xc000
const ROM_UPPER_END: u16 = ROM_UPPER_START + 0x3FFF;


pub struct Interconnect {
    ram: Vec<u8>, // Make this an array at some time. I think it needs boxed
    ppu: ppu::Ppu,

    // rom: rom::Rom
    lower_rom: Option<Bank>,
    upper_rom: Option<Bank>,

    cycles: usize,
}

// TODO Come back to these. I think it's just going to required moving the code from
// read_word/write_word below and then having those old functions call into this
//
// impl Index<u16> for Interconnect {
//     type Output = u8;

//     fn index(&self, addr: u16) -> &u8 {
//         &self.data[idx]
//     }
// }

// impl IndexMut<u16> for Interconnect {
//     fn index_mut(&mut self, addr: u16) -> &mut u8 {
//         &mut self.data[idx]
//     }
// }

impl Interconnect {
    pub fn new_double_bank(lower_rom: Bank, upper_rom: Bank) -> Self {
        Interconnect {
            ram: vec![0; SYSTEM_RAM],
            ppu: ppu::Ppu::new(),
            lower_rom: Some(lower_rom),
            upper_rom: Some(upper_rom),
            cycles: 0,
        }
    }

    pub fn new_single_bank(rom: Bank) -> Self {
        Interconnect {
            ram: vec![0; SYSTEM_RAM],
            ppu: ppu::Ppu::new(),
            lower_rom: Some(rom),
            upper_rom: None,
            cycles: 0,
        }
    }

    pub fn read_word(&self, addr: u16) -> u8 {
        match addr {
            ROM_LOWER_START...ROM_LOWER_END => {
                // Lower bank
                let reladdr: u16 = addr - ROM_LOWER_START;
                self.lower_rom.as_ref().unwrap()[reladdr as usize]
            }
            ROM_UPPER_START...ROM_UPPER_END => {
                let reladdr: u16 = addr - (ROM_LOWER_START + ROM_BANK_SIZE);
                if self.upper_rom.is_some() {
                    self.upper_rom.as_ref().unwrap()[reladdr as usize]
                } else {
                    self.lower_rom.as_ref().unwrap()[reladdr as usize]
                }
            }
            0x0000...0x07ff => {
                self.ram[addr as usize] // Includes zero page, stack, and ram
            }
            0x0800...0x0fff => {
                self.ram[(addr-0x0800) as usize] // Mirror 1
            }
            0x1000...0x17ff => {
                self.ram[(addr-0x1000) as usize] // Mirror 2
            }
            0x1800...0x1fff => {
                self.ram[(addr-0x1800) as usize] // Mirror 3
            }
            0x2002 => self.ppu.read_at(addr),
            _ => {
                panic!("unknown address {:#x}", addr);
            }
        }
    }

    pub fn write_word(&mut self, addr: u16, value: u8) {
        match addr {
            0x00...0x07ff => {
                self.ram[addr as usize] = value;
            }
            0x0800...0x0fff => {
                self.ram[(addr-0x0800) as usize] = value; // Mirror 1
            }
            0x1000...0x17ff => {
                self.ram[(addr-0x1000) as usize] = value; // Mirror 2
            }
            0x1800...0x1fff => {
                self.ram[(addr-0x1800) as usize] = value; // Mirror 3
            }
            0x2000 => {
                self.ppu.write_register(ppu::PpuRegister::Ctrl, value);
            }
            0x2001 => {
                self.ppu.write_register(ppu::PpuRegister::Mask, value);
            }
            0x2005 => {
                self.ppu.write_register(ppu::PpuRegister::Scroll, value);
            }
            0x2006 => {
                self.ppu.write_register(ppu::PpuRegister::Addr, value);
            }
            0x2007 => {
                self.ppu.write_register(ppu::PpuRegister::Data, value);
            }
            0x2002...0x2004 => {
                // PPU
                // TODO Should we do something similiar to what we did above?
                panic!("ppu not implemented yet. write access at {:#x}", addr);
            }
            0x4015 => {
                println!("Write APU status not implemented. Skipping");
            }
            _ => {
                panic!("unimplemented write address {:#x}", addr);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Interconnect;
    use nes::address::Bank;

    #[test]
    fn test_write_word() {
        // TODO Adjust for every writable section of address space
        let mut mem = Interconnect::new_double_bank(Bank::new(&[0; 16384]), Bank::new(&[0; 16384]));
        let mut result: u8;

        result = mem.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);

        mem.write_word(0x0010, 0xff);
        result = mem.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_read_system_ram() {
        let mut mem = Interconnect::new_double_bank(Bank::new(&[0; 16384]), Bank::new(&[0; 16384]));
        mem.ram[0] = 0xFF;
        mem.ram[0x10] = 0xFF;
        mem.ram[0xa0] = 0xFF;
        mem.ram[0x7ff] = 0xFF;

        assert_eq!(0xFF, mem.read_word(0x00));
        assert_eq!(0xFF, mem.read_word(0x10));
        assert_eq!(0xFF, mem.read_word(0x7ff));
        assert_eq!(0, mem.read_word(0x01));
    }

    #[test]
    fn test_read_rom_single_bank() {
        let mut mock_rom = vec![0; 16*1024];
        mock_rom[0] = 0xFF;
        mock_rom[0x10] = 0xFF;
        mock_rom[0xa0] = 0xFF;
        mock_rom[0x3FFF] = 0xFF;


        let mem = Interconnect::new_single_bank(Bank::new(&mock_rom));
        // Lower bank
        assert_eq!(0xFF, mem.read_word(0x8000));
        assert_eq!(0xFF, mem.read_word(0x8010));
        assert_eq!(0xFF, mem.read_word(0xbfff));
        assert_eq!(0, mem.read_word(0x8001));
        // Upper bank
        assert_eq!(0xFF, mem.read_word(0xc000));
        assert_eq!(0xFF, mem.read_word(0xc010));
        assert_eq!(0xFF, mem.read_word(0xffff));
        assert_eq!(0, mem.read_word(0xc001));
    }

    #[test]
    fn test_read_rom_double_bank() {
        let mut mock_rom = vec![0; 32*1024];
        mock_rom[0] = 0xFF; // beginning of bank
        mock_rom[0x10] = 0xFF;
        mock_rom[0xa0] = 0xFF;
        mock_rom[0x3FFF] = 0xFF; // end of bank

        mock_rom[0x4000] = 0xAA; // beginning of bank
        mock_rom[0x4010] = 0xAA;
        mock_rom[0x40a0] = 0xAA;
        mock_rom[0x7FFF] = 0xAA; // end of bank


        let mem = Interconnect::new_double_bank(Bank::new(&mock_rom[0..16 * 1024]), Bank::new(&mock_rom[16 * 1024..]));
        // Lower bank
        assert_eq!(0xFF, mem.read_word(0x8000));
        assert_eq!(0xFF, mem.read_word(0x8010));
        assert_eq!(0xFF, mem.read_word(0xbfff));
        assert_eq!(0, mem.read_word(0x8001));
        // Upper bank
        assert_eq!(0xAA, mem.read_word(0xc000));
        assert_eq!(0xAA, mem.read_word(0xc010));
        assert_eq!(0xAA, mem.read_word(0xffff));
        assert_eq!(0, mem.read_word(0xc001));
    }
}
