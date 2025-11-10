mod cli;

use std::{
    fs::File,
    io::{self, IsTerminal},
};

use clap::Parser;
use cli::Cli;
use word_counter::get_counts;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    if let Some(input) = args.input {
        let file = File::open(input)?;
        let counts = get_counts(file)?;
        counts.print_conditional(args.lines, args.words, args.chars);
    } else {
        let stdin = io::stdin();

        if stdin.is_terminal() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "No input provided. Pass a file path or pipe text via stdin.",
            ));
        }

        let counts = get_counts(stdin.lock())?;
        counts.print_conditional(args.lines, args.words, args.chars);
    }

    Ok(())
}
