# The Option Enum and Its Advantages Over Null Values

The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.

Rust doesn’t have the `null` feature that many other languages have. 

- `Null` is a value that means there is no value there. 
- In languages with `null`, variables can always be in one of two states: `null` or `not-null`.


As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`


```rs
enum Option<T> {
    None,
    Some(T),
}

```