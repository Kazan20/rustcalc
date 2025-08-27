use std::io::{self, Write};
use text_io::read;

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let num1: f64 = read!();

    print!("Enter an operator (+, -, *, /): ");
    io::stdout().flush().unwrap();
    let op: char = read!();

    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let num2: f64 = read!();

    let result: f64 = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => {
            let res = num1 * num2;
            println!("{}", res);
            print!("Times result by PI? (y/n): ");
            io::stdout().flush().unwrap();
            let time_pi: char = read!();

            match time_pi {
                'y' | 'Y' => res * std::f64::consts::PI,
                _ => res,
            }
        }
        '/' if num2 != 0.0 => num1 / num2,
        '/' => {
            eprintln!("Error: Division by zero!");
            return;
        }
        _ => {
            eprintln!("Error: Invalid operator!");
            return;
        }
    };

    println!("Result: {}", result);
}
