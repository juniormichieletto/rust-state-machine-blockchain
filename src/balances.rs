use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

// This is the Balances Module.
// It is a simple module which keeps track of how much balance each account has in this state
// machine.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    pub balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
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

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
    Transfer {
        to: T::AccountId,
        amount: T::Balance,
    },
}

/// Implementation of the dispatch logic, mapping from `BalancesCall` to the appropriate underlying
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(caller, to, amount)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    struct TestConfig;

    impl super::Config for TestConfig {
        type Balance = u32;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_balances() {
        let mut ballances = super::Pallet::<TestConfig>::new();

        assert_eq!(ballances.balance(&String::from("alice")), 0);
        assert_eq!(ballances.balance(&String::from("bob")), 0);

        ballances.set_balance(&String::from("alice"), 100);

        assert_eq!(ballances.balance(&String::from("alice")), 100);
        assert_eq!(ballances.balance(&String::from("bob")), 0);
    }

    #[test]
    fn transfer_without_balance() {
        let mut ballances = super::Pallet::<TestConfig>::new();

        let result = ballances.transfer(String::from("alice"), String::from("bob"), 100);

        assert!(result.is_err_and(|e| e == "Balance not enough for the transfer"));
    }

    #[test]
    fn transfer_balance() {
        let mut ballances = super::Pallet::<TestConfig>::new();
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
