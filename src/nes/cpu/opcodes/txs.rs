use nes::cpu::{Cpu,Implied,STACK_START};

pub struct Txs { }

impl Implied for Txs {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.SP = cpu.X;
        println!("Stack pointer changed: {:#06x}", cpu.SP as u16 + STACK_START);
        cpu.stack_depth = 0;

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_txs() {
        let mut cpu = mock_cpu(&[0x9a]);
        cpu.X = 0xf0;
        cpu.SP = 0x00;

        assert!(cpu.X == 0xf0, "expected 0xf0, got {:#x}", cpu.X);
        assert!(cpu.SP == 0x00, "expected 0x00, got {:#x}", cpu.SP);
        cpu.execute_instruction();
        assert!(cpu.SP == 0xf0, "expected 0xf0, got {:#x}", cpu.SP);
        //TODO Make assertions on status registers
    }
}
