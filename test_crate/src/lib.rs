#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width.checked_mul(self.height).expect("Overflow in area calculation!")
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// This tells the compiler not to include the module into binary.
#[cfg(test)] 
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_area() {
        let rect = Rectangle::new(2, 3);
        assert_eq!(rect.area(), 6, "Area calculation failed for 2x3 rectangle");
    }

    #[test]
    #[should_panic(expected = "Overflow")]
    fn test_overflow() {
        let rect = Rectangle::new(u32::MAX, 2);
        rect.area();
    }

    #[test]
    fn test_is_square() {
        let square = Rectangle::new(5, 5);
        let rectangle = Rectangle::new(5, 10);
        
        assert!(square.is_square(), "5x5 should be a square");
        assert!(!rectangle.is_square(), "5x10 should not be a square");
    }

    #[test]
    fn test_multiple_sizes() {
        let test_cases = vec![
            (2, 3, 6),
            (4, 4, 16),
            (5, 2, 10),
        ];
        
        for (width, height, expected_area) in test_cases {
            let rect = Rectangle::new(width, height);
            assert_eq!(
                rect.area(), 
                expected_area,
                "Failed for rectangle {}x{}", 
                width, 
                height
            );
        }
    }
}