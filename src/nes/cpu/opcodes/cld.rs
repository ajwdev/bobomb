use nes::cpu::{Cpu,Implied};
use super::store::Store;

pub struct Cld { }

impl Implied for Cld {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.SR.Decimal = false;
        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_cld() {
        let mut cpu = mock_cpu(&[0xd8]);

        cpu.SR.Decimal = true;
        assert!(cpu.SR.Decimal == true,
                "expected true, got {:#?}",
                cpu.SR.Decimal);
        cpu.execute_instruction();
        assert!(cpu.SR.Decimal == false,
                "expected false, got {:#?}",
                cpu.SR.Decimal);
    }
}
