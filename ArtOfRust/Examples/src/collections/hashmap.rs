pub mod hs {
    use std::collections::HashMap;



    pub fn main() {
        let mut scores = HashMap::new();
        
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
    
        // println!("{}", score);
    
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }
}


pub mod hash_map_and_ownership {
    use std::{collections::HashMap, hash::Hash};
    
    pub fn main() {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();

        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point, try using them and
        // see what compiler error you get!
    }
}

pub mod updating_hash_map {
    use std::{collections::HashMap};

    pub fn main() {
        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
    
        println!("{:?}", scores);
    }
    
    pub fn adding_key_value_if_not_exist() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
    
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
    
        println!("{:?}", scores);
    
    }

    pub fn update_value_based_on_old_value() {
        let text = "hello world wonderfull world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}