use parking_lot::Mutex;
use std::sync::Arc;

#[macro_use]
mod macros;

pub mod disassemble;

mod status;
mod opcodes;
mod address_modes;

pub use nes::cpu::address_modes::*;

use nes::address::{Address,Addressable};
use nes::interconnect::Interconnect;
use nes::cpu::status::{Flags,StatusRegister};
use nes::cpu::opcodes::*;

// TODO Consider breaking the CPU logic out into its own private module and re-exporting it. This
// will require adjusting the visibility on a lot of methods.

// Power on state is defined here: https://wiki.nesdev.com/w/index.php/CPU_power_up_state

pub const STACK_START: u16 = 0x100;

#[derive(Debug,Clone,Copy)]
pub enum Interrupt {
    Nmi,
    Irq,
}

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
    pub interconnect: Arc<Mutex<Interconnect>>,

    PC: u16, // Program counter
    X: u8, // General purpose register
    Y: u8, // General purpose register
    AC: u8, // Accumlator register
    SP: u8, // Stack pointer
    SR: StatusRegister, // Status register

    cycles: u32,

    stack_depth: i16,
    last_pc: u16,

}

impl Cpu {
    pub fn new(interconnect: Arc<Mutex<Interconnect>>) -> Self {
        // See comment at top of file for power on state
        let pc = interconnect.lock().find_reset_vector_address();
        Cpu {
            X: 0,
            Y: 0,
            AC: 0,
            PC: pc.into(),
            SP: 0xfd,
            SR: StatusRegister::new(),

            interconnect: interconnect,
            cycles: 0,

            stack_depth: 0,
            last_pc: 0,
        }
    }

    pub fn get_pc(&self) -> Address {
        Address(self.PC)
    }

    pub fn write_at<T: Addressable>(&mut self, addr: T, value: u8) {
        let mut mem = self.interconnect.lock();
        mem.write_word(addr.nes_address(), value);
    }

    pub fn read_at<T: Addressable>(&self, addr: T) -> u8 {
        let mut mem = self.interconnect.lock();
        mem.read_word(addr.nes_address())
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
        // TODO Considering refactoring to always accept Address's
        Address::new_zeropage(low).into()
    }

    #[inline]
    fn read_word_and_increment(&mut self) -> u8 {
        let word = self.read_at(self.PC);
        self.PC += 1;
        word
    }

    #[inline]
    fn read_dword_and_increment(&mut self) -> u16 {
        let lo = self.read_at(self.PC);
        let hi = self.read_at(self.PC + 1);
        self.PC += 2;

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

    // TODO Consider breaking the stack code out into a seperate struct

    // 6502 stack grows downward
    pub fn push_word(&mut self, word: u8) {
        // TODO Panic if pointer ends up in a page besides page 1
        let ptr = STACK_START + self.SP as u16;
        self.write_at(ptr, word);
        self.SP -= 1;
        self.stack_depth += 1;
    }

    pub fn pop_word(&mut self) -> u8 {
        // TODO Panic if pointer ends up in a page besides page 1
        self.SP += 1;
        self.stack_depth -= 1;
        let ptr = STACK_START + self.SP as u16;
        self.read_at(ptr)
    }

    pub fn push_address(&mut self, addr: Address) {
        self.push_word(addr.high());
        self.push_word(addr.low());
    }

    pub fn pop_address(&mut self) -> Address {
        let lo = self.pop_word();
        let hi = self.pop_word();
        Address::new(hi, lo)
    }

    fn debug_stack(&self) {
        println!("Stack");
        println!(" Addr  | Value");
        println!("---------------");
        let start = self.SP as u16 + STACK_START + 1; // Add 1 to not display the current "slot"
        let end = start + self.stack_depth as u16;
        for idx in (start..end).rev() {
            println!("{:#06x} | {:02X}", idx as u16, self.read_at(idx as u16));
        }
        println!("---------------");
    }

    #[inline]
    pub fn page_crossed<T: Addressable>(a: T, b: T) -> bool {
        a.high() != b.high()
    }

    // If we wrap, only increment the lower byte. This was a bug in the 6502
    #[inline]
    fn buggy_read_dword(&self, addr: u16) -> u16 {
        let lo = self.read_at(addr);
        let hi = self.read_at((addr & 0xFF00) | (addr as u8).wrapping_add(1) as u16);

        (hi as u16) << 8 | lo as u16
    }

    // https://wiki.nesdev.com/w/index.php/CPU_addressing_modes
    // TODO Returning a tuple here feels weird. Maybe an enum indciating page crossed
    pub fn translate_address(&mut self, mode: AddressMode) -> (Address, bool) {
        let result: u16;
        let mut pages_differ = false;

        // TODO Not sure what happens here in case of overflows. Needs research. Add tests
        match mode {
            AddressMode::ZeroPage => {
                let word = self.read_word_and_increment();
                result = Cpu::zero_page_address(word);
            },
            AddressMode::ZeroPageX => {
                let word = self.read_word_and_increment();
                result = Cpu::zero_page_address(word) + self.X as u16;
            },
            AddressMode::ZeroPageY => {
                let word = self.read_word_and_increment();
                result = Cpu::zero_page_address(word) + self.Y as u16;
            },
            AddressMode::Indirect => {
                let indirect_addr = self.read_dword_and_increment();
                result = self.buggy_read_dword(indirect_addr);
            },
            AddressMode::IndirectY => {
                let indirect_addr = Cpu::zero_page_address(self.read_word_and_increment());
                result = self.buggy_read_dword(indirect_addr) + self.Y as u16;
                pages_differ = Cpu::page_crossed(self.PC, result);
            },
            AddressMode::IndirectX => {
                let indirect_addr = Cpu::zero_page_address(self.read_word_and_increment());
                result = self.buggy_read_dword(indirect_addr + (self.X as u16));
            },
            AddressMode::Absolute => {
                result = self.read_dword_and_increment();
            },
            AddressMode::AbsoluteX => {
                result = self.read_dword_and_increment() + self.X as u16;
                pages_differ = Cpu::page_crossed(self.PC, result);
            },
            AddressMode::AbsoluteY => {
                result = self.read_dword_and_increment() + self.Y as u16;
                pages_differ = Cpu::page_crossed(self.PC, result);
            },
            _ => { panic!("unimplemented {:?} for translate_address", mode); }
        }

        (Address(result), pages_differ)
    }

    fn execute_interrupt(&mut self, intr: Interrupt) -> u32 {
        let pc = Address(self.PC);
        let sr: u8 = self.SR.to_u8();

        self.push_address(pc);
        self.push_word(sr);

        let mut interconnect = self.interconnect.lock();

        match intr {
            Interrupt::Nmi => {
                self.PC = interconnect.find_nmi_vector_address().into();
            }
            Interrupt::Irq => {
                self.PC = interconnect.find_irq_vector_address().into();
            }
        }
        self.SR.set(Flags::Interrupt);

        7
    }

    // This is only used in testing/debugging.
    fn rewind(&mut self) -> u16 {
        let previous = self.PC;
        self.PC = self.last_pc;

        previous
    }

    pub fn step(&mut self, pending_interrupt: Option<Interrupt>) -> u32 {
        let mut burned_cycles = 0;
        self.last_pc = self.PC;

        // On a real NES, what happens if an interrupt fires during DMA?
        {
            let mut interconnect = self.interconnect.lock();

            if interconnect.dma_in_progress {
                let next_byte = self.read_at(
                    Address::new(
                        interconnect.dma_high_byte,
                        interconnect.dma_write_iteration
                    ).to_u16()
                );
                interconnect.ppu.write_dma(next_byte);

                interconnect.dma_write_iteration += 1;

                if interconnect.dma_write_iteration == 255 {
                    interconnect.dma_in_progress = false;
                    interconnect.dma_write_iteration = 0;
                    return 3;   // This equal a total of 513 cycles per DMA
                } else {
                    return 2;
                }
            }
        }

        if let Some(intr) = pending_interrupt {
            match intr {
                Interrupt::Nmi => {
                    burned_cycles += self.execute_interrupt(intr);
                }
                Interrupt::Irq => {
                    if !self.SR.is_set(Flags::Interrupt) {
                        burned_cycles += self.execute_interrupt(intr);
                    }
                }
            }
        }

        let instr = self.read_word_and_increment();

        // XXX Make this less terrible. It'd be nice we could expose
        // memory as a byte stream and/or get a slice out of memory
        // with the mapping all working correctly
        // println!("{}", Disassembler::disassemble(
        //     self.PC-1,
        //     instr,
        //     &[self.read_at(self.PC),self.read_at(self.PC+1)]
        // ));


        // TODO How does this perform? Look into an array of opcodes like in the disassembler
        burned_cycles += match instr {
            0x06 => {
                Asl::from_address(self, AddressMode::ZeroPage)
            }
            0x0a => {
                Asl::from_accumulator(self)
            }
            0x09 => {
                Ora::from_immediate(self)
            }
            0x10 => {
                Bpl::relative(self) as u32
            }
            0xf0 => {
                Beq::relative(self) as u32
            }
            0xd0 => {
                Bne::relative(self) as u32
            }
            0xb0 => {
                Bcs::from_relative(self)
            }
            0x30 => {
                Bmi::from_relative(self)
            }
            0x90 => {
                Bcc::from_relative(self)
            }
            0xc9 => {
                Cmp::immediate(self) as u32
            }
            0x20 => {
                Jsr::absolute(self) as u32
            }
            0x40 => {
                Rti::from_implied(self)
            }
            0x60 => {
                Rts::from_implied(self)
            }
            0x4a => {
                Lsr::accumulator(self) as u32
            }
            0x46 => {
                Lsr::from_address(self, AddressMode::ZeroPage)
            }
            0x4c => {
                Jmp::absolute(self) as u32
            }
            0x48 => {
                Pha::implied(self) as u32
            }
            0x45 => {
                Eor::from_address(self, AddressMode::ZeroPage)
            }
            0x49 => {
                Eor::from_immediate(self)
            }
            0x68 => {
                Pla::from_implied(self)
            }
            0x25 => {
                And::from_address(self, AddressMode::ZeroPage)
            }
            0x35 => {
                And::from_address(self, AddressMode::ZeroPageX)
            }
            0x05 => {
                Ora::from_address(self, AddressMode::ZeroPage)
            }
            0x29 => {
                And::from_immediate(self)
            }
            0x6d => {
                Adc::from_address(self, AddressMode::Absolute)
            }
            0x65 => {
                Adc::from_address(self, AddressMode::ZeroPage)
            }
            0x69 => {
                Adc::from_immediate(self)
            }
            0xe5 => {
                Sbc::from_address(self, AddressMode::ZeroPage)
            }
            0xe9 => {
                Sbc::from_immediate(self)
            }
            0x66 => {
                Ror::from_address(self, AddressMode::ZeroPage)
            }
            0x6a => {
                Ror::from_accumulator(self)
            }
            0x2a => {
                Rol::from_accumulator(self)
            }
            0x38 => {
                Sec::from_implied(self)
            }
            0x78 => {
                Sei::implied(self) as u32
            }
            0x18 => {
                Clc::implied(self) as u32
            }
            0xd8 => {
                Cld::implied(self) as u32
            }
            0xaa => {
                Tax::from_implied(self)
            }
            0xa8 => {
                Tay::from_implied(self)
            }
            0x8a => {
                Txa::implied(self) as u32
            }
            0x98 => {
                Tya::implied(self) as u32
            }
            0x9a => {
                Txs::implied(self) as u32
            }
            0xa0 => {
                Ldy::from_immediate(self)
            }
            0xa4 => {
                Ldy::from_address(self, AddressMode::ZeroPage)
            }
            0xb4 => {
                Ldy::from_address(self, AddressMode::ZeroPageX)
            }
            0xa2 => {
                Ldx::from_immediate(self)
            }
            0xa6 => {
                Ldx::from_address(self, AddressMode::ZeroPage)
            }
            0xad => {
                Lda::from_address(self, AddressMode::Absolute)
            }
            0xae => {
                Ldx::from_address(self, AddressMode::Absolute)
            }
            0xa5 => {
                Lda::from_address(self, AddressMode::ZeroPage)
            }
            0xb5 => {
                Lda::from_address(self, AddressMode::ZeroPageX)
            }
            0xa9 => {
                Lda::from_immediate(self)
            }
            0xb1 => {
                Lda::from_address(self, AddressMode::IndirectY)
            }
            0xb9 => {
                Lda::from_address(self, AddressMode::AbsoluteY)
            }
            0xbd => {
                Lda::from_address(self, AddressMode::AbsoluteX)
            }
            0x84 => {
                Sty::from_address(self, AddressMode::ZeroPage)
            }
            0x94 => {
                Sty::from_address(self, AddressMode::ZeroPageX)
            }
            0x8c => {
                Sty::from_address(self, AddressMode::Absolute)
            }
            0x85 => {
                Sta::from_address(self, AddressMode::ZeroPage)
            }
            0x86 => {
                Stx::from_address(self, AddressMode::ZeroPage)
            }
            0x8d => {
                Sta::from_address(self, AddressMode::Absolute)
            }
            0x8e => {
                Stx::from_address(self, AddressMode::Absolute)
            }
            0x9d => {
                Sta::from_address(self, AddressMode::AbsoluteX)
            }
            0x91 => {
                Sta::from_address(self, AddressMode::IndirectY)
            }
            0x95 => {
                Sta::from_address(self, AddressMode::ZeroPageX)
            }
            0x99 => {
                Sta::from_address(self, AddressMode::AbsoluteY)
            }
            0xc6 => {
                Dec::from_address(self, AddressMode::ZeroPage)
            }
            0xd6 => {
                Dec::from_address(self, AddressMode::ZeroPageX)
            }
            0xce => {
                Dec::from_address(self, AddressMode::Absolute)
            }
            0xe6 => {
                Inc::from_address(self, AddressMode::ZeroPage)
            }
            0xf6 => {
                Inc::from_address(self, AddressMode::ZeroPageX)
            }
            0xee => {
                Inc::from_address(self, AddressMode::Absolute)
            }
            0xfe => {
                Inc::from_address(self, AddressMode::AbsoluteX)
            }
            0x88 => {
                Dey::implied(self) as u32
            }
            0xca => {
                Dex::from_implied(self)
            }
            0xc0 => {
                Cpy::from_immediate(self)
            }
            0xc8 => {
                Iny::from_implied(self)
            }
            0xe8 => {
                Inx::from_implied(self)
            }
            0x00 => {
                self.debug_stack();
                panic!("Hit a BRK instruction which is probably wrong: {:#x}, {:#x} {:#x}, PC: {:#x}",
                   instr,
                   self.read_at(self.PC),
                   self.read_at(self.PC + 1),
                   self.PC
                );
            }
            _ => {
                self.debug_stack();
                panic!("unrecognized opcode {:#x}, {:#x} {:#x}, PC: {:#x}",
                   instr,
                   self.read_at(self.PC),
                   self.read_at(self.PC + 1),
                   self.PC
                );
            }
        };

        self.cycles += burned_cycles;
        burned_cycles
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;
    use nes::ppu::Ppu;
    use nes::rom::{Bank,Rom};
    use nes::interconnect::Interconnect;


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

    pub fn memory_from_rom(interconnect: Vec<u8>, doublebank: bool) -> Interconnect {
        let rom = if doublebank {
            if interconnect.len() <= 0x4000 {
                // 16k
                panic!("rom not large enough for double banking");
            }
            Rom::new_double_bank(Bank::new(&interconnect[0..0x4000]), Bank::new(&interconnect[0x4000..]))
        } else {
            // single banked
            Rom::new_single_bank(Bank::new(&interconnect[0..0x4000]))
        };

        let ppu = Ppu::new();
        Interconnect::new(ppu, rom)
    }

    pub fn mock_cpu(words: &[u8]) -> Cpu {
        let mock_rom = rom_with_pc_at_start(words);
        let interconnect = memory_from_rom(mock_rom, true);
        Cpu::new(interconnect)
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

        cpu.push_word(0x10);
        let mut result = cpu.read_at(0x01FF);

        assert!(cpu.SP == 0xFE, "expected 0xFE, got {:#x}", cpu.SP);
        assert!(result == 0x10, "expected 0x10, got {:#x}", result);

        cpu.push_word(0x11);
        result = cpu.read_at(0x01FE);

        assert!(cpu.SP == 0xFD, "expected 0xFD, got {:#x}", cpu.SP);
        assert!(result == 0x11, "expected 0x11, got {:#x}", result);

        result = cpu.pop_word();
        assert!(result == 0x11, "expected 0x11, got {:#x}", result);
        assert!(cpu.SP == 0xFE, "expected 0xFE, got {:#x}", cpu.SP);

        result = cpu.pop_word();
        assert!(result == 0x10, "expected 0x10, got {:#x}", result);
        assert!(cpu.SP == 0xFF, "expected 0xFF, got {:#x}", cpu.SP);
    }

    #[test]
    fn test_translate_address_indirect_y() {
        // TODO Write these tests
    }
}
