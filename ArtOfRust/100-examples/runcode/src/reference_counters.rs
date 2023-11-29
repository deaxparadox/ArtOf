use std::rc::Rc;

pub fn run() {

    let mut one = Rc::new("Nitish");
    let two = Rc::clone(&one);
    
    // println!("{}", one);
    // display_name(Rc::clone(&one));
    // display_name(Rc::clone(&two));
    // println!("{}", one == two);
}