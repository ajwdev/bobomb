use nes::cpu::{Cpu,ZeroPage};

pub struct Dec { }

impl ZeroPage for Dec {
    fn zero_page(cpu: &mut Cpu) -> usize {
        // TODO Try and DRY this up. I think we can come up
        // with a good solution for all address modes
        let addr = Cpu::zero_page_address(cpu.read_word_and_increment());
        let mut word = cpu.interconnect.read_word(addr);

        word = word.wrapping_sub(1);
        cpu.interconnect.write_word(addr, word);
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
        cpu.interconnect.write_word(0x10, 0xff);

        let mut result = cpu.interconnect.read_word(0x10);
        assert_equalx!(result, 0xff);
        cpu.step(None);
        result = cpu.interconnect.read_word(0x10);
        assert_equalx!(result, 0xfe);
        //TODO Make assertions on status registers
    }
}
