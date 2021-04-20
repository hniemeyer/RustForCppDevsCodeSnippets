struct Square {
    length: f32,
}

trait Shape {
    fn area(&self) -> f32;
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.length * self.length
    }
}

// TODO: use the #[test] annotation to write a test checking the implementation of `area`.
// If you'd like to explore more, try grouping several tests in a `#[cfg(test)]` module
// see: https://doc.rust-lang.org/book/ch11-01-writing-tests.html