pub mod mean_mod_median {
    use std::vec;

    pub fn main() {
        let arr1 = [1];
        let arr2 = [3, 4];


        let mut merged: Vec<i32> = vec![];

        for i in arr1.to_owned().iter() {
            merged.push(*i)
        }

        for i in arr2.to_owned().iter() {
            merged.push(*i)
        }

        // println!("{:?}", (merged.len()-1) as f32/2.0);

        let middle: usize;
        if merged.len() % 2 == 0 {
            // even
            println!("Even array");
            middle = (merged.len()-1)/2;
            println!("{:?}", middle);
            let median = (merged[middle] + merged[middle + 1]) as f32 / 2.0;
            println!("Median : {median}")
        } else {
            // odd
            println!("Odd array");
            middle = (merged.len()-1)/2;
            println!("Median : {}", merged[middle]);
        }

    }
}