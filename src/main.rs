// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod block;
mod blockchain;
mod network;
mod node;
mod transaction;
mod wallet;

use std::error::Error;

use tracing::{info, subscriber::set_global_default};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use network::NetworkNode;
use node::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logging();

    let mut args = std::env::args().into_iter().skip(1);
    let arg1 = args.next();
    let arg2 = args.next();

    let nn = NetworkNode::new(&arg1.unwrap());
    let mut node = Node::new(nn);

    if let Some(peer_addr) = arg2 {
        info!("Bootstrapping via {}", peer_addr);
        node.bootstrap(&peer_addr).await?;
    }

    node.run().await?;

    info!("Blockchain stopped");

    Ok(())
}

fn setup_logging() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::Layer::default().with_target(false);
    let subscriber = Registry::default().with(env_filter).with(fmt_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
}
