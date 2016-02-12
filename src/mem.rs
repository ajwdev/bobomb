const SYSTEM_RAM: usize = 2 * 1024;
const PPU_SIZE: usize = 8;

// http://wiki.nesdev.com/w/index.php/CPU_memory_map
// These constants ignore mirrors
const SYSTEM_RAM_START: u16 = 0x0;
const SYSTEM_RAM_END: u16 = 0x07FF;

const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x2007;

const ROM_BANK_SIZE: u16 = 16 * 1024;

const ROM_LOWER_START: u16 = 0x8000;
const ROM_LOWER_END: u16 = ROM_LOWER_START + 0x3FFF;
const ROM_UPPER_START: u16 = 0xc000;
const ROM_UPPER_END: u16 = ROM_UPPER_START + 0x3FFF;


// TODO Should we call this address space?
pub struct Memory<'a> {
    // TODO Look at this again once lifetimes make more sense
    lower_rom: &'a [u8],
    upper_rom: &'a [u8],

    // TODO Look into a Cell<T> here
    ram: Vec<u8>,
    ppu: Vec<u8>,
}

impl<'a> Memory<'a> {
    pub fn new(lower_rom: &'a [u8], upper_rom: &'a [u8]) -> Self {
        Memory {
            ram: vec![0; SYSTEM_RAM],
            ppu: vec![0; PPU_SIZE],
            lower_rom: lower_rom,
            upper_rom: upper_rom,
        }
    }

    pub fn read_word(&self, addr: u16) -> u8 {
        match addr {
            ROM_LOWER_START...ROM_LOWER_END => {
                // Lower bank
                let reladdr: u16 = (addr - ROM_LOWER_START);
                return self.lower_rom[reladdr as usize];
            }
            ROM_UPPER_START...ROM_UPPER_END => {
                let reladdr: u16 = addr - (ROM_LOWER_START + ROM_BANK_SIZE);
                return self.upper_rom[reladdr as usize];
            }
            // TODO Review this as RAM technically starts 0x0200
            0x00...0x07ff => {
                self.ram[addr as usize]
            }
            _ => {
                panic!("unknown address {:#x}", addr);
            }
        }
    }

    pub fn write_word(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000...0x2007 => {    // PPU
                let offset: usize = (addr - 0x2000) as usize;
                self.ppu[offset] = value;
            }
            _ => { panic!("unimplemented write address {:#x}", addr); }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Memory;

    #[test]
    fn test_read_system_ram() {
        let faux_rom = vec![0; 1024*16];
        let mut mem = Memory::new(&faux_rom, &faux_rom);
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
        let mut faux_rom = vec![0; 16*1024];
        faux_rom[0]      = 0xFF;
        faux_rom[0x10]   = 0xFF;
        faux_rom[0xa0]   = 0xFF;
        faux_rom[0x3FFF] = 0xFF;


        let mem = Memory::new(&faux_rom, &faux_rom);
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
        let mut faux_rom = vec![0; 32*1024];
        faux_rom[0]       = 0xFF;
        faux_rom[0x10]   = 0xFF;
        faux_rom[0xa0]   = 0xFF;
        faux_rom[0x3FFF] = 0xFF;

        faux_rom[0x4000] = 0xAA;
        faux_rom[0x4010] = 0xAA;
        faux_rom[0x40a0] = 0xAA;
        faux_rom[0x7FFF] = 0xAA;


        let mem = Memory::new(&faux_rom[0..16*1024], &faux_rom[16*1024..]);
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
    
    #[test]
    #[ignore]
    fn test_write_word() {
        panic!("implement");
    }
}
