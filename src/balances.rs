use std::collections::BTreeMap;

pub struct Pallet {
    pub balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.to_string(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use crate::balances::Pallet;

    #[test]
    fn init_balances() {
        let alice: &String = &"alice".to_string();
        let bob: &String = &"bob".to_string();

        let mut ballances = Pallet::new();

        assert_eq!(ballances.balance(alice), 0);
        assert_eq!(ballances.balance(bob), 0);

        ballances.set_balance(alice, 100);

        assert_eq!(ballances.balance(alice), 100);
        assert_eq!(ballances.balance(bob), 0);
    }
}
