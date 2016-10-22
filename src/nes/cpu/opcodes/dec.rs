use nes::cpu::{Cpu,ZeroPage};

pub struct Dec { }

impl ZeroPage for Dec {
    fn zero_page(cpu: &mut Cpu) -> usize {
        // TODO Try and DRY this up. I think we can come up
        // with a good solution for all address modes
        let addr = Cpu::zero_page_address(cpu.read_word_and_increment());
        let mut word = cpu.mem.read_word(addr);

        word = word.wrapping_sub(1);
        cpu.mem.write_word(addr, word);
        cpu.zero_and_negative_status(word);
        5
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_dec_zero() {
        let mut cpu = mock_cpu(&[0xc6, 0x10]);
        cpu.mem.write_word(0x10, 0xff);

        let mut result = cpu.mem.read_word(0x10);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x10);
        assert!(result == 0xfe, "expected 0xfe, got {:#x}", result);
        //TODO Make assertions on status registers
    }
}
