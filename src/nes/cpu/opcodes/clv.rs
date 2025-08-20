use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{Cpu, FromImplied};

pub struct Clv {}

impl FromImplied for Clv {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        cpu.SR.reset(Flags::Overflow);
        2
    }
}
