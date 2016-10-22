use nes::cpu::Cpu;

pub struct Branch { }

impl Branch {
    pub fn branch_on_true<F>(cpu: &mut Cpu, cond: F) where F: Fn(&Cpu) -> bool {
        let word = cpu.read_word_and_increment();
        if cond(&cpu) { cpu.move_pc_relative(word); }
    }
}
