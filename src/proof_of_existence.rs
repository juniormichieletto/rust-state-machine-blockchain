use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    /// Get the owner (if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    /// Create a new claim on behalf of the `caller`.
    /// This function will return an error if someone already has claimed that content.
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err(&"Claim already exists"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    /// Revoke an existing claim on some content.
    /// This function should only succeed if the caller is the owner of an existing claim.
    /// It will return an error if the claim does not exist, or if the caller is not the owner.
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("Claim does not exists")?;

        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    //TODO split the test in different tests
    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();

        let _ = poe.create_claim("alice".to_string(), &"my_document");
        assert_eq!(poe.get_claim(&"my_document"), Some(&"alice".to_string()));

        let res = poe.revoke_claim("bob".to_string(), &"my_document");
        assert_eq!(res, Err("Caller is not the owner of the claim"));

        let res = poe.create_claim("alice".to_string(), &"my_document");
        assert_eq!(res, Err("Claim already exists"));

        let res = poe.revoke_claim("alice".to_string(), &"non existent document");
        assert_eq!(res, Err("Claim does not exists"));

        let res = poe.revoke_claim("alice".to_string(), &"my_document");
        assert_eq!(res, Ok(()));
        assert_eq!(poe.get_claim(&"my_document"), None);
    }
}
