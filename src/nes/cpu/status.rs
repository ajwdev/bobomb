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
        // Power on state == 00110100 (0x34). See comment at top of file
        StatusRegister {
            negative: false,
            overflow: false,
            // break: true,
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
            Flags::Decimal => { &mut self.decimal }
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

    #[inline]
    pub fn set(&mut self, flag: Flags) {
        let mut reg = self.find_flag_mut(flag);
        *reg = true;
    }

    #[inline]
    pub fn reset(&mut self, flag: Flags) {
        let mut reg = self.find_flag_mut(flag);
        *reg = false;
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
