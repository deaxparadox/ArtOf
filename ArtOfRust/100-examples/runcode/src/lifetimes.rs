use std::rc::Rc;
use std::process::exit;

fn generate_and_return_string<'a, 'b>() -> &'a &'b str {
    return &"Nitish";
}

pub fn run() {
    let gr =  (**generate_and_return_string()).to_string();
    println!("{}", gr);

    exit(0);
}