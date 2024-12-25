pub fn data_types(){
    println!("Data-Types");
    
    let arr = [1, 2, 3, 4, 5];
    let tup = ("Deep", 17, "2004");

    let float_val = 10.7;
    let is_alive = true;

    println!("float = {}, bool = {}", float_val, is_alive);
    println!("arr = {:?}, tuple = {:?}", arr, tup);

    println!();
}