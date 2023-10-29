mod client;
mod ctrl_c;
mod repl;
pub use repl::Repl;

use clap::Parser;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(name = "bbdb")]
#[command(author = "Andrew Williams <me@ajw.dev>")]
struct Args {
    /// Hostname of the Bobomb instance to connect to
    #[arg(short = 'H', long, default_value_t = String::from("127.0.0.1"))]
    host: String,

    /// Port of the Bobomb instance to connect to
    #[arg(short, long, default_value_t = 6502)]
    port: u16,

    /// Whether to print API responses. Only useful for debugging.
    #[arg(long, default_value_t = false)]
    debug_requests: bool,
}

#[tokio::main]
async fn main() {
    let opts = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let url = format!("https://{}:{}", opts.host, opts.port);
    let mut cli = Repl::new(&url, opts.debug_requests).unwrap();
    cli.run();
}
