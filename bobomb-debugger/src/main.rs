#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(grammar);

use clap::{App, Arg, value_t};

mod ast;
mod client;
mod ctrl_c;
mod disassemble;
mod parser;
mod repl;

pub use repl::Repl;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Opts<'a> {
    pub host: &'a str,
    pub port: u16,
    pub debug_requests: bool,
}

fn main() {
    let args = App::new("bobomb-debugger")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .help("Hostname/IP address of Bobomb remote debugger API")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Port number of Bobomb remote debugger API")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug_api")
                .long("debug-api")
                .help("Print debug logs from Bobomb API"),
        )
        .get_matches();

    let opts = Opts {
        host: args.value_of("host").unwrap_or("127.0.0.1"),
        port: value_t!(args.value_of("port"), u16).unwrap_or(6502),
        debug_requests: args.is_present("debug_api"),
    };
    let mut cli = Repl::new(opts).unwrap();
    cli.run();
}
