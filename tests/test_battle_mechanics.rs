use poke_engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::choices::{Choice, MoveCategory, MOVES};
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeSideConditionInstruction, ChangeStatusInstruction, DamageInstruction,
    DisableMoveInstruction, EnableMoveInstruction, HealInstruction, Instruction,
    RemoveVolatileStatusInstruction, SetSubstituteHealthInstruction, StateInstructions,
    SwitchInstruction,
};
use poke_engine::state::{
    Move, PokemonBoostableStat, PokemonSideCondition, PokemonStatus, PokemonType,
    PokemonVolatileStatus, SideReference, State, Terrain, Weather,
};

#[test]
fn test_basic_move_pair_instruction_generation() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_move_pair_instruction_generation_where_first_move_branches() {
    let mut state = State::default();
    state.side_one.get_active().speed = 101;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("playrough").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_move_pair_instruction_generation_where_second_move_branches() {
    let mut state = State::default();
    state.side_one.get_active().speed = 50;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("playrough").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_basic_flinching_functionality() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("ironhead").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
    );

    let expected_instructions = vec![
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
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_fliching_first_and_second_move() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("ironhead").unwrap().clone(),
        MOVES.get("ironhead").unwrap().clone(),
    );

    let expected_instructions = vec![
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
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_flinching_on_move_that_can_miss() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("airslash").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_using_protect_against_damaging_move() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_self_boosting_move_against_protect() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("swordsdance").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_crash_move_into_protect() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("jumpkick").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_protect_stops_secondaries() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("ironhead").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_protect_stops_after_damage_hit_callback() {
    let mut state = State::default();
    state.side_one.get_active().item = String::from("dummyitem");

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("knockoff").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_move_that_goes_through_protect() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("feint").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_using_spikyshield_against_contact_move() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("spikyshield").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_spikyshield_recoil_does_not_overkill() {
    let mut state = State::default();
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("spikyshield").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_spikyshield_does_not_activate_on_non_contact_move() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("spikyshield").unwrap().clone(),
        MOVES.get("watergun").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_banefulbunker_poisons() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("banefulbunker").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
                pokemon_index: 0,
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_banefulbunker_cannot_poison_already_statused_target() {
    let mut state = State::default();
    state.side_two.get_active().status = PokemonStatus::Burn;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("banefulbunker").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
                damage_amount: 6,
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_silktrap() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("silktrap").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_protect_side_condition_is_removed() {
    let mut state = State::default();
    state.side_one.side_conditions.protect = 1;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("splash").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_protect_for_second_turn_in_a_row() {
    let mut state = State::default();
    state.side_one.side_conditions.protect = 1;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_double_protect() {
    let mut state = State::default();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("protect").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_basic_substitute_usage() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("substitute").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_substitute_does_not_let_secondary_status_effect_happen() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("substitute").unwrap().clone(),
        MOVES.get("firepunch").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_secondary_on_self_works_against_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("substitute").unwrap().clone(),
        MOVES.get("poweruppunch").unwrap().clone(),
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
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_move_goes_through_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("substitute").unwrap().clone(),
        MOVES.get("boomburst").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_infiltrator_goes_through_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().ability = "infiltrator".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("substitute").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_using_protect_with_a_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .get_active()
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.get_active().substitute_health = 25;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("tackle").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_drag_move_against_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .get_active()
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.get_active().substitute_health = 25;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("splash").unwrap().clone(),
        MOVES.get("dragontail").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_whirlwind_move_against_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .get_active()
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.get_active().substitute_health = 25;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("splash").unwrap().clone(),
        MOVES.get("whirlwind").unwrap().clone(),
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
                    previous_index: 0,
                    next_index: 1,
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
                    previous_index: 0,
                    next_index: 2,
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
                    previous_index: 0,
                    next_index: 3,
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
                    previous_index: 0,
                    next_index: 4,
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
                    previous_index: 0,
                    next_index: 5,
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_drag_move_against_protect_and_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .get_active()
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.get_active().substitute_health = 25;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("protect").unwrap().clone(),
        MOVES.get("dragontail").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_rockyhelmet_damage_taken() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().item = String::from("rockyhelmet");

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
                heal_amount: -12,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_rockyhelmet_does_not_overkill() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().item = String::from("rockyhelmet");

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_choiceband_locking() {
    let mut state = State::default();
    state.side_one.get_active().item = String::from("choiceband");
    state.side_one.get_active().moves[0] = Move {
        id: "willowisp".to_string(),
        disabled: false,
        pp: 35,
    };

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("willowisp").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 14.999998,
            instruction_list: vec![
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 1,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 2,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 3,
                }),
            ],
        },
        StateInstructions {
            percentage: 85.0,
            instruction_list: vec![
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 1,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 2,
                }),
                Instruction::DisableMove(DisableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 3,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: 0,
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_locked_moves_unlock_on_switchout() {
    let mut state = State::default();
    state.side_one.get_active().moves[1].disabled = true;
    state.side_one.get_active().moves[2].disabled = true;
    state.side_one.get_active().moves[3].disabled = true;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        Choice {
            category: MoveCategory::Switch,
            switch_id: 1,
            ..Default::default()
        },
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::EnableMove(EnableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: 1,
            }),
            Instruction::EnableMove(EnableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: 2,
            }),
            Instruction::EnableMove(EnableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: 3,
            }),
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_fighting_move_with_blackbelt() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_one.get_active().item = "blackbelt".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("drainpunch").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 142,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_expert_belt_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_one.get_active().item = "expertbelt".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("drainpunch").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 142,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_expert_belt_does_not_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Dragon);
    state.side_one.get_active().item = "expertbelt".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("drainpunch").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 60,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_lifeorb_boost_and_recoil() {
    let mut state = State::default();
    state.side_one.get_active().item = "lifeorb".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_shellbell_drain() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().item = "shellbell".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_absorbbulb() {
    let mut state = State::default();
    state.side_two.get_active().item = "absorbbulb".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("watergun").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
                current_item: "absorbbulb".to_string(),
                new_item: "".to_string(),
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_ground_move_versus_airballoon() {
    let mut state = State::default();
    state.side_two.get_active().item = "airballoon".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("earthquake").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_non_ground_move_versus_airballoon() {
    let mut state = State::default();
    state.side_two.get_active().item = "airballoon".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
                current_item: "airballoon".to_string(),
                new_item: "".to_string(),
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_assaultvest() {
    let mut state = State::default();
    state.side_two.get_active().item = "assaultvest".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("watergun").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 22,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_weaknesspolicy() {
    let mut state = State::default();
    state.side_two.get_active().item = "weaknesspolicy".to_string();
    state.side_two.get_active().hp = 200;
    state.side_two.get_active().maxhp = 200;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("watergun").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
                current_item: "weaknesspolicy".to_string(),
                new_item: "".to_string(),
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_weaknesspolicy_does_not_overboost() {
    let mut state = State::default();
    state.side_two.get_active().item = "weaknesspolicy".to_string();
    state.side_two.get_active().hp = 200;
    state.side_two.get_active().maxhp = 200;
    state.side_two.get_active().attack_boost = 5;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("watergun").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
                current_item: "weaknesspolicy".to_string(),
                new_item: "".to_string(),
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_switching_in_with_grassyseed_in_grassy_terrain() {
    let mut state = State::default();
    state.side_two.pokemon[1].item = "grassyseed".to_string();
    state.terrain.terrain_type = Terrain::GrassyTerrain;
    state.terrain.turns_remaining = 3;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("splash").unwrap().clone(),
        Choice {
            category: MoveCategory::Switch,
            switch_id: 1,
            ..Default::default()
        },
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideTwo,
                previous_index: 0,
                next_index: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Defense,
                amount: 1,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: "grassyseed".to_string(),
                new_item: "".to_string(),
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_contrary_with_seed() {
    let mut state = State::default();
    state.side_two.pokemon[1].item = "psychicseed".to_string();
    state.side_two.pokemon[1].ability = "contrary".to_string();
    state.terrain.terrain_type = Terrain::PsychicTerrain;
    state.terrain.turns_remaining = 3;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("splash").unwrap().clone(),
        Choice {
            category: MoveCategory::Switch,
            switch_id: 1,
            ..Default::default()
        },
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideTwo,
                previous_index: 0,
                next_index: 1,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::SpecialDefense,
                amount: -1,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideTwo,
                current_item: "psychicseed".to_string(),
                new_item: "".to_string(),
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_contrary() {
    let mut state = State::default();
    state.side_one.get_active().ability = "contrary".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("swordsdance").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideOne,
            stat: PokemonBoostableStat::Attack,
            amount: -2,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_contrary_with_secondary() {
    let mut state = State::default();
    state.side_one.get_active().ability = "contrary".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("poweruppunch").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_throatspray_with_move_that_can_miss() {
    let mut state = State::default();
    state.side_one.get_active().item = "throatspray".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("grasswhistle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
                    pokemon_index: 0,
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
                    current_item: "throatspray".to_string(),
                    new_item: "".to_string(),
                }),
            ],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_adaptability() {
    let mut state = State::default();
    state.side_one.get_active().ability = "adaptability".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 63,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_poisonpoint_with_poisonjab() {
    let mut state = State::default();
    state.side_one.get_active().ability = "poisonpoint".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("poisonjab").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 51.000004,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 63,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: 0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 12,
                }),
            ],
        },
        StateInstructions {
            percentage: 49.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 63,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_serenegrace_with_secondary() {
    let mut state = State::default();
    state.side_one.get_active().ability = "serenegrace".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("poisonjab").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 60.000004,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 63,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: 0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 12,
                }),
            ],
        },
        StateInstructions {
            percentage: 39.999996,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 63,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_technician() {
    let mut state = State::default();
    state.side_one.get_active().ability = "technician".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 72,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_unseenfist() {
    let mut state = State::default();
    state.side_one.get_active().ability = "unseenfist".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("protect").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_ironbarbs() {
    let mut state = State::default();
    state.side_two.get_active().ability = "ironbarbs".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
                heal_amount: -12,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_rattled() {
    let mut state = State::default();
    state.side_two.get_active().ability = "rattled".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("feintattack").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_taunt_into_aromaveil() {
    let mut state = State::default();
    state.side_two.get_active().ability = "aromaveil".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("taunt").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_explosion_into_damp() {
    let mut state = State::default();
    state.side_two.get_active().ability = "damp".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("explosion").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_waterabsorb() {
    let mut state = State::default();
    state.side_two.get_active().ability = "waterabsorb".to_string();
    state.side_two.get_active().hp = 50;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("watergun").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 25,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_dryskin_does_not_overheal() {
    let mut state = State::default();
    state.side_two.get_active().ability = "dryskin".to_string();
    state.side_two.get_active().hp = 90;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("watergun").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 10,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_dryskin_in_rain() {
    let mut state = State::default();
    state.side_two.get_active().ability = "dryskin".to_string();
    state.side_two.get_active().hp = 90;
    state.weather.weather_type = Weather::Rain;
    state.weather.turns_remaining = 5;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("splash").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 10,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_filter() {
    let mut state = State::default();
    state.side_two.get_active().ability = "filter".to_string();
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("watergun").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 49,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_effectspore() {
    let mut state = State::default();
    state.side_two.get_active().ability = "effectspore".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("tackle").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 8.999999,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: 0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
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
                    pokemon_index: 0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Paralyze,
                }),
            ],
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
                    pokemon_index: 0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Sleep,
                }),
            ],
        },
        StateInstructions {
            percentage: 72.891,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_flashfire() {
    let mut state = State::default();
    state.side_two.get_active().ability = "flashfire".to_string();
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("ember").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
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
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_hypercutter() {
    let mut state = State::default();
    state.side_two.get_active().ability = "hypercutter".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("aurorabeam").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 51,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_innerfocus() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_two.get_active().ability = "innerfocus".to_string();

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        MOVES.get("ironhead").unwrap().clone(),
        MOVES.get("splash").unwrap().clone(),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 63,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions)
}
