//! # Data Types Demo
//!
//! This module demonstrates basic built-in data types in Rust,
//! including arrays, tuples, floating-point numbers, and booleans.

/// Entry point demonstrating usage of several basic Rust data types.
pub fn main() {
    println!("Data-Types");

    // Array of integers
    let arr = [1, 2, 3, 4, 5];

    // Tuple containing mixed types: (&str, i32, &str)
    let tup = ("Deep", 17, "2004");

    let float_val = 10.7;
    let is_alive = true;

    println!("float = {}, bool = {}", float_val, is_alive);
    println!("arr = {:#?}, tuple = {:?}", arr, tup);

    println!();
}
