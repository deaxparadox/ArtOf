use core::num;
use std::fs;

pub fn run() {
    let height = vec![1,8,6,2,5,4,8,3,7];
    println!("Area: {}", Solution::max_area(height));
    let height = vec![1,1];
    println!("Area: {}", Solution::max_area(height));

    // let contents = fs::read_to_string("new_input.txt").expect("Unable to read file.");
    // let lines = contents.split("\n");
    // for l in lines {
    //     if l.trim() == " " { continue; }
    //     println!("L: {}", l.trim());
    // }

}

struct Solution;
impl Solution {
    // pub fn max_area(height: Vec<i32>) -> i32 {
    //     let mut area: i32 = 0;
    //     let mut h1:i32;
    //     let mut h2:i32;
    //     let mut x: i32;
    //     let mut temp_area: i32;
    //     let length = height.len();
    //     for i in 0..length {
    //         for j in 0..length {


    //             if i > j || i == j {
    //                 continue;
    //             }

    //             println!("{}, {}", i, j);

    //             h1 = height[i];
    //             h2 = height[j];

    //             x = (j as i32) - (i as i32);
    //             if h1  > h2 {
    //                 temp_area = x * h2;
    //             } else {
    //                 temp_area = x * h1;
    //             }
    //             if area < temp_area {
    //                 area = temp_area;
    //             }

    //         }
    //     }
    //     return area;
    // }

    pub fn max_area(height: Vec<i32>) -> i32 {
        
        let mut max_water: i32 = 0;
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;

        while left < right {
            println!("{}, {}", left, right);
            max_water = std::cmp::max(max_water, std::cmp::min(height[left], height[right]) * (right - left) as i32);

            if height[left] <= height[right] {
                left += 1;
            }
            else {
                right -= 1;
            }
        }
        max_water
    }

}