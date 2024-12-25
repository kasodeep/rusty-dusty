mod concepts;

fn main() {
    println!("Hello, World!");

    let x = 5;
    let mut y = 10;

    println!("x = {}, y = {}", x, y);
    y += 1;
    println!("y = {}\n", y);

    // Data-Types.
    concepts::data_types::data_types();
    // Slices.
    concepts::slices::slices();
    // Strings.
    concepts::strings::strings();

    // Functions.
    let sum = concepts::function::add(10, 20);
    println!("Function Returned: {sum}\n");

    // Tuples.
    concepts::tuples::tuples();
    // Loops.
    concepts::loops::loops();
}
