use std::path::PathBuf;
use std::process::ExitCode;

mod algorithms;

fn main() -> ExitCode {
    match algorithms::linear_search::run() {
        Ok(()) => {
            println!("Successfull");
            ExitCode::SUCCESS
        }
        Err(()) => {
            println!("Failed");
            ExitCode::FAILURE
        }
    }
}
