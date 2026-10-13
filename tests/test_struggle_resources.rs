#![cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
use poke_engine::choices::Choices;
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::engine::state::MoveChoice;
use poke_engine::mcts::perform_mcts;
use poke_engine::mcts_threaded::perform_mcts_shared_tree;
use poke_engine::state::{PokemonIndex, PokemonMoveIndex, PokemonType, State};
use std::time::Duration;

fn exhausted_position(disabled: bool) -> State {
    let mut s = State::default();
    for index in [
        PokemonIndex::P1,
        PokemonIndex::P2,
        PokemonIndex::P3,
        PokemonIndex::P4,
        PokemonIndex::P5,
    ] {
        s.side_one.pokemon[index].hp = 0;
        s.side_two.pokemon[index].hp = 0;
    }
    let own = s.side_one.get_active();
    own.hp = 500;
    own.maxhp = 500;
    own.speed = 1;
    own.replace_move(PokemonMoveIndex::M0, Choices::SPLASH);
    for index in [
        PokemonMoveIndex::M0,
        PokemonMoveIndex::M1,
        PokemonMoveIndex::M2,
        PokemonMoveIndex::M3,
    ] {
        own.moves[&index].pp = if disabled { 32 } else { 0 };
        own.moves[&index].disabled = disabled;
    }
    let foe = s.side_two.get_active();
    foe.hp = 500;
    foe.maxhp = 500;
    foe.speed = 200;
    foe.replace_move(PokemonMoveIndex::M0, Choices::SPLASH);
    s
}

#[test]
fn exhausted_or_disabled_moves_offer_struggle_without_spending_pp() {
    for disabled in [false, true] {
        let mut s = exhausted_position(disabled);
        let (own, _) = s.root_get_all_options();
        assert_eq!(
            own.iter()
                .map(|m| m.to_string(&s.side_one))
                .collect::<Vec<_>>(),
            vec!["struggle"]
        );
        let struggle = MoveChoice::from_string("struggle", &s.side_one).unwrap();
        s.side_one.get_active().hp = 283;
        s.side_one.get_active().maxhp = 283;
        s.side_two.get_active().types = (PokemonType::GHOST, PokemonType::TYPELESS);
        let original = format!("{:?}", s);
        let branches = generate_instructions_from_move_pair(
            &mut s,
            &struggle,
            &MoveChoice::Move(PokemonMoveIndex::M0),
            false,
        );
        for b in branches {
            s.apply_instructions(&b.instruction_list);
            assert!(s.side_two.get_active_immutable().hp < 500);
            assert_eq!(s.side_one.get_active_immutable().hp, 212);
            assert_eq!(
                s.side_one.get_active_immutable().moves[&PokemonMoveIndex::M0].pp,
                if disabled { 32 } else { 0 }
            );
            s.reverse_instructions(&b.instruction_list);
            assert_eq!(format!("{:?}", s), original);
        }
    }
}

#[test]
fn faster_uturn_preserves_pending_struggle_for_both_search_implementations() {
    for disabled in [false, true] {
        for pivot_side in [1, 2] {
            let mut s = exhausted_position(disabled);
            s.side_two
                .get_active()
                .replace_move(PokemonMoveIndex::M0, Choices::UTURN);
            s.side_two.pokemon[PokemonIndex::P1].hp = 500;
            if pivot_side == 1 {
                std::mem::swap(&mut s.side_one, &mut s.side_two);
            }
            let struggle = MoveChoice::from_string(
                "struggle",
                if pivot_side == 1 {
                    &s.side_two
                } else {
                    &s.side_one
                },
            )
            .unwrap();
            let normal = MoveChoice::Move(PokemonMoveIndex::M0);
            let (a, b) = if pivot_side == 1 {
                (&normal, &struggle)
            } else {
                (&struggle, &normal)
            };
            let branches = generate_instructions_from_move_pair(&mut s, a, b, false);
            for branch in branches {
                s.apply_instructions(&branch.instruction_list);
                let (own, foe) = s.get_all_options();
                let waiting = if pivot_side == 1 { &foe } else { &own };
                assert_eq!(waiting, &vec![struggle]);
                let original = format!("{:?}", s);
                for threads in [1, 2] {
                    let (own, foe) = s.root_get_all_options();
                    let result = if threads == 1 {
                        perform_mcts(&mut s, own, foe, Duration::ZERO, 1000)
                    } else {
                        perform_mcts_shared_tree(&mut s, own, foe, Duration::ZERO, 1000, threads)
                    };
                    assert!(result.iteration_count >= 1000);
                    assert_eq!(format!("{:?}", s), original);
                }
                let (a, b) = if pivot_side == 1 {
                    (MoveChoice::Switch(PokemonIndex::P1), struggle)
                } else {
                    (struggle, MoveChoice::Switch(PokemonIndex::P1))
                };
                let resumed = generate_instructions_from_move_pair(&mut s, &a, &b, false);
                for r in resumed {
                    s.apply_instructions(&r.instruction_list);
                    let receiver = if pivot_side == 1 {
                        &s.side_one
                    } else {
                        &s.side_two
                    };
                    assert!(receiver.pokemon[PokemonIndex::P1].hp < 500);
                    s.reverse_instructions(&r.instruction_list);
                }
                s.reverse_instructions(&branch.instruction_list);
            }
        }
    }
}
