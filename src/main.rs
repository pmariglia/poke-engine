#![allow(dead_code)]

use poke_engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::search::expectiminimax_search;
use poke_engine::state::{Move, State};
use poke_engine::choices::MOVES;

extern crate lazy_static;

fn main() {
    let mut state: State = State::default();

    // println!("{:?}", state);
    let (side_one_options, side_two_options) = state.get_all_options();

    let result = expectiminimax_search(
        &mut state,
        3,
        side_one_options,
        side_two_options,
        false,
    );
    // println!("{:?}", state);
}
