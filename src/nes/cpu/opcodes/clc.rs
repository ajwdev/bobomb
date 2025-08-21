use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{Cpu, FromImplied};

pub struct Clc {}

impl FromImplied for Clc {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        cpu.SR.reset(Flags::Carry);
        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;

    #[test]
    fn test_clc() {
        let mut cpu = mock_cpu(&[0x18]);

        // TODO Cleanup Cld so it looks more like this
        cpu.SR.set(Flags::Carry);
        cpu.step(None);
        assert_status_reset!(cpu, Flags::Carry);
    }
}
