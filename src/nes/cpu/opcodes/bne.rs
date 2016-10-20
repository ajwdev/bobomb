use nes::cpu::{Cpu,Relative};
use super::branch::Branch;

pub struct Bne { }

impl Relative for Bne {
    fn relative(cpu: &mut Cpu) -> usize {
        Branch::branch_on_true(cpu, |c| !c.SR.Zero);

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_bne_skip() {
        let mut cpu = mock_cpu(&[0xd0, 0xff]);

        cpu.SR.Zero = true;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction();
        assert!(cpu.PC == 0x8002, "expected 0x8002, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_bne_take_positive() {
        let mut cpu = mock_cpu(&[0xd0, 0x2a]);

        cpu.SR.Zero = false;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction(); // Two byte instruction so *add* two below
        assert!(cpu.PC == 0x802c, "expected 0x802a, got {:#x}", cpu.PC);
    }

    #[test]
    fn test_bne_take_negative() {
        let mut cpu = mock_cpu(&[0xd0, 0x82]); // hex 0x82 is signed -126

        cpu.SR.Zero = false;
        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);
        cpu.execute_instruction(); // Two byte instruction so *substract* two bytes below

        // NOTE For posterity, this actually drops us below the ROM
        // start range which I dont think will happen with real ROMs.
        // This should be fine for our test though.
        assert!(cpu.PC == 0x7f84, "expected 0x7f82, got {:#x}", cpu.PC);
    }
}
