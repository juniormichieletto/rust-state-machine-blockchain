mod balances;
mod system;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
    pub type AccountId = String;
    pub type Balance = u128;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    pub system: system::Pallet,
    pub balances: balances::Pallet<types::AccountId, types::Balance>,
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
    /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    let _res = runtime
        .balances
        .transfer("alice".to_string(), "bob".to_string(), 20)
        .map_err(|e| print!("Error to transfer from alice to bob, {}", e));

    assert_eq!(runtime.balances.balance(&"alice".to_string()), 50);
    assert_eq!(runtime.balances.balance(&"bob".to_string()), 50);

    println!("Okok {:#?}", runtime);
}
