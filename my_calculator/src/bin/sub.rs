use rust_calculator::{subtract, get_two_numbers};

fn main() {
    println!("Subtraction selected.");
    let (a, b) = get_two_numbers();
    println!("Result: {} - {} = {}", a, b, subtract(a, b));
}
