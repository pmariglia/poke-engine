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
