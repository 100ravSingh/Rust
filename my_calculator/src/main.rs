use std::io;
use rust_calculator::{add, subtract, multiply, divide, get_two_numbers};

fn main() {
    println!("🧮 Welcome to the Rust Calculator!");

    loop {
        println!("\nSelect an operation:");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let (a, b) = get_two_numbers();
                println!("Result: {} + {} = {}", a, b, add(a, b));
            },
            "2" => {
                let (a, b) = get_two_numbers();
                println!("Result: {} - {} = {}", a, b, subtract(a, b));
            },
            "3" => {
                let (a, b) = get_two_numbers();
                println!("Result: {} * {} = {}", a, b, multiply(a, b));
            },
            "4" => {
                let (a, b) = get_two_numbers();
                match divide(a, b) {
                    Ok(result) => println!("Result: {} / {} = {}", a, b, result),
                    Err(e) => println!("❌ Error: {}", e),
                }
            },
            "5" => {
                println!("👋 Exiting calculator. Goodbye!");
                break;
            },
            _ => println!("❌ Invalid option. Please try again."),
        }
    }
}

