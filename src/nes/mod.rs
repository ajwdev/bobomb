use minifb::{Key, Window, WindowOptions};
use parking_lot::{Mutex,Condvar};
use std::sync::Arc;
use std::thread;
use std::time::{Duration,Instant};

pub mod macros;
pub mod cpu;
pub mod ppu;
pub mod rom;
pub mod controller;
pub mod address;
pub mod interconnect;
pub mod debugger;

use bobomb_grpc::api_grpc;
use bobomb_grpc::grpc::ServerBuilder;


mod executor;
pub use crate::nes::executor::ExecutorLock;

pub struct Nes {
    cpu: Arc<Mutex<cpu::Cpu>>,
    interconnect: Arc<Mutex<interconnect::Interconnect>>,

    rom_header: [u8; 16],
    cycles: u32,

    window: Window,
}

impl Nes {
    pub fn new(rom_buffer: Vec<u8>) -> Nes {
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

        let ppu = ppu::Ppu::new();
        let interconnect = Arc::new(Mutex::new(interconnect::Interconnect::new(ppu, rom)));
        let cpu = Arc::new(Mutex::new(cpu::Cpu::new(interconnect.clone())));

        Nes {
            cpu,
            interconnect,
            window: Window::new("Bobomb", 256, 240, WindowOptions{
                title: true,
                resize: false,
                scale: minifb::Scale::X2,
               ..WindowOptions::default()
            }).unwrap(),

            rom_header: header,
            cycles: 0,
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

    pub fn start_emulation(&mut self) {
        let mut intr: Option<cpu::Interrupt> = None;
        let lock_pair: ExecutorLock = Arc::new((Mutex::new(true), Condvar::new()));
        let &(ref lock, ref cvar) = &*lock_pair;

        let shim = Arc::new(debugger::DebuggerShim::new(
            self.cpu.clone(),
            self.interconnect.clone(),
            lock_pair.clone()
        ));

        let service_def = api_grpc::BobombDebuggerServer::new_service_def(debugger::Server::new(shim.clone()));
        let mut server_builder = ServerBuilder::new_plain();
        server_builder.add_service(service_def);
        server_builder.http.set_addr("127.0.0.1:6502").unwrap();
        // TODO Not sure what to do here
        let _server = server_builder.build().expect("server builder fail");

        // Limit to max ~60 fps update rate
        // self.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        probe!(bobomb, start_emulation);
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            {
                let pc: u16 = self.cpu.lock().get_pc().into();
                if shim.is_breakpoint(pc) {
                    shim.stop_execution();
                }

                let mut running = lock.lock();
                if !*running {
                    // If we're here, the debugger has blocked us
                    println!("!!! Stopped by debugger ...");
                    cvar.wait(&mut running);
                }
            }

            let cycles = self.cpu.lock().step(intr);
            intr = None;

            let mut x = self.interconnect.lock();
            let ppu_cycles = cycles * 3;
            for _ in 0..ppu_cycles {
                // This feels gross
                let result = x.ppu.step();
                if let Some(x) = result.interrupt {
                    intr = Some(x);
                }
                if result.should_redraw {
                    self.window.update_with_buffer(
                        &x.ppu.front, 256, 240).unwrap();
                }
            }

            // We'll need to figure out the timings latter. For now, lets
            // not burn our cpu so much
            // let one_milli = Duration::from_millis(3);
            // thread::sleep(one_milli);
        }

        println!("FRAME COUNT {}", self.interconnect.lock().ppu.frames);
        println!("CYCLE {}", self.interconnect.lock().ppu.cycle);
        println!("SCAN {}", self.interconnect.lock().ppu.scanline);
    }
}
