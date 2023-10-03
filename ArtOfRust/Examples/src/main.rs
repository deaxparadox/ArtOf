use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

mod collections;
mod enump;
mod error;
mod slice;
mod structure;
mod rust;
mod question;
mod generics;


fn main() {
    question::mean_mod_median::main();
    
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("error").about("error learning").arg(
                arg!(--error_1 "lists test values"
                )
                .action(ArgAction::SetTrue),
            ).arg(
                arg!(--error_2 "lists test values"
                )
                .action(ArgAction::SetTrue),
            ).arg(
                arg!(--error_3 "lists test values"
                )
                .action(ArgAction::SetTrue),
            ).arg(
                arg!(--error_4 "lists test values"
                )
                .action(ArgAction::SetTrue),
            ).subcommand(
                Command::new("unwrap").about("Short for Result enum").arg(
                    arg!(--unwrap_1 "unwrap example").action(ArgAction::SetTrue))
            
            ),
        )
        .get_matches();

    if let Some(errors) = matches.subcommand_matches("error") {

        match errors.subcommand_name() {
            Some("unwrap") => {
                if let Some(unwraps) = errors.subcommand_matches("unwrap") {
                    if unwraps.get_flag("unwrap_1") {
                        error::unwrap_1::main();
                        println!("Printing file with unwrap");
                    }
                };
            },
            _ => {
                if errors.get_flag("error_1") {
                    error::error_1::main();
                } else if errors.get_flag("error_2") {
                    error::error_2::main();
                } else if errors.get_flag("error_3") {
                    error::error_3::main();
                } else if errors.get_flag("error_4") {
                    error::error_4::main();
                } else {
                    println!("Not printing testing lists...")
                }
            }
        };
    };

    generics::generic_method_definition_1::main();
}
