extern crate tokio;

mod tunnel;

use clap::Parser;
use tokio::io;

/// Simple TCP relay written in rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address to listen
    #[arg(short, long)]
    listen: String,

    /// Target address
    #[arg(short, long)]
    target: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let bind_addr = args.listen.parse().expect("Invalid listen address");
    let remote_addr = args.target.parse().expect("Invalid target address");
    let tun = tunnel::XTunnel {
        bind_addr: bind_addr,
        remote_addr: remote_addr,
    };
    tun.run().await
}
