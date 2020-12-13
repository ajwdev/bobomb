use anyhow::*;
use ansi_term::Color::{Blue,Yellow};

// TODO Consider moving this into a common package to avoid pulling in all of core
#[derive(Debug, Copy, Clone)]
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

impl AddressMode {
    pub fn len(&self) -> usize {
        match *self {
            AddressMode::Implied | AddressMode::Accumulator => 1,

            AddressMode::Relative
            | AddressMode::Immediate
            | AddressMode::Indirect
            | AddressMode::IndirectY
            | AddressMode::IndirectX
            | AddressMode::ZeroPage
            | AddressMode::ZeroPageX
            | AddressMode::ZeroPageY => 2,

            AddressMode::Absolute | AddressMode::AbsoluteX | AddressMode::AbsoluteY => 3,
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Instruction {
    pub name: &'static str,
    pub mode: AddressMode,

    address: usize,
    offset: usize,
    byte0: u8,
    byte1: Option<u8>,
    byte2: Option<u8>,
}

impl Instruction {
    pub fn len(&self) -> usize {
        self.mode.len()
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {yellow}{:>6}:{yellow_end}  {} {:>7}  ; {}",
            Blue.paint(self.format_location()),
            self.format_offset(),
            self.name,
            self.format_address_mode(),
            self.format_bytes(),
            yellow = Yellow.prefix(),
            yellow_end = Yellow.suffix(),
        )
    }

    fn format_location(&self) -> String {
        format!("{:#06x}", self.address)
    }

    fn format_offset(&self) -> String {
        format!("<+{}>", self.offset)
    }

    fn format_address_mode(&self) -> String {
        match self.mode {
            AddressMode::Implied => {
                String::new()
            }
            AddressMode::Accumulator => {
                "A".to_string()
            }
            AddressMode::Immediate => {
                format!("#{}", self.format_addr())
            }
            AddressMode::Absolute | AddressMode::ZeroPage | AddressMode::Relative => {
                format!("{}", self.format_addr())
            }
            AddressMode::AbsoluteY | AddressMode::ZeroPageY => {
                format!("{},Y",  self.format_addr())
            }
            AddressMode::AbsoluteX | AddressMode::ZeroPageX => {
                format!("{},X",  self.format_addr())
            }
            AddressMode::Indirect => {
                format!("({})",  self.format_addr())
            }
            AddressMode::IndirectX => {
                format!("({},X)",  self.format_addr())
            }
            AddressMode::IndirectY => {
                format!("({}),Y",  self.format_addr())
            }
        }
    }

    fn format_bytes(&self) -> String {
        match (self.byte1, self.byte2) {
            (None, None) =>
                format!("{:#04x}", self.byte0),
            (Some(a), None) =>
                format!("{:#04x} {:#04x}", self.byte0, a),
            (Some(a), Some(b)) =>
                format!("{:#04x} {:#04x} {:#04x}", self.byte0, a, b),

            _ => panic!("first byte None but second byte Some")
        }

    }

    fn format_addr(&self) -> String {
        match (self.byte1, self.byte2) {
            (None, None) => String::new(),

            (Some(a), None) => {
                let s = format!("{:#04x}", a);
                format!("${}", s.strip_prefix("0x").unwrap())
            }

            (Some(a), Some(b)) => {
                let addr: u16 = (a as u16) << 8 | (b as u16);
                let s = format!("{:#06x}", addr);

                format!("${}", s.strip_prefix("0x").unwrap())
            }

            _ => panic!("first byte None but second byte Some")
        }
    }
}

#[derive(Debug)]
pub struct Disassembly {
    instructions: Vec<Instruction>,
}

pub fn disassemble_instruction<'a,I: Iterator<Item = &'a u8>>(
    address: usize,
    offset: usize,
    iter: &mut I,
) -> Result<Option<Instruction>> {
    let b = iter.next();
    if b.is_none() {
        return Ok(None);
    }

    let opc = OPCODES[*b.unwrap() as usize]
        .ok_or_else(|| anyhow!("opcode {:?} unrecognized", b))?;

    let mut buf: [Option<u8>; 2] = [None, None];
    let mut i = 0;

    while i < opc.1.len()-1 {
        buf[i] = iter.next().map(|x| *x);
        if buf[i].is_none() {
            bail!("this shouldnt be none");
        }

        i += 1;
    }

    Ok(Some(Instruction {
        name: opc.0,
        mode: opc.1,
        address,
        offset,
        byte0: *b.unwrap(),
        byte1: buf[0],
        byte2: buf[1],
    }))
}

impl Disassembly{
    pub fn disassemble<'a,I: Iterator<Item = &'a u8>>(
        start_address: usize,
        iter: &mut I,
    ) -> Result<Self> {
        let mut output: Vec<Instruction> = Vec::new();
        let mut addr = start_address;
        let mut offset = 0;

        while let Some(instr) = disassemble_instruction(addr, offset, iter)? {
            output.push(instr);
            addr += instr.len();
            offset += instr.len();
        }

        Ok(Self{instructions: output})
    }

    pub fn print(&self, program_counter: usize) {
        for instr in &self.instructions {
            if instr.address == program_counter {
                print!("=> ");
            } else {
                print!("   ");
            }

            println!("{}", instr.to_string());
        }
    }
}

pub(crate) static OPCODES: [Option<(&'static str, AddressMode)>; 256] = [
    // 0x00
    Some(("brk", AddressMode::Implied)),
    Some(("ora", AddressMode::IndirectX)),
    None,
    None,
    None,
    Some(("ora", AddressMode::ZeroPage)),
    Some(("asl", AddressMode::ZeroPage)),
    None,
    Some(("php", AddressMode::Implied)),
    Some(("ora", AddressMode::Immediate)),
    Some(("asl", AddressMode::Accumulator)),
    None,
    None,
    Some(("ora", AddressMode::Absolute)),
    Some(("asl", AddressMode::Absolute)),
    None,
    // 0x10
    Some(("bpl", AddressMode::Relative)),
    Some(("ora", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("ora", AddressMode::ZeroPageX)),
    Some(("asl", AddressMode::ZeroPageX)),
    None,
    Some(("clc", AddressMode::Implied)),
    Some(("ora", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("ora", AddressMode::AbsoluteX)),
    Some(("asl", AddressMode::AbsoluteX)),
    None,
    // 0x20
    Some(("jsr", AddressMode::Absolute)),
    Some(("and", AddressMode::IndirectX)),
    None,
    None,
    Some(("bit", AddressMode::ZeroPage)),
    Some(("and", AddressMode::ZeroPage)),
    Some(("rol", AddressMode::ZeroPage)),
    None,
    Some(("plp", AddressMode::Implied)),
    Some(("and", AddressMode::Immediate)),
    Some(("rol", AddressMode::Accumulator)),
    None,
    Some(("bit", AddressMode::Absolute)),
    Some(("and", AddressMode::Absolute)),
    Some(("rol", AddressMode::Absolute)),
    None,
    // 0x30
    Some(("bmi", AddressMode::Relative)),
    Some(("and", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("and", AddressMode::ZeroPageX)),
    Some(("rol", AddressMode::ZeroPageX)),
    None,
    Some(("sec", AddressMode::Implied)),
    Some(("and", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("and", AddressMode::AbsoluteX)),
    Some(("rol", AddressMode::AbsoluteX)),
    None,
    // 0x40
    Some(("rti", AddressMode::Implied)),
    Some(("eor", AddressMode::IndirectX)),
    None,
    None,
    None,
    Some(("eor", AddressMode::ZeroPage)),
    Some(("lsr", AddressMode::ZeroPage)),
    None,
    Some(("pha", AddressMode::Implied)),
    Some(("eor", AddressMode::Immediate)),
    Some(("lsr", AddressMode::Accumulator)),
    None,
    Some(("jmp", AddressMode::Absolute)),
    Some(("eor", AddressMode::Absolute)),
    Some(("lsr", AddressMode::Absolute)),
    None,
    // 0x50
    Some(("bvc", AddressMode::Relative)),
    Some(("eor", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("eor", AddressMode::ZeroPageX)),
    Some(("lsr", AddressMode::ZeroPageX)),
    None,
    None,
    Some(("eor", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("eor", AddressMode::AbsoluteX)),
    Some(("lsr", AddressMode::AbsoluteX)),
    None,
    // 0x60
    Some(("rts", AddressMode::Implied)),
    Some(("adc", AddressMode::IndirectX)),
    None,
    None,
    None,
    Some(("adc", AddressMode::ZeroPage)),
    Some(("ror", AddressMode::ZeroPage)),
    None,
    Some(("pla", AddressMode::Implied)),
    Some(("adc", AddressMode::Immediate)),
    Some(("ror", AddressMode::Accumulator)),
    None,
    Some(("jmp", AddressMode::Indirect)),
    Some(("adc", AddressMode::Absolute)),
    Some(("ror", AddressMode::Absolute)),
    None,
    // 0x70
    Some(("bvs", AddressMode::Relative)),
    Some(("adc", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("adc", AddressMode::ZeroPageX)),
    Some(("ror", AddressMode::ZeroPageX)),
    None,
    Some(("sei", AddressMode::Implied)),
    Some(("adc", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("adc", AddressMode::AbsoluteX)),
    Some(("ror", AddressMode::AbsoluteX)),
    None,
    // 0x80
    None,
    Some(("sta", AddressMode::IndirectX)),
    None,
    None,
    Some(("sty", AddressMode::ZeroPage)),
    Some(("sta", AddressMode::ZeroPage)),
    Some(("stx", AddressMode::ZeroPage)),
    None,
    Some(("dey", AddressMode::Implied)),
    None,
    Some(("txa", AddressMode::Implied)),
    None,
    Some(("sty", AddressMode::Absolute)),
    Some(("sta", AddressMode::Absolute)),
    Some(("stx", AddressMode::Absolute)),
    None,
    // 0x90
    Some(("bcc", AddressMode::Relative)),
    Some(("sta", AddressMode::IndirectY)),
    None,
    None,
    Some(("sty", AddressMode::ZeroPageX)),
    Some(("sta", AddressMode::ZeroPageX)),
    Some(("stx", AddressMode::ZeroPageY)),
    None,
    Some(("tya", AddressMode::Implied)),
    Some(("sta", AddressMode::AbsoluteY)),
    Some(("txs", AddressMode::Implied)),
    None,
    None,
    Some(("sta", AddressMode::AbsoluteX)),
    None,
    None,
    // 0xA0
    Some(("ldy", AddressMode::Immediate)),
    Some(("lda", AddressMode::IndirectX)),
    Some(("ldx", AddressMode::Immediate)),
    None,
    Some(("ldy", AddressMode::ZeroPage)),
    Some(("lda", AddressMode::ZeroPage)),
    Some(("ldx", AddressMode::ZeroPage)),
    None,
    Some(("tay", AddressMode::Implied)),
    Some(("lda", AddressMode::Immediate)),
    Some(("tax", AddressMode::Implied)),
    None,
    Some(("ldy", AddressMode::Absolute)),
    Some(("lda", AddressMode::Absolute)),
    Some(("ldx", AddressMode::Absolute)),
    None,
    // 0xB0
    Some(("bcs", AddressMode::Relative)),
    Some(("lda", AddressMode::IndirectY)),
    None,
    None,
    Some(("ldy", AddressMode::ZeroPageX)),
    Some(("lda", AddressMode::ZeroPageX)),
    Some(("ldx", AddressMode::ZeroPageY)),
    None,
    Some(("clv", AddressMode::Implied)),
    Some(("lda", AddressMode::AbsoluteY)),
    Some(("tsx", AddressMode::Implied)),
    None,
    Some(("ldy", AddressMode::AbsoluteX)),
    Some(("lda", AddressMode::AbsoluteX)),
    Some(("ldx", AddressMode::AbsoluteY)),
    None,
    // 0xC0
    Some(("cpy", AddressMode::Immediate)),
    Some(("cmp", AddressMode::IndirectX)),
    None,
    None,
    Some(("cpy", AddressMode::ZeroPage)),
    Some(("cmp", AddressMode::ZeroPage)),
    Some(("dec", AddressMode::ZeroPage)),
    None,
    Some(("iny", AddressMode::Implied)),
    Some(("cmp", AddressMode::Immediate)),
    Some(("dex", AddressMode::Implied)),
    None,
    Some(("cpy", AddressMode::Absolute)),
    Some(("cmp", AddressMode::Absolute)),
    Some(("dec", AddressMode::Absolute)),
    None,
    // 0xD0
    Some(("bne", AddressMode::Relative)),
    Some(("cmp", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("cmp", AddressMode::ZeroPageX)),
    Some(("dec", AddressMode::ZeroPageX)),
    None,
    Some(("cld", AddressMode::Implied)),
    Some(("cmp", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("cmp", AddressMode::AbsoluteX)),
    Some(("dec", AddressMode::AbsoluteX)),
    None,
    // 0xE0
    Some(("cpx", AddressMode::Immediate)),
    Some(("sbc", AddressMode::IndirectX)),
    None,
    None,
    Some(("cpx", AddressMode::ZeroPage)),
    Some(("sbc", AddressMode::ZeroPage)),
    Some(("inc", AddressMode::ZeroPage)),
    None,
    Some(("inx", AddressMode::Implied)),
    Some(("sbc", AddressMode::Immediate)),
    Some(("nop", AddressMode::Implied)),
    None,
    Some(("cpx", AddressMode::Absolute)),
    Some(("sbc", AddressMode::Absolute)),
    Some(("inc", AddressMode::Absolute)),
    None,
    // 0xF0
    Some(("beq", AddressMode::Relative)),
    Some(("sbc", AddressMode::IndirectY)),
    None,
    None,
    None,
    Some(("sbc", AddressMode::ZeroPageX)),
    Some(("inc", AddressMode::ZeroPageX)),
    None,
    Some(("sed", AddressMode::Implied)),
    Some(("sbc", AddressMode::AbsoluteY)),
    None,
    None,
    None,
    Some(("sbc", AddressMode::AbsoluteX)),
    Some(("inc", AddressMode::AbsoluteX)),
    None,
];
