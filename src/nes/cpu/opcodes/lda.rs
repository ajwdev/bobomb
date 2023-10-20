use crate::nes::cpu::{Cpu,Registers,AddressMode,FromImmediate,FromAddress};

pub struct Lda { }

impl Lda {
    #[inline]
    fn load_accumulator(cpu: &mut Cpu, word: u8) {
        cpu.AC = word;
        cpu.zero_and_negative_status(word);
    }
}

impl FromImmediate for Lda {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        Lda::load_accumulator(cpu, word);

        2
    }
}

impl FromAddress for Lda {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());

        Lda::load_accumulator(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::ZeroPageX => 4,
            AddressMode::Absolute => 4,
            AddressMode::AbsoluteX => { 4 + (extra_cycles as u32) },
            AddressMode::AbsoluteY => { 4 + (extra_cycles as u32) },
            AddressMode::IndirectY => { 5 + (extra_cycles as u32) },
            AddressMode::IndirectX => 6,
            _ => { panic!("unimplemented address mode {:?} for LDX", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;

    #[test]
    fn test_lda_abs() {
        let mut cpu = mock_cpu(&[0xad, 0x03, 0x80, 0xff]);

        assert_cpu_register!(cpu, Registers::AC, 0);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0xff);
    }

    #[test]
    fn test_lda_imm() {
        let mut cpu = mock_cpu(&[0xa9, 0xff]);

        assert_cpu_register!(cpu, Registers::AC, 0);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0xff);
    }

    #[test]
    fn test_lda_zeropage() {
        let mut cpu = mock_cpu(&[0xa5, 0xff]);
        cpu.write_at(0x00ff, 0xbe);

        assert_cpu_register!(cpu, Registers::AC, 0);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0xbe);
    }
}
