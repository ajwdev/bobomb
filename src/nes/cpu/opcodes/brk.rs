use crate::nes::address::Address;
use crate::nes::cpu::status::Flags;
use crate::nes::cpu::{Cpu, FromImplied};

pub struct Brk {}

impl FromImplied for Brk {
    fn from_implied(cpu: &mut Cpu) -> u32 {
        // BRK pushes PC+2 onto stack (not PC+1 like other instructions)
        let pc_plus_2 = Address(cpu.PC + 1); // PC was already incremented by 1
        cpu.push_address(pc_plus_2);

        // Push status register with break flag set
        let mut sr = cpu.SR.to_u8();
        sr |= 1 << 4; // Set break flag (bit 4)
        cpu.push_word(sr);

        // Set interrupt disable flag
        cpu.SR.set(Flags::Interrupt);

        // Load IRQ vector address
        let interconnect = cpu.interconnect.lock();
        cpu.PC = interconnect.find_irq_vector_address().into();

        7
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::status::Flags;
    use crate::nes::cpu::test::*;

    #[test]
    fn test_brk() {
        let mut cpu = mock_cpu(&[0x00]);
        let initial_pc = cpu.PC;
        let initial_sp = cpu.SP;

        cpu.step(None).unwrap();

        assert_status_set!(cpu, Flags::Interrupt);

        // Check that stack pointer was decremented by 3 (PC high, PC low, status)
        assert_eq!(cpu.SP, initial_sp.wrapping_sub(3));

        // Check that PC was loaded from IRQ vector (in test setup this would be specific to test ROM)
        // The exact PC value depends on test ROM setup, just verify it changed
        assert_ne!(cpu.PC, initial_pc);
    }
}
