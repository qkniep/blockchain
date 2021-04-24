// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use blake3::Hash;

use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![Block::genesis()],
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
    pub fn checkpoint(&self) {
        let cp = Checkpoint {
            state: self.stateHash(),
        };
    }

    /// A hash that identifies the current state of the blockchain.
    pub fn stateHash(&self) -> Hash {
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
