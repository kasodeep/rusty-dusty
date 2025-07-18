use std::time::{Duration, Instant};

pub fn main() {
    loops_demo();
    if_else_demo();
    match_demo();
    increment_per_second();
}

fn loops_demo() {
    println!("Loops");

    // for loop examples.
    println!("For Loops:");

    // basic range loop.
    print!("Basic range: ");
    for num in 1..=5 {
        print!("{num} ");
    }
    println!();

    // iterate over array.
    print!("Array iteration: ");
    let numbers = [1, 2, 3, 4, 5];
    for num in numbers {
        print!("{num} ");
    }
    println!();

    // iterate with enumerate.
    print!("Enumerate: ");
    for (index, value) in numbers.iter().enumerate() {
        print!("{}:{} ", index, value);
    }
    println!();

    // while loop examples
    println!("\n2. While Loops:");

    let mut counter = 1;
    '_outer: while counter <= 5 {
        print!("{counter} ");
        counter += 1;
    }
    println!();

    // loop (Infinite Loop) examples.
    println!("\n3. Loop Examples:");

    // basic loop with break.
    print!("Basic loop with break: ");
    let mut count = 0;
    loop {
        print!("{count} ");
        count += 1;
        if count == 5 {
            break;
        }
    }
    println!();

    // loop with return value.
    let result = loop {
        if count >= 10 {
            break count * 2;
        }
        count += 1;
    };
    println!("Loop with return value: {result}\n");
}

fn if_else_demo() {
    println!("If/Else");

    let age = 25;
    let has_license = true;

    if age >= 18 && has_license {
        println!("Can drive");
    } else if age >= 18 {
        println!("Need to get a license");
    } else {
        println!("Too young to drive");
    }

    let arr = [1, 2, 4];
    if let Some(max) = arr.get(2) {
        println!("Maximum is configured to be {max}");
    }

    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("Value from if expression: {value}\n");
}

fn match_demo() {
    println!("Expression");

    let number = 13;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime number"),
        _ => println!("Something else"),
    }

    let grade = 85;
    match grade {
        90..=100 => println!("A"),
        80..=89 => println!("B"),
        70..=79 => println!("C"),
        60..=69 => println!("D"),
        _ => println!("F"),
    }

    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Sum to zero"),
        (x, _) if x % 2 == 0 => println!("First is even"),
        _ => println!("No match"),
    }

    let msg = Some(String::from("Hello"));
    match msg {
        Some(text) if text.len() > 5 => println!("Long message: {text}"),
        Some(text) => println!("Short message: {text}"),
        None => println!("No message"),
    }
    println!();
}

fn increment_per_second() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();
    
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("Counting speed per second: {count}");
}
