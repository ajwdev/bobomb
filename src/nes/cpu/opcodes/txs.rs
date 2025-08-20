use crate::nes::cpu::{Cpu, FromImplied, STACK_START};
use tracing::debug;

pub struct Txs {}

impl FromImplied for Txs {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        cpu.SP = cpu.X;
        debug!(
            "Stack pointer changed: {:#06x}",
            cpu.SP as u16 + STACK_START
        );
        cpu.stack_depth = 0;

        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;

    #[test]
    fn test_txs() {
        let mut cpu = mock_cpu(&[0x9a]);
        cpu.X = 0xf0;
        cpu.SP = 0x00;

        assert_cpu_register!(cpu, Registers::X, 0xf0);
        assert_cpu_register!(cpu, Registers::SP, 0x00);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::SP, 0xf0);
        //TODO Make assertions on status registers
    }
}
