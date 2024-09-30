from dataclasses import dataclass, field

# noinspection PyUnresolvedReferences
from ._poke_engine import (
    State as _State,
    Side as _Side,
    SideConditions as _SideConditions,
    Pokemon as _Pokemon,
    Move as _Move,
    state_from_string as _state_from_string,
)


@dataclass
class Move:
    """

    :param id: The name of the move
    :type id: str
    :param disabled: Whether the move is disabled
    :type disabled: bool
    :param pp: The current PP of the move
    :type pp: int
    """

    id: str = "none"
    disabled: bool = False
    pp: int = 32

    def _into_rust_obj(self):
        return _Move(
            id=self.id,
            disabled=self.disabled,
            pp=self.pp,
        )


@dataclass
class Pokemon:
    """

    :param id: The name of the Pokemon
    :type id: str
    :param level: The level of the Pokemon
    :type level: int
    :param types: The types of the Pokemon
    :type types: (str, str)
    :param hp: The current HP of the Pokemon
    :type hp: int
    :param maxhp: The maximum HP of the Pokemon
    :type maxhp: int
    :param ability: The ability of the Pokemon
    :type ability: str
    :param item: The item held by the Pokemon
    :type item: str
    :param attack: The attack stat of the Pokemon
    :type attack: int
    :param defense: The defense stat of the Pokemon
    :type defense: int
    :param special_attack: The special attack stat of the Pokemon
    :type special_attack: int
    :param special_defense: The special defense stat of the Pokemon
    :type special_defense: int
    :param speed: The speed stat of the Pokemon
    :type speed: int
    :param status: The status of the Pokemon
    :type status: str
    :param rest_turns: The remaining number of turns for a Pokemon that is asleep due to the move rest
    :type rest_turns: int
    :param sleep_turns: The number of turns this Pokemon has been asleep
    :type sleep_turns: int
    :param weight_kg: The weight of the Pokemon in kilograms
    :type weight_kg: float
    :param moves: The moves of the Pokemon
    :type moves: list[Move]
    """

    id: str = ""
    level: int = 100
    types: (str, str) = ("normal", "typeless")
    hp: int = 100
    maxhp: int = 100
    ability: str = "none"
    item: str = "none"
    attack: int = 100
    defense: int = 100
    special_attack: int = 100
    special_defense: int = 100
    speed: int = 100
    status: str = "none"
    rest_turns: int = 0
    sleep_turns: int = 0
    weight_kg: float = 0
    moves: list[Move] = field(default_factory=list)

    def _into_rust_obj(self):
        if len(self.types) == 1:
            self.types = (self.types[0], "typeless")

        return _Pokemon(
            id=self.id,
            level=self.level,
            types=self.types,
            hp=self.hp,
            maxhp=self.maxhp,
            ability=self.ability,
            item=self.item,
            attack=self.attack,
            defense=self.defense,
            special_attack=self.special_attack,
            special_defense=self.special_defense,
            speed=self.speed,
            status=self.status,
            rest_turns=self.rest_turns,
            sleep_turns=self.sleep_turns,
            weight_kg=self.weight_kg,
            moves=[m._into_rust_obj() for m in self.moves],
        )


@dataclass
class SideConditions:
    """
    The side conditions on a Side

    :param aurora_veil:
    :type aurora_veil: int
    :param crafty_shield:
    :type crafty_shield: int
    :param healing_wish:
    :type healing_wish: int
    :param light_screen:
    :type light_screen: int
    :param lucky_chant:
    :type lucky_chant: int
    :param lunar_dance:
    :type lunar_dance: int
    :param mat_block:
    :type mat_block: int
    :param mist:
    :type mist: int
    :param protect:
    :type protect: int
    :param quick_guard:
    :type quick_guard: int
    :param reflect:
    :type reflect: int
    :param safeguard:
    :type safeguard: int
    :param spikes:
    :type spikes: int
    :param stealth_rock:
    :type stealth_rock: int
    :param sticky_web:
    :type sticky_web: int
    :param tailwind:
    :type tailwind: int
    :param toxic_count:
    :type toxic_count: int
    :param toxic_spikes:
    :type toxic_spikes: int
    :param wide_guard:
    :type wide_guard: int
    """

    aurora_veil: int = 0
    crafty_shield: int = 0
    healing_wish: int = 0
    light_screen: int = 0
    lucky_chant: int = 0
    lunar_dance: int = 0
    mat_block: int = 0
    mist: int = 0
    protect: int = 0
    quick_guard: int = 0
    reflect: int = 0
    safeguard: int = 0
    spikes: int = 0
    stealth_rock: int = 0
    sticky_web: int = 0
    tailwind: int = 0
    toxic_count: int = 0
    toxic_spikes: int = 0
    wide_guard: int = 0

    def _into_rust_obj(self) -> _SideConditions:
        return _SideConditions(
            aurora_veil=self.aurora_veil,
            crafty_shield=self.crafty_shield,
            healing_wish=self.healing_wish,
            light_screen=self.light_screen,
            lucky_chant=self.lucky_chant,
            lunar_dance=self.lunar_dance,
            mat_block=self.mat_block,
            mist=self.mist,
            protect=self.protect,
            quick_guard=self.quick_guard,
            reflect=self.reflect,
            safeguard=self.safeguard,
            spikes=self.spikes,
            stealth_rock=self.stealth_rock,
            sticky_web=self.sticky_web,
            tailwind=self.tailwind,
            toxic_count=self.toxic_count,
            toxic_spikes=self.toxic_spikes,
            wide_guard=self.wide_guard,
        )


@dataclass
class Side:
    """
    One of the sides of a pokemon battle

    :param active_index: The index of the active Pokemon
    :type active_index: str
    :param baton_passing: Set to `true` if the next move this side is making is a switch due to having used baton pass.
        `force_switch` will always be `true` if this is `true`
    :type baton_passing: bool
    :param pokemon: The Pokemon on this side
    :type pokemon: list[Pokemon]
    :param side_conditions: The SideConditions on this side
    :type side_conditions: SideConditions
    :param wish: Tuple representing the wish status. Format is (turns_remaining, health)
    :type wish: (int, int)
    :param force_switch: Whether this side is forced to switch next turn
    :type force_switch: bool
    :param force_trapped: Whether the side is forcibly trapped and cannot switch
    :type force_trapped: bool
    :param volatile_statuses: List of active volatile statuses for this sides active Pokemon
    :type volatile_statuses: list[str]
    :param substitute_health: Health of the substitute for this sides active Pokemon.
        Only applies if the active Pokemon has a substitute volatile status
    :type substitute_health: int
    :param attack_boost: Attack boost of the active Pokemon
    :type attack_boost: int
    :param defense_boost: Defense boost of the active Pokemon
    :type defense_boost: int
    :param special_attack_boost: Special attack boost of the active Pokemon
    :type special_attack_boost: int
    :param special_defense_boost: Special defense boost of the active Pokemon
    :type special_defense_boost: int
    :param speed_boost: Speed boost of the active Pokemon
    :type speed_boost: int
    :param accuracy_boost: Accuracy boost of the active Pokemon
    :type accuracy_boost: int
    :param evasion_boost: Evasion boost of the active Pokemon
    :type evasion_boost: int
    :param last_used_move: The last move used by this side.
        Format is "move:<move_id>" for a move, "switch:<pokemon_index>" for a switch, or "none" if neither are applicable
    :type last_used_move: str
    :param slow_uturn_move: Whether a slow U-turn move is pending.
        This is set to `true` if the opposing side is switching out and this side has a pending move that will be executed after the switch
    :type slow_uturn_move: bool
    :param switch_out_move_second_saved_move: If `slow_uturn_move` is active,
        this is the move that will be executed after the switch
    :type switch_out_move_second_saved_move: str
    """

    active_index: str = "0"
    baton_passing: bool = False
    pokemon: list[Pokemon] = field(default_factory=list)
    side_conditions: SideConditions = field(default_factory=SideConditions)
    wish: (int, int) = (0, 0)
    force_switch: bool = False
    force_trapped: bool = False
    volatile_statuses: list[str] = field(default_factory=list)
    substitute_health: int = 0
    attack_boost: int = 0
    defense_boost: int = 0
    special_attack_boost: int = 0
    special_defense_boost: int = 0
    speed_boost: int = 0
    accuracy_boost: int = 0
    evasion_boost: int = 0
    last_used_move: str = "move:none"
    slow_uturn_move: bool = False
    switch_out_move_second_saved_move: str = "none"

    def _into_rust_obj(self):
        return _Side(
            active_index=self.active_index,
            baton_passing=self.baton_passing,
            pokemon=[p._into_rust_obj() for p in self.pokemon],
            side_conditions=self.side_conditions._into_rust_obj(),
            wish=self.wish,
            force_switch=self.force_switch,
            force_trapped=self.force_trapped,
            slow_uturn_move=self.slow_uturn_move,
            volatile_statuses=self.volatile_statuses,
            substitute_health=self.substitute_health,
            attack_boost=self.attack_boost,
            defense_boost=self.defense_boost,
            special_attack_boost=self.special_attack_boost,
            special_defense_boost=self.special_defense_boost,
            speed_boost=self.speed_boost,
            accuracy_boost=self.accuracy_boost,
            evasion_boost=self.evasion_boost,
            last_used_move=self.last_used_move,
            switch_out_move_second_saved_move=self.switch_out_move_second_saved_move,
        )


@dataclass
class State:
    """
    The state of a pokemon battle

    :param side_one: the first side of the battle
    :type side_one: Side
    :param side_two: the second side of the battle
    :type side_two: Side
    :param weather: the currently active weather
    :type weather: str
    :param weather_turns_remaining: number of turns remaining for weather
    :type weather_turns_remaining: int
    :param terrain: the currently active terrain
    :type terrain: str
    :param terrain_turns_remaining: number of turns remaining for terrain
    :type terrain_turns_remaining: int
    :param trick_room: whether trick room is active
    :type trick_room: bool
    :param trick_room_turns_remaining: number of turns remaining for trick room
    :type trick_room_turns_remaining: int
    :param team_preview: if the battle is currently in team preview
    :type team_preview: bool
    """

    side_one: Side = field(default_factory=Side)
    side_two: Side = field(default_factory=Side)
    weather: str = "none"
    weather_turns_remaining: int = -1
    terrain: str = "none"
    terrain_turns_remaining: int = 0
    trick_room: bool = False
    trick_room_turns_remaining: int = 0
    team_preview: bool = False

    def _into_rust_obj(self):
        return _State(
            side_one=self.side_one._into_rust_obj(),
            side_two=self.side_two._into_rust_obj(),
            weather=self.weather,
            weather_turns_remaining=self.weather_turns_remaining,
            terrain=self.terrain,
            terrain_turns_remaining=self.terrain_turns_remaining,
            trick_room=self.trick_room,
            trick_room_turns_remaining=self.trick_room_turns_remaining,
            team_preview=self.team_preview,
        )

    def to_string(self):
        return self._into_rust_obj().to_string()

    @classmethod
    def from_string(cls, state_str: str):
        return _state_from_string(state_str)
