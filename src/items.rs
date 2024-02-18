use std::cmp;
use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::choices::{Choice, Effect, Heal, MoveCategory, MoveTarget, Secondary, StatBoosts};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::immune_to_status;
use crate::instruction::{
    ChangeItemInstruction, ChangeStatusInstruction, DamageInstruction, DisableMoveInstruction,
    HealInstruction, Instruction,
};
use crate::state::{Pokemon, PokemonType};
use crate::state::{PokemonStatus, SideReference};
use crate::state::{Side, State};

type ItemBeforeMoveFn = fn(&State, &Choice, &SideReference) -> Vec<Instruction>;
type ModifyAttackBeingUsed = fn(&State, &mut Choice, &SideReference);
type ModifyAttackAgainst = fn(&State, &mut Choice, &SideReference);
type ItemOnSwitchInFn = fn(&State, &SideReference) -> Vec<Instruction>;
type ItemEndOfTurn = fn(&State, &SideReference) -> Vec<Instruction>;

lazy_static! {
    pub static ref ITEMS: HashMap<String, Item> = {
        let mut items: HashMap<String, Item> = HashMap::new();
        items.insert(
            "absorbbulb".to_string(),
            Item {
                modify_attack_against: Some(
                    |_state, attacking_choice: &mut Choice, attacking_side_ref| {
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
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "airballoon".to_string(),
            Item {
                modify_attack_against: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                    if attacking_choice.move_type == PokemonType::Ground
                        && attacking_choice.move_id != "thousandarrows"
                    {
                        attacking_choice.base_power = 0.0;
                    } else {
                        attacking_choice.add_or_create_secondaries(Secondary {
                            chance: 100.0,
                            effect: Effect::RemoveItem,
                            target: MoveTarget::Opponent,
                        });
                    }
                }),
                ..Default::default()
            },
        );
        items.insert(
            "assaultvest".to_string(),
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
        items.insert(
            "blackbelt".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Fighting {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
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
                            return vec![Instruction::Heal(HealInstruction {
                                side_ref: side_ref.clone(),
                                heal_amount: cmp::min(
                                    attacker.maxhp / 16,
                                    attacker.maxhp - attacker.hp,
                                ),
                            })];
                        }
                    }
                    return vec![Instruction::Damage(DamageInstruction {
                        side_ref: side_ref.clone(),
                        damage_amount: cmp::min(attacker.maxhp / 16, attacker.maxhp - attacker.hp),
                    })];
                }),
                ..Default::default()
            },
        );
        items.insert(
            "blackglasses".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Dark {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "charcoal".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Fire {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
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
                before_move: Some(|state, choice, side_ref| {
                    return get_choice_move_disable_instructions(
                        state.get_side_immutable(side_ref).get_active_immutable(),
                        side_ref,
                        &choice.move_id,
                    );
                }),
                ..Default::default()
            },
        );
        items.insert(
            "choicespecs".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.category == MoveCategory::Special {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                before_move: Some(|state, choice, side_ref| {
                    return get_choice_move_disable_instructions(
                        state.get_side_immutable(side_ref).get_active_immutable(),
                        side_ref,
                        &choice.move_id,
                    );
                }),
                ..Default::default()
            },
        );
        items.insert(
            "choicescarf".to_string(),
            Item {
                before_move: Some(|state, choice, side_ref| {
                    return get_choice_move_disable_instructions(
                        state.get_side_immutable(side_ref).get_active_immutable(),
                        side_ref,
                        &choice.move_id,
                    );
                }),
                ..Default::default()
            },
        );
        items.insert(
            "dragonfang".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Dragon {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "dreadplate".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Dark {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "expertbelt".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |state: &State, attacking_choice: &mut Choice, side_ref: &SideReference| {
                        if type_effectiveness_modifier(
                            &attacking_choice.move_type,
                            &state
                                .get_side_immutable(&side_ref.get_other_side())
                                .get_active_immutable()
                                .types,
                        ) > 1.0
                        {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "fairyfeather".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Fairy {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "flameorb".to_string(),
            Item {
                end_of_turn: Some(|state: &State, side_ref: &SideReference| {
                    let side = state.get_side_immutable(side_ref);
                    if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::Burn) {
                        return vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: side_ref.clone(),
                            pokemon_index: side.active_index,
                            new_status: PokemonStatus::Burn,
                            old_status: PokemonStatus::None,
                        })];
                    }
                    return vec![];
                }),
                ..Default::default()
            },
        );
        items.insert(
            "leftovers".to_string(),
            Item {
                end_of_turn: Some(|state: &State, side_ref: &SideReference| {
                    let attacker = state.get_side_immutable(side_ref).get_active_immutable();
                    if attacker.hp < attacker.maxhp {
                        return vec![Instruction::Heal(HealInstruction {
                            side_ref: side_ref.clone(),
                            heal_amount: cmp::min(
                                attacker.maxhp / 16,
                                attacker.maxhp - attacker.hp,
                            ),
                        })];
                    }
                    return vec![];
                }),
                ..Default::default()
            },
        );
        items.insert(
            "lifeorb".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        attacking_choice.base_power *= 1.3;
                        attacking_choice.add_or_create_secondaries(Secondary {
                            chance: 100.0,
                            effect: Effect::Heal(-0.1),
                            target: MoveTarget::User,
                        });
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "metalcoal".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Steel {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "muscleband".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.category == MoveCategory::Physical {
                            attacking_choice.base_power *= 1.1;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "mysticwater".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Water {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "nevermeltice".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Ice {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "oddincense".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Psychic {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "poisonbarb".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Poison {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "punchingglove".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.flags.punch {
                            attacking_choice.base_power *= 1.1;
                            attacking_choice.flags.contact = false
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "rockyhelmet".to_string(),
            Item {
                modify_attack_against: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                    if attacking_choice.flags.contact {
                        attacking_choice.add_or_create_secondaries(Secondary {
                            chance: 100.0,
                            effect: Effect::Heal(-0.125),
                            target: MoveTarget::User,
                        })
                    }
                }),
                ..Default::default()
            },
        );
        items.insert(
            "seaincense".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Water {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "sharpbeak".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Flying {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "shellbell".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        attacking_choice.drain = Some(0.125);
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "silkscarf".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Normal {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "silverpowder".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Bug {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "softsand".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Ground {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "toxicorb".to_string(),
            Item {
                end_of_turn: Some(|state: &State, side_ref: &SideReference| {
                    let side = state.get_side_immutable(side_ref);
                    if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::Toxic)
                    {
                        return vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: side_ref.clone(),
                            pokemon_index: side.active_index,
                            new_status: PokemonStatus::Toxic,
                            old_status: PokemonStatus::None,
                        })];
                    }
                    return vec![];
                }),
                ..Default::default()
            },
        );
        items.insert(
            "twistedspoon".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Psychic {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "waveincense".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.move_type == PokemonType::Water {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        items.insert(
            "wiseglasses".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.category == MoveCategory::Special {
                            attacking_choice.base_power *= 1.1;
                        }
                    },
                ),
                ..Default::default()
            },
        );

        items
    };
}

fn get_choice_move_disable_instructions(
    pkmn: &Pokemon,
    side_ref: &SideReference,
    move_name: &String,
) -> Vec<Instruction> {
    let mut moves_to_disable = vec![];
    for (i, m) in pkmn.moves.iter().enumerate() {
        if &m.id != move_name {
            moves_to_disable.push(Instruction::DisableMove(DisableMoveInstruction {
                side_ref: *side_ref,
                move_index: i,
            }));
        }
    }
    return moves_to_disable;
}

pub struct Item {
    pub before_move: Option<ItemBeforeMoveFn>,
    pub modify_attack_being_used: Option<ModifyAttackBeingUsed>,
    pub modify_attack_against: Option<ModifyAttackAgainst>,
    pub on_switch_in: Option<ItemOnSwitchInFn>,
    pub end_of_turn: Option<ItemEndOfTurn>,
}

impl Default for Item {
    fn default() -> Item {
        return Item {
            before_move: None,
            modify_attack_being_used: None,
            modify_attack_against: None,
            on_switch_in: None,
            end_of_turn: None,
        };
    }
}
