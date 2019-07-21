#![allow(dead_code)]
#[macro_use]
extern crate keyby;

use std::collections::hash_map::DefaultHasher;

#[key_by(id, price)]
pub struct Item {
    id: u64,
    price: u32,
    ts: u64,
}

fn main() {
    let i1 = Item {
        id: 1,
        price: 50,
        ts: 30,
    };
    let i2 = Item {
        id: 2,
        price: 150,
        ts: 150,
    };
    let i3 = Item {
        id: 1,
        price: 50,
        ts: 100,
    };

    assert_eq!(calc_hash(&i1), calc_hash(&i3));
    assert!(calc_hash(&i1) != calc_hash(&i2));
}

fn calc_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::Hasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
