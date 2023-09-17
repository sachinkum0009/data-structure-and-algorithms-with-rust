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

}
