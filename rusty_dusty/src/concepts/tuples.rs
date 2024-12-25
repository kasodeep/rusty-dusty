pub fn tuples(){
    println!("Tuples!");

    let (product, sum) = multiply_and_add(10, 20);
    println!("Product = {}, Sum = {}", product, sum);

    let result = multiply_and_add(11, 22);
    println!("Product = {}", result.0);
    println!("Sum = {}", result.1);

    println!();
}

fn multiply_and_add(a: i32, b: i32) -> (i32, i32) {
    (a * b, a + b)
}
