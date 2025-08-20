use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{AddressMode, Cpu, FromAddress, FromImmediate};

pub struct Sbc {}

impl Sbc {
    fn subtract_with_carry(cpu: &mut Cpu, word: u8) {
        // We ignore the Decimal status register because on the NES
        // it is unused. Consider adding support in the future.

        // SBC formula: A = A - M - (1 - C)
        // This is equivalent to: A = A + ~M + C
        let complement = !word;
        let carry = cpu.SR.is_set(Flags::Carry) as u16;

        let result = (cpu.AC as u16) + (complement as u16) + carry;
        let result_byte = result as u8;

        cpu.zero_and_negative_status(result_byte);

        // Carry is set if no borrow occurred (result >= 0x100)
        if result > 0xFF {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }

        // Overflow occurs when operands have same sign but result has different sign
        if (cpu.AC ^ result_byte) & (complement ^ result_byte) & 0x80 != 0 {
            cpu.SR.set(Flags::Overflow);
        } else {
            cpu.SR.reset(Flags::Overflow);
        }

        cpu.AC = result_byte;
    }
}

impl FromImmediate for Sbc {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        Sbc::subtract_with_carry(cpu, word);

        2
    }
}

impl FromAddress for Sbc {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());

        Sbc::subtract_with_carry(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::ZeroPageX => 4,
            AddressMode::Absolute => 4,
            AddressMode::AbsoluteX => 4 + (extra_cycles as u32),
            AddressMode::AbsoluteY => 4 + (extra_cycles as u32),
            AddressMode::IndirectX => 6,
            AddressMode::IndirectY => 5 + (extra_cycles as u32),
            _ => {
                panic!("unimplemented address mode {:?} for SBC", mode);
            }
        }
    }
}
