//! # Demo of Basic Rust Concepts
//!
//! This module demonstrates a collection of Rust language concepts such as
//! variables, shadowing, number systems, type conversion using `try_into`,
//! and operations with complex numbers using the `num` crate.

use num::complex::Complex;

/// Entry point of the demo.
///
/// Calls individual demo functions to showcase basic concepts.
pub fn main() {
    variables_demo();
    shadowing_demo();
    numbers_with_base();
    try_into_demo();
    complex_demo();
}

/// Demonstrates immutable, mutable, and constant variables in Rust.
///
/// - `x` is immutable.
/// - `y` is mutable and modified later.
/// - `LEETCODE_RATING` is a constant.
fn variables_demo() {
    println!("\nVariables");

    let x = 5;
    let mut y = 10;
    const LEETCODE_RATING: i32 = 1889;
    let _ = 1627_i32;

    println!("x = {}, y = {}", x, y);
    y += 1;
    println!("y = {}, Rating = {LEETCODE_RATING}\n", y);
}

/// Demonstrates variable shadowing in Rust.
///
/// A variable `x` is re-declared (shadowed) in both the outer and inner scopes.
fn shadowing_demo() {
    let x = 10;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}\n");
}

/// Prints numbers in different bases (binary, octal, hexadecimal, decimal).
///
/// Demonstrates how to define and format integers using different numeral systems.
fn numbers_with_base() {
    let three = 0b11;        // binary
    let thirty = 0o36;       // octal
    let three_hundred = 0x12C; // hexadecimal

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}\n", three, thirty, three_hundred);
}

/// Demonstrates type conversion with `try_into`.
///
/// Converts a `u16` into `i32` and compares it with another `i32` variable.
fn try_into_demo() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap(); // Convert u16 to i32 safely
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

/// Demonstrates use of the `num::complex::Complex` type.
///
/// Adds two complex numbers and prints the result.
fn complex_demo() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);

    let result = a + b;
    println!("{} + {}i", result.re, result.im)
}
