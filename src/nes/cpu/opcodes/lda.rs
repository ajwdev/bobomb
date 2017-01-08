use nes::cpu::{Cpu,Registers,AddressMode,FromImmediate,FromAddress};

pub struct Lda { }

impl Lda {
    #[inline]
    fn load_accumulator(cpu: &mut Cpu, word: u8) {
        cpu.AC = word;
        cpu.zero_and_negative_status(word);
    }
}

impl FromImmediate for Lda {
    fn from_immediate(cpu: &mut Cpu) -> usize {
        let word = cpu.read_word_and_increment();
        Lda::load_accumulator(cpu, word);

        2
    }
}

impl FromAddress for Lda {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let src = cpu.translate_address(mode);
        let word = cpu.interconnect.read_word(src.to_u16());

        Lda::load_accumulator(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            AddressMode::AbsoluteY => 4, // This could have extra cycle added
            _ => { panic!("unimplemented address mode {:?} for LDX", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_lda_abs() {
        let mut cpu = mock_cpu(&[0xad, 0x03, 0x80, 0xff]);

        assert_cpu_register!(cpu, Registers::AC, 0);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0xff);
    }

    #[test]
    fn test_lda_imm() {
        let mut cpu = mock_cpu(&[0xa9, 0xff]);

        assert_cpu_register!(cpu, Registers::AC, 0);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0xff);
    }

    #[test]
    fn test_lda_zeropage() {
        let mut cpu = mock_cpu(&[0xa5, 0xff]);
        cpu.interconnect.write_word(0x00ff, 0xbe);

        assert_cpu_register!(cpu, Registers::AC, 0);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0xbe);
    }
}
