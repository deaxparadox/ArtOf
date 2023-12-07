# The Iterator Trait and the next Method

All iterators implements a trait named `Iterator` that is defined in the standard library. The definition of the trait looks like thid:

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

```