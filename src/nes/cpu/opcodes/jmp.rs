use nes::cpu::{Cpu,Absolute};

pub struct Jmp { }

impl Absolute for Jmp {
    fn absolute(cpu: &mut Cpu) -> usize {
        // DRY this up with JSR
        let addr = cpu.read_dword_and_increment();
        cpu.PC = addr;

        3
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_jmp() {
        let mut cpu = mock_cpu(&[0x4c, 0xef, 0xbe]);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction();
        assert!(cpu.PC == 0xbeef, "expected 0xbeef, got {:#x}", cpu.PC);
    }
}
