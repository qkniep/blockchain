// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::net::SocketAddr;
use std::sync::Arc;

use rand::{thread_rng, Rng};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info, warn};

const MAX_MESSAGE_SIZE: usize = 8 * 1024 * 1024; // 8 MB
const MIN_PEERS: usize = 8;
const MAX_PEERS: usize = 16;

/// Shorthand for the transmit half of an mpsc channel.
type Tx = mpsc::UnboundedSender<String>;

/// Shorthand for the receive half of an mpsc channel.
type Rx = mpsc::UnboundedReceiver<String>;

#[derive(Clone)]
pub struct NetworkNode {
    address: SocketAddr,
    peers: Arc<Mutex<Vec<(SocketAddr, Tx)>>>,
}

impl NetworkNode {
    pub fn new(addr: &str) -> Self {
        Self {
            address: addr.parse().unwrap(),
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        info!("Listening to P2P connections on {}", self.address);
        let listener = TcpListener::bind(self.address).await?;

        while let Ok((stream, _)) = listener.accept().await {
            let mut nn = self.clone();

            tokio::spawn(async move {
                if let Err(e) = nn.handle_peer(stream).await {
                    error!("{:?}", e);
                }
            });
        }

        Ok(())
    }

    /// Handles communications with a single peer node.
    pub async fn handle_peer(&mut self, mut stream: TcpStream) -> Result<(), std::io::Error> {
        info!("Connected to peer: {:?}", stream.peer_addr().unwrap());

        let mut buf = Vec::with_capacity(MAX_MESSAGE_SIZE);
        let (tx, rx) = mpsc::unbounded_channel();
        let mut peers = self.peers.lock().await;
        peers.push((stream.peer_addr()?, tx.clone()));

        // Send them a random peer of us
        if peers.len() > 1 {
            let peer_i = thread_rng().gen_range(0, peers.len() - 1);
            let peer_addr = peers[peer_i].0;
            println!("sending {}", peer_addr);
            let l = peers.len();
            self.send_to(l - 1, format!("{}", peer_addr)).await;
        }

        drop(peers);

        loop {
            // TODO send what we receive through mpsc out over socket

            let n = match stream.read(&mut buf).await {
                // Socket closed
                Ok(0) => {
                    warn!("Disconnected from peer: {:?}", stream.peer_addr().unwrap());
                    return Ok(());
                }
                Ok(n) => n,
                Err(e) => {
                    error!("Failed to read from socket: {:?}", e);
                    return Err(e);
                }
            };

            println!("{}", std::str::from_utf8(&buf[0..n]).unwrap());

            // Write the data back
            //if let Err(e) = stream.write_all(&buf[0..n]).await {
            //    eprintln!("failed to write to socket; err = {:?}", e);
            //    return Err(e);
            //}
        }
    }

    /// Connects to a new peer node.
    pub async fn connect(&self, to: &str) -> Result<(), std::io::Error> {
        let stream = TcpStream::connect(to).await?;

        let mut nn = self.clone();

        tokio::spawn(async move {
            if let Err(e) = nn.handle_peer(stream).await {
                error!("{:?}", e);
            }
        });

        Ok(())
    }

    pub async fn broadcast(&self, msg: String) {
        for peer in self.peers.lock().await.iter_mut() {
            peer.1.send(msg.clone()).unwrap();
        }
    }

    pub async fn send_to(&self, receiver: usize, msg: String) {
        let peer = &self.peers.lock().await[receiver];
        peer.1.send(msg).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn basics() {
        let _n1 = NetworkNode::new("127.0.0.1:9901");
        let n2 = NetworkNode::new("127.0.0.1:9902");
        let n3 = NetworkNode::new("127.0.0.1:9903");

        std::thread::sleep(std::time::Duration::from_secs(3));

        aw!(n2.connect("127.0.0.1:9901")).expect("n2 failed to connect");
        aw!(n3.connect("127.0.0.1:9901")).expect("n3 failed to connect");
    }
}
