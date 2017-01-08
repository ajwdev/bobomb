const VRAM_SIZE: usize = 16 * 1024;

// http://wiki.nesdev.com/w/index.php/PPU
// http://wiki.nesdev.com/w/index.php/PPU_programmer_reference
#[derive(Debug,Copy,Clone)]
pub enum PpuRegister {
    Oamaddr, // $2003
    Oamdata, // $2004
    Oamdma, // $4014
    // In docs, these are prefixed with PPU
    Ctrl, // $2000
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

    Oamaddr: u8,
    Oamdata: u8,
    Oamdma: u8,

    Ctrl: u8,
    Mask: u8,
    Status: u8,
    Scroll: u8,
    Addr: u8,
    Data: u8,

    // PPUADDR
    vram_address: u16,
    addr_latch_first_write_done: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: vec![0; VRAM_SIZE],
            Oamaddr: 0,
            Oamdata: 0,
            Oamdma: 0,

            // https://wiki.nesdev.com/w/index.php/PPU_power_up_state
            Ctrl: 0,
            Mask: 0,
            Status: 0xa0, // Docs aren't clear if this should be 0x80 or 0xa0 on start
            Scroll: 0,
            Addr: 0,
            Data: 0,

            vram_address: 0,
            addr_latch_first_write_done: false,
        }
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

    fn write_reg_ctrl(&mut self, value: u8) {
        self.Ctrl = value;
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

        println!("VRAM address: {:#x}", self.vram_address);
    }

    #[inline]
    fn increment_vram_address(&mut self) {
        // If $2000:2 is high, increment by 32
        if self.Ctrl & 0b00000100 == 0 {
            self.vram_address += 1
        } else {
            self.vram_address += 32
        }
    }

    fn write_reg_data(&mut self, value: u8) {
        println!("VRAM write: {:#x}", self.vram_address);
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

    pub fn write_register(&mut self, reg: PpuRegister, value: u8) {
        match reg {
            PpuRegister::Addr => { self.write_reg_addr(value) }
            PpuRegister::Scroll => { self.write_reg_scroll(value) }
            PpuRegister::Mask => { self.write_reg_mask(value) }
            PpuRegister::Ctrl => { self.write_reg_ctrl(value) }
            PpuRegister::Data => { self.write_reg_data(value) }
            _ => { panic!("PPU register {:?} is not implemented", reg); }
        }
    }
}
