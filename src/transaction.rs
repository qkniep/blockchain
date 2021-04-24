// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::fmt::{self, Debug};

use pqcrypto::sign::*;

#[derive(Clone, Debug)]
pub struct Transaction {
    id: usize,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
}

impl Transaction {
    // TODO generate ID
    // TODO take inputs, calculate their total, calculate correct change
    pub fn new(amount: usize, to: falcon512::PublicKey) -> Self {
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
            inputs: Vec::new(),
            outputs: vec![spending, change],
        }
    }

    pub fn validate(&self) -> bool {
        self.total_input_amount() >= self.total_output_amount()
    }

    // TODO calculate
    fn total_input_amount(&self) -> usize {
        0
    }

    fn total_output_amount(&self) -> usize {
        self.outputs.iter().map(|o| o.amount).sum()
    }
}

#[derive(Clone)]
struct TxIn {
    tx_out_id: usize,
    tx_out_block: usize,
    signature: falcon512::DetachedSignature,
}

impl Debug for TxIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TxIn")
            .field("tx_out_id", &self.tx_out_id)
            .field("tx_out_block", &self.tx_out_block)
            .finish()
    }
}

#[derive(Clone)]
struct TxOut {
    address: falcon512::PublicKey,
    amount: usize,
}

impl Debug for TxOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TxOut")
            .field("amount", &self.amount)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {}
}
