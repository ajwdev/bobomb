use nes::cpu::{Cpu,Registers,ZeroPage};
use super::store::Store;

pub struct Sty { }

impl ZeroPage for Sty {
    fn zero_page(cpu: &mut Cpu) -> usize {
        Store::zero_page(cpu, Registers::Y);
        3
    }
}

mod test {
    use nes::cpu::Cpu;
    use nes::cpu::test::*;
    use nes::address::{AddressSpace,Bank};

    #[test]
    fn test_sty_zero() {
        let mut cpu = mock_cpu(&[0x84, 0x10]);
        cpu.Y = 0xff;

        let mut result = cpu.mem.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }
}
