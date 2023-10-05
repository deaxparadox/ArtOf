pub mod example_1 {
    pub fn main() {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum if configured to be {}", max),
            _ => (),
        }
    }
}

pub mod example_2 {
    pub fn main() {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum if configured to be {}", max);
        }
    }
}