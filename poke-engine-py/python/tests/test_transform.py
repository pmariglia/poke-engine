"""Reversible Transform/Imposter binding regressions."""

import poke_engine as pe


def mon(name, moves, **kwargs):
    return pe.Pokemon(
        id=name, moves=[pe.Move(id=m, pp=pp) for m, pp in moves], **kwargs
    )


def state_with_ditto(**target):
    return pe.State(
        side_one=pe.Side(
            pokemon=[
                mon("mew", [("splash", 40)]),
                mon(
                    "ditto",
                    [("transform", 16)],
                    ability="imposter",
                    hp=120,
                    maxhp=200,
                    item="choicescarf",
                    attack=60,
                    status="poison",
                    level=83,
                ),
            ]
        ),
        side_two=pe.Side(
            pokemon=[
                mon(
                    "jumpluff",
                    [("leechseed", 16), ("splash", 40)],
                    types=("grass", "flying"),
                    ability="chlorophyll",
                    attack=150,
                    weight_kg=3,
                    **target,
                )
            ],
            attack_boost=2,
            speed_boost=1,
        ),
    )


def branches(state, a, b):
    original = state.to_string()
    for branch in pe.generate_instructions(state, a, b):
        if branch.percentage <= 0:
            continue
        after = state.apply_instructions(branch)
        assert after.reverse_instructions(branch).to_string() == original
        assert pe.State.from_string(after.to_string()).to_string() == after.to_string()
        yield branch, after
    assert state.to_string() == original


def test_imposter_copies_grass_and_blocks_leech_seed_preserving_own_resources():
    for _, after in branches(state_with_ditto(), "ditto", "leechseed"):
        ditto = after.side_one.pokemon[1]
        assert ditto.id.lower() == "jumpluff"
        assert tuple(t.lower() for t in ditto.types) == ("grass", "flying")
        assert ditto.ability.lower() == "chlorophyll"
        assert ditto.base_ability.lower() == "imposter"
        assert ditto.hp == 95  # own 200 max HP, poison tick, no seed
        assert ditto.maxhp == 200 and ditto.level == 83
        assert ditto.attack == 150 and ditto.weight_kg == 3
        assert ditto.item.lower() == "choicescarf" and ditto.status.lower() == "poison"
        assert [m.pp for m in ditto.moves] == [5, 5, 0, 0]
        assert "leechseed" not in after.side_one.volatile_statuses
        assert after.side_one.attack_boost == 2
        assert after.side_one.speed_boost == 1
        assert ditto.transform_original is not None


def test_switch_out_restores_original_moves_and_stats_preserving_hp_item_status():
    _, changed = next(branches(state_with_ditto(), "ditto", "splash"))
    _, after = next(branches(changed, "mew", "splash"))
    ditto = after.side_one.pokemon[1]
    assert ditto.id.lower() == "ditto" and ditto.attack == 60
    assert tuple(t.lower() for t in ditto.types) == ("normal", "typeless")
    assert ditto.ability.lower() == "imposter"
    assert ditto.moves[0].id.lower() == "transform" and ditto.moves[0].pp == 16
    assert (
        ditto.hp == 95
        and ditto.status.lower() == "poison"
        and ditto.item.lower() == "choicescarf"
    )
    assert ditto.transform_original is None
    _, back = next(branches(after, "ditto", "splash"))
    assert back.side_one.pokemon[1].id.lower() == "jumpluff"


def test_substitute_blocks_imposter():
    state = state_with_ditto()
    state = pe.State(
        side_one=state.side_one,
        side_two=pe.Side(
            pokemon=state.side_two.pokemon,
            volatile_statuses={"substitute"},
            substitute_health=25,
        ),
    )
    for _, after in branches(state, "ditto", "splash"):
        assert after.side_one.pokemon[1].id.lower() == "ditto"
        assert after.side_one.pokemon[1].transform_original is None


def test_transform_copies_disabled_or_exhausted_move_with_five_pp():
    state = state_with_ditto()
    state = pe.State(
        side_one=state.side_one,
        side_two=pe.Side(
            pokemon=[
                pe.Pokemon(
                    id="mew",
                    moves=[
                        pe.Move(id="recover", pp=0, disabled=True),
                        pe.Move(id="splash", pp=40),
                    ],
                )
            ]
        ),
    )
    _, after = next(branches(state, "ditto", "splash"))
    assert after.side_one.pokemon[1].moves[0].pp == 5
    assert not after.side_one.pokemon[1].moves[0].disabled


def test_transform_move_restores_its_own_remaining_pp_after_switch_out():
    state = pe.State(
        side_one=pe.Side(
            pokemon=[
                mon("ditto", [("transform", 9)], speed=200),
                mon("mew", [("splash", 40)]),
            ]
        ),
        side_two=pe.Side(
            pokemon=[mon("jumpluff", [("splash", 40)], types=("grass", "flying"))]
        ),
    )
    _, after = next(branches(state, "transform", "splash"))
    assert after.side_one.pokemon[0].moves[0].pp == 5
    _, restored = next(branches(after, "mew", "splash"))
    assert restored.side_one.pokemon[0].moves[0].pp == 8
