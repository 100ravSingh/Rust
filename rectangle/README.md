# 🦀 Rust Lesson 3 – Structs and Methods

In this lesson, you'll learn how to define your own data types using `struct`, and how to implement behavior on those types using `impl` blocks. We'll build and experiment with a `Rectangle` struct to compute area, compare sizes, and create square shapes.

---

## 📚 Concepts Covered

✅ Defining custom data types with `struct`  
✅ Implementing methods with `impl`  
✅ Using `self` references in method signatures  
✅ Creating associated functions (`Self::new`)  
✅ Writing unit tests with `#[cfg(test)]`  
✅ Modularizing logic with `lib.rs`

---

## 📁 Project Structure

```
lesson_3_structs/
├── Cargo.toml
└── src/
    ├── main.rs       # Entry point that uses the Rectangle library
    └── lib.rs        # Rectangle struct, methods, and unit tests
```

---

## 🧱 Code Overview

### `src/lib.rs`

```rust
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_works() {
        let rect = Rectangle { width: 10, height: 20 };
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn can_hold_true() {
        let r1 = Rectangle { width: 30, height: 50 };
        let r2 = Rectangle { width: 20, height: 40 };
        assert!(r1.can_hold(&r2));
    }

    #[test]
    fn can_hold_false() {
        let r1 = Rectangle { width: 20, height: 20 };
        let r2 = Rectangle { width: 30, height: 40 };
        assert!(!r1.can_hold(&r2));
    }

    #[test]
    fn square_creates_equal_sides() {
        let s = Rectangle::square(15);
        assert_eq!(s.width, 15);
        assert_eq!(s.height, 15);
    }
}
```

---

### `src/main.rs`

```rust
use rectangle::Rectangle;

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle::square(25);

    println!("rect1: {:#?}", rect1);
    println!("Area of rect1: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

> ⚠️ Make sure the crate name in your `Cargo.toml` is `rectangle`:
> ```toml
> [package]
> name = "rectangle"
> ```

---

## 🚀 Run the Program

```bash
cargo run
```

### Sample Output:

```
rect1: Rectangle {
    width: 30,
    height: 50,
}
Area of rect1: 1500
Can rect1 hold rect2? true
Can rect1 hold rect3? true
```

---

## 🧪 Run the Tests

```bash
cargo test
```

### Output:
```bash
running 4 tests
test tests::area_works ... ok
test tests::can_hold_false ... ok
test tests::can_hold_true ... ok
test tests::square_creates_equal_sides ... ok

test result: ok. 4 passed; 0 failed
```

---

## 🧩 Summary

| Concept             | Syntax / Example                            |
|---------------------|---------------------------------------------|
| Define struct        | `struct Rectangle { width: u32, ... }`      |
| Add method           | `impl Rectangle { fn area(&self) { ... } }` |
| Use `self` reference | Required for accessing instance data        |
| Associated function  | `Rectangle::square(25)`                     |
| Unit testing         | `#[test]`, `assert_eq!`, `assert!`          |

---

## ✅ Next Up

In **Lesson 4**, we’ll dive into:

- Enums and pattern matching
- Using `match`, `Option`, and `Result`
- Building expressive decision logic in Rust

Stay tuned!
