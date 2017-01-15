use nes::cpu::{Cpu,FromAddress,AddressMode};

pub struct Dec { }

impl Dec {
    #[inline]
    fn decrement(cpu: &mut Cpu, word: u8) -> u8 {
        let result = word.wrapping_sub(1);
        cpu.zero_and_negative_status(result);

        result
    }
}

impl FromAddress for Dec {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let (src, _) = cpu.translate_address(mode);
        let word = cpu.interconnect.read_word(src.to_u16());
        let result = Self::decrement(cpu, word);

        cpu.interconnect.write_word(src.into(), result);

        match mode {
            AddressMode::ZeroPage => 5,
            AddressMode::ZeroPageX => 6,
            AddressMode::Absolute => 6,
            _ => { panic!("unimplemented address mode {:?} for DEC", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_dec_zero() {
        let mut cpu = mock_cpu(&[0xc6, 0x10]);
        cpu.interconnect.write_word(0x10, 0xff);

        let mut result = cpu.interconnect.read_word(0x10);
        assert_equalx!(result, 0xff);
        cpu.step(None);
        result = cpu.interconnect.read_word(0x10);
        assert_equalx!(result, 0xfe);
        //TODO Make assertions on status registers
    }
}
