#![cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
//! At a speed tie both move orders must be resolved from the SAME, unmodified choices.
//!
//! generate_instructions_from_move_pair resolves a tie by running "side one first" and then "side two
//! first". handle_both_moves mutates its choices in place (before_move's modify_choice / ability / item
//! changes on the first mover; `first_move = false` on the second), so if the first branch is not given
//! copies, the second branch inherits its mutations: side one's modifications are applied twice and side
//! two is treated as having moved second even when it moves first. Each test compares the tie against the
//! same matchup with a strict speed order, which has no second branch to contaminate.

use poke_engine::choices::Choices;
use poke_engine::engine::abilities::Abilities;
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::engine::state::{MoveChoice, PokemonVolatileStatus};
use poke_engine::instruction::{Instruction, StateInstructions};
use poke_engine::state::{PokemonMoveIndex, SideReference, State};

fn run(state: &mut State, m1: Choices, m2: Choices) -> Vec<StateInstructions> {
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, m1);
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, m2);
    let before = format!("{:?}", state);
    let out = generate_instructions_from_move_pair(
        state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        false,
    );
    assert_eq!(before, format!("{:?}", state));
    out
}

fn bulky(speed_one: i16, speed_two: i16) -> State {
    let mut state = State::default();
    for side in [&mut state.side_one, &mut state.side_two] {
        let p = side.get_active();
        p.hp = 500;
        p.maxhp = 500;
    }
    state.side_one.get_active().speed = speed_one;
    state.side_two.get_active().speed = speed_two;
    state
}

fn damage_to(si: &StateInstructions, side: SideReference) -> Vec<i16> {
    si.instruction_list
        .iter()
        .filter_map(|i| match i {
            Instruction::Damage(d) if d.side_ref == side => Some(d.damage_amount),
            _ => None,
        })
        .collect()
}

fn side_one_moved_first(si: &StateInstructions) -> bool {
    matches!(si.instruction_list.first(), Some(Instruction::Damage(d)) if d.side_ref == SideReference::SideTwo)
}

#[test]
fn test_speed_tie_does_not_reapply_side_one_ability_modifier() {
    // Tough Claws (x1.3 contact) on side one. With a strict order there is one value for its damage;
    // at a tie, BOTH orders must deal exactly that value - the bug dealt x1.3 twice when side two moved first.
    let mut reference = bulky(100, 101);
    reference.side_one.get_active().ability = Abilities::TOUGHCLAWS;
    let expected = damage_to(
        &run(&mut reference, Choices::TACKLE, Choices::SPLASH)[0],
        SideReference::SideTwo,
    );

    let mut tie = bulky(100, 100);
    tie.side_one.get_active().ability = Abilities::TOUGHCLAWS;
    let branches = run(&mut tie, Choices::TACKLE, Choices::SPLASH);
    assert!(branches.len() >= 2, "a tie must produce both orders");
    for b in &branches {
        assert_eq!(
            expected,
            damage_to(b, SideReference::SideTwo),
            "order changed side one's damage: {:?}",
            b
        );
    }
}

#[test]
fn test_speed_tie_sucker_punch_mirror_lands_in_both_orders() {
    // Whoever moves first lands Sucker Punch; the reply fails. The bug left side two's Sucker Punch
    // failing even when side two moved first, so half the tie dealt no damage at all.
    let mut tie = bulky(100, 100);
    let branches = run(&mut tie, Choices::SUCKERPUNCH, Choices::SUCKERPUNCH);
    let mut p_side_one_hit = 0.0;
    let mut p_side_two_hit = 0.0;
    for b in &branches {
        let to_two = damage_to(b, SideReference::SideTwo).len();
        let to_one = damage_to(b, SideReference::SideOne).len();
        assert_eq!(
            1,
            to_one + to_two,
            "exactly one Sucker Punch must land: {:?}",
            b
        );
        if to_two == 1 {
            p_side_one_hit += b.percentage
        } else {
            p_side_two_hit += b.percentage
        }
    }
    assert!(
        (p_side_one_hit - 50.0).abs() < 1e-3,
        "side one lands {}%",
        p_side_one_hit
    );
    assert!(
        (p_side_two_hit - 50.0).abs() < 1e-3,
        "side two lands {}%",
        p_side_two_hit
    );
}

#[test]
fn test_speed_tie_side_two_can_flinch_when_it_moves_first() {
    // Flinch requires the user to move first. The bug carried first_move = false into the branch where
    // side two moves first, so side two could never flinch side one at a tie.
    let mut tie = bulky(100, 100);
    let branches = run(&mut tie, Choices::IRONHEAD, Choices::IRONHEAD);
    let flinched = |b: &StateInstructions, side: SideReference| {
        b.instruction_list.iter().any(|i| matches!(i,
            Instruction::ApplyVolatileStatus(v) if v.side_ref == side && v.volatile_status == PokemonVolatileStatus::FLINCH))
    };
    let p_one_flinches_two: f32 = branches
        .iter()
        .filter(|b| flinched(b, SideReference::SideTwo))
        .map(|b| b.percentage)
        .sum();
    let p_two_flinches_one: f32 = branches
        .iter()
        .filter(|b| flinched(b, SideReference::SideOne))
        .map(|b| b.percentage)
        .sum();
    assert!(
        p_one_flinches_two > 0.0,
        "control: side one flinches side two when it moves first"
    );
    assert!(
        (p_one_flinches_two - p_two_flinches_one).abs() < 1e-3,
        "a mirror tie must be symmetric: side one flinches {}%, side two {}%",
        p_one_flinches_two,
        p_two_flinches_one
    );
}

#[test]
fn test_control_strict_order_unaffected() {
    // No tie: one order, nothing to contaminate - must give the same damage as before the fix.
    let mut s = bulky(101, 100);
    s.side_one.get_active().ability = Abilities::TOUGHCLAWS;
    let branches = run(&mut s, Choices::TACKLE, Choices::SPLASH);
    assert!(branches.iter().all(side_one_moved_first));
}
