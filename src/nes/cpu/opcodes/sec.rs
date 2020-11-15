use crate::nes::cpu::{Cpu,FromImplied};
use crate::nes::cpu::status::Flags;
use super::store::Store;

pub struct Sec { }

impl FromImplied for Sec {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        cpu.SR.set(Flags::Carry);
        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::status::Flags;

    #[test]
    fn test_sec() {
        let mut cpu = mock_cpu(&[0x38]);
        cpu.SR.reset(Flags::Carry);

        cpu.step(None);
        assert_status_set!(cpu, Flags::Carry);
    }
}

