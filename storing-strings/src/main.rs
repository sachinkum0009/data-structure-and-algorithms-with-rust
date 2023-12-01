use std::io::{self, Read};
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::net::IpAddr;




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


    enum Result<T, E> {
        Ok(T),
        Err(E)
    }

    let greetings_file_result = File::open("hello.txt");

    let greeting_file = match greetings_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    // let new_greeting_file = File::open("new_hello.txt")
    //     .expect("new_hello.txt file doesn't exit");


    // propagating errors



    let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");

    println!("home ip is {}", home);

}
