use nes::cpu::{Cpu,FromAddress,AddressMode};
use nes::cpu::status::Flags;

pub struct Ror { }

impl FromAddress for Ror {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let src = cpu.translate_address(mode);
        let mut word = cpu.mem.read_word(src.to_u16());

        let old_carry_set = cpu.SR.is_set(Flags::Carry);
        let new_carry_set = (0x1 & word) > 0;

        word = word >> 1;
        if old_carry_set {
            word |= 0b10000000;
        } else {
            word &= !0b10000000;
        }

        cpu.mem.write_word(src.to_u16(), word);

        cpu.zero_and_negative_status(word);

        if new_carry_set {
            cpu.SR.set(Flags::Carry)
        } else {
            cpu.SR.reset(Flags::Carry)
        }


        match mode {
            AddressMode::ZeroPage => 5,
            _ => { panic!("unimplemented address mode {:?} for ROR", mode); }
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
        let mut cpu = mock_cpu(&[0x66,0xFF]);
        cpu.mem.write_word(0x00FF, 0b10000000);
        cpu.SR.set(Flags::Carry);

        // TODO test zero and negative status

        cpu.execute_instruction();
        let result = cpu.mem.read_word(0x00FF);
        assert_equalx!(0b11000000, result);
        assert_status_reset!(cpu, Flags::Carry);
    }
}



