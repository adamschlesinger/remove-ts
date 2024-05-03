use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // Get args
    let args: Vec<String> = env::args().collect();

    // Check if path in args
    if args.len() < 2 {
        panic!("Please provide remove-ts with a txt file.");
    }

    let mut path = args[1].clone();

    // Make sure file is text file
    if Some("txt".as_ref()) != Path::new(&path).extension() {
        panic!("File must be a .txt file.");
    }

    let contents = fs::read_to_string(path.clone())
        .unwrap_or_else(|e| panic!("Error reading file: {}", e));

    // Regex delete timestamps
    let re = Regex::new(r#"(\d+\.*\d+\s*){2}("')*"#)?;
    let processed_contents = re.replace_all(&contents, "").to_string();

    // Output processed file
    path.insert_str(path.len() - 4, "-proc");
    fs::write(&path, processed_contents)
        .unwrap_or_else(|e| panic!("Error writing file: {}", e));

    Ok(())
}
