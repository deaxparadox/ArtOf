# Generic Lifetimes in Functions

We’ll write a function that returns the longer of two string slices. This function will take two string slices and return a single string slice. After we’ve implemented the longest function, the following code should print The longest string is abcd.

```rs
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```