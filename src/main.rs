use arguments::Cli;
use clap::Parser;
use file_reader::read_file_to_string;

mod arguments;
mod file_reader;

fn main() {
    let cli = Cli::parse();

    if cli.character {
        let byte_count = read_file_to_string(&cli.file).as_bytes().len();
        eprintln!("  {} {}", byte_count, &cli.file);
    } else if cli.line {
        let line_count = read_file_to_string(&cli.file).lines().count();
        eprintln!("  {} {}", line_count, &cli.file);
    }
}
