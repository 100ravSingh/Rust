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
