use std::{f32::consts::PI as PI32, f64::consts::PI as PI64};

pub fn main() {
    println!("Tuples");
    
    // simple tuple with type inferences.
    let tuple = (1, "hello", PI32);
    println!("Simple tuple: {:?}", tuple);

    // xxplicit type declaration.
    let tuple_explicit: (i32, &str, f64) = (1, "hello", PI64);
    println!("Explicitly typed tuple: {:?}", tuple_explicit);

    // unit tuple.
    let unit = ();
    println!("Unit tuple: {:?}", unit);

    // nested tuples.
    let nested = (1, ("hello", PI64), true);
    println!("Nested tuple: {:?}", nested);

    // accessing tuple elements.
    println!("\nAccessing Tuple Elements:");
    
    // using index notation.
    println!("First element: {}", tuple.0);
    println!("Second element: {}", tuple.1);
    println!("Third element: {}", tuple.2);

    // destructuring.
    let (x, y, z) = tuple;
    println!("Destructured values: x = {}, y = {}, z = {}", x, y, z);

    // mutable tuples.
    println!("\nMutable Tuples:");
    
    let mut mutable_tuple = (1, String::from("night_fury"), PI64);
    mutable_tuple.1 = String::from("deepkaso");
    println!("Modified tuple: {:?}", mutable_tuple);

    // tuple methods and operations.
    println!("\nTuple Methods and Operations:");

    // copy and clone.
    let original = (1, 2, 3);    
    let mut copied = original;
    copied.0 = 2;
    println!("Original tuple: {:?}", original);
    println!("Copied tuple: {:?}", copied);

    // using tuples in functions.
    println!("\nUsing Tuples in Functions:");
    
    fn return_tuple() -> (i32, &'static str) {
        (42, "answer")
    }

    let (number, text) = return_tuple();
    println!("Returned tuple values: number = {}, text = {}", number, text);

    // advanced tuple usage.
    println!("\nAdvanced Tuple Usage:");

    // tuple as function parameters
    fn process_tuple((x, y): (i32, i32)) -> i32 {
        x + y
    }

    let sum = process_tuple((5, 3));
    println!("Sum from tuple: {}\n", sum);
}