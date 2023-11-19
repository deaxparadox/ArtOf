use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args);
    let config = Config::build(&args).expect("Argument errror");

    println!("Seraching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        return Config {
            query: query,
            file_path: file_path,
        };
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config { query, file_path });
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     Config::new(args)
// }
