use nes::cpu::{Cpu,Implied};

pub struct Txa { }

impl Implied for Txa {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.AC = cpu.X;

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_txa() {
        let mut cpu = mock_cpu(&[0x8a]);
        cpu.X = 0xf0;
        cpu.AC = 0x00;

        assert_cpu_register!(cpu, Registers::X, 0xf0);
        assert_cpu_register!(cpu, Registers::AC, 0x00);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0xf0);
        //TODO Make assertions on status registers
    }
}
