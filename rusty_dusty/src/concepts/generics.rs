use std::fmt::Display;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U: Display> {
    x: T,
    y: U,
}

impl<T, U: Display> Point<T, U> {
    fn x(&self) -> &T {
        println!("Point y: {}", self.y);
        &self.x
    }
}

pub fn demo() {
    println!("Generics");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let _integer_and_float = Point { x: 5, y: 4.0 };
    println!("Point x: {}\n", _integer_and_float.x());
}
