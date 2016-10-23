use nes::cpu::{Cpu,Registers,ZeroPage,Immediate,Absolute};
use super::load::Load;

pub struct Lda { }

impl Immediate for Lda {
    fn immediate(cpu: &mut Cpu) -> usize {
        Load::immediate(cpu, Registers::AC)
    }
}

impl Absolute for Lda {
    fn absolute(cpu: &mut Cpu) -> usize {
        Load::absolute(cpu, Registers::AC);
        4
    }
}

impl ZeroPage for Lda {
    fn zero_page(cpu: &mut Cpu) -> usize {
        Load::zero_page(cpu, Registers::AC);
        3
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_lda_abs() {
        let mut cpu = mock_cpu(&[0xad, 0x03, 0x80, 0xff]);

        assert!(cpu.AC == 0, "expected 0, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xff, "expected 0xff, got {:#x}", cpu.AC);
    }

    #[test]
    fn test_lda_imm() {
        let mut cpu = mock_cpu(&[0xa9, 0xff]);

        assert!(cpu.AC == 0, "expected 0, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xff, "expected 0xff, got {:#x}", cpu.AC);
    }

    #[test]
    fn test_lda_zeropage() {
        let mut cpu = mock_cpu(&[0xa5, 0xff]);
        cpu.mem.write_word(0x00ff, 0xbe);

        assert!(cpu.AC == 0, "expected 0, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xbe, "expected 0xbe, got {:#x}", cpu.AC);
    }
}