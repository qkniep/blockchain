// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod block;
mod blockchain;
mod network;
mod node;
mod transaction;
mod wallet;

use std::error::Error;

use network::NetworkNode;
use node::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().into_iter().skip(1);
    let arg1 = args.next();
    let arg2 = args.next();

    let nn = NetworkNode::new(&arg1.unwrap());
    let mut node = Node::new(nn);

    if let Some(peer_addr) = arg2 {
        println!("Bootstrapping via {}.", peer_addr);
        node.bootstrap(&peer_addr).await?;
    }

    node.run().await?;

    Ok(())
}
