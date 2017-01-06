use nes::cpu::{Cpu,Registers,Accumulator};
use nes::cpu::status::Flags;

pub struct Lsr { }

impl Accumulator for Lsr {
    fn accumulator(cpu: &mut Cpu) -> usize {
        if cpu.AC & 0x01 == 0 {
            cpu.SR.reset(Flags::Carry);
        } else {
            cpu.SR.set(Flags::Carry);
        }

        cpu.AC = cpu.AC.wrapping_shr(1);

        cpu.SR.reset(Flags::Negative); // We can never go negative

        if cpu.AC == 0 {
            cpu.SR.set(Flags::Zero);
        } else {
            cpu.SR.reset(Flags::Zero);
        }

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;
    use nes::cpu::status::Flags;

    #[test]
    fn test_lsr() {
        // TODO Would it be useful to have `rewind` function
        // that would let us replay an instruction (ignoring side effects)
        let mut cpu = mock_cpu(&[0x4a,0x4a,0x4a,0x4a]);
        cpu.AC = 255;
        cpu.SR.reset(Flags::Carry);
        cpu.SR.set(Flags::Negative);

        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 127);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_reset!(cpu, Flags::Zero);

        cpu.AC = 2;
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 1);
        assert_status_reset!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_reset!(cpu, Flags::Zero);

        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0);
        assert_status_set!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Zero);

        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0);
        assert_status_reset!(cpu, Flags::Carry);
        assert_status_reset!(cpu, Flags::Negative);
        assert_status_set!(cpu, Flags::Zero);
    }
}