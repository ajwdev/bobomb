use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{Cpu, FromImplied};

pub struct Sei {}

impl FromImplied for Sei {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        cpu.SR.set(Flags::Interrupt);
        2
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;

    #[test]
    fn test_sei() {
        let mut cpu = mock_cpu(&[0x78]);
        cpu.SR.reset(Flags::Interrupt);

        cpu.step(None);
        assert!(
            cpu.SR.is_set(Flags::Interrupt),
            "expected true, got {:#?}",
            cpu.SR.is_set(Flags::Interrupt)
        );
    }
}
