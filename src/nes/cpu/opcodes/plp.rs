use crate::nes::cpu::{Cpu, FromImplied};

pub struct Plp {}

impl FromImplied for Plp {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        let result = cpu.pop_word();
        cpu.SR.load_from_u8(result);
        4
    }
}
