# Exposing Paths with the pub Keyword

Let’s return to the error that told us the hosting module is private. We want the `eat_at_restaurant` function in the parent module to have access to the `add_to_waitlist` function in the child module, so we mark the hosting module with the `pub` keyword.

Filename: src/lib.rs

```rs
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

###### Listing 7-5: Declaring the hosting module as pub to use it from eat_at_restaurant

Unfortunately, the code in Listing 7-5 still results in an error, as shown in Listing 7-6.

```bash
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> src/lib.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors

```

###### Listing 7-6: Compiler errors from building the code in Listing 7-5

What happened? Adding the `pub` keyword in fron of `mod hosting` makes the module public. With this changed, if we can access `front_of_house`, we can access `hosting`. But the *contents* of `hosting` is still private; makiing the module public doesn't make its contents public. The `pub` keyword on a module only lets code in its ances