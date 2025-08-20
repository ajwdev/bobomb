use crate::nes::cpu::{Cpu, FromImplied};

pub struct Tya {}

impl FromImplied for Tya {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        let result = cpu.Y;
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
    fn test_tya() {
        let mut cpu = mock_cpu(&[0x98]);
        cpu.Y = 0xf0;
        cpu.AC = 0x00;

        assert_cpu_register!(cpu, Registers::Y, 0xf0);
        assert_cpu_register!(cpu, Registers::AC, 0x00);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0xf0);
        //TODO Make assertions on status registers
    }
}
