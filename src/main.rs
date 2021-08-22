#![allow(dead_code)]

use std::collections::HashMap;
extern crate lazy_static;

mod data;
mod find_instructions;
mod helpers;
mod state;

fn main() {
    let mut pikachu: state::Pokemon = helpers::create_basic_pokemon("pikachu".to_string(), 100);
    let charizard: state::Pokemon = helpers::create_basic_pokemon("charizard".to_string(), 100);
    let blastoise: state::Pokemon = helpers::create_basic_pokemon("blastoise".to_string(), 100);
    let espeon: state::Pokemon = helpers::create_basic_pokemon("espeon".to_string(), 100);
    let snorlax: state::Pokemon = helpers::create_basic_pokemon("snorlax".to_string(), 100);
    let venusaur: state::Pokemon = helpers::create_basic_pokemon("venusaur".to_string(), 100);

    let landorustherian: state::Pokemon =
        helpers::create_basic_pokemon("landorustherian".to_string(), 100);
    let tapulele: state::Pokemon = helpers::create_basic_pokemon("tapulele".to_string(), 100);
    let rillaboom: state::Pokemon = helpers::create_basic_pokemon("rillaboom".to_string(), 100);
    let rhyperior: state::Pokemon = helpers::create_basic_pokemon("rhyperior".to_string(), 100);
    let gengar: state::Pokemon = helpers::create_basic_pokemon("gengar".to_string(), 100);
    let melmetal: state::Pokemon = helpers::create_basic_pokemon("melmetal".to_string(), 100);

    pikachu.moves.push("volttackle".to_string());
    pikachu.moves.push("voltswitch".to_string());
    pikachu.moves.push("irontail".to_string());
    pikachu.moves.push("surf".to_string());

    let my_side: state::Side = state::Side {
        active_index: 0,
        reserve: [pikachu, charizard, blastoise, espeon, snorlax, venusaur],
        side_conditions: HashMap::<data::moves::SideCondition, i8>::new(),
        wish: (0, 0),
    };

    let your_side: state::Side = state::Side {
        active_index: 0,
        reserve: [
            landorustherian,
            tapulele,
            rillaboom,
            rhyperior,
            gengar,
            melmetal,
        ],
        side_conditions: HashMap::<data::moves::SideCondition, i8>::new(),
        wish: (0, 0),
    };

    let state_weather = state::StateWeather {
        weather_type: state::Weather::None,
        turns_remaining: 0
    };

    let state_terrain = state::StateTerrain {
        terrain_type: state::Terrain::None,
        turns_remaining: 0
    };

    let mut state: state::State = state::State {
        side_one: my_side,
        side_two: your_side,
        weather: state_weather,
        terrain: state_terrain,
        trick_room: false,
    };

    state.side_one.reserve[state.side_one.active_index].speed = 5;
    state.side_one.reserve[state.side_one.active_index].item = "none".to_string();
    state.side_one.reserve[state.side_one.active_index].status = data::conditions::Status::Paralyze;
    state.side_one.reserve[state.side_one.active_index].ability = "quickfeet".to_string();

    state.side_one.reserve[state.side_one.active_index].speed = 4;
    // state.trick_room = true;
    // state.side_two.active.ability = "prankster".to_string();

    let s1_move = find_instructions::MoveChoice {
        move_type: find_instructions::MoveType::Move,
        choice: "tackle".to_string(),
    };
    let s2_move = find_instructions::MoveChoice {
        move_type: find_instructions::MoveType::Move,
        choice: "tackle".to_string(),
    };

    let s1mf = find_instructions::side_one_moves_first(&state, &s1_move, &s2_move);

    println!("Side one moves first: {}", s1mf);

    let mut transpose_instruction: find_instructions::TransposeInstruction = find_instructions::TransposeInstruction {
        state: state,
        percentage: 1.0,
        instructions: vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string()
        ]
    };

    let result1 = find_instructions::forking_random_chance(
        &mut transpose_instruction,
        0.75
    );

    let result_doesnt_matter = find_instructions::forking_random_chance(
        &mut transpose_instruction,
        1.0
    );

    let result2 = find_instructions::forking_random_chance(
        &mut transpose_instruction,
        0.75
    );

    println!("{}, {}: {}", result1, result2, transpose_instruction.percentage);

}
