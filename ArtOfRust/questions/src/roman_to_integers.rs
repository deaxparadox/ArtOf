use std::{fs, collections::HashMap, fmt::format};

pub fn run() {
    let number = Solution::roman_to_int("III".to_string());
    println!("\n{:?}\n", number);

}
struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman = s;
        let mut hm: HashMap<char, i32> = HashMap::new();
        hm.insert( 'I', 1);
        hm.insert( 'V', 5);
        hm.insert( 'X', 10);
        hm.insert( 'L', 50);
        hm.insert( 'C', 100);
        hm.insert( 'D', 500);
        hm.insert( 'M', 1000);
        let number = 0;


        let mut roman_list: Vec<&str> = roman
                                        .split("")
                                        .into_iter()
                                        .filter(|x| x.to_owned() != "")
                                        .collect();

        while roman_list.len() > 0 {
            if let Some(key) = roman_list.pop() {
                // match hm.get(key.)  {
                //     _ => { println!("Not") }
                // }
            }
        }

        return number;
    }
}