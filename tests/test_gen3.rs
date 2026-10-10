#![cfg(feature = "gen3")]

use poke_engine::choices::{Choices, MOVES};
use poke_engine::engine::abilities::Abilities;
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::engine::items::Items;
use poke_engine::engine::state::{MoveChoice, PokemonVolatileStatus, Weather};
use poke_engine::instruction::ChangeSideConditionInstruction;
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, ChangeItemInstruction, ChangeStatusInstruction,
    ChangeVolatileStatusDurationInstruction, DamageInstruction, EnableMoveInstruction,
    HealInstruction, Instruction, RemoveVolatileStatusInstruction, SetSleepTurnsInstruction,
    StateInstructions, SwitchInstruction,
};
use poke_engine::state::PokemonSideCondition;
use poke_engine::state::{
    Move, PokemonIndex, PokemonMoveIndex, PokemonStatus, PokemonType, SideReference, State,
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
fn test_regular_move_with_protect_side_condition() {
    let mut state = State::default();
    state.side_one.side_conditions.protect = 1;

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
fn test_chestoberry_activates_when_being_put_to_sleep() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::CHESTOBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPORE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: SideReference::SideOne,
            current_item: Items::CHESTOBERRY,
            new_item: Items::NONE,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_chestoberry_activates_when_using_rest() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::CHESTOBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::REST,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
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
                heal_amount: 48,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::CHESTOBERRY,
                new_item: Items::NONE,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::SLEEP,
                new_status: PokemonStatus::NONE,
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
fn test_taunt_gets_applied_and_duration_increments_end_of_turn() {
    let mut state = State::default();
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TAUNT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::TAUNT,
            }),
            Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::TAUNT,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taunt_volatile_is_removed_end_of_turn_when_it_would_reach_2() {
    let mut state = State::default();
    state.side_one.volatile_status_durations.taunt = 1;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::TAUNT);
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
            }),
            Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taunt_re_enables_disabled_moves_when_being_removed() {
    let mut state = State::default();
    state.side_one.volatile_status_durations.taunt = 1;
    state.side_one.get_active().moves.m1.disabled = true;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::TAUNT);
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
            }),
            Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
                amount: -1,
            }),
            Instruction::EnableMove(EnableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switching_out_with_taunt_resets_duration_to_0() {
    let mut state = State::default();
    state.side_one.volatile_status_durations.taunt = 1;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::TAUNT);
    let vec_of_instructions = generate_instructions_with_state_assertion(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
                amount: -1,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
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
fn test_taunt_prevents_status_move() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].hp = 0;
    state.side_one.pokemon[PokemonIndex::P2].hp = 0;
    state.side_one.pokemon[PokemonIndex::P3].hp = 0;
    state.side_one.pokemon[PokemonIndex::P4].hp = 0;
    state.side_one.pokemon[PokemonIndex::P5].hp = 0;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::TAUNT);

    state.side_one.get_active().moves[&PokemonMoveIndex::M0] = Move {
        id: Choices::TOXIC,
        disabled: false,
        pp: 35,
        choice: MOVES.get(&Choices::TOXIC).unwrap().clone(),
    };

    state.side_one.get_active().moves[&PokemonMoveIndex::M1] = Move {
        id: Choices::TACKLE,
        disabled: false,
        pp: 35,
        choice: MOVES.get(&Choices::TACKLE).unwrap().clone(),
    };

    state.side_one.get_active().moves[&PokemonMoveIndex::M2] = Move {
        id: Choices::WATERGUN,
        disabled: false,
        pp: 35,
        choice: MOVES.get(&Choices::TACKLE).unwrap().clone(),
    };

    state.side_one.get_active().moves[&PokemonMoveIndex::M3] = Move {
        id: Choices::EMBER,
        disabled: false,
        pp: 35,
        choice: MOVES.get(&Choices::TACKLE).unwrap().clone(),
    };

    let (side_one_moves, _) = state.get_all_options();
    assert_eq!(
        vec![
            MoveChoice::Move(PokemonMoveIndex::M1),
            MoveChoice::Move(PokemonMoveIndex::M2),
            MoveChoice::Move(PokemonMoveIndex::M3),
        ],
        side_one_moves
    );
}

#[test]
fn test_rest_does_not_activate_when_fainted() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::CHESTOBERRY;
    state.side_one.get_active().hp = 1;

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
            percentage: 70.3125,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 46,
            })],
        },
        StateInstructions {
            percentage: 29.6875,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 50,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_fast_explosion_makes_other_side_unable_to_move() {
    let mut state = State::default();
    state.side_one.get_active().hp = 500;
    state.side_one.get_active().maxhp = 500;
    state.side_one.get_active().types = (PokemonType::STEEL, PokemonType::FLYING);
    state.side_one.get_active().speed = 45;
    state.side_two.get_active().speed = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPIKES,
        Choices::EXPLOSION,
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
                damage_amount: 292,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_end_of_turn_sand_kos_before_leftovers() {
    let mut state = State::default();
    state.weather.weather_type = Weather::SAND;
    state.weather.turns_remaining = -1;

    state.side_one.get_active().hp = 5;
    state.side_one.get_active().maxhp = 100;
    state.side_one.get_active().item = Items::LEFTOVERS;

    state.side_two.get_active().hp = 7;
    state.side_two.get_active().maxhp = 100;
    state.side_two.get_active().item = Items::LEFTOVERS;

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
                damage_amount: 6,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
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
fn test_intimidate_blocked_by_clearbody() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;
    state.side_two.get_active().ability = Abilities::CLEARBODY;

    let vec_of_instructions = generate_instructions_with_state_assertion(
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
fn test_same_speed_branch() {
    let mut state = State::default();
    state.side_one.get_active().speed = 100;
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().speed = 100;
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 50.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            })],
        },
        StateInstructions {
            percentage: 50.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_gen3_branch_when_a_roll_can_kill() {
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
    // The roll itself has a 25% chance of killing but the extra chance is accounting for a crit
    let expected_instructions = vec![
        StateInstructions {
            percentage: 70.3125,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 46,
            })],
        },
        StateInstructions {
            percentage: 29.6875,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 50,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

// Status-cure berries in gen 3 fire at END OF TURN (not before-move, unlike gens 4+). For
// statuses that cannot self-resolve mid-turn (burn/poison/toxic/paralysis) every branch
// therefore ends with the status cured and the berry consumed. (Freeze/sleep can resolve
// on their own before end of turn, in which case the berry is correctly NOT consumed, so
// they are not covered by this simple assertion.)
fn assert_gen3_status_cure_berry(berry: Items, status: PokemonStatus) {
    let mut state = State::default();
    state.side_one.get_active().item = berry;
    state.side_one.get_active().status = status;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    assert!(!vec_of_instructions.is_empty());
    for si in &vec_of_instructions {
        let mut branch = state.clone();
        branch.apply_instructions(&si.instruction_list);
        let active = branch.side_one.get_active_immutable();
        assert_eq!(
            PokemonStatus::NONE,
            active.status,
            "{:?} should have cured {:?} by end of turn",
            berry,
            status
        );
        assert_eq!(
            Items::NONE,
            active.item,
            "{:?} should have been consumed",
            berry
        );
    }
}

#[test]
fn test_rawstberry_cures_burn() {
    assert_gen3_status_cure_berry(Items::RAWSTBERRY, PokemonStatus::BURN);
}

#[test]
fn test_cheriberry_cures_paralysis() {
    assert_gen3_status_cure_berry(Items::CHERIBERRY, PokemonStatus::PARALYZE);
}

#[test]
fn test_pechaberry_cures_poison() {
    assert_gen3_status_cure_berry(Items::PECHABERRY, PokemonStatus::POISON);
}

#[test]
fn test_pechaberry_cures_toxic() {
    assert_gen3_status_cure_berry(Items::PECHABERRY, PokemonStatus::TOXIC);
}

#[test]
fn test_seismictoss_against_sub_breaking() {
    let mut state = State::default();
    state.side_one.get_active().level = 88;
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::SUBSTITUTE);
    state.side_two.substitute_health = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SEISMICTOSS,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 50,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::SUBSTITUTE,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_seismictoss_versus_ghost() {
    let mut state = State::default();
    state.side_two.get_active().types.0 = PokemonType::GHOST;

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
fn test_nightshade() {
    let mut state = State::default();
    state.side_one.get_active().level = 44;
    state.side_two.get_active().types.0 = PokemonType::FIRE;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::NIGHTSHADE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 44,
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

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![],
        },
        StateInstructions {
            percentage: 90.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_endeavor() {
    let mut state = State::default();
    state.side_one.get_active().hp = 30;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::ENDEAVOR,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 70,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_endeavor_when_attacker_hp_is_higher() {
    let mut state = State::default();
    state.side_two.get_active().hp = 30;

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
