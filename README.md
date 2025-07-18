# Rusty-Dusty

## Rust Overview

- Rust is a systems programming language designed for performance and safety. 
- It offers features that make it particularly well-suited for applications requiring concurrency, low-level control, and memory safety.

### Key Features

- **Speed and Efficiency**: Rust provides a fast and memory-efficient language with zero-cost abstractions.
- **Ownership Model**: Rust's ownership model ensures memory safety and prevents data races.
- **Reliability**: Compile-time checks catch bugs early, resulting in highly reliable software.
- **Concurrency**: Built-in support for concurrent programming.

### Benefits

- **Parallel Programming**: Simplifies building parallel applications.
- **Ease of Deployment**: Produces executables with minimal runtime dependencies.
- **Real-Time Systems**: Ideal for real-time and embedded systems due to its performance.

---

```bash
# To check the version.
rustc --version

# To create a new project.
cargo new rust_first

# To compile the project.
cargo build

# To compile a single file.
rustc file.rs

# To run the project. (Single Step)
cargo run

# To create library crate.
cargo new name --lib

# Run all tests
cargo test

# Run specific test file
cargo test --test integration_test

# Run tests with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored

# Run with release profile.
cargo build --release

# To add a new crate (dependency)
cargo add <package-name>

# To create the documentation
cargo doc
```
---

## Variables

- Rust emphasizes `immutability` by default, ensuring data `safety` and `predictability`.
- However, variables can be made `mutable` for cases requiring value reassignment.
- The `let` keyword is used to declare variables, while additional keywords like `mut` modify their behavior.
- We can declare a variable to `change` it's data-type or value, called `shadowing`.

### Key Characteristics

1. **Immutable by Default**: Encourages functional programming principles.
2. **Explicit Mutability**: Requires deliberate use of `mut` for mutable variables.
3. **Static Typing**: Rust ensures type safety at compile time, minimizing runtime errors.

---

## Data Types

- Rust provides a robust type system to ensure safety and performance.
- Data types in Rust can be categorized as scalar or compound.

### Scalar Types

- **Integer**: Represents whole numbers with varying sizes and signs (e.g., signed and unsigned).
- **Floating-Point**: Represents numbers with fractional parts and adheres to IEEE-754 standards.
- **Boolean**: Denotes logical values, `true` or `false`.
- **Character**: Represents Unicode scalar values, supporting a wide range of symbols and characters.

### Compound Types

- **Arrays**: Fixed-size collections of elements of the same type, providing `contiguous` memory storage.
- **Tuples**: Fixed-size `ordered` collections that can contain `multiple` data types, ideal for grouping related values.

### Slices

- Slices allow efficient `referencing` of subsets of data within collections.
- They are particularly useful for working with arrays without `copying` data, promoting memory `efficiency`.

### Strings

Rust offers two primary string types:

1. **String**: An owned, `growable` string stored on the heap, allowing `dynamic` modification.
2. **&str**: A borrowed, `immutable` string slice, typically used for efficient, `read-only` operations.

---

## Functions

- Functions in Rust are the building blocks of code, enabling modular and reusable design.
- Defined using the `fn` keyword, functions enforce strict type annotations for parameters and return values, ensuring clarity and correctness.
- Functions that don't return any value return unit tuple ().
- The ! symbol means the function doesn't return anything.

### Characteristics

1. **Type Safety**: Every parameter and return value must have an explicitly defined type.
2. **Reusable Logic**: Functions promote modular design and reduce code duplication.
3. **Pure Functions**: Rust encourages writing functions without side effects, enhancing predictability.

---

## Error Handling

    Rust provides robust mechanisms for error handling, avoiding traditional exceptions in favor of explicit types like `Result` and `Option`.

### Recoverable Errors

    These are errors that can be addressed by the program, allowing it to continue execution. Rust uses the `Result` type to encapsulate outcomes that can either succeed or fail.

### Unrecoverable Errors

These errors signify critical issues that prevent the program from continuing. Rust handles such scenarios using the `panic!` macro, which immediately terminates the program.

### Error Propagation

Error propagation allows errors to be passed up the call stack to be handled at higher levels. Techniques include:

1. **The `?` Operator**: Simplifies propagating errors in functions.
2. **Custom Error Types**: Enables detailed and domain-specific error reporting.

---

## Option Enum

    The `Option` type is used to represent optional values. It explicitly handles cases where data may be absent, ensuring safe and predictable behavior.

- **Explicit Null Safety**: Eliminates the risk of null pointer exceptions.
- **Variants**: Represents values as either `Some(value)` or `None`.
- **Seamless Integration**: Works with other Rust features like pattern matching for elegant handling of optional data.

---

## Profiling Tools

- Perf: Generate flamegraphs to collect and analyze performance.
```bash
perf record -g ./name
perf script | ./flamegraph.p1 > flamegraph.svg
```
- cargo-profiler: Simplifies by automating the setup and execution of performance tests.