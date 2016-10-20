use nes::cpu::{Cpu,Immediate};

pub struct And { }

impl Immediate for And {
    fn immediate(cpu: &mut Cpu) -> usize {
        // TODO Is it possible to refactor this out into common code?
        let word = cpu.read_word_and_increment();
        cpu.AC &= word;
        cpu.zero_and_negative_status(word);

        2
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
}
