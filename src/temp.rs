use text_io::read;
use std::io::{self, Write};

pub fn run() {
    println!("1. Fahrenheit -> Celsius");
    println!("2. Celsius -> Fahrenheit");

    print!("> ");
    io::stdout().flush().unwrap();
    let choice_t: i32 = read!();

    if choice_t == 1 {
        print!("Enter the amount in Fahrenheit: ");
        io::stdout().flush().unwrap();
        let fahrenheit: f64 = read!();

        let celsius = (fahrenheit - 32.0) / 1.8;
        println!("Result: {:.5} °C", celsius);
    } else if choice_t == 2 {
        print!("Enter the amount in Celsius: ");
        io::stdout().flush().unwrap();
        let celsius: f64 = read!();

        let fahrenheit = (celsius * 1.8) + 32.0;
        println!("Result: {:.5} °F", fahrenheit);
    } else {
        println!("Invalid choice!");
    }
}