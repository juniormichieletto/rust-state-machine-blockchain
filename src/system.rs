use num::traits::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub block_number: T::BlockNumber,
    pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        //could crash overflow by purpose
        self.block_number += T::BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());

        self.nonce.insert(who.clone(), nonce + T::Nonce::one());
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn block_number() {
        let pallet = super::Pallet::<TestConfig>::new();

        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut pallet = super::Pallet::<TestConfig>::new();

        pallet.inc_block_number();

        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let mut expected_nonce: BTreeMap<String, u32> = BTreeMap::new();
        expected_nonce.insert("aj".to_string(), 2);
        expected_nonce.insert("aj2".to_string(), 1);

        let mut pallet = super::Pallet::<TestConfig>::new();
        assert_eq!(pallet.nonce.len(), 0);

        pallet.inc_nonce(&"aj".to_string());
        pallet.inc_nonce(&"aj".to_string());
        pallet.inc_nonce(&"aj2".to_string());

        assert_eq!(pallet.nonce.len(), 2);
        assert_eq!(pallet.nonce, expected_nonce);
    }
}
