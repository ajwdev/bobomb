use nes::cpu::{Cpu,FromImmediate};
use nes::cpu::status::Flags;

pub struct Cpy { }

impl FromImmediate for Cpy {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();;
        let result = cpu.Y.wrapping_sub(word);

        cpu.zero_and_negative_status(result);

        if cpu.Y >= word {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;
    use nes::cpu::status::Flags;

    #[test]
    fn test_cpy() {
        let mut cpu = mock_cpu(&[0xc0,10]);
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
