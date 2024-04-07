#![allow(unused_variables)]
use std::cmp;

use crate::choices::{Choice, Choices, Effect, MoveCategory, MoveTarget, Secondary, StatBoosts};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::{get_boost_instruction, immune_to_status};
use crate::instruction::{
    ChangeItemInstruction, ChangeStatusInstruction, DamageInstruction, DisableMoveInstruction,
    HealInstruction, Instruction, StateInstructions,
};
use crate::state::{Pokemon, PokemonType};
use crate::state::{PokemonBoostableStat, State, Terrain};
use crate::state::{PokemonStatus, SideReference};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Items {
    NONE,
    UNKNOWNITEM,
    ABSORBBULB,
    AIRBALLOON,
    ASSAULTVEST,
    BLACKBELT,
    BLACKSLUDGE,
    BLACKGLASSES,
    CELLBATTERY,
    CHARCOAL,
    CHOICEBAND,
    CHOICESPECS,
    CHOICESCARF,
    DRAGONFANG,
    DREADPLATE,
    ELECTRICSEED,
    EXPERTBELT,
    EVIOLITE,
    FAIRYFEATHER,
    FLAMEORB,
    GRASSYSEED,
    LEFTOVERS,
    LIFEORB,
    METALCOAT,
    MISTYSEED,
    MUSCLEBAND,
    MYSTICWATER,
    NEVERMELTICE,
    ODDINCENSE,
    POISONBARB,
    PSYCHICSEED,
    PUNCHINGGLOVE,
    ROCKYHELMET,
    SEAINCENSE,
    SHARPBEAK,
    SHELLBELL,
    SILKSCARF,
    SILVERPOWDER,
    SOFTSAND,
    SOULDEW,
    THROATSPRAY,
    TOXICORB,
    TWISTEDSPOON,
    WAVEINCENSE,
    WEAKNESSPOLICY,
    WISEGLASSES,
    BLUNDERPOLICY,
    HEAVYDUTYBOOTS,
    CLEARAMULET,
    PROTECTIVEPADS,
}

fn get_choice_move_disable_instructions(
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
    return moves_to_disable;
}

pub fn item_before_move(
    state: &mut State,
    choice: &Choice,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let active_pkmn = state.get_side_immutable(side_ref).get_active_immutable();
    match active_pkmn.item {
        Items::CHOICESPECS | Items::CHOICEBAND | Items::CHOICESCARF => {
            let ins = get_choice_move_disable_instructions(active_pkmn, side_ref, &choice.move_id);
            for i in ins {
                state.apply_one_instruction(&i);
                instructions.instruction_list.push(i);
            }
        }
        _ => {}
    }
}

pub fn item_on_switch_in(
    state: &mut State,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let switching_in_pkmn = state.get_side_immutable(side_ref).get_active_immutable();
    match switching_in_pkmn.item {
        Items::ELECTRICSEED => {
            if state.terrain_is_active(&Terrain::ElectricTerrain) {
                if let Some(boost_instruction) = get_boost_instruction(
                    switching_in_pkmn,
                    &PokemonBoostableStat::Defense,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                    state.get_side(side_ref).get_active().item = Items::NONE;
                    instructions.instruction_list.push(Instruction::ChangeItem(
                        ChangeItemInstruction {
                            side_ref: side_ref.clone(),
                            current_item: Items::ELECTRICSEED,
                            new_item: Items::NONE,
                        },
                    ));
                }
            }
        }
        Items::GRASSYSEED => {
            if state.terrain_is_active(&Terrain::GrassyTerrain) {
                if let Some(boost_instruction) = get_boost_instruction(
                    switching_in_pkmn,
                    &PokemonBoostableStat::Defense,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                    state.get_side(side_ref).get_active().item = Items::NONE;
                    instructions.instruction_list.push(Instruction::ChangeItem(
                        ChangeItemInstruction {
                            side_ref: side_ref.clone(),
                            current_item: Items::GRASSYSEED,
                            new_item: Items::NONE,
                        },
                    ));
                }
            }
        }
        Items::MISTYSEED => {
            if state.terrain_is_active(&Terrain::MistyTerrain) {
                if let Some(boost_instruction) = get_boost_instruction(
                    switching_in_pkmn,
                    &PokemonBoostableStat::SpecialDefense,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                    state.get_side(side_ref).get_active().item = Items::NONE;
                    instructions.instruction_list.push(Instruction::ChangeItem(
                        ChangeItemInstruction {
                            side_ref: side_ref.clone(),
                            current_item: Items::MISTYSEED,
                            new_item: Items::NONE,
                        },
                    ));
                }
            }
        }
        Items::PSYCHICSEED => {
            if state.terrain_is_active(&Terrain::PsychicTerrain) {
                if let Some(boost_instruction) = get_boost_instruction(
                    switching_in_pkmn,
                    &PokemonBoostableStat::SpecialDefense,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                    state.get_side(side_ref).get_active().item = Items::NONE;
                    instructions.instruction_list.push(Instruction::ChangeItem(
                        ChangeItemInstruction {
                            side_ref: side_ref.clone(),
                            current_item: Items::PSYCHICSEED,
                            new_item: Items::NONE,
                        },
                    ));
                }
            }
        }
        _ => {}
    }
}

pub fn item_end_of_turn(
    state: &mut State,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let active_pkmn = state.get_side(side_ref).get_active();
    match active_pkmn.item {
        Items::BLACKSLUDGE => {
            if active_pkmn.has_type(&PokemonType::Poison) {
                if active_pkmn.hp < active_pkmn.maxhp {
                    let heal_amount =
                        cmp::min(active_pkmn.maxhp / 16, active_pkmn.maxhp - active_pkmn.hp);
                    let ins = Instruction::Heal(HealInstruction {
                        side_ref: side_ref.clone(),
                        heal_amount: heal_amount,
                    });
                    active_pkmn.hp += heal_amount;
                    instructions.instruction_list.push(ins);
                }
            } else {
                let damage_amount =
                    cmp::min(active_pkmn.maxhp / 16, active_pkmn.maxhp - active_pkmn.hp);
                let ins = Instruction::Damage(DamageInstruction {
                    side_ref: side_ref.clone(),
                    damage_amount: damage_amount,
                });
                active_pkmn.hp -= damage_amount;
                instructions.instruction_list.push(ins);
            }
        }
        Items::FLAMEORB => {
            if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::Burn) {
                let side = state.get_side(side_ref);
                let ins = Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: side_ref.clone(),
                    pokemon_index: side.active_index,
                    new_status: PokemonStatus::Burn,
                    old_status: PokemonStatus::None,
                });
                side.get_active().status = PokemonStatus::Burn;
                instructions.instruction_list.push(ins);
            }
        }
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
        Items::TOXICORB => {
            if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::Toxic) {
                let side = state.get_side(side_ref);
                let ins = Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: side_ref.clone(),
                    pokemon_index: side.active_index,
                    new_status: PokemonStatus::Toxic,
                    old_status: PokemonStatus::None,
                });
                side.get_active().status = PokemonStatus::Toxic;
                instructions.instruction_list.push(ins);
            }
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
        Items::ABSORBBULB => {
            if attacking_choice.move_type == PokemonType::Water {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                    target: MoveTarget::Opponent,
                });
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::Opponent,
                });
            }
        }
        Items::AIRBALLOON => {
            if attacking_choice.move_type == PokemonType::Ground
                && attacking_choice.move_id != Choices::THOUSANDARROWS
            {
                attacking_choice.base_power = 0.0;
            } else {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::Opponent,
                });
            }
        }
        Items::ASSAULTVEST => {
            if attacking_choice.category == MoveCategory::Special {
                attacking_choice.base_power /= 1.5;
            }
        }
        Items::CELLBATTERY => {
            if attacking_choice.move_type == PokemonType::Electric {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Boost(StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                    target: MoveTarget::Opponent,
                });
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::Opponent,
                });
            }
        }
        Items::EVIOLITE => {
            attacking_choice.base_power /= 1.5;
        }
        Items::ROCKYHELMET => {
            if attacking_choice.flags.contact {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Heal(-0.166),
                    target: MoveTarget::User,
                })
            }
        }
        Items::WEAKNESSPOLICY => {
            if attacking_choice.category != MoveCategory::Status
                && type_effectiveness_modifier(
                    &attacking_choice.move_type,
                    &defending_side.get_active_immutable().types,
                ) > 1.0
            {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Boost(StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                    target: MoveTarget::Opponent,
                });
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::Opponent,
                });
            }
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
            if attacking_choice.move_type == PokemonType::Fighting {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::BLACKGLASSES => {
            if attacking_choice.move_type == PokemonType::Dark {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::CHARCOAL => {
            if attacking_choice.move_type == PokemonType::Fire {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::CHOICEBAND => {
            if attacking_choice.category == MoveCategory::Physical {
                attacking_choice.base_power *= 1.5;
            }
        }
        Items::CHOICESPECS => {
            if attacking_choice.category == MoveCategory::Special {
                attacking_choice.base_power *= 1.5;
            }
        }
        Items::DRAGONFANG => {
            if attacking_choice.move_type == PokemonType::Dragon {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::DREADPLATE => {
            if attacking_choice.move_type == PokemonType::Dark {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::EXPERTBELT => {
            if type_effectiveness_modifier(
                &attacking_choice.move_type,
                &defending_side.get_active_immutable().types,
            ) > 1.0
            {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::FAIRYFEATHER => {
            if attacking_choice.move_type == PokemonType::Fairy {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::LIFEORB => {
            if attacking_choice.category != MoveCategory::Status {
                attacking_choice.base_power *= 1.3;
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Heal(-0.1),
                    target: MoveTarget::User,
                });
            }
        }
        Items::METALCOAT => {
            if attacking_choice.move_type == PokemonType::Steel {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::MUSCLEBAND => {
            if attacking_choice.category == MoveCategory::Physical {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::MYSTICWATER => {
            if attacking_choice.move_type == PokemonType::Water {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::NEVERMELTICE => {
            if attacking_choice.move_type == PokemonType::Ice {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::ODDINCENSE => {
            if attacking_choice.move_type == PokemonType::Psychic {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::POISONBARB => {
            if attacking_choice.move_type == PokemonType::Poison {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::PUNCHINGGLOVE => {
            if attacking_choice.flags.punch {
                attacking_choice.base_power *= 1.1;
                attacking_choice.flags.contact = false
            }
        }
        Items::SEAINCENSE => {
            if attacking_choice.move_type == PokemonType::Water {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SHARPBEAK => {
            if attacking_choice.move_type == PokemonType::Flying {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SHELLBELL => {
            attacking_choice.drain = Some(0.125);
        }
        Items::SILKSCARF => {
            if attacking_choice.move_type == PokemonType::Normal {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SILVERPOWDER => {
            if attacking_choice.move_type == PokemonType::Bug {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SOFTSAND => {
            if attacking_choice.move_type == PokemonType::Ground {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SOULDEW => {
            if attacking_side.get_active_immutable().id == "latios"
                || attacking_side.get_active_immutable().id == "latias"
            {
                #[cfg(any(feature = "gen4", feature = "gen5", feature = "gen6"))]
                if attacking_choice.category == MoveCategory::Special
                {
                    attacking_choice.base_power *= 1.5;
                }

                #[cfg(not(any(feature = "gen4", feature = "gen5", feature = "gen6")))]
                if attacking_choice.move_type == PokemonType::Dragon
                    || attacking_choice.move_type == PokemonType::Psychic
                {
                    attacking_choice.base_power *= 1.2;
                }
            }
        }
        Items::THROATSPRAY => {
            if attacking_choice.flags.sound {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                    target: MoveTarget::User,
                });
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::User,
                });
            }
        }
        Items::TWISTEDSPOON => {
            if attacking_choice.move_type == PokemonType::Psychic {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::WAVEINCENSE => {
            if attacking_choice.move_type == PokemonType::Water {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::WISEGLASSES => {
            if attacking_choice.category == MoveCategory::Special {
                attacking_choice.base_power *= 1.1;
            }
        }
        _ => {}
    }
}
