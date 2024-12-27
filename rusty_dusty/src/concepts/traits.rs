trait Animal {
    fn make_sound(&self) -> String;
    
    fn description(&self) -> String {
        String::from("Just a regular animal")
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
    
    fn description(&self) -> String {
        format!("A dog named {}", self.name)
    }
}

fn print_animal_sound<T: Animal>(animal: T) {
    println!("{}", animal.make_sound());
}

trait Walkable {
    fn walk(&self) -> String;
}

fn animal_activities<T>(animal: T) 
where 
    T: Animal + Walkable,
{
    println!("{}", animal.make_sound());
    println!("{}", animal.walk());
}

fn create_animal_chorus(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("{}", animal.make_sound());
    }
}

// Derivable Traits
#[derive(Debug, Clone, PartialEq)]
struct Cat {
    name: String,
    age: u8,
}

trait Container {
    type Item;
    fn get(&self) -> Option<&Self::Item>;
    fn insert(&mut self, item: Self::Item);
}

// Generic Traits
trait Comparable<T> {
    fn compare(&self, other: &T) -> std::cmp::Ordering;
}

// Supertraits
trait Flying: Animal {
    fn fly(&self) -> String;
}

struct CountDown {
    count: u32,
}

impl Iterator for CountDown {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            Some(self.count)
        }
    }
}

struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Fahrenheit> for Celsius {
    fn from(fahrenheit: Fahrenheit) -> Self {
        Celsius((fahrenheit.0 - 32.0) * 5.0 / 9.0)
    }
}

impl Default for Dog {
    fn default() -> Self {
        Dog {
            name: String::from("Unnamed Dog"),
        }
    }
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("{} is leaving the scope!", self.name);
    }
}

pub fn demo() {
    println!("Traits");

    let dog = Dog { name: String::from("Buddy") };
    println!("{}", dog.make_sound());
    println!("{}", dog.description());
    
    print_animal_sound(dog);
    
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: String::from("Rex") }),
        Box::new(Dog { name: String::from("Max") })
    ];
    create_animal_chorus(animals);
    
    let countdown = CountDown { count: 5 };
    for i in countdown {
        println!("{}", i);
    }
    
    let fahrenheit = Fahrenheit(98.6);
    let _celsius: Celsius = fahrenheit.into();
    println!("Celsius: {}", _celsius.0);
    
    // Default trait usage
    let _default_dog = Dog::default();
}