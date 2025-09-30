use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::{pyfunction, pymethods, pymodule, wrap_pyfunction, Bound, PyResult};
use std::collections::HashSet;

use poke_engine::choices::{Choices, MoveCategory, MOVES};
use poke_engine::engine::abilities::Abilities;
use poke_engine::engine::generate_instructions::{
    calculate_both_damage_rolls, generate_instructions_from_move_pair,
};
use poke_engine::engine::items::Items;
use poke_engine::engine::state::{MoveChoice, PokemonVolatileStatus, Terrain, Weather};
use poke_engine::instruction::{Instruction, StateInstructions};
use poke_engine::mcts::{perform_mcts, MctsResult, MctsSideResult};
use poke_engine::pokemon::PokemonName;
use poke_engine::search::iterative_deepen_expectiminimax;
use poke_engine::state::{
    LastUsedMove, Move, Pokemon, PokemonIndex, PokemonMoves, PokemonNature, PokemonStatus,
    PokemonType, Side, SideConditions, SidePokemon, State, StateTerrain, StateTrickRoom,
    StateWeather, VolatileStatusDurations,
};
use std::str::FromStr;
use std::time::Duration;

fn movechoice_to_string(side: &Side, move_choice: &MoveChoice) -> String {
    match move_choice {
        MoveChoice::Switch(_) => {
            format!("switch {}", move_choice.to_string(side))
        }
        _ => move_choice.to_string(side),
    }
}

#[derive(Clone)]
#[pyclass(name = "State", module = "poke_engine", get_all)]
pub struct PyState {
    pub side_one: PySide,
    pub side_two: PySide,
    pub weather: String,
    pub weather_turns_remaining: i8,
    pub terrain: String,
    pub terrain_turns_remaining: i8,
    pub trick_room: bool,
    pub trick_room_turns_remaining: i8,
    pub team_preview: bool,
}

impl From<State> for PyState {
    fn from(other: State) -> Self {
        PyState {
            side_one: PySide::from(other.side_one),
            side_two: PySide::from(other.side_two),
            weather: other.weather.weather_type.to_string(),
            weather_turns_remaining: other.weather.turns_remaining,
            terrain: other.terrain.terrain_type.to_string(),
            terrain_turns_remaining: other.terrain.turns_remaining,
            trick_room: other.trick_room.active,
            trick_room_turns_remaining: other.trick_room.turns_remaining,
            team_preview: other.team_preview,
        }
    }
}

impl Into<State> for PyState {
    fn into(self) -> State {
        let mut state = State {
            side_one: self.side_one.into(),
            side_two: self.side_two.into(),
            weather: StateWeather {
                weather_type: Weather::from_str(&self.weather).unwrap(),
                turns_remaining: self.weather_turns_remaining,
            },
            terrain: StateTerrain {
                terrain_type: Terrain::from_str(&self.terrain).unwrap(),
                turns_remaining: self.terrain_turns_remaining,
            },
            trick_room: StateTrickRoom {
                active: self.trick_room,
                turns_remaining: self.trick_room_turns_remaining,
            },
            team_preview: self.team_preview,
            use_last_used_move: false,
            use_damage_dealt: false,
        };
        state.set_conditional_mechanics();
        state
    }
}

#[pymethods]
impl PyState {
    #[new]
    #[pyo3(signature = (
        side_one=PySide::from(Side::default()),
        side_two=PySide::from(Side::default()),
        weather="none".to_string(),
        weather_turns_remaining=0,
        terrain="none".to_string(),
        terrain_turns_remaining=0,
        trick_room=false,
        trick_room_turns_remaining=0,
        team_preview=false,
    ))]
    fn new(
        side_one: PySide,
        side_two: PySide,
        weather: String,
        weather_turns_remaining: i8,
        terrain: String,
        terrain_turns_remaining: i8,
        trick_room: bool,
        trick_room_turns_remaining: i8,
        team_preview: bool,
    ) -> Self {
        PyState {
            side_one,
            side_two,
            weather,
            weather_turns_remaining,
            terrain,
            terrain_turns_remaining,
            trick_room,
            trick_room_turns_remaining,
            team_preview,
        }
    }
    fn apply_instructions(&self, instructions: PyStateInstructions) -> PyState {
        let mut state: State = self.clone().into();
        let instructions: StateInstructions = instructions.into();
        state.apply_instructions(&instructions.instruction_list);
        state.into()
    }
    fn reverse_instructions(&self, instructions: PyStateInstructions) -> PyState {
        let mut state: State = self.clone().into();
        let instructions: StateInstructions = instructions.into();
        state.reverse_instructions(&instructions.instruction_list);
        state.into()
    }
    #[classmethod]
    fn from_string(_cls: &Bound<'_, PyType>, state_str: String) -> PyResult<Self> {
        let state: State = State::deserialize(&state_str);
        Ok(PyState::from(state))
    }
    fn to_string(&self) -> String {
        let state: State = self.clone().into();
        state.serialize()
    }
}

#[derive(Clone)]
#[pyclass(name = "Side", module = "poke_engine", get_all)]
pub struct PySide {
    pokemon: [PyPokemon; 6],
    side_conditions: PySideConditions,

    active_index: String,
    baton_passing: bool,
    shed_tailing: bool,
    volatile_status_durations: PyVolatileStatusDurations,
    wish: (i8, i16),
    future_sight: (i8, String),
    force_switch: bool,
    force_trapped: bool,
    slow_uturn_move: bool,
    volatile_statuses: HashSet<String>,
    substitute_health: i16,
    attack_boost: i8,
    defense_boost: i8,
    special_attack_boost: i8,
    special_defense_boost: i8,
    speed_boost: i8,
    accuracy_boost: i8,
    evasion_boost: i8,
    last_used_move: String,
    switch_out_move_second_saved_move: String,
}

impl From<Side> for PySide {
    fn from(other: Side) -> Self {
        let pokemon = other.pokemon.pkmn.map(|pokemon| pokemon.into());
        PySide {
            pokemon,
            side_conditions: PySideConditions::from(other.side_conditions),
            active_index: other.active_index.serialize(),
            baton_passing: other.baton_passing,
            shed_tailing: other.shed_tailing,
            volatile_status_durations: PyVolatileStatusDurations::from(
                other.volatile_status_durations,
            ),
            wish: other.wish,
            future_sight: (other.future_sight.0, other.future_sight.1.serialize()),
            force_switch: other.force_switch,
            force_trapped: other.force_trapped,
            slow_uturn_move: other.slow_uturn_move,
            volatile_statuses: other
                .volatile_statuses
                .iter()
                .map(|s| s.to_string())
                .collect(),
            substitute_health: other.substitute_health,
            attack_boost: other.attack_boost,
            defense_boost: other.defense_boost,
            special_attack_boost: other.special_attack_boost,
            special_defense_boost: other.special_defense_boost,
            speed_boost: other.speed_boost,
            accuracy_boost: other.accuracy_boost,
            evasion_boost: other.evasion_boost,
            last_used_move: other.last_used_move.serialize(),
            switch_out_move_second_saved_move: other.switch_out_move_second_saved_move.to_string(),
        }
    }
}

impl Into<Side> for PySide {
    fn into(self) -> Side {
        Side {
            active_index: PokemonIndex::deserialize(&self.active_index),
            baton_passing: self.baton_passing,
            shed_tailing: self.shed_tailing,
            pokemon: SidePokemon {
                pkmn: [
                    self.pokemon[0].clone().into(),
                    self.pokemon[1].clone().into(),
                    self.pokemon[2].clone().into(),
                    self.pokemon[3].clone().into(),
                    self.pokemon[4].clone().into(),
                    self.pokemon[5].clone().into(),
                ],
            },
            side_conditions: self.side_conditions.into(),
            volatile_status_durations: VolatileStatusDurations::from(
                self.volatile_status_durations.into(),
            ),
            wish: self.wish,
            future_sight: (
                self.future_sight.0,
                PokemonIndex::deserialize(&self.future_sight.1),
            ),
            force_switch: self.force_switch,
            force_trapped: self.force_trapped,
            slow_uturn_move: self.slow_uturn_move,
            volatile_statuses: self
                .volatile_statuses
                .iter()
                .map(|s| PokemonVolatileStatus::from_str(s))
                .collect::<Result<HashSet<_>, _>>()
                .unwrap(),
            substitute_health: self.substitute_health,
            attack_boost: self.attack_boost,
            defense_boost: self.defense_boost,
            special_attack_boost: self.special_attack_boost,
            special_defense_boost: self.special_defense_boost,
            speed_boost: self.speed_boost,
            accuracy_boost: self.accuracy_boost,
            evasion_boost: self.evasion_boost,
            last_used_move: LastUsedMove::deserialize(&self.last_used_move),
            damage_dealt: Default::default(),
            switch_out_move_second_saved_move: Choices::from_str(
                &self.switch_out_move_second_saved_move,
            )
            .unwrap(),
        }
    }
}

#[pymethods]
impl PySide {
    #[new]
    #[pyo3(signature = (
        pokemon=Vec::<PyPokemon>::new(),
        side_conditions=PySideConditions::from(SideConditions::default()),
        active_index="0".to_string(),
        baton_passing=false,
        shed_tailing=false,
        volatile_status_durations=PyVolatileStatusDurations::new(0, 0, 0, 0, 0, 0),
        wish=(0, 0),
        future_sight=(0, "0".to_string()),
        force_switch=false,
        force_trapped=false,
        slow_uturn_move=false,
        volatile_statuses=HashSet::<String>::new(),
        substitute_health=0,
        attack_boost=0,
        defense_boost=0,
        special_attack_boost=0,
        special_defense_boost=0,
        speed_boost=0,
        accuracy_boost=0,
        evasion_boost=0,
        last_used_move="move:none".to_string(),
        switch_out_move_second_saved_move="none".to_string(),
    ))]
    fn new(
        mut pokemon: Vec<PyPokemon>,
        side_conditions: PySideConditions,

        active_index: String,
        baton_passing: bool,
        shed_tailing: bool,
        volatile_status_durations: PyVolatileStatusDurations,
        wish: (i8, i16),
        future_sight: (i8, String),
        force_switch: bool,
        force_trapped: bool,
        slow_uturn_move: bool,
        volatile_statuses: HashSet<String>,
        substitute_health: i16,
        attack_boost: i8,
        defense_boost: i8,
        special_attack_boost: i8,
        special_defense_boost: i8,
        speed_boost: i8,
        accuracy_boost: i8,
        evasion_boost: i8,
        last_used_move: String,
        switch_out_move_second_saved_move: String,
    ) -> Self {
        while pokemon.len() < 6 {
            pokemon.push(PyPokemon::create_fainted());
        }
        PySide {
            pokemon: [
                pokemon[0].clone(),
                pokemon[1].clone(),
                pokemon[2].clone(),
                pokemon[3].clone(),
                pokemon[4].clone(),
                pokemon[5].clone(),
            ],
            side_conditions,
            active_index,
            baton_passing,
            shed_tailing,
            volatile_status_durations,
            wish,
            future_sight,
            force_switch,
            force_trapped,
            slow_uturn_move,
            volatile_statuses,
            substitute_health,
            attack_boost,
            defense_boost,
            special_attack_boost,
            special_defense_boost,
            speed_boost,
            accuracy_boost,
            evasion_boost,
            last_used_move,
            switch_out_move_second_saved_move,
        }
    }
}

#[derive(Clone)]
#[pyclass(name = "VolatileStatusDurations", module = "poke_engine", get_all)]
pub struct PyVolatileStatusDurations {
    pub confusion: i8,
    pub encore: i8,
    pub lockedmove: i8,
    pub slowstart: i8,
    pub taunt: i8,
    pub yawn: i8,
}

impl From<VolatileStatusDurations> for PyVolatileStatusDurations {
    fn from(other: VolatileStatusDurations) -> Self {
        PyVolatileStatusDurations {
            confusion: other.confusion,
            encore: other.encore,
            lockedmove: other.lockedmove,
            slowstart: other.slowstart,
            taunt: other.taunt,
            yawn: other.yawn,
        }
    }
}

impl Into<VolatileStatusDurations> for PyVolatileStatusDurations {
    fn into(self) -> VolatileStatusDurations {
        VolatileStatusDurations {
            confusion: self.confusion,
            encore: self.encore,
            lockedmove: self.lockedmove,
            slowstart: self.slowstart,
            taunt: self.taunt,
            yawn: self.yawn,
        }
    }
}

#[pymethods]
impl PyVolatileStatusDurations {
    #[new]
    #[pyo3(signature = (
        confusion=0,
        encore=0,
        lockedmove=0,
        slowstart=0,
        taunt=0,
        yawn=0,
    ))]
    fn new(
        confusion: i8,
        encore: i8,
        lockedmove: i8,
        slowstart: i8,
        taunt: i8,
        yawn: i8,
    ) -> PyVolatileStatusDurations {
        PyVolatileStatusDurations {
            confusion,
            encore,
            lockedmove,
            slowstart,
            taunt,
            yawn,
        }
    }
}

#[derive(Clone)]
#[pyclass(name = "SideConditions", module = "poke_engine", get_all)]
pub struct PySideConditions {
    pub aurora_veil: i8,
    pub crafty_shield: i8,
    pub healing_wish: i8,
    pub light_screen: i8,
    pub lucky_chant: i8,
    pub lunar_dance: i8,
    pub mat_block: i8,
    pub mist: i8,
    pub protect: i8,
    pub quick_guard: i8,
    pub reflect: i8,
    pub safeguard: i8,
    pub spikes: i8,
    pub stealth_rock: i8,
    pub sticky_web: i8,
    pub tailwind: i8,
    pub toxic_count: i8,
    pub toxic_spikes: i8,
    pub wide_guard: i8,
}

impl From<SideConditions> for PySideConditions {
    fn from(other: SideConditions) -> Self {
        PySideConditions {
            aurora_veil: other.aurora_veil,
            crafty_shield: other.crafty_shield,
            healing_wish: other.healing_wish,
            light_screen: other.light_screen,
            lucky_chant: other.lucky_chant,
            lunar_dance: other.lunar_dance,
            mat_block: other.mat_block,
            mist: other.mist,
            protect: other.protect,
            quick_guard: other.quick_guard,
            reflect: other.reflect,
            safeguard: other.safeguard,
            spikes: other.spikes,
            stealth_rock: other.stealth_rock,
            sticky_web: other.sticky_web,
            tailwind: other.tailwind,
            toxic_count: other.toxic_count,
            toxic_spikes: other.toxic_spikes,
            wide_guard: other.wide_guard,
        }
    }
}

impl Into<SideConditions> for PySideConditions {
    fn into(self) -> SideConditions {
        SideConditions {
            aurora_veil: self.aurora_veil,
            crafty_shield: self.crafty_shield,
            healing_wish: self.healing_wish,
            light_screen: self.light_screen,
            lucky_chant: self.lucky_chant,
            lunar_dance: self.lunar_dance,
            mat_block: self.mat_block,
            mist: self.mist,
            protect: self.protect,
            quick_guard: self.quick_guard,
            reflect: self.reflect,
            safeguard: self.safeguard,
            spikes: self.spikes,
            stealth_rock: self.stealth_rock,
            sticky_web: self.sticky_web,
            tailwind: self.tailwind,
            toxic_count: self.toxic_count,
            toxic_spikes: self.toxic_spikes,
            wide_guard: self.wide_guard,
        }
    }
}

#[pymethods]
impl PySideConditions {
    #[new]
    #[pyo3(signature = (
        spikes=0,
        toxic_spikes=0,
        stealth_rock=0,
        sticky_web=0,
        tailwind=0,
        lucky_chant=0,
        lunar_dance=0,
        reflect=0,
        light_screen=0,
        aurora_veil=0,
        crafty_shield=0,
        safeguard=0,
        mist=0,
        protect=0,
        healing_wish=0,
        mat_block=0,
        quick_guard=0,
        toxic_count=0,
        wide_guard=0,
    ))]
    fn new(
        spikes: i8,
        toxic_spikes: i8,
        stealth_rock: i8,
        sticky_web: i8,
        tailwind: i8,
        lucky_chant: i8,
        lunar_dance: i8,
        reflect: i8,
        light_screen: i8,
        aurora_veil: i8,
        crafty_shield: i8,
        safeguard: i8,
        mist: i8,
        protect: i8,
        healing_wish: i8,
        mat_block: i8,
        quick_guard: i8,
        toxic_count: i8,
        wide_guard: i8,
    ) -> Self {
        PySideConditions {
            spikes,
            toxic_spikes,
            stealth_rock,
            sticky_web,
            tailwind,
            lucky_chant,
            lunar_dance,
            reflect,
            light_screen,
            aurora_veil,
            crafty_shield,
            safeguard,
            mist,
            protect,
            healing_wish,
            mat_block,
            quick_guard,
            toxic_count,
            wide_guard,
        }
    }
}

#[derive(Clone)]
#[pyclass(name = "Pokemon", module = "poke_engine", get_all)]
pub struct PyPokemon {
    pub id: String,
    pub level: i8,
    pub types: (String, String),
    pub base_types: (String, String),
    pub hp: i16,
    pub maxhp: i16,
    pub ability: String,
    pub base_ability: String,
    pub item: String,
    pub nature: String,
    pub evs: (u8, u8, u8, u8, u8, u8),
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
    pub status: String,
    pub rest_turns: i8,
    pub sleep_turns: i8,
    pub weight_kg: f32,
    pub terastallized: bool,
    pub tera_type: String,
    pub moves: Vec<PyMove>,
}

impl From<Pokemon> for PyPokemon {
    fn from(other: Pokemon) -> Self {
        Self {
            id: other.id.to_string(),
            level: other.level,
            types: (other.types.0.to_string(), other.types.1.to_string()),
            base_types: (
                other.base_types.0.to_string(),
                other.base_types.1.to_string(),
            ),
            hp: other.hp,
            maxhp: other.maxhp,
            ability: other.ability.to_string(),
            base_ability: other.base_ability.to_string(),
            item: other.item.to_string(),
            nature: other.nature.to_string(),
            evs: (
                other.evs.0,
                other.evs.1,
                other.evs.2,
                other.evs.3,
                other.evs.4,
                other.evs.5,
            ),
            attack: other.attack,
            defense: other.defense,
            special_attack: other.special_attack,
            special_defense: other.special_defense,
            speed: other.speed,
            status: other.status.to_string(),
            rest_turns: other.rest_turns,
            sleep_turns: other.sleep_turns,
            weight_kg: other.weight_kg,
            terastallized: other.terastallized,
            tera_type: other.tera_type.to_string(),
            moves: other
                .moves
                .into_iter()
                .map(|m| PyMove::from(m.clone()))
                .collect(),
        }
    }
}

impl Into<Pokemon> for PyPokemon {
    fn into(self) -> Pokemon {
        let mut moves_vec = self.moves.clone();
        while moves_vec.len() < 4 {
            moves_vec.push(PyMove::create_empty_move());
        }
        Pokemon {
            id: PokemonName::from_str(&self.id).unwrap(),
            level: self.level,
            types: (
                PokemonType::from_str(&self.types.0).unwrap(),
                PokemonType::from_str(&self.types.1).unwrap(),
            ),
            base_types: (
                PokemonType::from_str(&self.base_types.0).unwrap(),
                PokemonType::from_str(&self.base_types.1).unwrap(),
            ),
            hp: self.hp,
            maxhp: self.maxhp,
            ability: Abilities::from_str(&self.ability).unwrap(),
            base_ability: Abilities::from_str(&self.base_ability).unwrap(),
            item: Items::from_str(&self.item).unwrap(),
            nature: PokemonNature::from_str(&self.nature).unwrap(),
            evs: (
                self.evs.0, self.evs.1, self.evs.2, self.evs.3, self.evs.4, self.evs.5,
            ),
            attack: self.attack,
            defense: self.defense,
            special_attack: self.special_attack,
            special_defense: self.special_defense,
            speed: self.speed,
            status: PokemonStatus::from_str(&self.status).unwrap(),
            rest_turns: self.rest_turns,
            sleep_turns: self.sleep_turns,
            weight_kg: self.weight_kg,
            terastallized: self.terastallized,
            tera_type: PokemonType::from_str(&self.tera_type).unwrap(),
            moves: PokemonMoves {
                m0: moves_vec[0].clone().into(),
                m1: moves_vec[1].clone().into(),
                m2: moves_vec[2].clone().into(),
                m3: moves_vec[3].clone().into(),
            },
        }
    }
}

#[pymethods]
impl PyPokemon {
    #[new]
    #[pyo3(signature = (
        id="pikachu".to_string(),
        level=50,
        types=("normal".to_string(), "typeless".to_string()),
        base_types=("normal".to_string(), "typeless".to_string()),
        hp=100,
        maxhp=100,
        ability="none".to_string(),
        base_ability="".to_string(),
        item="none".to_string(),
        nature="serious".to_string(),
        evs=(85, 85, 85, 85, 85, 85),
        attack=100,
        defense=100,
        special_attack=100,
        special_defense=100,
        speed=100,
        status="none".to_string(),
        rest_turns=0,
        sleep_turns=0,
        weight_kg=0.0,
        moves=Vec::<PyMove>::new(),
        terastallized=false,
        tera_type="typeless".to_string(),
    ))]
    fn new(
        id: String,
        level: i8,
        types: (String, String),
        base_types: (String, String),
        hp: i16,
        maxhp: i16,
        ability: String,
        mut base_ability: String,
        item: String,
        nature: String,
        evs: (u8, u8, u8, u8, u8, u8),
        attack: i16,
        defense: i16,
        special_attack: i16,
        special_defense: i16,
        speed: i16,
        status: String,
        rest_turns: i8,
        sleep_turns: i8,
        weight_kg: f32,
        moves: Vec<PyMove>,
        terastallized: bool,
        tera_type: String,
    ) -> Self {
        if base_ability == "" {
            base_ability = ability.clone();
        }
        PyPokemon {
            id,
            level,
            types,
            base_types,
            hp,
            maxhp,
            ability,
            base_ability,
            item,
            nature,
            evs,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
            status,
            rest_turns,
            sleep_turns,
            weight_kg,
            terastallized,
            tera_type,
            moves,
        }
    }
    #[staticmethod]
    pub fn create_fainted() -> PyPokemon {
        let pkmn = Pokemon::default();
        let mut py_pkmn = PyPokemon::from(pkmn);
        py_pkmn.hp = 0; // fainted
        py_pkmn
    }
}

#[derive(Clone)]
#[pyclass(name = "Move", module = "poke_engine", get_all)]
pub struct PyMove {
    pub id: String,
    pub disabled: bool,
    pub pp: i8,
}

impl From<Move> for PyMove {
    fn from(other: Move) -> Self {
        PyMove {
            id: other.id.to_string(),
            disabled: other.disabled,
            pp: other.pp,
        }
    }
}

impl Into<Move> for PyMove {
    fn into(self) -> Move {
        Move {
            id: Choices::from_str(&self.id).unwrap(),
            disabled: self.disabled,
            pp: self.pp,
            choice: MOVES
                .get(&Choices::from_str(&self.id).unwrap())
                .unwrap()
                .clone(),
        }
    }
}

impl PyMove {
    fn create_empty_move() -> PyMove {
        PyMove {
            id: String::new(),
            disabled: true,
            pp: 0,
        }
    }
}

#[pymethods]
impl PyMove {
    #[new]
    #[pyo3(signature = (
        id="none".to_string(),
        pp=16,
        disabled=false,
    ))]
    fn new(id: String, pp: i8, disabled: bool) -> Self {
        PyMove { id, disabled, pp }
    }
}

#[derive(Clone)]
#[pyclass(get_all)]
struct PyMctsSideResult {
    pub move_choice: String,
    pub total_score: f32,
    pub visits: u32,
}

impl PyMctsSideResult {
    fn from_mcts_side_result(result: MctsSideResult, side: &Side) -> Self {
        PyMctsSideResult {
            move_choice: movechoice_to_string(side, &result.move_choice),
            total_score: result.total_score,
            visits: result.visits,
        }
    }
}

#[derive(Clone)]
#[pyclass(get_all)]
struct PyMctsResult {
    s1: Vec<PyMctsSideResult>,
    s2: Vec<PyMctsSideResult>,
    iteration_count: u32,
}

impl PyMctsResult {
    fn from_mcts_result(result: MctsResult, state: &State) -> Self {
        PyMctsResult {
            s1: result
                .s1
                .iter()
                .map(|r| PyMctsSideResult::from_mcts_side_result(r.clone(), &state.side_one))
                .collect(),
            s2: result
                .s2
                .iter()
                .map(|r| PyMctsSideResult::from_mcts_side_result(r.clone(), &state.side_two))
                .collect(),
            iteration_count: result.iteration_count,
        }
    }
}

#[derive(Clone)]
#[pyclass(get_all)]
struct PyIterativeDeepeningResult {
    s1: Vec<String>,
    s2: Vec<String>,
    matrix: Vec<f32>,
    depth_searched: i8,
}

impl PyIterativeDeepeningResult {
    fn from_iterative_deepening_result(
        result: (Vec<MoveChoice>, Vec<MoveChoice>, Vec<f32>, i8),
        state: &State,
    ) -> Self {
        PyIterativeDeepeningResult {
            s1: result
                .0
                .iter()
                .map(|c| movechoice_to_string(&state.side_one, c))
                .collect(),
            s2: result
                .1
                .iter()
                .map(|c| movechoice_to_string(&state.side_two, c))
                .collect(),
            matrix: result.2,
            depth_searched: result.3,
        }
    }
}

#[pyfunction]
fn mcts(py_state: PyState, duration_ms: u64) -> PyResult<PyMctsResult> {
    let mut state: State = py_state.into();
    let duration = Duration::from_millis(duration_ms);
    let (s1_options, s2_options) = state.root_get_all_options();
    let mcts_result = perform_mcts(&mut state, s1_options, s2_options, duration);

    let py_mcts_result = PyMctsResult::from_mcts_result(mcts_result, &state);
    Ok(py_mcts_result)
}

#[pyfunction]
fn id(py_state: PyState, duration_ms: u64) -> PyResult<PyIterativeDeepeningResult> {
    let mut state: State = py_state.into();
    let duration = Duration::from_millis(duration_ms);
    let (s1_options, s2_options) = state.root_get_all_options();
    let id_result = iterative_deepen_expectiminimax(&mut state, s1_options, s2_options, duration);

    let py_id_result =
        PyIterativeDeepeningResult::from_iterative_deepening_result(id_result, &state);
    Ok(py_id_result)
}

#[derive(Clone)]
#[pyclass(name = "StateInstructions", module = "poke_engine", get_all)]
struct PyStateInstructions {
    pub percentage: f32,
    pub instruction_list: Vec<PyInstruction>,
}

impl From<StateInstructions> for PyStateInstructions {
    fn from(other: StateInstructions) -> Self {
        PyStateInstructions {
            percentage: other.percentage,
            instruction_list: other
                .instruction_list
                .into_iter()
                .map(|i| PyInstruction::from(i))
                .collect(),
        }
    }
}

impl Into<StateInstructions> for PyStateInstructions {
    fn into(self) -> StateInstructions {
        StateInstructions {
            percentage: self.percentage,
            instruction_list: self
                .instruction_list
                .into_iter()
                .map(|i| i.into())
                .collect(),
        }
    }
}

#[pymethods]
impl PyStateInstructions {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "{}: {:?}",
            self.percentage,
            self.instruction_list
                .iter()
                .map(|x| format!("{:?}", x.instruction))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

#[derive(Clone)]
#[pyclass(module = "poke_engine")]
struct PyInstruction {
    instruction: Instruction,
}

impl From<Instruction> for PyInstruction {
    fn from(instruction: Instruction) -> Self {
        PyInstruction { instruction }
    }
}

impl Into<Instruction> for PyInstruction {
    fn into(self) -> Instruction {
        self.instruction
    }
}

#[pymethods]
impl PyInstruction {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.instruction))
    }
}

#[pyfunction]
fn generate_instructions(
    py_state: PyState,
    side_one_move: String,
    side_two_move: String,
) -> PyResult<Vec<PyStateInstructions>> {
    let (s1_move, s2_move);
    let mut state: State = py_state.into();
    match MoveChoice::from_string(&side_one_move, &state.side_one) {
        Some(m) => s1_move = m,
        None => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid move for s1: {}",
                side_one_move
            )))
        }
    }
    match MoveChoice::from_string(&side_two_move, &state.side_two) {
        Some(m) => s2_move = m,
        None => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid move for s2: {}",
                side_two_move
            )))
        }
    }
    let instructions = generate_instructions_from_move_pair(&mut state, &s1_move, &s2_move, true);
    let py_instructions = instructions.iter().map(|i| i.clone().into()).collect();

    Ok(py_instructions)
}

#[pyfunction]
fn calculate_damage(
    py_state: PyState,
    side_one_move: String,
    side_two_move: String,
    side_one_moves_first: bool,
) -> PyResult<(Vec<i16>, Vec<i16>)> {
    let state: State = py_state.into();
    let (mut s1_choice, mut s2_choice);
    match MOVES.get(&Choices::from_str(side_one_move.as_str()).unwrap()) {
        Some(m) => s1_choice = m.to_owned(),
        None => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid move for s1: {}",
                side_one_move
            )))
        }
    }
    match MOVES.get(&Choices::from_str(side_two_move.as_str()).unwrap()) {
        Some(m) => s2_choice = m.to_owned(),
        None => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid move for s2: {}",
                side_one_move
            )))
        }
    }
    if side_one_move == "switch" {
        s1_choice.category = MoveCategory::Switch
    }
    if side_two_move == "switch" {
        s2_choice.category = MoveCategory::Switch
    }

    let (s1_damage_rolls, s2_damage_rolls) =
        calculate_both_damage_rolls(&state, s1_choice, s2_choice, side_one_moves_first);

    let (s1_py_rolls, s2_py_rolls);
    match s1_damage_rolls {
        Some(rolls) => s1_py_rolls = rolls,
        None => s1_py_rolls = vec![0, 0],
    }
    match s2_damage_rolls {
        Some(rolls) => s2_py_rolls = rolls,
        None => s2_py_rolls = vec![0, 0],
    }

    Ok((s1_py_rolls, s2_py_rolls))
}

#[pymodule]
#[pyo3(name = "poke_engine")]
fn py_poke_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_damage, m)?)?;
    m.add_function(wrap_pyfunction!(generate_instructions, m)?)?;
    m.add_function(wrap_pyfunction!(id, m)?)?;
    m.add_function(wrap_pyfunction!(mcts, m)?)?;
    m.add_class::<PyState>()?;
    m.add_class::<PySide>()?;
    m.add_class::<PySideConditions>()?;
    m.add_class::<PyVolatileStatusDurations>()?;
    m.add_class::<PyPokemon>()?;
    m.add_class::<PyMove>()?;
    m.add_class::<PyStateInstructions>()?;
    m.add_class::<PyInstruction>()?;
    Ok(())
}
