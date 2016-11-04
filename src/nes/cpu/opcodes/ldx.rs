use nes::cpu::{Cpu,Registers,Immediate};
use super::load::Load;

pub struct Ldx { }

impl Immediate for Ldx {
    fn immediate(cpu: &mut Cpu) -> usize {
        Load::immediate(cpu, Registers::X)
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_ldx_imm() {
        let mut cpu = mock_cpu(&[0xa2, 0xff]);

        assert_cpu_register!(cpu, Registers::X, 0);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::X, 0xff);
    }
}
