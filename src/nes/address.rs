#[derive(Debug,Copy,Clone)]
pub struct Address(pub u16);

impl Address {
    pub fn new(hi: u8, lo: u8) -> Address {
        Address((hi as u16) << 8 | lo as u16)
    }

    pub fn new_zeropage(lo: u8) -> Address {
        Address(lo as u16)
    }

    pub fn from_bytes(buf: &[u8]) -> Address {
        // Assert the correct length
        Address((buf[1] as u16) << 8 | buf[0] as u16)
    }


    pub fn high(&self) -> u8 {
        ((self.0 & 0xFF00) >> 8) as u8
    }

    pub fn low(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub fn to_u16(&self) -> u16 {
        self.0
    }
}
