#![allow(dead_code)]

use crate::generate_instructions::generate_instructions_from_move_pair;
use crate::state::State;

extern crate lazy_static;

mod abilities;
mod choices;
mod damage_calc;
mod generate_instructions;
mod instruction;
mod items;
mod state;

fn main() {
    let mut state: State = State::default();
    state.side_one.get_active().speed = 100;
    state.side_one.get_active().hp = 99;
    state.side_two.get_active().speed = 50;

    let ins = generate_instructions_from_move_pair(
        &mut state,
        String::from("leechseed"),
        String::from("splash"),
    );

    for i in ins {
        println!(
            "Generated Instruction: {:?}: {:?}",
            i.percentage, i.instruction_list
        );
    }
}
