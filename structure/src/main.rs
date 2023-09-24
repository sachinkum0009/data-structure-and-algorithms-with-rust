fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("John"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("john@another.com");

    let black = Color(244, 24, 30);
    
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color (u32, u32, u32);