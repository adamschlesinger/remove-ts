use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Read};

// const PATTERN:&str = "^(\d+\.*\d+\s*){2}"'";

fn main() -> Result<(), Box<dyn Error>> {
    // Get args
    let args: Vec<String> = env::args().collect();

    // Check if path in args
    if args.len() < 2 {
        panic!("Please provide remove-ts with a txt file.");
    }

    let path = &args[1];

    // Make sure file is text file
    // todo

    let contents = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Error reading file: {}", e));

    // Regex delete timestamps
    // todo



    Ok(())
}
