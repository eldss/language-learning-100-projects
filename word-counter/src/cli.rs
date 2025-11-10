use clap::Parser;

/// word-counter is a command-line utility written in Rust that analyzes
/// text files or standard input to count words, characters, and lines.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Either a file name or text to count
    pub input: Option<String>,

    /// Count lines
    #[arg(short, long)]
    pub lines: bool,

    /// Count words
    #[arg(short, long)]
    pub words: bool,

    /// Count characters
    #[arg(short, long)]
    pub chars: bool,
}
