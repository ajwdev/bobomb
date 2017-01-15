use nes::cpu::{Cpu,AddressMode,Immediate,FromAddress};
use nes::cpu::status::Flags;

pub struct Cmp { }

// NOTE Per http://www.atariarchives.org/alp/appendix_1.php, the Cmp instruction does the
// subtraction as if both numbers are unsigned so watch out for overflows

impl Cmp {
    fn compare(cpu: &mut Cpu, word: u8) {
        let result = cpu.AC.wrapping_sub(word);

        cpu.zero_and_negative_status(result);
        if word <= cpu.AC {
            cpu.SR.set(Flags::Carry);
        } else {
            cpu.SR.reset(Flags::Carry);
        }
    }
}

impl Immediate for Cmp {
    fn immediate(cpu: &mut Cpu) -> usize {
        let word = cpu.read_word_and_increment();
        Self::compare(cpu, word);

        2
    }
}

impl FromAddress for Cmp {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let (src, extra_cycles) = cpu.translate_address(mode);
        let word = cpu.interconnect.read_word(src.to_u16());

        Self::compare(cpu, word);

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::ZeroPageX => 4,
            AddressMode::AbsoluteX => { 4 + (extra_cycles as usize) },
            AddressMode::AbsoluteY => { 4 + (extra_cycles as usize) },
            AddressMode::IndirectY => { 5 + (extra_cycles as usize) },
            _ => { panic!("unimplemented address mode {:?} for AND", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::status::Flags;
    use nes::cpu::Registers;

    #[test]
    fn test_cmp_equal() {
        let mut cpu = mock_cpu(&[0xc9,0xAA]);
        cpu.AC = 0xAA;
        cpu.step(None);

        assert_cpu_register!(cpu, Registers::AC, 0xAA);
        assert_status_set!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
    }

    #[test]
    fn test_cmp_less_than() {
        let mut cpu = mock_cpu(&[0xc9,0xA0]);
        cpu.AC = 0xAA;
        cpu.step(None);

        assert_cpu_register!(cpu, Registers::AC, 0xAA);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
    }

    #[test]
    fn test_cmp_less_than_twos_comp() {
        let mut cpu = mock_cpu(&[0xc9,0x10]);
        cpu.AC = 0xAA;
        cpu.step(None);

        assert_cpu_register!(cpu, Registers::AC, 0xAA);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_set!(cpu, Flags::Carry);
        // Negative here because 0xAA - 0x10 has the 7th bit set
        assert_status_set!(cpu, Flags::Negative);
    }

    #[test]
    fn test_cmp_greater_than() {
        let mut cpu = mock_cpu(&[0xc9,0xBB]);
        cpu.AC = 0xAA;
        cpu.step(None);

        assert_cpu_register!(cpu, Registers::AC, 0xAA);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Carry);
        assert_status_set!(cpu, Flags::Negative);
    }

    #[test]
    fn test_cmp_greater_than_twos_comp() {
        let mut cpu = mock_cpu(&[0xc9,0xBB]);
        cpu.AC = 0x10;
        cpu.step(None);

        assert_cpu_register!(cpu, Registers::AC, 0x10);
        assert_status_reset!(cpu, Flags::Zero);
        assert_status_reset!(cpu, Flags::Carry);
        // Not negative here because 0x10 - 0xBB does not have the 7th bit set
        assert_status_reset!(cpu, Flags::Negative);
    }
}
