use text_io::read;
use std::io::{self, Write};

pub fn run() {
    println!("1. km/h -> mi/h");
    println!("2. mi/h -> km/h");

    print!("> ");
    io::stdout().flush().unwrap();
    let choice_w: i32 = read!();

    if choice_w == 1 {
        print!("Enter the speed in km/h: ");
        io::stdout().flush().unwrap();
        let kmh: f64 = read!();

        let mih = kmh / 1.60934134;
        println!("Result: {:.5} mph", mih);
    } else if choice_w == 2 {
        print!("Enter the speed in mi/h: ");
        io::stdout().flush().unwrap();
        let mih: f64 = read!();

        let kmh = mih * 1.60934133;
        println!("Result: {:.5} kmh", kmh);
    } else {
        eprintln!("Invalid choice!");
    }
}