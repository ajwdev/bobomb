use nes::cpu::{Cpu,FromImplied,AddressMode};

pub struct Iny { }

impl FromImplied for Iny {
    fn from_implied(cpu: &mut Cpu) -> usize {
        let word = cpu.Y.wrapping_add(1);
        // See Dey about why we have `word`
        cpu.Y = word;

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
    fn test_iny() {
        let mut cpu = mock_cpu(&[0xc8]);
        cpu.Y = 10;

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::Y, 11);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);

        cpu.rewind();
        cpu.Y = 255;
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::Y, 0);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);

        cpu.rewind();
        cpu.Y = 127;
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::Y, 128);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);
    }
}
