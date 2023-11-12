pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello!")
}

#[cfg(test)]
mod custome_failure_message {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
        // assert!(
        //     result.contains("Carol"),
        //     "\nGreeting did ont contain name, value was`{}`\n",
        //     result
        // );
    }
}