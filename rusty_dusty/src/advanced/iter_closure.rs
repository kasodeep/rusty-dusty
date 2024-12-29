fn closure_basics() {
    // basic closure.
    let add = |x, y| x + y;
    assert_eq!(add(2, 3), 5);

    // closure capturing environment.
    let multiplier = 2;
    let multiply = |x| x * multiplier;
    assert_eq!(multiply(5), 10);

    // closure with explicit type annotations.
    let typed_closure: fn(i32) -> i32 = |x| x * x;
    assert_eq!(typed_closure(4), 16);

    // mutable closure.
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    assert_eq!(increment(), 1);
    assert_eq!(increment(), 2);
}

fn closure_traits() {
    // fn trait - no capture.
    let _add: fn(i32, i32) -> i32 = |x, y| x + y;
    
    // fnmut trait - mutable capture.
    let mut vec = vec![1, 2, 3];
    let mut mutate = || vec.push(4);
    mutate();
    assert_eq!(vec, vec![1, 2, 3, 4]);
    
    // fnonce trait - takes ownership.
    let vec = vec![1, 2, 3];
    let consume = move || {
        let sum: i32 = vec.iter().sum();
        sum
    };
    assert_eq!(consume(), 6);
}

fn iterator_creation() {    
    // creating custom iterator.
    struct CountTo(u32);
    
    impl Iterator for CountTo {
        type Item = u32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == 0 {
                None
            } else {
                self.0 -= 1;
                Some(self.0 + 1)
            }
        }
    }
    
    let count = CountTo(3);
    let collected: Vec<u32> = count.collect();
    assert_eq!(collected, vec![3, 2, 1]);
}

fn iterator_operations() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // map
    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    
    // filter
    let even: Vec<&i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .collect();
    assert_eq!(even, vec![&2, &4]);
    
    // fold
    let sum = numbers.iter()
        .fold(0, |acc, x| acc + x);
    assert_eq!(sum, 15);
    
    // chain
    let more_numbers = vec![6, 7];
    let combined: Vec<&i32> = numbers.iter()
        .chain(more_numbers.iter())
        .collect();
    assert_eq!(combined, vec![&1, &2, &3, &4, &5, &6, &7]);
    
    // enumerate
    let enumerated: Vec<(usize, &i32)> = numbers.iter()
        .enumerate()
        .collect();
    assert_eq!(enumerated[0], (0, &1));
}

fn advanced_iterators() {
    // zip
    let xs = vec![1, 2, 3];
    let ys = vec![4, 5, 6];
    let zipped: Vec<(i32, i32)> = xs.into_iter()
        .zip(ys.into_iter())
        .collect();
    assert_eq!(zipped, vec![(1, 4), (2, 5), (3, 6)]);
    
    // flat_map
    let nested = vec![vec![1, 2], vec![3, 4]];
    let flattened: Vec<i32> = nested.into_iter()
        .flat_map(|x| x)
        .collect();
    assert_eq!(flattened, vec![1, 2, 3, 4]);
    
    // take and skip
    let numbers: Vec<i32> = (1..=10).collect();
    let subset: Vec<i32> = numbers.into_iter()
        .skip(2)
        .take(3)
        .collect();
    assert_eq!(subset, vec![3, 4, 5]);
}

// demo function that showcases all the functionality
pub fn demo() {
    println!("\nRunning closure examples...");
    closure_basics();
    closure_traits();
    
    println!("\nRunning iterator examples...");
    iterator_creation();
    iterator_operations();
    advanced_iterators();
    
    println!("\nAll examples completed successfully!\n");
}