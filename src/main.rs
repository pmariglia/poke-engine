#![allow(dead_code)]

use std::collections::HashMap;

use data::moves::{self, Choice, Flags, MoveCategory};

use crate::data::{moves::{Secondary, MoveTarget, Effect}, conditions::PokemonStatus};
extern crate lazy_static;

mod data;
mod instruction;

fn main() {
    let mut sample_choice: moves::Choice = Choice{
        id: "tackle".to_string(),
        accuracy: 100 as f32,
        category: MoveCategory::Physical,
        base_power: 40 as f32,
        boost: None,
        priority: 0,
        flags: Flags {
            authentic: false,
            bite: false,
            bullet: false,
            charge: false,
            contact: false,
            dance: false,
            defrost: false,
            distance: false,
            drag: false,
            gravity: false,
            heal: false,
            mirror: false,
            mystery: false,
            nonsky: false,
            powder: false,
            protect: false,
            pulse: false,
            punch: false,
            recharge: false,
            reflectable: false,
            snatch: false,
            sound: false,
        },
        heal: None,
        status: None,
        volatile_status: None,
        side_condition: None,
        secondaries: Some(vec![
            Secondary {
                chance: 50,
                target: MoveTarget::Opponent,
                effect: Effect::Status(PokemonStatus::Burn)
            }
        ]),
        target: MoveTarget::Opponent
    };

    println!("{:?}", sample_choice);
}
