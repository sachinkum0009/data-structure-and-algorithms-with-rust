fn main() {
    println!("Control Flow Cargo Package");
    let x : i32 = 10;
    if x < 30 {
        println!("x is less than 30");
    }
    else {
        println!("x is greater than 30");
    }
    let condition = true;
    let number = if condition {10} else {40};
    println!("number is {number}");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1
    }
    println!("End count = {count}");
    // loop {
        
    // }
}
