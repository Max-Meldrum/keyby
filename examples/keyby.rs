#[macro_use]
extern crate keyby;

use std::collections::hash_map::DefaultHasher;

#[key_by(id)]
pub struct Item {
    id: u64,
    price: u32,
}

fn main() {
    let i1 = Item { id: 1, price: 20 };
    let i2 = Item { id: 2, price: 150 };
    let i3 = Item { id: 1, price: 50 };

    assert_eq!(calc_hash(&i1), calc_hash(&i3));
    assert!(calc_hash(&i1) != calc_hash(&i2));
}

fn calc_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::Hasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
