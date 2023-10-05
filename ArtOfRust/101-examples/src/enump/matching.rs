pub mod simple {
    enum Coin {
        Penny, 
        Nickel,
        Dime,
        Quarter
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }

    pub fn main() {
        let paisa: u8 = value_in_cents(Coin::Nickel);

        println!("{}", paisa);
    }
}


pub mod options {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }
    pub fn main() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

    }
}

pub mod exhaustive {

    pub fn main() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
    
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {
            println!("Value: {}", num_spaces)
        }
    
    }


    
}