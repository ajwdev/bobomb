use crate::nes::cpu::{Cpu,FromImmediate,FromAddress,AddressMode};

pub struct Ora { }

impl Ora {
    #[inline]
    fn and(cpu: &mut Cpu, word: u8) {
        let result = cpu.AC | word;

        cpu.AC = result;
        cpu.zero_and_negative_status(result);
    }
}

impl FromImmediate for Ora {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        Self::and(cpu, word);

        2
    }
}

impl FromAddress for Ora {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());

        Self::and(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::AbsoluteX => { 4 + (extra_cycles as u32) },
            AddressMode::AbsoluteY => { 4 + (extra_cycles as u32) },
            AddressMode::IndirectY => { 5 + (extra_cycles as u32) },
            _ => { panic!("unimplemented address mode {:?} for AND", mode); }
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::nes::cpu::test::*;
//     use crate::nes::cpu::Registers;

//     #[test]
//     fn test_and_immediate() {
//         let mut cpu = mock_cpu(&[0x29, 0x84]);
//         cpu.AC = 0xf0;

//         assert_cpu_register!(cpu, Registers::AC, 0xf0);
//         cpu.step(None);
//         assert_cpu_register!(cpu, Registers::AC, 0x80);
//         //TODO Make assertions on status registers
//     }

//     #[test]
//     fn test_and_zero_page() {
//         let mut cpu = mock_cpu(&[0x25, 0xff]);
//         cpu.write_at(0xff, 0x84);
//         cpu.AC = 0xf0;

//         assert_cpu_register!(cpu, Registers::AC, 0xf0);
//         cpu.step(None);
//         assert_cpu_register!(cpu, Registers::AC, 0x80);
//         //TODO Make assertions on status registers
//     }
// }

