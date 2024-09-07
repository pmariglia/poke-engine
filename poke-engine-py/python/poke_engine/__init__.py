from dataclasses import dataclass

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


@dataclass
class MctsSideResult:
    """
    Result of a Monte Carlo Tree Search for a single side

    :param move_choice: The move that was chosen
    :type move_choice: str
    :param total_score: The total score of the chosen move
    :type total_score: float
    :param visits: The number of times the move was chosen
    :type visits: int
    """

    move_choice: str
    total_score: float
    visits: int


@dataclass
class MctsResult:
    """
    Result of a Monte Carlo Tree Search

    :param side_one: Result for side one
    :type side_one: list[MctsSideResult]
    :param side_two: Result for side two
    :type side_two: list[MctsSideResult]
    :param total_visits: Total number of monte carlo iterations
    :type total_visits: int
    """

    side_one: list[MctsSideResult]
    side_two: list[MctsSideResult]
    total_visits: int

    @classmethod
    def _from_rust(cls, rust_result):
        return cls(
            side_one=[
                MctsSideResult(
                    move_choice=i.move_choice,
                    total_score=i.total_score,
                    visits=i.visits,
                )
                for i in rust_result.s1
            ],
            side_two=[
                MctsSideResult(
                    move_choice=i.move_choice,
                    total_score=i.total_score,
                    visits=i.visits,
                )
                for i in rust_result.s2
            ],
            total_visits=rust_result.iteration_count,
        )


def generate_instructions(state: State, side_one_move: str, side_two_move: str):
    """
    TODO
    """
    return _gi(state._into_rust_obj(), side_one_move, side_two_move)


def monte_carlo_tree_search(state: State, duration_ms: int = 1000) -> MctsResult:
    """
    Perform monte-carlo-tree-search on the given state and for the given duration

    :param state: the state to search through
    :type state: State
    :param duration_ms: time in milliseconds to run the search
    :type duration_ms: int
    :return: the result of the search
    :rtype: MctsResult
    """
    return MctsResult._from_rust(_mcts(state._into_rust_obj(), duration_ms))


def calculate_damage(
    state: State, s1_move: str, s2_move: str, s1_moves_first: bool
) -> (list[int], list[int]):
    """
    Calculate the damage rolls for two moves

    :param state:
    :type state: State
    :param s1_move:
    :type s1_move: str
    :param s2_move:
    :type s2_move: str
    :param s1_moves_first:
    :type s1_moves_first: bool
    :return: (list[int], list[int]) - the damage rolls for the two moves
    """
    return _calculate_damage(state._into_rust_obj(), s1_move, s2_move, s1_moves_first)


__all__ = [
    "State",
    "Side",
    "SideConditions",
    "Pokemon",
    "Move",
    "MctsResult",
    "MctsSideResult",
    "generate_instructions",
    "monte_carlo_tree_search",
    "calculate_damage",
]
