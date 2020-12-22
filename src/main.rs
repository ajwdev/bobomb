#![feature(asm)]

use anyhow::Result;
use clap::{value_t, App, Arg};

use std::env;
use std::fs;
use std::io::Read;
use std::u16;

#[macro_use]
#[no_link]
extern crate probe;

mod nes;
use crate::nes::executor::Executor;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Opts {
    pub program_counter: Option<u16>,
    pub wait_for_attach: bool,
}

fn main() -> Result<()> {
    // let filename = env::args().nth(1).unwrap();

    // let mut file = fs::File::open(&filename).unwrap();
    // let mut file_buf = Vec::new();
    // file.read_to_end(&mut file_buf).unwrap();
    let args = App::new("bobomb")
        .arg(
            Arg::with_name("program-counter")
                .long("program-counter")
                .help("Start CPU emulation at given hex value instead of the reset vector. Only useful for debugging/tests")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("wait")
                .long("wait")
                .short("w")
                .required(false)
                .takes_value(false)
                .help("Wait for debugger to attach")
        )
        .arg(
            Arg::with_name("rom")
                .index(1)
                .value_name("FILE")
                .help("Path to NES rom")
                .required(true),
        )
        .get_matches();

    let opts = Opts {
        wait_for_attach: args.is_present("wait"),
        program_counter: args.value_of("program-counter").map(|s| {
            let n = s.strip_prefix("0x").unwrap_or(s);
            u16::from_str_radix(&n, 16).expect("could not parse hex program counter")
        }),
    };

    let mut buf = Vec::new();
    {
        let mut file = fs::File::open(args.value_of("rom").unwrap())?;
        file.read_to_end(&mut buf).unwrap();
    }

    let nes = nes::Nes::new(buf, &opts);
    let executor = Executor::new(nes, &opts)?;
    executor.run()
}
