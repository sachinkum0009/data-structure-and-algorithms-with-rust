#[derive(Debug)]
pub struct Banana {}
pub struct Apple {}

impl Apple {
    pub fn call(&self) {
        println!("calling apple");
    }
}