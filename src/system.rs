use std::collections::BTreeMap;

type AccountID = String;
type BlockNumber = u128;
type Nonce = u32;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
    pub block_number: BlockNumber,
    pub nonce: BTreeMap<AccountID, Nonce>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        //could crash overflow by purpose
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountID) {
        let nonce = self.nonce.get(&who.to_string()).unwrap_or(&0);

        self.nonce.insert(who.to_string(), nonce + 1);
    }
}

#[cfg(test)]
mod tests {

    use crate::system::{AccountID, Nonce, Pallet};
    use std::collections::BTreeMap;

    #[test]
    fn block_number() {
        let pallet = Pallet::new();

        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut pallet = Pallet::new();

        pallet.inc_block_number();

        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let mut expected_nonce: BTreeMap<AccountID, Nonce> = BTreeMap::new();
        expected_nonce.insert("aj".to_string(), 2);
        expected_nonce.insert("aj2".to_string(), 1);

        let mut pallet = Pallet::new();
        assert_eq!(pallet.nonce.len(), 0);

        pallet.inc_nonce(&"aj".to_string());
        pallet.inc_nonce(&"aj".to_string());
        pallet.inc_nonce(&"aj2".to_string());

        assert_eq!(pallet.nonce.len(), 2);
        assert_eq!(pallet.nonce, expected_nonce);
    }
}
