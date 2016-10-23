use nes::cpu::{Cpu,Registers,Absolute,IndirectY,ZeroPage};
use super::store::Store;

pub struct Sta { }

impl Absolute for Sta {
    fn absolute(cpu: &mut Cpu) -> usize {
        Store::absolute(cpu, Registers::AC);
        4
    }
}

impl IndirectY for Sta {
    fn indirect_y(cpu: &mut Cpu) -> usize {
        Store::indirect_y(cpu, Registers::AC);
        6
    }
}

impl ZeroPage for Sta {
    fn zero_page(cpu: &mut Cpu) -> usize {
        Store::zero_page(cpu, Registers::AC);
        3
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_sta_zeropage() {
        let mut cpu = mock_cpu(&[0x85, 0x10]);
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_sta_abs() {
        let mut cpu = mock_cpu(&[0x8d, 0x10, 0x00]);
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_sta_indirect_y() {
        let mut cpu = mock_cpu(&[0x91, 0x10]);
        cpu.mem.write_word(0x0010, 0xaa);
        cpu.Y = 0x10;
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x00ba);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x00ba);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }
}
