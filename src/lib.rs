#![crate_type = "lib"]
#![crate_name = "rust_compute"]

use std::fmt;

pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}/{}", self.numerator, self.denominator);
    }
}

impl Fraction {
    pub fn new() -> Self {
        Fraction {
            numerator: 1,
            denominator: 1,
        }
    }
}