use rust_calculator::{divide, get_two_numbers};

fn main() {
    println!("Division selected.");
    let (a, b) = get_two_numbers();
    match divide(a, b) {
        Ok(result) => println!("Result: {} / {} = {}", a, b, result),
        Err(e) => println!("❌ Error: {}", e),
    }
}
