extern crate tokio;

mod tunnel;

use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let bind_addr = "127.0.0.1:8080".parse().unwrap();
    let remote_addr = "127.0.0.1:8089".parse().unwrap();
    let tun = tunnel::XTunnel {
        bind_addr: bind_addr,
        remote_addr: remote_addr,
    };
    tun.run().await
}
