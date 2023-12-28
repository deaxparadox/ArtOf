use std::{fs, collections::HashMap, fmt::format};

pub fn run() {
    let string_vec: Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];    
    let number = Solution::longest_common_prefix(string_vec);
    println!("\n{:?}\n", number);

}
struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let j: i32 = 0;
        let ans = "".to_string();
        for i in 0..strs.len() {
            
            while j < strs[i].len() as i32 {
                if strs[i][j] == strs[0][j] {

                }                
            }
        }
        "".to_string()
    }

}