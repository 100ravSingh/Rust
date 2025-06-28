# 🦀 Rust Programming Learning Tutorial

This tutorial provides a quick start guide to installing Rust and running your first Rust project using both `rustc` and `cargo`.

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

---

## 🧠 Project Architecture: `main.rs`, `lib.rs`, and `bin/*.rs`

This Rust calculator is designed using a clean, modular architecture that separates concerns between:

### 📚 `src/lib.rs` — Shared Library
- Contains **reusable logic** for:
  - Math operations: `add`, `subtract`, `multiply`, `divide`
  - User input: `get_two_numbers()`
- Functions are marked `pub` so they can be used elsewhere.
- **Acts like the "brains"** of the project — pure logic without I/O control flow.

---

### 🧾 `src/main.rs` — Main Application
- Serves as the **primary user interface** for the calculator.
- Presents a **menu-based CLI** where users can:
  - Select an operation
  - Input numbers
  - View the result
- Directly uses functions from `lib.rs` for calculations.
- Run this with:

```bash
cargo run --bin my_calculator
```

Or set it as default in `Cargo.toml`:

```toml
[package]
name = "my_calculator"
default-run = "my_calculator"
```

---

### ⚙️ `src/bin/*.rs` — Individual Tools
Each file is a **standalone binary** compiled by Cargo:

- `bin/add.rs` → Addition-only calculator
- `bin/sub.rs` → Subtraction-only calculator
- `bin/multiply.rs` → Multiplication-only calculator
- `bin/divide.rs` → Division-only calculator

They all:
- Use the same shared functions from `lib.rs`
- Prompt for two numbers
- Perform a single calculation

Run any one directly:

```bash
cargo run --bin add
```

---

### 🔁 How They Work Together

```
User
 ↓
main.rs ─────────────┬──────────────→ bin/add.rs
   ↓                 │
lib.rs (shared logic)├──────────────→ bin/sub.rs
                     ├──────────────→ bin/multiply.rs
                     └──────────────→ bin/divide.rs
```

- All binaries reuse the same **core logic** from `lib.rs`
- This ensures consistency, reduces code duplication, and supports scaling

---
