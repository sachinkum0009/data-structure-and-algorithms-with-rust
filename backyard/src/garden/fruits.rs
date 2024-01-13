#[derive(Debug)]
pub struct Banana {}
pub struct Apple {}

impl Apple {
    pub fn call(&self) {
        self.internal_call();
    }
    fn internal_call(&self) {
        println!("calling apple");
    }
}