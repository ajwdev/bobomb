use nes::cpu::{Cpu,FromImmediate,FromAddress,AddressMode};

pub struct And { }

impl And {
    #[inline]
    fn and(cpu: &mut Cpu, word: u8) {
        let result = cpu.AC & word;

        cpu.AC = result;
        cpu.zero_and_negative_status(result);
    }
}

impl FromImmediate for And {
    fn from_immediate(cpu: &mut Cpu) -> usize {
        let word = cpu.read_word_and_increment();
        Self::and(cpu, word);

        2
    }
}

impl FromAddress for And {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let src = cpu.translate_address(mode);
        let word = cpu.interconnect.read_word(src.to_u16());

        Self::and(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::ZeroPageX => 4,
            _ => { panic!("unimplemented address mode {:?} for AND", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_and_immediate() {
        let mut cpu = mock_cpu(&[0x29, 0x84]);
        cpu.AC = 0xf0;

        assert_cpu_register!(cpu, Registers::AC, 0xf0);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0x80);
        //TODO Make assertions on status registers
    }

    #[test]
    fn test_and_zero_page() {
        let mut cpu = mock_cpu(&[0x25, 0xff]);
        cpu.interconnect.write_word(0xff, 0x84);
        cpu.AC = 0xf0;

        assert_cpu_register!(cpu, Registers::AC, 0xf0);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0x80);
        //TODO Make assertions on status registers
    }
}
