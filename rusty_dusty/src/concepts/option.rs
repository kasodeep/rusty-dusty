pub fn main() {
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
}

fn option() {
    println!("Option Enum!");

    let arr = [1, 3, 4];
    let first = arr.first();

    match first {
        Some(value) => println!("First Element: {value}"),
        None => println!("List is Empty"),
    }
}

fn is_even(num: i32) -> Option<bool> {
    if num % 2 == 0 {
        Some(true)
    } else if num % 2 == 1 {
        Some(false)
    } else {
        None
    }
}
