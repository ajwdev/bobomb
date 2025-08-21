use crate::nes::cpu::{Cpu, FromImplied};

pub struct Pha {}

impl FromImplied for Pha {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        let reg = cpu.AC;
        cpu.push_word(reg);

        3
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;

    #[test]
    fn test_pha() {
        let mut cpu = mock_cpu(&[0x48]);
        cpu.AC = 0xFF;

        cpu.step(None);

        let mut result = cpu.pop_word();
        assert!(result == 0xFF, "expected 0xFF, got {:#x}", result);
    }
}
