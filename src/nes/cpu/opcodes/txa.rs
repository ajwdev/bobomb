use nes::cpu::{Cpu,Implied};

pub struct Txa { }

impl Implied for Txa {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.AC = cpu.X;

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_txa() {
        let mut cpu = mock_cpu(&[0x8a]);
        cpu.X = 0xf0;
        cpu.AC = 0x00;

        assert!(cpu.X == 0xf0, "expected 0xf0, got {:#x}", cpu.X);
        assert!(cpu.AC == 0x00, "expected 0x00, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0xf0, "expected 0xf0, got {:#x}", cpu.AC);
        //TODO Make assertions on status registers
    }
}
