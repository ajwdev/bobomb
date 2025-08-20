use crate::nes::cpu::{Cpu, FromImplied};

pub struct Php {}

impl FromImplied for Php {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        let reg = cpu.SR.to_u8();
        cpu.push_word(reg);
        3
    }
}
