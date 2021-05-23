// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};
use rand::rngs::OsRng;

/// A collection of keypairs managing the associated funds.
pub struct Wallet {
    keypairs: Vec<Keypair>,
    funds: Vec<u64>,
}

// TODO persist wallet to disk

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);

        Self {
            keypairs: vec![keypair],
            funds: vec![0],
        }
    }

    pub fn sign_input(&self, pk: PublicKey) -> Signature {
        let kp = self.get_keypair_for_pk(pk).unwrap();
        let msg = "Hello World";
        let sig = kp.sign(msg.as_bytes());
        return sig;
    }

    pub fn total_funds(&self) -> u64 {
        self.funds.iter().sum()
    }

    fn get_keypair_for_pk(&self, pk: PublicKey) -> Option<&Keypair> {
        for kp in &self.keypairs {
            if kp.public == pk {
                return Some(kp);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
