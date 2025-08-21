use crate::nes::cpu::{AddressMode, Cpu, FromAddress};

pub struct Jmp {}

impl FromAddress for Jmp {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, _) = cpu.translate_address(mode);
        cpu.PC = src.into();

        match mode {
            AddressMode::Absolute => 3,
            AddressMode::Indirect => 5,
            _ => {
                panic!("unimplemented address mode {:?} for JMP", mode);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;

    #[test]
    fn test_jmp() {
        let mut cpu = mock_cpu(&[0x4c, 0xef, 0xbe]);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.step(None);
        assert!(cpu.PC == 0xbeef, "expected 0xbeef, got {:#x}", cpu.PC);
    }
}
