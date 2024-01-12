use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    println!("Todo CLI");
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(arg!(
            -c --config <FILE> "Sets a custom File"
        )
        .required(false)
        .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
            .about("doing a test")
            .arg(arg!(-l --list "list test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {name}");
    }
}
