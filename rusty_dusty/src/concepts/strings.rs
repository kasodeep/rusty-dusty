/// Demonstrates usage of string literals (`&str`) and heap-allocated `String` in Rust.
///
/// Covers:
/// - Creation and usage of string literals and raw strings
/// - String slicing and trimming
/// - Creating and modifying owned `String` types
/// - Concatenation and formatting
/// - Common string methods (length, contains, replace, etc.)
/// - String splitting and parsing
/// - Conversion between `String` and `&str`
/// - Character and byte iteration
/// - Line splitting and char transformation
/// - Capacity management
/// - Pattern matching with indices
pub fn main() {
    println!("String and Literals &str:");

    // string literal declaration.
    let string_literal = "  Hello, world!  ";
    println!("String literal: '{}'", string_literal);

    // raw string literal (no escape characters)
    let raw_string = r#"This is a "raw" string
    with \ backslashes"#;
    println!("Raw string: {}", raw_string);

    // string slice operations
    let slice = &string_literal[2..7];
    println!("Slice of string literal (2..7): '{}'", slice);

    // trimming
    println!("Trimmed string literal: '{}'", string_literal.trim());

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
    let s3 = s1 + &s2; // Note: s1 is moved here
    println!("Concatenated string: {}", s3);

    // format macro.
    let formatted = format!("{} {}!", "Hello", "world");
    println!("Formatted string: {}", formatted);

    // string methods.
    println!("\nString Methods:");
    
    let example = String::from("  Hello, World!  ");
    
    println!("Length: {}", example.len());
    println!("Is empty: {}", example.is_empty());
    println!("Contains 'World': {}", example.contains("World"));
    println!("Replaced: {}", example.replace("World", "Rust"));
    println!("Trimmed: '{}'", example.trim());
    println!("To lowercase: {}", example.to_lowercase());
    println!("To uppercase: {}", example.to_uppercase());
    println!("Starts with 'Hello': {}", example.starts_with("Hello"));
    println!("Ends with 'World!': {}", example.ends_with("World!"));
    println!("Find 'World' index: {:?}", example.find("World"));

    println!("Split by comma:");
    for part in example.split(",") {
        println!("  '{}'", part.trim());
    }

    println!("Split by whitespace:");
    for part in example.split_whitespace() {
        println!("  '{}'", part);
    }

    // string conversion.
    println!("\nString Conversion:");

    // String to &str
    let owned_string = String::from("Hello");
    let str_slice: &str = &owned_string;
    println!("String as &str: {}", str_slice);

    // &str to String
    let str_to_string = "Hello".to_string();
    println!("&str to String: {}", str_to_string);

    // parsing
    let number: Result<i32, _> = "42".parse();
    println!("Parsed '42' to number: {:?}", number);

    // advanced string operations.
    println!("\nAdvanced String Operations:");
    
    // chars iteration
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
    println!();

    // lines
    let multiline = "Line 1\nLine 2\nLine 3";
    println!("Lines in multiline string:");
    for line in multiline.lines() {
        println!("  '{}'", line);
    }

    // map and collect chars
    let upper_chars: String = text.chars().map(|c| c.to_ascii_uppercase()).collect();
    println!("Uppercase chars collected: {}", upper_chars);

    // capacity management
    let mut capacity_string = String::with_capacity(20);
    capacity_string.push_str("Hello");
    println!("Capacity: {}, Length: {}", capacity_string.capacity(), capacity_string.len());

    capacity_string.shrink_to_fit();
    println!("After shrink_to_fit, Capacity: {}", capacity_string.capacity());

    // match indices
    let text = "hello hello";
    println!("Indices of 'hello':");
    for (i, matched) in text.match_indices("hello") {
        println!("  Found '{}' at index {}", matched, i);
    }
}
