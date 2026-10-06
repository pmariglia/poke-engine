#![cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
use poke_engine::choices::Choices;
use poke_engine::engine::abilities::Abilities;
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::engine::items::Items;
use poke_engine::engine::state::{MoveChoice, PokemonVolatileStatus};
use poke_engine::pokemon::PokemonName;
use poke_engine::state::{PokemonIndex, PokemonMoveIndex, PokemonStatus, PokemonType, State};

fn entry_position() -> State {
    let mut s = State::default();
    s.side_one.get_active().id = PokemonName::MEW;
    let ditto = &mut s.side_one.pokemon[PokemonIndex::P1];
    ditto.id = PokemonName::DITTO;
    ditto.hp = 120;
    ditto.maxhp = 200;
    ditto.level = 83;
    ditto.attack = 60;
    ditto.ability = Abilities::IMPOSTER;
    ditto.base_ability = Abilities::IMPOSTER;
    ditto.item = Items::CHOICESCARF;
    ditto.status = PokemonStatus::POISON;
    ditto.replace_move(PokemonMoveIndex::M0, Choices::TRANSFORM);
    ditto.moves[&PokemonMoveIndex::M0].pp = 16;
    let foe = s.side_two.get_active();
    foe.id = PokemonName::JUMPLUFF;
    foe.types = (PokemonType::GRASS, PokemonType::FLYING);
    foe.ability = Abilities::CHLOROPHYLL;
    foe.attack = 150;
    foe.replace_move(PokemonMoveIndex::M0, Choices::LEECHSEED);
    s.side_two.attack_boost = 2;
    s
}

#[test]
fn imposter_entry_copies_grass_and_blocks_seed_preserving_own_resources() {
    let mut s = entry_position();
    let original = format!("{:?}", s);
    let branches = generate_instructions_from_move_pair(
        &mut s,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        false,
    );
    for b in branches {
        s.apply_instructions(&b.instruction_list);
        let d = s.side_one.get_active_immutable();
        assert_eq!(d.id, PokemonName::JUMPLUFF);
        assert_eq!(d.types, (PokemonType::GRASS, PokemonType::FLYING));
        assert_eq!(d.hp, 95);
        assert_eq!(d.maxhp, 200);
        assert_eq!(d.level, 83);
        assert_eq!(d.item, Items::CHOICESCARF);
        assert_eq!(d.status, PokemonStatus::POISON);
        assert_eq!(d.attack, 150);
        assert_eq!(d.ability, Abilities::CHLOROPHYLL);
        assert_eq!(d.moves[&PokemonMoveIndex::M0].pp, 5);
        assert_eq!(s.side_one.attack_boost, 2);
        assert!(!s
            .side_one
            .volatile_statuses
            .contains(&PokemonVolatileStatus::LEECHSEED));
        assert!(d.transform_original.is_some());
        assert_eq!(
            State::deserialize(&s.serialize()).serialize(),
            s.serialize()
        );
        let restore = generate_instructions_from_move_pair(
            &mut s,
            &MoveChoice::Switch(PokemonIndex::P0),
            &MoveChoice::None,
            false,
        );
        for rb in restore {
            s.apply_instructions(&rb.instruction_list);
            let d = &s.side_one.pokemon[PokemonIndex::P1];
            assert_eq!(d.id, PokemonName::DITTO);
            assert_eq!(d.attack, 60);
            assert_eq!(d.moves[&PokemonMoveIndex::M0].id, Choices::TRANSFORM);
            assert_eq!(d.moves[&PokemonMoveIndex::M0].pp, 16);
            assert_eq!(d.hp, 95);
            assert!(d.transform_original.is_none());
            s.reverse_instructions(&rb.instruction_list);
        }
        s.reverse_instructions(&b.instruction_list);
        assert_eq!(format!("{:?}", s), original);
    }
}

#[test]
fn substitute_and_neutralizing_gas_block_imposter_entry() {
    for gas in [false, true] {
        let mut s = entry_position();
        if gas {
            s.side_two.get_active().ability = Abilities::NEUTRALIZINGGAS;
        } else {
            s.side_two
                .volatile_statuses
                .insert(PokemonVolatileStatus::SUBSTITUTE);
            s.side_two.substitute_health = 25;
        }
        let branches = generate_instructions_from_move_pair(
            &mut s,
            &MoveChoice::Switch(PokemonIndex::P1),
            &MoveChoice::None,
            false,
        );
        for b in branches {
            s.apply_instructions(&b.instruction_list);
            assert_eq!(s.side_one.get_active_immutable().id, PokemonName::DITTO);
            s.reverse_instructions(&b.instruction_list);
        }
    }
}
