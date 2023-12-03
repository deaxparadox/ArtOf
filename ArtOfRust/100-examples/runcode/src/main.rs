use std::process::Output;
use std::process::exit;
use std::rc::Rc;
use std::thread;
use std::time;

mod closures;
mod lifetimes;
mod reference_counters;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}


enum Numbers {
    One(i32),
    Two(i32),
    Three(i32)
}
fn number() {
    let one = Numbers::One(1);

    if let Numbers::One(value) = one {
        println!("Everything Ok, {}", value);
    }
}

fn display_name(a0: Rc<&str>) {
    println!("Name is: {a0}");
}



fn generate_and_return_string<'a, 'b>() -> &'a &'b str {
    return &"Nitish";
}


fn return_from_loop() -> String {
    let mut counter = 0;
    
    'stop_anywhere: loop {
        counter += 1;

        if counter == 10 {
            break 'stop_anywhere;
        }
    };
    let result = counter;
    return format!("The result is {}", result);
}

fn nested_loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn nested_loop_label_2() {
    let mut count: i32 = 1;
    'outer: loop {
        if count > 10 {
            break 'outer;
        }
        
        if count % 2 == 0 {
            println!("Outer loop: {}", count);
        }
        

        'inner: loop {
            if count % 3 == 0 {
                println!("Inner loop: {}", count);
            }
            break;
        }
        count += 1;
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn main() {

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

}
