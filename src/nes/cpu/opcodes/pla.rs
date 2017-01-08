use nes::cpu::{Cpu,FromImplied};

pub struct Pla { }

impl FromImplied for Pla {
    fn from_implied(cpu: &mut Cpu) -> usize {
        cpu.AC = cpu.pop_stack();

        4
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_pla() {
        let mut cpu = mock_cpu(&[0x68]);
        cpu.AC = 0xFF;
        cpu.push_stack(0xAA);

        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0xAA);
    }
}
