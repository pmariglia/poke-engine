use poke_engine::abilities::Abilities;
use poke_engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeSideConditionInstruction, ChangeStatusInstruction, DamageInstruction,
    DisableMoveInstruction, EnableMoveInstruction, HealInstruction, Instruction,
    RemoveVolatileStatusInstruction, SetSubstituteHealthInstruction, StateInstructions,
    SwitchInstruction,
};
use poke_engine::items::Items;
use poke_engine::state::{
    Move, MoveChoice, Pokemon, PokemonBoostableStat, PokemonSideCondition, PokemonStatus,
    PokemonType, PokemonVolatileStatus, Side, SideConditions, SideReference, State, StateTerrain,
    StateWeather, Terrain, Weather,
};

fn set_moves_on_pkmn_and_call_generate_instructions(
    state: &mut State,
    move_one: String,
    move_two: String,
) -> Vec<StateInstructions> {
    state.side_one.get_active().replace_move(0, move_one);
    state.side_two.get_active().replace_move(0, move_two);

    let before_state_string = format!("{:?}", state);
    let instructions =
        generate_instructions_from_move_pair(state, &MoveChoice::Move(0), &MoveChoice::Move(0));

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

    let side_one_move = MoveChoice::Switch(1);
    let side_two_move = MoveChoice::None;
    let vec_of_instructions =
        generate_instructions_from_move_pair(&mut state, &side_one_move, &side_two_move);

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: 0,
            next_index: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_basic_move_pair_instruction_generation() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("tackle"),
        String::from("tackle"),
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
        String::from("playrough"),
        String::from("tackle"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_move_pair_instruction_generation_where_second_move_branches() {
    let mut state = State::default();
    state.side_one.get_active().speed = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("playrough"),
        String::from("tackle"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_basic_flinching_functionality() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("ironhead"),
        String::from("tackle"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_fliching_first_and_second_move() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("ironhead"),
        String::from("ironhead"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_flinching_on_move_that_can_miss() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150; // faster than side two

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("airslash"),
        String::from("tackle"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_protect_against_damaging_move() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("protect"),
        String::from("tackle"),
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
        String::from("protect"),
        String::from("swordsdance"),
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
        String::from("protect"),
        String::from("jumpkick"),
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
        String::from("protect"),
        String::from("ironhead"),
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
        String::from("protect"),
        String::from("knockoff"),
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
fn test_move_that_goes_through_protect() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("protect"),
        String::from("feint"),
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
        String::from("spikyshield"),
        String::from("tackle"),
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
        String::from("spikyshield"),
        String::from("tackle"),
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
fn test_spikyshield_does_not_activate_on_non_contact_move() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("spikyshield"),
        String::from("watergun"),
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
        String::from("banefulbunker"),
        String::from("tackle"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_banefulbunker_cannot_poison_already_statused_target() {
    let mut state = State::default();
    state.side_two.get_active().status = PokemonStatus::Burn;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("banefulbunker"),
        String::from("tackle"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_silktrap() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("silktrap"),
        String::from("tackle"),
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
        String::from("splash"),
        String::from("splash"),
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
        String::from("protect"),
        String::from("tackle"),
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
fn test_double_protect() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("protect"),
        String::from("protect"),
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
        String::from("substitute"),
        String::from("tackle"),
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
fn test_substitute_does_not_let_secondary_status_effect_happen() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("substitute"),
        String::from("firepunch"),
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
fn test_secondary_on_self_works_against_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("substitute"),
        String::from("poweruppunch"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_move_goes_through_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("substitute"),
        String::from("boomburst"),
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
        String::from("substitute"),
        String::from("tackle"),
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
        .get_active()
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.get_active().substitute_health = 25;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("protect"),
        String::from("tackle"),
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
        .get_active()
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.get_active().substitute_health = 25;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("splash"),
        String::from("dragontail"),
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
fn test_whirlwind_move_against_substitute() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state
        .side_one
        .get_active()
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.get_active().substitute_health = 25;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("splash"),
        String::from("whirlwind"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
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

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("protect"),
        String::from("dragontail"),
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
    state.side_two.get_active().item = Items::ROCKY_HELMET;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("tackle"),
        String::from("splash"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rockyhelmet_does_not_overkill() {
    let mut state = State::default();
    state.side_one.get_active().speed = 150;
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().item = Items::ROCKY_HELMET;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("tackle"),
        String::from("splash"),
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
fn test_choiceband_locking() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::CHOICE_BAND;
    state.side_one.get_active().moves[0] = Move {
        id: "willowisp".to_string(),
        disabled: false,
        pp: 35,
        ..Default::default()
    };

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("willowisp"),
        String::from("splash"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_locked_moves_unlock_on_switchout() {
    let mut state = State::default();
    state.side_one.get_active().moves[1].disabled = true;
    state.side_one.get_active().moves[2].disabled = true;
    state.side_one.get_active().moves[3].disabled = true;

    state
        .side_two
        .get_active()
        .replace_move(0, String::from("splash"));

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(1),
        &MoveChoice::Move(0),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_fighting_move_with_blackbelt() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_one.get_active().item = Items::BLACK_BELT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("drainpunch"),
        String::from("splash"),
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
fn test_expert_belt_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_one.get_active().item = Items::EXPERT_BELT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("drainpunch"),
        String::from("splash"),
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
fn test_expert_belt_does_not_boost() {
    let mut state = State::default();
    state.side_two.get_active().hp = 300;
    state.side_two.get_active().maxhp = 300;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Dragon);
    state.side_one.get_active().item = Items::EXPERT_BELT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("drainpunch"),
        String::from("splash"),
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
fn test_lifeorb_boost_and_recoil() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::LIFE_ORB;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("tackle"),
        String::from("splash"),
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
fn test_shellbell_drain() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().item = Items::SHELL_BELL;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("tackle"),
        String::from("splash"),
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
    state.side_two.get_active().item = Items::ABSORB_BULB;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("watergun"),
        String::from("splash"),
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
                current_item: Items::ABSORB_BULB,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_ground_move_versus_airballoon() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::AIR_BALLOON;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("earthquake"),
        String::from("splash"),
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
    state.side_two.get_active().item = Items::AIR_BALLOON;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("tackle"),
        String::from("splash"),
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
                current_item: Items::AIR_BALLOON,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_assaultvest() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::ASSAULT_VEST;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("watergun"),
        String::from("splash"),
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
    state.side_two.get_active().item = Items::WEAKNESS_POLICY;
    state.side_two.get_active().hp = 200;
    state.side_two.get_active().maxhp = 200;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("watergun"),
        String::from("splash"),
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
                current_item: Items::WEAKNESS_POLICY,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_weaknesspolicy_does_not_overboost() {
    let mut state = State::default();
    state.side_two.get_active().item = Items::WEAKNESS_POLICY;
    state.side_two.get_active().hp = 200;
    state.side_two.get_active().maxhp = 200;
    state.side_two.get_active().attack_boost = 5;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("watergun"),
        String::from("splash"),
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
                current_item: Items::WEAKNESS_POLICY,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switching_in_with_grassyseed_in_grassy_terrain() {
    let mut state = State::default();
    state.side_two.pokemon[1].item = Items::GRASSY_SEED;
    state.terrain.terrain_type = Terrain::GrassyTerrain;
    state.terrain.turns_remaining = 3;

    state
        .side_one
        .get_active()
        .replace_move(0, String::from("splash"));

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(0),
        &MoveChoice::Switch(1),
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
                current_item: Items::GRASSY_SEED,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_contrary_with_seed() {
    let mut state = State::default();
    state.side_two.pokemon[1].item = Items::PSYCHIC_SEED;
    state.side_two.pokemon[1].ability = Abilities::CONTRARY;
    state.terrain.terrain_type = Terrain::PsychicTerrain;
    state.terrain.turns_remaining = 3;

    state
        .side_one
        .get_active()
        .replace_move(0, String::from("splash"));

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(0),
        &MoveChoice::Switch(1),
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
                current_item: Items::PSYCHIC_SEED,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_contrary() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::CONTRARY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("swordsdance"),
        String::from("splash"),
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
        String::from("poweruppunch"),
        String::from("splash"),
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
fn test_throatspray_with_move_that_can_miss() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::THROAT_SPRAY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("grasswhistle"),
        String::from("splash"),
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
                    current_item: Items::THROAT_SPRAY,
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
        String::from("tackle"),
        String::from("splash"),
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
fn test_poisonpoint_with_poisonjab() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::POISONPOINT;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("poisonjab"),
        String::from("splash"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_serenegrace_with_secondary() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::SERENEGRACE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("poisonjab"),
        String::from("splash"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_technician() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::TECHNICIAN;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("tackle"),
        String::from("splash"),
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
fn test_unseenfist() {
    let mut state = State::default();
    state.side_one.get_active().ability = Abilities::UNSEENFIST;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("tackle"),
        String::from("protect"),
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
        String::from("tackle"),
        String::from("splash"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_rattled() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::RATTLED;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("feintattack"),
        String::from("splash"),
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
        String::from("taunt"),
        String::from("splash"),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_explosion_into_damp() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::DAMP;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("explosion"),
        String::from("splash"),
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
        String::from("watergun"),
        String::from("splash"),
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
fn test_dryskin_does_not_overheal() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::DRYSKIN;
    state.side_two.get_active().hp = 90;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("watergun"),
        String::from("splash"),
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
    state.weather.turns_remaining = 5;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("splash"),
        String::from("splash"),
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
fn test_filter() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::FILTER;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("watergun"),
        String::from("splash"),
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
        String::from("tackle"),
        String::from("splash"),
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
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_flashfire() {
    let mut state = State::default();
    state.side_two.get_active().ability = Abilities::FLASHFIRE;
    state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Normal);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        String::from("ember"),
        String::from("splash"),
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
        String::from("aurorabeam"),
        String::from("splash"),
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
        String::from("ironhead"),
        String::from("splash"),
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
