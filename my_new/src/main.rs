/**
 * enums in rust
 */


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
/// does something
fn do_something() -> i32 {
    16
}


fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Lucy Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let a = do_something();

    println!("value of a is {}", a);

    let my_coin = Coin::Quarter;
    let my_cents = value_in_cents(my_coin);
    println!("value in cents is {}", my_cents);
}
