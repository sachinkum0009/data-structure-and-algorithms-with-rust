use std::io;



fn main() {
    println!("Welcome to Collections");
    // let mut v: Vec<i32> = Vec::new();
    // let v2 = vec![1,2,3,4];
    // v.push(3);
    // v.push(4);
    // v.push(5);

    // // reading elements of vector
    // let third: &i32 = &v[2];
    // println!("third value is {}", third);

    // let second: Option<&i32> = v.get(1);
    // match second {
    //     Some(second) => println!("The second element is {}", second),
    //     None => println!("There is no second value"),
    // }

    let mut v = vec![1,2,3,4];
    let first = &v[0];
    println!("The first element is: {first}");
    v.push(9);

    // iterate over the values in vector
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    enum SpreadSheetCell {
            Int(i32),
            Float(f64),
            Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(10),
        SpreadSheetCell::Float(10.23),
        SpreadSheetCell::Text(String::from("blue"))
    ];

    let value = row.get(2);
    // println!("value is {}", value);
    // need to implement a function a display

    v.clear();

    

}
