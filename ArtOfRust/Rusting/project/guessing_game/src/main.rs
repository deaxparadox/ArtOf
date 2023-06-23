use std::io;

fn main() {
    println!("Guess the number!");
    print!("Please input you guess");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}