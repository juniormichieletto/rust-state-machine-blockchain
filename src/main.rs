mod balances;
mod system;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
    pub type AccountId = String;
    pub type BlockNumber = u128;
    pub type Nonce = u32;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    pub system: system::Pallet<Self>,
    pub balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type AccountId = String;
    type Balance = u32;
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();

    runtime.balances.set_balance(&alice, 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);

    let _res = runtime
        .balances
        .transfer(alice, bob, 30)
        .map_err(|e| print!("Error to transfer from alice to bob, {}", e));

    runtime.system.inc_nonce(&"alice".to_string());

    let _res = runtime
        .balances
        .transfer("alice".to_string(), "bob".to_string(), 20)
        .map_err(|e| print!("Error to transfer from alice to bob, {}", e));

    assert_eq!(runtime.balances.balance(&"alice".to_string()), 50);
    assert_eq!(runtime.balances.balance(&"bob".to_string()), 50);

    println!("Okok {:#?}", runtime);
}
