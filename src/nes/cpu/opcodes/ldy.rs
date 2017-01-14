use nes::cpu::{Cpu,Registers,AddressMode,FromImmediate,FromAddress};
use super::load::Load;

pub struct Ldy { }

impl FromImmediate for Ldy {
    fn from_immediate(cpu: &mut Cpu) -> usize {
        Load::immediate(cpu, Registers::Y)
    }
}

impl FromAddress for Ldy {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let src = cpu.translate_address(mode);
        let word = cpu.interconnect.read_word(src.to_u16());
        cpu.Y = word;

        cpu.zero_and_negative_status(word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::ZeroPageX => 4,
            _ => { panic!("unimplemented address mode {:?} for LDY", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_ldy_imm() {
        let mut cpu = mock_cpu(&[0xa0, 0xff]);

        assert_cpu_register!(cpu, Registers::Y, 0);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::Y, 0xff);
    }
}
