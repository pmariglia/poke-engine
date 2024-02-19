#![allow(dead_code)]

use poke_engine::choices::MOVES;
use poke_engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::state::State;

extern crate lazy_static;

fn main() {
    let mut state: State = State::default();
    state.side_one.get_active().speed = 100;
    state.side_one.get_active().hp = 99;
    state.side_two.get_active().speed = 50;

    let ins = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("leechseed").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    for i in ins {
        println!(
            "Generated Instruction: {:?}: {:?}",
            i.percentage, i.instruction_list
        );
    }
}
