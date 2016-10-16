// See http://e-tradition.net/bytes/6502/6502_instruction_set.html
// #[feature(clone_from_slice)]

use super::{AddressSpace,Bank};


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
pub struct Cpu {
    PC: u16, // Program counter
    X: u8, // General purpose register
    Y: u8, // General purpose register
    AC: u8, // Accumlator register
    SP: u8, // Stack pointer
    SR: StatusRegister, // Status register

    mem: AddressSpace,
}

impl Cpu {
    pub fn new(mem: AddressSpace) -> Self {
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

    fn find_pc_addr(mem: &AddressSpace) -> u16 {
        // http://forum.6502.org/viewtopic.php?t=1708
        (mem.read_word(0xFFFD) as u16) << 8 | mem.read_word(0xFFFC) as u16
    }

    #[inline]
    fn zero_page_address(low: u8) -> u16 {
        0x0000 + low as u16
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
        let hi = self.mem.read_word(self.PC + 1);
        self.PC += 2;    // 2 bytes forward

        (hi as u16) << 8 | lo as u16
    }

    // TODO This function needs tests
    // TODO I think this may only ever evaluate self.AC which makes the arg useless. Review this
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

    fn move_pc_relative(&mut self, word: u8) {
        // NOTE This can either move the PC register forward
        // or backwards. I.E we should treat `word` here as
        // if it were signed.
        if word < 128 {
            self.PC = self.PC.wrapping_add(word as u16);
        } else {
            // We're actually negative here so get the
            // inverse and add one to it. This is for
            // converting with two's completement since Rust
            // wont allow us to add signed and unsigned
            // numerals together.

            // NOTE '!' is used in Rust like '~' is used in C.
            self.PC = self.PC.wrapping_sub((!word + 1) as u16);
        }
    }

    // TODO Ok to make this public? Should we only use start() ?
    fn execute_instruction(&mut self) {
        let instr = self.read_word_and_increment();

        // See http://users.telenet.be/kim1-6502/6502/proman.html
        println!("Instruction: {:#x} {:#x} {:#x}",
                 instr,
                 self.mem.read_word(self.PC),
                 self.mem.read_word(self.PC + 1));
        // TODO This inline match logic is obviously not going to scale.
        // Eventually we should move decoding and execution into an
        // instruction struct (or something like that).
        match instr {
            // BPL (branch on result plus)
            0x10 => {
                let word = self.read_word_and_increment();
                if !self.SR.Negative { self.move_pc_relative(word); }
            }
            // BEQ (branch on zero result)
            0xf0 => {
                let word = self.read_word_and_increment();
                if self.SR.Zero { self.move_pc_relative(word); }
            }
            // AND # immediate
            0x29 => {
                let word = self.read_word_and_increment();
                self.AC &= word;
                self.zero_and_negative_status(word);
            }
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
            // LDY # immediate
            0xa0 => {
                let word = self.read_word_and_increment();
                self.Y = word;
                self.zero_and_negative_status(word);
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
            // STY zero page
            0x84 => {
                let word = self.read_word_and_increment();
                self.mem.write_word(Cpu::zero_page_address(word), self.Y);
            }
            // STA absolute
            0x8d => {
                let dest = self.read_dword_and_increment();
                self.mem.write_word(dest, self.AC);
            }
            // STA (Indrect), Y
            0x91 => {
                let word = self.read_word_and_increment();
                let indirect_addr = self.mem.read_word(Cpu::zero_page_address(word));
                self.mem.write_word((indirect_addr + self.Y) as u16, self.AC);
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
    use super::super::address::{AddressSpace,Bank};


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

        mock_rom
    }

    fn memory_from_rom(mem: Vec<u8>, doublebank: bool) -> AddressSpace {
        if doublebank {
            if mem.len() <= 0x4000 {
                // 16k
                panic!("rom not large enough for double banking");
            }
            AddressSpace::new_double_bank(Bank::new(&mem[0..0x4000]), Bank::new(&mem[0x4000..]))
        } else {
            // single banked
            AddressSpace::new_single_bank(Bank::new(&mem[0..0x4000]))
        }
    }

    fn mock_cpu(words: &[u8]) -> Cpu {
        let mock_rom = rom_with_pc_at_start(words);
        let mem = memory_from_rom(mock_rom, true);
        Cpu::new(mem)
    }


    #[test]
    fn test_find_pc_addr() {
        let mut mock_rom = vec![0; 1024*16];
        mock_rom[0x3ffc] = 0xef;
        mock_rom[0x3ffd] = 0xbe;

        let mem = AddressSpace::new_single_bank(Bank::new(&mock_rom));
        let result = Cpu::find_pc_addr(&mem);
        assert!(result == 0xbeef, "expected 0xbeef, got: {:#x}", result);
    }

    #[test]
    fn test_zero_page_address() {
        assert!(Cpu::zero_page_address(0xff) == 0x00ff);
        assert!(Cpu::zero_page_address(0x00) == 0x0000);
        assert!(Cpu::zero_page_address(0x12) == 0x0012);
    }

    #[test]
    fn test_read_word_and_increment() {
        let mut cpu = mock_cpu(&[0xAA]);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got: {:#x}", cpu.PC);
        let word = cpu.read_word_and_increment();

        assert!(cpu.PC == 0x8001, "expected 0x8001, got: {:#x}", cpu.PC);
        assert!(word == 0xAA, "expected 0xaa, got {:#x}", word);
    }

    #[test]
    fn test_read_dword_and_increment() {
        let mut cpu = mock_cpu(&[0xef, 0xbe]);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got: {:#x}", cpu.PC);
        let dword = cpu.read_dword_and_increment();

        assert!(cpu.PC == 0x8002, "expected 0x8002, got: {:#x}", cpu.PC);
        assert!(dword == 0xbeef, "expected 0xbeef, got {:#x}", dword);
    }

    // Instruction tests
    //

    #[test]
    fn test_bpl_skip() {
        let mut cpu = mock_cpu(&[0x10, 0xff]);

        cpu.SR.Negative = true;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction();
        assert!(cpu.PC == 0x8002, "expected 0x8002, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_bpl_take_positive() {
        let mut cpu = mock_cpu(&[0x10, 0x2a]);

        cpu.SR.Negative = false;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction(); // Two byte instruction so *add* two below
        assert!(cpu.PC == 0x802c, "expected 0x802a, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_bpl_take_negative() {
        let mut cpu = mock_cpu(&[0x10, 0x82]); // hex 0x82 is signed -126

        cpu.SR.Negative = false;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction(); // Two byte instruction so *substract* two bytes below

        // NOTE For posterity, this actually drops us below the ROM
        // start range which I dont think will happen with real ROMs.
        // This should be fine for our test though.
        assert!(cpu.PC == 0x7f84, "expected 0x7f82, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_beq_skip() {
        let mut cpu = mock_cpu(&[0xf0, 0xff]);

        cpu.SR.Zero = false;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction();
        assert!(cpu.PC == 0x8002, "expected 0x8002, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_beq_take_positive() {
        let mut cpu = mock_cpu(&[0xf0, 0x2a]);

        cpu.SR.Zero = true;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction(); // Two byte instruction so *add* two below
        assert!(cpu.PC == 0x802c, "expected 0x802a, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_beq_take_negative() {
        let mut cpu = mock_cpu(&[0xf0, 0x82]); // hex 0x82 is signed -126

        cpu.SR.Zero = true;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction(); // Two byte instruction so *substract* two bytes below

        // NOTE For posterity, this actually drops us below the ROM
        // start range which I dont think will happen with real ROMs.
        // This should be fine for our test though.
        assert!(cpu.PC == 0x7f84, "expected 0x7f82, got {:#x}", cpu.PC);
    }


    #[test]
    fn test_sei() {
        let mut cpu = mock_cpu(&[0x78]);

        assert!(cpu.SR.Interrupt == false,
                "expected false, got {:#?}",
                cpu.SR.Interrupt);
        cpu.execute_instruction();
        assert!(cpu.SR.Interrupt == true,
                "expected true, got {:#?}",
                cpu.SR.Interrupt);
    }

    #[test]
    fn test_cld() {
        let mut cpu = mock_cpu(&[0xd8]);

        cpu.SR.Decimal = true;
        assert!(cpu.SR.Decimal == true,
                "expected true, got {:#?}",
                cpu.SR.Decimal);
        cpu.execute_instruction();
        assert!(cpu.SR.Decimal == false,
                "expected false, got {:#?}",
                cpu.SR.Decimal);
    }

    #[test]
    fn test_txs() {
        let mut cpu = mock_cpu(&[0x9a]);

        cpu.X = 0xff;
        assert!(cpu.SP == 0, "expected 0x00, got {:#x}", cpu.SP);
        cpu.execute_instruction();
        assert!(cpu.SP == 0xff, "expected 0xff, got {:#x}", cpu.SP);
    }

    #[test]
    fn test_ldy_imm() {
        let mut cpu = mock_cpu(&[0xa0, 0xff]);

        assert!(cpu.Y == 0, "expected 0, got {:#x}", cpu.Y);
        cpu.execute_instruction();
        assert!(cpu.Y == 0xff, "expected 0xff, got {:#x}", cpu.Y);
    }

    #[test]
    fn test_ldx_imm() {
        let mut cpu = mock_cpu(&[0xa2, 0xff]);

        assert!(cpu.X == 0, "expected 0, got {:#x}", cpu.X);
        cpu.execute_instruction();
        assert!(cpu.X == 0xff, "expected 0xff, got {:#x}", cpu.X);
    }

    #[test]
    fn test_lda_abs() {
        let mut cpu = mock_cpu(&[0xad, 0x03, 0x80, 0xff]);

        assert!(cpu.AC == 0, "expected 0, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xff, "expected 0xff, got {:#x}", cpu.AC);
    }

    #[test]
    fn test_lda_imm() {
        let mut cpu = mock_cpu(&[0xa9, 0xff]);

        assert!(cpu.AC == 0, "expected 0, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xff, "expected 0xff, got {:#x}", cpu.AC);
    }

    #[test]
    fn test_sta_abs() {
        let mut cpu = mock_cpu(&[0x8d, 0x10, 0x00]);
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_sta_indirect_y() {
        let mut cpu = mock_cpu(&[0x91, 0x10]);
        cpu.mem.write_word(0x0010, 0xaa);
        cpu.Y = 0x10;
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x00ba);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x00ba);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_sty_zero() {
        let mut cpu = mock_cpu(&[0x84, 0x10]);
        cpu.Y = 0xff;

        let mut result = cpu.mem.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_and_immediate() {
        let mut cpu = mock_cpu(&[0x29, 0x84]);
        cpu.AC = 0xf0;

        assert!(cpu.AC == 0xf0, "expected 0xff, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0x80, "expected 0x80, got {:#x}", cpu.AC);
        //TODO Make assertions on status registers
    }
}
