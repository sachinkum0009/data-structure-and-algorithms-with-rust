use std::io;


struct Calculator {
    a : i32
}

impl Calculator {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
}
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("call method");
    }
}

fn main() {
    let obj = Calculator{
        a: 10
    };
    let c = obj.add(10, 20);
    println!("value is {},  {}", obj.a, c);
    println!("Hello, world!");

    let m = Message::Write(String::from("hello"));
    m.call();

}


