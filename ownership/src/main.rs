fn main() {
    println!("Ownership!");
    /*
     * accessing data in heap is slower than stack
    */

    let mut s = String::from("hello");

    println!("string is {}", s);
    s.push_str(", world");
    println!("string is {}", s);

    let x = 4;
    let mut y = x;

    y += 1;

    println!("value of x {}, y {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);

    let sb = String::from("hello");

    // let (sb2, len) = calculate_length(sb);
    let len = calculate_length(&sb);

    println!("The length of '{}', is {}", sb, len);

    let mut sc = String::from("hello");

    let r1 = &mut sc;
    // let r2 = &mut sc;

    // println!("{}, {}", r1, r2);

    change(&mut sc);

    println!("sc is {}", sc);


    let mut sd = String::from("hello world");
    let world = first_word(&s);

    s.clear();

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}