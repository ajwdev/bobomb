use crate::nes::cpu::{Cpu, FromImplied};

pub struct Dex {}

impl FromImplied for Dex {
    fn from_implied(cpu: &mut Cpu) -> u32 {
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
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;

    #[test]
    fn test_dex() {
        let mut cpu = mock_cpu(&[0xca]);
        cpu.X = 10;

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::X, 9);
    }
}
