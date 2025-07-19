use std::fmt::{Debug, Display};

/// Returns the largest element in a list of comparable items.
///
/// # Type Parameters
///
/// * `T` - Must implement `PartialOrd + Copy`
///
/// # Arguments
///
/// * `items` - A slice of items
///
/// # Returns
///
/// The largest item in the slice
///
/// # Panics
///
/// Panics if the slice is empty
///
/// # Example
///
/// ```
/// let numbers = vec![10, 40, 30];
/// assert_eq!(largest(&numbers), 40);
/// ```
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut max = items[0];

    for &item in items.iter() {
        if item > max {
            max = item;
        }
    }

    max
}

/// A 2D point that can hold different types for x and y.
///
/// # Type Parameters
///
/// * `X` - Type of x coordinate
/// * `Y` - Type of y coordinate (must implement `Display`)
///
/// # Example
///
/// ```
/// let p = Point { x: 1, y: 3.5 };
/// println!("{}", p.describe());
/// ```
#[derive(Debug)]
struct Point<X: Debug, Y: Display> {
    x: X,
    y: Y,
}

impl<X: Debug, Y: Display> Point<X, Y> {
    /// Describes the point by printing x and y.
    fn describe(&self) -> String {
        format!("Point at x = {:?}, y = {}", self.x, self.y)
    }
}

/// A generic wrapper that can hold any value and display it.
///
/// # Type Parameters
/// * `T` - Must implement `Display`
struct Wrapper<T: Display> {
    value: T,
}

impl<T: Display> Wrapper<T> {
    fn show(&self) {
        println!("Wrapped value: {}", self.value);
    }
}

/// Demonstrates generic functions, structs, trait bounds, and more.
///
/// This function:
/// - Finds largest items in lists
/// - Uses generic `Point` with different coordinate types
/// - Wraps a `String` in a generic `Wrapper`
pub fn demo() {
    println!("=== Generics Demo ===");

    // Generic function usage
    let nums = vec![1, 5, 3, 9, 2];
    let max_num = largest(&nums);
    println!("Largest number: {}", max_num);

    let chars = vec!['g', 'z', 'a', 'b'];
    let max_char = largest(&chars);
    println!("Largest char: {}", max_char);

    // Generic struct usage
    let point = Point { x: 10, y: 3.14 };
    println!("{}", point.describe());

    // Wrapper example
    let wrapped = Wrapper {
        value: String::from("Rust"),
    };
    wrapped.show();
}
