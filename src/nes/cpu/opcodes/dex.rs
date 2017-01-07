use nes::cpu::{Cpu,FromImplied,AddressMode};

pub struct Dex { }

impl FromImplied for Dex {
    fn from_implied(cpu: &mut Cpu) -> usize {
        cpu.X = cpu.X.wrapping_sub(1);
        // TODO The reason we create `word` here is because we can't pass cpu.X to
        // `zero_and_negative_status` as it's already mutably borrowed by the function
        // itcpu. Consider a better way to do this.
        let word = cpu.X;

        cpu.zero_and_negative_status(word);

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;
    use nes::cpu::status::Flags;

    #[test]
    fn test_dex() {
        let mut cpu = mock_cpu(&[0xca]);
        cpu.X = 10;

        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::X, 9);
    }
}
