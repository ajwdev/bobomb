#[macro_use]
mod macros;

mod disassemble;
mod status;
mod opcodes;
mod address_modes;

pub use nes::cpu::address_modes::*;

use nes::address::{AddressSpace,Address,Bank};
use nes::cpu::status::{StatusRegister};
use nes::cpu::opcodes::*;
use nes::cpu::disassemble::Disassembler;

// TODO Consider breaking the CPU logic out into its own private module and re-exporting it. This
// will require adjusting the visibility on a lot of methods.

// Power on state is defined here: https://wiki.nesdev.com/w/index.php/CPU_power_up_state

pub const STACK_START: u16 = 0x100;

#[derive(Debug)]
pub enum Registers {
    X,
    Y,
    AC,
    SP,
    PC,
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

    // These are only used for debugging purposes
    instruction_counter: i64,
    stack_depth: i16,
    last_pc: u16,

}

impl Cpu {
    pub fn new(mem: AddressSpace) -> Self {
        // See comment at top of file for power on state
        Cpu {
            X: 0,
            Y: 0,
            AC: 0,
            PC: Cpu::find_pc_addr(&mem),
            SP: 0xfd,
            SR: StatusRegister::new(),

            mem: mem,
            instruction_counter: 0,
            stack_depth: 0,
            last_pc: 0,
        }
    }

    pub fn start(&mut self) {
        println!("PC: {:#x}", self.PC);
        loop {
            self.execute_instruction();
            self.instruction_counter += 1;
        }
    }

    fn find_pc_addr(mem: &AddressSpace) -> u16 {
        // http://forum.6502.org/viewtopic.php?t=1708
        (mem.read_word(0xFFFD) as u16) << 8 | mem.read_word(0xFFFC) as u16
    }

    pub fn register_value(&self, reg: Registers) -> u8 {
        match reg {
            Registers::X => { self.X },
            Registers::Y => { self.Y },
            Registers::AC => { self.AC },
            Registers::SP => { self.SP },
            Registers::PC => { panic!("Register PC is too wide. FIX") },
        }
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
    #[inline]
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
        // This can either move the PC register forward
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

            // '!' is used in Rust like '~' is used in C.
            self.PC = self.PC.wrapping_sub((!word + 1) as u16);
        }
    }

    // 6502 stack grows downward
    pub fn push_stack(&mut self, word: u8) {
        // TODO Panic if pointer ends up in a page besides page 1
        let ptr = STACK_START + self.SP as u16;
        self.mem.write_word(ptr, word);
        self.SP -= 1;
        self.stack_depth += 1;
    }

    pub fn pop_stack(&mut self) -> u8 {
        // TODO Panic if pointer ends up in a page besides page 1
        self.SP += 1;
        self.stack_depth -= 1;
        let ptr = STACK_START + self.SP as u16;
        self.mem.read_word(ptr)
    }

    fn debug_stack(&self) {
        println!("Stack");
        println!(" Addr  | Value");
        println!("---------------");
        let start = self.SP as u16 + STACK_START + 1; // Add 1 to not display the current "slot"
        let end = start + self.stack_depth as u16;
        for idx in (start..end).rev() {
            println!("{:#06x} | {:02X}", idx as u16, self.mem.read_word(idx as u16));
        }
        println!("---------------");
    }

    // https://wiki.nesdev.com/w/index.php/CPU_addressing_modes
    pub fn translate_address(&mut self, mode: AddressMode) -> Address {
        let result: u16;

        // TODO Not sure what happens here in case of overflows. Needs research. Add tests
        match mode {
            AddressMode::ZeroPage => {
                let word = self.read_word_and_increment();
                result = Cpu::zero_page_address(word);
            },
            // Also known as Indirect Indexed Addressing
            AddressMode::IndirectY => {
                let word = self.read_word_and_increment();
                let indirect_addr =
                    (self.mem.read_word(Cpu::zero_page_address(word+1)) as u16) << 8 |
                        (self.mem.read_word(Cpu::zero_page_address(word)) as u16);
                result = indirect_addr + self.Y as u16;
            },
            AddressMode::Absolute => {
                result = self.read_dword_and_increment();
            },
            AddressMode::AbsoluteX => {
                result = self.read_dword_and_increment() + self.X as u16;
            },
            AddressMode::AbsoluteY => {
                result = self.read_dword_and_increment() + self.Y as u16;
            },
            _ => { panic!("unimplemented {:?} for translate_address", mode); }
        }

        Address(result)
    }

    // This is only used in testing/debugging.
    fn rewind(&mut self) -> u16 {
        let previous = self.PC;
        self.PC = self.last_pc;

        previous
    }

    // TODO Ok to make this public? Should we only use start() ?
    fn execute_instruction(&mut self) {
        self.last_pc = self.PC;
        let instr = self.read_word_and_increment();

        // XXX Make this less terrible. It'd be nice we could expose
        // memory as a byte stream and/or get a slice out of memory
        // with the mapping all working correctly
        println!("{}", Disassembler::disassemble(
            self.PC-1, instr, &[self.mem.read_word(self.PC),self.mem.read_word(self.PC+1)]
        ));


        // TODO How does this perform? Look into an array of opcodes like in the disassembler
        match instr {
            0x10 => {
                Bpl::relative(self);
            }
            0xf0 => {
                Beq::relative(self);
            }
            0xd0 => {
                Bne::relative(self);
            }
            0xc9 => {
                Cmp::immediate(self);
            }
            0x20 => {
                Jsr::absolute(self);
            }
            0x60 => {
                Rts::from_implied(self);
            }
            0x4a => {
                Lsr::accumulator(self);
            }
            0x4c => {
                Jmp::absolute(self);
            }
            0x48 => {
                Pha::implied(self);
            }
            0x45 => {
                Eor::from_address(self, AddressMode::ZeroPage);
            }
            0x49 => {
                Eor::from_immediate(self);
            }
            0x68 => {
                Pla::from_implied(self);
            }
            0x25 => {
                And::zero_page(self);
            }
            0x29 => {
                And::immediate(self);
            }
            0x6d => {
                Adc::from_address(self, AddressMode::Absolute);
            }
            0x65 => {
                Adc::from_address(self, AddressMode::ZeroPage);
            }
            0x66 => {
                Ror::from_address(self, AddressMode::ZeroPage);
            }
            0x69 => {
                Adc::from_immediate(self);
            }
            0x78 => {
                Sei::implied(self);
            }
            0x18 => {
                Clc::implied(self);
            }
            0xd8 => {
                Cld::implied(self);
            }
            0xaa => {
                Tax::from_implied(self);
            }
            0xa8 => {
                Tay::from_implied(self);
            }
            0x8a => {
                Txa::implied(self);
            }
            0x98 => {
                Tya::implied(self);
            }
            0x9a => {
                Txs::implied(self);
            }
            0xa0 => {
                Ldy::immediate(self);
            }
            0xa2 => {
                Ldx::from_immediate(self);
            }
            0xa6 => {
                Ldx::from_address(self, AddressMode::ZeroPage);
            }
            0xad => {
                Lda::from_address(self, AddressMode::Absolute);
            }
            0xa5 => {
                Lda::from_address(self, AddressMode::ZeroPage);
            }
            0xa9 => {
                Lda::from_immediate(self);
            }
            0x84 => {
                Sty::zero_page(self);
            }
            0x85 => {
                Sta::from_address(self, AddressMode::ZeroPage);
            }
            0x86 => {
                Stx::from_address(self, AddressMode::ZeroPage);
            }
            0x8d => {
                Sta::from_address(self, AddressMode::Absolute);
            }
            0x8e => {
                Stx::from_address(self, AddressMode::Absolute);
            }
            0x91 => {
                Sta::from_address(self, AddressMode::IndirectY);
            }
            0xc6 => {
                Dec::zero_page(self);
            }
            0x88 => {
                Dey::implied(self);
            }
            0xca => {
                Dex::from_implied(self);
            }
            0xc0 => {
                Cpy::from_immediate(self);
            }
            0xc8 => {
                Iny::from_implied(self);
            }
            0xe8 => {
                Inx::from_implied(self);
            }
            0x00 => {
                self.debug_stack();
                panic!("Hit a BRK instruction which is probably wrong: {:#x}, {:#x} {:#x}, PC: {:#x}, count: {}",
                   instr,
                   self.mem.read_word(self.PC),
                   self.mem.read_word(self.PC + 1),
                   self.PC,
                   self.instruction_counter,
                )
            }
            _ => {
                self.debug_stack();
                panic!("unrecognized opcode {:#x}, {:#x} {:#x}, PC: {:#x}, count: {}",
                   instr,
                   self.mem.read_word(self.PC),
                   self.mem.read_word(self.PC + 1),
                   self.PC,
                   self.instruction_counter,
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;
    use nes::address::{AddressSpace,Bank};


    pub fn rom_with_pc_at_start(words: &[u8]) -> Vec<u8> {
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

    pub fn memory_from_rom(mem: Vec<u8>, doublebank: bool) -> AddressSpace {
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

    pub fn mock_cpu(words: &[u8]) -> Cpu {
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
        // XXX Is this really testing anything?
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


    #[test]
    fn test_stack() {
        let mut cpu = mock_cpu(&[]);
        cpu.SP = 0xFF;

        cpu.push_stack(0x10);
        let mut result = cpu.mem.read_word(0x01FF);

        assert!(cpu.SP == 0xFE, "expected 0xFE, got {:#x}", cpu.SP);
        assert!(result == 0x10, "expected 0x10, got {:#x}", result);

        cpu.push_stack(0x11);
        result = cpu.mem.read_word(0x01FE);

        assert!(cpu.SP == 0xFD, "expected 0xFD, got {:#x}", cpu.SP);
        assert!(result == 0x11, "expected 0x11, got {:#x}", result);

        result = cpu.pop_stack();
        assert!(result == 0x11, "expected 0x11, got {:#x}", result);
        assert!(cpu.SP == 0xFE, "expected 0xFE, got {:#x}", cpu.SP);

        result = cpu.pop_stack();
        assert!(result == 0x10, "expected 0x10, got {:#x}", result);
        assert!(cpu.SP == 0xFF, "expected 0xFF, got {:#x}", cpu.SP);
    }

    #[test]
    fn test_translate_address_indirect_y() {
        // TODO Write these tests
    }
}
