use nes::cpu::Cpu;

pub trait Implied {
    fn implied(&mut Cpu) -> usize;
}

pub trait Relative {
    fn relative(&mut Cpu) -> usize;
}

pub trait Immediate {
    fn immediate(&mut Cpu) -> usize;
}

pub trait Absolute {
    fn absolute(&mut Cpu) -> usize;
}

pub trait ZeroPage {
    fn zero_page(&mut Cpu) -> usize;
}

pub trait IndirectY {
    fn indirect_y(&mut Cpu) -> usize;
}
