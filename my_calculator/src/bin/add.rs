use rust_calculator::{add, get_two_numbers};

fn main() {
    println!("Addition selected.");
    let (a, b) = get_two_numbers();
    println!("Result: {} + {} = {}", a, b, add(a, b));
}
