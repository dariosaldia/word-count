use arguments::Cli;
use clap::Parser;
use file_reader::read_file_to_string;

mod arguments;
mod file_reader;

fn main() {
    let cli = Cli::parse();

    // Since we'll print by default all options, for now it's the same
    // to check if the -c is set or not. So we print either way
    let byte_count = read_file_to_string(&cli.file).as_bytes().len();
    eprintln!("  {} {}", byte_count, &cli.file);
}
