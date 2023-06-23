mod result;
mod options;


use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command};

use result as Result;
use options as opt;



fn main() {

    let matches = command!()
        
        .subcommand(
            Command::new("option")
            .about("Specfic option command")
            .arg(
                arg!(--"by-example" "Run command option by example")
                .action(ArgAction::SetTrue),
            )
        )
        .get_matches();

    // Result::Main();
    // opt::by_example::main();
    if let Some(matches) = matches.subcommand_matches("option") {
        if matches.get_flag("by-example") {
            opt::by_example::main();
            return 
        }
        opt::main();        
    }

}
