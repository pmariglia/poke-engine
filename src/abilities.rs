#![allow(unused_variables)]
use std::cmp;

use lazy_static::lazy_static;

use crate::choices::{
    Choice, Effect, Heal, MoveCategory, MoveTarget, Secondary, StatBoosts, VolatileStatus,
};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::get_boost_instruction;
use crate::instruction::{
    BoostInstruction, ChangeStatusInstruction, ChangeType, HealInstruction, Instruction,
    StateInstructions,
};
use crate::state::{PokemonBoostableStat, PokemonType, Terrain};
use crate::state::{PokemonStatus, State};
use crate::state::{PokemonVolatileStatus, SideReference, Weather};

type ModifyAttackBeingUsed = fn(&State, &mut Choice, &Choice, &SideReference);
type ModifyAttackAgainst = fn(&State, &mut Choice, &Choice, &SideReference);
type AbilityBeforeMove = fn(&mut State, &Choice, &SideReference, &mut StateInstructions);
type AbilityAfterDamageHit = fn(&mut State, &Choice, &SideReference, i16, &mut StateInstructions);
type AbilityOnSwitchOut = fn(&mut State, &SideReference, &mut StateInstructions);
type AbilityOnSwitchIn = fn(&mut State, &SideReference, &mut StateInstructions);
type AbilityEndOfTurn = fn(&mut State, &SideReference, &mut StateInstructions);

lazy_static! {
    pub static ref ABILITIES: Vec<Ability> = {
        let mut abilities: Vec<Ability> = Vec::new();
        abilities.push(
            Ability {
                id: "ripen".to_string(),
                index: 0,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "tangledfeet".to_string(),
                index: 1,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "dragonsmaw".to_string(),
                index: 2,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Dragon {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "clearbody".to_string(),
                index: 3,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "galvanize".to_string(),
                index: 4,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Normal {
                            attacking_choice.move_type = PokemonType::Electric;
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "vitalspirit".to_string(),
                index: 5,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "aerilate".to_string(),
                index: 6,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Normal {
                            attacking_choice.move_type = PokemonType::Flying;
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "defiant".to_string(),
                index: 7,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "cutecharm".to_string(),
                index: 8,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "neuroforce".to_string(),
                index: 9,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if type_effectiveness_modifier(
                            &attacking_choice.move_type,
                            &state
                                .get_side_immutable(&attacking_side.get_other_side())
                                .get_active_immutable()
                                .types,
                        ) > 1.0
                        {
                            attacking_choice.base_power *= 1.25;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "soundproof".to_string(),
                index: 10,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "rkssystem".to_string(),
                index: 11,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "poisonpoint".to_string(),
                index: 12,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.contact {
                            attacking_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 30.0,
                                    target: MoveTarget::Opponent,
                                    effect: Effect::Status(PokemonStatus::Poison),
                                }
                            )
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "stakeout".to_string(),
                index: 13,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if defender_choice.category == MoveCategory::Switch {
                            attacking_choice.base_power *= 2.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "unnerve".to_string(),
                index: 14,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "rockhead".to_string(),
                index: 15,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "aurabreak".to_string(),
                index: 16,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "mimicry".to_string(),
                index: 17,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "bulletproof".to_string(),
                index: 18,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.flags.bullet {
                            attacker_choice.accuracy = 0.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "powerofalchemy".to_string(),
                index: 19,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "technician".to_string(),
                index: 20,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.base_power <= 60.0 {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "multiscale".to_string(),
                index: 21,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "arenatrap".to_string(),
                index: 22,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "battlebond".to_string(),
                index: 23,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "disguise".to_string(),
                index: 24,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "earlybird".to_string(),
                index: 25,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "lightningrod".to_string(),
                index: 26,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "magician".to_string(),
                index: 27,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "refrigerate".to_string(),
                index: 28,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Normal {
                            attacking_choice.move_type = PokemonType::Ice;
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "friendguard".to_string(),
                index: 29,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "noability".to_string(),
                index: 30,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "gulpmissile".to_string(),
                index: 31,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "powerconstruct".to_string(),
                index: 32,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "forecast".to_string(),
                index: 33,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "prankster".to_string(),
                index: 34,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "protean".to_string(),
                index: 35,
                before_move: Some(|state: &mut State, choice: &Choice, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                    let active_pkmn = state.get_side_immutable(side_ref).get_active_immutable();
                    if !active_pkmn.has_type(&choice.move_type) {
                        let ins = Instruction::ChangeType(ChangeType {
                            side_ref: *side_ref,
                            new_types: (choice.move_type, PokemonType::Typeless),
                            old_types: active_pkmn.types,
                        });
                        state.apply_one_instruction(&ins);
                        incoming_instructions.instruction_list.push(ins);
                    }
                }),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "asoneglastrier".to_string(),
                index: 36,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "shadowtag".to_string(),
                index: 37,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "skilllink".to_string(),
                index: 38,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "intrepidsword".to_string(),
                index: 39,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "soulheart".to_string(),
                index: 40,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "swiftswim".to_string(),
                index: 41,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "eartheater".to_string(),
                index: 42,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.move_type == PokemonType::Ground {
                            attacker_choice.base_power = 0.0;
                            attacker_choice.heal = Some(Heal {
                                target: MoveTarget::Opponent,
                                amount: 0.25
                            });
                            attacker_choice.category = MoveCategory::Status;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "superluck".to_string(),
                index: 43,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "supremeoverlord".to_string(),
                index: 44,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let mut boost_amount = 1.0;
                        let side = state.get_side_immutable(attacking_side);
                        boost_amount += 0.1 * side.num_alive_pkmn() as f32;
                        attacking_choice.base_power *= boost_amount;
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "insomnia".to_string(),
                index: 45,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "dancer".to_string(),
                index: 46,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "steamengine".to_string(),
                index: 47,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "angerpoint".to_string(),
                index: 48,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "contrary".to_string(),
                index: 49,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "magmaarmor".to_string(),
                index: 50,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "hungerswitch".to_string(),
                index: 51,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "receiver".to_string(),
                index: 52,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "zenmode".to_string(),
                index: 53,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "emergencyexit".to_string(),
                index: 54,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "illusion".to_string(),
                index: 55,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "weakarmor".to_string(),
                index: 56,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "drought".to_string(),
                index: 57,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "innardsout".to_string(),
                index: 58,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "shieldsdown".to_string(),
                index: 59,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "adaptability".to_string(),
                index: 60,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if state
                            .get_side_immutable(attacking_side)
                            .get_active_immutable()
                            .has_type(&attacking_choice.move_type)
                        {
                            attacking_choice.base_power *= 4.0 / 3.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "corrosion".to_string(),
                index: 61,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "longreach".to_string(),
                index: 62,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        attacking_choice.flags.contact = false;
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "purepower".to_string(),
                index: 63,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.category == MoveCategory::Physical {
                            attacking_choice.base_power *= 2.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "tintedlens".to_string(),
                index: 64,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if type_effectiveness_modifier(
                            &attacking_choice.move_type,
                            &state
                                .get_side_immutable(&attacking_side.get_other_side())
                                .get_active_immutable()
                                .types,
                        ) < 1.0
                        {
                            attacking_choice.base_power *= 2.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "queenlymajesty".to_string(),
                index: 65,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "desolateland".to_string(),
                index: 66,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "moxie".to_string(),
                index: 67,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sapsipper".to_string(),
                index: 68,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "slushrush".to_string(),
                index: 69,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "bigpecks".to_string(),
                index: 70,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "stall".to_string(),
                index: 71,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "whitesmoke".to_string(),
                index: 72,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "flareboost".to_string(),
                index: 73,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if state.get_side_immutable(attacking_side).get_active_immutable().status == PokemonStatus::Burn {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "shadowshield".to_string(),
                index: 74,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "liquidvoice".to_string(),
                index: 75,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.sound {
                            attacking_choice.move_type = PokemonType::Water;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "mistysurge".to_string(),
                index: 76,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "multitype".to_string(),
                index: 77,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "noguard".to_string(),
                index: 78,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        attacking_choice.accuracy = 100.0
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "torrent".to_string(),
                index: 79,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                        if attacking_choice.move_type == PokemonType::Water && attacking_pokemon.hp < attacking_pokemon.maxhp / 3 {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "deltastream".to_string(),
                index: 80,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "klutz".to_string(),
                index: 81,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "libero".to_string(),
                index: 82,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "serenegrace".to_string(),
                index: 83,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if let Some(secondaries) = &mut attacking_choice.secondaries {
                            for secondary in secondaries.iter_mut() {
                                secondary.chance *= 2.0;
                            }
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "cursedbody".to_string(),
                index: 84,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "unaware".to_string(),
                index: 85,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "lightmetal".to_string(),
                index: 86,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "marvelscale".to_string(),
                index: 87,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "telepathy".to_string(),
                index: 88,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "quickdraw".to_string(),
                index: 89,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "hypercutter".to_string(),
                index: 90,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "symbiosis".to_string(),
                index: 91,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "plus".to_string(),
                index: 92,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "mirrorarmor".to_string(),
                index: 93,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "pastelveil".to_string(),
                index: 94,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "toughclaws".to_string(),
                index: 95,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.contact {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "effectspore".to_string(),
                index: 96,
                modify_attack_against: Some(
                    |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                        if attacker_choice.flags.contact {
                            attacker_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 9.0,
                                    target: MoveTarget::User,
                                    effect: Effect::Status(PokemonStatus::Poison),
                                }
                            );
                            attacker_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 10.0,
                                    target: MoveTarget::User,
                                    effect: Effect::Status(PokemonStatus::Paralyze),
                                }
                            );
                            attacker_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 11.0,
                                    target: MoveTarget::User,
                                    effect: Effect::Status(PokemonStatus::Sleep),
                                }
                            );
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "mummy".to_string(),
                index: 97,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "baddreams".to_string(),
                index: 98,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "magicguard".to_string(),
                index: 99,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sandstream".to_string(),
                index: 100,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "powerspot".to_string(),
                index: 101,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "flamebody".to_string(),
                index: 102,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if state.move_makes_contact(&attacker_choice, attacking_side) {
                            let burn_secondary = Secondary {
                                chance: 30.0,
                                target: MoveTarget::User,
                                effect: Effect::Status(PokemonStatus::Burn),
                            };

                            if attacker_choice.secondaries.is_none() {
                                attacker_choice.secondaries = Some(vec![burn_secondary]);
                            } else {
                                attacker_choice
                                    .secondaries
                                    .as_mut()
                                    .unwrap()
                                    .push(burn_secondary);
                            }
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "reckless".to_string(),
                index: 103,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.crash.is_some() {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "pressure".to_string(),
                index: 104,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "gooey".to_string(),
                index: 105,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.flags.contact {
                            attacker_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 100.0,
                                    target: MoveTarget::User,
                                    effect: Effect::Boost(
                                        StatBoosts {
                                            attack: 0,
                                            defense: 0,
                                            special_attack: 0,
                                            special_defense: 0,
                                            speed: -1,
                                            accuracy: 0,
                                        }
                                    ),
                                }
                            )
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "immunity".to_string(),
                index: 106,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "leafguard".to_string(),
                index: 107,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "hugepower".to_string(),
                index: 108,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.category == MoveCategory::Physical {
                            attacking_choice.base_power *= 2.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "solarpower".to_string(),
                index: 109,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if state.weather_is_active(&Weather::Sun) {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "schooling".to_string(),
                index: 110,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "motordrive".to_string(),
                index: 111,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "anticipation".to_string(),
                index: 112,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "merciless".to_string(),
                index: 113,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "trace".to_string(),
                index: 114,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "naturalcure".to_string(),
                index: 115,
                on_switch_out: Some(|state: &mut State, side_reference: &SideReference, instructions: &mut StateInstructions| {
                    let side = state.get_side(side_reference);
                    let active_index = side.active_index;
                    let active = side.get_active();
                    if active.status != PokemonStatus::None {
                        instructions.instruction_list.push(
                            Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: *side_reference,
                            pokemon_index: active_index,
                            old_status: active.status,
                            new_status: PokemonStatus::None,
                            })
                        );
                        active.status = PokemonStatus::None;
                    }
                }),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "harvest".to_string(),
                index: 116,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "suctioncups".to_string(),
                index: 117,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "iceface".to_string(),
                index: 118,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "roughskin".to_string(),
                index: 119,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "wonderguard".to_string(),
                index: 120,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "waterveil".to_string(),
                index: 121,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "fairyaura".to_string(),
                index: 122,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Fairy {
                            attacking_choice.base_power *= 1.33;
                        }
                    },
                ),
                modify_attack_against: Some(
                    |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                        if attacker_choice.move_type == PokemonType::Fairy {
                            attacker_choice.base_power *= 1.33;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sandspit".to_string(),
                index: 123,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "intimidate".to_string(),
                index: 124,
                on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                    let target_side_ref = side_ref.get_other_side();
                    if let Some(boost_instruction) = get_boost_instruction(
                        state.get_side_immutable(&target_side_ref).get_active_immutable(),
                        &PokemonBoostableStat::Attack,
                        &-1,
                        side_ref,
                        &target_side_ref,
                    ) {
                        state.apply_one_instruction(&boost_instruction);
                        instructions.instruction_list.push(boost_instruction);
                    }
                }),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "dauntlessshield".to_string(),
                index: 125,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "aromaveil".to_string(),
                index: 126,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "airlock".to_string(),
                index: 127,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "normalize".to_string(),
                index: 128,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        attacking_choice.move_type = PokemonType::Normal;
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "darkaura".to_string(),
                index: 129,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Dark {
                            attacking_choice.base_power *= 1.33;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "victorystar".to_string(),
                index: 130,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        attacking_choice.accuracy *= 1.1;
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "grassysurge".to_string(),
                index: 131,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sturdy".to_string(),
                index: 132,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "pickpocket".to_string(),
                index: 133,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "electricsurge".to_string(),
                index: 134,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "runaway".to_string(),
                index: 135,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "oblivious".to_string(),
                index: 136,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "surgesurfer".to_string(),
                index: 137,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "levitate".to_string(),
                index: 138,
                modify_attack_against: Some(
                    |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                        if attacker_choice.move_type == PokemonType::Ground
                            && attacker_choice.target == MoveTarget::Opponent
                            && attacker_choice.move_id != "thousandarrows"
                        {
                            attacker_choice.base_power = 0.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "asonespectrier".to_string(),
                index: 139,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "pickup".to_string(),
                index: 140,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "icebody".to_string(),
                index: 141,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "curiousmedicine".to_string(),
                index: 142,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "flowerveil".to_string(),
                index: 143,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "static".to_string(),
                index: 144,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if state.move_makes_contact(&attacker_choice, attacking_side) {
                            attacker_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 30.0,
                                    target: MoveTarget::User,
                                    effect: Effect::Status(PokemonStatus::Paralyze),
                                }
                            )
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "wonderskin".to_string(),
                index: 145,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "overgrow".to_string(),
                index: 146,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                        if attacking_choice.move_type == PokemonType::Grass && attacking_pokemon.hp < attacking_pokemon.maxhp / 3 {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "propellertail".to_string(),
                index: 147,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "thickfat".to_string(),
                index: 148,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "gluttony".to_string(),
                index: 149,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "keeneye".to_string(),
                index: 150,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "mountaineer".to_string(),
                index: 151,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "flashfire".to_string(),
                index: 152,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.move_type == PokemonType::Fire {
                            attacker_choice.base_power = 0.0;
                            attacker_choice.volatile_status = Some(VolatileStatus {
                                target: MoveTarget::Opponent,
                                volatile_status: PokemonVolatileStatus::FlashFire,
                            });
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "compoundeyes".to_string(),
                index: 153,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        attacking_choice.accuracy *= 1.3;
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "steelworker".to_string(),
                index: 154,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if defender_choice.move_type == PokemonType::Steel {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "comatose".to_string(),
                index: 155,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "ballfetch".to_string(),
                index: 156,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "dazzling".to_string(),
                index: 157,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.priority > 0 {
                            attacker_choice.accuracy = 0.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "download".to_string(),
                index: 158,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "transistor".to_string(),
                index: 159,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Electric {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "moldbreaker".to_string(),
                index: 160,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "liquidooze".to_string(),
                index: 161,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "poisonheal".to_string(),
                index: 162,
                end_of_turn: Some(|state: &mut State,
                 side_ref: &SideReference,
                 incoming_instructions: &mut StateInstructions| {
                    let attacker = state.get_side(side_ref).get_active();
                    if attacker.hp < attacker.maxhp
                        && (attacker.status == PokemonStatus::Poison
                            || attacker.status == PokemonStatus::Toxic)
                    {
                        let heal_amount = cmp::min(attacker.maxhp / 8, attacker.maxhp - attacker.hp);
                        let ins = Instruction::Heal(HealInstruction {
                            side_ref: side_ref.clone(),
                            heal_amount: heal_amount,
                        });
                        attacker.hp += heal_amount;
                        incoming_instructions.instruction_list.push(ins);

                    }
                }),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "prismarmor".to_string(),
                index: 163,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sniper".to_string(),
                index: 164,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "stench".to_string(),
                index: 165,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let mut already_flinches = false;
                        if let Some(secondaries) = &mut attacking_choice.secondaries {
                            for secondary in secondaries.iter() {
                                if secondary.effect == Effect::VolatileStatus(PokemonVolatileStatus::Flinch) {
                                    already_flinches = true;
                                }
                            }
                        }
                        if !already_flinches {
                            attacking_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 10.0,
                                    target: MoveTarget::Opponent,
                                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                                }
                            )
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "competitive".to_string(),
                index: 166,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "swarm".to_string(),
                index: 167,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                        if attacking_choice.move_type == PokemonType::Bug && attacking_pokemon.hp < attacking_pokemon.maxhp / 3 {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "stalwart".to_string(),
                index: 168,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "illuminate".to_string(),
                index: 169,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "turboblaze".to_string(),
                index: 170,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "gorillatactics".to_string(),
                index: 171,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.category == MoveCategory::Physical {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "speedboost".to_string(),
                index: 172,
                end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                    let attacker = state.get_side(side_ref).get_active();
                    if attacker.speed_boost < 6 {
                        let ins = Instruction::Boost(BoostInstruction {
                            side_ref: side_ref.clone(),
                            stat: PokemonBoostableStat::Speed,
                            amount: 1,
                        });
                        attacker.speed_boost += 1;
                        incoming_instructions.instruction_list.push(ins);
                    }
                }),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "heatproof".to_string(),
                index: 173,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.move_type == PokemonType::Fire {
                            attacker_choice.base_power *= 0.5 ;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "snowcloak".to_string(),
                index: 174,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "teravolt".to_string(),
                index: 175,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "chillingneigh".to_string(),
                index: 176,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "shielddust".to_string(),
                index: 177,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "rivalry".to_string(),
                index: 178,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "primordialsea".to_string(),
                index: 179,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "screencleaner".to_string(),
                index: 180,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "magnetpull".to_string(),
                index: 181,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "honeygather".to_string(),
                index: 182,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "cottondown".to_string(),
                index: 183,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "grasspelt".to_string(),
                index: 184,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if state.terrain_is_active(&Terrain::GrassyTerrain) && attacker_choice.category == MoveCategory::Physical {
                            attacker_choice.base_power /= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "battlearmor".to_string(),
                index: 185,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "beastboost".to_string(),
                index: 186,
                after_damage_hit: Some(|state, _, attacking_side, damage_dealt, instructions| {
                    let (attacker_side, defender_side) =
                        state.get_both_sides(attacking_side);
                    if damage_dealt > 0 && defender_side.get_active_immutable().hp == 0 {
                        let highest_stat = &attacker_side
                            .get_active_immutable()
                            .calculate_highest_stat();
                        if let Some(boost_instruction) = get_boost_instruction(
                            state.get_side_immutable(attacking_side).get_active_immutable(),
                            highest_stat,
                            &1,
                            attacking_side,
                            attacking_side,
                        ) {
                            state.apply_one_instruction(&boost_instruction);
                            instructions.instruction_list.push(boost_instruction);
                        }
                    }
                }),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "berserk".to_string(),
                index: 187,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "minus".to_string(),
                index: 188,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "raindish".to_string(),
                index: 189,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "synchronize".to_string(),
                index: 190,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "filter".to_string(),
                index: 191,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if type_effectiveness_modifier(
                            &attacker_choice.move_type,
                            &state
                                .get_side_immutable(&attacking_side.get_other_side())
                                .get_active_immutable()
                                .types,
                        ) > 1.0
                        {
                            attacker_choice.base_power *= 0.75;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "truant".to_string(),
                index: 192,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "furcoat".to_string(),
                index: 193,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.category == MoveCategory::Physical {
                            attacker_choice.base_power *= 0.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "fullmetalbody".to_string(),
                index: 194,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "regenerator".to_string(),
                index: 195,
                on_switch_out: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                    let switching_out_pkmn =
                        state.get_side(side_ref).get_active();
                    let hp_recovered = cmp::min(
                        switching_out_pkmn.maxhp / 3,
                        switching_out_pkmn.maxhp - switching_out_pkmn.hp,
                    );

                    if hp_recovered > 0 && switching_out_pkmn.hp > 0 {
                        instructions.instruction_list.push(Instruction::Heal(HealInstruction {
                            side_ref: *side_ref,
                            heal_amount: hp_recovered,
                        }));
                        switching_out_pkmn.hp += hp_recovered;
                    }
                }),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "forewarn".to_string(),
                index: 196,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "ironbarbs".to_string(),
                index: 197,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.flags.contact {
                            attacker_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 100.0,
                                    target: MoveTarget::User,
                                    effect: Effect::Heal(-0.125),
                                }
                            );
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "stamina".to_string(),
                index: 198,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sandrush".to_string(),
                index: 199,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "colorchange".to_string(),
                index: 200,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "blaze".to_string(),
                index: 201,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                        if attacking_choice.move_type == PokemonType::Fire && attacking_pokemon.hp < attacking_pokemon.maxhp / 3 {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "analytic".to_string(),
                index: 202,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if !attacking_choice.first_move {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "tanglinghair".to_string(),
                index: 203,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "cloudnine".to_string(),
                index: 204,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "steelyspirit".to_string(),
                index: 205,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "quickfeet".to_string(),
                index: 206,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "magicbounce".to_string(),
                index: 207,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "megalauncher".to_string(),
                index: 208,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.pulse {
                            attacking_choice.base_power *= 1.5;
                        };
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "heavymetal".to_string(),
                index: 209,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "stormdrain".to_string(),
                index: 210,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "pixilate".to_string(),
                index: 211,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Normal {
                            attacking_choice.move_type = PokemonType::Fairy;
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "watercompaction".to_string(),
                index: 212,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "justified".to_string(),
                index: 213,
                modify_attack_against: Some(
                    |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                        if attacker_choice.move_type == PokemonType::Dark {
                            attacker_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 100.0,
                                    target: MoveTarget::Opponent,
                                    effect: Effect::Boost(
                                        StatBoosts {
                                            attack: 1,
                                            defense: 0,
                                            special_attack: 0,
                                            special_defense: 0,
                                            speed: 0,
                                            accuracy: 0,
                                        }
                                    ),
                                }
                            )
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "slowstart".to_string(),
                index: 214,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "snowwarning".to_string(),
                index: 215,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "flowergift".to_string(),
                index: 216,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "shedskin".to_string(),
                index: 217,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "wimpout".to_string(),
                index: 218,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "icescales".to_string(),
                index: 219,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.category == MoveCategory::Special {
                            attacker_choice.base_power *= 0.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "infiltrator".to_string(),
                index: 220,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "limber".to_string(),
                index: 221,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "psychicsurge".to_string(),
                index: 222,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "defeatist".to_string(),
                index: 223,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                        if attacking_pokemon.hp < attacking_pokemon.maxhp / 2 {
                            attacking_choice.base_power *= 0.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "waterabsorb".to_string(),
                index: 224,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.move_type == PokemonType::Water {
                            attacker_choice.base_power = 0.0;
                            attacker_choice.heal = Some(Heal {
                                target: MoveTarget::Opponent,
                                amount: 0.25
                            });
                            attacker_choice.category = MoveCategory::Status;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "imposter".to_string(),
                index: 225,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "dryskin".to_string(),
                index: 226,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.move_type == PokemonType::Water {
                            attacker_choice.base_power = 0.0;
                            attacker_choice.heal = Some(Heal {
                                target: MoveTarget::Opponent,
                                amount: 0.25
                            });
                            attacker_choice.category = MoveCategory::Status;
                        } else if attacker_choice.move_type == PokemonType::Fire {
                            attacker_choice.base_power *= 1.25;
                        }
                    },
                ),
                end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                    if state.weather_is_active(&Weather::Rain) {
                        let active_pkmn = state.get_side(side_ref).get_active();

                        if active_pkmn.hp < active_pkmn.maxhp {
                            let heal_amount = cmp::min(active_pkmn.maxhp / 8, active_pkmn.maxhp - active_pkmn.hp);
                            let ins = Instruction::Heal(HealInstruction {
                                side_ref: side_ref.clone(),
                                heal_amount: heal_amount,
                            });
                            active_pkmn.hp += heal_amount;
                            incoming_instructions.instruction_list.push(ins);
                        }
                    }
                }),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "fluffy".to_string(),
                index: 227,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.flags.contact {
                            attacker_choice.base_power *= 0.5;
                        }
                        if attacker_choice.move_type == PokemonType::Fire {
                            attacker_choice.base_power *= 2.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "unburden".to_string(),
                index: 228,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "cheekpouch".to_string(),
                index: 229,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "stancechange".to_string(),
                index: 230,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "moody".to_string(),
                index: 231,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "rockypayload".to_string(),
                index: 232,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.move_type == PokemonType::Rock {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "punkrock".to_string(),
                index: 233,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.sound {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sandveil".to_string(),
                index: 234,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "parentalbond".to_string(),
                index: 235,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "strongjaw".to_string(),
                index: 236,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.bite {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "battery".to_string(),
                index: 237,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.category == MoveCategory::Special {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "healer".to_string(),
                index: 238,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "steadfast".to_string(),
                index: 239,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "damp".to_string(),
                index: 240,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if ["selfdestruct", "explosion", "mindblown", "mistyexplosion"].contains(&attacker_choice.move_id.as_str()) {
                            attacker_choice.accuracy = 0.0;
                            attacker_choice.heal = None;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "perishbody".to_string(),
                index: 241,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "triage".to_string(),
                index: 242,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sheerforce".to_string(),
                index: 243,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.secondaries.is_some() {
                            attacking_choice.base_power *= 1.3;
                            attacking_choice.secondaries = None
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "owntempo".to_string(),
                index: 244,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "frisk".to_string(),
                index: 245,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "voltabsorb".to_string(),
                index: 246,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "galewings".to_string(),
                index: 247,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "aftermath".to_string(),
                index: 248,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "stickyhold".to_string(),
                index: 249,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "grimneigh".to_string(),
                index: 250,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "ironfist".to_string(),
                index: 251,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.punch {
                            attacking_choice.base_power *= 1.2;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "rebound".to_string(),
                index: 252,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "unseenfist".to_string(),
                index: 253,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.contact {
                            attacking_choice.flags.protect = false
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "solidrock".to_string(),
                index: 254,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "hustle".to_string(),
                index: 255,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.category == MoveCategory::Physical {
                            attacking_choice.base_power *= 1.5;
                            attacking_choice.accuracy *= 0.80
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "hydration".to_string(),
                index: 256,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "scrappy".to_string(),
                index: 257,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if state.get_side_immutable(&attacking_side.get_other_side()).get_active_immutable().has_type(&PokemonType::Ghost) {
                            // Technically wrong, come back to this later
                            attacking_choice.move_type = PokemonType::Typeless;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "overcoat".to_string(),
                index: 258,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "neutralizinggas".to_string(),
                index: 259,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sweetveil".to_string(),
                index: 260,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "drizzle".to_string(),
                index: 261,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "innerfocus".to_string(),
                index: 262,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "poisontouch".to_string(),
                index: 263,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "wanderingspirit".to_string(),
                index: 264,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "guts".to_string(),
                index: 265,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let attacking_pkmn = state.get_side_immutable(attacking_side).get_active_immutable();
                        if attacking_pkmn.status != PokemonStatus::None {
                            attacking_choice.base_power *= 1.5;

                            // not the right place to put this, but good enough
                            if attacking_pkmn.status == PokemonStatus::Burn {
                                attacking_choice.base_power *= 2.0;
                            }
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "shellarmor".to_string(),
                index: 266,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "rattled".to_string(),
                index: 267,
                modify_attack_against: Some(
                    |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                        if attacker_choice.move_type == PokemonType::Bug
                        || attacker_choice.move_type == PokemonType::Dark
                        || attacker_choice.move_type == PokemonType::Ghost {
                            attacker_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 100.0,
                                    target: MoveTarget::Opponent,
                                    effect: Effect::Boost(StatBoosts {
                                        attack: 0,
                                        defense: 0,
                                        special_attack: 0,
                                        special_defense: 0,
                                        speed: 1,
                                        accuracy: 0,
                                    }),
                                }
                            );
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "waterbubble".to_string(),
                index: 268,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "sandforce".to_string(),
                index: 269,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if state.weather_is_active(&Weather::Sand)
                            && (attacking_choice.move_type == PokemonType::Rock
                                || attacking_choice.move_type == PokemonType::Ground
                                || attacking_choice.move_type == PokemonType::Steel)
                        {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "toxicboost".to_string(),
                index: 270,
                modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        let active_pkmn = state.get_side_immutable(attacking_side).get_active_immutable();
                        if active_pkmn.status == PokemonStatus::Poison
                        || active_pkmn.status == PokemonStatus::Toxic {
                            attacking_choice.base_power *= 1.5;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "persistent".to_string(),
                index: 271,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "chlorophyll".to_string(),
                index: 272,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "simple".to_string(),
                index: 273,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "".to_string(),
                index: 274,
                ..Default::default()
            },
        );
        abilities.push(
            Ability {
                id: "purifyingsalt".to_string(),
                index: 275,
                ..Default::default()
            },
        );
        abilities
    };
}

#[non_exhaustive]
pub struct Abilities;

impl Abilities {
    pub const RIPEN: usize = 0;
    pub const TANGLEDFEET: usize = 1;
    pub const DRAGONSMAW: usize = 2;
    pub const CLEARBODY: usize = 3;
    pub const GALVANIZE: usize = 4;
    pub const VITALSPIRIT: usize = 5;
    pub const AERILATE: usize = 6;
    pub const DEFIANT: usize = 7;
    pub const CUTECHARM: usize = 8;
    pub const NEUROFORCE: usize = 9;
    pub const SOUNDPROOF: usize = 10;
    pub const RKSSYSTEM: usize = 11;
    pub const POISONPOINT: usize = 12;
    pub const STAKEOUT: usize = 13;
    pub const UNNERVE: usize = 14;
    pub const ROCKHEAD: usize = 15;
    pub const AURABREAK: usize = 16;
    pub const MIMICRY: usize = 17;
    pub const BULLETPROOF: usize = 18;
    pub const POWEROFALCHEMY: usize = 19;
    pub const TECHNICIAN: usize = 20;
    pub const MULTISCALE: usize = 21;
    pub const ARENATRAP: usize = 22;
    pub const BATTLEBOND: usize = 23;
    pub const DISGUISE: usize = 24;
    pub const EARLYBIRD: usize = 25;
    pub const LIGHTNINGROD: usize = 26;
    pub const MAGICIAN: usize = 27;
    pub const REFRIGERATE: usize = 28;
    pub const FRIENDGUARD: usize = 29;
    pub const NOABILITY: usize = 30;
    pub const GULPMISSILE: usize = 31;
    pub const POWERCONSTRUCT: usize = 32;
    pub const FORECAST: usize = 33;
    pub const PRANKSTER: usize = 34;
    pub const PROTEAN: usize = 35;
    pub const ASONEGLASTRIER: usize = 36;
    pub const SHADOWTAG: usize = 37;
    pub const SKILLLINK: usize = 38;
    pub const INTREPIDSWORD: usize = 39;
    pub const SOULHEART: usize = 40;
    pub const SWIFTSWIM: usize = 41;
    pub const EARTHEATER: usize = 42;
    pub const SUPERLUCK: usize = 43;
    pub const SUPREMEOVERLORD: usize = 44;
    pub const INSOMNIA: usize = 45;
    pub const DANCER: usize = 46;
    pub const STEAMENGINE: usize = 47;
    pub const ANGERPOINT: usize = 48;
    pub const CONTRARY: usize = 49;
    pub const MAGMAARMOR: usize = 50;
    pub const HUNGERSWITCH: usize = 51;
    pub const RECEIVER: usize = 52;
    pub const ZENMODE: usize = 53;
    pub const EMERGENCYEXIT: usize = 54;
    pub const ILLUSION: usize = 55;
    pub const WEAKARMOR: usize = 56;
    pub const DROUGHT: usize = 57;
    pub const INNARDSOUT: usize = 58;
    pub const SHIELDSDOWN: usize = 59;
    pub const ADAPTABILITY: usize = 60;
    pub const CORROSION: usize = 61;
    pub const LONGREACH: usize = 62;
    pub const PUREPOWER: usize = 63;
    pub const TINTEDLENS: usize = 64;
    pub const QUEENLYMAJESTY: usize = 65;
    pub const DESOLATELAND: usize = 66;
    pub const MOXIE: usize = 67;
    pub const SAPSIPPER: usize = 68;
    pub const SLUSHRUSH: usize = 69;
    pub const BIGPECKS: usize = 70;
    pub const STALL: usize = 71;
    pub const WHITESMOKE: usize = 72;
    pub const FLAREBOOST: usize = 73;
    pub const SHADOWSHIELD: usize = 74;
    pub const LIQUIDVOICE: usize = 75;
    pub const MISTYSURGE: usize = 76;
    pub const MULTITYPE: usize = 77;
    pub const NOGUARD: usize = 78;
    pub const TORRENT: usize = 79;
    pub const DELTASTREAM: usize = 80;
    pub const KLUTZ: usize = 81;
    pub const LIBERO: usize = 82;
    pub const SERENEGRACE: usize = 83;
    pub const CURSEDBODY: usize = 84;
    pub const UNAWARE: usize = 85;
    pub const LIGHTMETAL: usize = 86;
    pub const MARVELSCALE: usize = 87;
    pub const TELEPATHY: usize = 88;
    pub const QUICKDRAW: usize = 89;
    pub const HYPERCUTTER: usize = 90;
    pub const SYMBIOSIS: usize = 91;
    pub const PLUS: usize = 92;
    pub const MIRRORARMOR: usize = 93;
    pub const PASTELVEIL: usize = 94;
    pub const TOUGHCLAWS: usize = 95;
    pub const EFFECTSPORE: usize = 96;
    pub const MUMMY: usize = 97;
    pub const BADDREAMS: usize = 98;
    pub const MAGICGUARD: usize = 99;
    pub const SANDSTREAM: usize = 100;
    pub const POWERSPOT: usize = 101;
    pub const FLAMEBODY: usize = 102;
    pub const RECKLESS: usize = 103;
    pub const PRESSURE: usize = 104;
    pub const GOOEY: usize = 105;
    pub const IMMUNITY: usize = 106;
    pub const LEAFGUARD: usize = 107;
    pub const HUGEPOWER: usize = 108;
    pub const SOLARPOWER: usize = 109;
    pub const SCHOOLING: usize = 110;
    pub const MOTORDRIVE: usize = 111;
    pub const ANTICIPATION: usize = 112;
    pub const MERCILESS: usize = 113;
    pub const TRACE: usize = 114;
    pub const NATURALCURE: usize = 115;
    pub const HARVEST: usize = 116;
    pub const SUCTIONCUPS: usize = 117;
    pub const ICEFACE: usize = 118;
    pub const ROUGHSKIN: usize = 119;
    pub const WONDERGUARD: usize = 120;
    pub const WATERVEIL: usize = 121;
    pub const FAIRYAURA: usize = 122;
    pub const SANDSPIT: usize = 123;
    pub const INTIMIDATE: usize = 124;
    pub const DAUNTLESSSHIELD: usize = 125;
    pub const AROMAVEIL: usize = 126;
    pub const AIRLOCK: usize = 127;
    pub const NORMALIZE: usize = 128;
    pub const DARKAURA: usize = 129;
    pub const VICTORYSTAR: usize = 130;
    pub const GRASSYSURGE: usize = 131;
    pub const STURDY: usize = 132;
    pub const PICKPOCKET: usize = 133;
    pub const ELECTRICSURGE: usize = 134;
    pub const RUNAWAY: usize = 135;
    pub const OBLIVIOUS: usize = 136;
    pub const SURGESURFER: usize = 137;
    pub const LEVITATE: usize = 138;
    pub const ASONESPECTRIER: usize = 139;
    pub const PICKUP: usize = 140;
    pub const ICEBODY: usize = 141;
    pub const CURIOUSMEDICINE: usize = 142;
    pub const FLOWERVEIL: usize = 143;
    pub const STATIC: usize = 144;
    pub const WONDERSKIN: usize = 145;
    pub const OVERGROW: usize = 146;
    pub const PROPELLERTAIL: usize = 147;
    pub const THICKFAT: usize = 148;
    pub const GLUTTONY: usize = 149;
    pub const KEENEYE: usize = 150;
    pub const MOUNTAINEER: usize = 151;
    pub const FLASHFIRE: usize = 152;
    pub const COMPOUNDEYES: usize = 153;
    pub const STEELWORKER: usize = 154;
    pub const COMATOSE: usize = 155;
    pub const BALLFETCH: usize = 156;
    pub const DAZZLING: usize = 157;
    pub const DOWNLOAD: usize = 158;
    pub const TRANSISTOR: usize = 159;
    pub const MOLDBREAKER: usize = 160;
    pub const LIQUIDOOZE: usize = 161;
    pub const POISONHEAL: usize = 162;
    pub const PRISMARMOR: usize = 163;
    pub const SNIPER: usize = 164;
    pub const STENCH: usize = 165;
    pub const COMPETITIVE: usize = 166;
    pub const SWARM: usize = 167;
    pub const STALWART: usize = 168;
    pub const ILLUMINATE: usize = 169;
    pub const TURBOBLAZE: usize = 170;
    pub const GORILLATACTICS: usize = 171;
    pub const SPEEDBOOST: usize = 172;
    pub const HEATPROOF: usize = 173;
    pub const SNOWCLOAK: usize = 174;
    pub const TERAVOLT: usize = 175;
    pub const CHILLINGNEIGH: usize = 176;
    pub const SHIELDDUST: usize = 177;
    pub const RIVALRY: usize = 178;
    pub const PRIMORDIALSEA: usize = 179;
    pub const SCREENCLEANER: usize = 180;
    pub const MAGNETPULL: usize = 181;
    pub const HONEYGATHER: usize = 182;
    pub const COTTONDOWN: usize = 183;
    pub const GRASSPELT: usize = 184;
    pub const BATTLEARMOR: usize = 185;
    pub const BEASTBOOST: usize = 186;
    pub const BERSERK: usize = 187;
    pub const MINUS: usize = 188;
    pub const RAINDISH: usize = 189;
    pub const SYNCHRONIZE: usize = 190;
    pub const FILTER: usize = 191;
    pub const TRUANT: usize = 192;
    pub const FURCOAT: usize = 193;
    pub const FULLMETALBODY: usize = 194;
    pub const REGENERATOR: usize = 195;
    pub const FOREWARN: usize = 196;
    pub const IRONBARBS: usize = 197;
    pub const STAMINA: usize = 198;
    pub const SANDRUSH: usize = 199;
    pub const COLORCHANGE: usize = 200;
    pub const BLAZE: usize = 201;
    pub const ANALYTIC: usize = 202;
    pub const TANGLINGHAIR: usize = 203;
    pub const CLOUDNINE: usize = 204;
    pub const STEELYSPIRIT: usize = 205;
    pub const QUICKFEET: usize = 206;
    pub const MAGICBOUNCE: usize = 207;
    pub const MEGALAUNCHER: usize = 208;
    pub const HEAVYMETAL: usize = 209;
    pub const STORMDRAIN: usize = 210;
    pub const PIXILATE: usize = 211;
    pub const WATERCOMPACTION: usize = 212;
    pub const JUSTIFIED: usize = 213;
    pub const SLOWSTART: usize = 214;
    pub const SNOWWARNING: usize = 215;
    pub const FLOWERGIFT: usize = 216;
    pub const SHEDSKIN: usize = 217;
    pub const WIMPOUT: usize = 218;
    pub const ICESCALES: usize = 219;
    pub const INFILTRATOR: usize = 220;
    pub const LIMBER: usize = 221;
    pub const PSYCHICSURGE: usize = 222;
    pub const DEFEATIST: usize = 223;
    pub const WATERABSORB: usize = 224;
    pub const IMPOSTER: usize = 225;
    pub const DRYSKIN: usize = 226;
    pub const FLUFFY: usize = 227;
    pub const UNBURDEN: usize = 228;
    pub const CHEEKPOUCH: usize = 229;
    pub const STANCECHANGE: usize = 230;
    pub const MOODY: usize = 231;
    pub const ROCKYPAYLOAD: usize = 232;
    pub const PUNKROCK: usize = 233;
    pub const SANDVEIL: usize = 234;
    pub const PARENTALBOND: usize = 235;
    pub const STRONGJAW: usize = 236;
    pub const BATTERY: usize = 237;
    pub const HEALER: usize = 238;
    pub const STEADFAST: usize = 239;
    pub const DAMP: usize = 240;
    pub const PERISHBODY: usize = 241;
    pub const TRIAGE: usize = 242;
    pub const SHEERFORCE: usize = 243;
    pub const OWNTEMPO: usize = 244;
    pub const FRISK: usize = 245;
    pub const VOLTABSORB: usize = 246;
    pub const GALEWINGS: usize = 247;
    pub const AFTERMATH: usize = 248;
    pub const STICKYHOLD: usize = 249;
    pub const GRIMNEIGH: usize = 250;
    pub const IRONFIST: usize = 251;
    pub const REBOUND: usize = 252;
    pub const UNSEENFIST: usize = 253;
    pub const SOLIDROCK: usize = 254;
    pub const HUSTLE: usize = 255;
    pub const HYDRATION: usize = 256;
    pub const SCRAPPY: usize = 257;
    pub const OVERCOAT: usize = 258;
    pub const NEUTRALIZINGGAS: usize = 259;
    pub const SWEETVEIL: usize = 260;
    pub const DRIZZLE: usize = 261;
    pub const INNERFOCUS: usize = 262;
    pub const POISONTOUCH: usize = 263;
    pub const WANDERINGSPIRIT: usize = 264;
    pub const GUTS: usize = 265;
    pub const SHELLARMOR: usize = 266;
    pub const RATTLED: usize = 267;
    pub const WATERBUBBLE: usize = 268;
    pub const SANDFORCE: usize = 269;
    pub const TOXICBOOST: usize = 270;
    pub const PERSISTENT: usize = 271;
    pub const CHLOROPHYLL: usize = 272;
    pub const SIMPLE: usize = 273;
    pub const NONE: usize = 274;
    pub const PURIFYINGSALT: usize = 275;
}

pub struct Ability {
    pub id: String,
    pub index: usize,
    pub modify_attack_being_used: Option<ModifyAttackBeingUsed>,
    pub modify_attack_against: Option<ModifyAttackAgainst>,
    pub before_move: Option<AbilityBeforeMove>,
    pub after_damage_hit: Option<AbilityAfterDamageHit>,
    pub on_switch_out: Option<AbilityOnSwitchOut>,
    pub on_switch_in: Option<AbilityOnSwitchIn>,
    pub end_of_turn: Option<AbilityEndOfTurn>,
}

impl Default for Ability {
    fn default() -> Ability {
        return Ability {
            id: "".to_string(),
            index: 0,
            modify_attack_being_used: None,
            modify_attack_against: None,
            before_move: None,
            after_damage_hit: None,
            on_switch_out: None,
            on_switch_in: None,
            end_of_turn: None,
        };
    }
}
