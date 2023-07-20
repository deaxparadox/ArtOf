# String Slices

A *string slice* is a refrence to part of a `String`, and it looks like this:

```rs
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```