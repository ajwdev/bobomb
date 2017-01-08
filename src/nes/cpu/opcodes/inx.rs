use nes::cpu::{Cpu,FromImplied,AddressMode};

pub struct Inx { }

impl FromImplied for Inx {
    fn from_implied(cpu: &mut Cpu) -> usize {
        let word = cpu.X.wrapping_add(1);
        // See Dey about why we have `word`
        cpu.X = word;

        cpu.zero_and_negative_status(word);

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;
    use nes::cpu::status::Flags;

    #[test]
    fn test_inx() {
        let mut cpu = mock_cpu(&[0xe8]);
        cpu.X = 10;

        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::X, 11);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);

        cpu.rewind();
        cpu.X = 255;
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::X, 0);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);

        cpu.rewind();
        cpu.X = 127;
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::X, 128);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);
    }
}


