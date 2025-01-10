use arguments::Cli;
use clap::Parser;
use file_reader::{read_file_to_string, read_std_input_to_string};

mod arguments;
mod file_reader;

fn main() {
    let cli = Cli::parse();

    let file = cli.file.clone().map_or_else(|| read_std_input_to_string(), |f| read_file_to_string(&f));

    let path_file = cli.file.unwrap_or("".to_string());

    if cli.byte {
        let byte_count = file.as_bytes().len();
        eprintln!("  {} {}", byte_count, path_file);
    } else if cli.line {
        let line_count = file.lines().count();
        eprintln!("  {} {}", line_count, path_file);
    } else if cli.word {
        let word_count = file.split_ascii_whitespace().count();
        eprintln!("  {} {}", word_count, path_file);
    } else if cli.char {
        let char_count = file.chars().count();
        eprintln!("  {} {}", char_count, path_file);
    } else {
        let byte_count = file.as_bytes().len();
        let line_count = file.lines().count();
        let word_count = file.split_ascii_whitespace().count();
        eprintln!("    {}   {}  {} {}", line_count, word_count, byte_count, path_file);
    }
}
