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
    id as _id,
)


@dataclass
class IterativeDeepeningResult:
    """
    Result of an Iterative Deepening Expectiminimax Search

    :param side_one: The moves for side_one
    :type side_one: list[str]
    :param side_two: The moves for side_two
    :type side_two: list[str]
    :param matrix: A vector representing the payoff matrix of the search.
        Pruned branches are represented by None
    :type matrix: int
    :param depth_searched: The depth that was searched to
    :type depth_searched: int
    """

    side_one: list[str]
    side_two: list[str]
    matrix: list[float]
    depth_searched: int

    @classmethod
    def _from_rust(cls, rust_result):
        return cls(
            side_one=rust_result.s1,
            side_two=rust_result.s2,
            matrix=rust_result.matrix,
            depth_searched=rust_result.depth_searched,
        )

    def get_safest_move(self) -> str:
        """
        Get the safest move for side_one
        The safest move is the move that minimizes the loss for the turn

        :return: The safest move
        :rtype: str
        """
        safest_value = float("-inf")
        safest_s1_index = 0
        vec_index = 0
        for i in range(len(self.side_one)):
            worst_case_this_row = float("inf")
            for _ in range(len(self.side_two)):
                score = self.matrix[vec_index]
                if score < worst_case_this_row:
                    worst_case_this_row = score

            if worst_case_this_row > safest_value:
                safest_s1_index = i
                safest_value = worst_case_this_row

        return self.side_one[safest_s1_index]


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


def iterative_deepening_expectiminimax(
    state: State, duration_ms: int = 1000
) -> IterativeDeepeningResult:
    """
    Perform an iterative-deepening expectiminimax search on the given state and for the given duration

    :param state: the state to search through
    :type state: State
    :param duration_ms: time in milliseconds to run the search
    :type duration_ms: int
    :return: the result of the search
    :rtype: IterativeDeepeningResult
    """
    return IterativeDeepeningResult._from_rust(_id(state._into_rust_obj(), duration_ms))


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
    "IterativeDeepeningResult",
    "generate_instructions",
    "monte_carlo_tree_search",
    "iterative_deepening_expectiminimax",
    "calculate_damage",
]
