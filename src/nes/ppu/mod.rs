use nes::address::Address;
use nes::cpu::Interrupt;

mod control;
use nes::ppu::control::{ControlRegister,VramIncrement};

const VRAM_SIZE: usize = 16 * 1024;
const COLUMNS_PER_SCANLINE: usize = 340;
const CYCLES_PER_SCANLINE: usize = 341;
const SCANLINES_PER_FRAME: usize = 262;
const VBLANK_SCANLINE: usize = 241;

// http://wiki.nesdev.com/w/index.php/PPU
// http://wiki.nesdev.com/w/index.php/PPU_programmer_reference
#[derive(Debug,Copy,Clone)]
pub enum PpuRegister {
    Oamaddr, // $2003
    Oamdata, // $2004
    Oamdma, // $4014
    // In docs, these are prefixed with PPU
    Control, // $2000
    Mask, // $2001
    Status, // $2002
    Scroll, // $2005
    Addr, // $2006
    Data, // $2007
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,
    oam: Vec<u8>,

    // OAM == Object Attribute Memory. Its for sprites
    oamaddr: u8,
    Oamdata: u8,
    Oamdma: u8,

    control: ControlRegister,
    Mask: u8,
    Status: u8,
    Scroll: u8,
    Addr: u8,
    Data: u8,

    // PPUADDR
    vram_address: u16,
    addr_latch_first_write_done: bool,

    frame_is_even: bool,
    is_vblank: bool,

    cycles: usize,
    frames: usize,
    scanline: usize,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: vec![0; VRAM_SIZE],
            oam: vec![0; 256],
            oamaddr: 0,
            Oamdata: 0,
            Oamdma: 0,

            // https://wiki.nesdev.com/w/index.php/PPU_power_up_state
            control: ControlRegister::new(),
            Mask: 0,
            Status: 0xa0, // Docs aren't clear if this should be 0x80 or 0xa0 on start
            Scroll: 0,
            Addr: 0,
            Data: 0,

            cycles: 0,
            frames: 0,
            scanline: 0,

            vram_address: 0,
            addr_latch_first_write_done: false,
            frame_is_even: true,
            is_vblank: false,
        }
    }

    pub fn step(&mut self) -> Option<Interrupt> {
        let mut intr = None;

        if self.cycles == (COLUMNS_PER_SCANLINE * SCANLINES_PER_FRAME) {
            println!("PPU cycles resset");
            self.cycles = 0;
            return None;
        } else if self.cycles == ((CYCLES_PER_SCANLINE * VBLANK_SCANLINE) + 1) {
            if self.control.nmi_during_vblank {
                println!("PPU NMI!!!");
                intr = Some(Interrupt::Nmi);
            }

            self.is_vblank = true;
        } else if self.cycles == ((CYCLES_PER_SCANLINE * 261) + 1) {
            println!("PPU vblank cleared");
            self.is_vblank = false;
        }

        self.cycles += 1;
        intr
    }

    pub fn read_at(&self, address: u16) -> u8 {
        match address {
            0x2002 => self.Status,
            _ => {
                panic!("ppu not implemented yet. access at {:#x}", address);
            }
        }
    }

    // TODO Replace these methods with something less terrible
    fn write_reg_scroll(&mut self, value: u8) {
        self.Scroll = value;
    }

    fn write_reg_mask(&mut self, value: u8) {
        self.Mask = value;
    }

    fn write_reg_addr(&mut self, value: u8) {
        // XXX I technically think writes/reads to the data register reset this registers latch but
        // this implementation seems easier. Keep an eye out for bugs.

        if !self.addr_latch_first_write_done {
            self.vram_address = 0; // Clear it first

            self.vram_address = ((value as u16) << 8) & 0xFF00 ;
            self.addr_latch_first_write_done = true;
        } else {
            self.vram_address |= (value as u16) & 0x00FF;
            self.addr_latch_first_write_done = false;
        }
    }

    #[inline]
    fn increment_vram_address(&mut self) {
        match self.control.vram_address_increment {
            VramIncrement::AcrossOne => { self.vram_address += 1 },
            VramIncrement::DownThirtyTwo => { self.vram_address += 32 },
        }
    }

    fn write_reg_data(&mut self, value: u8) {
        match self.vram_address {
            0x00...0x3fff => {
                self.vram[self.vram_address as usize] = value;
            }
            0x4000...0x0ffff => {
                let new_addr = (self.vram_address - 0x4000) as usize;
                self.vram[new_addr] = value;
            }
            _ => { panic!("out of range write to video ram: 0x{:#X}", self.vram_address); }
        }

        self.increment_vram_address();
    }

    pub fn write_dma(&mut self, word: u8) {
        self.oam[self.oamaddr as usize] = word;
        self.oamaddr += 1
    }

    pub fn write_register(&mut self, reg: PpuRegister, value: u8) {
        match reg {
            PpuRegister::Addr => { self.write_reg_addr(value) }
            PpuRegister::Scroll => { self.write_reg_scroll(value) }
            PpuRegister::Mask => { self.write_reg_mask(value) }
            PpuRegister::Control => { self.control.write_register(value) }
            PpuRegister::Data => { self.write_reg_data(value) }
            PpuRegister::Oamaddr => { self.oamaddr = value }
            _ => { panic!("PPU register {:?} is not implemented", reg); }
        }
    }
}
