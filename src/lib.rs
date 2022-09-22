pub type Memory = Vec<u8>;
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
    /// +
    Increment,
    /// -
    Decrement,
    /// >
    Forward,
    /// <
    Backward,
    /// [
    StartLoop,
    /// ]
    EndLoop
}

impl Instruction {
    pub fn from_char (c: char) -> Option<Self> {
        use Instruction::*;
        Some(match c {
            '+' => Increment,
            '-' => Decrement,
            '>' => Forward,
            '<' => Backward,
            '[' => StartLoop,
            ']' => EndLoop,
            _ => { return None }
        })
    }
}
