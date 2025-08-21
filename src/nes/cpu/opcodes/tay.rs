use crate::nes::cpu::{Cpu, FromImplied};

pub struct Tay {}

impl FromImplied for Tay {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        // XXX This intermediate variable only exists because we can't
        // pass the borrowed CPU to zero_and_negative_status. Fix this?
        let result = cpu.AC;
        cpu.Y = result;

        cpu.zero_and_negative_status(result);

        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;

    #[test]
    fn test_tay() {
        let mut cpu = mock_cpu(&[0xa8, 0xa8]);
        cpu.Y = 0xf0;
        cpu.AC = 0x00;
        cpu.SR.reset(Flags::Zero);

        assert_cpu_register!(cpu, Registers::Y, 0xf0);
        assert_cpu_register!(cpu, Registers::AC, 0x00);

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::Y, 0x00);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);

        cpu.AC = 0xff;
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::Y, 0xff);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);
    }
}
