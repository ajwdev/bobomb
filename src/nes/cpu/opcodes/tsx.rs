use crate::nes::cpu::{Cpu,FromAddress,AddressMode};


pub struct Tsx { }

impl FromAddress for Tsx {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        cpu.X = cpu.SP;
        cpu.zero_and_negative_status(cpu.X);

        match mode {
            AddressMode::Implied => 2,
            // TODO Make a macro for this
            _ => { panic!("unimplemented address mode {:?} for TSX", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::status::Flags;

    #[test]
    fn test_tsx() {
        let mut cpu = mock_cpu(&[0xba]);
        cpu.X = 0xff;
        cpu.SP = 0xbe;

        cpu.step(None).unwrap();
        assert!(cpu.X == cpu.SP, "expected {:#x}, got {:#x}", cpu.SP, cpu.X);
    }

    #[test]
    fn test_tsx_zero() {
        let mut cpu = mock_cpu(&[0xba]);
        cpu.X = 0xff;
        cpu.SP = 0x00;

        // Clear so we know they get set
        cpu.SR.reset(Flags::Zero);

        cpu.step(None).unwrap();
        assert!(cpu.X == cpu.SP, "expected {:#x}, got {:#x}", cpu.SP, cpu.X);
        assert!(cpu.SR.is_set(Flags::Zero), "expected zero flag to be set but is clear")
    }

    #[test]
    fn test_tsx_negative() {
        let mut cpu = mock_cpu(&[0xba]);
        cpu.X = 0xbe;
        cpu.SP = 0xff;

        // Clear so we know they get set
        cpu.SR.reset(Flags::Negative);

        cpu.step(None).unwrap();
        assert!(cpu.X == cpu.SP, "expected {:#x}, got {:#x}", cpu.SP, cpu.X);
        assert!(cpu.SR.is_set(Flags::Negative), "expected negative flag to be set but is clear")
    }
}
