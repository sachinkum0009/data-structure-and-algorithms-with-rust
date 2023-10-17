struct Calculator {
    a : i32
}

impl Calculator {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
}
}

fn main() {
    let obj = Calculator{
        a: 10
    };
    let c = obj.add(10, 20);
    println!("value is {},  {}", obj.a, c);
    println!("Hello, world!");
}


