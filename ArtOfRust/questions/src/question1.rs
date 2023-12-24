use std::i32;



pub fn run() {
    let ret = Solution::my_atoi("42".to_string());
    println!("Return value: {ret}");
}

pub struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut _number_string = String::new();
        let mut chars = s.trim().chars();
        let mut negative = false;
        let mut read_string = false;

        while let Some(value) = chars.next() {
            match value {
                '0'..='9' => { read_string = true; _number_string.push(value); }
                '-' => { if read_string { break; } read_string = true; negative = true; }
                '+' => { if read_string { break; } read_string = true; }
                ' ' => { if read_string { break; } },
                _ => break,
            }
        }
        let mut result = "".to_string();

        if negative {
            result = String::from("-") + &_number_string;
        } else {
            result = _number_string;
        }

        match result.parse::<i32>() {
            Ok(value) => value,
            Err(message) => match message.kind() {
                std::num::IntErrorKind::PosOverflow => i32::MAX,
                std::num::IntErrorKind::NegOverflow => i32::MIN,
                _ => 0,
            },
        }
    }
}
