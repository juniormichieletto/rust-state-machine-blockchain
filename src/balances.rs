use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

// This is the Balances Module.
// It is a simple module which keeps track of how much balance each account has in this state
// machine.
#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    pub balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Balance not enough for the transfer");

        let to_new_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow to add balance");

        self.set_balance(&caller, new_caller_balance?);
        self.set_balance(&to, to_new_balance?);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use crate::balances::Pallet;
    mod types {
        pub type AccountId = String;
        pub type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut ballances = super::Pallet::<types::AccountId, types::Balance>::new();

        assert_eq!(ballances.balance(&String::from("alice")), 0);
        assert_eq!(ballances.balance(&String::from("bob")), 0);

        ballances.set_balance(&String::from("alice"), 100);

        assert_eq!(ballances.balance(&String::from("alice")), 100);
        assert_eq!(ballances.balance(&String::from("bob")), 0);
    }

    #[test]
    fn transfer_without_balance() {
        let mut ballances = super::Pallet::<types::AccountId, types::Balance>::new();

        let result = ballances.transfer(String::from("alice"), String::from("bob"), 100);

        assert!(result.is_err_and(|e| e == "Balance not enough for the transfer"));
    }

    #[test]
    fn transfer_balance() {
        let mut ballances = super::Pallet::<types::AccountId, types::Balance>::new();
        let transfer_result = ballances.transfer(String::from("alice"), String::from("bob"), 100);
        assert!(transfer_result.is_err_and(|e| e == "Balance not enough for the transfer"));

        ballances.set_balance(&String::from("alice"), 100);
        assert_eq!(ballances.balance(&String::from("alice")), 100);
        assert_eq!(ballances.balance(&String::from("bob")), 0);

        let transfer_result =
            ballances.transfer(String::from("alice"), String::from("bob").clone(), 100);
        assert!(transfer_result.is_ok());
        assert_eq!(ballances.balance(&String::from("alice")), 0);
        assert_eq!(ballances.balance(&String::from("bob")), 100);
    }
}
