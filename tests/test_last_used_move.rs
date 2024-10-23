use poke_engine::abilities::Abilities;
use poke_engine::choices::Choices;
use poke_engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, DamageInstruction, Instruction,
    RemoveVolatileStatusInstruction, SetLastUsedMoveInstruction, SetSubstituteHealthInstruction,
    StateInstructions, SwitchInstruction,
};
use poke_engine::state::{
    LastUsedMove, MoveChoice, PokemonBoostableStat, PokemonIndex, PokemonMoveIndex,
    PokemonVolatileStatus, SideReference, State,
};

#[test]
fn test_last_used_move_is_set_on_switch() {
    let mut state = State::default();
    state.use_last_used_move = true;
    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Switch(PokemonIndex::P1),
                previous_last_used_move: LastUsedMove::None,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_last_used_move_is_set_on_move() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideTwo,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
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
fn test_last_used_move_overwritten_when_dragged_out() {
    let mut state = State::default();
    state.use_last_used_move = true;

    // Only one drag option available to keep test simple
    state.side_one.pokemon.p2.hp = 0;
    state.side_one.pokemon.p3.hp = 0;
    state.side_one.pokemon.p4.hp = 0;
    state.side_one.pokemon.p5.hp = 0;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::WHIRLWIND);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideTwo,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Switch(PokemonIndex::P1),
                previous_last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_encore_causes_get_all_options_to_only_allow_last_used_move() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Encore);
    state.side_one.last_used_move = LastUsedMove::Move(PokemonMoveIndex::M0);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);

    let (side_one_moves, _side_two_moves) = state.get_all_options();

    assert_eq!(
        vec![
            MoveChoice::Move(PokemonMoveIndex::M0),
            MoveChoice::Switch(PokemonIndex::P1),
            MoveChoice::Switch(PokemonIndex::P2),
            MoveChoice::Switch(PokemonIndex::P3),
            MoveChoice::Switch(PokemonIndex::P4),
            MoveChoice::Switch(PokemonIndex::P5),
        ],
        side_one_moves
    );
}

#[test]
fn test_encore_and_arenatrapped_together() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state.side_two.get_active().ability = Abilities::ARENATRAP;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Encore);
    state.side_one.last_used_move = LastUsedMove::Move(PokemonMoveIndex::M0);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);

    let (side_one_moves, _side_two_moves) = state.get_all_options();

    assert_eq!(
        vec![MoveChoice::Move(PokemonMoveIndex::M0),],
        side_one_moves
    );
}

#[test]
fn test_encore_slow() {
    let mut state = State::default();
    state.use_last_used_move = true;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::ENCORE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::WATERGUN);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideTwo,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Encore,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);

    // now, apply those instructions and look for the options for next turn
    // ensure that M1 watergun is not an option
    state.apply_instructions(&vec_of_instructions[0].instruction_list);
    let (_side_one_moves, side_two_moves) = state.get_all_options();
    assert_eq!(
        vec![
            MoveChoice::Move(PokemonMoveIndex::M0),
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
fn test_encore_slow_into_substitute() {
    let mut state = State::default();
    state.use_last_used_move = true;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::ENCORE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SUBSTITUTE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::WATERGUN);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideTwo,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
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
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Encore,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);

    // now, apply those instructions and look for the options for next turn
    // ensure that M1 watergun is not an option
    state.apply_instructions(&vec_of_instructions[0].instruction_list);
    let (_side_one_moves, side_two_moves) = state.get_all_options();
    assert_eq!(
        vec![
            MoveChoice::Move(PokemonMoveIndex::M0),
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
fn test_encore_fast_fails_with_lastusedmove_equal_to_switch() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state.side_one.get_active().speed = 200;
    state.side_two.get_active().speed = 100;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::ENCORE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::WATERGUN);
    state.side_two.last_used_move = LastUsedMove::Switch(PokemonIndex::P0);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideTwo,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::Switch(PokemonIndex::P0),
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
fn test_encore_fast_fails_with_lastusedmove_equal_to_none() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state.side_one.get_active().speed = 200;
    state.side_two.get_active().speed = 100;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::ENCORE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::WATERGUN);
    state.side_two.last_used_move = LastUsedMove::None;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideTwo,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
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
fn test_encore_second_fails_when_opponent_switches() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state.side_one.get_active().speed = 200;
    state.side_two.pokemon[PokemonIndex::P1].speed = 100;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::ENCORE);
    state.side_two.last_used_move = LastUsedMove::Move(PokemonMoveIndex::M0);

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
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideTwo,
                last_used_move: LastUsedMove::Switch(PokemonIndex::P1),
                previous_last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
            }),
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_fast_encore_into_using_a_different_move_from_lum() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state.side_one.get_active().speed = 200;
    state.side_two.get_active().speed = 100;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::ENCORE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::SWORDSDANCE);
    state.side_two.last_used_move = LastUsedMove::Move(PokemonMoveIndex::M1);

    // side_two will try to use tackle, but will encored into watergun from last turn
    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Encore,
            }),
            // no setting last used move for s2 because it didn't change
            Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: 2,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_fakeout_first_turn_switched_in() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state.side_one.get_active().speed = 200;
    state.side_two.get_active().speed = 100;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::FAKEOUT);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);

    state.side_one.last_used_move = LastUsedMove::Switch(PokemonIndex::P0);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideOne,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::Switch(PokemonIndex::P0),
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Flinch,
            }),
            // no setting last used move for s2 because it flinched and didnt get to use a move
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::Flinch,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}
#[test]
fn test_fakeout_with_last_used_move_of_non_switch() {
    let mut state = State::default();
    state.use_last_used_move = true;
    state.side_one.get_active().speed = 200;
    state.side_two.get_active().speed = 100;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::FAKEOUT);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);

    state.side_one.last_used_move = LastUsedMove::Move(PokemonMoveIndex::M0);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
                side_ref: SideReference::SideTwo,
                last_used_move: LastUsedMove::Move(PokemonMoveIndex::M0),
                previous_last_used_move: LastUsedMove::None,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}
