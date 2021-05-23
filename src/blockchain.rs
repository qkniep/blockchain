// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use blake3::Hash;

use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    last_cp: Option<Checkpoint>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![Block::genesis()],
            last_cp: None,
        }
    }

    /// Validate the complete blockchain.
    pub fn validate(&self) -> bool {
        let mut prev = &self.chain[0];

        for b in self.chain[1..].iter() {
            if !b.validate(prev) {
                return false;
            }
            prev = b;
        }

        return true;
    }

    /// Create a checkpoint of the current blockchain state.
    pub fn checkpoint(&mut self) {
        let cp = Checkpoint {
            state: self.state_hash(),
        };
        self.last_cp = Some(cp);
    }

    /// A hash that identifies the current state of the blockchain.
    pub fn state_hash(&self) -> Hash {
        self.chain[self.chain.len() - 1].hash()
    }
}

struct Checkpoint {
    state: Hash,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_blockchain() {
        let mut blockchain = Blockchain::new();

        for i in 0..3 {
            let b_new = Block::mint(Vec::new(), &blockchain.chain[i]);
            blockchain.chain.push(b_new);
        }

        assert_eq!(blockchain.validate(), true);
    }
}
