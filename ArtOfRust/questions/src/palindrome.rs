use core::num;

pub fn run() {
    let target = 1221;
    println!("{}", Solution::is_palindrome(target));
}

struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut number = x;
        if number < 0 {
            return false;
        }

        // vector of reverse number
        let mut numbers = Solution::reverse_number(number);

        // copy of vector of reverse number
        let mut numbers2 = numbers.clone();

        // orignal number in vector form
        numbers2.reverse();

        for i in 0..numbers.len() {
            if numbers[i] != numbers2[i] {
                return  false;
            }
        }

        return true;
    }


    pub fn reverse_number(x: i32) -> Vec<i32> {
        let mut number = x;
        let mut numbers: Vec<i32> = Vec::new();
        let mut rem: i32;
        let mut quo: i32;
        while true {
            if number <= 0 {
                break;
            }
            rem = number / 10;
            quo = number % 10;
            number = rem;
            numbers.push(quo);
        }
        return numbers;
    }

}