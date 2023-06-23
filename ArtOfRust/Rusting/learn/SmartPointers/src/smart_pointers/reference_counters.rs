use std::rc::Rc;

mod reference_counter {
    #[derive(Clone, Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use std::rc::Rc;

    use crate::smart_pointers::reference_counters::reference_counter::List::{Cons, Nil};

    pub fn Main() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));

        let mut l = a;

        while true {
            match l.as_ref() {
                Nil => { 
                    println!("Empty!");
                    break;
                },
                Cons(i, r) => {
                    println!("Values: {}", i);
                    l = r.to_owned();
                }
            };
        };

    }
}

mod Source {
    use std::rc::Rc;
    use std::rc::Weak;
    use std::cell::RefCell;

    struct Owner {
        name: String,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    pub fn Main() {
        let gadget_owner: Rc<Owner> = Rc::new(Owner {
            name: "Gadget Man".to_string(),
        });

        let gadget1 = Gadget {
            id: 1,
            owner: gadget_owner.clone(),
        };

        let gadget2: Gadget = Gadget { id: 2, owner: Rc::clone(&gadget_owner) };

        drop(gadget_owner);

        println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
        println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    }
}

mod SourceRefCell {
    use std::borrow::Borrow;
    use std::rc::Rc;
    use std::rc::Weak;
    use std::cell::RefCell;


    struct Owner {
        name: String,
        gadgets: RefCell<Vec<Weak<Gadget>>>,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    pub fn Main() {
        let gadget_owner: Rc<Owner> = Rc::new(
            Owner {
                name: String::from("Gadget Man"),
                gadgets: RefCell::new(vec![]),
            }
        );

        let gadget1 = Rc::new(
            Gadget {
                id: 1,
                owner: Rc::clone(&gadget_owner),
            }
        );

        let gadget2 = Rc::new(
            Gadget {
                id: 2,
                owner: Rc::clone(&gadget_owner),
            }
        );

        {
            let mut gadgets = gadget_owner.gadgets.borrow_mut();
            gadgets.push(Rc::downgrade(&gadget1));
            gadgets.push(Rc::downgrade(&gadget2));
        }

        for gadget_weak in  gadget_owner.gadgets.borrow().iter() {
            let gadget = gadget_weak.upgrade().unwrap();
            println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
        }

    }


}

pub fn  Main() {
    use reference_counter;
    reference_counter::Main();
    Source::Main();
}