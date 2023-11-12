struct MyStruct {
    a: u8,
    b: u8,
    c: u8
}

impl MyStruct {
    fn add(&self) -> u8 {
        self.a + self.b + self.c
    }

    fn mul(&self) -> u8 {
        self.a * self.b
    }
}

fn main() {
    println!("Welcome to Music Player");
    
    let my_struct = MyStruct{a: 10, b: 10, c: 10};
    
    println!("Value of a is {}", my_struct.a);
    println!("Addition is {}", my_struct.add());
    println!("Multition is {}", my_struct.mul());
}
