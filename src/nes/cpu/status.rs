use std::clone::Clone;

#[derive(Debug,Copy,Clone)]
pub enum Flags {
    Negative,
    Overflow,
    Break,
    Decimal,
    Interrupt,
    Zero,
    Carry,
}

pub struct StatusRegister {
    // bit 7
    negative: bool, // N

    // bit 6
    overflow: bool, // V

    // bit 5
    // reserved

    // bit 4
    // break: bool, // B This may not actually get used

    // bit 3
    // NOTE This apparently affects ADC/SBC instructions
    decimal: bool, // D

    // bit 2
    interrupt: bool, // I

    // bit 1
    zero: bool, // Z

    // bit 0
    carry: bool, // C
}

impl StatusRegister {
    pub fn new() -> Self {
        Self {
            negative: false,
            overflow: false,
            decimal: false,
            interrupt: true,
            zero: false,
            carry: false,
        }
    }

    fn find_flag_mut(&mut self, flag: Flags) -> &mut bool {
        // TODO Research to see if this is DRY-able with immutable version
        match flag {
            Flags::Negative => { &mut self.negative }
            Flags::Overflow => { &mut self.overflow }
            Flags::Decimal => { &mut self.decimal } // TODO Log if this is changed
            Flags::Interrupt => { &mut self.interrupt }
            Flags::Zero => { &mut self.zero }
            Flags::Carry => { &mut self.carry }
            Flags::Break => { panic!("break status flag is not implemented"); }
        }
    }

    fn find_flag(&self, flag: Flags) -> bool {
        match flag {
            Flags::Negative => { self.negative }
            Flags::Overflow => { self.overflow }
            Flags::Decimal => { self.decimal }
            Flags::Interrupt => { self.interrupt }
            Flags::Zero => { self.zero }
            Flags::Carry => { self.carry }
            Flags::Break => { panic!("break status flag is not implemented"); }
        }
    }

    #[inline]
    pub fn is_set(&self, flag: Flags) -> bool {
        self.find_flag(flag)
    }

    pub fn is_clear(&self, flag: Flags) -> bool {
        !self.find_flag(flag)
    }

    #[inline]
    pub fn set(&mut self, flag: Flags) {
        let reg = self.find_flag_mut(flag);
        *reg = true;
    }

    #[inline]
    pub fn reset(&mut self, flag: Flags) {
        let reg = self.find_flag_mut(flag);
        *reg = false;
    }

    #[inline]
    pub fn load_from_u8(&mut self, word: u8) {
        self.carry     = ( word & 1) == 1;
        self.zero      = ( word & (1 << 1)) == 1;
        self.interrupt = ( word & (1 << 2)) == 1;
        self.decimal   = ( word & (1 << 3)) == 1;
        self.overflow  = ( word & (1 << 6)) == 1;
        self.negative  = ( word & (1 << 7)) == 1;
    }

    #[inline]
    pub fn to_u8(&self) -> u8 {
        let result
            = 1 << 5  // Unsused/Reserved bit but seems to be set
            | (self.carry as u8)
            | (self.zero as u8) << 1
            | (self.interrupt as u8) << 2
            | (self.decimal as u8) << 3
            | (self.overflow as u8) << 6
            | (self.negative as u8) << 7;

        result
    }


    #[inline]
    #[deprecated(note="please use `reset(T)` instead")]
    pub fn reset_zero(&mut self) {
        self.zero = false;
    }

    #[inline]
    #[deprecated(note="please use `set(T)` instead")]
    pub fn set_zero(&mut self) {
        self.zero = true;
    }

    #[inline]
    #[deprecated(note="please use `reset(T)` instead")]
    pub fn reset_negative(&mut self) {
        self.negative = false;
    }

    #[inline]
    #[deprecated(note="please use `set(T)` instead")]
    pub fn set_negative(&mut self) {
        self.negative = true;
    }

    #[inline]
    #[deprecated(note="please use `reset(T)` instead")]
    pub fn reset_interrupt(&mut self) {
        self.interrupt = false;
    }

    #[inline]
    #[deprecated(note="please use `set(T)` instead")]
    pub fn set_interrupt(&mut self) {
        self.interrupt = true;
    }

    #[inline]
    #[deprecated(note="please use `reset(T)` instead")]
    pub fn reset_decimal(&mut self) {
        self.decimal = false;
    }

    #[inline]
    #[deprecated(note="please use `set(T)` instead")]
    pub fn set_decimal(&mut self) {
        self.decimal = true;
    }
}

impl From<u8> for StatusRegister {
    fn from(t: u8) -> StatusRegister {
        let mut sr = StatusRegister::new();
        sr.load_from_u8(t);
        sr
    }
}

impl From<StatusRegister> for u8 {
    fn from(t: StatusRegister) -> u8 {
        t.to_u8()
    }
}
