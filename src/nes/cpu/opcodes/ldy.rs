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
    use nes::cpu::Registers;

    #[test]
    fn test_ldy_imm() {
        let mut cpu = mock_cpu(&[0xa0, 0xff]);

        assert_cpu_register!(cpu, Registers::Y, 0);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::Y, 0xff);
    }
}
