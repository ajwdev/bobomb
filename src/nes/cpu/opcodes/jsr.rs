use crate::nes::cpu::{Cpu,Absolute};
use crate::nes::address::Address;

pub struct Jsr { }

impl Absolute for Jsr {
    fn absolute(cpu: &mut Cpu) -> usize {
        // https://wiki.nesdev.com/w/index.php/RTS_Trick#About_JSR_and_RTS
        let addr = cpu.read_dword_and_increment();
        // PC is now at the next instruction. According to the doc above we are to
        // take this value and subtract one from it, THEN push it on the stack. On pop
        // we then add 1 to the address. I'm not sure why we just cant push the current PC
        // but there is probably a reason.
        let ret = Address(cpu.PC.wrapping_sub(1));

        cpu.push_address(ret);
        cpu.PC = addr;

        6
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;

    #[test]
    fn test_jsr() {
        let mut cpu = mock_cpu(&[0x20, 0xef, 0xbe]);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);

        cpu.step(None);
        assert!(cpu.PC == 0xbeef, "expected 0xbeef, got {:#x}", cpu.PC);

        let mut result = cpu.pop_word();
        assert!(result == 0x02, "expected 0x02, got {:#x}", result);
        result = cpu.pop_word();
        assert!(result == 0x80, "expected 0x80, got {:#x}", result);
    }
}
