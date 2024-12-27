use concepts::own::demo;

mod collections;
mod concepts;

fn main() {
    println!("Hello, World!");

    concepts::variables::main();
    concepts::data_types::main();

    concepts::array::main();
    concepts::tuples::main();
    
    concepts::strings::main();
    concepts::control_flow::main();

    concepts::errors::main();
    concepts::option::main();

    concepts::structs::main();
    concepts::enums::main();

    // advanced concepts.
    demo();
    collections::vector::demo();
    collections::hashmap::demo();
    collections::btreemap::demo();
    collections::hashset::demo();
}
