# 🦀 Hello World in Rust

This is a minimal Rust program that prints `"Hello, world!"` to the terminal. It's typically the first program written when learning a new language.

---

## 📦 Project Structure

```
hello_world/
└── src/
    └── main.rs    # Contains the main function
```

---

## 📝 Code

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` is the entry point of the program.
- `println!()` is a Rust macro that prints text to the console with a newline.

---

## 🚀 How to Run

### 1. Create the project:

```bash
cargo new hello_world
cd hello_world
```

### 2. Build and run:

```bash
cargo run
```

You will see:

```
Hello, world!
```

---

## 📚 What You Learn

- How to define the `main` function in Rust.
- How to use the `println!` macro to output text.
- How to compile and run a Rust project using Cargo.

---

## ✅ Next Steps

Try modifying the output or adding more `println!` lines to explore how Rust handles strings and formatting!

```rust
fn main() {
    println!("Hello, world!");
    println!("Welcome to Rust programming.");
}
```

---
