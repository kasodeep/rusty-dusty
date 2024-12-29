use std::fmt::Display;

/// Finds the largest element in a slice of comparable items.
/// 
/// # Arguments
/// 
/// * `list` - A slice of elements that implement `PartialOrd`
/// 
/// # Returns
/// 
/// A reference to the largest element in the slice
/// 
/// # Panics
/// 
/// Panics if the slice is empty
/// 
/// # Example
/// 
/// ```
/// let numbers = vec![34, 50, 25, 100, 65];
/// let largest_num = largest(&numbers);
/// assert_eq!(*largest_num, 100);
/// ```
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// A generic point in 2D space with coordinates of potentially different types.
/// 
/// # Type Parameters
/// 
/// * `T` - The type of the x coordinate
/// * `U` - The type of the y coordinate, must implement `Display`
/// 
/// # Examples
/// 
/// ```
/// let point = Point { x: 5, y: 4.0 };
/// assert_eq!(*point.x(), 5);
/// ```
struct Point<T, U: Display> {
    /// The x coordinate
    x: T,
    /// The y coordinate
    y: U,
}

impl<T, U: Display> Point<T, U> {
    /// Returns a reference to the x coordinate.
    /// 
    /// This method also prints the y coordinate to demonstrate the Display constraint.
    /// 
    /// # Returns
    /// 
    /// A reference to the x coordinate
    fn x(&self) -> &T {
        println!("Point y: {}", self.y);
        &self.x
    }
}

/// Demonstrates the usage of generic types with the `largest` function and `Point` struct.
/// 
/// This function shows how the same generic function can work with different types,
/// and demonstrates the usage of generic structs with different type parameters.
pub fn demo() {
    println!("Generics");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let _integer_and_float = Point { x: 5, y: 4.0 };
    println!("Point x: {}\n", _integer_and_float.x());
}