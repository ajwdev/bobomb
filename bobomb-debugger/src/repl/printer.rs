use std::fmt;
use std::marker::PhantomData;

use ansi_term::Color::{Blue, Red};
use anyhow::*;
use num_traits::{AsPrimitive, Num, Unsigned};

use crate::ast::{Display, Format};
use crate::disassemble::Disassembly;

const CHUNK_SIZE: usize = 8;

pub struct Printer<T: Num + fmt::Display + fmt::LowerHex + fmt::Binary> {
    last_print: Format,
    last_examine: Format,
    _expression_num_type: PhantomData<T>,
}

impl<T: Num + fmt::Display + fmt::LowerHex + fmt::Binary> Printer<T> {
    pub fn new() -> Self {
        Self {
            last_print: Format {
                display: Some(Display::Decimal),
                count: Some(1),
            },
            last_examine: Format {
                display: Some(Display::Hex),
                count: Some(1),
            },
            _expression_num_type: PhantomData,
        }
    }

    pub fn update_examine_format(&mut self, fmt: Option<Format>) -> Format {
        self.last_examine.merge(fmt)
    }

    pub fn update_print_format(&mut self, fmt: Option<Format>) -> Format {
        self.last_print.merge(fmt)
    }

    pub fn examine<N: Unsigned + AsPrimitive<usize>>(
        &mut self,
        start: N,
        pc: N,
        data: &[u8],
    ) -> Result<()> {
        self.examine_with_format(start, pc, data, self.last_examine)
    }

    pub fn examine_with_format<N: Unsigned + AsPrimitive<usize>>(
        &mut self,
        start: N,
        pc: N,
        data: &[u8],
        fmt: Format,
    ) -> Result<()> {
        match fmt.display.unwrap_or(Display::Decimal) {
            Display::Decimal => self.print_decimal(start.as_(), data),
            Display::Hex => self.print_hex(start.as_(), data),
            Display::Binary => self.print_binary(start.as_(), data),
            Display::Instruction => self.print_disassembly(start.as_(), pc.as_(), data)?,
        }
        Ok(())
    }

    pub fn print(&mut self, name: &str, num: T) -> Result<()> {
        self.print_with_format(name, num, self.last_print)
    }

    pub fn print_with_format(&mut self, name: &str, num: T, fmt: Format) -> Result<()> {
        match fmt.display.unwrap_or(Display::Decimal) {
            Display::Decimal => println!("{} = {}", name, num),
            Display::Hex => println!("{} = {:#06x}", name, num),
            Display::Binary => println!("{} = {:#010b}", name, num),
            Display::Instruction => {
                bail!("Format instruction unsupported in print command. Use Examine instead.")
            }
        }

        Ok(())
    }

    fn print_hex(&self, start: usize, data: &[u8]) {
        let mut addr = start;

        for chunk in data.chunks(CHUNK_SIZE) {
            println!(
                "{:#06x}:  {}",
                addr,
                chunk
                    .iter()
                    .map(|x| format!("{:#04x}", x))
                    .collect::<Vec<String>>()
                    .join("  "),
            );
            addr += CHUNK_SIZE;
        }
    }

    fn print_binary(&self, start: usize, data: &[u8]) {
        let mut addr = start;

        for chunk in data.chunks(CHUNK_SIZE) {
            println!(
                "{:#06x}:  {}",
                addr,
                chunk
                    .iter()
                    .map(|x| format!("{:#010b}", x))
                    .collect::<Vec<String>>()
                    .join("  "),
            );
            addr += CHUNK_SIZE;
        }
    }

    fn print_decimal(&self, start: usize, data: &[u8]) {
        let mut addr = start;

        for chunk in data.chunks(CHUNK_SIZE) {
            println!(
                "{:#06x}:  {}",
                addr,
                chunk
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("  "),
            );
            addr += CHUNK_SIZE;
        }
    }

    fn print_disassembly(&self, start: usize, pc: usize, data: &[u8]) -> Result<()> {
        let dis = Disassembly::disassemble(start, &mut data.iter())?;
        dis.print(pc);
        Ok(())
    }
}

impl crate::ast::Format {
    pub fn merge(&mut self, next: Option<Format>) -> Format {
        *self = self.clone().combine(next);
        *self
    }

    pub fn combine(mut self, next: Option<Format>) -> Format {
        match next {
            None => self,
            Some(unwrapped) => {
                if unwrapped.display.is_some() {
                    self.display = unwrapped.display;
                    // We specified a display format so we must reset the count as well
                    self.count = unwrapped.count.or(Some(1));
                } else {
                    // We're not setting the format but might be displaying more items
                    self.count = unwrapped.count.or(self.count);
                }

                self
            }
        }
    }
}

//
// Util
//

fn error_display<T: std::fmt::Display>(msg: T) {
    eprintln!("{}: {}", Red.bold().paint("Error"), msg)
}

pub fn error(why: anyhow::Error) {
    error_display(why)
}

pub fn parse_error<'input, T, E>(
    line: &'input str,
    err: lalrpop_util::ParseError<usize, T, E>,
) -> ()
where
    T: fmt::Display,
    E: fmt::Display,
{
    let highlight = |start: usize, end: usize| {
        eprint!("{} |\n | {}", Blue.prefix(), Blue.suffix());
        eprintln!("{}", line);
        eprint!("{} | {}", Blue.prefix(), Blue.suffix());
        eprint!("{}", " ".repeat(start));
        eprintln!(
            "{}{}{}",
            Red.prefix(),
            "^".repeat(end - start),
            Red.suffix(),
        );
    };

    error_display(&err);

    use lalrpop_util::ParseError;
    match err {
        ParseError::InvalidToken { location } => highlight(location, location + 1),

        ParseError::UnrecognizedEOF { location, .. } => highlight(location, location + 1),

        ParseError::UnrecognizedToken {
            token: (start, _, end),
            ..
        } => highlight(start, end),

        _ => {}
    }
}

pub fn debug<T: std::fmt::Display>(msg: T) {
    eprintln!("{}: {}", Blue.bold().paint("Debug"), msg)
}
