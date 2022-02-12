#![allow(dead_code)]

use std::collections::HashMap;
extern crate lazy_static;

mod data;
mod find_instructions;
mod helpers;
mod instruction;
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
    pikachu.ability = "naturalcure".to_string();
    pikachu.status = data::conditions::Status::Burn;

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
        turns_remaining: 0,
    };

    let state_terrain = state::StateTerrain {
        terrain_type: state::Terrain::None,
        turns_remaining: 0,
    };

    let state: state::State = state::State {
        side_one: my_side,
        side_two: your_side,
        weather: state_weather,
        terrain: state_terrain,
        trick_room: false,
    };

    let s1_move = find_instructions::MoveChoice {
        move_type: find_instructions::MoveType::Switch,
        choice: "charizard".to_string(),
    };
    let s2_move = find_instructions::MoveChoice {
        move_type: find_instructions::MoveType::Move,
        choice: "tackle".to_string(),
    };

    let s1mf = find_instructions::side_one_moves_first(&state, &s1_move, &s2_move);

    println!("Side one moves first: {}", s1mf);

    println!(
        "{} Status: {:?}",
        state.side_one.reserve[0].id,
        state.side_one.reserve[0].status
    );

    let list_of_instructions = find_instructions::find_all_instructions(state, s1_move, s2_move);

    for ins_set in list_of_instructions.into_iter() {
        println!(
            "{} Status: {:?}",
            ins_set.state.side_one.reserve[0].id,
            ins_set.state.side_one.reserve[0].status
        );

        for ins in ins_set.instructions {
            match ins {
                instruction::Instruction::Switch(a) => {
                    println!("Switch");
                    println!("\tis_side_one: {}", a.is_side_one);
                    println!("\tprevious_index: {}", a.previous_index);
                    println!("\tnext_index: {}", a.next_index);
                    println!("\n");
                }
                instruction::Instruction::RemoveVolatileStatus(a) => {
                    println!("Remove VolatileStatus");
                    println!("\tis_side_one: {}", a.is_side_one);
                    println!("\tvolatile_status: {:?}", a.volatile_status);
                    println!("\n");
                }
                instruction::Instruction::ChangeStatus(a) => {
                    println!("Change Status");
                    println!("\tis_side_one: {}", a.is_side_one);
                    println!("\tpkmn_index: {}", a.pokemon_index);
                    println!("\told_status: {:?}", a.old_status);
                    println!("\tnew_status: {:?}", a.new_status);
                    println!("\n");
                }
                // _ => {
                //     println!("Unhandled Instruction")
                // }
            }
        }
    }
}
