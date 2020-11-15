use crate::nes::cpu::Cpu;

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
    fn implied(cpu: &mut Cpu) -> usize;
}

pub trait FromImplied {
    fn from_implied(cpu: &mut Cpu) -> u32;
}

#[deprecated(note="please use `FromRelative` trait instead")]
pub trait Relative {
    fn relative(cpu: &mut Cpu) -> usize;
}

pub trait FromRelative {
    fn from_relative(cpu: &mut Cpu) -> u32;
}

#[deprecated(note="please use `FromImmediate` trait instead")]
pub trait Immediate {
    fn immediate(cpu: &mut Cpu) -> usize;
}

pub trait FromImmediate {
    fn from_immediate(cpu: &mut Cpu) -> u32;
}

pub trait FromAccumulator {
    fn from_accumulator(cpu: &mut Cpu) -> u32;
}

pub trait FromAddress {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32;
}

#[deprecated(note="please use `FromAddress` trait instead")]
pub trait Absolute {
    fn absolute(cpu: &mut Cpu) -> usize;
}

#[deprecated(note="please use `FromAddress` trait instead")]
pub trait ZeroPage {
    fn zero_page(cpu: &mut Cpu) -> usize;
}

#[deprecated(note="please use `FromAddress` trait instead")]
pub trait IndirectY {
    fn indirect_y(cpu: &mut Cpu) -> usize;
}

#[deprecated(note="please use `FromAccumulator` trait instead")]
pub trait Accumulator {
    fn accumulator(cpu: &mut Cpu) -> usize;
}
