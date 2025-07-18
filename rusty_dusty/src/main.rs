use std::env;

use concepts::own::demo as own_demo;

mod collections;
mod concepts;
mod advanced;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect(); // Skip program name.

    if args.is_empty() {
        println!("Usage: cargo run -- <demo_name> [<demo_name> ...]");
        println!("Example: cargo run -- variables data_types vector");
        return;
    }

    for arg in &args {
        match arg.as_str() {
            "variables" => concepts::variables::main(),
            "data_types" => concepts::data_types::main(),
            "array" => concepts::array::main(),
            "tuples" => concepts::tuples::main(),
            "strings" => concepts::strings::main(),
            "control_flow" => concepts::control_flow::main(),
            "errors" => concepts::errors::main(),
            "option" => concepts::option::main(),
            "structs" => concepts::structs::main(),
            "enums" => concepts::enums::main(),
            "own" => own_demo(),
            "vector" => collections::vector::demo(),
            "hashmap" => collections::hashmap::demo(),
            "btreemap" => collections::btreemap::demo(),
            "hashset" => collections::hashset::demo(),
            "generics" => concepts::generics::demo(),
            "traits" => concepts::traits::demo(),
            "lifetimes" => concepts::lifetimes::demo(),
            "iter_closure" => advanced::iter_closure::demo(),
            "concurrency" => advanced::concurrency::demo(),
            _ => println!("Unknown demo: {}", arg),
        }
    }
}
