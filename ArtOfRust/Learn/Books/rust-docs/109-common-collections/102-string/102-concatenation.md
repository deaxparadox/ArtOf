# Concatenation with the `+` operator or the format! macro

You'll want to combine two existing strings. One way to do so is to use the `+` operator

```rs

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

```


The string `s3` will contain `Hello, world!`. The reason `s1` is no longer valid after the addition, and the reason we used a reference to `s2`, has to do with the signature of the method that’s called when we use the `+` operator. The `+` operator uses the add method, whose signature looks something like this:

```rs
fn add(self, s: &str) -> String {
```

- In the standard library, you'll see `add` defined using generics and associated types. Here, we’ve substituted in concrete types, which is what happens when we call this method with `String` values.

- First, `s2` has an `&`, meaning that we’re adding a *reference* of the second string to the first string. This is because of the `s` parameter in the add function: we can only add a `&str` to a `String`; we can’t add two `String` values together. But wait—the type of `&s2` is `&String`, not `&str`, as specified in the second parameter to add.

- The reason we’re able to use `&s2` in the call to add is that the compiler can *coerce* the `&String` argument into a `&str`. When we call the `add` method, Rust uses a *deref coercion*, which here turns `&s2` into `&s2[..]`. Because `add` does not take ownership of the `s` parameter, `s2` will still be a valid `String` after this operation.

- Second, we can see in the signature that `add` takes ownership of `self`, because `self` does not have an `&`. This means `s1`  will be moved into the `add` call and will no longer be valid after that. So although `let s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this statement actually takes ownership of `s1`, appends a copy of the contents of `s2`, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.


If we need to concatenate multiple strings, the behaviour of the `+` operator gets unwieldy:

```rs

let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

```

- At this point, s will be tic-tac-toe. 

For more complicated string conbining, we can instead use the `format!` macro:

```rs

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

```

- This code also sets `s` to `tic-tac-toe`. 
- The `format!` macro works like `println!`, but instead of printing the output to the screen, it returns a `String` with the contents. The version of the code using `format!` is much easier to read, and the code generated by the `format!` macro uses references so that this call doesn’t take ownership of any of its parameters.