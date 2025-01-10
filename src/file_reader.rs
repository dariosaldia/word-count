use std::{fs, io::{self, Read}};

pub fn read_file_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1); // Exit the program with a non-zero status
    })
}

pub fn read_std_input_to_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1); // Exit the program with a non-zero status
        });
    input
}
