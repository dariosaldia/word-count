use arguments::Cli;
use clap::Parser;
use file_reader::read_file_to_string;

mod arguments;
mod file_reader;

fn main() {
    let cli = Cli::parse();

    if cli.byte {
        let byte_count = read_file_to_string(&cli.file).as_bytes().len();
        eprintln!("  {} {}", byte_count, &cli.file);
    } else if cli.line {
        let line_count = read_file_to_string(&cli.file).lines().count();
        eprintln!("  {} {}", line_count, &cli.file);
    } else if cli.word {
        let word_count = read_file_to_string(&cli.file).split_ascii_whitespace().count();
        eprintln!("  {} {}", word_count, &cli.file);
    } else if cli.char {
        let char_count = read_file_to_string(&cli.file).chars().count();
        eprintln!("  {} {}", char_count, &cli.file);
    } else {
        let byte_count = read_file_to_string(&cli.file).as_bytes().len();
        let line_count = read_file_to_string(&cli.file).lines().count();
        let word_count = read_file_to_string(&cli.file).split_ascii_whitespace().count();
        eprintln!("    {}   {}  {} {}", line_count, word_count, byte_count, &cli.file);
    }
}
