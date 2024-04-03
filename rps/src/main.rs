mod utils;
use std::process;

fn main() {
    let result = utils::run();

    match result {
        Err(utils::RPSError::InputError) => {
            println!("Failed to read input.");
            process::exit(-1);
        },
        Err(utils::RPSError::ParseError(s)) => {
            println!("Failed to parse value: {}", s);
            process::exit(-2);
        },
        Ok(()) => process::exit(0),
    }
}
