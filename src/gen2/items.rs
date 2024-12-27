#![allow(unused_variables)]
use super::instruction::{
    ChangeItemInstruction, ChangeStatusInstruction, DisableMoveInstruction, HealInstruction,
    Instruction, StateInstructions,
};
use super::state::State;
use super::state::{Pokemon, PokemonType};
use super::state::{PokemonStatus, Side, SideReference};
use crate::choices::{Choice, Choices};
use crate::define_enum_with_from_str;
use crate::pokemon::PokemonName;
use std::cmp;

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    Items {
        NONE,
        UNKNOWNITEM,
        LEFTOVERS,
        METALPOWDER,
        BLACKBELT,
        BLACKGLASSES,
        CHARCOAL,
        DRAGONFANG,
        DRAGONSCALE,
        METALCOAT,
        MYSTICWATER,
        NEVERMELTICE,
        PINKBOW,
        POLKADOTBOW,
        POISONBARB,
        SHARPBEAK,
        SILVERPOWDER,
        SOFTSAND,
        THICKCLUB,
        TWISTEDSPOON,
        HARDSTONE,
        MAGNET,
        LIGHTBALL,
        MIRACLEBERRY,
        MINTBERRY,
    },
    default = UNKNOWNITEM
}

fn miracle_berry(
    side_ref: &SideReference,
    attacking_side: &mut Side,
    instructions: &mut StateInstructions,
) {
    let active_index = attacking_side.active_index;
    let active_pkmn = attacking_side.get_active();
    instructions
        .instruction_list
        .push(Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: *side_ref,
            pokemon_index: active_index,
            new_status: PokemonStatus::NONE,
            old_status: active_pkmn.status,
        }));
    active_pkmn.status = PokemonStatus::NONE;
    instructions
        .instruction_list
        .push(Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: *side_ref,
            current_item: Items::MIRACLEBERRY,
            new_item: Items::NONE,
        }));
    active_pkmn.item = Items::NONE;
}

fn mint_berry(
    side_ref: &SideReference,
    attacking_side: &mut Side,
    instructions: &mut StateInstructions,
) {
    let active_index = attacking_side.active_index;
    let active_pkmn = attacking_side.get_active();
    instructions
        .instruction_list
        .push(Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: *side_ref,
            pokemon_index: active_index,
            new_status: PokemonStatus::NONE,
            old_status: active_pkmn.status,
        }));
    active_pkmn.status = PokemonStatus::NONE;
    instructions
        .instruction_list
        .push(Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: *side_ref,
            current_item: Items::MINTBERRY,
            new_item: Items::NONE,
        }));
    active_pkmn.item = Items::NONE;
}

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

pub fn item_before_move(
    state: &mut State,
    choice: &mut Choice,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let active_pkmn = attacking_side.get_active();
    let defending_pkmn = defending_side.get_active();
    match active_pkmn.item {
        Items::MIRACLEBERRY if active_pkmn.status != PokemonStatus::NONE => {
            miracle_berry(side_ref, attacking_side, instructions)
        }
        Items::MINTBERRY if active_pkmn.status == PokemonStatus::SLEEP => {
            mint_berry(side_ref, attacking_side, instructions)
        }
        _ => {}
    }
}

pub fn item_end_of_turn(
    state: &mut State,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let attacking_side = state.get_side(side_ref);
    let active_pkmn = attacking_side.get_active();
    match active_pkmn.item {
        Items::LEFTOVERS => {
            let attacker = state.get_side(side_ref).get_active();
            if attacker.hp < attacker.maxhp {
                let heal_amount = cmp::min(attacker.maxhp / 16, attacker.maxhp - attacker.hp);
                let ins = Instruction::Heal(HealInstruction {
                    side_ref: side_ref.clone(),
                    heal_amount: heal_amount,
                });
                attacker.hp += heal_amount;
                instructions.instruction_list.push(ins);
            }
        }
        Items::MIRACLEBERRY if active_pkmn.status != PokemonStatus::NONE => {
            miracle_berry(side_ref, attacking_side, instructions)
        }
        Items::MINTBERRY if active_pkmn.status == PokemonStatus::SLEEP => {
            mint_berry(side_ref, attacking_side, instructions)
        }
        _ => {}
    }
}

pub fn item_modify_attack_against(
    state: &State,
    attacking_choice: &mut Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    match defending_side.get_active_immutable().item {
        Items::METALPOWDER if attacking_side.get_active_immutable().id == PokemonName::DITTO => {
            attacking_choice.base_power /= 1.5;
        }
        _ => {}
    }
}

pub fn item_modify_attack_being_used(
    state: &State,
    attacking_choice: &mut Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    match attacking_side.get_active_immutable().item {
        Items::BLACKBELT => {
            if attacking_choice.move_type == PokemonType::FIGHTING {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::BLACKGLASSES => {
            if attacking_choice.move_type == PokemonType::DARK {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::CHARCOAL => {
            if attacking_choice.move_type == PokemonType::FIRE {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::DRAGONFANG | Items::DRAGONSCALE => {
            if attacking_choice.move_type == PokemonType::DRAGON {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::METALCOAT => {
            if attacking_choice.move_type == PokemonType::STEEL {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::MYSTICWATER => {
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::NEVERMELTICE => {
            if attacking_choice.move_type == PokemonType::ICE {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::PINKBOW | Items::POLKADOTBOW => {
            if attacking_choice.move_type == PokemonType::NORMAL {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::POISONBARB => {
            if attacking_choice.move_type == PokemonType::POISON {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SHARPBEAK => {
            if attacking_choice.move_type == PokemonType::FLYING {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SILVERPOWDER => {
            if attacking_choice.move_type == PokemonType::BUG {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SOFTSAND => {
            if attacking_choice.move_type == PokemonType::GROUND {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::THICKCLUB => match attacking_side.get_active_immutable().id {
            PokemonName::CUBONE
            | PokemonName::MAROWAK
            | PokemonName::MAROWAKALOLA
            | PokemonName::MAROWAKALOLATOTEM => {
                attacking_choice.base_power *= 2.0;
            }
            _ => {}
        },
        Items::TWISTEDSPOON => {
            if attacking_choice.move_type == PokemonType::PSYCHIC {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::HARDSTONE => {
            if attacking_choice.move_type == PokemonType::ROCK {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::MAGNET => {
            if attacking_choice.move_type == PokemonType::ELECTRIC {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::LIGHTBALL => {
            if attacking_side
                .get_active_immutable()
                .id
                .is_pikachu_variant()
            {
                attacking_choice.base_power *= 2.0;
            }
        }
        _ => {}
    }
}
