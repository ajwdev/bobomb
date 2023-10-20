use crate::nes::cpu::{Cpu,Implied};
use crate::nes::cpu::status::Flags;

pub struct Clv { }

impl Implied for Clv {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.SR.reset(Flags::Overflow);
        2
    }
}
