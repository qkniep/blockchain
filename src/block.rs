// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::thread::sleep;
use std::time::Duration;

use blake3::Hash;

use crate::transaction::Transaction;

const PROTOCOL_VERSION: u32 = 1;
const BLOCK_GENERATION_INTERVAL: Duration = Duration::from_secs(10);
const INITIAL_DIFFICULTY: usize = 200;

///
#[derive(Clone, Debug)]
pub struct Block {
    id: usize,
    version: u32,
    timestamp: usize,
    prev_hash: Hash,
    transactions: Vec<Transaction>,
    difficulty: usize,
    minter: usize,
}

impl Block {
    pub fn genesis() -> Self {
        Self {
            id: 0,
            version: 0,
            timestamp: 0,
            prev_hash: [0; blake3::OUT_LEN].into(),
            transactions: Vec::new(),
            difficulty: INITIAL_DIFFICULTY,
            minter: 0,
        }
    }

    pub fn next(&self) -> Self {
        Self {
            id: self.id + 1,
            version: PROTOCOL_VERSION,
            timestamp: 0,
            prev_hash: self.hash(),
            transactions: Vec::new(),
            // TODO adjust difficulty
            difficulty: INITIAL_DIFFICULTY,
            // TODO set minter
            minter: 0,
        }
    }

    pub fn mint(txs: Vec<Transaction>, prev: &Block) -> Self {
        let mut b = prev.next();
        loop {
            let hash = b.hash();
            let mut d = b.difficulty;
            for h in hash.as_bytes() {
                if (d >= 256 && *h != 0) || (d < 256 && *h < d as u8) {
                    break;
                } else if d < 256 {
                    return b;
                }
                d /= 256;
            }
            b.timestamp += 1;
            sleep(Duration::from_secs(1));
        }
    }

    // TODO validate timestamp
    // TODO validate adjusted difficulty
    pub fn validate(&self, prev: &Block) -> bool {
        self.id == prev.id + 1 && self.prev_hash == prev.hash()
    }

    pub fn hash(&self) -> Hash {
        return blake3::hash(self.as_bytes());
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                ::std::mem::size_of::<Self>(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minting() {
        let b1 = Block::genesis();
        let _b2 = Block::mint(Vec::new(), &b1);
    }

    #[test]
    fn validation() {
        let b1 = Block::genesis();
        let b2 = Block::mint(Vec::new(), &b1);
        assert_eq!(b2.validate(&b1), true);
        let b3 = Block::mint(Vec::new(), &b2);
        assert_eq!(b3.validate(&b2), true);
        let b4 = Block::mint(Vec::new(), &b3);
        assert_eq!(b4.validate(&b3), true);

        // Order matters
        assert_eq!(b4.validate(&b2), false);
        assert_eq!(b3.validate(&b4), false);
        assert_eq!(b2.validate(&b2), false);
    }

    #[test]
    fn hash() {
        let mut b = Block::genesis();
        let mut h = b.hash();

        // Hash depends on id
        b.id += 42;
        assert_ne!(b.hash(), h);
        h = b.hash();

        // Hash depends on version
        b.version += 1;
        assert_ne!(b.hash(), h);
        h = b.hash();

        // Hash depends on timestamp
        b.timestamp += 7;
        assert_ne!(b.hash(), h);
        h = b.hash();
    }
}
