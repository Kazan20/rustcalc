mod weight;
mod temp;
use std::io::{self, Write};
use text_io::read;

fn main() {
    println!("**********************");
    println!("*** Unit Converter ***");
    println!("**********************");

    println!("0. Exit");
    println!("1. Temperature");
    println!("2. Weight");

    print!("> ");
    io::stdout().flush().unwrap();
    let choice: i32 = read!();

    match choice {
        1 => temp::run(),
        2 => weight::run(),
        0 => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => println!("Invalid Choice!"),
    }
}
