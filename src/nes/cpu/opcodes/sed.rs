use crate::nes::cpu::{Cpu,Implied};
use crate::nes::cpu::status::Flags;


pub struct Sed { }

impl Implied for Sed {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.SR.set(Flags::Decimal);
        2
    }
}
