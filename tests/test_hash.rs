use poke_engine::engine::state::PokemonVolatileStatus;
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeSideConditionInstruction,
    ChangeStatusInstruction, ChangeVolatileStatusDurationInstruction, DamageInstruction,
    HealInstruction, Instruction, RemoveVolatileStatusInstruction, StateInstructions,
    SwitchInstruction, ToggleTerastallizedInstruction,
};
use poke_engine::state::PokemonSideCondition;
use poke_engine::state::{PokemonBoostableStat, PokemonIndex, PokemonStatus, SideReference, State};

fn get_starting_state_hash() -> u64 {
    2787307573912940442
}

fn state_with_default_hash() -> State {
    let mut state = State::default();
    state.hash.set_hash(get_starting_state_hash());
    state
}

fn assert_instructions_modify_hash(state: &mut State, instructions: &Vec<Instruction>) {
    let initial_hash = state.hash.get_hash();
    state.apply_instructions_with_hash(instructions);
    let modified_hash = state.hash.get_hash();
    assert_ne!(
        initial_hash, modified_hash,
        "Hash should change after applying instructions"
    );
    state.reverse_instructions_with_hash(instructions);
    let reverted_hash = state.hash.get_hash();
    assert_eq!(
        initial_hash, reverted_hash,
        "Hash should revert back after reversing instructions"
    );
}

fn assert_instructions_keep_hash_the_same(state: &mut State, instructions: &Vec<Instruction>) {
    let initial_hash = state.hash.get_hash();
    state.apply_instructions_with_hash(instructions);
    let modified_hash = state.hash.get_hash();
    assert_eq!(
        initial_hash, modified_hash,
        "Hash should revert back after applying and reversing instructions that reset state"
    );
    state.reverse_instructions_with_hash(instructions);
    let reverted_hash = state.hash.get_hash();
    assert_eq!(
        initial_hash, reverted_hash,
        "Hash should revert back after reversing instructions"
    );
}

#[test]
fn test_switch_hash() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![Instruction::Switch(SwitchInstruction {
        side_ref: SideReference::SideOne,
        previous_index: PokemonIndex::P0,
        next_index: PokemonIndex::P1,
    })];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_toggling_terastallization() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![Instruction::ToggleTerastallized(
        ToggleTerastallizedInstruction {
            side_ref: SideReference::SideOne,
        },
    )];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_switching_and_terastallizing_together() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
        Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        }),
        Instruction::ToggleTerastallized(ToggleTerastallizedInstruction {
            side_ref: SideReference::SideTwo,
        }),
    ];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_taking_damage() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
        side_ref: SideReference::SideOne,
        damage_amount: 50,
    })];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_both_sides_taking_damage() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
        Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 50,
        }),
        Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 45,
        }),
        Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideTwo,
            damage_amount: 6,
        }),
        Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 6,
        }),
    ];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_damage_and_heal_return_hash_to_original() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();

    // damage of 50 and heal of 50 on the same side, hashes should match
    state_instructions.instruction_list = vec![
        Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 50,
        }),
        Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideOne,
            heal_amount: 50,
        }),
    ];
    let initial_hash = state.hash.get_hash();
    state.apply_instructions_with_hash(&state_instructions.instruction_list);
    let modified_hash = state.hash.get_hash();
    assert_eq!(
        initial_hash, modified_hash,
        "Damage and Heal should result in the same hash"
    );
    state.reverse_instructions_with_hash(&state_instructions.instruction_list);
    let reverted_hash = state.hash.get_hash();
    assert_eq!(
        initial_hash, reverted_hash,
        "Hash should revert back after reversing instructions"
    );
}

#[test]
fn test_damage_and_heal_on_different_sides_does_not_change_hash() {
    let mut state = state_with_default_hash();
    state.side_two.get_active().hp = 25;
    let mut state_instructions = StateInstructions::default();

    // damage of 50 and heal of 50 on different sides, hashes should not match
    state_instructions.instruction_list = vec![
        Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 50,
        }),
        Instruction::Heal(HealInstruction {
            side_ref: SideReference::SideTwo,
            heal_amount: 50,
        }),
    ];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_volatile_status_change() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![Instruction::ApplyVolatileStatus(
        ApplyVolatileStatusInstruction {
            side_ref: SideReference::SideOne,
            volatile_status: PokemonVolatileStatus::LEECHSEED,
        },
    )];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_acquiring_volatile_switching_twice_resets_state() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
        Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
            side_ref: SideReference::SideOne,
            volatile_status: PokemonVolatileStatus::LEECHSEED,
        }),
        Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
            side_ref: SideReference::SideOne,
            volatile_status: PokemonVolatileStatus::LEECHSEED,
        }),
        Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        }),
        Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P1,
            next_index: PokemonIndex::P0,
        }),
    ];
    assert_instructions_keep_hash_the_same(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_status_change() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list =
        vec![Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: SideReference::SideOne,
            pokemon_index: PokemonIndex::P0,
            old_status: PokemonStatus::NONE,
            new_status: PokemonStatus::PARALYZE,
        })];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_status_change_and_reverse() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
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
    ];
    assert_instructions_keep_hash_the_same(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_boost_change() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![Instruction::Boost(BoostInstruction {
        side_ref: SideReference::SideOne,
        stat: PokemonBoostableStat::Attack,
        amount: 1,
    })];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_boost_change_keeps_the_same() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
        Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideOne,
            stat: PokemonBoostableStat::Attack,
            amount: 1,
        }),
        Instruction::Boost(BoostInstruction {
            side_ref: SideReference::SideOne,
            stat: PokemonBoostableStat::Attack,
            amount: -1,
        }),
    ];
    assert_instructions_keep_hash_the_same(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_side_condition() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
        Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
            side_ref: SideReference::SideOne,
            amount: 1,
            side_condition: PokemonSideCondition::AuroraVeil,
        }),
        Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
            side_ref: SideReference::SideTwo,
            amount: 1,
            side_condition: PokemonSideCondition::Tailwind,
        }),
    ];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_side_condition_instructions_that_nullify_each_other() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
        Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
            side_ref: SideReference::SideOne,
            amount: 1,
            side_condition: PokemonSideCondition::AuroraVeil,
        }),
        Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
            side_ref: SideReference::SideTwo,
            amount: 1,
            side_condition: PokemonSideCondition::Tailwind,
        }),
        Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
            side_ref: SideReference::SideOne,
            amount: -1,
            side_condition: PokemonSideCondition::AuroraVeil,
        }),
        Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
            side_ref: SideReference::SideTwo,
            amount: -1,
            side_condition: PokemonSideCondition::Tailwind,
        }),
    ];
    assert_instructions_keep_hash_the_same(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_volatile_duration() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
        Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
            side_ref: SideReference::SideOne,
            amount: 1,
            volatile_status: PokemonVolatileStatus::YAWN,
        }),
        Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
            side_ref: SideReference::SideTwo,
            amount: 1,
            volatile_status: PokemonVolatileStatus::TAUNT,
        }),
    ];
    assert_instructions_modify_hash(&mut state, &state_instructions.instruction_list);
}

#[test]
fn test_volatile_durations_that_nullify_each_other() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![
        Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
            side_ref: SideReference::SideOne,
            amount: 1,
            volatile_status: PokemonVolatileStatus::YAWN,
        }),
        Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
            side_ref: SideReference::SideTwo,
            amount: 1,
            volatile_status: PokemonVolatileStatus::TAUNT,
        }),
        Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
            side_ref: SideReference::SideOne,
            amount: -1,
            volatile_status: PokemonVolatileStatus::YAWN,
        }),
        Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
            side_ref: SideReference::SideTwo,
            amount: -1,
            volatile_status: PokemonVolatileStatus::TAUNT,
        }),
    ];
    assert_instructions_keep_hash_the_same(&mut state, &state_instructions.instruction_list);
}
