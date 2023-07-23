# Defining and Instantiating Structs

- The pieces of a struct can be of different types.
- in struct you'll name each piece of data so it's clear what the values mean.


# Define a `struct`

To define a structure we use the `struct` keyword and name the entire struct.

- then inside curly brackets, we define the names and types of the pieces of data, which we call fields.

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

```