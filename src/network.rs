// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};

const MAX_PACKET_SIZE: usize = 8 * 1024 * 1024; // 8 MB

/// Shorthand for the transmit half of the mpsc channel.
type Tx = mpsc::UnboundedSender<String>;

/// Shorthand for the receive half of the mpsc channel.
type Rx = mpsc::UnboundedReceiver<String>;

#[derive(Clone)]
pub struct NetworkNode {
    address: SocketAddr,
    peers: Arc<Mutex<Vec<Tx>>>,
}

impl NetworkNode {
    // TODO don't panic
    pub fn new(addr: &str) -> Self {
        Self {
            address: addr.parse().unwrap(),
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.address).await?;

        while let Ok((stream, _)) = listener.accept().await {
            println!("connected to {:?}", stream);

            let mut nn = self.clone();

            tokio::spawn(async move {
                if let Err(e) = nn.handle_peer(stream).await {
                    eprintln!("{:?}", e);
                }

                println!("returned...");
            });
        }

        Ok(())
    }

    /// Handles communications with a single peer node.
    pub async fn handle_peer(&mut self, mut stream: TcpStream) -> Result<(), std::io::Error> {
        let mut buf = Vec::with_capacity(MAX_PACKET_SIZE);
        let (tx, rx) = mpsc::unbounded_channel();
        self.peers.lock().await.push(tx.clone());

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

    pub async fn broadcast(&self) {
        for peer in self.peers.lock().await.iter_mut() {
            peer.send("hello".to_owned());
        }
    }

    // TODO specify receiver
    pub async fn send_to(&self) {
        let p = &self.peers.lock().await[0];
        p.send("hello".to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
