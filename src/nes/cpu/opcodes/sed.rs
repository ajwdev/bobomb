use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{Cpu, FromImplied};

pub struct Sed {}

impl FromImplied for Sed {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        cpu.SR.set(Flags::Decimal);
        2
    }
}
