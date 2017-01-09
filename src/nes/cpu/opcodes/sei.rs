use nes::cpu::{Cpu,Implied};
use nes::cpu::status::Flags;
use super::store::Store;

pub struct Sei { }

impl Implied for Sei {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.SR.set_interrupt();
        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::status::Flags;

    #[test]
    fn test_sei() {
        let mut cpu = mock_cpu(&[0x78]);
        cpu.SR.reset(Flags::Interrupt);

        cpu.step(None);
        assert!(cpu.SR.is_set(Flags::Interrupt),
                "expected true, got {:#?}",
                cpu.SR.is_set(Flags::Interrupt));
    }
}
