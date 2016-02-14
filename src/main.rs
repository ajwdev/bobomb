use std::env;
use std::fs;
use std::io::Read;

// TODO Review Rust's module system
mod cpu;
mod ppu;
//mod mem;

fn main() {
    let filename = env::args().nth(1).unwrap();

    let mut file = fs::File::open(&filename).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    let header = &file_buf[0..16];
    if !validate_header(header) {
        panic!("header validation failed: {:?}", &header);
    }

    let memory: cpu::AddressSpace;
    if rom_is_double_banked(&header) {
        memory = cpu::AddressSpace::new(
            &file_buf[16..16*1024+16], &file_buf[16*1024+16..]
        );
    } else {
        memory = cpu::AddressSpace::new(
            &file_buf[16..16*1024+16], &file_buf[16..16*1024+16]
        );
    }
    let mut cpu = cpu::Cpu::new(memory);
    cpu.start();
}

fn validate_header(rom: &[u8]) -> bool {
    // TODO Make this entire function better
    if rom.len() < 4 {
        return false
    }

    let header: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a]; // NES^Z
    if header != &rom[0..4] {
        return false
    }

    true
}

// TODO How is this going to work with catridges with more than two
// banks (i.e catridges with an MMC)?
//
// NOTE The `file` command on my Fedora 23 box appears to understand
// iNes formatted roms. Example:
//
// $ file ~/roms/example.nes
// /home/andrew/roms/example.nes: iNES ROM dump, 2x16k PRG, 1x8k CHR, [Vert.]
// $ file ~/roms/example2.nes
// /home/andrew/roms/example2.nes: iNES ROM dump, 1x16k PRG, 1x8k CHR, [Horiz.]
//
fn rom_is_double_banked(rom: &[u8]) -> bool {
    if rom.len() < 16 {
        panic!("rom header is too small. expected 16 bytes, got #{}", rom.len());
    }

    // The 5th byte indicates how large the ROM should be
    match rom[4] {
        // TODO Is this panic worthy or should we assume 1? Afterall, you can't
        // really have a ROM without any program data.
        0 => { panic!("rom unit size cannot be zero"); }

        // Single bank
        1 => { return false; }

        // Double bank
        2 => { return true; }

        // If we get here then MMC's do affect the PRG ROM unit size. My
        // guess is that they do.
        _ => { panic!("unrecognized rom unit size {}", rom[4]); }
    }
}
