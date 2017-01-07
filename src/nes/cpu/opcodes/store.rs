use nes::cpu::{Cpu,Registers};

pub struct Store { }

impl Store {
    pub fn absolute(cpu: &mut Cpu, src: Registers) {
        let dword = cpu.read_dword_and_increment();
        Self::save_destination(cpu, src, dword);
    }

    pub fn zero_page(cpu: &mut Cpu, src: Registers) {
        let word = cpu.read_word_and_increment();
        Self::save_destination(cpu, src, Cpu::zero_page_address(word));
    }

    pub fn indirect_y(cpu: &mut Cpu, src: Registers) {
        let word = cpu.read_word_and_increment();
        let indirect_addr = cpu.mem.read_word(Cpu::zero_page_address(word));
        let offset = cpu.Y;
        Self::save_destination(cpu, src, (indirect_addr + offset) as u16);
    }

    pub fn save_destination(cpu: &mut Cpu, src: Registers, dest: u16) {
        match src {
            Registers::X => { cpu.mem.write_word(dest, cpu.X); },
            Registers::Y => { cpu.mem.write_word(dest, cpu.Y); },
            Registers::AC => { cpu.mem.write_word(dest, cpu.AC); },
            Registers::SP => { cpu.mem.write_word(dest, cpu.SP); },
            Registers::PC => { panic!("PC is not supported here!") },
        }
    }
}

