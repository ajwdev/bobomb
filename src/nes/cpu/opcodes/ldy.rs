use nes::cpu::{Cpu,Registers,Immediate};
use super::load::Load;

pub struct Ldy { }

impl Immediate for Ldy {
    fn immediate(cpu: &mut Cpu) -> usize {
        Load::immediate(cpu, Registers::Y)
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_ldy_imm() {
        let mut cpu = mock_cpu(&[0xa0, 0xff]);

        assert!(cpu.Y == 0, "expected 0, got {:#x}", cpu.Y);
        cpu.execute_instruction();
        assert!(cpu.Y == 0xff, "expected 0xff, got {:#x}", cpu.Y);
    }
}
