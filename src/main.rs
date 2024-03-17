#![allow(dead_code)]

use poke_engine::choices::{Choices, MOVES};
use poke_engine::search::expectiminimax_search;
use poke_engine::state::{Move, PokemonIndex, PokemonMoves, State};
use std::mem;
use std::process::exit;

extern crate lazy_static;

fn main() {
    // println!("Choice: {}", mem::size_of::<Choice>());
    // println!("move id: {}", mem::size_of::<String>());
    // println!("PokemonIndex: {}", mem::size_of::<PokemonIndex>());
    // println!("PokemonType: {}", mem::size_of::<PokemonType>());
    // println!("accuracy: {}", mem::size_of::<f32>());
    // println!("MoveCategory: {}", mem::size_of::<MoveCategory>());
    // println!("base_power: {}", mem::size_of::<f32>());
    // println!("boost: {}", mem::size_of::<Option<Boost>>());
    // println!("priority: {}", mem::size_of::<i8>());
    // println!("flags: {}", mem::size_of::<Flags>());
    // println!("drain: {}", mem::size_of::<Option<f32>>());
    // println!("recoil: {}", mem::size_of::<Option<f32>>());
    // println!("Heal: {}", mem::size_of::<Option<Heal>>());
    // println!("Status: {}", mem::size_of::<Option<Status>>());
    // println!("Volatilestatus: {}", mem::size_of::<Option<VolatileStatus>>());
    // println!("SideCondition: {}", mem::size_of::<Option<SideCondition>>());
    // println!("Option<Vec<Secondary>>: {}", mem::size_of::<Option<Vec<Secondary>>>());
    // println!("ModifyChoiceFn: {}", mem::size_of::<Option<ModifyChoiceFn>>());
    // println!("AfterDamageHitFn: {}", mem::size_of::<Option<AfterDamageHitFn>>());
    // println!("HazardClearFn: {}", mem::size_of::<Option<HazardClearFn>>());
    // println!("MoveSPecialEffectFn: {}", mem::size_of::<Option<MoveSpecialEffectFn>>());
    // exit(1);

    let mut state: State = State::default();

    // state.side_one.pokemon[PokemonIndex::P0].moves = PokemonMoves {
    //     m0: Move {
    //         id: Choices::EARTHQUAKE,
    //         disabled: false,
    //         pp: 32,
    //         choice: MOVES.get(&Choices::EARTHQUAKE).unwrap().to_owned(),
    //     },
    //     m1: Move {
    //         id: Choices::THUNDERWAVE,
    //         disabled: false,
    //         pp: 32,
    //         choice: MOVES.get(&Choices::THUNDERWAVE).unwrap().to_owned(),
    //     },
    //     m2: Move {
    //         id: Choices::ZAPCANNON,
    //         disabled: false,
    //         pp: 32,
    //         choice: MOVES.get(&Choices::ZAPCANNON).unwrap().to_owned(),
    //     },
    //     m3: Move {
    //         id: Choices::AIRSLASH,
    //         disabled: false,
    //         pp: 32,
    //         choice: MOVES.get(&Choices::AIRSLASH).unwrap().to_owned(),
    //     },
    // };
    //
    // state.side_two.pokemon[PokemonIndex::P0].moves = PokemonMoves {
    //     m0: Move {
    //         id: Choices::EARTHQUAKE,
    //         disabled: false,
    //         pp: 32,
    //         choice: MOVES.get(&Choices::EARTHQUAKE).unwrap().to_owned(),
    //     },
    //     m1: Move {
    //         id: Choices::THUNDERWAVE,
    //         disabled: false,
    //         pp: 32,
    //         choice: MOVES.get(&Choices::THUNDERWAVE).unwrap().to_owned(),
    //     },
    //     m2: Move {
    //         id: Choices::ZAPCANNON,
    //         disabled: false,
    //         pp: 32,
    //         choice: MOVES.get(&Choices::ZAPCANNON).unwrap().to_owned(),
    //     },
    //     m3: Move {
    //         id: Choices::AIRSLASH,
    //         disabled: false,
    //         pp: 32,
    //         choice: MOVES.get(&Choices::AIRSLASH).unwrap().to_owned(),
    //     },
    // };

    // println!("{:?}", state);
    let (side_one_options, side_two_options) = state.get_all_options();

    let _result = expectiminimax_search(&mut state, 3, side_one_options, side_two_options, false);
    // println!("{:?}", state);
}
