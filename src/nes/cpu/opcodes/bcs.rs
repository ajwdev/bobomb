use nes::cpu::{Cpu,FromRelative};
use nes::cpu::status::Flags;
use super::branch::Branch;

pub struct Bcs { }

impl FromRelative for Bcs {
    fn from_relative(cpu: &mut Cpu) -> u32 {
        Branch::branch_on_true(cpu, |c| c.SR.is_set(Flags::Carry));

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::status::Flags;

    #[test]
    fn test_beq_skip() {
        let mut cpu = mock_cpu(&[0xb0, 0xff]);

        cpu.SR.reset(Flags::Carry);
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.step(None);
        assert!(cpu.PC == 0x8002, "expected 0x8002, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_beq_take_positive() {
        let mut cpu = mock_cpu(&[0xb0, 0x2a]);

        cpu.SR.set(Flags::Carry);
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.step(None); // Two byte instruction so *add* two below
        assert!(cpu.PC == 0x802c, "expected 0x802a, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_beq_take_negative() {
        let mut cpu = mock_cpu(&[0xb0, 0x82]); // hex 0x82 is signed -126

        cpu.SR.set(Flags::Carry);
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.step(None); // Two byte instruction so *substract* two bytes below

        // NOTE For posterity, this actually drops us below the ROM
        // start range which I dont think will happen with real ROMs.
        // This should be fine for our test though.
        assert!(cpu.PC == 0x7f84, "expected 0x7f82, got {:#x}", cpu.PC);
    }
}

