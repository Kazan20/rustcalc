use text_io::read;
use std::io::{self, Write};

fn main() {
    let choice: i32;

    println!("**********************");
    println!("*** Unit Converter ***");
    println!("**********************");

    println!("1. Temperature");
    println!("2. Weight");
    println!("3. TODO"); // TODO: make option

    print!("> ");
    choice = read!();
    io::stdout().flush().expect("Failed to flush stdout");

    match choice {
        1 => {
            let fahrenheit: f64;
            let celsius: f64;
            let choice_t: i32;

            println!("1. Fahrenheit -> Celsius. 2. Celsius -> Fahrenheit");
            print!("> ");
            choice_t = read!();

            if choice_t == 1 {
                print!("Enter the amount in Fahrenheit: ");
                fahrenheit = read!();
                io::stdout().flush().expect("Failed to flush stdout");

                celsius = (fahrenheit - 32.0) / 1.8;
                println!("Result: {}", celsius)
            } else if choice_t == 2 {
                print!("Enter the amount in Celsius: ");
                celsius = read!();
                io::stdout().flush().expect("Failed to flush stdout");

                fahrenheit = (celsius * 1.8) + 32.0;
                println!("Result: {}", fahrenheit);
            }
        }
        2 => {
            let pounds: f64;
            let kilograms: f64;
            let choice_w: i32;
            println!("1. Pounds -> Kilograms. 2. Kilograms -> Pounds");

            print!("> ");
            choice_w = read!();

            if choice_w == 1 {
                print!("Enter the amount in Pounds: ");
                pounds = read!();
                io::stdout().flush().expect("Failed to flush stdout");

                kilograms = pounds / 2.20462;
                println!("Result: {}", kilograms)
            } else if choice_w == 2 {
                print!("Enter the amount in Kilograms: ");
                kilograms = read!();
                io::stdout().flush().expect("Failed to flush stdout");

                pounds = kilograms * 2.20462;
                println!("Result: {}", pounds);
            }
        }
        _ => println!("Invalid Choice!"),
    }
}