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

    // accessing elements.
    println!("\nAccessing Elements:");
    println!("First element: {}", numbers[0]);
    println!("Last element: {}", numbers[numbers.len() - 1]);

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

    // slice operations.
    let slice = &numbers[1..4];
    println!("Slice of array [1..4]: {:?}", slice);

    // advanced operations.
    println!("\nAdvanced Operations:");

    // convert to vector.
    let vector: Vec<i32> = numbers.to_vec();
    println!("Array converted to vector: {:?}", vector);

    // using array methods.
    let contains_2 = numbers.contains(&2);
    println!("Array contains 2: {}", contains_2);

    // find first element matching condition.
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("First even number: {:?}", first_even);

    // multi-dimensional arrays
    println!("\nMulti-dimensional Arrays:");
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("2D array (matrix): {:?}", matrix);
    println!("Access matrix element [1][1]: {}", matrix[1][1]);

    // array comparison
    println!("\nArray Comparison:");
    let array1 = [1, 2, 3];
    let array2 = [1, 2, 3];
    let array3 = [1, 2, 4];
    
    println!("array1 == array2: {}", array1 == array2);
    println!("array1 == array3: {}\n", array1 == array3);
}