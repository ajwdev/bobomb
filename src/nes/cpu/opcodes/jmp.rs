use nes::cpu::{Cpu,Absolute,FromAddress,AddressMode};

pub struct Jmp { }

impl Absolute for Jmp {
    fn absolute(cpu: &mut Cpu) -> usize {
        // DRY this up with JSR
        let addr = cpu.read_dword_and_increment();
        cpu.PC = addr;

        3
    }
}

impl FromAddress for Jmp {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, _) = cpu.translate_address(mode);
        cpu.PC = src.into();

        match mode {
            AddressMode::Absolute => 3,
            AddressMode::Indirect => 5,
            _ => { panic!("unimplemented address mode {:?} for JMP", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_jmp() {
        let mut cpu = mock_cpu(&[0x4c, 0xef, 0xbe]);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.step(None);
        assert!(cpu.PC == 0xbeef, "expected 0xbeef, got {:#x}", cpu.PC);
    }
}
