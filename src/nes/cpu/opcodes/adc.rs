use crate::nes::cpu::{Cpu,AddressMode,FromAddress,FromImmediate};
use crate::nes::cpu::status::Flags;


pub struct Adc { }

impl Adc {
    fn add_with_carry(cpu: &mut Cpu, word: u8) {
        // We ignore the Decimal status register because on the NES
        // it is unused. Consider adding support in the future.
        let tmp = word.wrapping_add(cpu.SR.is_set(Flags::Carry) as u8);
        let (result, overflow) = cpu.AC.overflowing_add(tmp);

        cpu.zero_and_negative_status(result);

        if overflow {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }

        // Check if the sign bit changed
        if (word >> 7) != (result >> 7) {
            cpu.SR.set(Flags::Overflow);
        } else {
            cpu.SR.reset(Flags::Overflow);
        }

        cpu.AC = result;
    }
}

impl FromImmediate for Adc {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        Adc::add_with_carry(cpu, word);

        2
    }
}

impl FromAddress for Adc {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());

        Adc::add_with_carry(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            AddressMode::AbsoluteX => { 4 + (extra_cycles as u32) },
            AddressMode::AbsoluteY => { 4 + (extra_cycles as u32) },
            AddressMode::IndirectY => { 5 + (extra_cycles as u32) },
            _ => { panic!("unimplemented address mode {:?} for ADC", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;
    use crate::nes::cpu::status::Flags;

    // TODO Write tests by figuring out a better way to make a mock CPU
    // now that we have abstracted out the address resolution stuff
}
