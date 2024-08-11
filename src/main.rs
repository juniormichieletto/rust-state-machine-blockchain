mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::Dispatch;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u128;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    pub system: system::Pallet<Self>,
    pub balances: balances::Pallet<Self>,
    pub proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

fn main() {
    // Create a new instance of the Runtime.
    // It will instantiate with it all the modules it uses.
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    // Initialize the system with some initial balance.
    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 20,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: charlie,
                    amount: 30,
                }),
            },
        ],
    };

    runtime
        .execute_block(block_1)
        .expect("wrong block execution");

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "my_document",
                }),
            },
            support::Extrinsic {
                caller: bob,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Bobs Doc",
                }),
            },
        ],
    };

    runtime
        .execute_block(block_2)
        .expect("wrong block execution");

    // Simply print the debug format of our runtime state.
    println!("{:#?}", runtime);
}
