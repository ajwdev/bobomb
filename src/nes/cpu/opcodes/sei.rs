use nes::cpu::{Cpu,Implied};
use super::store::Store;

pub struct Sei { }

impl Implied for Sei {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.SR.Interrupt = true;
        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_sei() {
        let mut cpu = mock_cpu(&[0x78]);
        cpu.SR.Interrupt = false;

        cpu.execute_instruction();
        assert!(cpu.SR.Interrupt == true,
                "expected true, got {:#?}",
                cpu.SR.Interrupt);
    }
}
