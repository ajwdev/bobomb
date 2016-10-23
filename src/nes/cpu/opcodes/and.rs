use nes::cpu::{Cpu,Immediate,ZeroPage};

pub struct And { }

impl Immediate for And {
    fn immediate(cpu: &mut Cpu) -> usize {
        // TODO Is it possible to refactor this out into common code?
        let word = cpu.read_word_and_increment();
        cpu.AC &= word;
        // TODO I'm pretty sure this is wrong
        cpu.zero_and_negative_status(word);

        2
    }
}

impl ZeroPage for And {
    fn zero_page(cpu: &mut Cpu) -> usize {
        // TODO Is it possible to refactor this out into common code?
        let word = cpu.read_word_and_increment();
        let dest = cpu.mem.read_word(Cpu::zero_page_address(word));

        cpu.AC &= dest;
        cpu.zero_and_negative_status(word);

        3
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_and_immediate() {
        let mut cpu = mock_cpu(&[0x29, 0x84]);
        cpu.AC = 0xf0;

        assert!(cpu.AC == 0xf0, "expected 0xff, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0x80, "expected 0x80, got {:#x}", cpu.AC);
        //TODO Make assertions on status registers
    }

    #[test]
    fn test_and_zero_page() {
        let mut cpu = mock_cpu(&[0x25, 0xff]);
        cpu.mem.write_word(0xff, 0x84);
        cpu.AC = 0xf0;

        assert!(cpu.AC == 0xf0, "expected 0xff, got {:#x}", cpu.AC);
        cpu.execute_instruction();
        assert!(cpu.AC == 0x80, "expected 0x80, got {:#x}", cpu.AC);
        //TODO Make assertions on status registers
    }
}
