#[allow(unused)]
pub struct Guess {
    value: i32,
}
impl Guess {
    #[allow(unused)]
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod test_guess {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100, but 200")]
    fn greater_than_100() {
        Guess::new(200);
    }
}