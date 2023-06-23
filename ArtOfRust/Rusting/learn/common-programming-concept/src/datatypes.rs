pub fn datatypes() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("number: {guess}");
}