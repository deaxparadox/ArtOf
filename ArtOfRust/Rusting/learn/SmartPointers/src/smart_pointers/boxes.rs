mod ListMod {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use crate::smart_pointers::boxes::ListMod::List::{Cons, Nil};

    pub fn ListMain() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        // match list {
        //     Nil => println!("Empty!"),
        //     Cons(i, l) => {
        //         println!("Value: {}", i);
        //         match *l {
        //             Nil => println!("Empty!"),
        //             Cons(i, l) => {
        //                 println!("Value: {}", i);
        //                 match *l {
        //                     Nil => println!("Empty!"),
        //                     Cons(i, l) => {
        //                         println!("Value: {}", i);
        //                         match *l {
        //                             Nil => println!("Empty!"),
        //                             Cons(i, l) => {
        //                                 println!("Value: {}", i);

        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     },
        // }

        let mut l = list;

        while true {
            match l {
                Nil => {
                    println!("Empty!...");
                    break;
                }
                Cons(i, c) => {
                    println!("Value: {}", i);
                    l = *c;
                }
            }
        }
    }
}

mod CustomSP {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`", self.data);
        }
    }

    pub fn Main() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.")
    }
}

mod Example {
    use std::mem;

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    // A Rectangle can be specified by where its top left and bottom right coreners are is space
    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point, 
        bottom_right: Point,
    }

}

use CustomSP as CSP;
use ListMod::ListMain;

pub fn Main() {
    let b = Box::new(5);
    let b = Box::new("Nitish".to_string());
    println!("b = {}", b);

    ListMain();
    CSP::Main();
}
