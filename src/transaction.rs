// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use ed25519_dalek::{PublicKey, Signature};

#[derive(Clone, Debug)]
pub struct Transaction {
    id: u64,
    from: PublicKey,
    to: PublicKey,
    amount: u64,
    signature: Option<Signature>,
}

impl Transaction {
    // TODO support different tx types
    // TODO generate ID
    // TODO sign transaction?
    pub fn new(amount: u64, from: PublicKey, to: PublicKey) -> Self {
        Self {
            id: 0,
            from,
            to,
            amount,
            signature: None,
        }
    }

    /// Checks whether the transaction, including all signatures, is valid.
    pub fn validate(&self) -> bool {
        if let Some(sig) = self.signature {
            return self.from.verify_strict(b"", &sig).is_err();
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {}
}
