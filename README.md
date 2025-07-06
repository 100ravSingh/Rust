# 🦀 Rust Programming Learning Tutorial

This tutorial provides a quick start guide to installing Rust and running your first Rust project using both `rustc` and `cargo`.

---

## 📘 Lessons

| Lesson | Description                                                                 |
|--------|-----------------------------------------------------------------------------|
| 1️⃣     | [Hello World](./hello-world/README.md) – Your first Rust program with `fn main()` and `println!()` |
| 2️⃣     | [My Calculator](./my_calculator/README.md) – A modular Rust CLI calculator using `lib.rs`, `main.rs`, and `bin/*.rs` |
| 3️⃣     | [Rectangle](./rectangle/README.md) – Learn how to define `struct`s, implement methods with `impl`, and write unit tests in `lib.rs` |

---



## 🛠️ Installation (Linux / macOS)

Open your terminal and run the following:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts. Then activate Rust for your current session:

```bash
source $HOME/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
echo $PATH
```

---

## 🚀 Your First Rust Project

### Option A: Using `cargo`

Create a new project:

```bash
cargo new get-dependencies
cd get-dependencies
```

Build the project:

```bash
cargo build    # Compiles and creates an executable
cargo check    # Checks code without building an executable
```

Edit the default source file:

```bash
cd src
```

Open `main.rs` and add:

```rust
fn main() {
    println!("Hello, world!");
}
```

Run your code:

```bash
cargo run
```

---

### Option B: Using `rustc` (Direct Compilation)

If you want to manually compile without `cargo`:

1. Create a file named `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

2. Compile and run it:

```bash
rustc main.rs
./main
```

---

## 📦 Creating Another Project (e.g., `my_game`)

To create a new Rust project called `my_game`, run:

```bash
cargo new my_game
cd my_game
cargo run
```

---

## ✅ Summary

| Command             | Description                                |
|---------------------|--------------------------------------------|
| `cargo new <name>`  | Create a new Rust project                  |
| `cargo build`       | Compile the project                        |
| `cargo check`       | Check code for errors without building     |
| `cargo run`         | Compile and run the project                |
| `rustc main.rs`     | Compile a standalone Rust file             |
| `./main`            | Run compiled Rust binary                   |

---
