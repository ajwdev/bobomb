use nes::cpu::{Cpu,FromImplied};
use nes::address::Address;

pub struct Rti { }

impl FromImplied for Rti {
    fn from_implied(cpu: &mut Cpu) -> usize {
        let word = cpu.pop_word();
        cpu.SR.load_from_u8(word);
        cpu.PC = cpu.pop_address().into();

        6
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_rti() {
        let mut cpu = mock_cpu(&[0x40]);
        cpu.push_word(0xBE);
        cpu.push_word(0xEF);
        cpu.push_word(0xAA);

        cpu.step(None);
        assert_cpu_register!(cpu, Register::SP, 0xAA);
        assert_cpu_register!(cpu, Register::PC, 0xBEEF);
        // assert!(cpu.PC == 0xbeef, "expected 0xbeef, got {:#x}", cpu.PC);
    }
}

