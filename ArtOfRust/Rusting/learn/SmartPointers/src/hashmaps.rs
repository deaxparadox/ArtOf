#[allow(unused)]
mod by_example {
    use std::collections::HashMap;

    
    pub fn call(number: &str) -> &str {
        match number {
            "798-1364" => {
                "We're sorry, the call cannot be completed as dailed.\
                Please hang up and try again."
            },
            "645-7689" => {
                "Hello, this is Mr. Awesome's pizza. My name is Fred.\
                What can I get for you today?"
            },
            _ => "Hi! Who is this again?"
        }
    }

    pub fn main() {
        let mut contacts = HashMap::new();
        contacts.insert("Daniel", "798-1364");
        contacts.insert("Ashley", "645-7689");
        contacts.insert("Katie", "435-8291");
        contacts.insert("Robert", "956-1745");

        // Take a reference and returns Option<&V>
    }
}