use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{AddressMode, Cpu, FromAddress, FromImmediate};

pub struct Adc {}

impl Adc {
    fn add_with_carry(cpu: &mut Cpu, word: u8) {
        // We ignore the Decimal status register because on the NES
        // it is unused. Consider adding support in the future.
        let carry = cpu.SR.is_set(Flags::Carry) as u16;
        let result = (cpu.AC as u16) + (word as u16) + carry;
        let result_byte = result as u8;

        cpu.zero_and_negative_status(result_byte);

        // Carry flag is set if result exceeds 8 bits
        if result > 0xFF {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }

        // Overflow occurs when both operands have same sign but result has different sign
        if (cpu.AC ^ result_byte) & (word ^ result_byte) & 0x80 != 0 {
            cpu.SR.set(Flags::Overflow);
        } else {
            cpu.SR.reset(Flags::Overflow);
        }

        cpu.AC = result_byte;
    }
}

impl FromImmediate for Adc {
    fn from_immediate(cpu: &mut Cpu) -> u32 {
        let word = cpu.read_word_and_increment();
        Adc::add_with_carry(cpu, word);

        2
    }
}

impl FromAddress for Adc {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());

        Adc::add_with_carry(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::ZeroPageX => 4,
            AddressMode::Absolute => 4,
            AddressMode::AbsoluteX => 4 + (extra_cycles as u32),
            AddressMode::AbsoluteY => 4 + (extra_cycles as u32),
            AddressMode::IndirectX => 6,
            AddressMode::IndirectY => 5 + (extra_cycles as u32),
            _ => {
                panic!("unimplemented address mode {:?} for ADC", mode);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;
    use crate::nes::cpu::Registers;

    #[test]
    fn test_adc_immediate_basic() {
        let mut cpu = mock_cpu(&[0x69, 0x05]);
        cpu.AC = 0x03;

        cpu.step(None).unwrap();
        assert_cpu_register!(cpu, Registers::AC, 0x08);
        assert_status_reset!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Negative);
    }

    #[test]
    fn test_adc_immediate_with_carry() {
        let mut cpu = mock_cpu(&[0x69, 0x05]);
        cpu.AC = 0x03;
        cpu.SR.set(Flags::Carry);

        cpu.step(None).unwrap();
        assert_cpu_register!(cpu, Registers::AC, 0x09);
        assert_status_reset!(cpu, Flags::Carry);
    }

    #[test]
    fn test_adc_immediate_overflow() {
        let mut cpu = mock_cpu(&[0x69, 0xFF]);
        cpu.AC = 0x02;

        cpu.step(None).unwrap();
        assert_cpu_register!(cpu, Registers::AC, 0x01);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Zero);
    }

    #[test]
    fn test_adc_zero_page_x_timing() {
        let mut cpu = mock_cpu(&[0x75, 0x10]);
        let cycles = cpu.step(None).unwrap();
        assert_eq!(cycles, 4);
    }
}
