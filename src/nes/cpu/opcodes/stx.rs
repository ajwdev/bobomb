use super::store::Store;
use crate::nes::cpu::{AddressMode, Cpu, FromAddress, Registers};

pub struct Stx {}

impl FromAddress for Stx {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (dest, _) = cpu.translate_address(mode);
        Store::save_destination(cpu, Registers::X, dest.to_u16());

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::ZeroPageY => 4,
            AddressMode::Absolute => 4,
            // TODO(ajw): Make a macro for this
            _ => {
                panic!("unimplemented address mode {:?} for STX", mode);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;

    #[test]
    fn test_stx_zeropage() {
        let mut cpu = mock_cpu(&[0x86, 0x10]);
        cpu.X = 0xff;

        let mut result = cpu.read_at(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.step(None);
        result = cpu.read_at(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_stx_abs() {
        let mut cpu = mock_cpu(&[0x8e, 0x10, 0x00]);
        cpu.X = 0xff;

        let mut result = cpu.read_at(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.step(None);
        result = cpu.read_at(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }
}
