pub mod by_example {
    use core::panic;

    #[allow(unused)]
    fn given_drink(drink: Option<&str>) {
        match drink {
            Some("lemonade") => println!("Yuck! Too sugary."),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No drink? Oh well."),
        }
    }

    #[allow(unused)]
    fn drink(drink: Option<&str>) {
        let inside = drink.unwrap();
        if inside == "lemonade" { panic!("AAaaaaa!!!!"); }
        println!("I love {}s!!!!", inside);
    }

    #[allow(unused)]
    pub fn main() {
        let water = Some("water");
        let lemonade = Some("lemonade");
        let void = None;

        given_drink(water);
        given_drink(lemonade);
        given_drink(void);

        let coffee = Some("coffee");
        let nothing = None;

        drink(coffee);
        drink(nothing);

    }
}

pub mod by_example_2 {
    #[allow(unused)]
    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        // If `current_age` is `None`, this returns `None`.
        // If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
        let next_age: u8  = current_age? + 1;
        Some(format!("Next year I will be {}", next_age))
    }

    #[allow(unused)]
    struct Person {
        job: Option<Job>,
    }
    
    #[allow(unused)]
    #[derive(Clone, Copy)]
    struct Job {
        #[allow(unused)]
        phone_number: Option<PhoneNumber>,
    }

    #[allow(unused)]
    #[derive(Clone, Copy)]
    struct PhoneNumber {
        #[allow(unused)]
        area_code: Option<u8>,
        #[allow(unused)]
        number: u32,
    }

    impl Person {
        
    }
}

#[allow(unused)]
pub fn main() {
    println!("This is option main command.")
}