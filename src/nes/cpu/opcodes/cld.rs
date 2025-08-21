use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{Cpu, FromImplied};

pub struct Cld {}

impl FromImplied for Cld {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        cpu.SR.reset(Flags::Decimal);
        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;

    #[test]
    fn test_cld() {
        let mut cpu = mock_cpu(&[0xd8]);

        cpu.SR.set(Flags::Decimal);
        assert!(
            cpu.SR.is_set(Flags::Decimal),
            "expected true, got {:#?}",
            cpu.SR.is_set(Flags::Decimal)
        );
        cpu.step(None);
        assert!(
            !cpu.SR.is_set(Flags::Decimal),
            "expected false, got {:#?}",
            cpu.SR.is_set(Flags::Decimal)
        );
    }
}
