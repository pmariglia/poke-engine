#![cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]

use poke_engine::choices::Choices;
use poke_engine::engine::abilities::Abilities;
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::engine::state::{MoveChoice, PokemonVolatileStatus};
use poke_engine::state::{PokemonMoveIndex, PokemonType, State};

fn position(attack: Choices) -> State {
    let mut state = State::default();
    let own = state.side_one.get_active();
    own.hp = 55;
    own.maxhp = 200;
    own.level = 86;
    own.speed = 200;
    own.replace_move(PokemonMoveIndex::M0, attack);
    let foe = state.side_two.get_active();
    foe.hp = 150;
    foe.maxhp = 200;
    foe.speed = 100;
    foe.types = (PokemonType::WATER, PokemonType::TYPELESS);
    foe.replace_move(PokemonMoveIndex::M0, Choices::SPLASH);
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::SUBSTITUTE);
    state.side_two.substitute_health = 50;
    state
}

#[test]
fn fixed_damage_breaks_substitute_without_spilling_into_hp() {
    for attack in [
        Choices::SEISMICTOSS,
        Choices::NIGHTSHADE,
        Choices::SUPERFANG,
        Choices::ENDEAVOR,
        Choices::FINALGAMBIT,
    ] {
        let mut state = position(attack);
        let original = format!("{:?}", state);
        let branches = generate_instructions_from_move_pair(
            &mut state,
            &MoveChoice::Move(PokemonMoveIndex::M0),
            &MoveChoice::Move(PokemonMoveIndex::M0),
            true,
        );
        assert_eq!(format!("{:?}", state), original);
        for branch in branches {
            state.apply_instructions(&branch.instruction_list);
            assert_eq!(
                state.side_two.get_active_immutable().hp,
                150,
                "{:?}",
                attack
            );
            let hit = !state
                .side_two
                .volatile_statuses
                .contains(&PokemonVolatileStatus::SUBSTITUTE);
            assert_eq!(state.side_two.substitute_health, if hit { 0 } else { 50 });
            assert_eq!(
                state.side_one.get_active_immutable().hp,
                if hit && attack == Choices::FINALGAMBIT {
                    0
                } else {
                    55
                }
            );
            state.reverse_instructions(&branch.instruction_list);
            assert_eq!(format!("{:?}", state), original);
        }
    }
}

#[test]
fn infiltrator_bypasses_substitute_for_fixed_damage() {
    let mut state = position(Choices::NIGHTSHADE);
    state.side_one.get_active().ability = Abilities::INFILTRATOR;
    let branches = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );
    for branch in branches {
        state.apply_instructions(&branch.instruction_list);
        assert_eq!(state.side_two.get_active_immutable().hp, 64);
        assert_eq!(state.side_two.substitute_health, 50);
        state.reverse_instructions(&branch.instruction_list);
    }
}

#[test]
fn super_fang_uses_floor_half_hp_and_can_ko_one_hp() {
    for hp in [1, 101] {
        let mut state = position(Choices::SUPERFANG);
        state
            .side_two
            .volatile_statuses
            .remove(&PokemonVolatileStatus::SUBSTITUTE);
        state.side_two.get_active().hp = hp;
        let branches = generate_instructions_from_move_pair(
            &mut state,
            &MoveChoice::Move(PokemonMoveIndex::M0),
            &MoveChoice::Move(PokemonMoveIndex::M0),
            true,
        );
        for branch in branches {
            state.apply_instructions(&branch.instruction_list);
            let remaining = state.side_two.get_active_immutable().hp;
            assert!(remaining == hp || remaining == hp - (hp / 2).max(1));
            state.reverse_instructions(&branch.instruction_list);
        }
    }
}
