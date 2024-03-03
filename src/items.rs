#![allow(unused_variables)]
use std::cmp;

use lazy_static::lazy_static;

use crate::choices::{Choice, Effect, MoveCategory, MoveTarget, Secondary, StatBoosts};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::{get_boost_instruction, immune_to_status};
use crate::instruction::{
    ChangeItemInstruction, ChangeStatusInstruction, DamageInstruction, DisableMoveInstruction,
    HealInstruction, Instruction, StateInstructions,
};
use crate::state::{Pokemon, PokemonType};
use crate::state::{PokemonBoostableStat, State, Terrain};
use crate::state::{PokemonStatus, SideReference};

type ItemBeforeMoveFn = fn(&mut State, &Choice, &SideReference, &mut StateInstructions);
type ModifyAttackBeingUsed = fn(&State, &mut Choice, &SideReference);
type ModifyAttackAgainst = fn(&State, &mut Choice, &SideReference);
type ItemOnSwitchInFn = fn(&State, &SideReference) -> Vec<Instruction>;
type ItemEndOfTurn = fn(&mut State, &SideReference, &mut StateInstructions);

#[non_exhaustive]
pub struct Items;

impl Items {
    pub const NONE: usize = 0;
    pub const ABSORB_BULB: usize = 1;
    pub const AIR_BALLOON: usize = 2;
    pub const ASSAULT_VEST: usize = 3;
    pub const BLACK_BELT: usize = 4;
    pub const BLACK_SLUDGE: usize = 5;
    pub const BLACK_GLASSES: usize = 6;
    pub const CELL_BATTERY: usize = 7;
    pub const CHARCOAL: usize = 8;
    pub const CHOICE_BAND: usize = 9;
    pub const CHOICE_SPECS: usize = 10;
    pub const CHOICE_SCARF: usize = 11;
    pub const DRAGON_FANG: usize = 12;
    pub const DREAD_PLATE: usize = 13;
    pub const ELECTRIC_SEED: usize = 14;
    pub const EXPERT_BELT: usize = 15;
    pub const EVIOLITE: usize = 16;
    pub const FAIRY_FEATHER: usize = 17;
    pub const FLAME_ORB: usize = 18;
    pub const GRASSY_SEED: usize = 19;
    pub const LEFTOVERS: usize = 20;
    pub const LIFE_ORB: usize = 21;
    pub const METAL_COAL: usize = 22;
    pub const MISTY_SEED: usize = 23;
    pub const MUSCLE_BAND: usize = 24;
    pub const MYSTIC_WATER: usize = 25;
    pub const NEVER_MELT_ICE: usize = 26;
    pub const ODD_INCENSE: usize = 27;
    pub const POISON_BARB: usize = 28;
    pub const PSYCHIC_SEED: usize = 29;
    pub const PUNCHING_GLOVE: usize = 30;
    pub const ROCKY_HELMET: usize = 31;
    pub const SEA_INCENSE: usize = 32;
    pub const SHARP_BEAK: usize = 33;
    pub const SHELL_BELL: usize = 34;
    pub const SILK_SCARF: usize = 35;
    pub const SILVER_POWDER: usize = 36;
    pub const SOFT_SAND: usize = 37;
    pub const THROAT_SPRAY: usize = 38;
    pub const TOXIC_ORB: usize = 39;
    pub const TWISTED_SPOON: usize = 40;
    pub const WAVE_INCENSE: usize = 41;
    pub const WEAKNESS_POLICY: usize = 42;
    pub const WISE_GLASSES: usize = 43;
    pub const BLUNDER_POLICY: usize = 44;
    pub const HEAVY_DUTY_BOOTS: usize = 45;
    pub const CLEAR_AMULET: usize = 46;
    pub const PROTECTIVE_PADS: usize = 47;
}

lazy_static! {
    pub static ref ITEMS_VEC: Vec<Item> = {
        let mut items: Vec<Item> = Vec::new();
        items.push(Item {
            id: "".to_string(),
            index: 0,
            ..Default::default()
        });
        items.push(Item {
            id: "absorbbulb".to_string(),
            index: 1,
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
        });
        items.push(Item {
            id: "airballoon".to_string(),
            index: 2,
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
        });
        items.push(Item {
            id: "assaultvest".to_string(),
            index: 3,
            modify_attack_against: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.category == MoveCategory::Special {
                    attacking_choice.base_power /= 1.5;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "blackbelt".to_string(),
            index: 4,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Fighting {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "blacksludge".to_string(),
            index: 5,
            end_of_turn: Some(
                |state: &mut State,
                 side_ref: &SideReference,
                 incoming_instructions: &mut StateInstructions| {
                    let active_pkmn = state.get_side(side_ref).get_active();
                    if active_pkmn.has_type(&PokemonType::Poison) {
                        if active_pkmn.hp < active_pkmn.maxhp {
                            let heal_amount = cmp::min(
                                active_pkmn.maxhp / 16,
                                active_pkmn.maxhp - active_pkmn.hp,
                            );
                            let ins = Instruction::Heal(HealInstruction {
                                side_ref: side_ref.clone(),
                                heal_amount: heal_amount,
                            });
                            active_pkmn.hp += heal_amount;
                            incoming_instructions.instruction_list.push(ins);
                        }
                    } else {
                        let damage_amount =
                            cmp::min(active_pkmn.maxhp / 16, active_pkmn.maxhp - active_pkmn.hp);
                        let ins = Instruction::Damage(DamageInstruction {
                            side_ref: side_ref.clone(),
                            damage_amount: damage_amount,
                        });
                        active_pkmn.hp -= damage_amount;
                        incoming_instructions.instruction_list.push(ins);
                    }
                },
            ),
            ..Default::default()
        });
        items.push(Item {
            id: "blackglasses".to_string(),
            index: 6,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Dark {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "cellbattery".to_string(),
            index: 7,
            modify_attack_against: Some(
                |_state, attacking_choice: &mut Choice, attacking_side_ref| {
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
                },
            ),
            ..Default::default()
        });
        items.push(Item {
            id: "charcoal".to_string(),
            index: 8,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Fire {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "choiceband".to_string(),
            index: 9,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.category == MoveCategory::Physical {
                    attacking_choice.base_power *= 1.3;
                }
            }),
            before_move: Some(
                |state: &mut State,
                 choice: &Choice,
                 side_ref: &SideReference,
                 incoming_instructions: &mut StateInstructions| {
                    let ins = get_choice_move_disable_instructions(
                        state.get_side_immutable(side_ref).get_active_immutable(),
                        side_ref,
                        &choice.move_id,
                    );
                    for i in ins {
                        state.apply_one_instruction(&i);
                        incoming_instructions.instruction_list.push(i);
                    }
                },
            ),
            ..Default::default()
        });
        items.push(Item {
            id: "choicespecs".to_string(),
            index: 10,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.category == MoveCategory::Special {
                    attacking_choice.base_power *= 1.3;
                }
            }),
            before_move: Some(
                |state: &mut State,
                 choice: &Choice,
                 side_ref: &SideReference,
                 incoming_instructions: &mut StateInstructions| {
                    let ins = get_choice_move_disable_instructions(
                        state.get_side_immutable(side_ref).get_active_immutable(),
                        side_ref,
                        &choice.move_id,
                    );
                    for i in ins {
                        state.apply_one_instruction(&i);
                        incoming_instructions.instruction_list.push(i);
                    }
                },
            ),
            ..Default::default()
        });
        items.push(Item {
            id: "choicescarf".to_string(),
            index: 11,
            before_move: Some(
                |state: &mut State,
                 choice: &Choice,
                 side_ref: &SideReference,
                 incoming_instructions: &mut StateInstructions| {
                    let ins = get_choice_move_disable_instructions(
                        state.get_side_immutable(side_ref).get_active_immutable(),
                        side_ref,
                        &choice.move_id,
                    );
                    for i in ins {
                        state.apply_one_instruction(&i);
                        incoming_instructions.instruction_list.push(i);
                    }
                },
            ),
            ..Default::default()
        });
        items.push(Item {
            id: "dragonfang".to_string(),
            index: 12,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Dragon {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "dreadplate".to_string(),
            index: 13,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Dark {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "electricseed".to_string(),
            index: 14,
            on_switch_in: Some(|state: &State, side_ref: &SideReference| {
                if state.terrain_is_active(&Terrain::ElectricTerrain) {
                    if let Some(boost_instruction) = get_boost_instruction(
                        state,
                        &PokemonBoostableStat::Defense,
                        &1,
                        side_ref,
                        side_ref,
                    ) {
                        return vec![
                            boost_instruction,
                            Instruction::ChangeItem(ChangeItemInstruction {
                                side_ref: side_ref.clone(),
                                current_item: Items::ELECTRIC_SEED,
                                new_item: Items::NONE,
                            }),
                        ];
                    }
                }
                return vec![];
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "expertbelt".to_string(),
            index: 15,
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
        });
        items.push(Item {
            id: "eviolite".to_string(),
            index: 16,
            modify_attack_against: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                attacking_choice.base_power /= 1.5;
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "fairyfeather".to_string(),
            index: 17,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Fairy {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "flameorb".to_string(),
            index: 18,
            end_of_turn: Some(
                |state: &mut State,
                 side_ref: &SideReference,
                 incoming_instructions: &mut StateInstructions| {
                    if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::Burn) {
                        let side = state.get_side(side_ref);
                        let ins = Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: side_ref.clone(),
                            pokemon_index: side.active_index,
                            new_status: PokemonStatus::Burn,
                            old_status: PokemonStatus::None,
                        });
                        side.get_active().status = PokemonStatus::Burn;
                        incoming_instructions.instruction_list.push(ins);
                    }
                },
            ),
            ..Default::default()
        });
        items.push(Item {
            id: "grassyseed".to_string(),
            index: 19,
            on_switch_in: Some(|state: &State, side_ref: &SideReference| {
                if state.terrain_is_active(&Terrain::GrassyTerrain) {
                    if let Some(boost_instruction) = get_boost_instruction(
                        state,
                        &PokemonBoostableStat::Defense,
                        &1,
                        side_ref,
                        side_ref,
                    ) {
                        return vec![
                            boost_instruction,
                            Instruction::ChangeItem(ChangeItemInstruction {
                                side_ref: side_ref.clone(),
                                current_item: Items::GRASSY_SEED,
                                new_item: Items::NONE,
                            }),
                        ];
                    }
                }
                return vec![];
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "leftovers".to_string(),
            index: 20,
            end_of_turn: Some(
                |state: &mut State,
                 side_ref: &SideReference,
                 incoming_instructions: &mut StateInstructions| {
                    let attacker = state.get_side(side_ref).get_active();
                    if attacker.hp < attacker.maxhp {
                        let heal_amount =
                            cmp::min(attacker.maxhp / 16, attacker.maxhp - attacker.hp);
                        let ins = Instruction::Heal(HealInstruction {
                            side_ref: side_ref.clone(),
                            heal_amount: heal_amount,
                        });
                        attacker.hp += heal_amount;
                        incoming_instructions.instruction_list.push(ins);
                    }
                },
            ),
            ..Default::default()
        });
        items.push(Item {
            id: "lifeorb".to_string(),
            index: 21,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                attacking_choice.base_power *= 1.3;
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Heal(-0.1),
                    target: MoveTarget::User,
                });
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "metalcoal".to_string(),
            index: 22,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Steel {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "mistyseed".to_string(),
            index: 23,
            on_switch_in: Some(|state: &State, side_ref: &SideReference| {
                if state.terrain_is_active(&Terrain::MistyTerrain) {
                    if let Some(boost_instruction) = get_boost_instruction(
                        state,
                        &PokemonBoostableStat::SpecialDefense,
                        &1,
                        side_ref,
                        side_ref,
                    ) {
                        return vec![
                            boost_instruction,
                            Instruction::ChangeItem(ChangeItemInstruction {
                                side_ref: side_ref.clone(),
                                current_item: Items::MISTY_SEED,
                                new_item: Items::NONE,
                            }),
                        ];
                    }
                }
                return vec![];
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "muscleband".to_string(),
            index: 24,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.category == MoveCategory::Physical {
                    attacking_choice.base_power *= 1.1;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "mysticwater".to_string(),
            index: 25,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Water {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "nevermeltice".to_string(),
            index: 26,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Ice {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "oddincense".to_string(),
            index: 27,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Psychic {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "poisonbarb".to_string(),
            index: 28,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Poison {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "psychicseed".to_string(),
            index: 29,
            on_switch_in: Some(|state: &State, side_ref: &SideReference| {
                if state.terrain_is_active(&Terrain::PsychicTerrain) {
                    if let Some(boost_instruction) = get_boost_instruction(
                        state,
                        &PokemonBoostableStat::SpecialDefense,
                        &1,
                        side_ref,
                        side_ref,
                    ) {
                        return vec![
                            boost_instruction,
                            Instruction::ChangeItem(ChangeItemInstruction {
                                side_ref: side_ref.clone(),
                                current_item: Items::PSYCHIC_SEED,
                                new_item: Items::NONE,
                            }),
                        ];
                    }
                }
                return vec![];
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "punchingglove".to_string(),
            index: 30,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.flags.punch {
                    attacking_choice.base_power *= 1.1;
                    attacking_choice.flags.contact = false
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "rockyhelmet".to_string(),
            index: 31,
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
        });
        items.push(Item {
            id: "seaincense".to_string(),
            index: 32,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Water {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "sharpbeak".to_string(),
            index: 33,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Flying {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "shellbell".to_string(),
            index: 34,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                attacking_choice.drain = Some(0.125);
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "silkscarf".to_string(),
            index: 35,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Normal {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "silverpowder".to_string(),
            index: 36,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Bug {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "softsand".to_string(),
            index: 37,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Ground {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "throatspray".to_string(),
            index: 38,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
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
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "toxicorb".to_string(),
            index: 39,
            end_of_turn: Some(
                |state: &mut State,
                 side_ref: &SideReference,
                 incoming_instructions: &mut StateInstructions| {
                    if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::Toxic)
                    {
                        let side = state.get_side(side_ref);
                        let ins = Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: side_ref.clone(),
                            pokemon_index: side.active_index,
                            new_status: PokemonStatus::Toxic,
                            old_status: PokemonStatus::None,
                        });
                        side.get_active().status = PokemonStatus::Toxic;
                        incoming_instructions.instruction_list.push(ins);
                    }
                },
            ),
            ..Default::default()
        });
        items.push(Item {
            id: "twistedspoon".to_string(),
            index: 40,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Psychic {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "waveincense".to_string(),
            index: 41,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Water {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "weaknesspolicy".to_string(),
            index: 42,
            modify_attack_against: Some(|state, attacking_choice: &mut Choice, side_ref| {
                if attacking_choice.category != MoveCategory::Status
                    && type_effectiveness_modifier(
                        &attacking_choice.move_type,
                        &state
                            .get_side_immutable(&side_ref.get_other_side())
                            .get_active_immutable()
                            .types,
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
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "wiseglasses".to_string(),
            index: 43,
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.category == MoveCategory::Special {
                    attacking_choice.base_power *= 1.1;
                }
            }),
            ..Default::default()
        });
        items.push(Item {
            id: "blunderpolicy".to_string(),
            index: 44,
            ..Default::default()
        });
        items.push(Item {
            id: "heavydutyboots".to_string(),
            index: 45,
            ..Default::default()
        });
        items.push(Item {
            id: "clearamulet".to_string(),
            index: 46,
            ..Default::default()
        });
        items.push(Item {
            id: "protectivepads".to_string(),
            index: 47,
            ..Default::default()
        });

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
    id: String,
    index: usize,
    pub before_move: Option<ItemBeforeMoveFn>,
    pub modify_attack_being_used: Option<ModifyAttackBeingUsed>,
    pub modify_attack_against: Option<ModifyAttackAgainst>,
    pub on_switch_in: Option<ItemOnSwitchInFn>,
    pub end_of_turn: Option<ItemEndOfTurn>,
}

impl Default for Item {
    fn default() -> Item {
        return Item {
            id: "".to_string(),
            index: 0,
            before_move: None,
            modify_attack_being_used: None,
            modify_attack_against: None,
            on_switch_in: None,
            end_of_turn: None,
        };
    }
}
