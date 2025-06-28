## 🧠 Project Architecture: `my_calculator`

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
