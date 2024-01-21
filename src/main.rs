#![allow(dead_code)]

use crate::state::Terrain;
use crate::{
    choices::MOVES,
    generate_instructions::generate_instructions_from_move,
    instruction::{Instruction, StateInstructions},
    state::{SideReference, State},
};

extern crate lazy_static;

mod abilities;
mod choices;
mod damage_calc;
mod generate_instructions;
mod instruction;
mod items;
mod state;

fn main() {
    let _choice = MOVES.get("defog").unwrap().to_owned();

    let mut state: State = State::default();
    println!(
        "Starting side 1 active types: {:?}",
        state.side_one.get_active().types
    );

    // state.side_one.get_active().status = PokemonStatus::Paralyze;
    state.terrain.terrain_type = Terrain::ElectricTerrain;
    state.terrain.turns_remaining = 1;
    state.side_one.get_active().ability = String::from("beastboost");
    state.side_two.get_active().hp = 100;

    let state_instruction: StateInstructions = StateInstructions {
        percentage: 100.0,
        instruction_list: Vec::<Instruction>::new(),
        ..Default::default()
    };

    let ins = generate_instructions_from_move(
        &mut state,
        _choice,
        MOVES.get("tackle").unwrap(),
        SideReference::SideOne,
        state_instruction,
    );

    println!(
        "After generate_instructions_from_move side 1 active name: {:?}",
        state.side_one.get_active().id
    );

    for i in ins {
        println!("Generated Instruction: {:?}", i);
    }
}
