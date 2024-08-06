mod balances;
mod system;

use crate::system::Pallet;

fn main() {
    let _ = Pallet::new();
    println!("Okok");
}
