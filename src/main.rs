// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod block;
mod blockchain;
mod network;
mod transaction;

use std::error::Error;

use pqcrypto::sign::falcon512;

use block::Block;
use blockchain::Blockchain;
use network::NetworkNode;
use transaction::Transaction;

pub struct Node {
    blockchain: Blockchain,
    wallet: Wallet,

    /// Keeps track of blocks which can not yet be added onto the blockchain.
    blocks: Vec<Block>,

    network: NetworkNode,
    minter: Option<Minter>,
}

impl Node {
    pub fn new(nn: NetworkNode) -> Self {
        let (pk, sk) = falcon512::keypair();
        Self {
            blockchain: Blockchain::new(),
            wallet: Wallet {
                keypairs: vec![(pk, sk)],
                funds: vec![0],
                unspent_outputs: Vec::new(),
            },
            blocks: Vec::new(),
            network: nn,
            minter: None,
        }
    }

    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        self.network.run().await
    }

    pub async fn bootstrap(&self, bootstrap_node: &str) -> Result<(), std::io::Error> {
        self.network.connect(bootstrap_node).await?;
        Ok(())
    }

    pub fn perform_transaction(&self, amount: usize, receiver: falcon512::PublicKey) {
        let outs = self.wallet.find_outputs_for_amount(amount);
        let tx = Transaction::new(amount, receiver);
        // TODO send tx to peers
    }
}

/// A collection of keypairs managing the associated funds.
pub struct Wallet {
    keypairs: Vec<(falcon512::PublicKey, falcon512::SecretKey)>,
    funds: Vec<usize>,
    unspent_outputs: Vec<usize>,
}

impl Wallet {
    /// Find unspent outputs that can be used for spending the requested amount.
    pub fn find_outputs_for_amount(&self, amount: usize) -> Option<Vec<usize>> {
        if amount > self.total_funds() {
            return None;
        }
        return Some(Vec::new());
    }

    pub fn sign_input(&self, pk: falcon512::PublicKey) -> falcon512::DetachedSignature {
        let sk = self.get_sk_for_pk(pk).unwrap();
        let msg = "Hello World";
        let sig = falcon512::detached_sign(msg.as_bytes(), &sk);
        return sig;
    }

    pub fn total_funds(&self) -> usize {
        self.funds.iter().sum()
    }

    fn get_sk_for_pk(&self, pk: falcon512::PublicKey) -> Option<falcon512::SecretKey> {
        for (p, s) in &self.keypairs {
            if *p == pk {
                return Some(s.clone());
            }
        }
        return None;
    }
}

pub struct Minter {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().into_iter().skip(1);
    let arg1 = args.next();
    let arg2 = args.next();

    let nn = NetworkNode::new(&arg1.unwrap());
    let mut node = Node::new(nn);

    if let Some(peer_addr) = arg2 {
        node.bootstrap(&peer_addr).await?;
    }

    node.run().await?;

    Ok(())
}
