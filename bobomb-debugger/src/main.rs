#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(grammar);

mod ast;
mod parser;
mod ctrl_c;
mod client;
mod disassemble;
mod repl;

pub use repl::{Repl,Config};

fn main() {
    let cfg = Config{
        host: String::from("127.0.0.1"),
        port: 6502,
        debug_requests: true,
    };
    let mut cli = Repl::new(cfg).unwrap();
    cli.run();
}
