use nes::cpu::{Cpu,AddressMode,FromAddress,FromImmediate};
use nes::cpu::status::Flags;
use nes::address::Address;

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

        // TODO Consider breaking this out
        if (cpu.AC ^ result) == 0 {
            cpu.SR.reset(Flags::Overflow);
        } else {
            cpu.SR.reset(Flags::Overflow);
        }

        cpu.AC = result;
    }
}

impl FromImmediate for Adc {
    fn from_immediate(cpu: &mut Cpu) -> usize {
        let word = cpu.read_word_and_increment();
        Adc::add_with_carry(cpu, word);

        2
    }
}

impl FromAddress for Adc {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let src = cpu.translate_address(mode);
        let word = cpu.mem.read_word(src.to_u16());

        Adc::add_with_carry(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            _ => { panic!("unimplemented address mode {:?} for ADC", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;
    use nes::cpu::status::Flags;

    // TODO Write tests by figuring out a better way to make a mock CPU
    // now that we have abstracted out the address resolution stuff
}
