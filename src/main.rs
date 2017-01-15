#![feature(asm)]
#![feature(inclusive_range_syntax)]

#[macro_use]
#[no_link]
extern crate probe;

// For the debugger API
extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

mod nes;

use std::env;
use std::fs;
use std::io::Read;

use nes::cpu;
use nes::ppu;
use nes::rom;
use nes::interconnect::Interconnect;

fn main() {
    let filename = env::args().nth(1).unwrap();

    let mut file = fs::File::open(&filename).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    let mut nes = nes::Nes::new(file_buf);
    nes.start_emulation();
}
