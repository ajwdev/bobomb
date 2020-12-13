use crate::nes::cpu::{Cpu,FromAddress,AddressMode};

pub struct Php { }

impl Php {
    fn execute(cpu: &mut Cpu) {
        let reg = cpu.SR.to_u8();
        cpu.push_word(reg);
    }
}

impl FromAddress for Php {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        Self::execute(cpu);

        match mode {
            AddressMode::Implied => 3,
            _ => { panic!("unimplemented address mode {:?} for PHP", mode); }
        }
    }
}
