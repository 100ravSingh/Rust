use rust_calculator::{multiply, get_two_numbers};

fn main() {
    println!("Multiplication selected.");
    let (a, b) = get_two_numbers();
    println!("Result: {} * {} = {}", a, b, multiply(a, b));
}
