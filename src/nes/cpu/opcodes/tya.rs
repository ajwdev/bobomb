use nes::cpu::{Cpu,Implied};

pub struct Tya { }

impl Implied for Tya {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.AC = cpu.Y;

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_tya() {
        let mut cpu = mock_cpu(&[0x98]);
        cpu.Y = 0xf0;
        cpu.AC = 0x00;

        assert!(cpu.Y == 0xf0, "expected 0xf0, got {:#x}", cpu.Y);
        assert!(cpu.AC == 0x00, "expected 0x00, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xf0, "expected 0xf0, got {:#x}", cpu.AC);
        //TODO Make assertions on status registers
    }
}
