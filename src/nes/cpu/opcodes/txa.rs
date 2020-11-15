use crate::nes::cpu::{Cpu,Implied};

pub struct Txa { }

impl Implied for Txa {
    fn implied(cpu: &mut Cpu) -> usize {
        let result = cpu.X;
        cpu.AC = result;

        cpu.zero_and_negative_status(result);

        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;

    #[test]
    fn test_txa() {
        let mut cpu = mock_cpu(&[0x8a]);
        cpu.X = 0xf0;
        cpu.AC = 0x00;

        assert_cpu_register!(cpu, Registers::X, 0xf0);
        assert_cpu_register!(cpu, Registers::AC, 0x00);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0xf0);
        //TODO Make assertions on status registers
    }
}
