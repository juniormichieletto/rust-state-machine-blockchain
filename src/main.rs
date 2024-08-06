mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
    pub system: system::Pallet,
    pub balances: balances::Pallet,
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
    let _ = Runtime::new();
    println!("Okok");
}
