use nes::cpu::{Cpu,Implied};
use nes::cpu::status::Flags;

pub struct Clc { }

impl Implied for Clc {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.SR.reset(Flags::Carry);
        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::status::Flags;

    #[test]
    fn test_clc() {
        let mut cpu = mock_cpu(&[0x18]);

        // TODO Cleanup Cld so it looks more like this
        cpu.SR.set(Flags::Carry);
        cpu.execute_instruction();
        assert_status_reset!(cpu, Flags::Carry);
    }
}
