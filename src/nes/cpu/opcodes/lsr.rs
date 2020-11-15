use crate::nes::cpu::{Cpu,Registers,AddressMode,Accumulator,FromAddress};
use crate::nes::cpu::status::Flags;

pub struct Lsr { }

impl Lsr {
    #[inline]
    fn shift_right(cpu: &mut Cpu, mut word: u8) -> u8 {
        if word & 0x01 == 0 {
            cpu.SR.reset(Flags::Carry);
        } else {
            cpu.SR.set(Flags::Carry);
        }

        word = word.wrapping_shr(1);

        cpu.SR.reset(Flags::Negative); // We can never go negative

        if word == 0 {
            cpu.SR.set(Flags::Zero);
        } else {
            cpu.SR.reset(Flags::Zero);
        }

        word
    }
}

impl Accumulator for Lsr {
    fn accumulator(cpu: &mut Cpu) -> usize {
        let src = cpu.AC;
        let result = Lsr::shift_right(cpu, src);
        cpu.AC = result;

        2
    }
}

impl FromAddress for Lsr {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, _) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());
        let result = Self::shift_right(cpu, word);

        cpu.write_at(src.to_u16(), result);

        match mode {
            AddressMode::ZeroPage => 5,
            _ => { panic!("unimplemented address mode {:?} for LSR", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;
    use crate::nes::cpu::status::Flags;

    #[test]
    fn test_lsr() {
        // TODO Would it be useful to have `rewind` function
        // that would let us replay an instruction (ignoring side effects)
        let mut cpu = mock_cpu(&[0x4a,0x4a,0x4a,0x4a]);
        cpu.AC = 255;
        cpu.SR.reset(Flags::Carry);
        cpu.SR.set(Flags::Negative);

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 127);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_reset!(cpu, Flags::Zero);

        cpu.AC = 2;
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 1);
        assert_status_reset!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_reset!(cpu, Flags::Zero);

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Zero);

        cpu.step(None);
        assert_cpu_register!(cpu, Registers::AC, 0);
        assert_status_reset!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Zero);
    }
}
