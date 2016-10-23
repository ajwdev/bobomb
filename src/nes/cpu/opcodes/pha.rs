use nes::cpu::{Cpu,Registers,Implied};

pub struct Pha { }

impl Implied for Pha {
    fn implied(cpu: &mut Cpu) -> usize {
        let reg = cpu.AC;
        cpu.push_stack(reg);

        3
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_pha() {
        let mut cpu = mock_cpu(&[0x48]);
        cpu.AC = 0xFF;

        cpu.execute_instruction();

        let mut result = cpu.pop_stack();
        assert!(result == 0xFF, "expected 0xFF, got {:#x}", result);
    }
}
