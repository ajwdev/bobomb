// See http://e-tradition.net/bytes/6502/6502_instruction_set.html

use super::mem::Memory;

pub struct Cpu {
    PC : u16, // Program counter
    X  : u8,  // General purpose register
    Y  : u8,  // General purpose register
    AC : u8,  // Accumlator register
    SP : u8,  // Stack pointer
    SR : StatusRegister,  // Status register

    mem: Memory,
}

impl Cpu {
    pub fn new(mem: Memory) -> Self {
        Cpu {
            X: 0,
            Y: 0,
            AC: 0,
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

    fn execute_instruction(&mut self) {
        let instr = self.read_word_and_increment();

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
            // LDA # immediate
            0xa9 => {
                self.AC = self.read_word_and_increment();
            }
            // STA absolute
            0x8d => {
               let dest = self.read_dword_and_increment(); 
               self.mem.write_word(dest, self.AC);
            }

            _ => panic!("unrecognized opcode {:#x}, {:#x} {:#x}", 
                            instr, 
                            self.mem.read_word(self.PC),
                            self.mem.read_word(self.PC + 1),
                        ),
        }
    }
}


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
}
