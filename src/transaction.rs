// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use ed25519_dalek::{PublicKey, Signature};

#[derive(Clone, Debug)]
pub struct Transaction {
    id: u64,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
}

impl Transaction {
    // TODO generate ID
    // TODO take inputs, calculate their total, calculate correct change
    // TODO sign transaction?
    pub fn new(amount: u64, from: PublicKey, to: PublicKey) -> Self {
        //let input = TxIn { address: from };
        let spending = TxOut {
            address: to,
            amount,
        };
        let change = TxOut {
            address: to,
            amount,
        };
        Self {
            id: 0,
            inputs: vec![/*input*/],
            outputs: vec![spending, change],
        }
    }

    /// Checks whether the transaction, including all signatures, is valid.
    pub fn validate(&self) -> bool {
        for input in &self.inputs {
            if !input.address.verify_strict(b"", &input.signature).is_err() {
                return false;
            }
        }
        self.total_input_amount() >= self.total_output_amount()
    }

    // TODO calculate
    fn total_input_amount(&self) -> u64 {
        0
    }

    fn total_output_amount(&self) -> u64 {
        self.outputs.iter().map(|o| o.amount).sum()
    }
}

#[derive(Clone, Debug)]
struct TxIn {
    address: PublicKey,
    signature: Signature,
}

#[derive(Clone, Debug)]
struct TxOut {
    address: PublicKey,
    amount: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {}
}
