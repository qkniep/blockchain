// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const MAX_PACKET_SIZE: usize = 8 * 1024 * 1024; // 8 MB

pub struct NetworkNode {}

impl NetworkNode {
    pub async fn run() -> Result<(), std::io::Error> {
        let addr = "127.0.0.1:9000";
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;

            println!("connected to {:?}", stream);

            tokio::spawn(async move {
                if let Err(e) = NetworkNode::handle_peer(stream).await {
                    eprintln!("{:?}", e);
                }

                println!("returned...");
            });
        }
    }

    pub async fn handle_peer(mut stream: TcpStream) -> Result<(), std::io::Error> {
        let mut buf = Vec::with_capacity(MAX_PACKET_SIZE);

        loop {
            let n = match stream.read(&mut buf).await {
                // socket closed
                Ok(0) => return Ok(()),
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return Err(e);
                }
            };

            // Write the data back
            if let Err(e) = stream.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
