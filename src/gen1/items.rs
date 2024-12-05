#![allow(unused_variables)]

use super::instruction::{DisableMoveInstruction, Instruction};
use super::state::{Pokemon, SideReference};
use crate::choices::Choices;
use crate::define_enum_with_from_str;

pub fn get_choice_move_disable_instructions(
    pkmn: &Pokemon,
    side_ref: &SideReference,
    move_name: &Choices,
) -> Vec<Instruction> {
    let mut moves_to_disable = vec![];
    let mut iter = pkmn.moves.into_iter();
    while let Some(p) = iter.next() {
        if &p.id != move_name && p.disabled == false {
            moves_to_disable.push(Instruction::DisableMove(DisableMoveInstruction {
                side_ref: *side_ref,
                move_index: iter.pokemon_move_index,
            }));
        }
    }
    moves_to_disable
}

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    Items {
        NONE,
        UNKNOWNITEM,
    },
    default = NONE
}
