pub mod single {
    use std::cell::RefCell;
    use std::rc::Rc;

    type SingleLink = Option<Rc<RefCell<Node>>>;

    #[allow(unused)]
    #[derive(Clone)]
    struct Node {
        value: i32,
        next: SingleLink,
    }

    impl Node {
        #[allow(unused)]
        // A nice and short way of creating a new node
        fn new(value: i32) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                value: value,
                next: None,
            }))
        }
    }

    #[allow(unused)]
    struct TransactionLog {
        head: SingleLink,
        tail: SingleLink,
        pub length: u64,
    }

    impl TransactionLog {
        #[allow(unused)]
        pub fn new_empty() -> TransactionLog {
            TransactionLog {
                head: None,
                tail: None,
                length: 0,
            }
        }

        #[allow(unused)]
        pub fn append(&mut self, value: i32) {
            let new = Node::new(value);
            match self.tail.take() {
                Some(old) => old.borrow_mut().next = Some(new.clone()),
                None => self.head = Some(new.clone()),
            };
            self.length += 1;
            self.tail = Some(new);
        }

        #[allow(unused)]
        pub fn pop(&mut self) -> Option<i32> {
            self.head.take().map(|head| {
                if let Some(next) = head.borrow_mut().next.take() {
                    self.head = Some(next);
                } else {
                    self.tail.take();
                }
                self.length -= 1;
                Rc::try_unwrap(head)
                    .ok()
                    .expect("Something is terribly wrong")
                    .into_inner()
                    .value
            })
        }
    }
}


pub mod double {
    use std::cell::RefCell;
    use std::rc::Rc;
}