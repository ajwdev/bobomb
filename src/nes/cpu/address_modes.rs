use nes::cpu::Cpu;

// http://www.emulator101.com.s3-website-us-east-1.amazonaws.com/6502-addressing-modes.html
#[derive(Debug,Copy,Clone)]
pub enum AddressMode {
    Implied,
    Immediate,
    Relative,
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[deprecated(note="please use `FromImplied` trait instead")]
pub trait Implied {
    fn implied(&mut Cpu) -> usize;
}

pub trait FromImplied {
    fn from_implied(&mut Cpu) -> usize;
}

#[deprecated(note="please use `FromRelative` trait instead")]
pub trait Relative {
    fn relative(&mut Cpu) -> usize;
}

pub trait FromRelative {
    fn from_relative(&mut Cpu) -> usize;
}

#[deprecated(note="please use `FromImmediate` trait instead")]
pub trait Immediate {
    fn immediate(&mut Cpu) -> usize;
}

pub trait FromImmediate {
    fn from_immediate(&mut Cpu) -> usize;
}

pub trait FromAddress {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize;
}

#[deprecated(note="please use `FromAddress` trait instead")]
pub trait Absolute {
    fn absolute(&mut Cpu) -> usize;
}

#[deprecated(note="please use `FromAddress` trait instead")]
pub trait ZeroPage {
    fn zero_page(&mut Cpu) -> usize;
}

#[deprecated(note="please use `FromAddress` trait instead")]
pub trait IndirectY {
    fn indirect_y(&mut Cpu) -> usize;
}

#[deprecated(note="please use `FromAddress` trait instead")]
pub trait Accumulator {
    fn accumulator(&mut Cpu) -> usize;
}
