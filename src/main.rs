#![feature(asm)]

use std::env;
use std::fs;
use std::io::Read;

#[macro_use]
#[no_link]
extern crate probe;

mod nes;

fn main() {
    let filename = env::args().nth(1).unwrap();

    let mut file = fs::File::open(&filename).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    let mut nes = nes::Nes::new(file_buf);
    nes.start_emulation();
}
