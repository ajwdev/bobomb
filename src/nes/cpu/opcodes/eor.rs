use nes::cpu::{Cpu,FromImmediate,FromAddress,AddressMode};

pub struct Eor { }

impl Eor {
    #[inline]
    fn xor(cpu: &mut Cpu, word: u8) {
        let result = cpu.AC ^ word;
        cpu.AC = result;

        cpu.zero_and_negative_status(result);
    }
}

impl FromImmediate for Eor {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        Self::xor(cpu, word);

        2
    }
}

impl FromAddress for Eor {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.interconnect.read_word(src.to_u16());

        Self::xor(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::AbsoluteX => { 4 + (extra_cycles as u32) },
            AddressMode::AbsoluteY => { 4 + (extra_cycles as u32) },
            AddressMode::IndirectY => { 5 + (extra_cycles as u32) },
            _ => { panic!("unimplemented address mode {:?} for EOR", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;
    use nes::cpu::status::Flags;

    #[test]
    fn test_eor() {
        let mut cpu = mock_cpu(&[0x49,0xF0,0x49,0xFF]);
        cpu.AC = 0x0F;
        cpu.SR.reset(Flags::Zero);


        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0xFF);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0x00);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
    }
}


