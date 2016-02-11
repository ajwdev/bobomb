const SYSTEM_RAM: usize = 2 * 1024;
const PPU_SIZE: usize = 8;

// http://wiki.nesdev.com/w/index.php/CPU_memory_map
// These constants ignore mirrors
const SYSTEM_RAM_START: u16 = 0x0;
const SYSTEM_RAM_END: u16 = 0x07FF;

const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x2007;

const CARTRIDE_START: u16 = 0x4020;
const CARTRIDE_END: u16 = 0xFFFF;

// TODO Should we call this address space?
pub struct Memory {
    ram: Vec<u8>,
    rom: Vec<u8>,
    ppu: Vec<u8>,
}

pub enum Bank {
    SystemRam,
    LowerRom,
    UpperRom,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Self {
        Memory {
            ram: vec![0; SYSTEM_RAM],
            rom: rom,
            ppu: vec![0; PPU_SIZE],
        }
    }

    pub fn read_word(&self, addr: u16) -> u8 {
        // TODO Deal with ROM banking

        // Convert to pattern range
        //     1 ... 5 => println!("one through five"),
        if addr >= 0x8000 {
            // TODO Deal with multi banked ROM such as
            // super mario bros
            let reladdr;
            if addr >= 0xc000 {
                // We're in the second/upper bank
                reladdr = (addr - (0x8000 + 0x4000) + 0x10);
            } else {
                // Lower bank
                reladdr = (addr - 0x8000) + 0x10;
            }
            self.rom[reladdr as usize]

        } else if addr < 0x2000 {
            self.ram[addr as usize]

        } else {
            panic!("unknown address {:#x}", addr);
        }
    }

    pub fn write_word(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000 ... 0x2007 => {    // PPU
                let offset: usize = (addr - 0x2000) as usize;
                self.ppu[offset] = value;
            }
            _ => { panic!("unimplemented write address {:#x}", addr); }
        }
    }

    /*pub fn find_map(&self, addr: u16) -> Bank {
        if addr >= 0xc000 {
            return Bank::UpperRom;
        } else {
            return Bank::Lower;
        }
    }

    fn read_word_lower_bank(&self, addr: u16) -> u8 {
    }
    */

}

#[cfg(test)]
mod test {
    use super::Memory;

    #[test]
    fn test_read_system_ram() {
        let mut faux_rom = vec![0; 16*1024+16];
        let mem = Memory::new(faux_rom);
        mem.ram[0] = 0xFF;
        mem.ram[0x10] = 0xFF;
        mem.ram[0xa0] = 0xFF;
        mem.ram[0x800] = 0xFF;

        assert_eq!(0xFF, mem.read_word(0x00)); 
        assert_eq!(0xFF, mem.read_word(0x10)); 
        assert_eq!(0xFF, mem.read_word(0x7ff)); 
        assert_eq!(0, mem.read_word(0x01)); 
    }

    #[test]
    fn test_read_rom_single_bank() {
        let mut faux_rom = vec![0; 16*1024+16];
        faux_rom[16]        = 0xFF;
        faux_rom[16+0x10]   = 0xFF;
        faux_rom[16+0xa0]   = 0xFF;
        faux_rom[16+0x3FFF] = 0xFF;


        let mem = Memory::new(faux_rom);
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
        // TODO This test is expected to fail right now
        let mut faux_rom = vec![0; 32*1024+16];
        faux_rom[16]        = 0xFF;
        faux_rom[16+0x10]   = 0xFF;
        faux_rom[16+0xa0]   = 0xFF;
        faux_rom[16+0x3FFF] = 0xFF;

        faux_rom[16+0x4000] = 0xAA;
        faux_rom[16+0x4010] = 0xAA;
        faux_rom[16+0x40a0] = 0xAA;
        faux_rom[16+0x7FFF] = 0xAA;


        let mem = Memory::new(faux_rom);
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
    fn test_write_word() {
        panic!("implement");
    }
}
