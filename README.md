## Rusty-Dusty

### Rust
    The language is fast and memory efficient, featuring a robust system and ownership model.
    Rust's strict compile time checks enforce correct code, minimizing bugs and boosting software reliability.
    Rust empowers developers to write safe and concurrent codes.

### Benefits Of Rust
    Parallel Programming, Ease of Deployment, Real Time Systems.

### Installation
    Installed it on the wsl system in Windows.
    Downloaded different vscode extensions as helpers.

### Terminal Commands
```shell
# To check the version.
rustc --version

# To create a new project.
cargo new rust_first

# To compile the project.
cargo build

# To run the project. (Single Step)
cargo run
```

### Variables

1. By default, variables in Rust are immutable.
2. We need to use the `mut` keyword for making the varibale mutable.
3. `let` keyword is used to declare the variables in Rust.

### Datatypes

1. Data types define the nature of data that variables can store.
2. Ensures that the operations performed are safe and efficient.
3. Specifying data types lets Rust optimize memory and CPU usage.

#### Scalar Types

* Integer: Represents the whole numbers.
* Floating-Point: Represents the numbers with fractional parts.
* Boolean: Represents true or false values.
* Character: Represents a single unicode scalar value.

#### Compound Types

* Arrays: A fixed size collection of elements of same types.
* Tuples: A fixed size ordered and immutable collection of elements of different types.

#### Slices
    Slices are references to a continous sequences of elements in a collection and do not have ownersip.   

#### Strings
    Rust has two main string types: String and &str.    
    An owned, growable, and mutable string type. 
    It resides in the heap, meaning it owns the memory it occupies and is responsible for deallocating it.

#### Functions

```rust
fn name(param: Type) -> ReturnType {
    // Function Body
}
```