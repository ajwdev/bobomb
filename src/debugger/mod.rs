use lalrpop_util::lalrpop_mod;
lalrpop_mod!(grammar, "/debugger/grammar.rs");

pub mod ast;
pub mod disassemble;
pub mod parser;
