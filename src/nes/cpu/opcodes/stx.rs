use nes::cpu::{Cpu,Registers,FromAddress,AddressMode};
use super::store::Store;

pub struct Stx { }

impl FromAddress for Stx {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let dest = cpu.translate_address(mode);
        Store::save_destination(cpu, Registers::X, dest.to_u16());

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            // TODO Make a macro for this
            _ => { panic!("unimplemented address mode {:?} for STX", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_stx_zeropage() {
        let mut cpu = mock_cpu(&[0x86, 0x10]);
        cpu.X = 0xff;

        let mut result = cpu.interconnect.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.step(None);
        result = cpu.interconnect.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_stx_abs() {
        let mut cpu = mock_cpu(&[0x8e, 0x10, 0x00]);
        cpu.X = 0xff;

        let mut result = cpu.interconnect.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.step(None);
        result = cpu.interconnect.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }
}
