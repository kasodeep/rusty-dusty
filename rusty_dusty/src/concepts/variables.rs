pub fn main(){
    variables_demo();
    shadowing_demo();
}

fn variables_demo(){
    println!("\nVariables");

    // immutable variable.
    let x = 5;

    // mutable variable.
    let mut y = 10;

    // constant variable.
    const LEETCODE_RATING: i32 = 1889;

    println!("x = {}, y = {}", x, y);
    y += 1;
    println!("y = {}, Rating = {LEETCODE_RATING}\n", y);    
}

fn shadowing_demo(){
    let x = 10;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}\n");
}