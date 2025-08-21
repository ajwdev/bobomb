use crate::nes::cpu::Interrupt;
use std::cell::Cell;

// TODO This file is a mess. We need to heavily refactor after we're
// confident about the implementation

// See https://wiki.nesdev.com/w/index.php/PPU_scrolling#Register_controls to understand
// the address latch and where bits get shifted into place

mod control;
mod mask;
mod palette;
use crate::nes::ppu::control::{ControlRegister, VramIncrement};
use crate::nes::ppu::mask::MaskRegister;
use crate::nes::ppu::palette::COLORS;

const VRAM_SIZE: usize = 16 * 1024;
const COLUMNS_PER_SCANLINE: usize = 340;
const CYCLES_PER_SCANLINE: usize = 341;
const SCANLINES_PER_FRAME: usize = 262;
const VBLANK_SCANLINE: usize = 241;
const PRERENDER_SCANLINE: usize = 261;

#[derive(Clone, Copy, Debug)]
enum LatchState {
    Low,
    High,
}

impl LatchState {
    fn flip(&mut self) -> Self {
        match self {
            LatchState::Low => LatchState::High,
            LatchState::High => LatchState::Low,
        }
    }
}

// http://wiki.nesdev.com/w/index.php/PPU
// http://wiki.nesdev.com/w/index.php/PPU_programmer_reference
#[derive(Debug, Copy, Clone)]
pub enum PpuRegister {
    Oamaddr, // $2003
    Oamdata, // $2004
    Oamdma,  // $4014
    // In docs, these are prefixed with PPU
    Control, // $2000
    Mask,    // $2001
    Status,  // $2002
    Scroll,  // $2005
    Addr,    // $2006
    Data,    // $2007
}

pub struct PpuStepResult {
    pub should_redraw: bool,
    pub interrupt: Option<Interrupt>,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Ppu {
    // vram: Vec<u8>,
    vram: Box<[u8]>,
    oam: Box<[u8]>,

    // Frame buffers
    pub front: Box<[u32]>,
    back: Box<[u32]>,

    // OAM == Object Attribute Memory. Its for sprites
    oamaddr: u8,
    Oamdata: u8,
    Oamdma: u8,

    control: ControlRegister,
    mask: MaskRegister,
    // Status: u8,
    // Scroll: Cell<u8>,
    // Addr: Cell<u8>,
    Data: u8,

    last_write: u8,

    // Internal ppu registers. See nesdev "loopy" scrolling details. The variable names
    // match up with the docs after the underscore (i.e addr_t == "t", fine_x == "x", etc).
    addr_t: u16,
    addr_v: u16,
    fine_x: u8,
    // These need to be in a cell as they get reset on any reads to status (0x2002)
    latch_w: Cell<LatchState>,
    is_vblank: Cell<bool>,

    // background temps
    next_tile_attrib: u8,
    next_tile_id: u8,
    next_tile_lsb: u8,
    next_tile_msb: u8,
    pattern_lo: u16,
    pattern_hi: u16,
    attrib_lo: u16,
    attrib_hi: u16,

    sprite_overflow: bool,

    // frame_is_even: bool,
    pub cycle: i64,
    pub scanline: i64,
    pub frames: usize,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            // vram: vec![0; VRAM_SIZE],
            vram: Box::new([0; VRAM_SIZE]),
            oam: Box::new([0; 256]),
            oamaddr: 0,
            Oamdata: 0,
            Oamdma: 0,

            front: Box::new([0; 256 * 240]),
            back: Box::new([0; 256 * 240]),

            // https://wiki.nesdev.com/w/index.php/PPU_power_up_state
            control: ControlRegister::new(),
            mask: MaskRegister::new(),
            // Status: 0xa0, // Docs aren't clear if this should be 0x80 or 0xa0 on start
            // Scroll: Cell::new(0),
            // Addr: Cell::new(0),
            Data: 0,

            last_write: 0,

            cycle: 0,
            frames: 0,
            scanline: 0,

            addr_t: 0,
            addr_v: 0,
            latch_w: Cell::new(LatchState::Low),
            is_vblank: Cell::new(false),
            fine_x: 0,

            next_tile_attrib: 0,
            next_tile_id: 0,
            next_tile_lsb: 0,
            next_tile_msb: 0,
            pattern_lo: 0,
            pattern_hi: 0,
            attrib_lo: 0,
            attrib_hi: 0,

            // TODO Might delete these?
            // fine_x_scroll: 0,
            sprite_overflow: false,
            // frame_is_even: true,
            // TODO We should have seperate fields for an NMI being fired and a vblank. They are
            // not technically the same as reading the status register will reset the nmi state.
            // Maybe we just need to rename this?
        }
    }

    pub fn step(&mut self) -> PpuStepResult {
        let mut result = PpuStepResult {
            should_redraw: false,
            interrupt: None,
        };

        if self.scanline >= -1 && self.scanline < 240  {
            if self.scanline == 0 && self.cycle == 0 {
                // skip cycle
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 1 {
                self.is_vblank.set(false);
            }

            if (self.cycle >= 2 && self.cycle < 258) || (self.cycle >= 321 && self.cycle < 338) {
                self.fetch()
            }

            if self.cycle == 256 {
                self.increment_scroll_y();
            }

            if self.cycle == 257 {
                // copy X
                // https://wiki.nesdev.com/w/index.php/PPU_scrolling#At_dot_257_of_each_scanline
                self.load_background_shifters();
                self.copy_scroll_x();
            }

            if self.cycle == 338 || self.cycle == 340 {
                self.next_tile_id = self.ppu_read_at(
                    0x2000 | self.addr_v & 0x0fff
                );
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305 {
                self.copy_scroll_y();
            }
        }

        if self.scanline == 240 {
            // No op
        }

        if self.scanline >= 0 && self.scanline < 240 && self.cycle >= 1 && self.cycle <= 256 {
            self.draw();
        }

        if self.scanline >= 241 && self.scanline < 261 {
            if self.scanline == 241 && self.cycle == 1 {
                if self.control.nmi_during_vblank {
                    result.interrupt = Some(Interrupt::Nmi);
                }

                self.is_vblank.set(true);
                std::mem::swap(&mut self.front, &mut self.back);
                result.should_redraw = true;
            }
        }

        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 261 {
                self.scanline = -1;
                self.frames += 1;
            }
        }

        result


        // Draw some pixels
        // if self.rendering_enabled() {
        //     self.render();
        // }

        // if self.cycle == 1 {
        //     match self.scanline {
        //         VBLANK_SCANLINE => {

        //             if self.control.nmi_during_vblank {
        //                 result.interrupt = Some(Interrupt::Nmi);
        //             }

        //             self.is_vblank.set(true);
        //             std::mem::swap(&mut self.front, &mut self.back);
        //             result.should_redraw = true;
        //         }
        //         PRERENDER_SCANLINE => {
        //             self.is_vblank.set(false);

        //             // TODO reset sprite overflow/zerohit
        //         }
        //         _ => {} // Nothing to do
        //     }
        // }

    }

    fn copy_scroll_x(&mut self) {
        self.addr_v = (self.addr_v & 0xFBE0) | (self.addr_t & 0x041F);
    }

    fn copy_scroll_y(&mut self) {
        self.addr_v = (self.addr_v & 0x841F) | (self.addr_t & 0x7BE0);
    }

    fn increment_scroll_x(&mut self) {
        // https://wiki.nesdev.com/w/index.php/PPU_scrolling#Coarse_X_increment
        if self.addr_v & 0x001f == 31 {
            self.addr_v &= 0xffe0;
            self.addr_v ^= 0x0400;
        } else {
            // increment coarse X
            self.addr_v += 1;
        }
    }

    fn increment_scroll_y(&mut self) {
        // https://wiki.nesdev.com/w/index.php/PPU_scrolling#Y_increment
        if self.addr_v & 0x7000 != 0x7000 {
            self.addr_v += 0x1000;
        } else {
            self.addr_v &= 0x8FFF;
            let mut y = (self.addr_v & 0x03E0) >> 5;

            match y {
                29 => {
                    y = 0;
                    self.addr_v ^= 0x0800;
                }
                31 => {
                    y = 0;
                }
                _ => {
                    y += 1;
                }
            }

            self.addr_v = (self.addr_v & 0xFC1F) | (y << 5);
        }
    }

    fn draw(&mut self) {
        let mut pixel: u8 = 0;
        let mut palette: u8 = 0;

        // Backgrounds first
        if self.mask.show_background {
            let m = 0x8000 >> self.fine_x;

            let p0: u8 = ((self.pattern_lo & m) > 0).into();
            let p1: u8 = ((self.pattern_hi & m) > 0).into();
            pixel = (p1 << 1) | p0;

            let pal0: u8 = ((self.attrib_lo & m) > 0).into();
            let pal1: u8 = ((self.attrib_hi & m) > 0).into();
            palette = (pal1 << 1) | pal0;
        }

        let c = self.ppu_read_at(0x3f00 + ((palette as u16) << 2) + (pixel as u16)) & 0x3f;
        let i = (256 * self.scanline) + self.cycle - 1;
        // dbg!(self.scanline, self.cycle-1);
        self.back[i as usize] = COLORS[c as usize];
    }

    #[inline]
    fn get_fine_y(&self) -> u16 {
        (self.addr_v >> 12) & 0b111
    }

    fn load_background_shifters(&mut self) {
        self.pattern_lo = (self.pattern_lo & 0xff00) | self.next_tile_lsb as u16;
        self.pattern_hi = (self.pattern_hi & 0xff00) | self.next_tile_msb as u16;

        self.attrib_lo = (self.attrib_lo & 0xff00)
            | if self.next_tile_attrib & 0b01 > 0 {
                0xff
            } else {
                0x00
            };
        self.attrib_hi = (self.attrib_hi & 0xff00)
            | if self.next_tile_attrib & 0b10 > 0 {
                0xff
            } else {
                0x00
            };
    }

    fn fetch(&mut self) {
        self.pattern_lo <<= 1;
        self.pattern_hi <<= 1;
        self.attrib_lo <<= 1;
        self.attrib_hi <<= 1;

        let n = (self.cycle - 1) % 8;
        match n {
            0 => {
                // fetch name table
                self.load_background_shifters();

                self.next_tile_id = self.ppu_read_at(0x2000 | (self.addr_v & 0x0FFF));
            }
            2 => {
                // fetch attribute table
                let address = 0x23C0
                    | (self.addr_v & 0x0C00)
                    | ((self.addr_v >> 4) & 0x38)
                    | ((self.addr_v >> 2) & 0x07);

                let shift = ((self.addr_v >> 4) & 4) | (self.addr_v & 2);
                self.next_tile_attrib = ((self.ppu_read_at(address) >> shift) & 3) << 2;
            }
            4 => {
                // fetch low tile
                let address: u16 = self.control.background_address.to_u16()
                    + (self.next_tile_id as u16 * 16)
                    + self.get_fine_y();
                self.next_tile_lsb = self.ppu_read_at(address);
            }
            6 => {
                // fetch high tile
                let address: u16 = self.control.background_address.to_u16()
                    + (self.next_tile_id as u16 * 16)
                    + self.get_fine_y();
                self.next_tile_msb = self.ppu_read_at(address + 8);
            }
            7 => {
                self.increment_scroll_x();
            }
            // _ => panic!("fetch on non-fetch: cycle {}; arm {}", self.cycle, n)
            _ => {}
        }
    }

    fn ppu_read_at(&self, address: u16) -> u8 {
        let addr = address % 0x4000;

        match addr {
            // Pattern tables (CHR-ROM)
            0x0000..=0x1fff => {
                if let Some(ref chr_rom) = self.chr_rom {
                    chr_rom[addr as usize]
                } else {
                    0 // Return 0 if no CHR-ROM loaded
                }
            }
            // Name tables
            0x2000..=0x3eff => {
                let mirrored_addr = 0x2000 + ((addr - 0x2000) % 0x1000);
                self.vram[mirrored_addr as usize]
            }
            // Palettes
            0x3f00..=0x3fff => {
                let palette_addr = 0x3f00 + ((addr - 0x3f00) % 0x20);
                // Handle palette mirroring: $3F10/$3F14/$3F18/$3F1C mirror $3F00/$3F04/$3F08/$3F0C
                let final_addr = if palette_addr >= 0x3f10 && (palette_addr % 4) == 0 {
                    palette_addr - 0x10
                } else {
                    palette_addr
                };
                self.vram[final_addr as usize]
            }
            _ => 0,
        }
    }

    pub fn read_at(&self, address: u16) -> u8 {
        match address {
            0x2002 => {
                // Status
                let mut result = self.last_write & 0x1f;

                if self.is_vblank.get() {
                    result |= 1 << 7
                }
                if self.sprite_overflow {
                    result |= 1 << 5
                }
                // self.addr_v = 0;
                self.is_vblank.set(false);
                self.latch_w.set(LatchState::Low);

                result
            }
            0x2007 => {
                // Data
                panic!(
                    "ppu data read not implemented yet. access at {:#x}",
                    address
                );
            }
            _ => {
                panic!("ppu not implemented yet. access at {:#x}", address);
            }
        }
    }

    pub fn write_register(&mut self, reg: PpuRegister, value: u8) {
        // println!("PPU write at {:?} : {:#X}", reg, value);
        self.last_write = value;
        match reg {
            //0x2000
            PpuRegister::Control => {
                // TODO I think this can affect the nmi state
                self.control.write_register(value);

                let t: u16 = (value as u16 & 0x03) << 10;
                self.addr_t = (self.addr_t & 0xf3ff) | t;
            }
            // 0x2001
            PpuRegister::Mask => {
                self.mask.write_register(value);
            }
            // 0x2003
            PpuRegister::Oamaddr => self.oamaddr = value,
            // 0x2005
            PpuRegister::Scroll => {
                match self.latch_w.get() {
                    LatchState::Low => {
                        let addr_dest: u16 = value as u16 >> 3;
                        self.addr_t = (self.addr_t & 0xffe0) | addr_dest;

                        self.fine_x = value & 0x07;
                    }
                    LatchState::High => {
                        // TODO Is this right?
                        let mut addr_dest: u16 = (value as u16 & 0x07) << 12;
                        self.addr_t = (self.addr_t & 0x8fff) | addr_dest;

                        addr_dest = (value as u16 & 0xf8) << 2;
                        self.addr_t = (self.addr_t & 0xfc1f) | addr_dest;
                    }
                }

                self.flip_latch();
            }
            // 0x2006
            PpuRegister::Addr => {
                match self.latch_w.get() {
                    LatchState::Low => {
                        let dest = ((value as u16) & 0x3f) << 8;
                        // TODO check 0x80ff vs 0x00ff. olcNes has it as 00FF
                        // self.addr_t = (self.addr_t & 0x80ff) | dest;
                        self.addr_t = (self.addr_t & 0x00ff) | dest;
                    }
                    LatchState::High => {
                        self.addr_t = (self.addr_t & 0xff00) | value as u16;
                        self.addr_v = self.addr_t;
                    }
                }
                self.flip_latch();
            }
            // 0x2007
            PpuRegister::Data => {
                // PPU RAM is just mirroed starting at 0x4000
                let idx = (self.addr_v % 0x4000) as usize;
                self.vram[idx] = value;

                match self.control.vram_address_increment {
                    VramIncrement::AcrossOne => self.addr_v += 1,
                    VramIncrement::DownThirtyTwo => self.addr_v += 32,
                }
            }
            _ => {
                panic!("PPU register {:?} is not implemented", reg);
            }
        }
    }

    fn flip_latch(&mut self) {
        self.latch_w.set(self.latch_w.get().flip());
    }

    #[inline]
    fn rendering_enabled(&self) -> bool {
        return self.mask.show_background || self.mask.show_sprites;
    }

    pub fn write_dma(&mut self, word: u8) {
        self.last_write = word;
        self.oam[self.oamaddr as usize] = word;
        self.oamaddr += 1
    }
}
