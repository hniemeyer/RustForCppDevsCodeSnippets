pub struct Square {
    pub length: f32,
}

pub trait Shape {
    fn area(&self) -> f32;
}

impl Shape for Square {
    /// Calculates the area of a square.
    /// usage example:
    /// ```
    /// use rust_example_doctests::{Shape, Square};
    /// let test_square = Square {length: 23.0};
    /// test_square.area();
    /// ```
    fn area(&self) -> f32 {
        self.length * self.length
    }
}

#[test]
fn test_area() {
    let test_square = Square {length: 23.0};
    assert_eq!(529_f32, test_square.area());
}
