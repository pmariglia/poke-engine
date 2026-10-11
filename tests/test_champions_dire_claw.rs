#![cfg(feature = "champions")]
//! Dire Claw in Champions: one 30% roll, then sample(psn, par, slp). Each status lands 10% of the time;
//! a sampled status the target is immune to is wasted (not redrawn), so it contributes 0% and the others
//! stay at 10%. Expected values are Showdown's (data/mods/champions/moves.ts).

use poke_engine::choices::Choices;
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::engine::state::MoveChoice;
use poke_engine::instruction::{Instruction, StateInstructions};
use poke_engine::state::{PokemonMoveIndex, PokemonStatus, PokemonType, SideReference, State};

fn status_probabilities(state: &mut State) -> [f32; 3] {
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::DIRECLAW);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);
    state.side_one.get_active().speed = 101; // strict order: Dire Claw first, no tie
    state.side_two.get_active().speed = 100;
    state.side_two.get_active().hp = 500;
    state.side_two.get_active().maxhp = 500;
    let branches: Vec<StateInstructions> = generate_instructions_from_move_pair(
        state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        false,
    );
    let mut p = [0.0f32; 3];
    for b in &branches {
        for i in &b.instruction_list {
            if let Instruction::ChangeStatus(c) = i {
                if c.side_ref == SideReference::SideTwo {
                    match c.new_status {
                        PokemonStatus::POISON => p[0] += b.percentage,
                        PokemonStatus::PARALYZE => p[1] += b.percentage,
                        PokemonStatus::SLEEP => p[2] += b.percentage,
                        _ => {}
                    }
                }
            }
        }
    }
    p
}

fn assert_close(expected: [f32; 3], actual: [f32; 3]) {
    for k in 0..3 {
        assert!(
            (expected[k] - actual[k]).abs() < 1e-3,
            "psn/par/slp expected {:?}, got {:?}",
            expected,
            actual
        );
    }
}

#[test]
fn test_dire_claw_each_status_ten_percent() {
    let mut s = State::default();
    s.side_two.get_active().types = (PokemonType::NORMAL, PokemonType::TYPELESS);
    assert_close([10.0, 10.0, 10.0], status_probabilities(&mut s));
}

#[test]
fn test_dire_claw_poison_type_target_wastes_the_poison_pick() {
    let mut s = State::default();
    s.side_two.get_active().types = (PokemonType::POISON, PokemonType::TYPELESS);
    assert_close([0.0, 10.0, 10.0], status_probabilities(&mut s));
}

#[test]
fn test_dire_claw_electric_type_target_wastes_the_paralysis_pick() {
    let mut s = State::default();
    s.side_two.get_active().types = (PokemonType::ELECTRIC, PokemonType::TYPELESS);
    assert_close([10.0, 0.0, 10.0], status_probabilities(&mut s));
}

#[test]
fn test_dire_claw_already_statused_target_gets_nothing() {
    let mut s = State::default();
    s.side_two.get_active().status = PokemonStatus::BURN;
    assert_close([0.0, 0.0, 0.0], status_probabilities(&mut s));
}
