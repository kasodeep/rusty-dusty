pub fn demo() {
    println!("Vectors");

    vector_initialization_examples();
    vector_operations();
    
    advanced_vector_features();
    vector_safety_and_optimization();
}

fn vector_initialization_examples() {
    let mut _empty_vec: Vec<i32> = Vec::new();
    let mut _with_capacity: Vec<i32> = Vec::with_capacity(10);
    let _macro_vec = vec![1, 2, 3, 4, 5];
    let _filled_vec = vec![0; 10];

    let tuple_vector = vec![(1, 2), (3, 4), (5, 6)];
    println!("Tuple Vector: {:?}", tuple_vector);
}

fn vector_operations() {
    let mut vec = Vec::new();
    
    vec.push(1);
    vec.extend([2, 3, 4].iter().cloned());
    
    if let Some(last) = vec.pop() {
        println!("Popped: {}", last);
    }
    
    // panics if out of bounds
    let _second = vec[1]; 
    let _safe_second = vec.get(1);
    
    vec.insert(1, 5);
    vec.remove(1);
    
    let slice = &vec[1..3];
    println!("Slice: {:?}", slice);
    
    vec.reserve(10);
    vec.shrink_to_fit();
}

fn advanced_vector_features() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    let drained: Vec<_> = vec.drain(1..3).collect();
    println!("Drained: {:?}", drained);
    
    vec.retain(|&x| x % 2 == 0);
    vec.dedup();
    
    // let (_left, _right) = vec.split_at(2);
    
    for window in vec.windows(2) {
        println!("Window: {:?}", window);
    }
    
    for chunk in vec.chunks(2) {
        println!("Chunk: {:?}", chunk);
    }
}

fn vector_safety_and_optimization() {
    let mut vec = Vec::with_capacity(1000);
    vec.reserve_exact(1000);
    
    for i in 0..1000 {
        vec.push(i);
    }
    
    // safe concurrent access.
    let shared = &vec;
    let mut _iter = shared.iter();
    
    // zero-cost abstractions
    let sum: i32 = vec.iter().sum();
    println!("Sum: {}\n", sum);
}