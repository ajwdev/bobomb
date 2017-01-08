use nes::cpu::{Cpu,FromImmediate,FromAddress,AddressMode};

pub struct Eor { }

impl Eor {
    fn xor(cpu: &mut Cpu, word: u8) {
        let result = cpu.AC ^ word;
        cpu.AC = result;

        cpu.zero_and_negative_status(result);
    }
}

impl FromImmediate for Eor {
    fn from_immediate(cpu: &mut Cpu) -> usize {
        let word = cpu.read_word_and_increment();
        Self::xor(cpu, word);

        2
    }
}

impl FromAddress for Eor {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let src = cpu.translate_address(mode);
        let word = cpu.mem.read_word(src.to_u16());

        Self::xor(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
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


        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0xFF);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Negative);

        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0x00);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
    }
}


