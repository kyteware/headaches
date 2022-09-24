use std::io::{stdin, Read, Write, stdout};

pub mod interpret;

pub use interpret::{execute, parse, run, run_from_state};

/// A `Vec` of `u8`s representing a the memory
/// of a Brainfuck process.
pub type Memory = Vec<u8>;
/// The pointer that indicates the selected cell
/// of a Brainfuck process.
pub type Pointer = usize;

/// The state of a Brainfuck process.
#[derive(Debug)]
pub struct State {
    /// The process's memory.
    pub mem: Memory,
    /// The location of the pointer.
    pub pointer: Pointer,
}

impl State {
    /// Creates a new Brainfuck state.
    pub fn new() -> Self {
        Self {
            mem: vec![0],
            pointer: 0,
        }
    }
    #[allow(unused_must_use)]
    pub fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Increment => {
                self.mem[self.pointer] = self.mem[self.pointer].overflowing_add(1).0;
            }
            Instruction::Decrement => {
                self.mem[self.pointer] = self.mem[self.pointer].overflowing_sub(1).0;
            }
            Instruction::Forward => {
                if self.pointer + 1 == self.mem.len() {
                    self.mem.push(0)
                }
                self.pointer += 1;
            }
            Instruction::Backward => {
                if self.pointer != 0 {
                    self.pointer -= 1;
                }
            }
            Instruction::Loop(inners) => {
                for inner in inners {
                    self.run(inner)
                }
            }
            Instruction::LoopEnd => {}
            Instruction::Out => print!("{}", char::from(self.mem[self.pointer])),
            Instruction::In => {
                stdout().flush();
                if let Some(Ok(c)) = stdin().bytes().next() {
                    self.mem[self.pointer] = c
                }
            }
        }
    }
}

/// A Brainfuck instruction.
#[derive(Debug)]
pub enum Instruction {
    /// Represents the `+` instruction.
    ///
    /// Used to increment the currect cell.
    Increment,
    /// Represents the `-` instruction.
    ///
    /// Used to decrement the currect cell.
    Decrement,
    /// Represents the `>` instruction.
    ///
    /// Used to move to the next cell.
    Forward,
    /// Represents the `<` instruction.
    ///
    /// Used to increment the previous cell.
    Backward,
    /// Represents a loop (inside `[]`).
    ///
    /// Repeats everything within it for as long as the pointer
    /// starts on a non-null cell.
    Loop(Vec<Instruction>),
    /// Represents the end of a Brainfuck loop.
    ///
    /// Only used for parsing, not present in a parsed list
    /// of instructions.
    LoopEnd,
    /// Represents the `.` instruction.
    ///
    /// Used to increment the previous cell.
    Out,
    /// Represents the `,` instruction.
    ///
    /// Opens stdin and takes the first character of the inputted
    /// string.
    In,
}

#[derive(Debug)]
pub struct TryFromCharError;

impl std::fmt::Display for TryFromCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not convert from a `char`.")
    }
}

impl std::error::Error for TryFromCharError {}

impl TryFrom<char> for Instruction {
    type Error = TryFromCharError;
    /// Parses a character into a Brainfuck instruction.
    ///
    /// If the input is not an instruction,
    fn try_from(c: char) -> Result<Self, TryFromCharError> {
        use Instruction::*;
        Ok(match c {
            '+' => Increment,
            '-' => Decrement,
            '>' => Forward,
            '<' => Backward,
            '[' => Loop(vec![]),
            ']' => LoopEnd,
            '.' => Out,
            ',' => In,
            _ => return Err(TryFromCharError),
        })
    }
}
