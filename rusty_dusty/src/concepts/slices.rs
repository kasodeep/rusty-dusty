pub fn slices(){
    println!("Slices: No ownership and mutability!!");

    let arr = [1, 2, 3, 4];
    let arr_slice = &arr[1..3];

    println!("Slice: {:?}", arr_slice);
    println!();
}