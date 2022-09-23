use std::str::Chars;

use crate::{State, Instruction};

/// Parses raw Brainfuck code into list of instructions.
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

fn parse_loop (chars: &mut Chars, inners: &mut Vec<Instruction>) {
    while let Some(c) = chars.next() {
        if let Some(instruction) = Instruction::try_from(c).ok() {
            // Iterating through characters now
            match instruction {
                Instruction::Loop(mut local_inners) => {
                    parse_loop(chars, &mut local_inners);
                    inners.push(Instruction::Loop(local_inners));
                },
                Instruction::LoopEnd => { break },
                _ => inners.push(instruction)
            }
        }
    }
}

// fn get_deepest_loop<'a> (instructions: &'a mut Vec<Instruction>) -> &'a mut [Instruction] {
//     let mut target = instructions.as_mut_slice();
//     use Instruction::*;
//     loop {
//         // Checks if last item is list
//         if let Loop(inners) = target.last().unwrap_or(&LoopEnd) {
//             if Some(last_in_inners) = inners.last() {

//             }
//         } else { break }
//     }
//     target
// }

/// Executes a list of Brainfuck instructions.
pub fn execute(instructions: Vec<Instruction>) -> State {
    todo!()
}

/// Run Brainfuck code.
pub fn run(raw: &String) -> State {
    run_from_state(raw, State::new())
}

/// Run Brainfuck code from a previous state.
pub fn run_from_state(raw: &String, previous: State) -> State {
    todo!()
}