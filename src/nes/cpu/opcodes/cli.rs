use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{Cpu, FromImplied};

pub struct Cli {}

impl FromImplied for Cli {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        cpu.SR.reset(Flags::Interrupt);
        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;

    #[test]
    fn test_cli_clears_interrupt_flag() {
        let mut cpu = mock_cpu(&[0x58]);
        cpu.SR.set(Flags::Interrupt);

        assert_status_set!(cpu, Flags::Interrupt);
        cpu.step(None).unwrap();
        assert_status_reset!(cpu, Flags::Interrupt);
    }

    #[test]
    fn test_cli_when_interrupt_already_clear() {
        let mut cpu = mock_cpu(&[0x58]);
        cpu.SR.reset(Flags::Interrupt);

        assert_status_reset!(cpu, Flags::Interrupt);
        cpu.step(None).unwrap();
        assert_status_reset!(cpu, Flags::Interrupt);
    }

    #[test]
    fn test_cli_timing() {
        let mut cpu = mock_cpu(&[0x58]);
        let cycles = cpu.step(None).unwrap();
        assert_eq!(cycles, 2);
    }
}
