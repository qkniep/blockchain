// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::HashMap;

use ed25519_dalek::{Keypair, PublicKey};

use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::network::NetworkNode;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub struct Node {
    /// This node's local view of the blockchain.
    blockchain: Blockchain,
    wallet: Wallet,

    /// Keeps track of blocks which can not yet be added onto the blockchain.
    /// This can happen if we are behind and receive new blocks out of order.
    blocks: Vec<Block>,
    balances: HashMap<PublicKey, u64>,

    network: NetworkNode,
    validator: Option<Validator>,
}

impl Node {
    pub fn new(nn: NetworkNode) -> Self {
        Self {
            blockchain: Blockchain::new(),
            wallet: Wallet::new(),
            blocks: Vec::new(),
            balances: HashMap::new(),
            network: nn,
            validator: None,
        }
    }

    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        self.network.run().await
    }

    pub async fn bootstrap(&self, bootstrap_node: &str) -> Result<(), std::io::Error> {
        self.network.connect(bootstrap_node).await?;
        Ok(())
    }

    pub fn perform_transaction(&self, amount: u64, receiver: PublicKey) {
        let tx = Transaction::new(amount, receiver, receiver);
        // TODO send tx to peers
    }
}

pub struct Validator {
    transactions: Vec<Transaction>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
