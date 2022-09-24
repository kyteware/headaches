use std::{path::PathBuf, fs::read_to_string};
use std::io::{stdout, Write, stdin};

use clap::Parser;

use bf::{run, run_from_state, State};

/// A brainfuck interpreter.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Filename of .b file. If not used, opens repl.
    #[clap(value_parser)]
    filename: Option<PathBuf>,
    /// Reveal all memory after running (prints each time on repl).
    #[clap(short, long)]
    reveal: bool
}

#[allow(unused_must_use)]
fn main() {
    let cli = Cli::parse();

    match cli.filename {
        Some(fp) => {
            if let Ok(contents) = read_to_string(fp) {
                let state = run(&String::from(contents));
                if state.outted == true {
                    println!()
                }
            }
        }
        None => {
            let mut state = State::new();
            loop {
                if state.outted == true {
                    println!("");
                    state.outted = false;
                }
                print!(">>> ");
                stdout().flush();
                let mut raw = String::new();
                stdin()
                    .read_line(&mut raw)
                    .expect("could not read input");
                run_from_state(&raw, &mut state);
                if cli.reveal {
                    println!("{:?}", state);
                    stdout().flush();
                }
            }
        }
    }
}
