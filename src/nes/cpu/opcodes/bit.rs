use crate::nes::cpu::{Cpu,AddressMode,FromAddress};
use crate::nes::cpu::status::Flags;

pub struct Bit { }

impl FromAddress for Bit {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());

        if (cpu.AC & word) == 0 {
            cpu.SR.set(Flags::Zero);
        } else {
            cpu.SR.reset(Flags::Zero);
        }

        if ((word >> 7) & 1) == 1 {
            cpu.SR.set(Flags::Negative);
        } else {
            cpu.SR.reset(Flags::Negative);
        }

        if ((word >> 6) & 1) == 1 {
            cpu.SR.set(Flags::Overflow);
        } else {
            cpu.SR.reset(Flags::Overflow);
        }


        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::Absolute => 4,
            _ => { panic!("unimplemented address mode {:?} for BIT", mode); }
        }
    }
}
