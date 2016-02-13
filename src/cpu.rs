// See http://e-tradition.net/bytes/6502/6502_instruction_set.html
//#[feature(clone_from_slice)]


use super::mem::Memory;

// TODO Fix this at some point
#[allow(non_snake_case)]
pub struct StatusRegister {
    // bit 7
    Negative: bool, // N

    // bit 6
    Overflow: bool, // V

    // bit 5
    // reserved

    // bit 4
    Break: bool, // B

    // bit 3
    // NOTE This apparently affects ADC/SBC instructions
    Decimal: bool, // D

    // bit 2
    Interrupt: bool, // I

    // bit 1
    Zero: bool, // Z

    // bit 0
    Carry: bool, // C
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister {
            Negative: false,
            Overflow: false,
            Break: false,
            Decimal: false,
            Interrupt: false,
            Zero: false,
            Carry: false,
        }
    }

    #[inline]
    pub fn reset_zero(&mut self) {
        self.Zero = false;
    }

    #[inline]
    pub fn set_zero(&mut self) {
        self.Zero = true;
    }

    #[inline]
    pub fn reset_negative(&mut self) {
        self.Negative = false;
    }

    #[inline]
    pub fn set_negative(&mut self) {
        self.Negative = true;
    }
}


// TODO Fix this at some point
#[allow(non_snake_case)]
pub struct Cpu<'a> {
    PC : u16, // Program counter
    X  : u8,  // General purpose register
    Y  : u8,  // General purpose register
    AC : u8,  // Accumlator register
    SP : u8,  // Stack pointer
    SR : StatusRegister,  // Status register

    mem: Memory<'a>,
}

impl<'a> Cpu<'a> {
    pub fn new(mem: Memory<'a>) -> Self {
        Cpu {
            X: 0,
            Y: 0,
            AC: 0,
            // TODO Do we want to do this on startup?
            PC: Cpu::find_pc_addr(&mem),
            SP: 0,
            SR: StatusRegister::new(),

            mem: mem,
        }
    }

    pub fn start(&mut self) {
        println!("PC: {:#x}", self.PC);
        loop {
            self.execute_instruction();
        }
    }

    fn find_pc_addr(mem: &Memory) -> u16 {
        // http://forum.6502.org/viewtopic.php?t=1708
        (mem.read_word(0xFFFD) as u16) << 8 | mem.read_word(0xFFFC) as u16
    }

    #[inline]
    fn read_word_and_increment(&mut self) -> u8 {
        let word = self.mem.read_word(self.PC);
        self.PC += 1;    // 1 byte forward
        word
    }

    #[inline]
    fn read_dword_and_increment(&mut self) -> u16 {
        let lo = self.mem.read_word(self.PC);
        let hi = self.mem.read_word(self.PC+1);
        self.PC += 2;    // 2 bytes forward

        (hi as u16) << 8 | lo as u16
    }

    // TODO This function needs tests
    fn zero_and_negative_status(&mut self, word: u8) {
        // Set the Zero bit in the status register if the word is zero.
        // Else, reset it back to its default.
        if word == 0 {
            self.SR.set_zero();
        } else {
            self.SR.reset_zero();
        }

        // Set the Negative bit in the status register if the word is
        // negative. Else, reset it back to its default.
        if (word as i8) < 0 {
            self.SR.set_negative();
        } else {
            self.SR.reset_negative();
        }
    }

    // TODO Ok to make this public? Should we only use start() ?
    fn execute_instruction(&mut self) {
        let instr = self.read_word_and_increment();

        // See http://users.telenet.be/kim1-6502/6502/proman.html
        println!("Instruction: {:#x}", instr);
        match instr {
            // SEI
            0x78 => {
                self.SR.Interrupt = true;
            }
            // CLD
            0xd8 => {
                self.SR.Decimal = false;
            }
            // TXS
            0x9a => {
                self.SP = self.X;
            }
            // LDX # immediate
            0xa2 => {
                let word = self.read_word_and_increment();
                self.X = word;
                self.zero_and_negative_status(word);
            }
            // LDA absolute
            0xad => {
                let dest = self.read_dword_and_increment();
                let word = self.mem.read_word(dest);
                self.AC = word;
                self.zero_and_negative_status(word);
            }
            // LDA # immediate
            0xa9 => {
                let word = self.read_word_and_increment();
                self.AC = word;
                self.zero_and_negative_status(word);
            }
            // STA absolute
            0x8d => {
               let dest = self.read_dword_and_increment(); 
               self.mem.write_word(dest, self.AC);
            }
            _ => {
                panic!("unrecognized opcode {:#x}, {:#x} {:#x}",
                    instr,
                    self.mem.read_word(self.PC),
                    self.mem.read_word(self.PC + 1),
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;
    use mem::Memory;

    fn rom_with_pc_at_start(words: &[u8]) -> Vec<u8> {
        let mut mock_rom = vec![0; 1024*32];
        // Just set PC to the beginning of the rom
        // in each of the banks
        mock_rom[0x3ffc] = 0x00;
        mock_rom[0x3ffd] = 0x80;
        mock_rom[0x7ffc] = 0x00;
        mock_rom[0x7ffd] = 0x80;

        // Insert our words that are pointed to by the PC
        for (idx, &w) in words.iter().enumerate() {
            mock_rom[idx] = w;
        }

        return mock_rom;
    }

    fn memory_from_rom(mem: &[u8], doublebank: bool) -> Memory {
        if doublebank {
            if mem.len() <= 0x4000 { //16k
                panic!("rom not large enough for double banking");
            }
            Memory::new(&mem[0..0x4000], &mem[0x4000..])
        } else {
            // single banked
            Memory::new(&mem, &mem)
        }
    }

    #[test]
    fn test_find_pc_addr() {
        let mut mock_rom = vec![0; 1024*16];
        mock_rom[0x3ffc] = 0xef;
        mock_rom[0x3ffd] = 0xbe;

        let mem = Memory::new(&mock_rom, &mock_rom);
        let result = Cpu::find_pc_addr(&mem);
        assert!(result == 0xbeef, "expected 0xbeef, got: {:#x}", result);
    }

    // TODO Cleanup the repetive preamble in every test

    #[test]
    fn test_read_word_and_increment() {
        let mock_rom = rom_with_pc_at_start(&[0xAA]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got: {:#x}", cpu.PC);
        let word = cpu.read_word_and_increment();

        assert!(cpu.PC == 0x8001, "expected 0x8001, got: {:#x}", cpu.PC);
        assert!(word == 0xAA, "expected 0xaa, got {:#x}", word);
    }

    #[test]
    fn test_read_dword_and_increment() {
        let mock_rom = rom_with_pc_at_start(&[0xef,0xbe]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got: {:#x}", cpu.PC);
        let dword = cpu.read_dword_and_increment();

        assert!(cpu.PC == 0x8002, "expected 0x8002, got: {:#x}", cpu.PC);
        assert!(dword == 0xbeef, "expected 0xbeef, got {:#x}", dword);
    }

    //
    // Instruction tests
    //

    #[test]
    fn test_sei() {
        let mock_rom = rom_with_pc_at_start(&[0x78]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);

        assert!(cpu.SR.Interrupt == false, "expected false, got {:#?}", cpu.SR.Interrupt);
        cpu.execute_instruction();
        assert!(cpu.SR.Interrupt == true, "expected true, got {:#?}", cpu.SR.Interrupt);
    }

    #[test]
    fn test_cld() {
        let mock_rom = rom_with_pc_at_start(&[0xd8]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);

        cpu.SR.Decimal = true;
        assert!(cpu.SR.Decimal == true, "expected true, got {:#?}", cpu.SR.Decimal);
        cpu.execute_instruction();
        assert!(cpu.SR.Decimal == false, "expected false, got {:#?}", cpu.SR.Decimal);
    }

    #[test]
    fn test_txs() {
        let mock_rom = rom_with_pc_at_start(&[0x9a]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);

        cpu.X = 0xff;
        assert!(cpu.SP == 0, "expected 0x00, got {:#x}", cpu.SP);
        cpu.execute_instruction();
        assert!(cpu.SP == 0xff, "expected 0xff, got {:#x}", cpu.SP);
    }

    #[test]
    fn test_ldx_imm() {
        let mock_rom = rom_with_pc_at_start(&[0xa2,0xff]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);

        assert!(cpu.X == 0, "expected 0, got {:#x}", cpu.X);
        cpu.execute_instruction();
        assert!(cpu.X == 0xff, "expected 0xff, got {:#x}", cpu.X);
    }

    #[test]
    fn test_lda_abs() {
        let mock_rom = rom_with_pc_at_start(&[0xad,0x03,0x80,0xff]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);

        assert!(cpu.AC == 0, "expected 0, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xff, "expected 0xff, got {:#x}", cpu.AC);
    }

    #[test]
    fn test_lda_imm() {
        let mock_rom = rom_with_pc_at_start(&[0xa9,0xff]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);

        assert!(cpu.AC == 0, "expected 0, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xff, "expected 0xff, got {:#x}", cpu.AC);
    }

    #[test]
    fn test_sta_abs() {
        let mock_rom = rom_with_pc_at_start(&[0x8d,0x01,0x20]);
        let mem = memory_from_rom(&mock_rom, true);
        let mut cpu = Cpu::new(mem);
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x2001);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x2001);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }
}
