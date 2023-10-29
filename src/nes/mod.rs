use anyhow::*;
use parking_lot::{Mutex};
use std::sync::Arc;



pub mod macros;
pub mod cpu;
pub mod ppu;
pub mod rom;
pub mod controller;
pub mod address;
pub mod interconnect;
pub mod debugger;

pub mod executor;
pub use crate::nes::executor::ExecutorLock;

#[derive(Default)]
pub struct StepInfo {
    program_counter: u16,
    should_paint: bool,
}

pub struct Nes {
    cpu: cpu::Cpu,
    interconnect: Arc<Mutex<interconnect::Interconnect>>,

    rom_header: [u8; 16],
}

impl Nes {
    pub fn new(rom_buffer: &[u8], program_counter: Option<u16>) -> Nes {
        let mut header = [0u8; 16];
        header.copy_from_slice(&rom_buffer[0..16]);

        if !Self::validate_header(&header) {
            panic!("header validation failed: {:?}", &header);
        }

        // TODO const these
        let bank0_start = 16;
        let bank0_end = 16 * 1024 + bank0_start;
        let bank1_start = bank0_end;
        let bank1_end = 16 * 1024 + bank1_start;

        let rom = if Self::rom_is_double_banked(&header) {
            println!("ROM is double banked");
            let bank0 = rom::Bank::new(&rom_buffer[bank0_start..bank0_end]);
            let bank1 = rom::Bank::new(&rom_buffer[bank1_start..bank1_end]);
            rom::Rom::new_double_bank(bank0, bank1)
        } else {
            println!("ROM is single banked");
            let bank = rom::Bank::new(&rom_buffer[bank0_start..bank0_end]);
            rom::Rom::new_single_bank(bank)
        };

        let interconnect = Arc::new(Mutex::new(
                interconnect::Interconnect::new(ppu::Ppu::new(), rom)
        ));
        let cpu = match program_counter {
            None => cpu::Cpu::new(interconnect.clone()),
            Some(pc) => cpu::Cpu::new_with_pc(interconnect.clone(), pc.into()),
        };

        Nes {
            cpu,
            interconnect,
            rom_header: header,
        }
    }

    fn validate_header(rom: &[u8]) -> bool {
        // TODO Make this entire function better
        if rom.len() < 4 {
            return false;
        }

        let header: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a]; // NES^Z
        if header != &rom[0..4] {
            return false;
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
    fn rom_is_double_banked(rom: &[u8]) -> bool {
        if rom.len() < 16 {
            panic!("rom header is too small. expected 16 bytes, got #{}",
                   rom.len());
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

    pub fn step(&mut self) -> Result<StepInfo> {
        let intr = self.interconnect.lock().fetch_interrupt();
        let cycles = self.cpu.step(intr)?;

        let mut should_paint = false;
        {
            let mut interconnect = self.interconnect.lock();
            for _ in 0..cycles*3 {
                let result = interconnect.ppu.step();

                interconnect.update_interrupt(result.interrupt);
                if result.should_redraw {
                    should_paint = true;
                }
            }
        }

        Ok( StepInfo{
            should_paint,
            program_counter: self.cpu.PC,
        })
    }
}
