use crate::{State, Instruction};

/// Parses raw Brainfuck code into list of instructions.
pub fn parse(raw: &String) -> Vec<Instruction> {
    todo!()
}

/// Executes a list of Brainfuck instructions.
pub fn execute(instructions: Vec<Instruction>) -> State {
    todo!()
}

/// Run Brainfuck code.
pub fn run(raw: &String) -> State {
    todo!()
}

/// Run Brainfuck code from a previous state.
pub fn run_from_state(raw: &String, previous: State) -> State {
    todo!()
}