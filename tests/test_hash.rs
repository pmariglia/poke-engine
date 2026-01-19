use poke_engine::instruction::{
    Instruction, StateInstructions, SwitchInstruction, ToggleTerastallizedInstruction,
};
use poke_engine::state::{PokemonIndex, SideReference, State};

fn get_starting_state_hash() -> u64 {
    2787307573912940442
}

fn state_with_default_hash() -> State {
    let mut state = State::default();
    state.hash.set_hash(get_starting_state_hash());
    state
}

fn verify_hashing(state: &mut State, instructions: &Vec<Instruction>) {
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

#[test]
fn test_switch_hash() {
    let mut state = state_with_default_hash();
    let mut state_instructions = StateInstructions::default();
    state_instructions.instruction_list = vec![Instruction::Switch(SwitchInstruction {
        side_ref: SideReference::SideOne,
        previous_index: PokemonIndex::P0,
        next_index: PokemonIndex::P1,
    })];
    verify_hashing(&mut state, &state_instructions.instruction_list);
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
    verify_hashing(&mut state, &state_instructions.instruction_list);
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
    verify_hashing(&mut state, &state_instructions.instruction_list);
}
