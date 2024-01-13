// use crate::garden::vegetables::Asparagus;
// use crate::garden::fruits::Apple;

use crate::garden::{fruits::Apple, vegetables::Asparagus};

use std::collections::HashMap;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    let fruit = Apple{};
    fruit.call();

    let mut map = HashMap::new();
    map.insert(1, 2);

}
