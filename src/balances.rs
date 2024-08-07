use std::collections::BTreeMap;

type AccountId = String;
type Balance = u128;

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    pub balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: u128) {
        self.balances.insert(who.to_string(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Balance not enough for the transfer");

        let to_new_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow to add balance");

        self.set_balance(&caller, new_caller_balance?);
        self.set_balance(&to, to_new_balance?);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::balances::{AccountId, Pallet};

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

    #[test]
    fn transfer_without_balance() {
        let alice: AccountId = "alice".to_string();
        let bob: AccountId = "bob".to_string();
        let mut ballances = Pallet::new();

        let result = ballances.transfer(alice, bob, 100);

        assert!(result.is_err_and(|e| e == "Balance not enough for the transfer"));
    }

    #[test]
    fn transfer_balance() {
        let alice: AccountId = "alice".to_string();
        let bob: AccountId = "bob".to_string();

        let mut ballances = Pallet::new();
        let transfer_result = ballances.transfer(alice.clone(), bob.clone(), 100);
        assert!(transfer_result.is_err_and(|e| e == "Balance not enough for the transfer"));

        ballances.set_balance(&alice, 100);
        assert_eq!(ballances.balance(&alice), 100);
        assert_eq!(ballances.balance(&bob), 0);

        let transfer_result = ballances.transfer(alice.clone(), bob.clone(), 100);
        assert!(transfer_result.is_ok());
        assert_eq!(ballances.balance(&alice), 0);
        assert_eq!(ballances.balance(&bob), 100);
    }
}
