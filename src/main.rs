use std::path::PathBuf;

use clap::Parser;

use bf::{run, run_from_state};

/// A brainfuck interpreter.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Filename of .b file. If not used, opens repl.
    #[clap(value_parser)]
    filename: Option<PathBuf>,
    /// Reveal all memory after running.
    #[clap(short, long)]
    reveal: bool,
    /// Reveal all memory after each repl input
    #[clap(short='R', long, requires = "filename", conflicts_with = "reveal")]
    reveal_all: bool
}

fn main() {
    let cli = Cli::parse();

    match cli.filename {
        Some(fp) => {
            // INTERPRET FILE
        },
        None => {
            let finished = run(&String::from("+++>>>---"));

            println!("{:?}", finished)
        },
    }
    
}
