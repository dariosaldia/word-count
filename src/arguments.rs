use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The number of bytes in the file is written to the standard output.
    #[arg(short = 'c', long, group = "opts")]
    pub byte: bool,

    /// The number of lines in each input file is written to the standard output.
    #[arg(short = 'l', long, group = "opts")]
    pub line: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short = 'w', long, group = "opts")]
    pub word: bool,
    
    /// The file
    #[arg(group = "input")]
    pub file: String,
}
