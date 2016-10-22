use nes::cpu::{Cpu,Implied};
use nes::cpu::status::Flags;
use super::store::Store;

pub struct Cld { }

impl Implied for Cld {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.SR.reset_decimal();
        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::status::Flags;

    #[test]
    fn test_cld() {
        let mut cpu = mock_cpu(&[0xd8]);

        cpu.SR.set_decimal();
        assert!(cpu.SR.is_set(Flags::Decimal),
                "expected true, got {:#?}",
                cpu.SR.is_set(Flags::Decimal));
        cpu.execute_instruction();
        assert!(!cpu.SR.is_set(Flags::Decimal),
                "expected false, got {:#?}",
                cpu.SR.is_set(Flags::Decimal));
    }
}
