use clap::Parser;

use bf::parse;

/// A brainfuck interpreter.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Filename of .b file. If not used, opens repl.
    #[clap(value_parser)]
    filename: Option<String>,
    /// Reveal all memory after running.
    #[clap(short, long)]
    reveal: bool,
    /// Reveal all memory after each repl input
    #[clap(short='R', long, requires = "filename", conflicts_with = "reveal")]
    reveal_all: bool
}

fn main() {
    let cli = Cli::parse();

    let parsed = parse(&String::from("+[->>[--]><-]++[>>>]..,"));

    println!("{:?}", parsed)
}
