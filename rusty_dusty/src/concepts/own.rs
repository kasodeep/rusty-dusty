//! * The String type does not support Copy trait, but Clone is allowed.
pub fn demo(){
    println!("Ownership and Borrowing!");

    let s1 = String::from("Night");
    let s2 = s1;
    let _s3 = s2.clone();

    // println!("Out of Scope: {}", s1);
    println!("In Scope: {}", s2);

    // the ownership is transferred to function and s1 goes out of scope.
    let s1 = String::from("Fury");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    // borrowing and referencing.
    let len = calculate_length_reference(&s2);
    println!("The length of '{s2}' is {len}.\n");

    let mut s = String::from("kaso");
    change(&mut s);

    let mut x = 10;
    let mut y = 20;
    std::mem::swap(&mut x, &mut y);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_reference(s: &str) -> usize {
    s.len()
    // the function goes out of scope but since it doesn't own 's', it stays in scope.
}

fn change(s: &mut String) {
    s.push_str("deep");
}

fn _owner_so_change(mut s: String){
    s.push_str("deep");
}