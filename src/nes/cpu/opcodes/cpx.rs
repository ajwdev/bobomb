use crate::nes::cpu::{Cpu,FromImmediate};
use crate::nes::cpu::status::Flags;

pub struct Cpx { }

impl FromImmediate for Cpx {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        let result = cpu.X.wrapping_sub(word);

        cpu.zero_and_negative_status(result);

        if cpu.X >= word {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }

        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::status::Flags;

    #[test]
    fn test_cpx() {
        let mut cpu = mock_cpu(&[0xc0,10]);
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
