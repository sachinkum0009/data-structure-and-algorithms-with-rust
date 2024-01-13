use std::io::{self, Read};
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::net::IpAddr;
use std::cmp::PartialOrd;
use learning_traits::{Summary, Tweet};



fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn main() {

    // let s = "ok";
    // let d = s.to_string();
    // println!("value of s is {} and d is {}", s, d);
    // println!("Hello, world!");

    // for c in "abc".bytes() {
    //     println!("{c}");
    // }

    // let hello = "Здравствуйте";

    // let s = &hello[0..4];

    // println!("the value of s is {}", s);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Red"), 20);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // println!("score of team {} is {}", team_name, score);

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // println!("{:?}", scores);


    // let text = "hello world worderful world";

    // let mut map2 = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map2.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map2);

    // panic!("crash and burn");
    // let v = vec![1,2,3];

    // v[99];


    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }

    // let greetings_file_result = File::open("hello.txt");

    // let greeting_file = match greetings_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", error);
    //         }
    //     },
    // };

    // let new_greeting_file = File::open("new_hello.txt")
    //     .expect("new_hello.txt file doesn't exit");


    // propagating errors



    // let home: IpAddr = "127.0.0.1"
    // .parse()
    // .expect("Hardcoded IP address should be valid");

    // println!("home ip is {}", home);

    // let my_new_file = File::open("my_new_file.txt").expect("not able to read the file");

    // let a = String::from("hello ok");
    // let guess = match a.trim().parse() {
    //     Ok(num) => num,
    //     Err(e) => 10,
    // };
    // println!("guess is {}", guess);

    let number_list = vec![30, 20, 50, 40, 65];

    // let mut largest_num = &number_list[0];

    // for number in &number_list {
    //     if number > largest_num {
    //         largest_num = number;
    //     }
    // }

    let result = largest_generic(&number_list);

    println!("largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];

    let result = largest_generic(&char_list);
    println!("The largest char is {}", result);

    // let integer = Point{x: 1, y: 2};
    // let float_struct     = Point{x: 1.2, y: 3.3};

    // println!("interger.x is {}", integer.x());

    // let wont_work = Point{x: 3, y:20.0};

    let p1 = Point { x: 5, y: 10.4};
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let interger = Some(5.0);

}


struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}


impl<X1, Y1> Point<X1, Y1> {
    // fn x(&self) -> &T {
    //     &self.x
    // }

    // fn dist_from_origin<T>(&self) -> <T> {
    //     self.x + self.y
    // }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}


enum Option<T> {
    Some(T),
    None,
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// generic function

fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


pub trait Summary {
    fn summarize(&self) -> String;
}