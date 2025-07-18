/// Demonstrates usage of the `Option` enum and pattern matching
/// to handle potentially missing values and compute results.
pub fn main() {
    println!("Option<Some, None>");
    option();

    println!("\nChecking if numbers are even:");
    let numbers = [2, 3, 4, 7, -1];

    for &num in &numbers {
        match is_even(num) {
            Some(true) => println!("{num} is even"),
            Some(false) => println!("{num} is odd"),
            None => println!("Unable to determine for {num}"),
        }
    }

    println!();
}

/// Demonstrates accessing the first element of an array using `Option`.
///
/// If the array is empty, it will print `None`, otherwise it prints the first element.
fn option() {
    println!("Option Enum!");

    let arr = [1, 3, 4];
    let first = arr.first();

    match first {
        Some(value) => println!("First Element: {value}"),
        None => println!("List is Empty"),
    }
}

/// Determines if a number is even.
///
/// # Arguments
///
/// * `num` - An integer to be checked.
///
/// # Returns
///
/// * `Some(true)` if the number is even.
/// * `Some(false)` if the number is odd.
/// * `None` if the result cannot be determined (shouldn't happen with integers).
fn is_even(num: i32) -> Option<bool> {
    if num % 2 == 0 {
        Some(true)
    } else if num % 2 == 1 || num % 2 == -1 {
        Some(false)
    } else {
        None
    }
}
