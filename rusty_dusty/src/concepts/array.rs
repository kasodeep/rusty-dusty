/// Demonstrates a comprehensive set of operations on arrays in Rust.
///
/// This function covers:
/// - Array creation (inferred and explicit types)
/// - Initialization with default values
/// - Accessing properties and elements
/// - Iteration and mutation
/// - Sorting and reversing
/// - Slicing and splitting
/// - Chunking and sliding windows
/// - Conversion to vector and higher-order functions (map, filter, fold)
/// - Multidimensional arrays
/// - Array comparison
pub fn main() {
    println!("Arrays");

    // fixed-size array with type inference.
    let numbers = [1, 2, 3, 4, 5];
    println!("Simple array: {:?}", numbers);

    // explicit type declaration.
    let numbers_explicit: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Explicitly typed array: {:?}", numbers_explicit);

    // initialize array with default values.
    let zeros = [0; 5]; // Creates [0, 0, 0, 0, 0]
    println!("Array filled with zeros: {:?}", zeros);

    // array properties.
    println!("\nArray Properties:");
    println!("Length of array: {}", numbers.len());    
    println!("Size of array in bytes: {}", std::mem::size_of_val(&numbers));
    println!("Is array empty? {}", numbers.is_empty());

    // accessing elements.
    println!("\nAccessing Elements:");
    println!("First element: {}", numbers[0]);
    println!("Last element: {}", numbers[numbers.len() - 1]);
    println!("Safe access (get 2): {:?}", numbers.get(2)); // Safe indexing
    println!("Safe access (out of bounds): {:?}", numbers.get(10));

    // array methods and operations.
    println!("\nArray Methods and Operations:");

    // iterate over elements.
    print!("Iterating over array: ");
    for number in numbers.iter() {
        print!("{} ", number);
    }
    println!();

    // create mutable array.
    let mut mutable_array = [1, 2, 3, 4, 5];
    
    // modify elements.
    mutable_array[2] = 10;
    println!("Modified array: {:?}", mutable_array);

    // sort array.
    let mut sortable_array = [5, 2, 8, 1, 9];
    sortable_array.sort();
    println!("Sorted array: {:?}", sortable_array);

    // reverse array.
    sortable_array.reverse();
    println!("Reversed array: {:?}", sortable_array);

    // slice operations.
    let slice = &numbers[1..4];
    println!("Slice of array [1..4]: {:?}", slice);

    // split_at.
    let (left, right) = numbers.split_at(3);
    println!("Split at 3: Left: {:?}, Right: {:?}", left, right);

    // chunks.
    println!("Chunks of size 2:");
    for chunk in numbers.chunks(2) {
        println!("  {:?}", chunk);
    }

    // windows.
    println!("Windows of size 2:");
    for window in numbers.windows(2) {
        println!("  {:?}", window);
    }

    // advanced operations.
    println!("\nAdvanced Operations:");

    // convert to vector.
    let vector: Vec<i32> = numbers.to_vec();
    println!("Array converted to vector: {:?}", vector);

    // using array methods.
    let contains_2 = numbers.contains(&2);
    println!("Array contains 2: {}", contains_2);

    // find first element matching condition.
    let first_even = numbers.iter().find(|&x| x % 2 == 0);
    println!("First even number: {:?}", first_even);

    // map: double each element.
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled elements: {:?}", doubled);

    // filter: keep even numbers.
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("Even numbers: {:?}", evens);

    // fold: sum all elements.
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum of elements: {}", sum);

    // multi-dimensional arrays
    println!("\nMulti-dimensional Arrays:");
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("2D array (matrix): {:?}", matrix);

    // array comparison
    println!("\nArray Comparison:");
    let array1 = [1, 2, 3];
    let array2 = [1, 2, 3];
    let array3 = [1, 2, 4];
    
    println!("array1 == array2: {}", array1 == array2);
    println!("array1 == array3: {}", array1 == array3);
}
