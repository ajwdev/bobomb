#![feature(file_create_new)]
#![feature(result_option_inspect)]
#![feature(pattern)]

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use reqwest;
use tracing_subscriber::FmtSubscriber;

mod scraper;
mod types;
use crate::scraper::*;
pub use crate::types::*;

#[derive(ClapParser, Debug)]
struct Args {
    /// Hostname of the Bobomb instance to connect to
    #[arg(short, long, default_value_t = String::from("https://www.nesdev.org/obelisk-6502-guide/reference.html"))]
    url: String,

    /// File path to write JSON output to. A single "-" will send output to stdout.
    #[arg(short, long, default_value_t = String::from("-"))]
    output: String,

    /// Verbosity of logging.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Verbosity of logging.
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(match args.verbose {
            0 => tracing::Level::WARN,
            1 => tracing::Level::INFO,
            2 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        })
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let body = reqwest::blocking::get(args.url.clone())?.text()?;

    let mut docs = Scraper::new(&body).scrap()?;
    docs.generated_at = Some(chrono::Utc::now().to_rfc2822());
    docs.url = Some(args.url);

    if args.output == "-" {
        serde_json::to_writer_pretty(std::io::stdout(), &docs)?;
    } else {
        // TODO Implement force flag
        let w = std::fs::File::create_new(args.output.clone())
            .context(format!("unable to create file {}", &args.output))?;
        serde_json::to_writer_pretty(w, &docs)?;
    };

    Ok(())
}
