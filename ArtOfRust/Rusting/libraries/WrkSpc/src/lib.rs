use clap::Parser;



#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[arg(short, long)]
    user: String,

    // #[arg(short, long)]
    // first_name: String,

    // #[arg(short, long)]
    // last_name: String,

    // #[arg(short, long)]
    // age: u32,

    #[arg(short, long, default_value_t=1)]
    count: u8,
}

pub fn Greet(args: &Args) {
    for _ in 0..args.count.to_owned() {
        println!("Hello {}!", args.user);
    };
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
