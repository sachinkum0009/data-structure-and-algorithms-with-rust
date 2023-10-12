struct AlwaysEqual;


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("John"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("john@another.com");

    let black = Color(244, 24, 30);

    let subject = AlwaysEqual;

    // let width1 = 30;
    // let height1 = 50;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
        // area(width1, height1)
    );

    // println!("rect1 is {:?}", rect1);
    println!("area of rect1 is {} pixels", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color (u32, u32, u32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}