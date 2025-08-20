use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{AddressMode, Cpu, FromAccumulator, FromAddress};

pub struct Ror {}

impl Ror {
    #[inline]
    fn rotate_right(cpu: &mut Cpu, word: u8) -> u8 {
        let old_carry_set = cpu.SR.is_set(Flags::Carry);
        let new_carry_set = (0x1 & word) > 0;

        let mut result = word >> 1;
        if old_carry_set {
            result |= 0b10000000;
        } else {
            result &= !0b10000000;
        }

        cpu.zero_and_negative_status(result);

        if new_carry_set {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }

        result
    }
}

impl FromAddress for Ror {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.read_at(src);

        let result = Ror::rotate_right(cpu, word);

        cpu.write_at(src, result);

        match mode {
            AddressMode::ZeroPage => 5,
            AddressMode::Absolute => 6,
            AddressMode::AbsoluteX => 7,
            _ => {
                panic!("unimplemented address mode {:?} for ROR", mode);
            }
        }
    }
}

impl FromAccumulator for Ror {
    fn from_accumulator(cpu: &mut Cpu) -> u32 {
        let word = cpu.AC;
        let result = Ror::rotate_right(cpu, word);
        cpu.AC = result;

        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;

    #[test]
    fn test_eor() {
        let mut cpu = mock_cpu(&[0x66, 0xFF]);

        cpu.write_at(0x00FF, 0b10000000);
        cpu.SR.set(Flags::Carry);
        cpu.SR.reset(Flags::Zero);
        cpu.SR.reset(Flags::Negative);

        cpu.step(None);
        let result = cpu.read_at(0x00FF);
        assert_equalx!(0b11000000, result);
        assert_status_reset!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);

        cpu.rewind();
        cpu.write_at(0x00FF, 0b10000001);
        cpu.SR.reset(Flags::Carry);
        cpu.SR.reset(Flags::Zero);
        cpu.SR.reset(Flags::Negative);

        cpu.step(None);
        let result = cpu.read_at(0x00FF);
        assert_equalx!(0b01000000, result);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);

        cpu.rewind();
        cpu.write_at(0x00FF, 0b00000001);
        cpu.SR.reset(Flags::Carry);
        cpu.SR.reset(Flags::Zero);
        cpu.SR.reset(Flags::Negative);

        cpu.step(None);
        let result = cpu.read_at(0x00FF);
        assert_equalx!(0b00000000, result);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
    }
}
