#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(grammar);

mod ast;
mod disassemble;
mod repl;
pub use repl::Repl;

fn main() {
    let mut cli = Repl::new("127.0.0.1", 6502).unwrap();
    cli.run();
}
