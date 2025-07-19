// Import necessary modules
use std::env; // For accessing command-line arguments

// Bring a specific function into scope with an alias
use concepts::own::main as own_demo;

// Declare internal modules that contain various concepts and demos
mod collections;
mod concepts;
mod advanced;

/// Entry point of the program.
/// This program executes selected demo modules based on command-line arguments.
fn main() {
    // Collect command-line arguments, skipping the first one (program name)
    let args: Vec<String> = env::args().skip(1).collect();

    // If no arguments are passed, display usage instructions and return
    if args.is_empty() {
        println!("Usage: cargo run -- <demo_name> [<demo_name> ...]");
        println!("Example: cargo run -- variables data_types vector");
        return;
    }

    // Iterate over the provided demo names and execute the matching module
    for arg in &args {
        match arg.as_str() {
            // Basic Rust concepts
            "variables" => concepts::variables::main(),
            "data_types" => concepts::data_types::main(),
            "array" => concepts::array::main(),
            "tuples" => concepts::tuples::main(),
            "strings" => concepts::strings::main(),
            "control_flow" => concepts::control_flow::main(),
            "errors" => concepts::errors::main(),
            "option" => concepts::option::main(),
            "compound_types" => concepts::compound_types::main(),

            // Ownership and borrowing
            "own" => own_demo(),

            // Collections
            "vector" => collections::vector::demo(),
            "hashmap" => collections::hashmap::demo(),
            "btreemap" => collections::btreemap::demo(),
            "hashset" => collections::hashset::demo(),

            // Intermediate concepts
            "generics" => concepts::generics::demo(),
            "traits" => concepts::traits::trait_demo(),
            "lifetimes" => concepts::lifetimes::demo(),

            // Advanced topics
            "iter_closure" => advanced::iter_closure::demo(),
            "concurrency" => advanced::concurrency::demo(),

            // Fallback for unknown arguments
            _ => println!("Unknown demo: {}", arg),
        }
    }
}
