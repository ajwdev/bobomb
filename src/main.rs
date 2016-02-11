use std::env;
use std::fs;
use std::io::Read;

// TODO Review Rust's module system
mod cpu;
mod mem;

fn main() {
    let filename = env::args().nth(1).unwrap();

    let mut file = fs::File::open(&filename).unwrap();
    let mut file_buf = Vec::new();

    file.read_to_end(&mut file_buf).unwrap();
    if !validate_header(&file_buf) {
        panic!("header validation failed: {:?}", &file_buf[0..16]);
    }

    let mut memory = mem::Memory::new(file_buf);
    let mut cpu = cpu::Cpu::new(memory);
    cpu.start();
}

fn validate_header(rom: &Vec<u8>) -> bool {
    // TODO Make this better
    if rom.len() < 4 {
        return false
    }

    let header: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a]; // NES^Z
    if header != &rom[0..4] {
        return false
    }

    true
}
