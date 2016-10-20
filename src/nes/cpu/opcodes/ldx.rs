use nes::cpu::{Cpu,Registers,Immediate};
use super::load::Load;

pub struct Ldx { }

impl Immediate for Ldx {
    fn immediate(cpu: &mut Cpu) -> usize {
        Load::immediate(cpu, Registers::X)
    }
}

mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_ldx_imm() {
        let mut cpu = mock_cpu(&[0xa2, 0xff]);

        assert!(cpu.X == 0, "expected 0, got {:#x}", cpu.X);
        cpu.execute_instruction();
        assert!(cpu.X == 0xff, "expected 0xff, got {:#x}", cpu.X);
    }
}
