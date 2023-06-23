mod traiting;
mod generic;

use traiting as trt;

mod Generic {
    use std::{marker::PhantomData, task::Context};
    #[allow(dead_code, non_snake_case)]
    pub struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        #[allow(unused)]
        fn new(value: T) -> Self {
            Self { value }
        }
    }

    #[allow(non_snake_case, dead_code)]
    pub fn ContainerMain() {
        // let str_container: Container<Option<String>> = Container {
        //     value: None,
        // };

        // let str_container: Container<Option<String>> = Container::new(None);

        let str_container = Container::<Option<String>>::new(None);

        match str_container.value {
            Some(value) => println!("{}", value),
            None => println!("Empty"),
        }
    }

    #[derive(Clone)]
    pub struct ListItem<T>
    where
        T: Clone,
    {
        data: Box<T>,
        next: Option<Box<ListItem<T>>>,
    }

    pub enum Recursive<T> {
        Next(Box<Recursive<T>>),
        Boxed(Box<T>),
        Optional(Option<T>),
    }

    pub enum NextNode<T> {
        Next(Box<ListNode<T>>),
        End,
    }

    pub struct ListNode<T> {
        data: Box<T>,
        next: NextNode<T>,
    }

    struct Dog<Breed> {
        name: String,
        breed: PhantomData<Breed>,
    }

    pub struct Labrador {}
    pub struct Retriever {}
    pub struct Poodle {}
    pub struct Dachshunds {}

    impl Dog<Labrador> {
        fn breed_name(&self) -> &'static str {
            "labrador"
        }
    }

    impl Dog<Retriever> {
        fn breed_name(&self) -> &'static str {
            "Retriever"
        }
    }

    impl Dog<Poodle> {
        fn breed_name(&self) -> &'static str {
            "poodle"
        }
    }

    impl Dog<Dachshunds> {
        fn breed_name(&self) -> &'static str {
            "duchshunds"
        }
    }

    pub fn DogMain() {
        let my_poodle: Dog<Poodle> = Dog {
            name: "Jeffrey".into(),
            breed: PhantomData,
        };

        let my_retriever: Dog<Retriever> = Dog {
            name: "Jeffrey".into(),
            breed: PhantomData,
        };

        println!("{}", my_poodle.breed_name().to_string());
        println!("{}", my_retriever.breed_name().to_string());
    }
}

fn main() {
    println!("Hello, world!");
    // Generic::ContainerMain();
    // Generic::DogMain();
    // trt::square_and_rectangle();
    trt::main();
}
