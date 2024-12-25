use crate::evaluate::evaluate;
use crate::generate_instructions::generate_instructions_from_move_pair;
use crate::instruction::StateInstructions;
use crate::pokemon::PokemonName;
use crate::state::{MoveChoice, State};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use smallvec::SmallVec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

// Thread-local RNG
thread_local! {
    static THREAD_RNG: RefCell<ThreadRng> = RefCell::new(thread_rng());
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-0.0125 * x).exp())
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UniqueMove {
    move_choice: MoveChoice,
    pokemon_name: PokemonName,
    is_switch: bool,
}

#[derive(Debug)]
pub struct OpponentMoveStats {
    pub visits: i64,
    pub value: f32,
    pub last_ucb: Option<f32>,
}

#[derive(Debug)]
struct MoveStatistics {
    move_choice: MoveChoice,
    visits: i64,
    total_value: f32,
    avg_value: f32,
}

#[derive(Clone, Debug)]
pub struct MoveHistoryEntry {
    our_move: MoveChoice,
    our_active: PokemonName,
    opp_move: MoveChoice,
    opp_active: PokemonName,
}

pub struct DebugMCTS {
    root: Rc<RefCell<DebugMCTSNode>>,
    max_depth_seen: Rc<RefCell<usize>>,
}

#[derive(Debug)]
pub struct DebugMCTSNode {
    pub parent: Option<Weak<RefCell<DebugMCTSNode>>>,
    pub children: HashMap<MoveChoice, Rc<RefCell<DebugMCTSNode>>>,
    pub opponent_move_stats: HashMap<UniqueMove, OpponentMoveStats>,
    pub visits: i64,
    pub value: f32,
    pub our_move: Option<MoveChoice>,
    pub depth: i32,
    pub creation_time: std::time::Instant,
    pub last_simulation_score: Option<f32>,
    pub actual_opponent_move: Option<UniqueMove>,
    pub original_active: Option<PokemonName>,
}

impl DebugMCTS {
    pub fn new() -> Self {
        DebugMCTS {
            root: Rc::new(RefCell::new(DebugMCTSNode::new(0))),
            max_depth_seen: Rc::new(RefCell::new(0)),
        }
    }
}

impl DebugMCTSNode {
    const NONE_MOVE: &'static str = "none";
    const TERA_SUFFIX: &'static str = "-tera";

    pub fn new(depth: i32) -> Self {
        DebugMCTSNode {
            parent: None,
            children: HashMap::new(),
            opponent_move_stats: HashMap::new(),
            visits: 0,
            value: 0.0,
            our_move: None,
            depth,
            creation_time: std::time::Instant::now(),
            last_simulation_score: None,
            actual_opponent_move: None,
            original_active: None,
        }
    }

    pub fn format_available_moves(
        state: &State,
        moves: &[MoveChoice],
        is_side_one: bool,
    ) -> String {
        moves
            .iter()
            .map(|m| Self::format_move_name(state, m, is_side_one))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn format_move_name(state: &State, mov: &MoveChoice, is_side_one: bool) -> String {
        let side = if is_side_one {
            &state.side_one
        } else {
            &state.side_two
        };

        match mov {
            MoveChoice::None => Self::NONE_MOVE.to_string(),
            MoveChoice::Move(index) => {
                format!("{:?}", side.get_active_immutable().moves[index].id).to_lowercase()
            }
            MoveChoice::MoveTera(index) => format!(
                "{:?}{}",
                side.get_active_immutable().moves[index].id,
                Self::TERA_SUFFIX
            )
            .to_lowercase(),
            MoveChoice::Switch(index) => {
                let active_pokemon = side.get_active_immutable().id;
                let target_pokemon = side.pokemon[*index].id;
                format!("switch {} (from {})", target_pokemon, active_pokemon)
            }
        }
    }

    fn format_move_history_entry(state: &State, entry: &MoveHistoryEntry) -> String {
        format!(
            "{} vs {}",
            Self::format_move_with_context(state, &entry.our_move, entry.our_active, true),
            Self::format_move_with_context(state, &entry.opp_move, entry.opp_active, false)
        )
    }

    fn format_move_with_context(
        state: &State,
        mov: &MoveChoice,
        active: PokemonName,
        is_side_one: bool,
    ) -> String {
        let side = if is_side_one {
            &state.side_one
        } else {
            &state.side_two
        };

        match mov {
            MoveChoice::None => Self::NONE_MOVE.to_string(),
            MoveChoice::Move(index) | MoveChoice::MoveTera(index) => {
                let pokemon = side
                    .pokemon
                    .into_iter()
                    .find(|p| p.id == active)
                    .unwrap_or_else(|| side.get_active_immutable());

                let suffix = if matches!(mov, MoveChoice::MoveTera(_)) {
                    Self::TERA_SUFFIX
                } else {
                    ""
                };

                format!("{:?}{}", pokemon.moves[index].id, suffix).to_lowercase()
            }
            MoveChoice::Switch(index) => {
                let target_pokemon = side.pokemon[*index].id;
                format!("switch {} (from {})", target_pokemon, active)
            }
        }
    }

    #[inline(always)]
    pub fn ucb1_score(&self, parent_visits: i64) -> f32 {
        if self.visits == 0 {
            return f32::INFINITY;
        }
        let visits_f = self.visits as f32;
        let exploit = self.value / visits_f;
        let explore = (2.0 * (parent_visits as f32).ln() / visits_f).sqrt();
        exploit + explore
    }

    pub fn select_opponent_move(
        &mut self,
        available_moves: &[MoveChoice],
        state: &State,
        mut debug_file: Option<&mut File>,
    ) -> MoveChoice {
        let current_opponent_active = state.side_two.get_active_immutable().id;

        // Identify untried moves
        let untried_moves: Vec<_> = available_moves
            .iter()
            .filter(|m| {
                let unique_move = UniqueMove {
                    move_choice: (*m).clone(),
                    pokemon_name: current_opponent_active,
                    is_switch: matches!(m, MoveChoice::Switch(_)),
                };
                !self.opponent_move_stats.contains_key(&unique_move)
            })
            .collect();

        if !untried_moves.is_empty() {
            let chosen = untried_moves[thread_rng().gen_range(0..untried_moves.len())].clone();
            let unique_move = UniqueMove {
                move_choice: chosen.clone(),
                pokemon_name: current_opponent_active,
                is_switch: matches!(chosen, MoveChoice::Switch(_)),
            };

            if let Some(file) = debug_file.as_deref_mut() {
                writeln!(
                    file,
                    "\nSelecting untried opponent move: {}",
                    Self::format_move_name(state, &chosen, false)
                )
                .unwrap();
            }

            self.opponent_move_stats.insert(
                unique_move,
                OpponentMoveStats {
                    visits: 0,
                    value: 0.0,
                    last_ucb: None,
                },
            );

            return chosen;
        }

        // Calculate UCB1 scores
        let total_visits = self
            .opponent_move_stats
            .values()
            .map(|stats| stats.visits)
            .sum::<i64>();

        let mut best_score = f32::NEG_INFINITY;
        let mut best_move = None;

        for move_choice in available_moves {
            let unique_move = UniqueMove {
                move_choice: move_choice.clone(),
                pokemon_name: current_opponent_active,
                is_switch: matches!(move_choice, MoveChoice::Switch(_)),
            };

            if let Some(stats) = self.opponent_move_stats.get_mut(&unique_move) {
                let exploitation = 1.0 - (stats.value / stats.visits as f32);
                let exploration = (2.0 * (total_visits as f32).ln() / stats.visits as f32).sqrt();
                let ucb_score = exploitation + exploration;
                stats.last_ucb = Some(ucb_score);

                if ucb_score > best_score {
                    best_score = ucb_score;
                    best_move = Some(move_choice.clone());
                }
            }
        }

        best_move.unwrap_or_else(|| available_moves[0].clone())
    }

    fn select_and_expand(
        node: Rc<RefCell<DebugMCTSNode>>,
        mut debug_file: Option<&mut File>,
        state: &mut State,
        max_depth_seen: &Rc<RefCell<usize>>,
    ) -> (Rc<RefCell<DebugMCTSNode>>, SmallVec<[MoveHistoryEntry; 16]>) {
        let mut current_node = node;
        let mut move_history = SmallVec::new();

        if let Some(file) = debug_file.as_deref_mut() {
            writeln!(file, "\nInitial state:").unwrap();
            writeln!(
                file,
                "Active Pokemon: {:?}",
                state.side_one.get_active_immutable().id
            )
            .unwrap();
            let (our_moves, _) = state.get_all_options();
            writeln!(
                file,
                "Available moves: {}",
                DebugMCTSNode::format_available_moves(state, &our_moves, true)
            )
            .unwrap();
        }

        loop {
            {
                let node_guard = current_node.borrow();
                let mut depth_seen = max_depth_seen.borrow_mut();
                *depth_seen = (*depth_seen).max(node_guard.depth as usize);
            }

            let (our_moves, opp_moves) = state.get_all_options();
            if state.battle_is_over() != 0.0 || (our_moves.is_empty() && opp_moves.is_empty()) {
                return (current_node, move_history);
            }

            let valid_our_moves = if our_moves.contains(&MoveChoice::None) {
                vec![MoveChoice::None]
            } else {
                our_moves
                    .iter()
                    .filter(|&m| !matches!(m, MoveChoice::None))
                    .cloned()
                    .collect::<Vec<_>>()
            };

            let valid_opp_moves = if opp_moves.contains(&MoveChoice::None) {
                let switch_moves: Vec<_> = opp_moves
                    .iter()
                    .filter(|&m| matches!(m, MoveChoice::Switch(_)))
                    .cloned()
                    .collect();
                if !switch_moves.is_empty() {
                    switch_moves
                } else {
                    vec![MoveChoice::None]
                }
            } else if our_moves.iter().all(|m| matches!(m, MoveChoice::Switch(_))) {
                vec![MoveChoice::None]
            } else {
                opp_moves
                    .iter()
                    .filter(|&m| !matches!(m, MoveChoice::None))
                    .cloned()
                    .collect::<Vec<_>>()
            };

            // Check for untried moves
            let untried_move = {
                let node_guard = current_node.borrow();
                valid_our_moves
                    .iter()
                    .find(|m| !node_guard.children.contains_key(*m))
                    .cloned()
            };

            if let Some(our_move) = untried_move {
                let current_opponent_active = state.side_two.get_active_immutable().id;
                let current_our_active = state.side_one.get_active_immutable().id;

                let opp_move = if valid_opp_moves
                    .iter()
                    .all(|m| matches!(m, MoveChoice::Switch(_)))
                {
                    valid_opp_moves[0].clone()
                } else {
                    current_node.borrow_mut().select_opponent_move(
                        &valid_opp_moves,
                        state,
                        debug_file.as_deref_mut(),
                    )
                };

                let unique_move = UniqueMove {
                    move_choice: opp_move.clone(),
                    pokemon_name: current_opponent_active,
                    is_switch: matches!(opp_move, MoveChoice::Switch(_)),
                };

                let instructions =
                    generate_instructions_from_move_pair(state, &our_move, &opp_move, true);
                let chosen_inst = sample_instruction(&instructions);
                state.apply_instructions(&chosen_inst.instruction_list);

                move_history.push(MoveHistoryEntry {
                    our_move: our_move.clone(),
                    our_active: current_our_active,
                    opp_move: opp_move.clone(),
                    opp_active: current_opponent_active,
                });

                let new_depth = current_node.borrow().depth + 1;
                let mut new_node = DebugMCTSNode::new(new_depth);
                new_node.our_move = Some(our_move.clone());
                new_node.parent = Some(Rc::downgrade(&current_node));
                new_node.actual_opponent_move = Some(unique_move);
                new_node.original_active = Some(current_our_active);

                let new_node_rc = Rc::new(RefCell::new(new_node));
                current_node
                    .borrow_mut()
                    .children
                    .insert(our_move, new_node_rc.clone());

                return (new_node_rc, move_history);
            }

            // Selection phase
            let selection_result = {
                let node_guard = current_node.borrow();
                let mut best_move = None;
                let mut best_score = f32::NEG_INFINITY;
                let mut best_node = None;

                for (move_choice, child) in &node_guard.children {
                    if valid_our_moves.contains(move_choice) {
                        let child_guard = child.borrow();
                        let score = child_guard.ucb1_score(node_guard.visits);

                        if score > best_score {
                            best_score = score;
                            best_move = Some(move_choice.clone());
                            best_node = Some(Rc::clone(child));
                        }
                    }
                }
                (best_move, best_node)
            };

            let (selected_move, next_node) = match selection_result {
                (Some(mov), Some(node)) => (mov, node),
                _ => return (current_node, move_history),
            };

            let selected_opp_move = if valid_opp_moves
                .iter()
                .all(|m| matches!(m, MoveChoice::Switch(_)))
            {
                valid_opp_moves[0].clone()
            } else {
                current_node.borrow_mut().select_opponent_move(
                    &valid_opp_moves,
                    state,
                    debug_file.as_deref_mut(),
                )
            };

            let current_opponent_active = state.side_two.get_active_immutable().id;
            let current_our_active = state.side_one.get_active_immutable().id;

            let unique_move = UniqueMove {
                move_choice: selected_opp_move.clone(),
                pokemon_name: current_opponent_active,
                is_switch: matches!(selected_opp_move, MoveChoice::Switch(_)),
            };

            let instructions = generate_instructions_from_move_pair(
                state,
                &selected_move,
                &selected_opp_move,
                true,
            );
            let chosen_inst = sample_instruction(&instructions);
            state.apply_instructions(&chosen_inst.instruction_list);

            move_history.push(MoveHistoryEntry {
                our_move: selected_move,
                our_active: current_our_active,
                opp_move: selected_opp_move,
                opp_active: current_opponent_active,
            });

            next_node.borrow_mut().actual_opponent_move = Some(unique_move);
            current_node = next_node;
        }
    }
    fn backpropagate(
        node: Rc<RefCell<DebugMCTSNode>>,
        score: f32,
        move_history: &[MoveHistoryEntry],
    ) {
        let mut current = node;

        // Update the leaf node
        {
            let mut node_guard = current.borrow_mut();
            node_guard.visits += 1;
            node_guard.value += score;
            node_guard.last_simulation_score = Some(score);
        }

        // Walk back up the tree
        for entry in move_history.iter().rev() {
            let parent = {
                let node_guard = current.borrow();
                node_guard.parent.as_ref().and_then(|p| p.upgrade())
            };

            if let Some(parent_node) = parent {
                {
                    let mut parent_guard = parent_node.borrow_mut();
                    parent_guard.visits += 1;
                    parent_guard.value += score;

                    let unique_move = UniqueMove {
                        move_choice: entry.opp_move.clone(),
                        pokemon_name: entry.opp_active,
                        is_switch: matches!(entry.opp_move, MoveChoice::Switch(_)),
                    };

                    let stats = parent_guard
                        .opponent_move_stats
                        .entry(unique_move)
                        .or_insert(OpponentMoveStats {
                            visits: 0,
                            value: 0.0,
                            last_ucb: None,
                        });
                    stats.visits += 1;
                    stats.value += score;
                }
                current = parent_node;
            } else {
                break;
            }
        }
    }

    fn collect_move_statistics(&self, our_moves: &[MoveChoice]) -> Vec<MoveStatistics> {
        let mut move_stats = Vec::with_capacity(our_moves.len());

        for mov in our_moves {
            if let Some(child) = self.children.get(mov) {
                let child_guard = child.borrow();
                let avg_value = if child_guard.visits > 0 {
                    child_guard.value / child_guard.visits as f32
                } else {
                    0.0
                };

                move_stats.push(MoveStatistics {
                    move_choice: mov.clone(),
                    visits: child_guard.visits,
                    total_value: child_guard.value,
                    avg_value,
                });
            } else {
                move_stats.push(MoveStatistics {
                    move_choice: mov.clone(),
                    visits: 0,
                    total_value: 0.0,
                    avg_value: 0.0,
                });
            }
        }

        move_stats.sort_by_key(|stat| -stat.visits);
        move_stats
    }

    fn visualize(&self, state: &mut State, output_file: Option<&mut File>) -> String {
        let mut output = String::new();
        self.visualize_internal(&mut output, "", true, state);

        if let Some(file) = output_file {
            writeln!(file, "{}", output).expect("Failed to write to output file");
        }

        output
    }

    fn visualize_internal(
        &self,
        output: &mut String,
        prefix: &str,
        is_last: bool,
        state: &mut State,
    ) {
        let marker = if is_last { "└── " } else { "├── " };
        output.push_str(&format!("{}{}", prefix, marker));

        let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });

        if self.parent.is_none() {
            output.push_str("Root\n");
        } else if let Some(our_move) = &self.our_move {
            let avg = if self.visits > 0 {
                self.value / self.visits as f32
            } else {
                0.0
            };

            let move_str = if let Some(active) = self.original_active {
                Self::format_move_with_context(state, our_move, active, true)
            } else {
                Self::format_move_name(state, our_move, true)
            };

            output.push_str(&format!(
                "{} (V:{} W:{:.1} A:{:.3} D:{} T:{:?})\n",
                move_str,
                self.visits,
                self.value,
                avg,
                self.depth,
                self.creation_time.elapsed()
            ));
        }

        if !self.opponent_move_stats.is_empty() {
            self.print_opponent_move_stats(output, &new_prefix, state);
        }

        let mut children: Vec<_> = self.children.iter().collect();
        children.sort_by(|(_, a), (_, b)| {
            let a_ref = a.borrow();
            let b_ref = b.borrow();
            b_ref
                .visits
                .cmp(&a_ref.visits)
                .then(b_ref.creation_time.cmp(&a_ref.creation_time))
        });

        for (i, (_, child)) in children.iter().enumerate() {
            let child_ref = child.borrow();
            let mut child_state = state.clone();

            if let Some(our_move) = &child_ref.our_move {
                if let Some(actual_opp_move) = &child_ref.actual_opponent_move {
                    let instructions = generate_instructions_from_move_pair(
                        &mut child_state,
                        our_move,
                        &actual_opp_move.move_choice,
                        true,
                    );
                    if !instructions.is_empty() {
                        child_state.apply_instructions(&instructions[0].instruction_list);
                    }
                }
            }

            child_ref.visualize_internal(
                output,
                &new_prefix,
                i == children.len() - 1,
                &mut child_state,
            );
        }
    }

    fn print_opponent_move_stats(&self, output: &mut String, prefix: &str, state: &State) {
        if !self.opponent_move_stats.is_empty() {
            let mut moves: Vec<_> = self.opponent_move_stats.iter().collect();
            moves.sort_by_key(|(_, stats)| -stats.visits);

            output.push_str(&format!("{}Opponent moves:\n", prefix));

            for (unique_move, stats) in moves {
                let avg = if stats.visits > 0 {
                    stats.value / stats.visits as f32
                } else {
                    0.0
                };

                let move_desc = if unique_move.is_switch {
                    if let MoveChoice::Switch(idx) = &unique_move.move_choice {
                        format!(
                            "switch {} (from {})",
                            state.side_two.pokemon[*idx].id, unique_move.pokemon_name
                        )
                    } else {
                        unreachable!()
                    }
                } else {
                    format!(
                        "{} (from {})",
                        Self::format_move_with_context(
                            state,
                            &unique_move.move_choice,
                            unique_move.pokemon_name,
                            false
                        ),
                        unique_move.pokemon_name
                    )
                };

                output.push_str(&format!(
                    "{}  {}: V:{} W:{:.1} A:{:.3}{}\n",
                    prefix,
                    move_desc,
                    stats.visits,
                    stats.value,
                    avg,
                    stats
                        .last_ucb
                        .map_or(String::new(), |u| format!(" UCB:{:.3}", u))
                ));
            }
        }
    }
}

fn sample_instruction(instructions: &[StateInstructions]) -> &StateInstructions {
    if instructions.len() == 1 {
        return &instructions[0];
    }

    let mut weights = Vec::with_capacity(instructions.len());
    weights.extend(instructions.iter().map(|i| i.percentage as f64));

    THREAD_RNG.with(|rng| match WeightedIndex::new(&weights) {
        Ok(dist) => &instructions[dist.sample(&mut *rng.borrow_mut())],
        Err(_) => &instructions[0],
    })
}

pub fn perform_mcts_search_st(
    state: &mut State,
    iterations: Option<u32>,
    time_limit: Option<Duration>,
) -> MoveChoice {
    let start_time = Instant::now();
    let mcts = DebugMCTS::new();
    let root_eval = evaluate(state);

    while !should_stop(&start_time, iterations, time_limit, &mcts) {
        let mut sim_state = state.clone();
        let (selected_node, move_history) = DebugMCTSNode::select_and_expand(
            Rc::clone(&mcts.root),
            None,
            &mut sim_state,
            &mcts.max_depth_seen,
        );

        let score = if sim_state.battle_is_over() != 0.0 {
            if sim_state.battle_is_over() > 0.0 {
                1.0
            } else {
                0.0
            }
        } else {
            sigmoid(evaluate(&sim_state) - root_eval)
        };

        DebugMCTSNode::backpropagate(selected_node, score, &move_history);
    }

    // Print statistics
    let (our_moves, _) = state.get_all_options();
    let root = mcts.root.borrow();
    print_move_statistics(&root, &our_moves, state);
    println!(
        "Time elapsed: {:?}, Maximum depth explored: {}",
        start_time.elapsed(),
        *mcts.max_depth_seen.borrow()
    );

    choose_best_move(&root, state)
}

pub fn debug_mcts_st(
    state: &mut State,
    max_iterations: u32,
    print_frequency: u32,
    output_path: Option<&str>,
) -> MoveChoice {
    let start_time = Instant::now();
    let mut output_file =
        output_path.map(|path| File::create(path).expect("Failed to create output file"));
    let result = internal_mcts_search(
        state,
        max_iterations,
        Some(print_frequency),
        output_file.as_mut(),
    );

    let (our_moves, _) = state.get_all_options();
    print_move_statistics(&result.root.borrow(), &our_moves, state);

    if let Some(ref mut file) = output_file {
        print_move_statistics_to_file(&result.root.borrow(), &our_moves, state, file);
    }

    println!(
        "Time elapsed: {:?}, Maximum depth explored: {}",
        start_time.elapsed(),
        *result.max_depth_seen.borrow()
    );
    let root = result.root.borrow();
    choose_best_move(&root, state)
}

fn internal_mcts_search(
    state: &mut State,
    max_iterations: u32,
    print_frequency: Option<u32>,
    mut debug_file: Option<&mut File>,
) -> DebugMCTS {
    let mcts = DebugMCTS::new();
    let root_ref = Rc::clone(&mcts.root);
    let max_depth_ref = Rc::clone(&mcts.max_depth_seen);
    let root_eval = evaluate(state);

    for iteration in 0..max_iterations {
        if let Some(file) = debug_file.as_deref_mut() {
            writeln!(
                file,
                "Iteration {}, Root visits: {}",
                iteration,
                mcts.root.borrow().visits
            )
            .unwrap();
        }

        if let Some(freq) = print_frequency {
            if iteration % freq == 0 {
                if let Some(file) = debug_file.as_deref_mut() {
                    writeln!(file, "\n=== Iteration {} ===", iteration).unwrap();
                    mcts.root.borrow().visualize(state, Some(file));
                }
            }
        }

        let mut sim_state = state.clone();
        let (selected_node, move_history) = DebugMCTSNode::select_and_expand(
            Rc::clone(&root_ref),
            debug_file.as_deref_mut(),
            &mut sim_state,
            &max_depth_ref,
        );

        if let Some(file) = debug_file.as_deref_mut() {
            writeln!(file, "\nState after move sequence:").unwrap();
            writeln!(file, "{}", sim_state.visualize()).unwrap();
        }

        let score = if sim_state.battle_is_over() == 0.0 {
            let eval = evaluate(&sim_state);
            sigmoid(eval - root_eval)
        } else if sim_state.battle_is_over() > 0.0 {
            1.0
        } else {
            0.0
        };
        if let Some(file) = debug_file.as_deref_mut() {
            writeln!(file, "Move sequence:").unwrap();
            for entry in &move_history {
                writeln!(
                    file,
                    "  {}",
                    DebugMCTSNode::format_move_history_entry(&sim_state, entry)
                )
                .unwrap();
            }
        }
        DebugMCTSNode::backpropagate(selected_node, score, &move_history);
    }

    mcts
}

fn print_move_statistics(root: &DebugMCTSNode, available_moves: &[MoveChoice], state: &State) {
    let stats = root.collect_move_statistics(available_moves);
    let total_visits: i64 = stats.iter().map(|s| s.visits).sum();

    println!("\nMove Statistics:");
    println!(
        "{:<35}{:>12}{:>12}{:>10}",
        "Move", "Visits", "Avg Value", "% Visits"
    );
    println!("{:-<69}", "");

    for stat in stats {
        let visit_percentage = if total_visits > 0 {
            (stat.visits as f32 / total_visits as f32) * 100.0
        } else {
            0.0
        };

        let move_name = DebugMCTSNode::format_move_name(state, &stat.move_choice, true);
        println!(
            "{:<35}{:>12}{:>12.3}{:>9.1}%",
            move_name, stat.visits, stat.avg_value, visit_percentage
        );
    }
}

fn print_move_statistics_to_file(
    root: &DebugMCTSNode,
    available_moves: &[MoveChoice],
    state: &State,
    file: &mut File,
) {
    let stats = root.collect_move_statistics(available_moves);
    writeln!(file, "\nDetailed Move Statistics:").unwrap();

    for stat in stats {
        let move_name = DebugMCTSNode::format_move_name(state, &stat.move_choice, true);
        writeln!(
            file,
            "{}: visits={}, total_value={:.3}, avg_value={:.3}",
            move_name, stat.visits, stat.total_value, stat.avg_value
        )
        .unwrap();
    }
}

fn choose_best_move(root: &DebugMCTSNode, state: &State) -> MoveChoice {
    let (our_moves, _) = state.get_all_options();
    let mut best_visits = 0;
    let mut best_moves = Vec::new();

    for mov in &our_moves {
        if let Some(child) = root.children.get(mov) {
            let visits = child.borrow().visits;
            if visits > best_visits {
                best_visits = visits;
                best_moves.clear();
                best_moves.push(mov);
            } else if visits == best_visits {
                best_moves.push(mov);
            }
        }
    }

    let mut rng = thread_rng();
    best_moves[rng.gen_range(0..best_moves.len())].clone()
}

fn should_stop(
    start_time: &Instant,
    iterations: Option<u32>,
    time_limit: Option<Duration>,
    mcts: &DebugMCTS,
) -> bool {
    let visits = mcts.root.borrow().visits;

    if let Some(max_iter) = iterations {
        if visits >= max_iter as i64 {
            return true;
        }
    }

    if let Some(limit) = time_limit {
        if start_time.elapsed() >= limit {
            return true;
        }
    }

    visits >= 10_000_000
}
