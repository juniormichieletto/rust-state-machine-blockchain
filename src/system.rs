use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
    pub block_number: u32,
    pub nonce: BTreeMap<String, u32>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }
}
