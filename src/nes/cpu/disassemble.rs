use nes::cpu::AddressMode;
use nes::cpu::opcodes;
use nes::address::{Address,Addressable};

pub struct Disassembler { }

impl Disassembler {
    pub fn disassemble<T: Addressable>(opc: u8, pipeline: &[u8], address: T) -> Result<(String,u32), String> {
        match OPCODES[opc as usize] {
            Some((label, mode)) => {
                let byte_str = Self::decode_bytes(mode, opc, pipeline);
                let operands = Self::decode_operands(mode, pipeline);

                Ok((
                    Self::format_line(address.nes_address(), label.to_string(), byte_str, operands),
                    Self::instruction_length(mode),
                ))
            }
            None => {
                Err(format!("Unmatched opcode {:#x}", opc))
            }
        }
    }

    #[inline]
    fn format_line(address: u16, label: String, byte_str: String, operands: Option<String>) -> String {
        let mut result = String::new();

        result.push_str(format!("0x{:04X} | ", address).as_str());
        result.push_str(format!("{} | {} ", byte_str, label).as_str());
        if let Some(x) = operands {
            result.push_str(format!("{}", x).as_str());
        }

        result
    }

    #[inline]
    fn instruction_length(mode: AddressMode) -> u32 {
        match mode {
            AddressMode::Implied | AddressMode::Accumulator => {
                1
            }
            AddressMode::Relative | AddressMode::Immediate
            | AddressMode::Indirect | AddressMode::IndirectY | AddressMode::IndirectX
            | AddressMode::ZeroPage | AddressMode::ZeroPageX | AddressMode::ZeroPageY => {
                2
            }
            AddressMode::Absolute | AddressMode::AbsoluteX | AddressMode::AbsoluteY => {
                3
            }
        }
    }

    fn decode_bytes(mode: AddressMode, opc: u8, buf: &[u8]) -> String {
        match mode {
            AddressMode::Implied | AddressMode::Accumulator => {
                format!("{:02X}      ", opc)
            }
            AddressMode::Relative |
                    AddressMode::Indirect |
                    AddressMode::ZeroPage |
                    AddressMode::Immediate |
                    AddressMode::ZeroPageX |
                    AddressMode::ZeroPageY |
                    AddressMode::IndirectX |
                    AddressMode::IndirectY
            => {
                format!("{:02X} {:02X}   ", opc, buf[0])
            }
            AddressMode::Absolute | AddressMode::AbsoluteX | AddressMode::AbsoluteY => {
                format!("{:02X} {:02X} {:02X}", opc, buf[0], buf[1])
            }
        }
    }

    fn decode_operands(mode: AddressMode, buf: &[u8]) -> Option<String> {
        match mode {
            AddressMode::Relative | AddressMode::Indirect | AddressMode::ZeroPage => {
                Some(format!("${:02X}", buf[0]))
            }
            AddressMode::Immediate => {
                Some(format!("#${:02X}", buf[0]))
            }
            AddressMode::Absolute => {
                Some(format!("${:04X}", Address::from_bytes(buf).to_u16()))
            }
            AddressMode::AbsoluteX => {
                Some(format!("${:04X},X", Address::from_bytes(buf).to_u16()))
            }
            AddressMode::AbsoluteY => {
                Some(format!("${:04X},Y", Address::from_bytes(buf).to_u16()))
            }
            AddressMode::ZeroPageX => {
                Some(format!("${:02X},X", buf[0]))
            }
            AddressMode::ZeroPageY => {
                Some(format!("${:02X},Y", buf[0]))
            }
            AddressMode::IndirectX => {
                Some(format!("(${:02X},X)", buf[0]))
            }
            AddressMode::IndirectY => {
                Some(format!("(${:02X}),Y", buf[0]))
            }
            _ => None
        }
    }
}

// NOTE I'm pretty confident we'll implement something like this
// or re-use this structure for decoding instructions in our CPU. I
// dont see the giant match statement scaling well.
pub static OPCODES: [Option<(&'static str, AddressMode)>; 256] = [
    // 0x00
    Some(("BRK", AddressMode::Implied)),
    Some(("ORA", AddressMode::IndirectX)),
    None,
    None,
    None,
    Some(("ORA", AddressMode::ZeroPage)),
    Some(("ASL", AddressMode::ZeroPage)),
    None,
    Some(("PHP", AddressMode::Implied)),
    Some(("ORA", AddressMode::Immediate)),
    Some(("ASL", AddressMode::Accumulator)),
    None,
    None,
    Some(("ORA", AddressMode::Absolute)),
    Some(("ASL", AddressMode::Absolute)),
    None,
    // 0x10
    Some(("BPL", AddressMode::Relative)),
    Some(("ORA", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("ORA", AddressMode::ZeroPageX)),
    Some(("ASL", AddressMode::ZeroPageX)),
    None,
    Some(("CLC", AddressMode::Implied)),
    Some(("ORA", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("ORA", AddressMode::AbsoluteX)),
    Some(("ASL", AddressMode::AbsoluteX)),
    None,
    // 0x20
    Some(("JSR", AddressMode::Absolute)),
    Some(("AND", AddressMode::IndirectX)),
    None,
    None,
    Some(("BIT", AddressMode::ZeroPage)),
    Some(("AND", AddressMode::ZeroPage)),
    Some(("ROL", AddressMode::ZeroPage)),
    None,
    Some(("PLP", AddressMode::Implied)),
    Some(("AND", AddressMode::Immediate)),
    Some(("ROL", AddressMode::Accumulator)),
    None,
    Some(("BIT", AddressMode::Absolute)),
    Some(("AND", AddressMode::Absolute)),
    Some(("ROL", AddressMode::Absolute)),
    None,
    // 0x30
    Some(("BMI", AddressMode::Relative)),
    Some(("AND", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("AND", AddressMode::ZeroPageX)),
    Some(("ROL", AddressMode::ZeroPageX)),
    None,
    Some(("SEC", AddressMode::Implied)),
    Some(("AND", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("AND", AddressMode::AbsoluteX)),
    Some(("ROL", AddressMode::AbsoluteX)),
    None,
    // 0x40
    Some(("RTI", AddressMode::Implied)),
    Some(("EOR", AddressMode::IndirectX)),
    None,
    None,
    None,
    Some(("EOR", AddressMode::ZeroPage)),
    Some(("LSR", AddressMode::ZeroPage)),
    None,
    Some(("PHA", AddressMode::Implied)),
    Some(("EOR", AddressMode::Immediate)),
    Some(("LSR", AddressMode::Accumulator)),
    None,
    Some(("JMP", AddressMode::Absolute)),
    Some(("EOR", AddressMode::Absolute)),
    Some(("LSR", AddressMode::Absolute)),
    None,
    // 0x50
    Some(("BVC", AddressMode::Relative)),
    Some(("EOR", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("EOR", AddressMode::ZeroPageX)),
    Some(("LSR", AddressMode::ZeroPageX)),
    None,
    None,
    Some(("EOR", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("EOR", AddressMode::AbsoluteX)),
    Some(("LSR", AddressMode::AbsoluteX)),
    None,
    // 0x60
    Some(("RTS", AddressMode::Implied)),
    Some(("ADC", AddressMode::IndirectX)),
    None,
    None,
    None,
    Some(("ADC", AddressMode::ZeroPage)),
    Some(("ROR", AddressMode::ZeroPage)),
    None,
    Some(("PLA", AddressMode::Implied)),
    Some(("ADC", AddressMode::Immediate)),
    Some(("ROR", AddressMode::Accumulator)),
    None,
    Some(("JMP", AddressMode::Indirect)),
    Some(("ADC", AddressMode::Absolute)),
    Some(("ROR", AddressMode::Absolute)),
    None,
    // 0x70
    Some(("BVS", AddressMode::Relative)),
    Some(("ADC", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("ADC", AddressMode::ZeroPageX)),
    Some(("ROR", AddressMode::ZeroPageX)),
    None,
    Some(("SEI", AddressMode::Implied)),
    Some(("ADC", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("ADC", AddressMode::AbsoluteX)),
    Some(("ROR", AddressMode::AbsoluteX)),
    None,
    // 0x80
    None,
    Some(("STA", AddressMode::IndirectX)),
    None,
    None,
    Some(("STY", AddressMode::ZeroPage)),
    Some(("STA", AddressMode::ZeroPage)),
    Some(("STX", AddressMode::ZeroPage)),
    None,
    Some(("DEY", AddressMode::Implied)),
    None,
    Some(("TXA", AddressMode::Implied)),
    None,
    Some(("STY", AddressMode::Absolute)),
    Some(("STA", AddressMode::Absolute)),
    Some(("STX", AddressMode::Absolute)),
    None,
    // 0x90
    Some(("BCC", AddressMode::Relative)),
    Some(("STA", AddressMode::IndirectY)),
    None,
    None,
    Some(("STY", AddressMode::ZeroPageX)),
    Some(("STA", AddressMode::ZeroPageX)),
    Some(("STX", AddressMode::ZeroPageY)),
    None,
    Some(("TYA", AddressMode::Implied)),
    Some(("STA", AddressMode::AbsoluteY)),
    Some(("TXS", AddressMode::Implied)),
    None,
    None,
    Some(("STA", AddressMode::AbsoluteX)),
    None,
    None,
    // 0xA0
    Some(("LDY", AddressMode::Immediate)),
    Some(("LDA", AddressMode::IndirectX)),
    Some(("LDX", AddressMode::Immediate)),
    None,
    Some(("LDY", AddressMode::ZeroPage)),
    Some(("LDA", AddressMode::ZeroPage)),
    Some(("LDX", AddressMode::ZeroPage)),
    None,
    Some(("TAY", AddressMode::Implied)),
    Some(("LDA", AddressMode::Immediate)),
    Some(("TAX", AddressMode::Implied)),
    None,
    Some(("LDY", AddressMode::Absolute)),
    Some(("LDA", AddressMode::Absolute)),
    Some(("LDX", AddressMode::Absolute)),
    None,
    // 0xB0
    Some(("BCS", AddressMode::Relative)),
    Some(("LDA", AddressMode::IndirectY)),
    None,
    None,
    Some(("LDY", AddressMode::ZeroPageX)),
    Some(("LDA", AddressMode::ZeroPageX)),
    Some(("LDX", AddressMode::ZeroPageY)),
    None,
    Some(("CLV", AddressMode::Implied)),
    Some(("LDA", AddressMode::AbsoluteY)),
    Some(("TSX", AddressMode::Implied)),
    None,
    Some(("LDY", AddressMode::AbsoluteX)),
    Some(("LDA", AddressMode::AbsoluteX)),
    Some(("LDX", AddressMode::AbsoluteY)),
    None,
    // 0xC0
    Some(("CPY", AddressMode::Immediate)),
    Some(("CMP", AddressMode::IndirectX)),
    None,
    None,
    Some(("CPY", AddressMode::ZeroPage)),
    Some(("CMP", AddressMode::ZeroPage)),
    Some(("DEC", AddressMode::ZeroPage)),
    None,
    Some(("INY", AddressMode::Implied)),
    Some(("CMP", AddressMode::Immediate)),
    Some(("DEX", AddressMode::Implied)),
    None,
    Some(("CPY", AddressMode::Absolute)),
    Some(("CMP", AddressMode::Absolute)),
    Some(("DEC", AddressMode::Absolute)),
    None,
    // 0xD0
    Some(("BNE", AddressMode::Relative)),
    Some(("CMP", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("CMP", AddressMode::ZeroPageX)),
    Some(("DEC", AddressMode::ZeroPageX)),
    None,
    Some(("CLD", AddressMode::Implied)),
    Some(("CMP", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("CMP", AddressMode::AbsoluteX)),
    Some(("DEC", AddressMode::AbsoluteX)),
    None,
    // 0xE0
    Some(("CPX", AddressMode::Immediate)),
    Some(("SBC", AddressMode::IndirectX)),
    None,
    None,
    Some(("CPX", AddressMode::ZeroPage)),
    Some(("SBC", AddressMode::ZeroPage)),
    Some(("INC", AddressMode::ZeroPage)),
    None,
    Some(("INX", AddressMode::Implied)),
    Some(("SBC", AddressMode::Immediate)),
    Some(("NOP", AddressMode::Implied)),
    None,
    Some(("CPX", AddressMode::Absolute)),
    Some(("SBC", AddressMode::Absolute)),
    Some(("INC", AddressMode::Absolute)),
    None,
    // 0xF0
    Some(("BEQ", AddressMode::Relative)),
    Some(("SBC", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("SBC", AddressMode::ZeroPageX)),
    Some(("INC", AddressMode::ZeroPageX)),
    None,
    Some(("SED", AddressMode::Implied)),
    Some(("SBC", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("SBC", AddressMode::AbsoluteX)),
    Some(("INC", AddressMode::AbsoluteX)),
    None,
];
