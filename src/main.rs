mod balances;
use crate::balances::Pallet;

fn main() {
    let mut pallet = Pallet::new();
    pallet.balances.insert("123".to_string(), 100);
    pallet.balances.insert("321".to_string(), 50);

    println!("Pallet, nr of ballances: {}", pallet.balances.len());
}
