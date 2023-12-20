# Enabling Recursive Types with Boxes

A value of *recursive type* can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to known how much space a type takes up. Howevere, the nesting of values of recursive types could theoretical continue infinitely, so Rust can't known how much space the value needs. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

As an example of a recursive type, let's explore the *cons list*. This is a data type commonly found in functional programming languages. The cons list type we'll define is straightforward except for the recursion; therefore, the concepts in the example we'll work with will be usefull any time you get into more complex situations involving recursive types.

### More Information About the Cons List