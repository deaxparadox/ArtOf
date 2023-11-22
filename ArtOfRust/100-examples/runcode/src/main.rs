use std::process::exit;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}


enum Numbers {
    One(i32),
    Two(i32),
    Three(i32)
}
fn main() {
    // let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    // println!("{list:?}");

    let one = Numbers::One(1);

    if let Numbers::One(value) = one {
        println!("Everything Ok, {}", value);
    }

    exit(0);
}
