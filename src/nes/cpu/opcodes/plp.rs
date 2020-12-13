use crate::nes::cpu::{Cpu,FromAddress,AddressMode};

pub struct Plp { }

impl Plp {
    fn execute(cpu: &mut Cpu) {
        let result = cpu.pop_word();
        cpu.SR.load_from_u8(result);
    }
}

impl FromAddress for Plp {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        Self::execute(cpu);

        match mode {
            AddressMode::Implied => 4,
            _ => { panic!("unimplemented address mode {:?} for PLP", mode); }
        }
    }
}
