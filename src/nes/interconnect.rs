
use std::cell::{RefCell};

use crate::nes::cpu;
use crate::nes::ppu;
use crate::nes::controller;
use crate::nes::address::{Address};
use crate::nes::rom::{Rom};
use crate::nes::cpu::disassemble::OPCODES;

const SYSTEM_RAM: usize = 2 * 1024;

pub struct Interconnect {
    pub ppu: ppu::Ppu,

    ram: Vec<u8>, // Make this an array at some time. I think it needs boxed
    rom: Rom,
    controller1: RefCell<controller::Controller>,
    controller2: RefCell<controller::Controller>,

    pub dma_in_progress: bool,
    pub dma_write_iteration: u8,
    pub dma_high_byte: u8,

    interrupt: Option<cpu::Interrupt>,
}


impl Interconnect {
    pub fn new(ppu: ppu::Ppu, rom: Rom) -> Self {
        Interconnect {
            ppu,
            rom,
            ram: vec![0; SYSTEM_RAM],
            controller1: RefCell::new(controller::Controller::new()),
            controller2: RefCell::new(controller::Controller::new()),
            dma_in_progress: false,
            dma_write_iteration: 0,
            dma_high_byte: 0,
            interrupt: None,
        }
    }

    /// set_interrupt set the interrupt on the interconnect
    pub fn set_interrupt(&mut self, int: Option<cpu::Interrupt>) {
        self.interrupt = int;
    }

    /// update_interrupt sets the interrupt only if the provided option is Some(_). This is useful
    /// for ensuring a previously set Interrupt is not cleared prematurely.
    pub fn update_interrupt(&mut self, int: Option<cpu::Interrupt>) {
        if int.is_some() {
            self.interrupt = int;
        }
    }

    /// get_interrupt returns the interrupt on the interconnect
    pub fn get_interrupt(&self) -> Option<cpu::Interrupt> {
        self.interrupt
    }

    /// fetch_interrupt returns, and then resets, the interrupt on the interconnect
    pub fn fetch_interrupt(&mut self) -> Option<cpu::Interrupt> {
        let ret = self.interrupt;
        self.interrupt = None;
        ret
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

    // TODO Make this work
    // pub fn read_word<T: Addressable>(&self, addr: T) -> u8 {
    pub fn read_word(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x07ff => {
                self.ram[addr as usize] // Includes zero page, stack, and ram
            }
            0x0800..=0x0fff => {
                self.ram[(addr-0x0800) as usize] // Mirror 1
            }
            0x1000..=0x17ff => {
                self.ram[(addr-0x1000) as usize] // Mirror 2
            }
            0x1800..=0x1fff => {
                self.ram[(addr-0x1800) as usize] // Mirror 3
            }
            // PPU
            0x2002|0x2007 => {
                self.ppu.read_at(addr)
            }
            // Controllers
            0x4016 => {
                {
                    let mut controller = self.controller1.borrow_mut();
                    controller.read() as u8
                }
            }
            0x4017 => {
                {
                    let mut controller = self.controller2.borrow_mut();
                    controller.read() as u8
                }
            }
            // ROM
            0x8000..=0xFFFF => {
                self.rom[addr]
            }
            _ => {
                panic!("unknown address {:#x}", addr);
            }
        }
    }

    pub fn read_range(&self, mut start: u16, count: i16) -> (Vec<u8>, u16, usize) {
        let end = if count >= 0 {
            start + (count as u16)
        } else {
            let e = start+1;
            start = (start + 1) - (-count as u16);
            e
        };

        let mut result: Vec<u8> = Vec::new();
        let mut real_count = 0;
        for i in start..end {
            // XXX This is super inefficient but since this is only used by the debugger I'm not
            // super concerned. Its unlikely that we'll read large amounts of memory where the
            // function call cost will be noticeable.
            result.push(self.read_word(i));
            real_count += 1;
        }

        (result, start, real_count)
    }

    pub fn read_range_by_instruction(&self, start: u16, count: i16) -> (Vec<u8>, u16, usize) {
        let mut addr = start;
        let abs_count = if count > 0 {
            count as usize
        } else {
            addr = (start + 1) - (-count as u16);
            -count as usize
        };

        let mut result: Vec<u8> = Vec::new();
        let mut i = 0;

        while i < abs_count {
            let b0 = self.read_word(addr);
            addr += 1;

            match OPCODES[b0 as usize] {
                Some(op) => {
                    result.push(b0);
                    match op.1.len() {
                        2 => {
                            result.push(self.read_word(addr));
                            addr += 1;
                        }
                        3 => {
                            result.push(self.read_word(addr));
                            addr += 1;
                            result.push(self.read_word(addr));
                            addr += 1;
                        }
                        _ => {}
                    }

                }
                None => break,
            }

            i += 1;
        }

        (result, start, i)
    }

    pub fn write_word(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM
            0x00..=0x07ff => {
                self.ram[addr as usize] = value;
            }
            0x0800..=0x0fff => {
                self.ram[(addr-0x0800) as usize] = value; // Mirror 1
            }
            0x1000..=0x17ff => {
                self.ram[(addr-0x1000) as usize] = value; // Mirror 2
            }
            0x1800..=0x1fff => {
                self.ram[(addr-0x1800) as usize] = value; // Mirror 3
            }
            // PPU Control
            0x2000 => {
                self.ppu.write_register(ppu::PpuRegister::Control, value);
            }
            // PPU Mask
            0x2001 => {
                self.ppu.write_register(ppu::PpuRegister::Mask, value);
            }
            0x2002 => {
                panic!("ppu not implemented yet. write access at {:#x}", addr);
            }
            // PPU OAMADDR
            0x2003 => {
                self.ppu.write_register(ppu::PpuRegister::Oamaddr, value);
            }
            0x2004 => {
                panic!("ppu not implemented yet. write access at {:#x}", addr);
            }
            // PPU Scroll
            0x2005 => {
                self.ppu.write_register(ppu::PpuRegister::Scroll, value);
            }
            // PPU Addr
            0x2006 => {
                self.ppu.write_register(ppu::PpuRegister::Addr, value);
            }
            // PPU Data
            0x2007 => {
                self.ppu.write_register(ppu::PpuRegister::Data, value);
            }
            // PPU
            0x4014 => {
                self.dma_high_byte = value;
            }
            // APU
            0x4015 => {
                // println!("Write APU status not implemented. Skipping");
            }
            // Controllers
            0x4016 => {
                {
                    let mut controller = self.controller1.borrow_mut();
                    controller.write(value);
                }
                {
                    let mut controller = self.controller2.borrow_mut();
                    controller.write(value);
                }
            }
            // APU
            0x4000..=0x4008 => {
                // println!("Write APU thing not implemented. Skipping");
            }
            0x400A..=0x4017 => {
                // println!("Write APU thing not implemented. Skipping");
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
    use crate::nes::ppu::Ppu;
    use crate::nes::rom::{Bank,Rom};

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
    fn test_read_range() {
        let rom = Rom::new_double_bank(Bank::new(&[0; 16384]), Bank::new(&[0; 16384]));
        let ppu = Ppu::new();
        let mut interconnect = Interconnect::new(ppu, rom);

        interconnect.ram[0x0080] = 0xFF;
        interconnect.ram[0x0081] = 0xFF;
        interconnect.ram[0x0082] = 0xFF;
        interconnect.ram[0x0083] = 0xFF;
        interconnect.ram[0x0084] = 0xFF;
        interconnect.ram[0x0085] = 0xFF;
        interconnect.ram[0x0086] = 0xFF;
        interconnect.ram[0x0087] = 0xFF;
        interconnect.ram[0x0088] = 0xFF;
        interconnect.ram[0x0089] = 0xFF;
        interconnect.ram[0x008a] = 0xAA;
        interconnect.ram[0x008b] = 0xAA;
        interconnect.ram[0x008c] = 0xAA;
        interconnect.ram[0x008d] = 0xAA;
        interconnect.ram[0x008e] = 0xAA;
        interconnect.ram[0x008f] = 0xAA;

        let (result, start, count) = interconnect.read_range(0x0080, 10);
        assert!(result.len() == 10, "expected length of 10m got {}", result.len());
        assert!(result.iter().all(|x| *x == 0xFF), "not all elements equal 0xFF: {:?}", &result);
        assert!(start == 0x0080, "starting address is wrong; expect 0x0080, got {}", start);
        assert!(count == 10, "count is wrong; expect 10, got {}", count);

        let (result2, start2, count2) = interconnect.read_range(0x008f, -6);
        assert!(result2.len() == 6, "expected length of 6, got {}", result2.len());
        assert!(result2.iter().all(|x| *x == 0xAA), "not all elements equal 0xAA: {:?}", &result2);
        assert!(start2 == 0x008a, "starting address is wrong; expect 0x008a, got {:#04x}", start2);
        assert!(count2 == 6, "count is wrong; expect 6, got {}", count2);
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
