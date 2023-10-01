use std::io;

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
    println!(" color value is {}", black.0);

    let ip = IpAddrKind::V4;

    // if ip == IpAddrKind::V4 {
    //     println!("Ip Addr is V4");
    // }
    let machine_type = MachineType::active;
}

enum IpAddrKind {
    V4,
    V6,
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