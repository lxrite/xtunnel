use std::net::SocketAddr;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct XTunnel {
    pub bind_addr: SocketAddr,
    pub remote_addr: SocketAddr,
}

impl XTunnel {
    pub async fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.bind_addr).await?;
        let remote_addr = self.remote_addr;
        while let Ok((mut client_socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut server_socket =
                    if let Ok(server_socket) = TcpStream::connect(&remote_addr).await {
                        server_socket
                    } else {
                        return;
                    };
                let (mut client_reader, mut client_writer) = client_socket.split();
                let (mut server_reader, mut server_writer) = server_socket.split();
                let client_to_server = async {
                    io::copy(&mut client_reader, &mut server_writer).await?;
                    server_writer.shutdown().await
                };
                let server_to_client = async {
                    io::copy(&mut server_reader, &mut client_writer).await?;
                    client_writer.shutdown().await
                };
                _ = tokio::try_join!(client_to_server, server_to_client);
            });
        }
        Ok(())
    }
}
