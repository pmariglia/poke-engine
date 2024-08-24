from dataclasses import dataclass, field

# noinspection PyUnresolvedReferences
from ._poke_engine import (
    State as _State,
    Side as _Side,
    SideConditions as _SideConditions,
    Pokemon as _Pokemon,
    Move as _Move,
)


@dataclass
class Move:
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
    id: str = ""
    level: int = 100
    types: (str, str) = ("normal", "typeless")
    hp: int = 0
    maxhp: int = 0
    ability: str = "none"
    item: str = "none"
    attack: int = 0
    defense: int = 0
    special_attack: int = 0
    special_defense: int = 0
    speed: int = 0
    status: str = "none"
    rest_turns: int = 0
    weight_kg: float = 0
    moves: list[Move] = field(default_factory=list)

    def _into_rust_obj(self):
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
            weight_kg=self.weight_kg,
            moves=[m._into_rust_obj() for m in self.moves],
        )


@dataclass
class SideConditions:
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
    active_index: str = "0"
    baton_passing: bool = False
    pokemon: list[Pokemon] = field(default_factory=list)
    side_conditions: SideConditions = field(default_factory=SideConditions)
    wish: (int, int) = (0, 0)
    force_switch: bool = False
    force_trapped: bool = False
    slow_uturn_move: bool = False
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
    side_one: Side = field(default_factory=Side)
    side_two: Side = field(default_factory=Side)
    weather: str = "none"
    terrain: str = "none"
    trick_room: bool = False
    team_preview: bool = False

    def _into_rust_obj(self):
        return _State(
            side_one=self.side_one._into_rust_obj(),
            side_two=self.side_two._into_rust_obj(),
            weather=self.weather,
            terrain=self.terrain,
            trick_room=self.trick_room,
            team_preview=self.team_preview,
        )
