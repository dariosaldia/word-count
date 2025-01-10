use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The number of bytes in the file is written to the standard output.
    #[arg(short = 'c', long, group = "opts")]
    pub character: bool,

    /// The file
    #[arg(group = "input")]
    pub file: String,
}
