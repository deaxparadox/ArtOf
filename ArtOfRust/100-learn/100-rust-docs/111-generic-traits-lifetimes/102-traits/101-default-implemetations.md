# Default Implementations

Sometimes it's useful to have default behavior for some or all the methods in a trait method of requiring implementations for all methods on every type. Then as we, implement the trait on a particular type, we can keep or override each method's default behavior.

Next, we specify a default string for the `summarize` method of the `Summary` trait instead of only defining the method signature:

```rs
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)");
    }
}
```

To use a default implementation to summarize instances of `NewsArticle`, we specify an empty `impl` block with `impl Summary for NewsArticle {}`.

Even though we're no longer defining the `summarize` method on `NewsArticle` directly, we've provided a default implemenetation and specified that `NewsArticle` implements the `Summary` trait. As a result, we can still call the summarize method on an instance of `NewsArticle`, like this:

```rs
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

```


This code prints `New article available! (Read more...)`.

Creating a default implementation doesn't require us to change anything about the implementation of `Summary` on `Tweet`. The reason is that the syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn't have a default implementation.


Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it. For example, we could define the `Summary` trait to have a `summarize_author` method whose implementation is required, and then define a `summarize` method that has a default implementation that calls the `summarize_author` method:

```rs
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

```

To use this version of  `Summary`, we only need to define `summarize_author` when we implement the trait on a type:

```rs
impl Summary for Tweet {
    fn summarize_tweet(&self) -> String {
        format!("@{}", self.username)
    }
}
```