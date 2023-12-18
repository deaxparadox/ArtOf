
fn return_void() -> () {
    return
}

pub fn method_that_produce_other_iterators() {
    // let return_value = return_void();
    // if return_value == () {
    //     println!("Void returned");
    // }

    let mut v1: Vec<i32>  = Vec::new();
    for i in 1..10 {
        v1.push(i);
    }

    let incremented_values: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    let positive_values: Vec<i32> = incremented_values.into_iter().filter(|x| x % 2 == 0).collect();
    for i in positive_values.into_iter() {
        println!("{i}");
    }
}


#[derive(Debug, PartialEq)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}