// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use ed25519_dalek::{Keypair, PublicKey};
use rand::rngs::OsRng;

use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::network::NetworkNode;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub struct Node {
    blockchain: Blockchain,
    wallet: Wallet,

    /// Keeps track of blocks which can not yet be added onto the blockchain.
    /// This can happen if we are behind and receive new blocks out of order.
    blocks: Vec<Block>,

    network: NetworkNode,
    minter: Option<Minter>,
}

impl Node {
    pub fn new(nn: NetworkNode) -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        Self {
            blockchain: Blockchain::new(),
            wallet: Wallet::new(keypair),
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

    pub fn perform_transaction(&self, amount: u64, receiver: PublicKey) {
        let outs = self.wallet.find_outputs_for_amount(amount);
        let tx = Transaction::new(amount, receiver, receiver);
        // TODO send tx to peers
    }
}

pub struct Minter {
    transactions: Vec<Transaction>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
