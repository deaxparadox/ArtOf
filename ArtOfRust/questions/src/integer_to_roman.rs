use core::num;
use std::{fs, collections::HashMap, fmt::format};

pub fn run() {
    let romans = Solution::int_to_roman(58);
    println!("{:?}", romans);

}

struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut hm = HashMap::new();
        hm.insert(1, 'I');
        hm.insert(5, 'V');
        hm.insert(10, 'X');
        hm.insert(50, 'L');
        hm.insert(100, 'C');
        hm.insert(500, 'D');
        hm.insert(1000, 'M');

        let mut roman = String::from("");

        let mut numbers = Solution::reverse(num);
        // numbers.reverse();
        let mut numbers_places: Vec<i32> = Vec::new();
        
        let t: i32 = 10;
        let mut tens: i32;
        let mut value: char;
        while numbers.len() > 0 {
            if let Some(key) = numbers.pop() {
                tens = t.pow(numbers.len() as u32);
                value =  hm.get(&tens).unwrap().to_owned();
                match key {
                    1 => roman.push(value),
                    2 => {roman.push_str(&format!("{}{}", value, value))},
                    3 => {roman.push_str(&format!("{}{}{}", value, value, value))},
                    4 => {
                        roman.push(value);
                        value = hm.get(&(tens*5)).unwrap().to_owned();
                        roman.push(value);
                    },
                    5 => {
                        value = hm.get(&(tens*5)).unwrap().to_owned();
                        roman.push(value);
                    },
                    6 => {
                        value = hm.get(&(tens*5)).unwrap().to_owned();
                        roman.push(value);
                        value = hm.get(&(tens)).unwrap().to_owned();
                        roman.push(value);
                    },
                    7 => {
                        value = hm.get(&(tens*5)).unwrap().to_owned();
                        roman.push(value);
                        value = hm.get(&(tens)).unwrap().to_owned();
                        roman.push(value);
                        roman.push(value);
                    },
                    8 => {
                        
                        value = hm.get(&(tens*5)).unwrap().to_owned();
                        roman.push(value);
                        value = hm.get(&(tens)).unwrap().to_owned();
                        roman.push(value);
                        roman.push(value);
                        roman.push(value);
                    },
                    9 => {
                        value = hm.get(&(tens)).unwrap().to_owned();
                        roman.push(value);
                        value = hm.get(&(tens*10)).unwrap().to_owned();
                        roman.push(value);
                    }

                    _ => println!("Not implemented"),
                }
                println!("{value}");
            }
        }

        // println!("{:?}", numbers_places);

        return roman;
    }

    fn reverse(x: i32) -> Vec<i32>{
        let mut number = x;
        let (mut rem, mut quo): (i32, i32);
        let mut numbers: Vec<i32> = vec![];
        while number > 0 {
            quo = number / 10;
            rem = number % 10;
            numbers.push(rem);
            number = quo;
        }
        return numbers;
    }
}