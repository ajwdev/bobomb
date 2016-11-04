use nes::cpu::{Cpu,Implied};

pub struct Dey { }

impl Implied for Dey {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.Y = cpu.Y.wrapping_sub(1);
        // TODO The reason we create `word` here is because we can't pass cpu.Y to
        // `zero_and_negative_status` as it's already mutably borrowed by the function
        // itcpu. Consider a better way to do this.
        let word = cpu.Y;
        cpu.zero_and_negative_status(word);

        2
   }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_dey() {
        let mut cpu = mock_cpu(&[0x88]);
        cpu.Y = 0xff;

        assert_cpu_register!(cpu, Registers::Y, 0xff);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::Y, 0xfe);
        //TODO Make assertions on status registers
    }
}
