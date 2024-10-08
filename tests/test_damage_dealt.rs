#![cfg(feature = "damage_dealt")]

use poke_engine::choices::{Choices, MoveCategory};
use poke_engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::instruction::{
    DamageInstruction, Instruction, RemoveVolatileStatusInstruction, SetDamageDealtInstruction,
    StateInstructions,
};
use poke_engine::state::{
    MoveChoice, PokemonMoveIndex, PokemonVolatileStatus, SideReference, State, Weather,
};

#[test]
fn test_counter_after_physical_hit() {
    let mut state = State::default();

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
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 48,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 96,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_metalburst_after_physical_move() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::METALBURST);
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
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 48,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 72,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_metalburst_after_special_move() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::METALBURST);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::WATERGUN);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 32,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 32,
                previous_damage: 0,
                move_category: MoveCategory::Special,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
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
fn test_metalburst_after_substitute_being_hit() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::METALBURST);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 5;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 5,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: true,
                previous_hit_substitute: false,
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
fn test_metalburst_fails_moving_first() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::METALBURST);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
    state.side_one.get_active().speed = 100;
    state.side_two.get_active().speed = 50;

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 48,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_metalburst_after_status_move() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::METALBURST);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_counter_after_special_hit() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::COUNTER);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::WATERGUN);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 32,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 32,
                previous_damage: 0,
                move_category: MoveCategory::Special,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_mirrorcoat_after_special_hit() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::MIRRORCOAT);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::WATERGUN);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 32,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 32,
                previous_damage: 0,
                move_category: MoveCategory::Special,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
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
fn test_mirrorcoat_after_physical_hit() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::MIRRORCOAT);
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
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 48,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_focuspunch_after_getting_hit() {
    let mut state = State::default();
    state.weather.weather_type = Weather::Sun;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::FOCUSPUNCH);
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
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 48,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_focuspunch_after_substitute_getting_hit() {
    let mut state = State::default();
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::Substitute);
    state.side_one.substitute_health = 1;

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::FOCUSPUNCH);
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
            Instruction::DamageSubstitute(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideTwo,
                damage: 1,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: true,
                previous_hit_substitute: false,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Substitute,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 100,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideOne,
                damage: 100,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_focuspunch_after_status_move() {
    let mut state = State::default();

    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::FOCUSPUNCH);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 100,
            }),
            Instruction::SetDamageDealt(SetDamageDealtInstruction {
                side_ref: SideReference::SideOne,
                damage: 100,
                previous_damage: 0,
                move_category: MoveCategory::Physical,
                previous_move_category: MoveCategory::Physical,
                hit_substitute: false,
                previous_hit_substitute: false,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}
