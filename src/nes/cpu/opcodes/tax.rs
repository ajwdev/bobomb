use nes::cpu::{Cpu,FromImplied};

pub struct Tax { }

impl FromImplied for Tax {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        // XXX This intermediate variable only exists because we can't
        // pass the borrowed CPU to zero_and_negative_status. Fix this?
        let result = cpu.AC;
        cpu.X = result;

        cpu.zero_and_negative_status(result);

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;
    use nes::cpu::status::Flags;

    #[test]
    fn test_tax() {
        let mut cpu = mock_cpu(&[0xaa,0xaa]);
        cpu.X = 0xf0;
        cpu.AC = 0x00;
        cpu.SR.reset(Flags::Zero);

        assert_cpu_register!(cpu, Registers::X, 0xf0);
        assert_cpu_register!(cpu, Registers::AC, 0x00);

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::X, 0x00);
        // TODO Add these to TXA
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);

        cpu.AC = 0xff;
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::X, 0xff);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);
    }
}

