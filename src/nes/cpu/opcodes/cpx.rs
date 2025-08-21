use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{AddressMode, Cpu, FromAddress, FromImmediate};

pub struct Cpx {}

impl Cpx {
    fn compare_x(cpu: &mut Cpu, word: u8) {
        let result = cpu.X.wrapping_sub(word);

        cpu.zero_and_negative_status(result);

        if cpu.X >= word {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }
    }
}

impl FromImmediate for Cpx {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        Self::compare_x(cpu, word);
        2
    }
}

impl FromAddress for Cpx {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, _) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());
        
        Self::compare_x(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            _ => {
                panic!("unimplemented address mode {:?} for CPX", mode);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;

    #[test]
    fn test_cpx() {
        let mut cpu = mock_cpu(&[0xc0, 10]);
        cpu.X = 100;

        cpu.step(None);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Carry);

        cpu.rewind();
        cpu.X = 240;
        cpu.step(None);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Carry);

        cpu.rewind();
        cpu.X = 10;
        cpu.step(None);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Carry);

        cpu.rewind();
        cpu.X = 2;
        cpu.step(None);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);
        assert_status_reset!(cpu, Flags::Carry);
    }
}
