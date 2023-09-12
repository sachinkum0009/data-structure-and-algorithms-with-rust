fn main() {
    let mut x = 5;
    
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y is in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let spaces = "    ";
    let spaces = spaces.len();

    let mut another_spaces = "     ";
    // another_spaces = another_spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");

    /*
     * Scalar types
     * 
     * Intergers
     * Floating Point Numbers
     * Booleans
     * Characters
     * 
     * numeric operations
     * addition
     * subtraction
     * multiplication
     * division
     * remainder
     * 
     * Compound Types
     * Tuples
     * Arrays
     * 
     */
    let tup: (i32, f64, u8) = (500, 5.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let first_val = tup.0;
    println!("The first value is: {first_val}");

    let a_arr = [1,2,3,4,5];
    let second_val = a_arr[1];
    let third_val = a_arr[2];

}
