use nes::address::Address;
use nes::cpu::Interrupt;
use std::cell::Cell;

// TODO This file is a mess. We need to heavily refactor after we're
// confident about the implementation

// See https://wiki.nesdev.com/w/index.php/PPU_scrolling#Register_controls to understand
// the address latch and where bits get shifted into place

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
    // Status: u8,
    // Scroll: Cell<u8>,
    // Addr: Cell<u8>,
    Data: u8,

    last_write: u8,

    // PPUADDR
    tmp_vram_address: Cell<u16>,
    vram_address: Cell<u16>,
    addr_latch_first_write_done: Cell<bool>,
    sprite_overflow: Cell<bool>,

    // frame_is_even: bool,
    is_vblank: Cell<bool>,

    fine_x_scroll: u8,

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
            // Status: 0xa0, // Docs aren't clear if this should be 0x80 or 0xa0 on start
            // Scroll: Cell::new(0),
            // Addr: Cell::new(0),
            Data: 0,

            last_write: 0,

            cycles: 340,
            frames: 0,
            scanline: 240,

            tmp_vram_address: Cell::new(0),
            vram_address: Cell::new(0),
            fine_x_scroll: 0,
            sprite_overflow: Cell::new(false),
            addr_latch_first_write_done: Cell::new(false),
            // frame_is_even: true,
            // TODO We should have seperate fields for an NMI being fired and a vblank. They are
            // not technically the same as reading the status register will reset the nmi state.
            // Maybe we just need to rename this?
            is_vblank: Cell::new(false),
        }
    }

    pub fn step(&mut self) -> Option<Interrupt> {
        let mut intr = None;

        if self.cycles == (COLUMNS_PER_SCANLINE * SCANLINES_PER_FRAME) {
            self.cycles = 0;
            return None;
        } else if self.cycles == ((CYCLES_PER_SCANLINE * VBLANK_SCANLINE) + 1) {
            probe!(vblank, begin);

            if self.control.nmi_during_vblank {
                probe!(interrupt, nmi);
                intr = Some(Interrupt::Nmi);
            }

            self.is_vblank.set(true);
        } else if self.cycles == ((CYCLES_PER_SCANLINE * 261) + 1) {
            probe!(vblank, end);

            self.is_vblank.set(false);
        }

        self.cycles += 1;
        intr
    }

    pub fn read_at(&self, address: u16) -> u8 {
        match address {
            0x2002 => { // Status
                let mut result = self.last_write & 0x1f;

                if self.is_vblank.get() {
                    println!("reset vblank through status");
                    result |= 1 << 7
                }
                if self.sprite_overflow.get() {
                    result |= 1 << 5
                }
                self.is_vblank.set(false);
                self.vram_address.set(0);
                self.addr_latch_first_write_done.set(false);

                result
            }
            _ => {
                panic!("ppu not implemented yet. access at {:#x}", address);
            }
        }
    }

    fn write_reg_mask(&mut self, value: u8) {
        self.Mask = value;
    }

    fn write_reg_scroll(&mut self, value: u8) {
        if !self.addr_latch_first_write_done.get() {
            // self.tmp_vram_address.set(0); // Clear it first
            self.fine_x_scroll = value & 0x07;

            let addr_dest = (value >> 3) as u16;
            *self.tmp_vram_address.get_mut() =
                (self.tmp_vram_address.get() & 0xffe0) | addr_dest;

            self.addr_latch_first_write_done.set(true);
        } else {
            let mut addr_dest = ((value as u16) & 0x07) << 12;
            *self.tmp_vram_address.get_mut() =
                (self.tmp_vram_address.get() & 0x8fff) | addr_dest;

            addr_dest = ((value as u16) & 0xf8) << 2;
            *self.tmp_vram_address.get_mut() =
                (self.tmp_vram_address.get() & 0xfc1f) | addr_dest;

            self.addr_latch_first_write_done.set(false);
        }

    }

    fn write_reg_addr(&mut self, value: u8) {
        // XXX I technically think writes/reads to the data register reset this registers latch but
        // this implementation seems easier. Keep an eye out for bugs.

        if !self.addr_latch_first_write_done.get() {
            // self.tmp_vram_address.set(0); // Clear it first

            let dest = ((value as u16) & 0x3f) << 8;
            *self.tmp_vram_address.get_mut() =
                (self.tmp_vram_address.get() & 0x80ff) | dest;

            self.addr_latch_first_write_done.set(true);
        } else {
            *self.tmp_vram_address.get_mut() |= (value as u16);

            self.addr_latch_first_write_done.set(false);

            self.vram_address.set(self.tmp_vram_address.get());
        }
    }

    #[inline]
    fn increment_vram_address(&mut self) {
        match self.control.vram_address_increment {
            VramIncrement::AcrossOne => { *self.vram_address.get_mut() += 1 },
            VramIncrement::DownThirtyTwo => { *self.vram_address.get_mut() += 32 },
        }
    }

    fn write_reg_data(&mut self, value: u8) {
        // PPU RAM is just mirroed starting at 0x4000
        let idx = (self.vram_address.get() % 0x4000) as usize;
        self.vram[idx] = value;

        self.increment_vram_address();
    }

    pub fn write_dma(&mut self, word: u8) {
        self.last_write = word;
        self.oam[self.oamaddr as usize] = word;
        self.oamaddr += 1
    }

    #[inline]
    fn write_nametable_into_addr_latch(&mut self, value: u8) {
        let dest = ((value & 0x03) as u16) << 10;
        *self.tmp_vram_address.get_mut() |=
            (self.tmp_vram_address.get() & 0xf3ff) | dest;
    }

    pub fn write_register(&mut self, reg: PpuRegister, value: u8) {
        // println!("PPU write at {:?} : {:#X}", reg, value);
        self.last_write = value;
        match reg {
            PpuRegister::Addr => { self.write_reg_addr(value) }
            PpuRegister::Scroll => { self.write_reg_scroll(value) }
            PpuRegister::Mask => { self.write_reg_mask(value) }
            //0x2000
            PpuRegister::Control => {
                self.control.write_register(value);
                self.write_nametable_into_addr_latch(value);
            }
            PpuRegister::Data => { self.write_reg_data(value) } // 0x2007
            PpuRegister::Oamaddr => { self.oamaddr = value }
            _ => { panic!("PPU register {:?} is not implemented", reg); }
        }
    }
}
