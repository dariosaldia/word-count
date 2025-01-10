use std::fs;

pub fn read_file_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1); // Exit the program with a non-zero status
    })
}
