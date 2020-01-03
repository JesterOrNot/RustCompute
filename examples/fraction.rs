extern crate rust_compute;

fn main() {
    let mut my_fraction = rust_compute::Fraction::new();
    my_fraction.denominator = 5;
    my_fraction.numerator = 3;
    println!("{}", my_fraction);
}