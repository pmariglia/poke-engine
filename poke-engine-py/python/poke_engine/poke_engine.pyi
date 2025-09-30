from typing import List, Tuple, Set, Optional

from poke_engine import Weather, Terrain, PokemonIndex

class VolatileStatusDurations:
    """
    Durations for volatile statuses on a SideSlot.

    :param confusion: Confusion turns remaining
    :type confusion: int
    :param encore: Encore turns remaining
    :type encore: int
    :param lockedmove: Locked move turns remaining
    :type lockedmove: int
    :param slowstart: Slow Start turns remaining
    :type slowstart: int
    :param taunt: Taunt turns remaining
    :type taunt: int
    :param yawn: Yawn turns remaining
    :type yawn: int
    """

    confusion: int
    encore: int
    lockedmove: int
    slowstart: int
    taunt: int
    yawn: int

    def __init__(
        self,
        confusion: int = 0,
        encore: int = 0,
        lockedmove: int = 0,
        slowstart: int = 0,
        taunt: int = 0,
        yawn: int = 0,
    ) -> None: ...

class SideConditions:
    """
    Side conditions affecting a Side.

    :param aurora_veil: Aurora Veil turns remaining
    :type aurora_veil: int
    :param crafty_shield: Crafty Shield turns remaining
    :type crafty_shield: int
    :param healing_wish: Healing Wish turns remaining
    :type healing_wish: int
    :param light_screen: Light Screen turns remaining
    :type light_screen: int
    :param lucky_chant: Lucky Chant turns remaining
    :type lucky_chant: int
    :param lunar_dance: Lunar Dance turns remaining
    :type lunar_dance: int
    :param mat_block: Mat Block turns remaining
    :type mat_block: int
    :param mist: Mist turns remaining
    :type mist: int
    :param protect: Protect turns remaining
    :type protect: int
    :param quick_guard: Quick Guard turns remaining
    :type quick_guard: int
    :param reflect: Reflect turns remaining
    :type reflect: int
    :param safeguard: Safeguard turns remaining
    :type safeguard: int
    :param spikes: Number of Spikes layers
    :type spikes: int
    :param stealth_rock: Stealth Rock present
    :type stealth_rock: int
    :param sticky_web: Sticky Web present
    :type sticky_web: int
    :param tailwind: Tailwind turns remaining
    :type tailwind: int
    :param toxic_count: Toxic counter
    :type toxic_count: int
    :param toxic_spikes: Number of Toxic Spikes layers
    :type toxic_spikes: int
    :param wide_guard: Wide Guard turns remaining
    :type wide_guard: int
    """

    aurora_veil: int
    crafty_shield: int
    healing_wish: int
    light_screen: int
    lucky_chant: int
    lunar_dance: int
    mat_block: int
    mist: int
    protect: int
    quick_guard: int
    reflect: int
    safeguard: int
    spikes: int
    stealth_rock: int
    sticky_web: int
    tailwind: int
    toxic_count: int
    toxic_spikes: int
    wide_guard: int

    def __init__(
        self,
        spikes: int = 0,
        toxic_spikes: int = 0,
        stealth_rock: int = 0,
        sticky_web: int = 0,
        tailwind: int = 0,
        lucky_chant: int = 0,
        lunar_dance: int = 0,
        reflect: int = 0,
        light_screen: int = 0,
        aurora_veil: int = 0,
        crafty_shield: int = 0,
        safeguard: int = 0,
        mist: int = 0,
        protect: int = 0,
        healing_wish: int = 0,
        mat_block: int = 0,
        quick_guard: int = 0,
        toxic_count: int = 0,
        wide_guard: int = 0,
    ) -> None: ...

class Move:
    """
    Represents a Pokemon move.

    :param id: Move name
    :type id: str
    :param disabled: Whether the move is disabled
    :type disabled: bool
    :param pp: Remaining PP
    :type pp: int
    """

    id: str
    disabled: bool
    pp: int

    def __init__(
        self,
        id: str = "none",
        pp: int = 16,
        disabled: bool = False,
    ) -> None: ...

class Pokemon:
    """
    Represents a Pokemon with all its battle properties.

    :param id: Pokemon name
    :type id: str
    :param level: Pokemon level
    :type level: int
    :param types: Current types
    :type types: tuple[str, str]
    :param base_types: Base types. Different from types if the Pokemon had its types changed by a move or ability.
    :type base_types: tuple[str, str]
    :param hp: Current HP
    :type hp: int
    :param maxhp: Maximum HP
    :type maxhp: int
    :param ability: Current ability
    :type ability: str
    :param base_ability: Base ability. Different from ability if the Pokemon had its ability changed by a move or ability.
    :type base_ability: str
    :param item: Held item
    :type item: str
    :param nature: Nature
    :type nature: str
    :param evs: Effort values
    :type evs: tuple[int, int, int, int, int, int]
    :param attack: Attack stat
    :type attack: int
    :param defense: Defense stat
    :type defense: int
    :param special_attack: Special Attack stat
    :type special_attack: int
    :param special_defense: Special Defense stat
    :type special_defense: int
    :param speed: Speed stat
    :type speed: int
    :param status: Status condition
    :type status: str
    :param rest_turns: Rest turns remaining. Decrements from 2.
    :type rest_turns: int
    :param sleep_turns: Turns spent asleep. Increments from 0.
    :type sleep_turns: int
    :param weight_kg: Weight in kilograms
    :type weight_kg: float
    :param terastallized: if the Pokemon is terastallized
    :type terastallized: bool
    :param tera_type: Tera type
    :type tera_type: str
    :param moves: List of moves
    :type moves: list[Move]
    :param times_attacked: The number of times this Pokemon has been attacked
    :type times_attacked: int
    :param stellar_boosted_types: For pokemon terastallized into the Stellar type, this keeps track the types that have been boosted by Stellar Boost.
    :type stellar_boosted_types: list[str]
    """

    id: str
    level: int
    types: Tuple[str, str]
    base_types: Tuple[str, str]
    hp: int
    maxhp: int
    ability: str
    base_ability: str
    item: str
    nature: str
    evs: Tuple[int, int, int, int, int, int]
    attack: int
    defense: int
    special_attack: int
    special_defense: int
    speed: int
    status: str
    rest_turns: int
    sleep_turns: int
    weight_kg: float
    terastallized: bool
    tera_type: str
    moves: List[Move]
    times_attacked: int
    stellar_boosted_types: Optional[list[str]]

    def __init__(
        self,
        id: str = "pikachu",
        level: int = 50,
        types: Tuple[str, str] = ("normal", "typeless"),
        base_types: Tuple[str, str] = ("normal", "typeless"),
        hp: int = 100,
        maxhp: int = 100,
        ability: str = "none",
        base_ability: str = "",
        item: str = "none",
        nature: str = "serious",
        evs: Tuple[int, int, int, int, int, int] = (85, 85, 85, 85, 85, 85),
        attack: int = 100,
        defense: int = 100,
        special_attack: int = 100,
        special_defense: int = 100,
        speed: int = 100,
        status: str = "none",
        rest_turns: int = 0,
        sleep_turns: int = 0,
        weight_kg: float = 0.0,
        moves: Optional[List[Move]] = None,
        terastallized: bool = False,
        tera_type: str = "typeless",
        times_attacked: int = 0,
        stellar_boosted_types: Optional[list[str]] = None,
    ) -> None: ...
    @staticmethod
    def create_fainted() -> Pokemon: ...

class Side:
    """
    Represents a Side

    :param active_index: Index of the active Pokemon
    :type active_index: str
    :param baton_passing: Whether Baton Pass is active
    :type baton_passing: bool
    :param shed_tailing: Whether Shed Tail is active
    :type shed_tailing: bool
    :param volatile_status_durations: Durations for volatile statuses
    :type volatile_status_durations: VolatileStatusDurations
    :param wish: Wish status (turns, HP)
    :type wish: tuple[int, int]
    :param future_sight: Future Sight status (turns, target)
    :type future_sight: tuple[int, str]
    :param force_switch: Whether forced to switch
    :type force_switch: bool
    :param force_trapped: Whether trapped
    :type force_trapped: bool
    :param slow_uturn_move: Whether slow U-turn is active
    :type slow_uturn_move: bool
    :param volatile_statuses: Set of volatile status names
    :type volatile_statuses: set[str]
    :param substitute_health: Substitute HP
    :type substitute_health: int
    :param attack_boost: Attack boost
    :type attack_boost: int
    :param defense_boost: Defense boost
    :type defense_boost: int
    :param special_attack_boost: Special Attack boost
    :type special_attack_boost: int
    :param special_defense_boost: Special Defense boost
    :type special_defense_boost: int
    :param speed_boost: Speed boost
    :type speed_boost: int
    :param accuracy_boost: Accuracy boost
    :type accuracy_boost: int
    :param evasion_boost: Evasion boost
    :type evasion_boost: int
    :param last_used_move: Last used move
    :type last_used_move: str
    :param switch_out_move_second_saved_move: A move waiting to be used after another Pokemon completes it's pivot move
    :type switch_out_move_second_saved_move: str
    """

    pokemon: List[Pokemon]
    side_conditions: SideConditions
    active_index: PokemonIndex
    baton_passing: bool
    shed_tailing: bool
    volatile_status_durations: VolatileStatusDurations
    wish: Tuple[int, int]
    future_sight: Tuple[int, str]
    force_switch: bool
    force_trapped: bool
    slow_uturn_move: bool
    volatile_statuses: Set[str]
    substitute_health: int
    attack_boost: int
    defense_boost: int
    special_attack_boost: int
    special_defense_boost: int
    speed_boost: int
    accuracy_boost: int
    evasion_boost: int
    last_used_move: str
    switch_out_move_second_saved_move: str

    def __init__(
        self,
        pokemon: List[Pokemon] = None,
        active_index: PokemonIndex = PokemonIndex.P0,
        baton_passing: bool = False,
        shed_tailing: bool = False,
        volatile_status_durations: Optional[VolatileStatusDurations] = None,
        wish: Tuple[int, int] = (0, 0),
        future_sight: Tuple[int, str] = (0, "0"),
        force_switch: bool = False,
        force_trapped: bool = False,
        slow_uturn_move: bool = False,
        volatile_statuses: Optional[Set[str]] = None,
        substitute_health: int = 0,
        attack_boost: int = 0,
        defense_boost: int = 0,
        special_attack_boost: int = 0,
        special_defense_boost: int = 0,
        speed_boost: int = 0,
        accuracy_boost: int = 0,
        evasion_boost: int = 0,
        last_used_move: str = "move:none",
        switch_out_move_second_saved_move: str = "none",
        side_conditions: Optional[SideConditions] = None,
    ) -> None: ...

class StateInstructions:
    """State instructions with percentage and instruction list."""

    percentage: float
    instruction_list: List["Instruction"]

    def __repr__(self) -> str: ...

class Instruction:
    """Represents a single instruction."""
    def __repr__(self) -> str: ...

class State:
    """
    Represents a Pokemon battle state

    :param side_one: The first side of the battle
    :type side_one: Side
    :param side_two: The second side of the battle
    :type side_two: Side
    :param weather: The current weather condition
    :type weather: Weather
    :param weather_turns_remaining: The number of turns remaining for the current weather condition
    :type weather_turns_remaining: int
    :param terrain: The current terrain condition
    :type terrain: str
    :param terrain_turns_remaining: The number of turns remaining for the current terrain condition
    :type terrain_turns_remaining: int
    :param trick_room: Whether Trick Room is active
    :type trick_room: bool
    """

    side_one: Side
    side_two: Side
    weather: str
    weather_turns_remaining: int
    terrain: str
    terrain_turns_remaining: int
    trick_room: bool
    trick_room_turns_remaining: int
    team_preview: bool

    def __init__(
        self,
        side_one: Side = None,
        side_two: Side = None,
        weather: Weather = Weather.NONE,
        weather_turns_remaining: int = 0,
        terrain: Terrain = Terrain.NONE,
        terrain_turns_remaining: int = 0,
        trick_room: bool = False,
        trick_room_turns_remaining: int = 0,
        team_preview: bool = False,
    ) -> None: ...
    def apply_instructions(self, instructions: StateInstructions) -> State: ...
    def reverse_instructions(self, instructions: StateInstructions) -> State: ...
    @classmethod
    def from_string(cls, state_str: str) -> State: ...
    def to_string(self) -> str: ...

class MctsSideResult:
    """Result for one side from MCTS."""

    move_choice: Tuple[str, str]
    total_score: float
    visits: int

class MctsResult:
    """Result from MCTS algorithm."""

    side_one: List[MctsSideResult]
    side_two: List[MctsSideResult]
    iteration_count: int

def mcts(py_state: State, duration_ms: int) -> MctsResult:
    """
    Perform Monte Carlo Tree Search on the given state.

    :param py_state: The game state to analyze
    :param duration_ms: Duration in milliseconds to run MCTS
    :return: MCTS results for both sides
    """
    ...

def generate_instructions(
    py_state: State,
    side_one_move: str,
    side_two_move: str,
) -> List[StateInstructions]:
    """
    Generate instructions for a turn given moves for both sides.

    :param py_state: The current game state
    :param side_one_move: Move for side one
    :param side_two_move: Move for side two
    :return: List of state instructions
    """
    ...

def calculate_damage(
    py_state: State,
    side_one_move: str,
    side_two_move: str,
    side_one_moves_first: bool,
) -> List[int]:
    """
    Calculate damage rolls for a move.

    :param py_state: The current game state
    :param side_one_move: The move used for side_one
    :param side_two_move: The move used for side_two
    :param side_one_moves_first: if side_one moves first
    :return: List of damage values
    """
    ...
