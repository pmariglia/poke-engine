#![cfg(feature = "gen2")]

use poke_engine::choices::{Choices, MoveCategory};
use poke_engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeStatusInstruction, DamageInstruction, DecrementRestTurnsInstruction, HealInstruction,
    Instruction, RemoveVolatileStatusInstruction, SetDamageDealtSideTwoInstruction,
    SetSleepTurnsInstruction, StateInstructions, SwitchInstruction,
};
use poke_engine::items::Items;
use poke_engine::state::{
    MoveChoice, PokemonBoostableStat, PokemonIndex, PokemonMoveIndex, PokemonStatus, PokemonType,
    PokemonVolatileStatus, SideReference, State,
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
fn test_branch_when_a_roll_can_kill() {
    let mut state = State::default();
    state.side_two.get_active().hp = 50;

    let move_one = Choices::TACKLE;
    let move_two = Choices::SPLASH;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_two);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );

    // This damage roll is 44-52, so it can kill
    // Normally without considering the roll, the damage is 48 (0.925 * 52)
    let expected_instructions = vec![
        StateInstructions {
            percentage: 70.01953,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 46,
            })],
        },
        StateInstructions {
            percentage: 29.980469,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 50,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_branch_on_crit() {
    let mut state = State::default();
    state.side_two.get_active().hp = 100;

    let move_one = Choices::TACKLE;
    let move_two = Choices::SPLASH;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_two);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 93.359375,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 6.640625,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 97,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_highcrit_move() {
    let mut state = State::default();
    state.side_two.get_active().hp = 100;

    let move_one = Choices::RAZORLEAF;
    let move_two = Choices::SPLASH;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_two);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 5.000001,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 83.125,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 44,
            })],
        },
        StateInstructions {
            percentage: 11.875,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 88,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_crit_does_not_overkill() {
    let mut state = State::default();
    state.side_two.get_active().hp = 85;

    let move_one = Choices::TACKLE;
    let move_two = Choices::SPLASH;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_two);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 93.359375,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 6.640625,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 85,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_branch_when_a_roll_can_kill_on_the_low_side() {
    let mut state = State::default();
    state.side_two.get_active().hp = 45;

    let move_one = Choices::TACKLE;
    let move_two = Choices::SPLASH;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_two);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );

    // This damage roll is 44-52, so it can kill
    // Normally without considering the roll, the damage is 48 (0.925 * 52)
    // The roll has an 87.5% chance of killing, and a 6.64% chance of landing a crit,
    // totalling a 94.14% chance of killing
    let expected_instructions = vec![
        StateInstructions {
            percentage: 11.669922,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 44,
            })],
        },
        StateInstructions {
            percentage: 88.33008,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 45,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_min_damage_killing_does_not_branch() {
    let mut state = State::default();
    state.side_two.get_active().hp = 44;

    let move_one = Choices::TACKLE;
    let move_two = Choices::SPLASH;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_two);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.00,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 44,
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
fn test_sleeptalk_rest_has_no_effect_at_full_hp() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::SLEEP;
    state.side_one.get_active().rest_turns = 3;
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);
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
    let vec_of_instructions = generate_instructions_with_state_assertion(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![Instruction::DecrementRestTurns(
                DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                },
            )],
        },
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
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
                Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::CURSE,
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
fn test_guaranteed_to_stay_asleep_sleeptalk_move_when_not_rested() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SLEEPTALK);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::REST);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M2, Choices::TACKLE);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M3, Choices::CURSE);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SLEEPTALK,
        Choices::SPORE,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::SLEEP,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 1,
                    previous_turns: 0,
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
        },
        StateInstructions {
            percentage: 33.333336,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::SLEEP,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 1,
                    previous_turns: 0,
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
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::SLEEP,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 1,
                    previous_turns: 0,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::CURSE,
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
fn test_small_chance_to_awaken_sleeptalk_move_when_not_rested() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().status = PokemonStatus::SLEEP;
    state.side_one.get_active().sleep_turns = 1;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SLEEPTALK);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::REST);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M2, Choices::TACKLE);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M3, Choices::CURSE);

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SLEEPTALK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 16.666668,
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
                    previous_turns: 1,
                }),
            ],
        },
        StateInstructions {
            percentage: 27.777777,
            instruction_list: vec![
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 2,
                    previous_turns: 1,
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
        },
        StateInstructions {
            percentage: 27.777777,
            instruction_list: vec![
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 2,
                    previous_turns: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
            ],
        },
        StateInstructions {
            percentage: 27.777777,
            instruction_list: vec![
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 2,
                    previous_turns: 1,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::CURSE,
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
fn test_sleeptalk_can_call_rest() {
    let mut state = State::default();
    state.side_one.get_active().status = PokemonStatus::SLEEP;
    state.side_one.get_active().rest_turns = 2;
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
    let vec_of_instructions = generate_instructions_with_state_assertion(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

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
                Instruction::SetRestTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 3,
                    previous_turns: 1,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 48,
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
                    volatile_status: PokemonVolatileStatus::CURSE,
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
fn test_mintberry_cures_rest() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().item = Items::MINTBERRY;

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
                old_status: PokemonStatus::SLEEP,
                new_status: PokemonStatus::NONE,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::MINTBERRY,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_miracleberry_cures_rest() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().item = Items::MIRACLEBERRY;

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
                old_status: PokemonStatus::SLEEP,
                new_status: PokemonStatus::NONE,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::MIRACLEBERRY,
                new_item: Items::NONE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_miracleberry_cures_paralysis_and_attack_does_not_branch() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().item = Items::MIRACLEBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::THUNDERWAVE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::PARALYZE,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::PARALYZE,
                new_status: PokemonStatus::NONE,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::MIRACLEBERRY,
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
fn test_bellydrum() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BELLYDRUM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 50,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_bellydrum_with_negative_prior_boost() {
    let mut state = State::default();
    state.side_one.attack_boost = -1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BELLYDRUM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 50,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 7,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_bellydrum_below_50_percent_boosts_by_2_bug() {
    let mut state = State::default();
    state.side_one.get_active().hp = 49;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BELLYDRUM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideOne,
            stat: PokemonBoostableStat::Attack,
            amount: 2,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_bellydrum_below_50_percent_boosts_by_2_bug_does_not_overboost() {
    let mut state = State::default();
    state.side_one.get_active().hp = 49;
    state.side_one.attack_boost = 5;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BELLYDRUM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideOne,
            stat: PokemonBoostableStat::Attack,
            amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_bellydrum_at_75_percent() {
    let mut state = State::default();
    state.side_one.get_active().hp = 75;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BELLYDRUM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 50,
            }),
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_bellydrum_at_exactly_50_percent() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::BELLYDRUM,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideOne,
            stat: PokemonBoostableStat::Attack,
            amount: 2,
        })],
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
fn test_counter_cannot_hit_ghost_type() {
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
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_counter_reflects_special_hiddenpower() {
    let mut state = State::default();
    state.use_damage_dealt = true;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::COUNTER);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::HIDDENPOWERWATER70);

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
                damage_amount: 55,
            }),
            Instruction::SetDamageDealtSideTwo(SetDamageDealtSideTwoInstruction {
                damage_change: 55,
                move_category: MoveCategory::Special,
                previous_move_category: MoveCategory::Physical,
                toggle_hit_substitute: false,
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
fn test_mirrorcoat_does_not_reflect_special_hiddenpower() {
    let mut state = State::default();
    state.use_damage_dealt = true;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::MIRRORCOAT);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::HIDDENPOWERWATER70);

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
                damage_amount: 55,
            }),
            Instruction::SetDamageDealtSideTwo(SetDamageDealtSideTwoInstruction {
                damage_change: 55,
                move_category: MoveCategory::Special,
                previous_move_category: MoveCategory::Physical,
                toggle_hit_substitute: false,
            }),
        ],
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
fn test_mint_berry_does_not_cure_paralysis() {
    let mut state = State::default();
    state.side_one.get_active().hp = 50;
    state.side_one.get_active().item = Items::MINTBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::THUNDERWAVE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: SideReference::SideOne,
            pokemon_index: PokemonIndex::P0,
            old_status: PokemonStatus::NONE,
            new_status: PokemonStatus::PARALYZE,
        })],
    }];
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
