fn main() {
    println!("Hello, world!");
    hello_func(10, 'c');

    let y = {
        let x = 5;
        x + 1
    };

    println!("The value of y is {y}");

    
}

fn hello_func(x: i32, unit_label: char) {
    println!("Hello Function {x}{unit_label}!");
    let a = plus_one(3);
    let j = five();
    println!("value of j is {j}");
    println!("value of a is : {a}");
    if a < 5 {
        println!("condition is true");
    }
    else {
        println!("condition is false");
    }
}

/// returns 5
fn five() -> i32 {
    5
}

/**
* fn to plus one
* @param x: i32
*/
fn plus_one(x: i32) -> i32 {
    // fn to plus 1
    x + 1
}


