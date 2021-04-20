struct Square {
    length: f32,
}

trait Shape {
    fn area(&self) -> f32;
}

impl Shape for Square {
    // INSTRUCTOR NOTE: for doctest examples, see `../rust-example-doctests`
    // (doctests only work in libraries and this is a bin target)
    fn area(&self) -> f32 {
        self.length * self.length
    }
}

#[test]
fn test_area() {
    let test_square = Square {length: 23.0};
    assert_eq!(529_f32, test_square.area());
}
