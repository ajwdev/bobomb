use crate::nes::cpu::Cpu;

pub struct Branch { }

impl Branch {
    pub fn branch_on_true<F: Fn(&Cpu) -> bool>(cpu: &mut Cpu, cond: F) -> u32 {
        // All branch operations have the same cycle counts
        let mut cycles: u32 = 2;

        let orig_pc = cpu.PC;
        let word = cpu.read_word_and_increment();

        if cond(&cpu) {
            cycles += 1;

            cpu.move_pc_relative(word);
            if Cpu::page_crossed(orig_pc, cpu.PC) {
                cycles += 1;
            }
        }

        cycles
    }
}
