use poke_engine::abilities::Abilities;
use poke_engine::choices::{Choices, MOVES};
use poke_engine::generate_instructions::{generate_instructions_from_move_pair, MAX_SLEEP_TURNS};
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeSideConditionInstruction, ChangeStatusInstruction, ChangeTerrain, ChangeType,
    ChangeWeather, DamageInstruction, DecrementRestTurnsInstruction, DecrementWishInstruction,
    DisableMoveInstruction, EnableMoveInstruction, HealInstruction, Instruction,
    RemoveVolatileStatusInstruction, SetSecondMoveSwitchOutMoveInstruction,
    SetSleepTurnsInstruction, SetSubstituteHealthInstruction, SetWishInstruction,
    StateInstructions, SwitchInstruction, ToggleBatonPassingInstruction,
    ToggleTrickRoomInstruction,
};
use poke_engine::items::Items;
use poke_engine::state::{
    pokemon_index_iter, Move, MoveChoice, PokemonBoostableStat, PokemonIndex, PokemonMoveIndex,
    PokemonSideCondition, PokemonStatus, PokemonType, PokemonVolatileStatus, SideReference, State,
    StateWeather, Terrain, Weather,
};

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

    let before_state_string = format!("{:?}", state);
    let instructions = generate_instructions_from_move_pair(
        state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let after_state_string = format!("{:?}", state);
    assert_eq!(before_state_string, after_state_string);

    return instructions;
}

#[test]
fn test_force_switch_after_faint_does_not_trigger_end_of_turn() {
    let mut state = State::default();
    state.side_one.get_active().hp = 0;

    // Hail shouldn't do any damage
    state.weather.weather_type = Weather::Hail;
    state.weather.turns_remaining = 2;

    let side_one_move = MoveChoice::Switch(PokemonIndex::P1);
    let side_two_move = MoveChoice::None;
    let vec_of_instructions =
        generate_instructions_from_move_pair(&mut state, &side_one_move, &side_two_move);

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_basic_move_pair_instruction_generation() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_compound_eyes_does_not_cause_instructions_with_more_than_100_percent() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::COMPOUNDEYES;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_move_pair_instruction_generation_where_first_move_branches() {
    let mut state = State::default();
    state.side_one.get_active().speed = 101;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PLAYROUGH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 81.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 71,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
            ],
        },
        StateInstructions {
            percentage: 9.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 71,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Attack,
                    amount: -1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    // playrough lowered attack means this does less dmg than other branches
                    damage_amount: 33,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_move_pair_instruction_generation_where_second_move_branches() {
    let mut state = State::default();
    state.side_one.get_active().speed = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PLAYROUGH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 81.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 71,
                }),
            ],
        },
        StateInstructions {
            percentage: 9.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 71,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Attack,
                    amount: -1,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_basic_flinching_functionality() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::IRONHEAD,
        Choices::TACKLE,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 70.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 63,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
            ],
        },
        StateInstructions {
            percentage: 30.0000019,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 63,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Flinch,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Flinch,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_flinching_first_and_second_move() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::IRONHEAD,
        Choices::IRONHEAD,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 70.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 63,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 63,
                }),
            ],
        },
        StateInstructions {
            percentage: 30.0000019,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 63,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Flinch,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Flinch,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_flinching_on_move_that_can_miss() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::AIRSLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 5.000001,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 66.5,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 60,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
            ],
        },
        StateInstructions {
            percentage: 28.5000019,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 60,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Flinch,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Flinch,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_protect_against_damaging_move() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_self_boosting_move_against_protect() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::SWORDSDANCE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: 2,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_crash_move_into_protect() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::JUMPKICK,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 50,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_protect_stops_secondaries() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::IRONHEAD,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_protect_stops_after_damage_hit_callback() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LEFTOVERS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::KNOCKOFF,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[cfg(feature = "gen8")]
#[test]
fn test_knockoff_removing_item() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LEFTOVERS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::KNOCKOFF,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 76,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::LEFTOVERS,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[cfg(feature = "gen8")]
#[test]
fn test_knockoff_cannot_remove_arceus_plate() {
    let mut state = State::default();
    state.side_one.get_active().id = "arceusghost".to_string();
    state.side_one.get_active().item = Items::SPOOKYPLATE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::KNOCKOFF,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 51,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[cfg(feature = "gen8")]
#[test]
fn test_knockoff_boosts_damage_but_cannot_remove_if_sub_is_hit() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LEFTOVERS;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::KNOCKOFF,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::DamageSubstitute(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 76, // 51 is unboosted dmg
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_move_that_goes_through_protect() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::FEINT,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 37,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_spikyshield_against_contact_move() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPIKYSHIELD,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SpikyShield,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideTwo,
                heal_amount: -12,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SpikyShield,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_spikyshield_recoil_does_not_overkill() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPIKYSHIELD,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SpikyShield,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideTwo,
                heal_amount: -1,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SpikyShield,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_magician_doing_damage_steals_opponents_item() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::MAGICIAN;
    state.side_one.get_active().item = Items::NONE;
    state.side_two.get_active().item = Items::LEFTOVERS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::NONE,
                new_item: Items::LEFTOVERS,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::LEFTOVERS,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_magician_does_not_steal_if_move_misses() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::MAGICIAN;
    state.side_one.get_active().item = Items::NONE;
    state.side_two.get_active().item = Items::LEFTOVERS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::AIRSLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 5.000001,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 95.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 60,
                }),
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: SideReference::SideOne,
                    current_item: Items::NONE,
                    new_item: Items::LEFTOVERS,
                }),
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: SideReference::SideTwo,
                    current_item: Items::LEFTOVERS,
                    new_item: Items::NONE,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_magician_does_not_remove_from_stickyhold() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::MAGICIAN;
    state.side_two.get_active().ability = Abilities::STICKYHOLD;
    state.side_one.get_active().item = Items::NONE;
    state.side_two.get_active().item = Items::LEFTOVERS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideTwo,
                heal_amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_moxie_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_one.get_active().ability = Abilities::MOXIE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_chillingneigh_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_one.get_active().ability = Abilities::CHILLINGNEIGH;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_grimneigh_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_one.get_active().ability = Abilities::GRIMNEIGH;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_chillingneigh_does_not_overboost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_one.attack_boost = 6;
    state.side_one.get_active().ability = Abilities::CHILLINGNEIGH;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_grimneigh_does_not_overboost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_one.special_attack_boost = 6;
    state.side_one.get_active().ability = Abilities::GRIMNEIGH;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_berserk_when_going_below_half() {
    let mut state = State::default();
    state.side_one.get_active().hp = 51;
    state.side_one.get_active().ability = Abilities::BERSERK;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_berserk_when_staying_above_half() {
    let mut state = State::default();
    state.side_one.get_active().hp = 100;
    state.side_one.get_active().ability = Abilities::BERSERK;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_berserk_cannot_overboost() {
    let mut state = State::default();
    state.side_one.get_active().hp = 51;
    state.side_one.special_attack_boost = 6;
    state.side_one.get_active().ability = Abilities::BERSERK;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_aftermath_damage() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().ability = Abilities::AFTERMATH;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_innards_out() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().ability = Abilities::INNARDSOUT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_innards_out_does_not_overkill() {
    let mut state = State::default();
    state.side_two.get_active().hp = 10;
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().ability = Abilities::INNARDSOUT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 10,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_aftermath_cannot_overkill() {
    let mut state = State::default();
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().ability = Abilities::AFTERMATH;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_color_change_modifying_type() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::COLORCHANGE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::WATERGUN,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 32,
            }),
            Instruction::ChangeType(ChangeType {
                side_ref: SideReference::SideOne,
                new_types: (PokemonType::Water, PokemonType::Typeless),
                old_types: (PokemonType::Normal, PokemonType::Typeless),
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_color_change_does_not_activate_when_fainting() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::COLORCHANGE;
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::WATERGUN,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_color_change_does_not_activate_if_type_is_already_the_same() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::COLORCHANGE;
    state.side_one.get_active().types.0 = PokemonType::Grass;
    state.side_one.get_active().types.1 = PokemonType::Water;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::WATERGUN,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 7,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_cottondown() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::COTTONDOWN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Speed,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_cottondown_activates_when_fainting() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::COTTONDOWN;
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Speed,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_cottondown_cannot_boost_below_minus_6() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::COTTONDOWN;
    state.side_two.speed_boost = -6;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_stamina_activating() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::STAMINA;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::WATERGUN,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 32,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Defense,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_stamina_activating_on_multi_hit() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::STAMINA;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::DOUBLEHIT,
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
                    side_ref: SideReference::SideOne,
                    damage_amount: 42,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 42,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: 1,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_stamina_does_not_activate_when_fainting() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::STAMINA;
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::WATERGUN,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_spikyshield_does_not_activate_on_non_contact_move() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPIKYSHIELD,
        Choices::WATERGUN,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SpikyShield,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SpikyShield,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_banefulbunker_poisons() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BANEFULBUNKER,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::BanefulBunker,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Poison,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 12,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::BanefulBunker,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_banefulbunker_cannot_poison_already_statused_target() {
    let mut state = State::default();
    state.side_two.get_active().status = PokemonStatus::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BANEFULBUNKER,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::BanefulBunker,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 12,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::BanefulBunker,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_silktrap() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SILKTRAP,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SilkTrap,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Speed,
                amount: -1,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SilkTrap,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_protect_side_condition_is_removed() {
    let mut state = State::default();
    state.side_one.side_conditions.protect = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeSideCondition(
            ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: -1,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_protect_for_second_turn_in_a_row() {
    let mut state = State::default();
    state.side_one.side_conditions.protect = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_outrage_locking() {
    let mut state = State::default();
    state.side_one.get_active().moves[PokemonMoveIndex::M0] = Move {
        id: Choices::OUTRAGE,
        disabled: false,
        pp: 16,
        choice: MOVES.get(&Choices::OUTRAGE).unwrap().clone(),
    };

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::OUTRAGE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DisableMove(DisableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M1,
            }),
            Instruction::DisableMove(DisableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M2,
            }),
            Instruction::DisableMove(DisableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M3,
            }),
            Instruction::DisableMove(DisableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M4,
            }),
            Instruction::DisableMove(DisableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M5,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 94,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::LockedMove,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_judgement_typechange_with_arceus_multitype() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SPOOKYPLATE;
    state.side_two.get_active().types.0 = PokemonType::Normal;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::JUDGMENT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_haze_resets_both_side_boosts() {
    let mut state = State::default();
    state.side_one.attack_boost = 3;
    state.side_one.defense_boost = -3;
    state.side_two.special_attack_boost = 2;
    state.side_two.special_defense_boost = -2;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HAZE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: -3,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Defense,
                amount: 3,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: -2,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialDefense,
                amount: 2,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_clearsmog_removes_boosts_on_target() {
    let mut state = State::default();
    state.side_one.attack_boost = 3;
    state.side_one.defense_boost = -3;
    state.side_two.special_attack_boost = -2;
    state.side_two.special_defense_boost = 2;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::CLEARSMOG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 21,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 2,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialDefense,
                amount: -2,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_clearsmog_does_not_reset_boosts_if_defender_is_immune() {
    let mut state = State::default();
    state.side_two.get_active().types.0 = PokemonType::Steel;
    state.side_one.attack_boost = 3;
    state.side_one.defense_boost = -3;
    state.side_two.special_attack_boost = -2;
    state.side_two.special_defense_boost = 2;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::CLEARSMOG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_basic_healbell() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HEALBELL,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: SideReference::SideOne,
            pokemon_index: PokemonIndex::P0,
            old_status: PokemonStatus::Poison,
            new_status: PokemonStatus::None,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_healbell_with_multiple_reserves_statused() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Poison;
    state.side_one.pokemon[PokemonIndex::P1].status = PokemonStatus::Burn;
    state.side_one.pokemon[PokemonIndex::P3].status = PokemonStatus::Sleep;
    state.side_one.pokemon[PokemonIndex::P5].status = PokemonStatus::Toxic;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HEALBELL,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Poison,
                new_status: PokemonStatus::None,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P1,
                old_status: PokemonStatus::Burn,
                new_status: PokemonStatus::None,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P3,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P5,
                old_status: PokemonStatus::Toxic,
                new_status: PokemonStatus::None,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_healbell_when_one_reserve_was_rested() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Poison;
    state.side_one.pokemon[PokemonIndex::P1].status = PokemonStatus::Burn;
    state.side_one.pokemon[PokemonIndex::P3].status = PokemonStatus::Sleep;
    state.side_one.pokemon[PokemonIndex::P3].rest_turns = 3;
    state.side_one.pokemon[PokemonIndex::P5].status = PokemonStatus::Toxic;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HEALBELL,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Poison,
                new_status: PokemonStatus::None,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P1,
                old_status: PokemonStatus::Burn,
                new_status: PokemonStatus::None,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P3,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P3,
                new_turns: 0,
                previous_turns: 3,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P5,
                old_status: PokemonStatus::Toxic,
                new_status: PokemonStatus::None,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_double_protect() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::PROTECT,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_basic_substitute_usage() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUBSTITUTE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
            Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                side_ref: SideReference::SideOne,
                new_health: 25,
                old_health: 0,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_destinybond_kills_on_knockout() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DESTINYBOND,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::DestinyBond,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 100,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_destinybond_against_toxic_damage_does_not_kill_opponent() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_one.get_active().hp = 1;

    state.side_two.get_active().types.0 = PokemonType::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DESTINYBOND,
        Choices::TOXIC,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::DestinyBond,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Toxic,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::ToxicCount,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_destinybond_volatile_is_removed_at_end_of_turn_if_not_fainted() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_one.get_active().hp = 1;

    state.side_two.get_active().types.0 = PokemonType::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DESTINYBOND,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::DestinyBond,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::DestinyBond,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_substitute_when_it_is_already_up() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUBSTITUTE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taking_damage_with_0_hp_sub_but_with_vs() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 0,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_substitute_does_not_let_secondary_status_effect_happen() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUBSTITUTE,
        Choices::FIREPUNCH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
            Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                side_ref: SideReference::SideOne,
                new_health: 25,
                old_health: 0,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_side_one_using_unboosting_move_versus_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 50;
    state.side_two.get_active().speed = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PSYCHIC,
        Choices::SUBSTITUTE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 25,
            }),
            Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                side_ref: SideReference::SideTwo,
                new_health: 25,
                old_health: 0,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 25,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_vcreate_unboosts_only_on_hit() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::VCREATE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 5.000001,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 95.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::SpecialDefense,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: -1,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_side_one_self_unboost_versus_sub() {
    let mut state = State::default();
    state.side_one.get_active().speed = 50;
    state.side_two.get_active().speed = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::LEAFSTORM,
        Choices::SUBSTITUTE,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 25,
                }),
                Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                    side_ref: SideReference::SideTwo,
                    new_health: 25,
                    old_health: 0,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
            ],
        },
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 25,
                }),
                Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                    side_ref: SideReference::SideTwo,
                    new_health: 25,
                    old_health: 0,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                Instruction::DamageSubstitute(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 25,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::SpecialAttack,
                    amount: -2,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_secondary_on_self_works_against_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUBSTITUTE,
        Choices::POWERUPPUNCH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
            Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                side_ref: SideReference::SideOne,
                new_health: 25,
                old_health: 0,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_move_goes_through_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUBSTITUTE,
        Choices::BOOMBURST,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
            Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                side_ref: SideReference::SideOne,
                new_health: 25,
                old_health: 0,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 75,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_infiltrator_goes_through_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().ability = Abilities::INFILTRATOR;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUBSTITUTE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            }),
            Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                side_ref: SideReference::SideOne,
                new_health: 25,
                old_health: 0,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_protect_with_a_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 25;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_drag_move_against_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 25;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::DRAGONTAIL,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![
                Instruction::DamageSubstitute(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 25,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_suctioncups() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::SUCTIONCUPS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DRAGONTAIL,
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
                damage_amount: 48,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_tanglinghair() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::TANGLINGHAIR;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Speed,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_whirlwind_move_against_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 25;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::WHIRLWIND,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 20.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
        },
        StateInstructions {
            percentage: 20.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P2,
                }),
            ],
        },
        StateInstructions {
            percentage: 20.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P3,
                }),
            ],
        },
        StateInstructions {
            percentage: 20.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P4,
                }),
            ],
        },
        StateInstructions {
            percentage: 20.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P5,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_whirlwind_against_guarddog() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_one.get_active().ability = Abilities::GUARDDOG;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::WHIRLWIND,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_drag_move_against_protect_and_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 25;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::DRAGONTAIL,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rockyhelmet_damage_taken() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().item = Items::ROCKYHELMET;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: -16,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_roughskin_damage_taken() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::ROUGHSKIN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_roughskin_damage_taken_when_target_faints() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::ROUGHSKIN;
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_roughskin_damage_taken_on_multihit_move() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::ROUGHSKIN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DOUBLEHIT,
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
                    damage_amount: 42,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 42,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(not(feature = "gen5"))]
fn test_flyinggem_and_acrobatics_together() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::FLYINGGEM;
    state.side_two.get_active().hp = 400;
    state.side_two.get_active().maxhp = 400;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ACROBATICS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::FLYINGGEM,
                new_item: Items::NONE,
            }),
            // 44 damage normally
            // 2x for acrobatics without item
            // 1.3x for gem usage
            // ~= 112
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 112,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen5")]
fn test_flyinggem_and_acrobatics_together_gen5() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::FLYINGGEM;
    state.side_two.get_active().hp = 400;
    state.side_two.get_active().maxhp = 400;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ACROBATICS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::FLYINGGEM,
                new_item: Items::NONE,
            }),
            // 44 damage normally
            // 2x for acrobatics without item
            // 1.5x for gem usage
            // ~= 129
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 129,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(not(feature = "gen5"))]
fn test_normalgem_boosting_tackle() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::NORMALGEM;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::NORMALGEM,
                new_item: Items::NONE,
            }),
            // 48 damage normally. 1.3x for gem
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 61,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_chopleberry_damage_reduction() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::CHOPLEBERRY;
    state.side_two.get_active().types = (PokemonType::Normal, PokemonType::Typeless);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POWERUPPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::CHOPLEBERRY,
                new_item: Items::NONE,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 33, // 64 damage normally
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_sitrus_berry_activate_after_taking_damage_when_slower() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SITRUSBERRY;
    state.side_one.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 25,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::SITRUSBERRY,
                new_item: Items::NONE,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_healing_move_after_sitrusberry() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SITRUSBERRY;
    state.side_one.get_active().hp = 95;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::RECOVER,
        Choices::TACKLE,
    );

    /*
        95 starting hp
        -48 from tackle = 47
        +25 from sitrus = 72
        hp remaining for recover to heal is 28
    */
    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 25,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::SITRUSBERRY,
                new_item: Items::NONE,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 28,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_sitrus_berry_activate_after_taking_damage_when_faster() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SITRUSBERRY;
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 25,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::SITRUSBERRY,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_sitrus_berry_does_not_activate_if_above_half() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SITRUSBERRY;
    state.side_one.get_active().hp = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lumberry_curing_before_move() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LUMBERRY;
    state.side_one.get_active().status = PokemonStatus::Burn;

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
                old_status: PokemonStatus::Burn,
                new_status: PokemonStatus::None,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::LUMBERRY,
                new_item: Items::NONE,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lumberry_does_nothing_with_no_status() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LUMBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lum_cures_same_turn_when_slower() {
    let mut state = State::default();
    state.side_one.get_active().speed = 100;
    state.side_two.get_active().speed = 150;
    state.side_one.get_active().item = Items::LUMBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPORE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Sleep,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::LUMBERRY,
                new_item: Items::NONE,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lum_cures_same_turn_when_faster() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().speed = 100;
    state.side_one.get_active().item = Items::LUMBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPORE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Sleep,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::LUMBERRY,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_chopleberry_damage_reduction_does_not_happen_on_water_move() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::CHOPLEBERRY;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 64,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_chopleberry_damage_reduction_does_not_happen_if_not_supereffective() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::CHOPLEBERRY;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Typeless);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POWERUPPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 32,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rockyhelmet_does_not_overkill() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().item = Items::ROCKYHELMET;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_choiceband_locking() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::CHOICEBAND;
    state.side_one.get_active().moves[PokemonMoveIndex::M0] = Move {
        id: Choices::WILLOWISP,
        disabled: false,
        pp: 35,
        ..Default::default()
    };

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WILLOWISP,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 14.999998,
            instruction_list: vec![
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M1,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M2,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M3,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M4,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M5,
                }),
            ],
        },
        StateInstructions {
            percentage: 85.0,
            instruction_list: vec![
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M1,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M2,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M3,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M4,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M5,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Burn,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_gorillatactics_locking() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::GORILLATACTICS;
    state.side_one.get_active().moves[PokemonMoveIndex::M0] = Move {
        id: Choices::ZAPCANNON,
        disabled: false,
        pp: 35,
        ..Default::default()
    };

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ZAPCANNON,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 50.0,
            instruction_list: vec![
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M1,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M2,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M3,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M4,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M5,
                }),
            ],
        },
        StateInstructions {
            percentage: 50.0,
            instruction_list: vec![
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M1,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M2,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M3,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M4,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M5,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 94,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Paralyze,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_earthquake_hits_roosted_flying_type() {
    let mut state = State::default();
    state.side_two.get_active().types = (PokemonType::Flying, PokemonType::Normal);
    state.side_two.get_active().speed = 101;
    state.side_one.get_active().speed = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::EARTHQUAKE,
        Choices::ROOST,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Roost,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 79,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Roost,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_locked_moves_unlock_on_switchout() {
    let mut state = State::default();
    state.side_one.get_active().moves[PokemonMoveIndex::M1].disabled = true;
    state.side_one.get_active().moves[PokemonMoveIndex::M2].disabled = true;
    state.side_one.get_active().moves[PokemonMoveIndex::M3].disabled = true;

    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::EnableMove(EnableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M1,
            }),
            Instruction::EnableMove(EnableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M2,
            }),
            Instruction::EnableMove(EnableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M3,
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
#[cfg(feature = "gen9")]
fn test_fighting_move_with_blackbelt() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_one.get_active().item = Items::BLACKBELT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DRAINPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 142,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_expert_belt_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_one.get_active().item = Items::EXPERTBELT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DRAINPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 142,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_hydrosteam() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.weather.weather_type = Weather::Sun;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HYDROSTEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 93,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_weatherball_in_sun() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sun;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WEATHERBALL,
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
#[cfg(any(feature = "gen9"))]
fn test_terrainpulse_gen9() {
    let mut state = State::default();
    state.terrain.terrain_type = Terrain::ElectricTerrain;
    state.side_two.get_active().maxhp = 300;
    state.side_two.get_active().hp = 300;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TERRAINPULSE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 102,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(feature = "gen7"))]
fn test_terrainpulse_gen7() {
    let mut state = State::default();
    state.terrain.terrain_type = Terrain::ElectricTerrain;
    state.side_two.get_active().maxhp = 300;
    state.side_two.get_active().hp = 300;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TERRAINPULSE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 119,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_growth_in_sun() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sun;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::GROWTH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 2,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 2,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_batonpass_with_boosts() {
    let mut state = State::default();
    state.side_one.attack_boost = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BATONPASS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleBatonPassing(ToggleBatonPassingInstruction {
                side_ref: SideReference::SideOne,
            }),
            Instruction::ToggleSideOneForceSwitch,
            Instruction::SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction {
                new_choice: Choices::NONE,
                previous_choice: Choices::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen4")]
fn test_simple_in_gen4_doubles_effective_boost() {
    let mut state = State::default();
    state.side_one.attack_boost = 1; // should behave as +2 in gen4

    let regular_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    state.side_one.get_active().ability = Abilities::SIMPLE;

    let simple_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    assert_ne!(regular_instructions, simple_instructions);

    let expected_regular_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 72,
        })],
    }];
    let expected_simple_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 95,
        })],
    }];

    assert_eq!(expected_regular_instructions, regular_instructions);
    assert_eq!(expected_simple_instructions, simple_instructions);
}

#[test]
fn test_switching_from_batonpass_with_boosts() {
    let mut state = State::default();
    state.side_one.attack_boost = 5;
    state.side_one.speed_boost = 5;
    state.side_one.force_switch = true;
    state.side_one.baton_passing = true;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
            Instruction::ToggleBatonPassing(ToggleBatonPassingInstruction {
                side_ref: SideReference::SideOne,
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
fn test_batonpass_with_leechseed() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::LeechSeed);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BATONPASS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleBatonPassing(ToggleBatonPassingInstruction {
                side_ref: SideReference::SideOne,
            }),
            Instruction::ToggleSideOneForceSwitch,
            Instruction::SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction {
                new_choice: Choices::NONE,
                previous_choice: Choices::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switching_from_batonpass_with_leechseed() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::LeechSeed);
    state.side_one.force_switch = true;
    state.side_one.baton_passing = true;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    // leechseed remains (no instructions to remove it)
    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
            Instruction::ToggleBatonPassing(ToggleBatonPassingInstruction {
                side_ref: SideReference::SideOne,
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
fn test_non_baton_pass_switching_with_leechseed() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::LeechSeed);
    state.side_one.force_switch = true;
    state.side_one.baton_passing = false;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::LeechSeed,
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
fn test_switching_from_batonpass_with_sub() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 25;
    state.side_one.force_switch = true;
    state.side_one.baton_passing = true;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    // substitute remains (no instructions to remove it or change sub health)
    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
            Instruction::ToggleBatonPassing(ToggleBatonPassingInstruction {
                side_ref: SideReference::SideOne,
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
fn test_non_baton_pass_switching_with_sub() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::LeechSeed);
    state.side_one.force_switch = true;
    state.side_one.baton_passing = false;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::LeechSeed,
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
fn test_faster_uturn() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().speed = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 55,
            }),
            Instruction::ToggleSideOneForceSwitch,
            Instruction::SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction {
                new_choice: Choices::SPLASH,
                previous_choice: Choices::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_uturn_into_protect_does_not_cause_switchout() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::PROTECT,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_uturn_into_fainted_pkmn_does_not_cause_switchout() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::BRAVEBIRD,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 94,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_partingshot_into_protect_does_not_cause_switchout() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::PROTECT,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_faster_uturn_does_not_trigger_end_of_turn() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sand; // would normally cause damage to both sides
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().speed = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 55,
            }),
            Instruction::ToggleSideOneForceSwitch,
            Instruction::SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction {
                new_choice: Choices::SPLASH,
                previous_choice: Choices::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_faster_uturn_knocking_out_opponent() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().speed = 100;
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::ToggleSideOneForceSwitch,
            Instruction::SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction {
                new_choice: Choices::TACKLE,
                previous_choice: Choices::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_faster_uturn_with_opponent_move() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().speed = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 55,
            }),
            Instruction::ToggleSideOneForceSwitch,
            Instruction::SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction {
                new_choice: Choices::TACKLE,
                previous_choice: Choices::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_slower_uturn_with_opponent_move() {
    let mut state = State::default();
    state.side_one.get_active().speed = 100;
    state.side_two.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 55,
            }),
            Instruction::ToggleSideOneForceSwitch,
            Instruction::SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction {
                new_choice: Choices::NONE,
                previous_choice: Choices::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switch_out_move_does_not_trigger_end_of_turn() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sand; // would normally cause damage to both sides
    state.weather.turns_remaining = 5;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 55,
            }),
            Instruction::ToggleSideOneForceSwitch,
            Instruction::SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction {
                new_choice: Choices::NONE,
                previous_choice: Choices::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switch_out_move_does_not_trigger_if_user_is_last_alive_pkmn() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().speed = 100;

    state.side_one.pokemon[PokemonIndex::P1].hp = 0;
    state.side_one.pokemon[PokemonIndex::P2].hp = 0;
    state.side_one.pokemon[PokemonIndex::P3].hp = 0;
    state.side_one.pokemon[PokemonIndex::P4].hp = 0;
    state.side_one.pokemon[PokemonIndex::P5].hp = 0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::UTURN,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 55,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switch_out_move_does_not_trigger_if_voltswitch_missed() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().speed = 100;
    state.side_two.get_active().types.1 = PokemonType::Ground;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::VOLTSWITCH,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switchout_flag_where_faster_switchout_move_knocked_out_opponent() {
    let mut state = State::default();
    state.side_one.force_switch = true;
    state.side_two.switch_out_move_second_saved_move = Choices::TACKLE;
    state.side_two.get_active().hp = 0;
    state.side_two.get_active().moves[PokemonMoveIndex::M0] = Move {
        id: Choices::TACKLE,
        disabled: false,
        pp: 35,
        choice: MOVES.get(&Choices::TACKLE).unwrap().clone(),
    };

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
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
fn test_switchout_flag_where_slower_switchout_move_knocked_out_opponent() {
    let mut state = State::default();
    state.side_one.force_switch = true;
    state.side_two.switch_out_move_second_saved_move = Choices::NONE;
    state.side_two.get_active().hp = 0;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
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
fn test_switch_out_move_flag_is_unset_after_next_move() {
    let mut state = State::default();
    state.side_one.force_switch = true;
    state.side_two.switch_out_move_second_saved_move = Choices::TACKLE;
    state.side_two.get_active().moves[PokemonMoveIndex::M0] = Move {
        id: Choices::TACKLE,
        disabled: false,
        pp: 35,
        choice: MOVES.get(&Choices::TACKLE).unwrap().clone(),
    };

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_end_of_turn_triggered_when_switchout_flag_is_removed() {
    let mut state = State::default();
    state.weather = StateWeather {
        weather_type: Weather::Sand,
        turns_remaining: -1,
    };
    state.side_one.force_switch = true;
    state.side_two.switch_out_move_second_saved_move = Choices::TACKLE;
    state.side_two.get_active().moves[PokemonMoveIndex::M0] = Move {
        id: Choices::TACKLE,
        disabled: false,
        pp: 35,
        choice: MOVES.get(&Choices::TACKLE).unwrap().clone(),
    };

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_end_of_turn_triggered_when_switchout_flag_is_removed_and_other_side_did_nothing() {
    let mut state = State::default();
    state.weather = StateWeather {
        weather_type: Weather::Sand,
        turns_remaining: -1,
    };
    state.side_one.force_switch = true;
    state.side_two.switch_out_move_second_saved_move = Choices::NONE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleSideOneForceSwitch,
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_pkmn_is_not_trapped_if_it_has_fainted() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::ARENATRAP;
    state.side_two.get_active().hp = 0;

    let (side_one_moves, side_two_moves) = state.get_all_options();

    assert_eq!(vec![MoveChoice::None], side_one_moves);

    assert_eq!(
        vec![
            MoveChoice::Switch(PokemonIndex::P1),
            MoveChoice::Switch(PokemonIndex::P2),
            MoveChoice::Switch(PokemonIndex::P3),
            MoveChoice::Switch(PokemonIndex::P4),
            MoveChoice::Switch(PokemonIndex::P5),
        ],
        side_two_moves
    );
}

#[test]
fn test_arenatrap_traps_opponent() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::ARENATRAP;

    let (side_one_moves, side_two_moves) = state.get_all_options();

    assert_eq!(
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
        side_one_moves
    );

    // no switches allowed
    assert_eq!(
        vec![
            MoveChoice::Move(PokemonMoveIndex::M0),
            MoveChoice::Move(PokemonMoveIndex::M1),
            MoveChoice::Move(PokemonMoveIndex::M2),
            MoveChoice::Move(PokemonMoveIndex::M3),
        ],
        side_two_moves
    );
}

#[test]
fn test_arenatrap_does_not_trap_flying() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::ARENATRAP;
    state.side_two.get_active().types.1 = PokemonType::Flying;

    let (side_one_moves, side_two_moves) = state.get_all_options();

    assert_eq!(
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
        side_one_moves
    );

    assert_eq!(
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
        side_two_moves
    );
}

#[test]
fn test_arenatrap_does_not_trap_ghost() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::ARENATRAP;
    state.side_two.get_active().types.1 = PokemonType::Ghost;

    let (side_one_moves, side_two_moves) = state.get_all_options();

    assert_eq!(
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
        side_one_moves
    );

    assert_eq!(
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
        side_two_moves
    );
}

#[test]
fn test_arenatrap_does_not_trap_shedshell() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::ARENATRAP;
    state.side_two.get_active().item = Items::SHEDSHELL;

    let (side_one_moves, side_two_moves) = state.get_all_options();

    assert_eq!(
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
        side_one_moves
    );

    assert_eq!(
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
        side_two_moves
    );
}

#[test]
fn test_turn_after_switch_out_move_other_side_does_nothing() {
    let mut state = State::default();
    state.side_one.force_switch = true;
    state.side_two.switch_out_move_second_saved_move = Choices::NONE;

    let (side_one_moves, side_two_moves) = state.get_all_options();

    assert_eq!(
        vec![
            MoveChoice::Switch(PokemonIndex::P1),
            MoveChoice::Switch(PokemonIndex::P2),
            MoveChoice::Switch(PokemonIndex::P3),
            MoveChoice::Switch(PokemonIndex::P4),
            MoveChoice::Switch(PokemonIndex::P5),
        ],
        side_one_moves
    );

    assert_eq!(vec![MoveChoice::None], side_two_moves);
}

#[test]
fn test_lockedmove_prevents_switches() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::LockedMove);

    let (side_one_moves, _) = state.get_all_options();
    assert_eq!(
        vec![
            MoveChoice::Move(PokemonMoveIndex::M0),
            MoveChoice::Move(PokemonMoveIndex::M1),
            MoveChoice::Move(PokemonMoveIndex::M2),
            MoveChoice::Move(PokemonMoveIndex::M3),
        ],
        side_one_moves
    );
}

#[test]
fn test_turn_after_switch_out_move_other_side_has_forced_move() {
    let mut state = State::default();
    state.side_one.force_switch = true;
    state.side_two.switch_out_move_second_saved_move = Choices::TACKLE;
    state.side_two.get_active().moves[PokemonMoveIndex::M0] = Move {
        id: Choices::TACKLE,
        disabled: false,
        pp: 35,
        ..Default::default()
    };

    let (side_one_moves, side_two_moves) = state.get_all_options();

    assert_eq!(
        vec![
            MoveChoice::Switch(PokemonIndex::P1),
            MoveChoice::Switch(PokemonIndex::P2),
            MoveChoice::Switch(PokemonIndex::P3),
            MoveChoice::Switch(PokemonIndex::P4),
            MoveChoice::Switch(PokemonIndex::P5),
        ],
        side_one_moves
    );

    assert_eq!(vec![MoveChoice::Move(PokemonMoveIndex::M0)], side_two_moves);
}

#[test]
fn test_noretreat_with_vs_already() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::NoRetreat);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::NORETREAT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_poltergeist() {
    let mut state = State::default();
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Dragon);
    state.side_two.get_active().item = Items::BLUNDERPOLICY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POLTERGEIST,
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
                damage_amount: 86,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_shoreup_in_sand() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sand;
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SHOREUP,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 66,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 6,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rock_spdef_in_sand() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sand;
    state.side_one.get_active().types.0 = PokemonType::Rock;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::WATERGUN,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 44, // 66 normally
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rock_spdef_in_sand_versus_secretsword_doesnt_change_damageroll() {
    /*
    There was a bug that arose based on how I calculated the modifier for Secret Sword.
    I initially modified the basepower by the ratio of the defender's defense/special defense.
    This was incorrect for all use cases, specifically when the defender's spdef is modified
    later on in the calc by something like sandstorm
    */

    let mut state = State::default();
    state.side_one.get_active().hp = 300;
    state.side_one.get_active().maxhp = 300;

    // make them both immune to sand damage to make comparison of
    // instructions easier
    state.side_one.get_active().types.0 = PokemonType::Rock;
    state.side_two.get_active().types.0 = PokemonType::Rock;

    let first_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SECRETSWORD,
    );

    // spdef gets boosted, but it shouldnt affect secretsword
    state.weather.weather_type = Weather::Sand;
    state.side_one.get_active().types.0 = PokemonType::Rock;
    let second_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SECRETSWORD,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 135,
        })],
    }];
    assert_eq!(first_instructions, second_instructions);
    assert_eq!(first_instructions, expected_instructions);
}

#[test]
fn test_rain_turns_do_not_decrement_if_turns_remaining_are_negative() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Rain;
    state.weather.turns_remaining = -1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rain_turns_decrement_if_turns_remaining_are_greater_than_1() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Rain;
    state.weather.turns_remaining = 3;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::DecrementWeatherTurnsRemaining],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rain_ends_if_turns_remaining_is_1() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Rain;
    state.weather.turns_remaining = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DecrementWeatherTurnsRemaining,
            Instruction::ChangeWeather(ChangeWeather {
                new_weather: Weather::None,
                new_weather_turns_remaining: 0,
                previous_weather: Weather::Rain,
                previous_weather_turns_remaining: 0,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_sand_does_not_inflict_damage_when_ending() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sand;
    state.weather.turns_remaining = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DecrementWeatherTurnsRemaining,
            Instruction::ChangeWeather(ChangeWeather {
                new_weather: Weather::None,
                new_weather_turns_remaining: 0,
                previous_weather: Weather::Sand,
                previous_weather_turns_remaining: 0,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_trickroom_decrements() {
    let mut state = State::default();
    state.trick_room.active = true;
    state.trick_room.turns_remaining = 3;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::DecrementTrickRoomTurnsRemaining],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_trickroom_ends_when_decrementing_to_zero() {
    let mut state = State::default();
    state.trick_room.active = true;
    state.trick_room.turns_remaining = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DecrementTrickRoomTurnsRemaining,
            Instruction::ToggleTrickRoom(ToggleTrickRoomInstruction {
                currently_active: true,
                new_trickroom_turns_remaining: 0,
                previous_trickroom_turns_remaining: 0,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_morningsun_in_rain() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Rain;
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::MORNINGSUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideOne,
            heal_amount: 25,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_morningsun_in_sun() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sun;
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::MORNINGSUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideOne,
            heal_amount: 66,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_bodypress() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1000;
    state.side_two.get_active().maxhp = 1000;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BODYPRESS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 127,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_filletaway() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1000;
    state.side_two.get_active().maxhp = 1000;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::FILLETAWAY,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: -50,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 2,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 2,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Speed,
                amount: 2,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_spdef_drop_does_not_affect_fainted_pkmn() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Dragon);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SHADOWBALL,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_powerup_punch_works_on_kill() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POWERUPPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_powerup_punch_does_not_boost_if_self_knocked_out() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().ability = Abilities::IRONBARBS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POWERUPPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_volatile_is_not_applied_to_fainted_pkmn() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().ability = Abilities::NOGUARD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DYNAMICPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_filletaway_lowhp() {
    let mut state = State::default();
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::FILLETAWAY,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_clangoroussoul() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::CLANGOROUSSOUL,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: -33,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Defense,
                amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::SpecialDefense,
                amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Speed,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_boltbeak() {
    let mut state = State::default();
    state.side_one.get_active().speed = 500;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BOLTBEAK,
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
fn test_trickroom() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICKROOM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleTrickRoom(ToggleTrickRoomInstruction {
                currently_active: false,
                new_trickroom_turns_remaining: 5,
                previous_trickroom_turns_remaining: 0,
            }),
            Instruction::DecrementTrickRoomTurnsRemaining,
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_trickroom_when_trickroom_is_already_active() {
    let mut state = State::default();
    state.trick_room.active = true;
    state.trick_room.turns_remaining = 3;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICKROOM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ToggleTrickRoom(ToggleTrickRoomInstruction {
            currently_active: true,
            new_trickroom_turns_remaining: 0,
            previous_trickroom_turns_remaining: 3,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_wish() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WISH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetWish(SetWishInstruction {
                side_ref: SideReference::SideOne,
                wish_amount: state.side_one.get_active().maxhp / 2,
                previous_wish_amount: 0,
            }),
            Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_cannot_use_wish_when_already_active() {
    let mut state = State::default();
    state.side_one.wish = (2, 50);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WISH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::DecrementWish(DecrementWishInstruction {
            side_ref: SideReference::SideOne,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(not(feature = "gen4"))]
fn test_wish_healing_end_of_turn() {
    let mut state = State::default();
    state.side_one.wish = (1, 50);
    state.side_one.get_active().hp = 1;
    state.side_one.get_active().maxhp = 400;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 50,
            }),
            Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_wish_does_not_overheal() {
    let mut state = State::default();
    state.side_one.wish = (1, 50);
    state.side_one.get_active().hp = 75;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 25,
            }),
            Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_wish_does_not_produce_heal_instruction_when_at_maxhp() {
    let mut state = State::default();
    state.side_one.wish = (1, 50);
    state.side_one.get_active().hp = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::DecrementWish(DecrementWishInstruction {
            side_ref: SideReference::SideOne,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen4")]
fn test_wish_in_gen4_uses_target_pkmn_maxhp() {
    let mut state = State::default();
    state.side_one.wish = (1, 100);
    state.side_one.get_active().hp = 1;
    state.side_one.get_active().maxhp = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 50,
            }),
            Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_undo_trickroom() {
    let mut state = State::default();
    state.trick_room.active = true;
    state.trick_room.turns_remaining = 3;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICKROOM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ToggleTrickRoom(ToggleTrickRoomInstruction {
            currently_active: true,
            new_trickroom_turns_remaining: 0,
            previous_trickroom_turns_remaining: 3,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_painsplit() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_two.get_active().hp = 60;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PAINSPLIT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: -5,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 5,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_superfang() {
    let mut state = State::default();
    state.side_two.get_active().hp = 60;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUPERFANG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 30,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_seismictoss() {
    let mut state = State::default();
    state.side_one.get_active().level = 88;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SEISMICTOSS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 88,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_seismictoss_versus_ghost_type() {
    let mut state = State::default();
    state.side_one.get_active().level = 88;
    state.side_two.get_active().types.0 = PokemonType::Ghost;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SEISMICTOSS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_seismictoss_does_not_overkill() {
    let mut state = State::default();
    state.side_one.get_active().level = 88;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SEISMICTOSS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 50,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_superfang_at_1hp() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUPERFANG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_superfang_versus_ghost_type() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().types.0 = PokemonType::Ghost;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUPERFANG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_endeavor() {
    let mut state = State::default();
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ENDEAVOR,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 99,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_endeavor_when_higher_hp_than_opponent() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ENDEAVOR,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_endeavor_versus_ghost() {
    let mut state = State::default();
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().types.0 = PokemonType::Ghost;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ENDEAVOR,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_finalgambit_versus_ghost() {
    let mut state = State::default();
    state.side_one.get_active().hp = 100;
    state.side_two.get_active().types.0 = PokemonType::Ghost;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::FINALGAMBIT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_finalgambit() {
    let mut state = State::default();
    state.side_one.get_active().hp = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::FINALGAMBIT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 100,
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
fn test_basic_spore() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPORE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: SideReference::SideTwo,
            pokemon_index: PokemonIndex::P0,
            old_status: PokemonStatus::None,
            new_status: PokemonStatus::Sleep,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_sleep_clause_prevents_sleep_move_used_on_opponent() {
    let mut state = State::default();
    state.side_two.pokemon[PokemonIndex::P1].status = PokemonStatus::Sleep;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPORE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_removing_sleep_via_healbell_sets_sleep_turns_to_zero() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].status = PokemonStatus::Sleep;
    state.side_one.pokemon[PokemonIndex::P1].sleep_turns = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HEALBELL,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P1,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            }),
            Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P1,
                new_turns: 0,
                previous_turns: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_sleeppowder_as_faster_pkmn() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SLEEPPOWDER,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 25.0,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 75.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Sleep,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 1,
                    previous_turns: 0,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen4")]
fn test_gen4_one_turn_asleep_trying_to_wake_up() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Sleep;
    state.side_one.get_active().sleep_turns = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 75.0,
            instruction_list: vec![Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 2,
                previous_turns: 1,
            })],
        },
        StateInstructions {
            percentage: 25.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::Sleep,
                    new_status: PokemonStatus::None,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen5")]
fn test_gen5_switchout_while_sleep_resets_rest_turns() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Sleep;
    state.side_one.get_active().rest_turns = 1;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 3,
                previous_turns: 1,
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
#[cfg(feature = "gen5")]
fn test_gen5_switchout_while_sleep_resets_sleep_turns() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Sleep;
    state.side_one.get_active().sleep_turns = 1;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 0,
                previous_turns: 1,
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
#[cfg(feature = "gen5")]
fn test_gen5_one_turn_asleep_trying_to_wake_up() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Sleep;
    state.side_one.get_active().sleep_turns = 1;

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
                new_turns: 2,
                previous_turns: 1,
            })],
        },
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::Sleep,
                    new_status: PokemonStatus::None,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen5")]
fn test_gen5_guaranteed_wake_up() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Sleep;
    state.side_one.get_active().sleep_turns = 3;

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
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            }),
            Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 0,
                previous_turns: 3,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rest_can_be_used_if_sleep_clause_would_activate() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].status = PokemonStatus::Sleep;
    state.side_one.get_active().hp = 50;

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
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Sleep,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 3,
                previous_turns: 0,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 50,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_strengthsap() {
    let mut state = State::default();
    state.side_one.get_active().maxhp = 500;
    state.side_one.get_active().hp = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::STRENGTHSAP,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 100,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_gets_applied_and_swapped_with_next_turn_volatile() {
    let mut state = State::default();
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Yawn,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Yawn,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::YawnSleepThisTurn,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_substitute_blocks_yawn() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_protect_blocks_yawn() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::PROTECT,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_sleep_next_turn_causes_pkmn_to_sleep() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::YawnSleepThisTurn);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::YawnSleepThisTurn,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Sleep,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_cannot_reapply_yawn_when_already_inflicted() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::YawnSleepThisTurn);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::YawnSleepThisTurn,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Sleep,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_is_removed_but_no_status_change_if_pkmn_already_statused() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::YawnSleepThisTurn);
    state.side_two.get_active().status = PokemonStatus::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 12,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::YawnSleepThisTurn,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_cannot_be_inflicted_with_an_existing_status() {
    let mut state = State::default();
    state.side_two.get_active().status = PokemonStatus::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 12,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_cannot_be_inflicted_to_insomnia() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::INSOMNIA;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_cannot_be_inflicted_to_vitalspirit() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::VITALSPIRIT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_cannot_be_inflicted_with_electricterrain() {
    let mut state = State::default();
    state.terrain.terrain_type = Terrain::ElectricTerrain;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_cannot_be_inflicted_with_mistyterrain() {
    let mut state = State::default();
    state.terrain.terrain_type = Terrain::MistyTerrain;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_can_be_inflicted_with_electricterrain_on_nongrounded_pkmn() {
    let mut state = State::default();
    state.terrain.terrain_type = Terrain::ElectricTerrain;
    state.side_two.get_active().types.0 = PokemonType::Flying;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Yawn,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Yawn,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::YawnSleepThisTurn,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_yawn_can_be_inflicted_with_mistyterrain_when_target_is_not_grounded() {
    let mut state = State::default();
    state.terrain.terrain_type = Terrain::MistyTerrain;
    state.side_two.get_active().types.0 = PokemonType::Flying;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::YAWN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Yawn,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Yawn,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::YawnSleepThisTurn,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_perish_gets_applied_to_both_sides() {
    let mut state = State::default();
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PERISHSONG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish3,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish3,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_perish_bypasses_sub() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_two.substitute_health = 50;
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PERISHSONG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish3,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish3,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_perish_bypasses_protect() {
    let mut state = State::default();
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PERISHSONG,
        Choices::PROTECT,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish3,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish3,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_perish_cannot_be_applied_to_soundproof_pkmn() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::SOUNDPROOF;
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PERISHSONG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish3,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_perish_cannot_be_applied_to_pkmn_with_a_perish_volatile() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Perish2);
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PERISHSONG,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish2,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Perish1,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish4,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish3,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_perish1_causes_faint_end_of_turn() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Perish1);
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 100,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_perish1_switching_out_prevents_faint() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Perish1);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Perish1,
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
fn test_leechseed_does_not_trigger_if_receiving_side_fainted_this_turn() {
    let mut state = State::default();
    state.side_one.get_active().hp = 5;
    state.side_two.get_active().hp = 50;
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::LeechSeed);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 5,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_leechseed_into_substitute() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_two.substitute_health = 10;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::LEECHSEED,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_leechseed_into_grass_type() {
    let mut state = State::default();
    state.side_two.get_active().types.0 = PokemonType::Grass;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::LEECHSEED,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_solarbeam_not_in_sun() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SOLARBEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ApplyVolatileStatus(
            ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SolarBeam,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_solarbeam_with_active_volatile_status() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::SolarBeam);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SOLARBEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SolarBeam,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 94,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_solarbeam_with_powerherb() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::POWERHERB;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SOLARBEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::POWERHERB,
                new_item: Items::NONE,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 94,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_solarbeam_in_sun() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sun;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SOLARBEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 94,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_trick() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SILVERPOWDER;
    state.side_two.get_active().item = Items::EXPERTBELT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::SILVERPOWDER,
                new_item: Items::EXPERTBELT,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::EXPERTBELT,
                new_item: Items::SILVERPOWDER,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_trick_fails_versus_arceus_with_plate() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SILVERPOWDER;
    state.side_two.get_active().item = Items::SKYPLATE;
    state.side_two.get_active().id = "arceus".to_string();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_trick_versus_arceus_without_plate() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SILVERPOWDER;
    state.side_two.get_active().item = Items::LEFTOVERS;
    state.side_two.get_active().id = "arceus".to_string();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::SILVERPOWDER,
                new_item: Items::LEFTOVERS,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::LEFTOVERS,
                new_item: Items::SILVERPOWDER,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_trick_with_one_side_having_no_item() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SILVERPOWDER;
    state.side_two.get_active().item = Items::NONE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::SILVERPOWDER,
                new_item: Items::NONE,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::NONE,
                new_item: Items::SILVERPOWDER,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_trick_against_substitute_fails() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::SILVERPOWDER;
    state.side_two.get_active().item = Items::LEFTOVERS;
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_two.substitute_health = 10;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_identical_items_generates_no_instructions() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::NONE;
    state.side_two.get_active().item = Items::NONE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TRICK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_sunnyday() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SUNNYDAY,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeWeather(ChangeWeather {
                new_weather: Weather::Sun,
                new_weather_turns_remaining: 5,
                previous_weather: Weather::None,
                previous_weather_turns_remaining: -1,
            }),
            Instruction::DecrementWeatherTurnsRemaining,
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_focuspunch_after_not_getting_hit() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sun;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::FOCUSPUNCH,
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
fn test_grassknot_basepower_changing_based_on_weight() {
    let mut state = State::default();
    state.side_two.get_active().weight_kg = 10.0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::GRASSKNOT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 32,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_grassknot_basepower_changing_to_max_damage() {
    let mut state = State::default();
    state.side_two.get_active().weight_kg = 250.0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::GRASSKNOT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 94,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lowkick_basepower_lowest_damage() {
    let mut state = State::default();
    state.side_two.get_active().weight_kg = 1.0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::LOWKICK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 33,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lowkick_basepower_highest_damage() {
    let mut state = State::default();
    state.side_two.get_active().weight_kg = 250.0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::LOWKICK,
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
fn test_heavyslam_lowest_base_power() {
    let mut state = State::default();
    state.side_one.get_active().weight_kg = 250.0;
    state.side_two.get_active().weight_kg = 250.0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HEAVYSLAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 32,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_heavyslam_highest_base_power() {
    let mut state = State::default();
    state.side_one.get_active().weight_kg = 250.0;
    state.side_two.get_active().weight_kg = 10.0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HEAVYSLAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 94,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_heatcrash_highest_base_power() {
    let mut state = State::default();
    state.side_one.get_active().weight_kg = 250.0;
    state.side_two.get_active().weight_kg = 10.0;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::HEATCRASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 94,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_blizzard_in_hail() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Hail;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BLIZZARD,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 86,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
            ],
        },
        StateInstructions {
            percentage: 10.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 86,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Freeze,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_poisontype_using_toxic() {
    let mut state = State::default();
    state.side_one.get_active().types.0 = PokemonType::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TOXIC,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Toxic,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 6,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::ToxicCount,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8"))]
fn test_scenario_where_choice_gets_updated_on_second_move_that_has_branched_on_first_turn() {
    /*
    There was a bug that caused the choice to get incorrectly updated twice when
    the first move branched into a second move that caused the choice to update.
    */
    let mut state = State::default();
    state.side_two.get_active().speed = 150;
    state.side_one.get_active().item = Items::LIFEORB;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DARKPULSE,
        Choices::TOXIC,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 82,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: -10,
                }),
            ],
        },
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Toxic,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 82,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: -10,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicCount,
                    amount: 1,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_toxic_into_shedinja() {
    // makes sure that toxic always does at least 1 damage
    let mut state = State::default();
    state.side_one.get_active().types.0 = PokemonType::Poison;
    state.side_two.get_active().id = "shedinja".to_string();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().maxhp = 1;
    state.side_two.get_active().ability = Abilities::WONDERGUARD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TOXIC,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Toxic,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::ToxicCount,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_pursuit() {
    let mut state = State::default();
    state.side_one.get_active().moves.m0 = Move {
        id: Choices::PURSUIT,
        disabled: false,
        pp: 35,
        choice: MOVES.get(&Choices::PURSUIT).unwrap().to_owned(),
    };

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Switch(PokemonIndex::P1),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 63,
            }),
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideTwo,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_clangoroussoul_missing() {
    let mut state = State::default();
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::CLANGOROUSSOUL,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_poltergeist_missing() {
    let mut state = State::default();
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Dragon);
    state.side_two.get_active().item = Items::NONE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POLTERGEIST,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_expert_belt_does_not_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Dragon);
    state.side_one.get_active().item = Items::EXPERTBELT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DRAINPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 60,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_basic_multi_hit_move() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DRAGONDARTS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 40,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 40,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_multi_hit_move_where_first_hit_breaks_substitute() {
    let mut state = State::default();
    state.side_two.substitute_health = 10;
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DRAGONDARTS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 10,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 40,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_contact_multi_hit_move_versus_rockyhelmet() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::ROCKYHELMET;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SURGINGSTRIKES,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 21,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 21,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 21,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: -16,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(feature = "gen7", feature = "gen8", feature = "gen9"))]
fn test_souldew_20_percent_boost_on_dragon_move() {
    let mut state = State::default();
    state.side_one.get_active().id = String::from("latios");
    state.side_one.get_active().item = Items::SOULDEW;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DRAGONPULSE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 80,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(feature = "gen4", feature = "gen5", feature = "gen6"))]
fn test_earlier_gen_souldew_50_percent_boost_on_any_special_move() {
    let mut state = State::default();
    state.side_one.get_active().id = String::from("latios");
    state.side_one.get_active().item = Items::SOULDEW;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_skilllink_always_has_5_hits() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::ROCKYHELMET;
    state.side_one.get_active().ability = Abilities::SKILLLINK;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ROCKBLAST,
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
                    damage_amount: 21,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 21,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 21,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 21,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16, // 21 * 4 = 84, 16hp remaining on last hit
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_triple_multihit_move_versus_substitute_and_rockyhelmet() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::ROCKYHELMET;
    state.side_two.substitute_health = 25;
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SURGINGSTRIKES,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 21,
            }),
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 4,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 21,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: -16,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_scaleshot_only_boosts_once() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SCALESHOT,
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
                    damage_amount: 21,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 21,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 21,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: 1,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lifeorb_boost_and_recoil() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LIFEORB;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 61,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: -10,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_no_lifeorb_recoil_with_magicguard() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LIFEORB;
    state.side_one.get_active().ability = Abilities::MAGICGUARD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 61,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lifeorb_on_non_damaging_move() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LIFEORB;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::NASTYPLOT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideOne,
            stat: PokemonBoostableStat::SpecialAttack,
            amount: 2,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_refresh_curing_status() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Burn;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::REFRESH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: SideReference::SideOne,
            pokemon_index: PokemonIndex::P0,
            old_status: PokemonStatus::Burn,
            new_status: PokemonStatus::None,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_shellbell_drain() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().item = Items::SHELLBELL;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_absorbbulb() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::ABSORBBULB;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 32,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 1,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::ABSORBBULB,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_leafguard() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::LEAFGUARD;
    state.weather.weather_type = Weather::Sun;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPORE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_basic_levitate() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::LEVITATE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::EARTHQUAKE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_mold_breaker_into_levitate() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::MOLDBREAKER;
    state.side_two.get_active().ability = Abilities::LEVITATE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::EARTHQUAKE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 79,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_mold_breaker_into_waterabsorb() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::MOLDBREAKER;
    state.side_two.get_active().ability = Abilities::WATERABSORB;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 32,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_mold_breaker_into_wonderguard() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::MOLDBREAKER;
    state.side_two.get_active().ability = Abilities::WONDERGUARD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::EARTHQUAKE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 79,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lightning_rod() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::LIGHTNINGROD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::THUNDERSHOCK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideTwo,
            stat: PokemonBoostableStat::SpecialAttack,
            amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_motor_drive() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::MOTORDRIVE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::THUNDERSHOCK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideTwo,
            stat: PokemonBoostableStat::Speed,
            amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_lightning_rod_versus_status_move() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::LIGHTNINGROD;
    state.side_two.get_active().hp -= 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::THUNDERWAVE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideTwo,
            stat: PokemonBoostableStat::SpecialAttack,
            amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_liquidooze() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::LIQUIDOOZE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ABSORB,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 16,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: -8,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_stealthrock_basic() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::STEALTHROCK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeSideCondition(
            ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Stealthrock,
                amount: 1,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_stealthrock_after_opponent_faints_still_works() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::STEALTHROCK,
        Choices::BRAVEBIRD,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 94,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Stealthrock,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_magicbounce_with_side_condition() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::MAGICBOUNCE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::STEALTHROCK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeSideCondition(
            ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne, // side-one used SR, and gets it up
                side_condition: PokemonSideCondition::Stealthrock,
                amount: 1,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_magicbounce_with_side_condition_that_is_already_up() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::MAGICBOUNCE;
    state.side_one.side_conditions.stealth_rock = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::STEALTHROCK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_magicbounce_with_status() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::MAGICBOUNCE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WILLOWISP,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 14.999998,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 85.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Burn,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_marvelscale() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::MARVELSCALE;
    state.side_two.get_active().status = PokemonStatus::Paralyze;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 33,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_multiscale() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::MULTISCALE;

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
fn test_overcoat_vs_powder_move() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::OVERCOAT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPORE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_overcoat_vs_weather_damage() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::OVERCOAT;
    state.weather.weather_type = Weather::Hail;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 6,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_poisonpoint() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::POISONPOINT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 66.99999,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 33.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_owntempo_versus_intimidate() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::OWNTEMPO;
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_magicguard_switching_into_rocks() {
    let mut state = State::default();
    state.side_one.side_conditions.stealth_rock = 1;
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::MAGICGUARD;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_dauntlessshield() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::DAUNTLESSSHIELD;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Defense,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_intrepidsword() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTREPIDSWORD;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_slowstart_activates_on_switch_in() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::SLOWSTART;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::SlowStart,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_oblivious_versus_intimidate() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::OBLIVIOUS;
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_drizzle() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::DRIZZLE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::ChangeWeather(ChangeWeather {
                new_weather: Weather::Rain,
                new_weather_turns_remaining: -1,
                previous_weather: Weather::None,
                previous_weather_turns_remaining: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_electricsurge() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::ELECTRICSURGE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::ChangeTerrain(ChangeTerrain {
                new_terrain: Terrain::ElectricTerrain,
                new_terrain_turns_remaining: 5,
                previous_terrain: Terrain::None,
                previous_terrain_turns_remaining: 0,
            }),
            Instruction::DecrementTerrainTurnsRemaining,
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_hadronenegine_terrain_application() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::HADRONENGINE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::ChangeTerrain(ChangeTerrain {
                new_terrain: Terrain::ElectricTerrain,
                new_terrain_turns_remaining: 5,
                previous_terrain: Terrain::None,
                previous_terrain_turns_remaining: 0,
            }),
            Instruction::DecrementTerrainTurnsRemaining,
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_screencleaner() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::SCREENCLEANER;
    state.side_one.side_conditions.reflect = 1;
    state.side_one.side_conditions.aurora_veil = 1;
    state.side_two.side_conditions.light_screen = 1;
    state.side_two.side_conditions.reflect = 1;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Reflect,
                amount: -1,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Reflect,
                amount: -1,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::LightScreen,
                amount: -1,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::AuroraVeil,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_drought() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::DROUGHT;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::ChangeWeather(ChangeWeather {
                new_weather: Weather::Sun,
                new_weather_turns_remaining: -1,
                previous_weather: Weather::None,
                previous_weather_turns_remaining: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_download_for_defense() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::DOWNLOAD;
    state.side_two.get_active().defense = 100;
    state.side_two.get_active().special_defense = 150;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_download_for_defense_when_switching_in_with_baton_boosted_max_attack() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::DOWNLOAD;
    state.side_one.attack_boost = 6;
    state.side_one.baton_passing = true;
    state.side_two.get_active().defense = 100;
    state.side_two.get_active().special_defense = 150;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ToggleBatonPassing(ToggleBatonPassingInstruction {
                side_ref: SideReference::SideOne,
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
fn test_download_for_special_defense() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::DOWNLOAD;
    state.side_two.get_active().defense = 150;
    state.side_two.get_active().special_defense = 100;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_innerfocus_versus_intimidate() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::INNERFOCUS;
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_scrappy_versus_intimidate() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::SCRAPPY;
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_scrappy_versus_ghost_type() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::SCRAPPY;
    state.side_two.get_active().types.1 = PokemonType::Ghost;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_mindseye_versus_ghost_type() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::MINDSEYE;
    state.side_two.get_active().types.1 = PokemonType::Ghost;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_scrappy_fighting_move_becomes_supereffective_against_ghost_normal() {
    let mut state = State::default();

    state.side_two.get_active().types.0 = PokemonType::Normal;
    state.side_two.get_active().types.1 = PokemonType::Typeless;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::KARATECHOP,
        Choices::SPLASH,
    );

    state.side_one.get_active().ability = Abilities::SCRAPPY;
    state.side_two.get_active().types.1 = PokemonType::Ghost;

    let vec_of_instructions_after_scrappy = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::KARATECHOP,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 81,
        })],
    }];
    assert_eq!(vec_of_instructions, vec_of_instructions_after_scrappy);
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_substitute_versus_intimidate() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_two.substitute_health = 25;
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_intimidate() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_ground_move_versus_airballoon() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::AIRBALLOON;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::EARTHQUAKE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_non_ground_move_versus_airballoon() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::AIRBALLOON;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::AIRBALLOON,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_assaultvest() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::ASSAULTVEST;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 22,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_weaknesspolicy() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::WEAKNESSPOLICY;
    state.side_two.get_active().hp = 200;
    state.side_two.get_active().maxhp = 200;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 64,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: 2,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 2,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::WEAKNESSPOLICY,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_weaknesspolicy_does_not_overboost() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::WEAKNESSPOLICY;
    state.side_two.get_active().hp = 200;
    state.side_two.get_active().maxhp = 200;
    state.side_two.attack_boost = 5;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 64,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: 2,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::WEAKNESSPOLICY,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switching_in_with_grassyseed_in_grassy_terrain() {
    let mut state = State::default();
    state.side_two.pokemon[PokemonIndex::P1].item = Items::GRASSYSEED;
    state.terrain.terrain_type = Terrain::GrassyTerrain;
    state.terrain.turns_remaining = 3;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Switch(PokemonIndex::P1),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideTwo,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Defense,
                amount: 1,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::GRASSYSEED,
                new_item: Items::NONE,
            }),
            Instruction::DecrementTerrainTurnsRemaining,
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_contrary_with_seed() {
    let mut state = State::default();
    state.side_two.pokemon[PokemonIndex::P1].item = Items::PSYCHICSEED;
    state.side_two.pokemon[PokemonIndex::P1].ability = Abilities::CONTRARY;
    state.terrain.terrain_type = Terrain::PsychicTerrain;
    state.terrain.turns_remaining = 3;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Switch(PokemonIndex::P1),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideTwo,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialDefense,
                amount: -1,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: Items::PSYCHICSEED,
                new_item: Items::NONE,
            }),
            Instruction::DecrementTerrainTurnsRemaining,
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_contrary_when_pre_swapped_boost_goes_above_max() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::CONTRARY;
    state.side_two.attack_boost = 6;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::CHARM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_contrary() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::CONTRARY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SWORDSDANCE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideOne,
            stat: PokemonBoostableStat::Attack,
            amount: -2,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_contrary_with_secondary() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::CONTRARY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POWERUPPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 64,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_shielddust_doesnt_stop_self_secondary() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::SHIELDDUST;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POWERUPPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 64,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_stamina() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::STAMINA;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Defense,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_steamengine() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::STEAMENGINE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 32,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Speed,
                amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_stormdrain() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::STORMDRAIN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideTwo,
            stat: PokemonBoostableStat::SpecialAttack,
            amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_shielddust_stops_secondary_against_opponent() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::SHIELDDUST;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::THUNDERPUNCH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 60,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_throatspray_with_move_that_can_miss() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::THROATSPRAY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::GRASSWHISTLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 45.0,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 55.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Sleep,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::SpecialAttack,
                    amount: 1,
                }),
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: SideReference::SideOne,
                    current_item: Items::THROATSPRAY,
                    new_item: Items::NONE,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_adaptability() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::ADAPTABILITY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
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
fn test_armortail_against_priority() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::ARMORTAIL;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::QUICKATTACK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_moldbreaker_negating_armortail() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::MOLDBREAKER;
    state.side_two.get_active().ability = Abilities::ARMORTAIL;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::QUICKATTACK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_armortail_against_non_priority() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::ARMORTAIL;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 48,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_thickclub() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::THICKCLUB;
    state.side_one.get_active().id = "marowak".to_string();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 95,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_poisontouch_with_poisonjab() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::POISONTOUCH;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POISONJAB,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 49.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 63,
            })],
        },
        StateInstructions {
            percentage: 51.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 63,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 12,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_serenegrace_with_secondary() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::SERENEGRACE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POISONJAB,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 39.999996,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 63,
            })],
        },
        StateInstructions {
            percentage: 60.000004,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 63,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 12,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_technician() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::TECHNICIAN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 72,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_hadronengine_boost() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::HADRONENGINE;
    state.terrain.terrain_type = Terrain::ElectricTerrain;
    state.terrain.turns_remaining = 5;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 42,
            }),
            Instruction::DecrementTerrainTurnsRemaining,
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_unseenfist() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::UNSEENFIST;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::PROTECT,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Protect,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::Protect,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_ironbarbs() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::IRONBARBS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rattled() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::RATTLED;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::FEINTATTACK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Speed,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taunt_into_aromaveil() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::AROMAVEIL;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TAUNT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taunt_into_glare() {
    let mut state = State::default();
    state.side_one.get_active().speed = 105;
    state.side_two.get_active().speed = 100;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TAUNT,
        Choices::GLARE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ApplyVolatileStatus(
            ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Taunt,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
fn test_glare_into_electric_type() {
    let mut state = State::default();
    state.side_two.get_active().types.1 = PokemonType::Electric;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::GLARE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(feature = "gen4"))]
fn test_gen4_glare_into_electric_type() {
    let mut state = State::default();
    state.side_two.get_active().types.1 = PokemonType::Electric;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::GLARE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 25.0,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 75.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Paralyze,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_explosion_into_damp() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::DAMP;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::EXPLOSION,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_waterabsorb() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::WATERABSORB;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 25,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_eartheater() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::EARTHEATER;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::EARTHQUAKE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 25,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_dryskin_from_water_move() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::DRYSKIN;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 25,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_dryskin_prevents_scald_brun() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::DRYSKIN;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SCALD,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 25,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_voltabsorb() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::VOLTABSORB;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::THUNDERSHOCK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 25,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_watercompaction() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::WATERCOMPACTION;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 32,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Defense,
                amount: 2,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(feature = "gen4", feature = "gen5"))]
fn test_gen5_or_earlier_ghost_versus_steel() {
    let mut state = State::default();
    state.side_two.get_active().types = (PokemonType::Steel, PokemonType::Typeless);

    let vec_of_instructions =
        set_moves_on_pkmn_and_call_generate_instructions(&mut state, Choices::HEX, Choices::SPLASH);

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 20,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(feature = "gen7", feature = "gen8"))]
fn test_pixilate() {
    let mut state = State::default();
    state.side_one.get_active().types = (PokemonType::Fairy, PokemonType::Typeless);
    state.side_one.get_active().ability = Abilities::PIXILATE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 58,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen6")]
fn test_pixilate_gen6() {
    let mut state = State::default();
    state.side_one.get_active().types = (PokemonType::Fairy, PokemonType::Typeless);
    state.side_one.get_active().ability = Abilities::PIXILATE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 61,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_transistor() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::TRANSISTOR;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::THUNDERSHOCK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 41,
            })],
        },
        StateInstructions {
            percentage: 10.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 41,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Paralyze,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(any(
    feature = "gen8",
    feature = "gen7",
    feature = "gen6",
    feature = "gen5",
    feature = "gen4"
))]
fn test_transistor_higher_boost_before_gen8() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::TRANSISTOR;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::THUNDERSHOCK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 10.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Paralyze,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[cfg(any(feature = "gen9"))]
#[test]
fn test_wickedblow_gen9() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WICKEDBLOW,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 60,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen8")]
fn test_wickedblow_gen8() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WICKEDBLOW,
        Choices::SPLASH,
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
#[cfg(feature = "gen7")]
fn test_gen7_rapidspin_does_not_boost_speed() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::RAPIDSPIN,
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
#[cfg(feature = "gen9")]
fn test_gen9_rapidspin_boosts_speed() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::RAPIDSPIN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 61,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Speed,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_weakarmor() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::WEAKARMOR;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Defense,
                amount: -1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Speed,
                amount: 2,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_wonderguard() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::WONDERGUARD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_wonderguard_against_spore() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::WONDERGUARD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPORE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: SideReference::SideTwo,
            pokemon_index: PokemonIndex::P0,
            old_status: PokemonStatus::None,
            new_status: PokemonStatus::Sleep,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen9")]
fn test_wonderguard_against_willowisp() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::WONDERGUARD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WILLOWISP,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 14.999998,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 85.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Burn,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
#[cfg(feature = "gen4")]
fn test_wonderguard_1_hp_against_willowisp() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().maxhp = 1;
    state.side_two.get_active().ability = Abilities::WONDERGUARD;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WILLOWISP,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 25.0,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 75.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Burn,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 1,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_wonderguard_1hp_against_poison() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().maxhp = 1;
    state.side_two.get_active().ability = Abilities::WONDERGUARD;
    state.side_two.get_active().status = PokemonStatus::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_wonderguard_1hp_against_toxic() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;
    state.side_two.get_active().maxhp = 1;
    state.side_two.get_active().ability = Abilities::WONDERGUARD;
    state.side_two.get_active().status = PokemonStatus::Toxic;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideTwo,
                side_condition: PokemonSideCondition::ToxicCount,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_wonderskin_against_poisonpowder() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::WONDERSKIN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::POISONPOWDER,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 50.0,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 50.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 12,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_dryskin_does_not_overheal() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::DRYSKIN;
    state.side_two.get_active().hp = 90;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 10,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_dryskin_in_rain() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::DRYSKIN;
    state.side_two.get_active().hp = 90;
    state.weather.weather_type = Weather::Rain;
    state.weather.turns_remaining = -1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 10,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_shedskin_end_of_turn() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::SHEDSKIN;
    state.side_two.get_active().status = PokemonStatus::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 12,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Poison,
                new_status: PokemonStatus::None,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_hydration_end_of_turn() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::HYDRATION;
    state.side_two.get_active().status = PokemonStatus::Poison;
    state.weather.weather_type = Weather::Rain;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 12,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Poison,
                new_status: PokemonStatus::None,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_rest_with_existing_status_condition_and_hydration() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::HYDRATION;
    state.side_one.get_active().status = PokemonStatus::Burn;
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().rest_turns = 0;
    state.weather.weather_type = Weather::Rain;

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
                old_status: PokemonStatus::Burn,
                new_status: PokemonStatus::Sleep,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 3,
                previous_turns: 0,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 50,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 0,
                previous_turns: 3,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_rest_with_existing_status_condition_and_shedskin() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::SHEDSKIN;
    state.side_one.get_active().status = PokemonStatus::Burn;
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().rest_turns = 0;

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
                old_status: PokemonStatus::Burn,
                new_status: PokemonStatus::Sleep,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 3,
                previous_turns: 0,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 50,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 0,
                previous_turns: 3,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_being_knocked_out_before_using_rest() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::HYDRATION;
    state.side_one.get_active().status = PokemonStatus::Burn;
    state.side_one.get_active().hp = 1;
    state.side_one.get_active().rest_turns = 0;
    state.weather.weather_type = Weather::Rain;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::REST,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_hydration_without_weather() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::HYDRATION;
    state.side_two.get_active().status = PokemonStatus::Poison;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 12,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_icebody_no_heal() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::ICEBODY;
    state.weather.weather_type = Weather::Hail;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 6,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_icebody_heal() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::ICEBODY;
    state.weather.weather_type = Weather::Hail;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideTwo,
                heal_amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_raindish_heal() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::RAINDISH;
    state.weather.weather_type = Weather::Rain;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 6,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_solarpower_damage() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::SOLARPOWER;
    state.weather.weather_type = Weather::Sun;
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 12,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_baddreams() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::BADDREAMS;
    state.side_two.get_active().status = PokemonStatus::Sleep;
    state.side_two.get_active().sleep_turns = MAX_SLEEP_TURNS - 2;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 66.666664,
            instruction_list: vec![
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: MAX_SLEEP_TURNS - 1,
                    previous_turns: MAX_SLEEP_TURNS - 2,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 12,
                }),
            ],
        },
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::Sleep,
                    new_status: PokemonStatus::None,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: MAX_SLEEP_TURNS - 2,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_freeze_chance_to_thaw() {
    let mut state = State::default();
    state.side_two.get_active().status = PokemonStatus::Freeze;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 80.0,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 20.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::Freeze,
                new_status: PokemonStatus::None,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_sleeptalk_when_asleep_and_rest_turns_active() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::Sleep;
    state.side_one.get_active().rest_turns = 3;
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::REST);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::SLEEPTALK);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M2, Choices::TACKLE);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M3, Choices::CURSE);
    let before_state_string = format!("{:?}", state);
    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );
    let after_state_string = format!("{:?}", state);
    assert_eq!(before_state_string, after_state_string);

    let expected_instructions = vec![
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
                Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        },
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
                Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
            ],
        },
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
                Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Curse,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: -1,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_baddreams_does_not_overkill() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::BADDREAMS;
    state.side_two.get_active().status = PokemonStatus::Sleep;
    state.side_two.get_active().sleep_turns = MAX_SLEEP_TURNS - 2;
    state.side_two.get_active().hp = 5;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 66.666664,
            instruction_list: vec![
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: MAX_SLEEP_TURNS - 1,
                    previous_turns: MAX_SLEEP_TURNS - 2,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 5,
                }),
            ],
        },
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::Sleep,
                    new_status: PokemonStatus::None,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: MAX_SLEEP_TURNS - 2,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_filter() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::FILTER;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 49,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_prismarmor() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::PRISMARMOR;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::WATERGUN,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 49,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_effectspore() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::EFFECTSPORE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 72.891,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 9.009,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Sleep,
                }),
            ],
        },
        StateInstructions {
            percentage: 9.1,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Paralyze,
                }),
            ],
        },
        StateInstructions {
            percentage: 8.999999,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_flashfire() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::FLASHFIRE;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::EMBER,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ApplyVolatileStatus(
            ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::FlashFire,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_hypercutter() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::HYPERCUTTER;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::AURORABEAM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 51,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_innerfocus() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().ability = Abilities::INNERFOCUS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::IRONHEAD,
        Choices::SPLASH,
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
fn test_battle_is_over_when_battle_is_not_over() {
    let state = State::default();
    assert_eq!(state.battle_is_over(), 0.0);
}

#[test]
fn test_battle_is_over_when_side_one_lost() {
    let mut state = State::default();
    for pkmn_index in pokemon_index_iter() {
        state.side_one.pokemon[pkmn_index].hp = 0;
    }

    assert_eq!(state.battle_is_over(), -1.0);
}

#[test]
fn test_battle_is_over_when_side_two_lost() {
    let mut state = State::default();
    for pkmn_index in pokemon_index_iter() {
        state.side_two.pokemon[pkmn_index].hp = 0;
    }

    assert_eq!(state.battle_is_over(), 1.0);
}

#[test]
fn test_battle_is_over_when_side_two_has_unrevealed_pkmn() {
    let mut state = State::default();
    state.side_two.pokemon[PokemonIndex::P0].hp = 0;
    state.side_two.pokemon[PokemonIndex::P1].hp = 0;
    state.side_two.pokemon[PokemonIndex::P2].hp = 0;
    state.side_two.pokemon[PokemonIndex::P3].hp = 0;
    state.side_two.pokemon[PokemonIndex::P4].hp = 0;
    state.side_two.pokemon[PokemonIndex::P5].hp = 0;
    state.side_two.pokemon[PokemonIndex::P5].level = 1;

    assert_eq!(state.battle_is_over(), 0.0);
}
