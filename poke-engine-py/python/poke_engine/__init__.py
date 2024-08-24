from .state import (
    State,
    Side,
    SideConditions,
    Pokemon,
    Move,
)

# noinspection PyUnresolvedReferences
from ._poke_engine import (
    gi as _gi,
    calculate_damage as _calculate_damage,
    mcts as _mcts,
)


def generate_instructions(state: State, side_one_move: str, side_two_move: str):
    return _gi(state._into_rust_obj(), side_one_move, side_two_move)


def monte_carlo_tree_search(state: State, duration_ms: int = 1000) -> str:
    return _mcts(state._into_rust_obj(), duration_ms)


def calculate_damage(
    state: State, s1_move: str, s2_move: str, s1_moves_first: bool
) -> str:
    return _calculate_damage(state._into_rust_obj(), s1_move, s2_move, s1_moves_first)
