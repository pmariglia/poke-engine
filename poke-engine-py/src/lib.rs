use pyo3::prelude::*;
use pyo3::{pyfunction, pymethods, pymodule, wrap_pyfunction, Bound, PyResult};
use std::collections::HashSet;

use poke_engine::abilities::Abilities;
use poke_engine::choices::{Choices, MoveCategory, MOVES};
use poke_engine::generate_instructions::{
    calculate_both_damage_rolls, generate_instructions_from_move_pair,
};
use poke_engine::instruction::{Instruction, StateInstructions};
use poke_engine::io::io_get_all_options;
use poke_engine::items::Items;
use poke_engine::mcts::{perform_mcts, MctsResult, MctsSideResult};
use poke_engine::pokemon::PokemonName;
use poke_engine::search::iterative_deepen_expectiminimax;
use poke_engine::state::{
    LastUsedMove, Move, MoveChoice, Pokemon, PokemonIndex, PokemonMoves, PokemonStatus,
    PokemonType, PokemonVolatileStatus, Side, SideConditions, SidePokemon, State, StateTerrain,
    StateTrickRoom, StateWeather, Terrain, Weather,
};
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone)]
#[pyclass(name = "State")]
pub struct PyState {
    pub state: State,
}

#[pymethods]
impl PyState {
    #[new]
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
        let mut state = State {
            side_one: side_one.create_side(),
            side_two: side_two.create_side(),
            weather: StateWeather {
                weather_type: Weather::from_str(&weather).unwrap(),
                turns_remaining: weather_turns_remaining,
            },
            terrain: StateTerrain {
                terrain_type: Terrain::from_str(&terrain).unwrap(),
                turns_remaining: terrain_turns_remaining,
            },
            trick_room: StateTrickRoom {
                active: trick_room,
                turns_remaining: trick_room_turns_remaining,
            },
            team_preview,
            use_damage_dealt: false,
            use_last_used_move: false,
        };
        state.set_conditional_mechanics();
        PyState { state }
    }

    fn apply_one_instruction(&mut self, instruction: PyInstruction) {
        self.state.apply_one_instruction(&instruction.instruction);
    }

    fn apply_instructions(&mut self, instructions: Vec<PyInstruction>) {
        for instruction in instructions {
            self.apply_one_instruction(instruction);
        }
    }

    fn to_string(&self) -> String {
        self.state.serialize()
    }

    #[getter]
    fn get_side_one(&self) -> PyResult<PySide> {
        Ok(PySide {
            side: self.state.side_one.clone(),
        })
    }

    #[getter]
    fn get_side_two(&self) -> PyResult<PySide> {
        Ok(PySide {
            side: self.state.side_two.clone(),
        })
    }

    #[getter]
    fn get_weather(&self) -> String {
        self.state.weather.weather_type.to_string()
    }

    #[getter]
    fn get_weather_turns_remaining(&self) -> i8 {
        self.state.weather.turns_remaining
    }

    #[getter]
    fn get_terrain(&self) -> String {
        self.state.terrain.terrain_type.to_string()
    }

    #[getter]
    fn get_terrain_turns_remaining(&self) -> i8 {
        self.state.terrain.turns_remaining
    }

    #[getter]
    fn get_trick_room(&self) -> bool {
        self.state.trick_room.active
    }

    #[getter]
    fn get_trick_room_turns_remaining(&self) -> i8 {
        self.state.trick_room.turns_remaining
    }

    #[getter]
    fn get_team_preview(&self) -> bool {
        self.state.team_preview
    }
}

#[derive(Clone)]
#[pyclass(name = "Side")]
pub struct PySide {
    pub side: Side,
}

impl PySide {
    fn create_side(&self) -> Side {
        self.side.clone()
    }
}

#[pymethods]
impl PySide {
    #[new]
    fn new(
        active_index: String,
        baton_passing: bool,
        mut pokemon: Vec<PyPokemon>,
        side_conditions: PySideConditions,
        wish: (i8, i16),
        future_sight: (i8, String),
        force_switch: bool,
        force_trapped: bool,
        slow_uturn_move: bool,
        volatile_statuses: Vec<String>,
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
        let mut vs_hashset = HashSet::new();
        for vs in volatile_statuses {
            vs_hashset.insert(PokemonVolatileStatus::from_str(&vs).unwrap());
        }
        while pokemon.len() < 6 {
            pokemon.push(PyPokemon::create_fainted());
        }
        PySide {
            side: Side {
                active_index: PokemonIndex::deserialize(&active_index),
                baton_passing,
                pokemon: SidePokemon {
                    p0: pokemon[0].create_pokemon(),
                    p1: pokemon[1].create_pokemon(),
                    p2: pokemon[2].create_pokemon(),
                    p3: pokemon[3].create_pokemon(),
                    p4: pokemon[4].create_pokemon(),
                    p5: pokemon[5].create_pokemon(),
                },
                side_conditions: side_conditions.create_side_conditions(),
                wish,
                future_sight: (future_sight.0, PokemonIndex::deserialize(&future_sight.1)),
                force_switch,
                force_trapped,
                slow_uturn_move,
                volatile_statuses: vs_hashset,
                substitute_health,
                attack_boost,
                defense_boost,
                special_attack_boost,
                special_defense_boost,
                speed_boost,
                accuracy_boost,
                evasion_boost,
                last_used_move: LastUsedMove::deserialize(&last_used_move),
                damage_dealt: Default::default(),
                switch_out_move_second_saved_move: Choices::from_str(
                    &switch_out_move_second_saved_move,
                )
                .unwrap(),
            },
        }
    }

    #[getter]
    fn get_active_index(&self) -> String {
        self.side.active_index.serialize()
    }

    #[getter]
    fn get_baton_passing(&self) -> bool {
        self.side.baton_passing
    }

    #[getter]
    fn get_pokemon(&self) -> Vec<PyPokemon> {
        vec![
            PyPokemon {
                pokemon: self.side.pokemon.p0.clone(),
            },
            PyPokemon {
                pokemon: self.side.pokemon.p1.clone(),
            },
            PyPokemon {
                pokemon: self.side.pokemon.p2.clone(),
            },
            PyPokemon {
                pokemon: self.side.pokemon.p3.clone(),
            },
            PyPokemon {
                pokemon: self.side.pokemon.p4.clone(),
            },
            PyPokemon {
                pokemon: self.side.pokemon.p5.clone(),
            },
        ]
    }

    #[getter]
    fn get_side_conditions(&self) -> PyResult<PySideConditions> {
        Ok(PySideConditions {
            side_conditions: self.side.side_conditions.clone(),
        })
    }

    #[getter]
    fn get_wish(&self) -> (i8, i16) {
        self.side.wish
    }

    #[getter]
    fn get_future_sight(&self) -> (i8, String) {
        (
            self.side.future_sight.0,
            self.side.future_sight.1.serialize(),
        )
    }

    #[getter]
    fn get_force_switch(&self) -> bool {
        self.side.force_switch
    }

    #[getter]
    fn get_force_trapped(&self) -> bool {
        self.side.force_trapped
    }

    #[getter]
    fn get_volatile_statuses(&self) -> Vec<String> {
        self.side
            .volatile_statuses
            .iter()
            .map(|vs| vs.to_string())
            .collect()
    }

    #[getter]
    fn get_substitute_health(&self) -> i16 {
        self.side.substitute_health
    }

    #[getter]
    fn get_attack_boost(&self) -> i8 {
        self.side.attack_boost
    }

    #[getter]
    fn get_defense_boost(&self) -> i8 {
        self.side.defense_boost
    }

    #[getter]
    fn get_special_attack_boost(&self) -> i8 {
        self.side.special_attack_boost
    }

    #[getter]
    fn get_special_defense_boost(&self) -> i8 {
        self.side.special_defense_boost
    }

    #[getter]
    fn get_speed_boost(&self) -> i8 {
        self.side.speed_boost
    }

    #[getter]
    fn get_accuracy_boost(&self) -> i8 {
        self.side.accuracy_boost
    }

    #[getter]
    fn get_evasion_boost(&self) -> i8 {
        self.side.evasion_boost
    }

    #[getter]
    fn get_last_used_move(&self) -> String {
        self.side.last_used_move.serialize()
    }

    #[getter]
    fn get_switch_out_move_second_saved_move(&self) -> String {
        self.side.switch_out_move_second_saved_move.to_string()
    }
}

#[derive(Clone)]
#[pyclass(name = "SideConditions")]
pub struct PySideConditions {
    pub side_conditions: SideConditions,
}

impl PySideConditions {
    fn create_side_conditions(&self) -> SideConditions {
        self.side_conditions.clone()
    }
}

#[pymethods]
impl PySideConditions {
    #[new]
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
            side_conditions: SideConditions {
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
            },
        }
    }
    #[getter]
    fn get_aurora_veil(&self) -> i8 {
        self.side_conditions.aurora_veil
    }

    #[getter]
    fn get_crafty_shield(&self) -> i8 {
        self.side_conditions.crafty_shield
    }

    #[getter]
    fn get_healing_wish(&self) -> i8 {
        self.side_conditions.healing_wish
    }

    #[getter]
    fn get_light_screen(&self) -> i8 {
        self.side_conditions.light_screen
    }

    #[getter]
    fn get_lucky_chant(&self) -> i8 {
        self.side_conditions.lucky_chant
    }

    #[getter]
    fn get_lunar_dance(&self) -> i8 {
        self.side_conditions.lunar_dance
    }

    #[getter]
    fn get_mat_block(&self) -> i8 {
        self.side_conditions.mat_block
    }

    #[getter]
    fn get_mist(&self) -> i8 {
        self.side_conditions.mist
    }

    #[getter]
    fn get_protect(&self) -> i8 {
        self.side_conditions.protect
    }

    #[getter]
    fn get_quick_guard(&self) -> i8 {
        self.side_conditions.quick_guard
    }

    #[getter]
    fn get_reflect(&self) -> i8 {
        self.side_conditions.reflect
    }

    #[getter]
    fn get_safeguard(&self) -> i8 {
        self.side_conditions.safeguard
    }

    #[getter]
    fn get_spikes(&self) -> i8 {
        self.side_conditions.spikes
    }

    #[getter]
    fn get_stealth_rock(&self) -> i8 {
        self.side_conditions.stealth_rock
    }

    #[getter]
    fn get_sticky_web(&self) -> i8 {
        self.side_conditions.sticky_web
    }

    #[getter]
    fn get_tailwind(&self) -> i8 {
        self.side_conditions.tailwind
    }

    #[getter]
    fn get_toxic_count(&self) -> i8 {
        self.side_conditions.toxic_count
    }

    #[getter]
    fn get_toxic_spikes(&self) -> i8 {
        self.side_conditions.toxic_spikes
    }

    #[getter]
    fn get_wide_guard(&self) -> i8 {
        self.side_conditions.wide_guard
    }
}

#[derive(Clone)]
#[pyclass(name = "Pokemon")]
pub struct PyPokemon {
    pub pokemon: Pokemon,
}

impl PyPokemon {
    fn create_pokemon(&self) -> Pokemon {
        self.pokemon.clone()
    }
    fn create_fainted() -> PyPokemon {
        let mut pkmn = Pokemon::default();
        pkmn.hp = 0;
        PyPokemon { pokemon: pkmn }
    }
}

#[pymethods]
impl PyPokemon {
    #[new]
    fn new(
        id: String,
        level: i8,
        types: [String; 2],
        hp: i16,
        maxhp: i16,
        ability: String,
        item: String,
        attack: i16,
        defense: i16,
        special_attack: i16,
        special_defense: i16,
        speed: i16,
        status: String,
        rest_turns: i8,
        sleep_turns: i8,
        weight_kg: f32,
        mut moves: Vec<PyMove>,
        terastallized: bool,
        tera_type: String,
    ) -> Self {
        while moves.len() < 6 {
            moves.push(PyMove::create_empty_move());
        }
        PyPokemon {
            pokemon: Pokemon {
                id: PokemonName::from_str(&id).unwrap(),
                level,
                types: (
                    PokemonType::from_str(&types[0]).unwrap(),
                    PokemonType::from_str(&types[1]).unwrap(),
                ),
                hp,
                maxhp,
                ability: Abilities::from_str(&ability).unwrap(),
                item: Items::from_str(&item).unwrap(),
                attack,
                defense,
                special_attack,
                special_defense,
                speed,
                status: PokemonStatus::from_str(&status).unwrap(),
                rest_turns,
                sleep_turns,
                weight_kg,
                moves: PokemonMoves {
                    m0: moves[0].create_move(),
                    m1: moves[1].create_move(),
                    m2: moves[2].create_move(),
                    m3: moves[3].create_move(),
                    m4: moves[4].create_move(),
                    m5: moves[5].create_move(),
                },
                terastallized,
                tera_type: PokemonType::from_str(&tera_type).unwrap(),
            },
        }
    }
    #[getter]
    fn get_id(&self) -> String {
        self.pokemon.id.to_string()
    }

    #[getter]
    fn get_level(&self) -> i8 {
        self.pokemon.level
    }

    #[getter]
    fn get_types(&self) -> [String; 2] {
        [
            self.pokemon.types.0.to_string(),
            self.pokemon.types.1.to_string(),
        ]
    }

    #[getter]
    fn get_hp(&self) -> i16 {
        self.pokemon.hp
    }

    #[getter]
    fn get_maxhp(&self) -> i16 {
        self.pokemon.maxhp
    }

    #[getter]
    fn get_ability(&self) -> String {
        self.pokemon.ability.to_string()
    }

    #[getter]
    fn get_item(&self) -> String {
        self.pokemon.item.to_string()
    }

    #[getter]
    fn get_attack(&self) -> i16 {
        self.pokemon.attack
    }

    #[getter]
    fn get_defense(&self) -> i16 {
        self.pokemon.defense
    }

    #[getter]
    fn get_special_attack(&self) -> i16 {
        self.pokemon.special_attack
    }

    #[getter]
    fn get_special_defense(&self) -> i16 {
        self.pokemon.special_defense
    }

    #[getter]
    fn get_speed(&self) -> i16 {
        self.pokemon.speed
    }

    #[getter]
    fn get_status(&self) -> String {
        self.pokemon.status.to_string()
    }

    #[getter]
    fn get_rest_turns(&self) -> i8 {
        self.pokemon.rest_turns
    }

    #[getter]
    fn get_sleep_turns(&self) -> i8 {
        self.pokemon.sleep_turns
    }

    #[getter]
    fn get_weight_kg(&self) -> f32 {
        self.pokemon.weight_kg
    }

    #[getter]
    fn get_moves(&self) -> Vec<PyMove> {
        vec![
            PyMove {
                mv: self.pokemon.moves.m0.clone(),
            },
            PyMove {
                mv: self.pokemon.moves.m1.clone(),
            },
            PyMove {
                mv: self.pokemon.moves.m2.clone(),
            },
            PyMove {
                mv: self.pokemon.moves.m3.clone(),
            },
            PyMove {
                mv: self.pokemon.moves.m4.clone(),
            },
            PyMove {
                mv: self.pokemon.moves.m5.clone(),
            },
        ]
    }

    #[getter]
    fn get_terastallized(&self) -> bool {
        self.pokemon.terastallized
    }

    #[getter]
    fn get_tera_type(&self) -> String {
        self.pokemon.tera_type.to_string()
    }
}

#[derive(Clone)]
#[pyclass(name = "Move")]
pub struct PyMove {
    pub mv: Move,
}

impl PyMove {
    fn create_move(&self) -> Move {
        self.mv.clone()
    }
    fn create_empty_move() -> PyMove {
        let mut mv = Move::default();
        mv.disabled = true;
        mv.pp = 0;
        PyMove { mv }
    }
}

#[pymethods]
impl PyMove {
    #[new]
    fn new(id: String, pp: i8, disabled: bool) -> Self {
        let choice = Choices::from_str(&id).unwrap();
        PyMove {
            mv: Move {
                id: choice,
                disabled,
                pp,
                choice: MOVES.get(&choice).unwrap().clone(),
            },
        }
    }
    #[getter]
    fn get_id(&self) -> String {
        self.mv.id.to_string()
    }

    #[getter]
    fn get_disabled(&self) -> bool {
        self.mv.disabled
    }

    #[getter]
    fn get_pp(&self) -> i8 {
        self.mv.pp
    }
}

#[derive(Clone)]
#[pyclass(get_all)]
struct PyMctsSideResult {
    pub move_choice: String,
    pub total_score: f32,
    pub visits: i64,
}

impl PyMctsSideResult {
    fn from_mcts_side_result(result: MctsSideResult, side: &Side) -> Self {
        PyMctsSideResult {
            move_choice: result.move_choice.to_string(side),
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
    iteration_count: i64,
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
                .map(|c| c.to_string(&state.side_one))
                .collect(),
            s2: result
                .1
                .iter()
                .map(|c| c.to_string(&state.side_two))
                .collect(),
            matrix: result.2,
            depth_searched: result.3,
        }
    }
}

#[pyfunction]
fn mcts(mut py_state: PyState, duration_ms: u64) -> PyResult<PyMctsResult> {
    let duration = Duration::from_millis(duration_ms);
    let (s1_options, s2_options) = io_get_all_options(&py_state.state);
    let mcts_result = perform_mcts(&mut py_state.state, s1_options, s2_options, duration);

    let py_mcts_result = PyMctsResult::from_mcts_result(mcts_result, &py_state.state);
    Ok(py_mcts_result)
}

#[pyfunction]
fn id(mut py_state: PyState, duration_ms: u64) -> PyResult<PyIterativeDeepeningResult> {
    let duration = Duration::from_millis(duration_ms);
    let (s1_options, s2_options) = io_get_all_options(&py_state.state);
    let id_result =
        iterative_deepen_expectiminimax(&mut py_state.state, s1_options, s2_options, duration);

    let py_id_result =
        PyIterativeDeepeningResult::from_iterative_deepening_result(id_result, &py_state.state);
    Ok(py_id_result)
}

#[derive(Clone)]
#[pyclass(name = "Instruction")]
struct PyInstruction {
    pub instruction: Instruction,
}

#[pymethods]
impl PyInstruction {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.instruction))
    }
}

impl PyInstruction {
    fn from_instruction(instruction: Instruction) -> Self {
        PyInstruction { instruction }
    }
}

#[derive(Clone)]
#[pyclass(name = "StateInstructions")]
struct PyStateInstructions {
    #[pyo3(get)]
    pub percentage: f32,
    pub instruction_list: Vec<PyInstruction>,
}

#[pymethods]
impl PyStateInstructions {
    #[getter]
    fn get_instruction_list(&self) -> PyResult<Vec<PyInstruction>> {
        Ok(self.instruction_list.clone())
    }
}

impl PyStateInstructions {
    fn from_state_instructions(instructions: StateInstructions) -> Self {
        PyStateInstructions {
            percentage: instructions.percentage,
            instruction_list: instructions
                .instruction_list
                .into_iter()
                .map(|i| PyInstruction::from_instruction(i))
                .collect(),
        }
    }
}

#[pyfunction]
fn gi(
    mut py_state: PyState,
    side_one_move: String,
    side_two_move: String,
) -> PyResult<Vec<PyStateInstructions>> {
    let (s1_move, s2_move);
    match py_state.state.side_one.string_to_movechoice(&side_one_move) {
        Some(m) => s1_move = m,
        None => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid move for s1: {}",
                side_one_move
            )))
        }
    }
    match py_state.state.side_two.string_to_movechoice(&side_two_move) {
        Some(m) => s2_move = m,
        None => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid move for s2: {}",
                side_two_move
            )))
        }
    }
    let instructions =
        generate_instructions_from_move_pair(&mut py_state.state, &s1_move, &s2_move, true);
    let py_instructions = instructions
        .iter()
        .map(|i| PyStateInstructions::from_state_instructions(i.clone()))
        .collect();

    Ok(py_instructions)
}

#[pyfunction]
fn calculate_damage(
    py_state: PyState,
    side_one_move: String,
    side_two_move: String,
    side_one_moves_first: bool,
) -> PyResult<(Vec<i16>, Vec<i16>)> {
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
        calculate_both_damage_rolls(&py_state.state, s1_choice, s2_choice, side_one_moves_first);

    let (s1_py_rolls, s2_py_rolls);
    match s1_damage_rolls {
        Some(rolls) => s1_py_rolls = rolls,
        None => s1_py_rolls = vec![0],
    }
    match s2_damage_rolls {
        Some(rolls) => s2_py_rolls = rolls,
        None => s2_py_rolls = vec![0],
    }

    Ok((s1_py_rolls, s2_py_rolls))
}

#[pyfunction]
fn state_from_string(s: String) -> PyResult<PyState> {
    Ok(PyState {
        state: State::deserialize(&s),
    })
}

#[pymodule]
#[pyo3(name = "_poke_engine")]
fn py_poke_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(state_from_string, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_damage, m)?)?;
    m.add_function(wrap_pyfunction!(gi, m)?)?;
    m.add_function(wrap_pyfunction!(id, m)?)?;
    m.add_function(wrap_pyfunction!(mcts, m)?)?;
    m.add_class::<PyState>()?;
    m.add_class::<PySide>()?;
    m.add_class::<PySideConditions>()?;
    m.add_class::<PyPokemon>()?;
    m.add_class::<PyMove>()?;
    m.add_class::<PyStateInstructions>()?;
    m.add_class::<PyInstruction>()?;
    Ok(())
}
