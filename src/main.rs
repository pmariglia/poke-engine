#![allow(dead_code)]

use poke_engine::choices::{Choices, MOVES};
use poke_engine::search::expectiminimax_search;
use poke_engine::state::{Move, Pokemon, PokemonIndex, PokemonMoveIndex, PokemonMoves, Side, State};
use std::mem;
use std::process::exit;
use poke_engine::io::command_loop;

extern crate lazy_static;

fn main() {

    command_loop();
    exit(1);

    println!("{:?}", State::deserialize(
        "charmander,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        1\
        -\
        0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0\
        -0-0\
        -true-TACKLE\
        /\
        squirtle,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        id,100,Normal,Typeless,100,100,INTIMIDATE,LEFTOVERS,100,100,100,100,100,1,1,1,1,1,1,1,\
        None,25,Serious,,TACKLE;false;32,EARTHQUAKE;false;32,ZAPCANNON;false;32,LEECHSEED;false;32\
        -\
        1\
        -\
        0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0\
        -0-0\
        -true-TACKLE\
        /\
        None;5\
        /\
        None;4\
        /\
        false"
    ))

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
    // let (side_one_options, side_two_options) = state.get_all_options();

    // let _result = expectiminimax_search(&mut state, 3, side_one_options, side_two_options, false);
    // println!("{:?}", state);
}
