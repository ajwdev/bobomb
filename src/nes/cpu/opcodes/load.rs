use nes::cpu::{Cpu,Destination};

pub struct Load { }

impl Load {
    pub fn immediate(cpu: &mut Cpu, dst: Destination) -> usize {
        let word = cpu.read_word_and_increment();
        Self::save_destination(cpu, dst, word);
        cpu.zero_and_negative_status(word);

        // Immediate addressing instructions are always two cycles
        2
    }

    pub fn absolute(cpu: &mut Cpu, dst: Destination) {
        let dest = cpu.read_dword_and_increment();
        let word = cpu.mem.read_word(dest);
        Self::save_destination(cpu, dst, word);
        cpu.zero_and_negative_status(word);
    }

    fn save_destination(cpu: &mut Cpu, dest: Destination, value: u8) {
        match dest {
            Destination::RegX => { cpu.X = value },
            Destination::RegY => { cpu.Y = value },
            Destination::RegAC => { cpu.AC = value },
            Destination::Mem(idx) => { cpu.mem.write_word(idx, value) },
        }
    }
}
