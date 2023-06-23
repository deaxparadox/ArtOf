use clap::Parser;
use WrkSpc::{Args, Greet};

mod smart_pointers;
mod  hashmaps;


// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about=None)]
// struct Args {
//     #[arg(short, long)]
//     name: String,
//     #[arg(short, long, default_value_t=1)]
//     count: u8,
// }

// fn Greeting(args: &Args) {
//     for _ in 0..args.count.to_owned() {
//         println!("Hello {}!", args.name);
//     };
// }

fn main() {
    smart_pointers::reference_counters::Main();
    
    let args = Args::parse();
    Greet(&args);
}
