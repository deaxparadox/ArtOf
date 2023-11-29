use std::process::exit;
use std::rc::Rc;

mod closures;
mod lifetimes;
mod reference_counters;

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
fn number() {
    let one = Numbers::One(1);

    if let Numbers::One(value) = one {
        println!("Everything Ok, {}", value);
    }
}

fn display_name(a0: Rc<&str>) {
    println!("Name is: {a0}");
}



fn generate_and_return_string<'a, 'b>() -> &'a &'b str {
    return &"Nitish";
}

fn main() {


closures::run();

}
