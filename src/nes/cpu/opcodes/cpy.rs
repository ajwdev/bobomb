use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{AddressMode, FromAddress};
use crate::nes::cpu::{Cpu, FromImmediate};

pub struct Cpy {}

impl Cpy {
    #[inline]
    fn copy(cpu: &mut Cpu, word: u8) {
        let result = cpu.Y.wrapping_sub(word);

        cpu.zero_and_negative_status(result);

        if cpu.Y >= word {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }
    }
}

impl FromImmediate for Cpy {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        Self::copy(cpu, word);

        2
    }
}

impl FromAddress for Cpy {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, _) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());

        Self::copy(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            _ => {
                panic!("unimplemented address mode {:?} for CPY", mode);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;

    #[test]
    fn test_cpy() {
        let mut cpu = mock_cpu(&[0xc0, 10]);
        cpu.Y = 100;

        cpu.step(None);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Carry);

        cpu.rewind();
        cpu.Y = 240;
        cpu.step(None);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Carry);

        cpu.rewind();
        cpu.Y = 10;
        cpu.step(None);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Carry);

        cpu.rewind();
        cpu.Y = 2;
        cpu.step(None);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);
        assert_status_reset!(cpu, Flags::Carry);
    }
}
