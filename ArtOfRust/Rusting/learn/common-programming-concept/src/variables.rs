pub fn immutable() {
    let x = 5;
    println!("This value of x is: {x}");
    // x = 6;
    println!("The value of x is: {x}");
}

pub fn mutable() {
    let mut x = 5;
    println!("This value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

pub const gvalue: u32 = 9999;

pub fn contant() {
    const name: u32 = 7328;
    println!("{name}");
}

pub fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}


pub fn len() {
    let names = String::from("I am from planet far apar");
    println!("name: {names}");
    let names = names.len();
    println!("length: {names}");
}