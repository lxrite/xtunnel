extern crate futures;
extern crate tokio_core;

mod tunnel;

use tunnel::XTunnel;

fn main() {
    let bind_addr = "127.0.0.1:8080".parse().unwrap();
    let remote_addr = "127.0.0.1:8089".parse().unwrap();
    let tun = XTunnel {
        bind_addr: bind_addr,
        remote_addr: remote_addr,
    };
    tun.run().unwrap();
}
