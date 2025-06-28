use std::io;

/// Prompts user for two numbers and returns them as a tuple.
pub fn get_two_numbers() -> (f64, f64) {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter the second number:");
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();

    (a, b)
}

/// Add two numbers
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtract two numbers
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiply two numbers
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divide two numbers with zero-check
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
