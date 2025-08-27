use text_io::read;

fn main() {
    print!("Enter a number: ");
    let num1: f64 = read!();

    print!("Enter a operator: ");
    let op: char = read!();

    print!("Enter a number: ");
    let num2: f64 = read!();

    let result: f64 = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
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

    println!("Result: {}", result)
}