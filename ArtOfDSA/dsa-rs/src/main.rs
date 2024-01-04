use clap::{command, Arg, ArgAction, Command};
use std::path::PathBuf;
use std::process::ExitCode;
mod algorithms;
mod ds;

fn main() -> ExitCode {
    let matches = command!()
        .arg(
            Arg::new("linear_serach")
                .long("linear_search")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("binary_serach")
                .long("binary_search")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("algorithm")
                .about("Run Algorithm with custom arguments")
                .subcommand(
                    Command::new("linear_search").about("Linear Search with custom argument"),
                ),
        )
        .get_matches();

    if let Some(linear_search) = matches.get_one::<bool>("linear_serach") {
        if *linear_search {
            match algorithms::linear_search::run() {
                Ok(()) => {
                    println!("Successfull");
                    ExitCode::SUCCESS;
                }
                Err(()) => {
                    println!("Failed");
                    ExitCode::FAILURE;
                }
            }
        }
    }

    if let Some(binary_search) = matches.get_one::<bool>("binary_serach") {
        if *binary_search {
            // match binary_searc::run() {
            //     Ok(()) => {
            //         println!("Successfull");
            //         ExitCode::SUCCESS;
            //     }
            //     Err(()) => {
            //         println!("Failed");
            //         ExitCode::FAILURE;
            //     }
            // }
        }
    }

    println!("Specify a DAS :)");
    ExitCode::SUCCESS
}
