#![allow(dead_code)]

use crate::state::PokemonStatus;
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
    let mut _choice = MOVES.get("earthquake").unwrap().to_owned();

    //_choice = Choice {
    //    move_id: "bulbasaur".to_string(),
    //    switch_id: 1,
    //    move_type: PokemonTypes::Typeless,
    //    accuracy: 100.0,
    //    category: MoveCategory::Switch,
    //    base_power: 0.0,
    //    boost: None,
    //    priority: 0,
    //    flags: Flags {
    //        authentic: false,
    //        bite: false,
    //        bullet: false,
    //        charge: false,
    //        contact: false,
    //        dance: false,
    //        defrost: false,
    //        distance: false,
    //        drag: false,
    //        gravity: false,
    //        heal: false,
    //        mirror: false,
    //        mystery: false,
    //        nonsky: false,
    //        powder: false,
    //        protect: false,
    //        pulse: false,
    //        punch: false,
    //        recharge: false,
    //        reflectable: false,
    //        snatch: false,
    //        sound: false,
    //    },
    //    heal: None,
    //    status: None,
    //    volatile_status: None,
    //    side_condition: None,
    //    secondaries: None,
    //    target: MoveTarget::Opponent,
    //};

    let mut state: State = State::default();
    println!(
        "Starting side 1 active types: {:?}",
        state.side_one.get_active().types
    );

    state.side_one.get_active().status = PokemonStatus::Paralyze;
    state.side_one.get_active().ability = String::from("protean");
    // state.side_two.get_active().ability = String::from("levitate");

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
