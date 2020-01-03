extern crate rust_compute;

fn main() {
    let mut my_fraction = rust_compute::Fraction::new();
    my_fraction.denominator = 5;
    my_fraction.numerator = 3;
    let derived_fraction = rust_compute::Fraction::from(4);
    println!("Defined: {}       Derived: {}", my_fraction, derived_fraction);
}