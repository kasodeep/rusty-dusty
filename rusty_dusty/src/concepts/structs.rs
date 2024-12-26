#[derive(Debug)]
struct Point {
    pub x: f64,
    y: f64,
}

// unit struct (useful for trait implementations)
struct AlwaysEqual;

// tuple struct.
struct Color(u8, u8, u8);

// struct implementation.
impl Point {
    // associated function (static method)
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // method with &self.
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // method with &mut self.
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

// deriving common traits.
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

// struct with default values.
impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::from("John Doe"),
            age: 30,
        }
    }
}

pub fn main() {
    println!("Structs");

    // standard initialization.
    let point = Point { x: 1.0, y: 2.0 };
    
    // struct update syntax.
    let point2 = Point { x: 5.0, ..point };
    println!("New Point: {:#?}", point2);

    // calling the methods.
    let mut new_point = Point::new(10.2, 20.2);
    println!("Distance from (0, 0): {}", new_point.distance_from_origin());
    new_point.move_by(10.1, 20.2);

    // destructuring.
    let Point { x, y } = point;
    println!("Destructed, x = {}, y = {}", x, y);
    
    // tuple struct construction
    let black = Color(0, 0, 0);
    
    // pattern matching on tuple structs
    let Color(_r, _g, _b) = black;


    let _equal = AlwaysEqual;
    println!()
}