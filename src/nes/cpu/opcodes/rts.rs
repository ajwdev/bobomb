use crate::nes::address::Address;
use crate::nes::cpu::{Cpu, FromImplied};

pub struct Rts {}

impl FromImplied for Rts {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        let lo = cpu.pop_word();
        let hi = cpu.pop_word();
        let addr = Address::new(hi, lo);

        // See JSR for an explanation of why we add one
        cpu.PC = addr.to_u16().wrapping_add(1);

        6
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;

    #[test]
    fn test_rts() {
        let mut cpu = mock_cpu(&[0x60]);
        cpu.push_word(0xBE);
        cpu.push_word(0xEE);

        cpu.step(None);
        assert!(cpu.PC == 0xbeef, "expected 0xbeef, got {:#x}", cpu.PC);
    }
}
