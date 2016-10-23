use nes::cpu::{Cpu,Registers};

pub struct Load { }

impl Load {
    pub fn immediate(cpu: &mut Cpu, dst: Registers) -> usize {
        let word = cpu.read_word_and_increment();
        Self::save_destination(cpu, dst, word);
        cpu.zero_and_negative_status(word);

        // Immediate addressing instructions are always two cycles
        2
    }

    pub fn absolute(cpu: &mut Cpu, dst: Registers) {
        let dest = cpu.read_dword_and_increment();
        let word = cpu.mem.read_word(dest);
        Self::save_destination(cpu, dst, word);
        cpu.zero_and_negative_status(word);
    }

    pub fn zero_page(cpu: &mut Cpu, dst: Registers) {
        let src = cpu.read_word_and_increment();
        let word = cpu.mem.read_word(Cpu::zero_page_address(src));
        Self::save_destination(cpu, dst, word);
        cpu.zero_and_negative_status(word);
    }

    fn save_destination(cpu: &mut Cpu, dest: Registers, value: u8) {
        match dest {
            Registers::X => { cpu.X = value },
            Registers::Y => { cpu.Y = value },
            Registers::AC => { cpu.AC = value },
        }
    }
}
