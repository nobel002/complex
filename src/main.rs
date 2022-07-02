mod complex;
mod tests;

fn main() {
    println!("Hello, world!");
    let a = complex::Complex { re: 1.0, im: 2.0 };
    let b = complex::Complex { re: 3.0, im: 4.0 };
    let c = a + b;
    println!("{}•e^({}i)", c.re, c.im);
}
