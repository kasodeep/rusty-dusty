# Rust Project Structure Guide

## Table of Contents
- [Packages, Crates, and Modules](#packages-crates-and-modules)
- [Binary vs Library Projects](#binary-vs-library-projects)
- [Module System](#module-system)
- [Dependencies Management](#dependencies-management)

## Packages, Crates, and Modules

### Package
- A package is the highest level of organization in Rust projects
- Contains one or more crates
- Must have a `Cargo.toml` file defining package metadata and dependencies
- Can have multiple binary crates but only one library crate

### Crates
- Fundamental compilation unit in Rust
- Tree of modules that produces a library or executable
- Two types:
  - Binary crates: Compile to executables (`src/main.rs`)
  - Library crates: Code intended for other programs (`src/lib.rs`)

### Modules
- Control organization, scope, and privacy of paths
- Created using `mod` keyword
- Can be nested
- Follow filesystem structure:
  ```
  src/
  ├── lib.rs
  ├── main.rs
  └── module_name/
      ├── mod.rs
      └── submodule.rs
  ```

## Binary vs Library Projects

### Binary Project Structure
```
my_project/
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── main.rs
    ├── lib.rs (optional)
    └── bin/
        └── additional_binaries.rs
```

### Library Project Structure
```
my_library/
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── lib.rs
    └── modules/
        ├── mod.rs
        └── feature.rs
```

## Module System

### Module Declaration
```rust
// In lib.rs or main.rs
mod module_name;    // Loads from module_name.rs or module_name/mod.rs
pub mod public_module;  // Public module

// Nested modules
mod outer_module {
    pub mod inner_module {
        pub fn function() {}
    }
}
```

### Visibility Rules
- Items are private by default
- `pub` keyword makes items public
- `pub(crate)` for crate-level visibility
- `pub(super)` for parent module visibility
- `pub(in path)` for specific module path visibility

## Dependencies Management

### Cargo.toml Structure
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = "1.0"

[dev-dependencies]
criterion = "0.3"

[build-dependencies]
cc = "1.0"
```

### Workspace Structure
```toml
[workspace]
members = [
    "package1",
    "package2"
]

[workspace.dependencies]
shared_dependency = "1.0"
```