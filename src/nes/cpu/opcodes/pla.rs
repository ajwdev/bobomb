use nes::cpu::{Cpu,FromImplied};

pub struct Pla { }

impl FromImplied for Pla {
    fn from_implied(cpu: &mut Cpu) -> usize {
        cpu.AC = cpu.pop_word();

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
        cpu.push_word(0xAA);

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0xAA);
    }
}

