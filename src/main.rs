#![allow(dead_code)]

use poke_engine::choices::{Choices, MOVES};
use poke_engine::search::{expectiminimax_search, pick_safest};
use poke_engine::state::{Move, Pokemon, PokemonIndex, PokemonMoveIndex, PokemonMoves, Side, State};
use std::mem;
use std::process::exit;
use poke_engine::io::command_loop;

extern crate lazy_static;

fn main() {

    // command_loop();
    // exit(1);

    let mut state = State::deserialize(
        "charmander,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-bulbasaur,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-squirtle,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-pidgey,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-pikachu,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-weedle,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;33-1-0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0-0-0-false-NONE/charmander,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-bulbasaur,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-squirtle,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-pikachu,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-pidgey,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-weedle,100,Normal,Typeless,100,100,INTIMIDATE,NONE,100,100,100,100,100,0,0,0,0,0,0,0,None,25,Serious,,TACKLE;false;32,UTURN;false;32,ZAPCANNON;false;32,LEECHSEED;false;32-1-0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0-0-0-false-NONE/None;0/None;0/false"
    );

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
    let s1_len = side_one_options.len();
    let s2_len = side_two_options.len();

    let result = expectiminimax_search(&mut state, 4, side_one_options, side_two_options, true);

    for i in 0..s1_len {
        for j in 0..s2_len {
            let index = i * s2_len + j;
            print!("{:?} ", result[index]);
        }
        print!("\n");
    }
    println!("{:?}", pick_safest(s1_len, s2_len, result));
}
