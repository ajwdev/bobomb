use anyhow::{Context, Result};
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use std::fs;
use std::io::Read;

use bobomb::nes::executor::{Executor, ExitStatus};
use bobomb::nes::Nes;

#[derive(Parser, Debug)]
#[command(name = "bobomb")]
#[command(author = "Andrew Williams <me@ajw.dev>")]
struct Args {
    /// Start CPU emulation at given hex value instead of the reset vector. Only useful for debugging/tests
    #[arg(short, long, value_parser = parse_hex)]
    program_counter: Option<u16>,

    /// Wait for debugger to attach
    #[arg(short, long = "wait", default_value_t = false)]
    wait_for_attach: bool,

    /// Filename of the ROM
    rom: String,
}

fn parse_hex(s: &str) -> Result<u16> {
    use std::u16;
    let n = s.strip_prefix("0x").unwrap_or(s);
    u16::from_str_radix(&n, 16).context("could not parse hex program counter")
}

fn main() -> Result<()> {
    let mut opts = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut buf = Vec::new();
    {
        let mut file = fs::File::open(opts.rom)?;
        file.read_to_end(&mut buf).unwrap();
    }

    info!("Starting nes emulation");

    loop {
        let nes = Nes::new(&buf, opts.program_counter);
        let executor = Executor::new(nes, opts.wait_for_attach)?;

        match executor.run() {
            Ok(exitstatus) => match exitstatus {
                ExitStatus::Restart(new_pc) => {
                    println!("Restart reqested");
                    if new_pc.is_some() {
                        opts.program_counter = new_pc
                    }
                }
                ExitStatus::Success => return Ok(()),
            },
            Err(why) => return Err(why),
        }
    }
}
