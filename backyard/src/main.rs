use crate::garden::vegetables::Asparagus;
use crate::garden::fruits::Apple;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    let fruit = Apple{};
    fruit.call();

}
