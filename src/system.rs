use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
    pub block_number: BlockNumber,
    pub nonce: BTreeMap<AccountID, Nonce>,
}

impl<AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce>
where
    AccountID: Ord + Clone,
    BlockNumber: Zero + One + Clone + CheckedAdd + CheckedSub + Copy + AddAssign,
    Nonce: Zero + One + Ord + Clone + Copy + Add,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number.clone()
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        //could crash overflow by purpose
        self.block_number += BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountID) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());

        self.nonce.insert(who.clone(), nonce + Nonce::one());
    }

    pub fn get_nonce(&self, who: &AccountID) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    #[test]
    fn block_number() {
        let pallet = super::Pallet::<String, u32, u32>::new();

        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut pallet = super::Pallet::<String, u32, u32>::new();

        pallet.inc_block_number();

        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let mut expected_nonce: BTreeMap<String, u32> = BTreeMap::new();
        expected_nonce.insert("aj".to_string(), 2);
        expected_nonce.insert("aj2".to_string(), 1);

        let mut pallet = super::Pallet::<String, u32, u32>::new();
        assert_eq!(pallet.nonce.len(), 0);

        pallet.inc_nonce(&"aj".to_string());
        pallet.inc_nonce(&"aj".to_string());
        pallet.inc_nonce(&"aj2".to_string());

        assert_eq!(pallet.nonce.len(), 2);
        assert_eq!(pallet.nonce, expected_nonce);
    }
}
