mod ballances;
use crate::ballances::Pallet;

fn main() {
    let pallet = Pallet { ballance: 100 };
    println!("Pallet 1, ballance:{}", pallet.ballance);
}
