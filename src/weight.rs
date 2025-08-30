use text_io::read;
use std::io::{self, Write};

pub fn run() {
    println!("1. Pounds -> Kilograms");
    println!("2. Kilograms -> Pounds");

    print!("> ");
    io::stdout().flush().unwrap();
    let choice_w: i32 = read!();

    if choice_w == 1 {
        print!("Enter the amount in Pounds: ");
        io::stdout().flush().unwrap();
        let pounds: f64 = read!();

        let kilograms = pounds / 2.20462;
        println!("Result: {:.5} kg", kilograms);
    } else if choice_w == 2 {
        print!("Enter the amount in Kilograms: ");
        io::stdout().flush().unwrap();
        let kilograms: f64 = read!();

        let pounds = kilograms * 2.20462;
        println!("Result: {:.5} lb", pounds);
    } else {
        eprintln!("Invalid choice!");
    }
}