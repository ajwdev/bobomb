#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(grammar);

use std::collections::HashMap;

mod ast;
mod disassemble;
mod repl;
pub use repl::Repl;

fn main() {
    // let p = grammar::ExprParser::new();
    // println!("{}", p.parse("$foo").unwrap());

    // let mut hsh = HashMap::<String,i32>::new();
    // hsh.insert("$foo".to_string(), 10);
    // hsh.insert("$bar".to_string(), 2);
    // hsh.insert("$cpu.pc".to_string(), 0x4040);
    // println!("{}", p.parse("$cpu.pc").unwrap().reduce(&hsh).unwrap());
    // println!("{}", p.parse("$foo * $bar").unwrap().reduce(&hsh).unwrap());

    // println!("{}", p.parse("2 * 4").unwrap().reduce());
    // println!("{}", p.parse("2 + 4 * 2").unwrap().reduce());
    // println!("{:#?}", p.parse("(2 + 4) * 2").unwrap());
    // println!("{}", p.parse("(2 + 4) * 2").unwrap().reduce());
    // // println!("{:?}", p.parse("64").unwrap());
    // // println!("{:?}", p.parse("0xff").unwrap());
    // // println!("{:?}", p.parse("0xFF").unwrap());
    // // println!("{:?}", p.parse("((0xFF))").unwrap());
    // // println!("{:?}", p.parse("1 + 2").unwrap());
    // // println!("{:?}", p.parse("1 + 2 * 3").unwrap());

    // let f = FmtExprParser::new();
    // println!("{:?}", f.parse("/x").unwrap());
    // // // println!("{:?}", f.parse("x / 10 x").unwrap());
    // println!("{:?}", f.parse("/10x").unwrap());
    // println!("{:?}", f.parse("/-10x").unwrap());

    let c = grammar::CommandParser::new();
    println!("{:?}", c.parse("ping foo bar baz").unwrap());
    // println!("{:?}", c.parse("x").unwrap());
    // println!("{:?}", c.parse("p /10i").unwrap());
    // println!("{:?}", c.parse("p 0x40 << 2").unwrap());
    // println!("{:?}", c.parse("p/i 0x40 << 2").unwrap());
    // println!("{:?}", c.parse("status").unwrap());
    // println!("{:?}", c.parse("break 0x4040").unwrap());
    // println!("{:?}", c.parse("break").unwrap());
    // println!("{:?}", c.parse("clear 1").unwrap());
    // println!("{:?}", c.parse("ping").unwrap());
    // println!("{:?}", c.parse("c").unwrap());
    // println!("{:?}", c.parse("stop").unwrap());

    // match c.parse("p/x (2 + 4) * 2").unwrap() {
    //     Cmd::Print(expr, fmt) => {
    //         match fmt {
    //             Some(x) => {
    //                 match x.display {
    //                     Display::Hex => println!("{:#04x}", expr.unwrap().reduce()),
    //                     Display::Decimal => println!("{}", expr.unwrap().reduce()),
    //                     Display::Instruction => panic!("not implemented"),
    //                 }
    //             },
    //             None => println!("{}", expr.unwrap().reduce()),
    //         }
    //     },
    //     _ => panic!("nope"),
    // }

    let mut cli = Repl::new("127.0.0.1", 6502).unwrap();
    cli.run();
}
