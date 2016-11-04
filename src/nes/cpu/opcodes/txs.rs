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
    use nes::cpu::Registers;

    #[test]
    fn test_txs() {
        let mut cpu = mock_cpu(&[0x9a]);
        cpu.X = 0xf0;
        cpu.SP = 0x00;

        assert_cpu_register!(cpu, Registers::X, 0xf0);
        assert_cpu_register!(cpu, Registers::SP, 0x00);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::SP, 0xf0);
        //TODO Make assertions on status registers
    }
}
