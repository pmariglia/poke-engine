use std::cmp;
use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::choices::{Choice, MoveCategory, MoveTarget};
use crate::generate_instructions::immune_to_status;
use crate::instruction::{
    ChangeStatusInstruction, DamageInstruction, HealInstruction, Instruction,
};
use crate::state::PokemonType;
use crate::state::State;
use crate::state::{PokemonStatus, SideReference};

type ModifyAttackBeingUsed = fn(&State, &mut Choice, &SideReference);
type ModifyAttackAgainst = fn(&State, &mut Choice, &SideReference);
type ItemOnSwitchInFn = fn(&State, &SideReference) -> Vec<Instruction>;
type ItemEndOfTurn = fn(&State, &SideReference) -> Vec<Instruction>;

lazy_static! {
    pub static ref ITEMS: HashMap<String, Item> = {
        let mut items: HashMap<String, Item> = HashMap::new();
        items.insert(
            "choiceband".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.category == MoveCategory::Physical {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "leftovers".to_string(),
            Item {
                end_of_turn: Some(|state: &State, side_ref: &SideReference| {
                    let attacker = state.get_side_immutable(side_ref).get_active_immutable();
                    if attacker.hp < attacker.maxhp {
                        return vec![
                            Instruction::Heal(HealInstruction {
                                side_ref: side_ref.clone(),
                                heal_amount: cmp::min(attacker.maxhp / 16, attacker.maxhp - attacker.hp),
                            }),
                        ];

                    }
                    return vec![];
                }),
                ..Default::default()
            },
        );
        items.insert(
            "blacksludge".to_string(),
            Item {
                end_of_turn: Some(|state: &State, side_ref: &SideReference| {
                    let attacker = state.get_side_immutable(side_ref).get_active_immutable();
                    if attacker.has_type(&PokemonType::Poison) {
                        if attacker.hp < attacker.maxhp {
                            return vec![
                                Instruction::Heal(HealInstruction {
                                    side_ref: side_ref.clone(),
                                    heal_amount: cmp::min(attacker.maxhp / 16, attacker.maxhp - attacker.hp),
                                }),
                            ];
                        }
                    }
                    return vec![
                        Instruction::Damage(DamageInstruction {
                            side_ref: side_ref.clone(),
                            damage_amount: cmp::min(attacker.maxhp / 16, attacker.maxhp - attacker.hp),
                        }),
                    ];
                }),
                ..Default::default()
            },
        );
        items.insert(
            "flameorb".to_string(),
            Item {
                end_of_turn: Some(|state: &State, side_ref: &SideReference| {
                    let side = state.get_side_immutable(side_ref);
                    if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::Burn){
                        return vec![
                            Instruction::ChangeStatus(ChangeStatusInstruction {
                                side_ref: side_ref.clone(),
                                pokemon_index: side.active_index,
                                new_status: PokemonStatus::Burn,
                                old_status: PokemonStatus::None,
                            }),
                        ];
                    }
                    return vec![];
                }),
                ..Default::default()
            },
        );
        items.insert(
            "toxicorb".to_string(),
            Item {
                end_of_turn: Some(|state: &State, side_ref: &SideReference| {
                    let side = state.get_side_immutable(side_ref);
                    if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::Toxic){
                        return vec![
                            Instruction::ChangeStatus(ChangeStatusInstruction {
                                side_ref: side_ref.clone(),
                                pokemon_index: side.active_index,
                                new_status: PokemonStatus::Toxic,
                                old_status: PokemonStatus::None,
                            }),
                        ];
                    }
                    return vec![];
                }),
                ..Default::default()
            },
        );
        // items.insert(
        //     "rockyhelmet".to_string(),
        //     Item {
        //         modify_attack_against: Some(
        //             |_state, attacking_choice: &mut Choice, _side_ref| {
        //                 if attacking_choice.flags.contact {
        //                     if let Some(secondaries) = attacking_choice.secondaries {
        //
        //                     }
        //                 }
        //             },
        //         ),
        //         ..Default::default()
        //     },
        // );
        items.insert(
            "airballoon".to_string(),
            Item {
                modify_attack_against: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                    if attacking_choice.move_type == PokemonType::Ground
                        && attacking_choice.move_id != "thousandarrows"
                    {
                        attacking_choice.base_power = 0.0;
                    }
                }),
                ..Default::default()
            },
        );

        items
    };
}

pub struct Item {
    pub modify_attack_being_used: Option<ModifyAttackBeingUsed>,
    pub modify_attack_against: Option<ModifyAttackAgainst>,
    pub on_switch_in: Option<ItemOnSwitchInFn>,
    pub end_of_turn: Option<ItemEndOfTurn>,
}

impl Default for Item {
    fn default() -> Item {
        return Item {
            modify_attack_being_used: None,
            modify_attack_against: None,
            on_switch_in: None,
            end_of_turn: None,
        };
    }
}
