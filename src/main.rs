use std::path::PathBuf;
use std::io;

use clap::Parser;

use bf::{run, run_from_state, State};

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
    #[clap(short = 'R', long, requires = "filename", conflicts_with = "reveal")]
    reveal_all: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.filename {
        Some(fp) => {
            todo!()
        }
        None => {
            let mut state = State::new();
            loop {
                let mut raw = String::new();
                io::stdin()
                    .read_line(&mut raw)
                    .expect("could not read input");
                run_from_state(&raw, &mut state)
            }
        }
    }
}
