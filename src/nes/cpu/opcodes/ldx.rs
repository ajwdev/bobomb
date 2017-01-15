use nes::cpu::{Cpu,Registers,AddressMode,FromImmediate,FromAddress};
use super::load::Load;

pub struct Ldx { }

impl FromImmediate for Ldx {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        // TODO Check the status registers (i.e N/Z)
        Load::immediate(cpu, Registers::X) as u32
    }
}

impl FromAddress for Ldx {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());
        cpu.X = word;

        cpu.zero_and_negative_status(word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            AddressMode::AbsoluteY => { 4 + (extra_cycles as u32) },
            _ => { panic!("unimplemented address mode {:?} for LDX", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_ldx_imm() {
        let mut cpu = mock_cpu(&[0xa2, 0xff]);

        assert_cpu_register!(cpu, Registers::X, 0);
        cpu.step(None);
        assert_cpu_register!(cpu, Registers::X, 0xff);
    }

    // TODO Write tests for zero_page by figuring out a better way to make a mock CPU now that we
    // have abstracted out the address resolution stuff. See adc.rs
}
