pub fn main() {
    println!("String:");

    // string literals (&str)
    println!("String Literals (&str):");
    
    // string literal declaration.
    let string_literal = "Hello, world!";
    println!("String literal: {}", string_literal);

    // raw string literal (no escape characters)
    let raw_string = r#"This is a "raw" string
    with \ backslashes"#;
    println!("Raw string: {}", raw_string);

    // string slice operations
    let slice = &string_literal[0..5];
    println!("Slice of string literal (0..5): {}", slice);

    // string type
    println!("\nString Type:");
    
    // creating new string.
    let mut owned_string = String::new();
    owned_string.push_str("Hello");
    println!("Created and modified String: {}", owned_string);

    // different ways to create string.
    let from_literal = String::from("Hello, world!");
    let to_string = "Hello, world!".to_string();
    println!("String from literal: {}", from_literal);
    println!("String from to_string(): {}", to_string);

    // string manipulation.
    println!("\nString Manipulation:");
    
    // push operations.
    let mut s = String::from("Hello");
    s.push(' ');
    s.push_str("world!");
    println!("After push operations: {}", s);

    // concatenation.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // Note: s1 is moved here
    println!("Concatenated string: {}", s3);

    // format macro.
    let formatted = format!("{} {}!", "Hello", "world");
    println!("Formatted string: {}", formatted);

    // string methods.
    println!("\nString Methods:");
    
    let example = String::from("Hello, World!");
    
    println!("Length: {}", example.len());
    println!("Is empty: {}", example.is_empty());
    println!("Contains 'World': {}", example.contains("World"));
    println!("Replaced: {}", example.replace("World", "Rust"));

    println!("Split by comma:");
    for part in example.split(",") {
        println!("  {}", part.trim());
    }

    // string conversion.
    println!("\nString Conversion:");

    // String to &str
    let owned_string = String::from("Hello");
    let str_slice: &str = &owned_string;
    println!("String as &str: {}", str_slice);

    // advanced string operations.
    println!("\n6. Advanced String Operations:");
    
    // Chars iteration
    let text = "Hello, 世界";
    println!("Characters in string:");
    for c in text.chars() {
        print!("{} ", c);
    }
    println!();

    // bytes
    println!("Bytes in 'Hello':");
    for b in "Hello".bytes() {
        print!("{} ", b);
    }
    println!("\n");
}