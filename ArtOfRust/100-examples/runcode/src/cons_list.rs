enum List {
    Cons(i32, List),
    Nil,
}

use crate::cons_list::List::{Cons, Nil};

pub fn run() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));

}