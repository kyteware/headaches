pub mod interpret;

pub use interpret::{run, run_from_state};

/// A `Vec` of `u8`s representing a the memory
/// of a Brainfuck process.
pub type Memory = Vec<u8>;
/// The pointer that indicates the selected cell 
/// of a Brainfuck process.
pub type Pointer = u32;

/// The state of a Brainfuck process.
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
            mem: Memory::new(),
            pointer: 0
        }
    }
}

/// A Brainfuck instruction.
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
    In
}

impl Instruction {
    /// Parses a character into a Brainfuck instruction.
    /// 
    /// If the input is not an instruction, 
    pub fn from_char (c: char) -> Option<Self> {
        use Instruction::*;
        Some(match c {
            '+' => Increment,
            '-' => Decrement,
            '>' => Forward,
            '<' => Backward,
            '[' => Loop(vec![]),
            ']' => LoopEnd,
            _ => { return None }
        })
    }
}
