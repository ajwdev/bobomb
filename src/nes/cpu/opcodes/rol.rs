use nes::cpu::{Cpu,FromAddress,FromAccumulator,AddressMode};
use nes::cpu::status::Flags;

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
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let (src, _) = cpu.translate_address(mode);
        let word = cpu.interconnect.read_word(src.to_u16());
        let result = Self::rotate_one_bit_left(cpu, word);

        cpu.interconnect.write_word(src.to_u16(), result);

        match mode {
            AddressMode::ZeroPage => 5,
            _ => { panic!("unimplemented address mode {:?} for ROL", mode); }
        }
    }
}

impl FromAccumulator for Rol {
    fn from_accumulator(cpu: &mut Cpu) -> usize {
        let word = cpu.AC;
        let result = Self::rotate_one_bit_left(cpu, word);

        cpu.AC = word;

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;
    use nes::cpu::status::Flags;

    #[test]
    fn test_eor() {
        let mut cpu = mock_cpu(&[0x66,0xFF]);

        cpu.interconnect.write_word(0x00FF, 0b10000000);
        cpu.SR.set(Flags::Carry);
        cpu.SR.reset(Flags::Zero);
        cpu.SR.reset(Flags::Negative);

        cpu.step(None);
        let result = cpu.interconnect.read_word(0x00FF);
        assert_equalx!(0b11000000, result);
        assert_status_reset!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);


        cpu.rewind();
        cpu.interconnect.write_word(0x00FF, 0b10000001);
        cpu.SR.reset(Flags::Carry);
        cpu.SR.reset(Flags::Zero);
        cpu.SR.reset(Flags::Negative);

        cpu.step(None);
        let result = cpu.interconnect.read_word(0x00FF);
        assert_equalx!(0b01000000, result);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);


        cpu.rewind();
        cpu.interconnect.write_word(0x00FF, 0b00000001);
        cpu.SR.reset(Flags::Carry);
        cpu.SR.reset(Flags::Zero);
        cpu.SR.reset(Flags::Negative);

        cpu.step(None);
        let result = cpu.interconnect.read_word(0x00FF);
        assert_equalx!(0b00000000, result);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
    }
}
