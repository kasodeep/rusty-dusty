use num::complex::Complex;

pub fn main() {
    variables_demo();
    shadowing_demo();
    numbers_with_base();
    try_into_demo();
    complex_demo();
}

fn variables_demo() {
    println!("\nVariables");

    // immutable variable.
    let x = 5;

    // mutable variable.
    let mut y = 10;

    // constant variable.
    const LEETCODE_RATING: i32 = 1889;
    let _ = 1627_i32;

    println!("x = {}, y = {}", x, y);
    y += 1;
    println!("y = {}, Rating = {LEETCODE_RATING}\n", y);
}

fn shadowing_demo() {
    let x = 10;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}\n");
}

fn numbers_with_base() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;
    println!("base 10: {} {} {}", three, thirty, three_hundred);

    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}\n", three, thirty, three_hundred);
}

fn try_into_demo() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

fn complex_demo() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);

    let result = a + b;
    println!("{} + {}i", result.re, result.im)
}
