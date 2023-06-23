mod loops;

use regex::Regex;
use std::io;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn control_flow() {
        println!("Guess the number!");
    
        loop {
            println!("\nPlease input your guess: ");
    
            let mut guess = String::new();
        
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess = guess.trim().parse::<i32>().expect("Please type a number!");

            if guess == 0 {
                println!("You have entered 0")
            } else if guess > 0 && guess <= 5 {
                println!("Number lie between 1 and 5")
            } else {
                println!("Unknown number!");
                break;
            }
        };
}


fn count() -> i32 {
    let mut i: i32 = 0;
    for i in 1..10 {
        if i == 5 {
            println!("Encountered 5: skipping this value");
            continue;
        }
        if i == 16 {
            println!("Encountered 6: break now");
            break;
        }
        println!("number is {}", i);
        thread::sleep(Duration::from_millis(500));
    }
    i
}

fn count_loop() -> i32 {
    let mut i: i32 = 0;
    loop {
        if i == 5 {
            println!("Encountered 5: skipping this value");
            i +=1;
            continue;
        }
        if i == 6 {
            println!("Encountered 6: break now");
            break;
        }
        println!("number is {}", i);
        i +=1;
    }
    i
}
// 
fn count_loop_threaded() {

    
    let mut ts: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        ts.push(
            thread::spawn(move|| {
                count_loop();
            })
        )
    }
    
    for t in ts {
        t.join().unwrap();
    }    
}

fn main() {
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // println!("Did our date match? {}", re.is_match("2014-01-01"));
    // println!("Hello, world!");
    // control_flow();
    // println!("{}", count_loop());
    // count_loop_threaded();
    loops::main();
}
