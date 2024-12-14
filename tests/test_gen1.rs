#![cfg(feature = "gen1")]

use poke_engine::choices::Choices;
use poke_engine::choices::MoveCategory;
use poke_engine::generate_instructions::{generate_instructions_from_move_pair, MAX_SLEEP_TURNS};
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, ChangeStatusInstruction, DamageInstruction,
    DecrementRestTurnsInstruction, HealInstruction, Instruction, RemoveVolatileStatusInstruction,
    SetDamageDealtSideOneInstruction, SetDamageDealtSideTwoInstruction, SetSleepTurnsInstruction,
    StateInstructions, SwitchInstruction,
};
use poke_engine::pokemon::PokemonName;
use poke_engine::state::{
    MoveChoice, PokemonIndex, PokemonMoveIndex, PokemonStatus, PokemonType, PokemonVolatileStatus,
    SideReference, State,
};

pub fn generate_instructions_with_state_assertion(
    state: &mut State,
    side_one_move: &MoveChoice,
    side_two_move: &MoveChoice,
) -> Vec<StateInstructions> {
    let before_state_string = format!("{:?}", state);
    let instructions =
        generate_instructions_from_move_pair(state, side_one_move, side_two_move, false);
    let after_state_string = format!("{:?}", state);
    assert_eq!(before_state_string, after_state_string);
    instructions
}

fn set_moves_on_pkmn_and_call_generate_instructions(
    state: &mut State,
    move_one: Choices,
    move_two: Choices,
) -> Vec<StateInstructions> {
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_two);

    let instructions = generate_instructions_with_state_assertion(
        state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );
    instructions
}

#[test]
fn test_reflect_halves_physical_damage_as_volatile() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::REFLECT);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 24,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_reflect_and_lightscreen_set_volatiles_instead_of_sideconditions() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::REFLECT,
        Choices::LIGHTSCREEN,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::LIGHTSCREEN,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::REFLECT,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lightscreen_halves_special_damage_as_volatile() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::LIGHTSCREEN);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 15,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_thunderwave_into_substitute() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::SUBSTITUTE);
    state.side_two.substitute_health = 20;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::THUNDERWAVE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_confuseray_into_substitute() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::SUBSTITUTE);
    state.side_two.substitute_health = 20;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::CONFUSERAY,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_counter_into_normal_move() {
    let mut state = State::default();
    state.use_damage_dealt = true;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::COUNTER,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::SetDamageDealtSideOne(SetDamageDealtSideOneInstruction {
                damage_change: 48,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                toggle_hit_substitute: false,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 96,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_counter_into_fighting_move() {
    let mut state = State::default();
    state.use_damage_dealt = true;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::KARATECHOP,
        Choices::COUNTER,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 61,
            }),
            Instruction::SetDamageDealtSideOne(SetDamageDealtSideOneInstruction {
                damage_change: 61,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                toggle_hit_substitute: false,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 100,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_counter_into_flying_move_does_not_set_damage_dealt() {
    let mut state = State::default();
    state.use_damage_dealt = true;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DRILLPECK,
        Choices::COUNTER,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 63,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_toxic_turns_into_poison_when_switching() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::TOXIC;

    let vec_of_instructions = generate_instructions_with_state_assertion(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::TOXIC,
                new_status: PokemonStatus::POISON,
            }),
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_gen1_bite_flinch_with_counter() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BITE,
        Choices::COUNTER,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 72,
                }),
                Instruction::SetDamageDealtSideOne(SetDamageDealtSideOneInstruction {
                    damage_change: 72,
                    move_category: MoveCategory::Physical,
                    previous_move_category: MoveCategory::Physical,
                    toggle_hit_substitute: false,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 100,
                }),
            ],
        },
        StateInstructions {
            percentage: 10.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 72,
                }),
                Instruction::SetDamageDealtSideOne(SetDamageDealtSideOneInstruction {
                    damage_change: 72,
                    move_category: MoveCategory::Physical,
                    previous_move_category: MoveCategory::Physical,
                    toggle_hit_substitute: false,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::FLINCH,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::FLINCH,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_special_attack_boosts_defense() {
    let mut state = State::default();

    let un_boosted_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    // side 2 getting a SPA boost should cause side 1 to deal less damage
    state.side_two.special_attack_boost = 1;

    let boosted_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    assert_ne!(un_boosted_instructions, boosted_instructions);
}

#[test]
fn test_cannot_use_move_when_waking_from_sleep() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::SLEEP;
    state.side_one.get_active().sleep_turns = MAX_SLEEP_TURNS; // guaranteed to wake up

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::SLEEP,
                new_status: PokemonStatus::NONE,
            }),
            Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 0,
                previous_turns: 7,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_cannot_use_move_after_waking_when_only_a_chance_to_wake_up() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::SLEEP;
    state.side_one.get_active().sleep_turns = 5;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 66.666664,
            instruction_list: vec![Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 6,
                previous_turns: 5,
            })],
        },
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: 5,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rest_wake_up_cannot_use_move() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::SLEEP;
    state.side_one.get_active().rest_turns = 1; // guaranteed to wake up

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::SLEEP,
                new_status: PokemonStatus::NONE,
            }),
            Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                side_ref: SideReference::SideOne,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_rest_sets_rest_turns_to_2() {
    let mut state = State::default();
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::REST,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::SLEEP,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 2,
                previous_turns: 0,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 99,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_persion_using_slash_guaranteed_crit() {
    let mut state = State::default();
    state.side_one.get_active().id = PokemonName::PERSIAN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 100,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_persion_using_tackle_rolls_crit() {
    let mut state = State::default();
    state.side_one.get_active().id = PokemonName::PERSIAN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 77.64706,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 22.352942,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 94,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_reflect() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::REFLECT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.00,
        instruction_list: vec![Instruction::ApplyVolatileStatus(
            ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::REFLECT,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_freeze_clause() {
    let mut state = State::default();
    state.side_two.pokemon[PokemonIndex::P1].status = PokemonStatus::FREEZE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ICEBEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 74,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_counter_hits_ghost_type() {
    let mut state = State::default();
    state.use_damage_dealt = true;
    state.side_two.get_active().types.0 = PokemonType::GHOST;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::COUNTER);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        false,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 32,
            }),
            Instruction::SetDamageDealtSideTwo(SetDamageDealtSideTwoInstruction {
                damage_change: 32,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                toggle_hit_substitute: false,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 64,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_crit_roll_ignores_reflect() {
    let mut state = State::default();
    state.side_one.get_active().id = PokemonName::PERSIAN;
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::REFLECT);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 77.64706,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 24,
            })],
        },
        StateInstructions {
            percentage: 22.352942,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 94,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_crit_roll_ignores_own_boost() {
    let mut state = State::default();
    state.side_one.get_active().id = PokemonName::PERSIAN;
    state.side_one.attack_boost = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 77.64706,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 72,
            })],
        },
        StateInstructions {
            percentage: 22.352942,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 94,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_crit_roll_ignores_other_boost() {
    let mut state = State::default();
    state.side_one.get_active().id = PokemonName::PERSIAN;
    state.side_two.defense_boost = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 77.64706,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 33,
            })],
        },
        StateInstructions {
            percentage: 22.352942,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 94,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_crit_roll_ignores_other_boost_negative_boost() {
    let mut state = State::default();
    state.side_one.get_active().id = PokemonName::PERSIAN;
    state.side_two.defense_boost = -1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 77.64706,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 72,
            })],
        },
        StateInstructions {
            percentage: 22.352942,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 94,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_hyperbeam_sets_mustrecharge() {
    let mut state = State::default();
    state.side_two.get_active().hp = 500;
    state.side_two.get_active().maxhp = 500;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HYPERBEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 177,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::MUSTRECHARGE,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_hyperbeam_does_not_set_mustrecharge_on_ko() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HYPERBEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 100,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_none_with_mustrecharge_removes_volatile() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::MUSTRECHARGE);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::NONE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::RemoveVolatileStatus(
            RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::MUSTRECHARGE,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_mustrecharge_move_only_allows_none() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::MUSTRECHARGE);

    let options = state.get_all_options();

    let expected_options = (
        vec![MoveChoice::None],
        vec![
            MoveChoice::Move(PokemonMoveIndex::M0),
            MoveChoice::Move(PokemonMoveIndex::M1),
            MoveChoice::Move(PokemonMoveIndex::M2),
            MoveChoice::Move(PokemonMoveIndex::M3),
            MoveChoice::Switch(PokemonIndex::P1),
            MoveChoice::Switch(PokemonIndex::P2),
            MoveChoice::Switch(PokemonIndex::P3),
            MoveChoice::Switch(PokemonIndex::P4),
            MoveChoice::Switch(PokemonIndex::P5),
        ],
    );
    assert_eq!(expected_options, options);
}
