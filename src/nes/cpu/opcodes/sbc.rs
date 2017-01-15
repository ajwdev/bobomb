use nes::cpu::{Cpu,AddressMode,FromAddress,FromImmediate};
use nes::cpu::status::Flags;
use nes::address::Address;

pub struct Sbc { }

impl Sbc {
    fn subtract_with_carry(cpu: &mut Cpu, word: u8) {
        // We ignore the Decimal status register because on the NES
        // it is unused. Consider adding support in the future.
        let tmp = word.wrapping_sub(cpu.SR.is_set(Flags::Carry) as u8);
        let (result, overflow) = cpu.AC.overflowing_add(tmp);

        cpu.zero_and_negative_status(result);

        if overflow {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }

        if ((word >> 7) != (result >> 7)) {
            cpu.SR.set(Flags::Overflow);
        } else {
            cpu.SR.reset(Flags::Overflow);
        }

        cpu.AC = result;
    }
}

impl FromImmediate for Sbc {
    fn from_immediate(cpu: &mut Cpu) -> usize {
        let word = cpu.read_word_and_increment();
        Sbc::subtract_with_carry(cpu, word);

        2
    }
}

impl FromAddress for Sbc {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let src = cpu.translate_address(mode);
        let word = cpu.interconnect.read_word(src.to_u16());

        Sbc::subtract_with_carry(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            _ => { panic!("unimplemented address mode {:?} for SBC", mode); }
        }
    }
}

