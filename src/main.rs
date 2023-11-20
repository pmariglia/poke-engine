#![allow(dead_code)]

use std::collections::HashSet;

use data::moves::{self, Choice, Flags, MoveCategory, MOVES};

use crate::{
    data::{
        conditions::{PokemonStatus, PokemonVolatileStatus},
        moves::{Effect, MoveTarget, Secondary, StatBoosts},
    },
    generate_instructions::generate_instructions_from_move,
    instruction::{
        BoostInstruction, ChangeTerrain, DamageInstruction, Instruction, StateInstruction,
    },
    state::{Pokemon, PokemonNatures, PokemonTypes, SideConditions, SideReference, Terrain},
};
extern crate lazy_static;

mod damage_calc;
mod data;
mod generate_instructions;
mod instruction;
mod state;

fn main() {
    let mut _choice = MOVES.get("absorb").unwrap().to_owned();

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


    let pikachu: state::Pokemon = Pokemon {
        id: "pikachu".to_string(),
        level: 100,
        types: (PokemonTypes::Electric, PokemonTypes::Typeless),
        hp: 100,
        maxhp: 100,
        ability: "voltabsorb".to_string(),
        item: "none".to_string(),
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
        attack_boost: 0,
        defense_boost: 0,
        special_attack_boost: 0,
        special_defense_boost: 0,
        speed_boost: 0,
        accuracy_boost: 0,
        evasion_boost: 0,
        status: PokemonStatus::None,
        nature: PokemonNatures::Serious,
        volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
        moves: vec![],
    };

    let bulbasaur: state::Pokemon = Pokemon {
        id: "bulbasaur".to_string(),
        level: 100,
        types: (PokemonTypes::Grass, PokemonTypes::Poison),
        hp: 100,
        maxhp: 100,
        ability: "overgrow".to_string(),
        item: "none".to_string(),
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
        attack_boost: 0,
        defense_boost: 0,
        special_attack_boost: 0,
        special_defense_boost: 0,
        speed_boost: 0,
        accuracy_boost: 0,
        evasion_boost: 0,
        status: PokemonStatus::None,
        nature: PokemonNatures::Serious,
        volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
        moves: vec![],
    };

    let squirtle: state::Pokemon = Pokemon {
        id: "squirtle".to_string(),
        level: 100,
        types: (PokemonTypes::Water, PokemonTypes::Typeless),
        hp: 100,
        maxhp: 100,
        ability: "voltabsorb".to_string(),
        item: "none".to_string(),
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
        attack_boost: 0,
        defense_boost: 0,
        special_attack_boost: 0,
        special_defense_boost: 0,
        speed_boost: 0,
        accuracy_boost: 0,
        evasion_boost: 0,
        status: PokemonStatus::None,
        nature: PokemonNatures::Serious,
        volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
        moves: vec![],
    };

    let charmander: state::Pokemon = Pokemon {
        id: "charmander".to_string(),
        level: 100,
        types: (PokemonTypes::Fire, PokemonTypes::Typeless),
        hp: 100,
        maxhp: 100,
        ability: "voltabsorb".to_string(),
        item: "none".to_string(),
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
        attack_boost: 0,
        defense_boost: 0,
        special_attack_boost: 0,
        special_defense_boost: 0,
        speed_boost: 0,
        accuracy_boost: 0,
        evasion_boost: 0,
        status: PokemonStatus::None,
        nature: PokemonNatures::Serious,
        volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
        moves: vec![],
    };

    let my_side: state::Side = state::Side {
        active_index: 0,
        pokemon: [pikachu, bulbasaur],
        side_conditions: SideConditions {
            ..Default::default()
        },
        wish: (0, 0),
    };

    let your_side: state::Side = state::Side {
        active_index: 0,
        pokemon: [squirtle, charmander],
        side_conditions: SideConditions {
            ..Default::default()
        },
        wish: (0, 0),
    };

    let state_weather = state::StateWeather {
        weather_type: state::Weather::None,
        turns_remaining: 0,
    };

    let state_terrain = state::StateTerrain {
        terrain_type: state::Terrain::None,
        turns_remaining: 0,
    };

    let mut state: state::State = state::State {
        side_one: my_side,
        side_two: your_side,
        weather: state_weather,
        terrain: state_terrain,
        trick_room: false,
    };

    println!(
        "Starting side 1 active name: {:?}",
        state.side_one.get_active().id
    );

    let mut state_instruction: StateInstruction = StateInstruction {
        percentage: 100.0,
        instruction_list: Vec::<Instruction>::new(),
    };

    let ins = generate_instructions_from_move(
        &mut state,
        _choice,
        SideReference::SideOne,
        &mut state_instruction,
    );

    println!(
        "After generate_instructions_from_move side 1 active name: {:?}",
        state.side_one.get_active().id
    );

    println!("Instructions Generated: {:?}", ins);
}
