use std::cmp::Ordering;

/// A trait representing general animal behavior.
trait Animal {
    fn name(&self) -> &str;

    fn speak(&self) -> String;

    /// Default implementation for describing an animal.
    fn description(&self) -> String {
        format!("{} is a mysterious creature.", self.name())
    }
}

/// A trait for animals that can walk.
trait Walkable {
    fn walk(&self) -> String;
}

/// A trait for animals that can fly; it depends on being an Animal.
trait Flyable: Animal {
    fn fly(&self) -> String;
}

/// A dog that can walk but not fly.
#[derive(Debug, Clone)]
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        "Woof!".to_string()
    }

    fn description(&self) -> String {
        format!("{} is a loyal dog.", self.name)
    }
}

impl Walkable for Dog {
    fn walk(&self) -> String {
        format!("{} walks on 4 legs.", self.name)
    }
}

/// A bird that can both walk and fly.
#[derive(Debug)]
struct Bird {
    name: String,
}

impl Animal for Bird {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        "Tweet!".to_string()
    }
}

impl Walkable for Bird {
    fn walk(&self) -> String {
        format!("{} hops around.", self.name)
    }
}

impl Flyable for Bird {
    fn fly(&self) -> String {
        format!("{} soars through the sky!", self.name)
    }
}

/// A generic activity function for all walkable animals.
fn perform_walk<T: Animal + Walkable>(animal: &T) {
    println!("{}", animal.description());
    println!("{}", animal.walk());
}

/// Accept any animal (dynamic dispatch via trait objects).
fn print_animal_sounds(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("{} says: {}", animal.name(), animal.speak());
    }
}

/// Trait with associated type
trait Container {
    type Item;
    fn insert(&mut self, item: Self::Item);
    fn retrieve(&self) -> Option<&Self::Item>;
}

/// A toy box that stores one toy.
struct ToyBox<T> {
    toy: Option<T>,
}

impl<T> Container for ToyBox<T> {
    type Item = T;

    fn insert(&mut self, item: T) {
        self.toy = Some(item);
    }

    fn retrieve(&self) -> Option<&Self::Item> {
        self.toy.as_ref()
    }
}

/// Trait for comparing items (generic)
trait Comparable<T> {
    fn compare(&self, other: &T) -> Ordering;
}

/// Temperature types with `From` and custom trait.
#[derive(Debug)]
struct Celsius(f64);
#[derive(Debug)]
struct Fahrenheit(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

impl Comparable<Celsius> for Celsius {
    fn compare(&self, other: &Celsius) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

/// Showcasing Drop and Default traits.
impl Drop for Dog {
    fn drop(&mut self) {
        println!("{} has gone to sleep. 🐶", self.name);
    }
}

impl Default for Dog {
    fn default() -> Self {
        Dog {
            name: "Default Dog".to_string(),
        }
    }
}

/// Demo function to run everything
pub fn trait_demo() {
    println!("--- Animal Traits Demo ---");

    let dog = Dog {
        name: "Buddy".into(),
    };
    let bird = Bird {
        name: "Tweety".into(),
    };

    perform_walk(&dog);
    perform_walk(&bird);
    println!("{}", bird.fly());

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(dog.clone()),
        Box::new(bird),
        Box::new(Dog::default()),
    ];
    print_animal_sounds(animals);

    println!("\n--- Container Trait ---");
    let mut toy_box = ToyBox { toy: None };
    toy_box.insert("Rubber Bone");
    if let Some(toy) = toy_box.retrieve() {
        println!("The toy in the box is: {}", toy);
    }

    println!("\n--- Temperature Conversion ---");
    let f = Fahrenheit(100.0);
    let c: Celsius = f.into();
    println!("100°F in Celsius is {:.2}°C", c.0);

    let c2 = Celsius(37.0);
    match c.compare(&c2) {
        Ordering::Less => println!("Cooler than normal."),
        Ordering::Equal => println!("Just right."),
        Ordering::Greater => println!("Hot!"),
    }

    println!("\n--- Drop Trait Demonstration ---");
    let _temp_dog = Dog {
        name: "Shadow".into(),
    }; // Will trigger `drop()` at the end of scope
}
