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
type ItemOnSwitchInFn = fn(&mut State, &SideReference, &mut StateInstructions);
type ItemEndOfTurn = fn(&mut State, &SideReference, &mut StateInstructions);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Items {
    NONE,
    AbsorbBulb,
    AirBalloon,
    AssaultVest,
    BlackBelt,
    BlackSludge,
    BlackGlasses,
    CellBattery,
    CHARCOAL,
    ChoiceBand,
    ChoiceSpecs,
    ChoiceScarf,
    DragonFang,
    DreadPlate,
    ElectricSeed,
    ExpertBelt,
    EVIOLITE,
    FairyFeather,
    FlameOrb,
    GrassySeed,
    LEFTOVERS,
    LifeOrb,
    MetalCoat,
    MistySeed,
    MuscleBand,
    MysticWater,
    NeverMeltIce,
    OddIncense,
    PoisonBarb,
    PsychicSeed,
    PunchingGlove,
    RockyHelmet,
    SeaIncense,
    SharpBeak,
    ShellBell,
    SilkScarf,
    SilverPowder,
    SoftSand,
    ThroatSpray,
    ToxicOrb,
    TwistedSpoon,
    WaveIncense,
    WeaknessPolicy,
    WiseGlasses,
    BlunderPolicy,
    HeavyDutyBoots,
    ClearAmulet,
    ProtectivePads,
}

#[non_exhaustive]
pub struct ItemsStruct {
    none: Item,
    absorb_bulb: Item,
    air_balloon: Item,
    assault_vest: Item,
    black_belt: Item,
    black_sludge: Item,
    black_glasses: Item,
    cell_battery: Item,
    charcoal: Item,
    choice_band: Item,
    choice_specs: Item,
    choice_scarf: Item,
    dragon_fang: Item,
    dread_plate: Item,
    electric_seed: Item,
    expert_belt: Item,
    eviolite: Item,
    fairy_feather: Item,
    flame_orb: Item,
    grassy_seed: Item,
    leftovers: Item,
    life_orb: Item,
    metal_coat: Item,
    misty_seed: Item,
    muscle_band: Item,
    mystic_water: Item,
    never_melt_ice: Item,
    odd_incense: Item,
    poison_barb: Item,
    psychic_seed: Item,
    punching_glove: Item,
    rocky_helmet: Item,
    sea_incense: Item,
    sharp_beak: Item,
    shell_bell: Item,
    silk_scarf: Item,
    silver_powder: Item,
    soft_sand: Item,
    throat_spray: Item,
    toxic_orb: Item,
    twisted_spoon: Item,
    wave_incense: Item,
    weakness_policy: Item,
    wise_glasses: Item,
    blunder_policy: Item,
    heavy_duty_boots: Item,
    clear_amulet: Item,
    protective_pads: Item,
}

lazy_static! {
    static ref ALL_ITEMS: ItemsStruct = ItemsStruct {
        none: Item {
            ..Default::default()
        },
        absorb_bulb: Item {
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
        air_balloon: Item {
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
        assault_vest: Item {
            modify_attack_against: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.category == MoveCategory::Special {
                    attacking_choice.base_power /= 1.5;
                }
            }),
            ..Default::default()
        },
        black_belt: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Fighting {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        black_sludge: Item {
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
        },
        black_glasses: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Dark {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        cell_battery: Item {
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
        },
        charcoal: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Fire {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        choice_band: Item {
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
        },
        choice_specs: Item {
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
        },
        choice_scarf: Item {
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
        },
        dragon_fang: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Dragon {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        dread_plate: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Dark {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        electric_seed: Item {
            on_switch_in: Some(
                |state: &mut State,
                 side_ref: &SideReference,
                 instructions: &mut StateInstructions| {
                    if state.terrain_is_active(&Terrain::ElectricTerrain) {
                        if let Some(boost_instruction) = get_boost_instruction(
                            state,
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
                                    current_item: Items::ElectricSeed,
                                    new_item: Items::NONE,
                                },
                            ));
                        }
                    }
                }
            ),
            ..Default::default()
        },
        expert_belt: Item {
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
        eviolite: Item {
            modify_attack_against: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                attacking_choice.base_power /= 1.5;
            }),
            ..Default::default()
        },
        fairy_feather: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Fairy {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        flame_orb: Item {
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
        },
        grassy_seed: Item {
            on_switch_in: Some(
                |state: &mut State,
                 side_ref: &SideReference,
                 instructions: &mut StateInstructions| {
                    if state.terrain_is_active(&Terrain::GrassyTerrain) {
                        if let Some(boost_instruction) = get_boost_instruction(
                            state,
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
                                    current_item: Items::GrassySeed,
                                    new_item: Items::NONE,
                                },
                            ));
                        }
                    }
                }
            ),
            ..Default::default()
        },
        leftovers: Item {
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
        },
        life_orb: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                attacking_choice.base_power *= 1.3;
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Heal(-0.1),
                    target: MoveTarget::User,
                });
            }),
            ..Default::default()
        },
        metal_coat: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Steel {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        misty_seed: Item {
            on_switch_in: Some(
                |state: &mut State,
                 side_ref: &SideReference,
                 instructions: &mut StateInstructions| {
                    if state.terrain_is_active(&Terrain::MistyTerrain) {
                        if let Some(boost_instruction) = get_boost_instruction(
                            state,
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
                                    current_item: Items::MistySeed,
                                    new_item: Items::NONE,
                                },
                            ));
                        }
                    }
                }
            ),
            ..Default::default()
        },
        muscle_band: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.category == MoveCategory::Physical {
                    attacking_choice.base_power *= 1.1;
                }
            }),
            ..Default::default()
        },
        mystic_water: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Water {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        never_melt_ice: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Ice {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        odd_incense: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Psychic {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        poison_barb: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Poison {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        psychic_seed: Item {
            on_switch_in: Some(
                |state: &mut State,
                 side_ref: &SideReference,
                 instructions: &mut StateInstructions| {
                    if state.terrain_is_active(&Terrain::PsychicTerrain) {
                        if let Some(boost_instruction) = get_boost_instruction(
                            state,
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
                                    current_item: Items::PsychicSeed,
                                    new_item: Items::NONE,
                                },
                            ));
                        }
                    }
                }
            ),
            ..Default::default()
        },
        punching_glove: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.flags.punch {
                    attacking_choice.base_power *= 1.1;
                    attacking_choice.flags.contact = false
                }
            }),
            ..Default::default()
        },
        rocky_helmet: Item {
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
        sea_incense: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Water {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        sharp_beak: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Flying {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        shell_bell: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                attacking_choice.drain = Some(0.125);
            }),
            ..Default::default()
        },
        silk_scarf: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Normal {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        silver_powder: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Bug {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        soft_sand: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Ground {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        throat_spray: Item {
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
        },
        toxic_orb: Item {
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
        },
        twisted_spoon: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Psychic {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        wave_incense: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.move_type == PokemonType::Water {
                    attacking_choice.base_power *= 1.2;
                }
            }),
            ..Default::default()
        },
        weakness_policy: Item {
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
        },
        wise_glasses: Item {
            modify_attack_being_used: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                if attacking_choice.category == MoveCategory::Special {
                    attacking_choice.base_power *= 1.1;
                }
            }),
            ..Default::default()
        },
        blunder_policy: Item {
            ..Default::default()
        },
        heavy_duty_boots: Item {
            ..Default::default()
        },
        clear_amulet: Item {
            ..Default::default()
        },
        protective_pads: Item {
            ..Default::default()
        },
    };
}

pub fn item_from_index<'a>(index: Items) -> &'a Item {
    match index {
        Items::NONE => &ALL_ITEMS.none,
        Items::AbsorbBulb => &ALL_ITEMS.absorb_bulb,
        Items::AirBalloon => &ALL_ITEMS.air_balloon,
        Items::AssaultVest => &ALL_ITEMS.assault_vest,
        Items::BlackBelt => &ALL_ITEMS.black_belt,
        Items::BlackSludge => &ALL_ITEMS.black_sludge,
        Items::BlackGlasses => &ALL_ITEMS.black_glasses,
        Items::CellBattery => &ALL_ITEMS.cell_battery,
        Items::CHARCOAL => &ALL_ITEMS.charcoal,
        Items::ChoiceBand => &ALL_ITEMS.choice_band,
        Items::ChoiceSpecs => &ALL_ITEMS.choice_specs,
        Items::ChoiceScarf => &ALL_ITEMS.choice_scarf,
        Items::DragonFang => &ALL_ITEMS.dragon_fang,
        Items::DreadPlate => &ALL_ITEMS.dread_plate,
        Items::ElectricSeed => &ALL_ITEMS.electric_seed,
        Items::ExpertBelt => &ALL_ITEMS.expert_belt,
        Items::EVIOLITE => &ALL_ITEMS.eviolite,
        Items::FairyFeather => &ALL_ITEMS.fairy_feather,
        Items::FlameOrb => &ALL_ITEMS.flame_orb,
        Items::GrassySeed => &ALL_ITEMS.grassy_seed,
        Items::LEFTOVERS => &ALL_ITEMS.leftovers,
        Items::LifeOrb => &ALL_ITEMS.life_orb,
        Items::MetalCoat => &ALL_ITEMS.metal_coat,
        Items::MistySeed => &ALL_ITEMS.misty_seed,
        Items::MuscleBand => &ALL_ITEMS.muscle_band,
        Items::MysticWater => &ALL_ITEMS.mystic_water,
        Items::NeverMeltIce => &ALL_ITEMS.never_melt_ice,
        Items::OddIncense => &ALL_ITEMS.odd_incense,
        Items::PoisonBarb => &ALL_ITEMS.poison_barb,
        Items::PsychicSeed => &ALL_ITEMS.psychic_seed,
        Items::PunchingGlove => &ALL_ITEMS.punching_glove,
        Items::RockyHelmet => &ALL_ITEMS.rocky_helmet,
        Items::SeaIncense => &ALL_ITEMS.sea_incense,
        Items::SharpBeak => &ALL_ITEMS.sharp_beak,
        Items::ShellBell => &ALL_ITEMS.shell_bell,
        Items::SilkScarf => &ALL_ITEMS.silk_scarf,
        Items::SilverPowder => &ALL_ITEMS.silver_powder,
        Items::SoftSand => &ALL_ITEMS.soft_sand,
        Items::ThroatSpray => &ALL_ITEMS.throat_spray,
        Items::ToxicOrb => &ALL_ITEMS.toxic_orb,
        Items::TwistedSpoon => &ALL_ITEMS.twisted_spoon,
        Items::WaveIncense => &ALL_ITEMS.wave_incense,
        Items::WeaknessPolicy => &ALL_ITEMS.weakness_policy,
        Items::WiseGlasses => &ALL_ITEMS.wise_glasses,
        Items::BlunderPolicy => &ALL_ITEMS.blunder_policy,
        Items::HeavyDutyBoots => &ALL_ITEMS.heavy_duty_boots,
        Items::ClearAmulet => &ALL_ITEMS.clear_amulet,
        Items::ProtectivePads => &ALL_ITEMS.protective_pads,
    }
}

fn get_choice_move_disable_instructions(
    pkmn: &Pokemon,
    side_ref: &SideReference,
    move_name: &String,
) -> Vec<Instruction> {
    let mut moves_to_disable = vec![];
    let mut iter = pkmn.moves.into_iter();
    while let Some(p) = iter.next() {
        if &p.id != move_name {
            moves_to_disable.push(Instruction::DisableMove(DisableMoveInstruction {
                side_ref: *side_ref,
                move_index: iter.pokemon_move_index,
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
