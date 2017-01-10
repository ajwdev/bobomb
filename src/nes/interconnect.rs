use std::ops::{Index,IndexMut};

use nes::ppu;
use nes::address::Address;
use nes::rom::{Rom,Bank};

const SYSTEM_RAM: usize = 2 * 1024;

pub struct Interconnect {
    ram: Vec<u8>, // Make this an array at some time. I think it needs boxed
    rom: Rom,
    ppu: ppu::Ppu,

    cycles: usize,
}


impl Interconnect {
    pub fn new(ppu: ppu::Ppu, rom: Rom) -> Self {
        Interconnect {
            ram: vec![0; SYSTEM_RAM],
            rom: rom,
            ppu: ppu,
            cycles: 0,
        }
    }

    pub fn find_reset_vector_address(&self) -> Address {
        Address::new(self.read_word(0xFFFD), self.read_word(0xFFFC))
    }

    pub fn find_nmi_vector_address(&self) -> Address {
        Address::new(self.read_word(0xFFFB), self.read_word(0xFFFA))
    }

    pub fn find_irq_vector_address(&self) -> Address {
        Address::new(self.read_word(0xFFFF), self.read_word(0xFFFE))
    }

    pub fn read_word(&self, addr: u16) -> u8 {
        match addr {
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
            // PPU
            0x2002 => {
                self.ppu.read_at(addr)
            }
            // ROM
            0x8000...0xFFFF => {
                self.rom[addr]
            }
            _ => {
                panic!("unknown address {:#x}", addr);
            }
        }
    }

    pub fn write_word(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM
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
            // PPU
            0x2000 => {
                println!("PPU Status Write: {:#X}", value);
                self.ppu.write_register(ppu::PpuRegister::Control, value);
            }
            0x2001 => {
                self.ppu.write_register(ppu::PpuRegister::Mask, value);
            }
            0x2002...0x2004 => {
                panic!("ppu not implemented yet. write access at {:#x}", addr);
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
            // APU
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
    use nes::ppu::Ppu;
    use nes::rom::{Bank,Rom};

    #[test]
    fn test_write_word() {
        let rom = Rom::new_double_bank(Bank::new(&[0; 16384]), Bank::new(&[0; 16384]));
        let ppu = Ppu::new();
        let mut interconnect = Interconnect::new(ppu, rom);
        let mut result: u8;

        result = interconnect.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);

        interconnect.write_word(0x0010, 0xff);
        result = interconnect.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_read_system_ram() {
        let rom = Rom::new_double_bank(Bank::new(&[0; 16384]), Bank::new(&[0; 16384]));
        let ppu = Ppu::new();
        let mut interconnect = Interconnect::new(ppu, rom);
        interconnect.ram[0] = 0xFF;
        interconnect.ram[0x10] = 0xFF;
        interconnect.ram[0xa0] = 0xFF;
        interconnect.ram[0x7ff] = 0xFF;

        assert_eq!(0xFF, interconnect.read_word(0x00));
        assert_eq!(0xFF, interconnect.read_word(0x10));
        assert_eq!(0xFF, interconnect.read_word(0x7ff));
        assert_eq!(0, interconnect.read_word(0x01));
    }

    #[test]
    fn test_find_reset_vector_address() {
        let mut mock_rom = vec![0; 1024*16];
        mock_rom[0x3ffc] = 0xef;
        mock_rom[0x3ffd] = 0xbe;

        let rom = Rom::new_single_bank(Bank::new(&mock_rom));
        let ppu = Ppu::new();
        let interconnect = Interconnect::new(ppu, rom);
        let result = interconnect.find_reset_vector_address();
        assert!(result.to_u16() == 0xbeef, "expected 0xbeef, got: {:#x}", result.to_u16());
    }

    #[test]
    fn test_read_rom_single_bank() {
        let mut mock_rom = vec![0; 16*1024];
        mock_rom[0] = 0xFF;
        mock_rom[0x10] = 0xFF;
        mock_rom[0xa0] = 0xFF;
        mock_rom[0x3FFF] = 0xFF;


        let rom = Rom::new_single_bank(Bank::new(&mock_rom));
        let ppu = Ppu::new();
        let mut interconnect = Interconnect::new(ppu, rom);
        // Lower bank
        assert_eq!(0xFF, interconnect.read_word(0x8000));
        assert_eq!(0xFF, interconnect.read_word(0x8010));
        assert_eq!(0xFF, interconnect.read_word(0xbfff));
        assert_eq!(0, interconnect.read_word(0x8001));
        // Upper bank
        assert_eq!(0xFF, interconnect.read_word(0xc000));
        assert_eq!(0xFF, interconnect.read_word(0xc010));
        assert_eq!(0xFF, interconnect.read_word(0xffff));
        assert_eq!(0, interconnect.read_word(0xc001));
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


        let rom = Rom::new_double_bank(Bank::new(&mock_rom[0..16 * 1024]), Bank::new(&mock_rom[16 * 1024..]));
        let ppu = Ppu::new();
        let mut interconnect = Interconnect::new(ppu, rom);
        // Lower bank
        assert_eq!(0xFF, interconnect.read_word(0x8000));
        assert_eq!(0xFF, interconnect.read_word(0x8010));
        assert_eq!(0xFF, interconnect.read_word(0xbfff));
        assert_eq!(0, interconnect.read_word(0x8001));
        // Upper bank
        assert_eq!(0xAA, interconnect.read_word(0xc000));
        assert_eq!(0xAA, interconnect.read_word(0xc010));
        assert_eq!(0xAA, interconnect.read_word(0xffff));
        assert_eq!(0, interconnect.read_word(0xc001));
    }
}
