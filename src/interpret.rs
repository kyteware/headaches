use std::str::Chars;

use crate::{Instruction, State};

/// Parses raw Brainfuck code into list of instructions.
///
/// Ignores characters not in the Brainfuck language.
pub fn parse(raw: &String) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut chars = raw.chars().into_iter();
    while let Some(c) = chars.next() {
        if let Some(instruction) = Instruction::try_from(c).ok() {
            // Iterating through characters now
            if let Instruction::Loop(mut inners) = instruction {
                parse_loop(&mut chars, &mut inners);
                instructions.push(Instruction::Loop(inners))
            } else {
                instructions.push(instruction)
            }
        }
    }
    instructions
}

fn parse_loop(chars: &mut Chars, inners: &mut Vec<Instruction>) {
    while let Some(c) = chars.next() {
        if let Some(instruction) = Instruction::try_from(c).ok() {
            // Iterating through characters now
            match instruction {
                Instruction::Loop(mut local_inners) => {
                    parse_loop(chars, &mut local_inners);
                    inners.push(Instruction::Loop(local_inners));
                }
                Instruction::LoopEnd => {
                    break;
                }
                _ => inners.push(instruction),
            }
        }
    }
}

/// Executes a list of Brainfuck instructions.
pub fn execute(state: &mut State, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        state.run(instruction);
    }
}

/// Run Brainfuck code.
pub fn run(raw: &String) -> State {
    let mut state = State::new();
    run_from_state(raw, &mut state);
    state
}

/// Run Brainfuck code from a previous state.
pub fn run_from_state(raw: &String, state: &mut State) {
    let code = parse(raw);
    for instruction in code {
        state.run(&instruction)
    }
}
