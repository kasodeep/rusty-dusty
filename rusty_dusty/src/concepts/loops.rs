pub fn loops(){
    println!("Loops!");

    println!("for loop:");
    for num in 1..=5 {
        print!("{num} ");
    }
    
    println!();
    println!("while loop:");
    
    let mut counter = 1;
    while counter <= 10 {
        print!("{counter} ");
        counter += 1;
    }

    println!();
    println!("loop loop:");

    let mut count = 0;
    loop {
        print!("{count} ");
        count += 1;

        // break keyword.
        if count == 5 {
            break;
        }        
    }

    println!();
}