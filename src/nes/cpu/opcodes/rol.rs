use crate::nes::cpu::{Cpu,FromAddress,FromAccumulator,AddressMode};
use crate::nes::cpu::status::Flags;

pub struct Rol { }

impl Rol {
    #[inline]
    fn rotate_one_bit_left(cpu: &mut Cpu, mut word: u8) -> u8 {
        let old_carry_set = cpu.SR.is_set(Flags::Carry);
        let new_carry_set = (0b10000000 & word) > 0;

        word = word << 1;
        if old_carry_set {
            word |= 0x1;
        } else {
            word &= !0x1;
        }

        cpu.zero_and_negative_status(word);

        if new_carry_set {
            cpu.SR.set(Flags::Carry)
        } else {
            cpu.SR.reset(Flags::Carry)
        }

        word
    }
}

impl FromAddress for Rol {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, _) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());
        let result = Self::rotate_one_bit_left(cpu, word);

        cpu.write_at(src.to_u16(), result);

        match mode {
            AddressMode::ZeroPage => 5,
            _ => { panic!("unimplemented address mode {:?} for ROL", mode); }
        }
    }
}

impl FromAccumulator for Rol {
    fn from_accumulator(cpu: &mut Cpu) -> u32 {
        let word = cpu.AC;
        let _result = Self::rotate_one_bit_left(cpu, word);

        cpu.AC = word;

        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;
    use crate::nes::cpu::status::Flags;

    #[test]
    fn test_eor() {
        let mut cpu = mock_cpu(&[0x66,0xFF]);

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
