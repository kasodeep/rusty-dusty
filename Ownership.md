# Ownership

    Ownership is a set of rules that govern how a Rust program manages memory. 
    All programs have to manage the way they use a computer’s memory while running.

## Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Transfer of Ownership

- Assigning value to another variable.
- Passing value to a function.
- Returning value from a function.

## String type

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String.

## References and Borrowing

Borrowing gives temporary access to a resource without transferring ownership.

A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

## The Rules of References

Let’s recap what we’ve discussed about references:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.