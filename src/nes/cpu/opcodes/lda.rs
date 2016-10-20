use nes::cpu::{Cpu,Destination,Immediate,Absolute};
use super::load::Load;

pub struct Lda { }

impl Immediate for Lda {
    fn immediate(cpu: &mut Cpu) -> usize {
        Load::immediate(cpu, Destination::RegAC)
    }
}

impl Absolute for Lda {
    fn absolute(cpu: &mut Cpu) -> usize {
        Load::absolute(cpu, Destination::RegAC);
        4
    }
}

mod test {
    use nes::cpu::Cpu;
    use nes::cpu::test::*;
    use nes::address::{AddressSpace,Bank};

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
}
