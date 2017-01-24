use std::io;
use std::net::SocketAddr;
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::io::{copy, Io};

pub struct XTunnel {
    pub bind_addr: SocketAddr,
    pub remote_addr: SocketAddr,
}

impl XTunnel {
    pub fn run(&self) -> io::Result<()>{
        let mut core = try!(Core::new());
        let handle = core.handle();
        let listener = try!(TcpListener::bind(&self.bind_addr, &handle));
        let server = listener.incoming().for_each(|(client_socket, _)| {
            let handle_clone = handle.clone();
            let trans = TcpStream::connect(&self.remote_addr, &handle).map(move |server_socket| {
                let (client_reader, client_writer) = client_socket.split();
                let (server_reader, server_writer) = server_socket.split();
                handle_clone.spawn(copy(client_reader, server_writer).join(copy(server_reader, client_writer)).map(|_| ()).map_err(|_| ()));
            });
            handle.spawn(trans.map(|_| ()).map_err(|_| ()));
            Ok(())
        });
        try!(core.run(server));
        Ok(())
    }
}
