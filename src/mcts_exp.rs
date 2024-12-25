use crate::evaluate::evaluate;
use crate::generate_instructions::generate_instructions_from_move_pair;
use crate::instruction::StateInstructions;
use crate::pokemon::PokemonName;
use crate::state::{MoveChoice, State};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;
// use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
// Thread-local RNG
thread_local! {
    static THREAD_RNG: RefCell<ThreadRng> = RefCell::new(thread_rng());
}

fn sigmoid(x: f32) -> f32 {
    // Tuned so that ~200 points is very close to 1.0
    1.0 / (1.0 + (-0.0125 * x).exp())
}

// fn calculate_hash<T: Hash>(t: &T) -> u64 {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }

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

// Add a new struct to store move context
#[derive(Clone, Debug)]
pub struct MoveHistoryEntry {
    our_move: MoveChoice,
    our_active: PokemonName, // Active at time of move
    opp_move: MoveChoice,
    opp_active: PokemonName, // Active at time of move
}

pub struct DebugMCTS {
    root: Arc<Mutex<DebugMCTSNode>>,
    max_depth_seen: Arc<Mutex<usize>>,
}

impl DebugMCTS {
    pub fn new() -> Self {
        DebugMCTS {
            root: Arc::new(Mutex::new(DebugMCTSNode::new(0))), // Only one Arc
            max_depth_seen: Arc::new(Mutex::new(0)),
        }
    }
}

#[derive(Debug)]
pub struct DebugMCTSNode {
    pub parent: Option<std::sync::Weak<Mutex<DebugMCTSNode>>>,
    pub children: HashMap<MoveChoice, Arc<Mutex<DebugMCTSNode>>>,
    pub opponent_move_stats: HashMap<UniqueMove, OpponentMoveStats>,
    pub visits: i64,
    pub value: f32,
    pub our_move: Option<MoveChoice>,
    pub depth: i32,
    pub creation_time: std::time::Instant,
    pub last_simulation_score: Option<f32>,
    pub actual_opponent_move: Option<UniqueMove>,
    pub original_active: Option<PokemonName>, // Store active Pokemon at time node was created
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
                // Removed the Self::SWITCH_PREFIX here since it's already part of the format
                format!("switch {} (from {})", target_pokemon, active_pokemon)
            }
        }
    }
    // Format move history entry
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
        active: PokemonName, // The source Pokémon name
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

    fn collect_move_statistics(&self, our_moves: &[MoveChoice]) -> Vec<MoveStatistics> {
        let mut move_stats = Vec::with_capacity(our_moves.len());

        for mov in our_moves {
            if let Some(child) = self.children.get(mov) {
                let child = child.lock().unwrap(); // Changed from borrow()
                let avg_value = if child.visits > 0 {
                    child.value / child.visits as f32
                } else {
                    0.0
                };

                move_stats.push(MoveStatistics {
                    move_choice: mov.clone(),
                    visits: child.visits,
                    total_value: child.value,
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
                pokemon_name: state.side_two.get_active_immutable().id,
                is_switch: matches!(move_choice, MoveChoice::Switch(_)),
            };

            if let Some(stats) = self.opponent_move_stats.get_mut(&unique_move) {
                let exploitation = 1.0 - (stats.value / stats.visits as f32); // Convert to opponent value
                let exploration = (2.0 * (total_visits as f32).ln() / stats.visits as f32).sqrt();
                let ucb_score = exploitation + exploration; // Add exploration since we're maximizing
                stats.last_ucb = Some(ucb_score);

                // if let Some(file) = debug_file.as_deref_mut() {
                //     writeln!(
                //         file,
                //         "Move {} (from {}) (V:{} W:{:.3} A:{:.3}): UCB={:.3}",
                //         Self::format_move_name(state, move_choice, false),
                //         unique_move.pokemon_name,
                //         stats.visits,
                //         stats.value,
                //         exploitation,
                //         ucb_score
                //     )
                //     .unwrap();
                // }

                if ucb_score > best_score {
                    best_score = ucb_score;
                    best_move = Some(move_choice.clone());
                }
            }
        }

        best_move.unwrap_or_else(|| available_moves[0].clone())
    }

    fn select_and_expand(
        node: Arc<Mutex<DebugMCTSNode>>,
        mut debug_file: Option<&mut File>,
        state: &mut State,
        max_depth_seen: &Arc<Mutex<usize>>,
    ) -> (Arc<Mutex<DebugMCTSNode>>, SmallVec<[MoveHistoryEntry; 16]>) {
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
            // Update max depth seen
            {
                let node_guard = current_node.lock().unwrap();
                let mut depth_seen = max_depth_seen.lock().unwrap();
                *depth_seen = (*depth_seen).max(node_guard.depth as usize);
            }

            // Get all possible moves
            let (our_moves, opp_moves) = state.get_all_options();
            // Check if game is over, either by battle outcome or no valid moves
            if state.battle_is_over() != 0.0 || (our_moves.is_empty() && opp_moves.is_empty()) {
                return (current_node, move_history);
            }

            if let Some(file) = debug_file.as_deref_mut() {
                writeln!(
                    file,
                    "\nCurrent state at depth {}:",
                    current_node.lock().unwrap().depth
                )
                .unwrap();
                writeln!(
                    file,
                    "Active Pokemon: {:?}",
                    state.side_one.get_active_immutable().id
                )
                .unwrap();
                writeln!(
                    file,
                    "Available moves: {}",
                    DebugMCTSNode::format_available_moves(state, &our_moves, true)
                )
                .unwrap();
            }

            // Truly game over - no moves available for either side
            if our_moves.is_empty() && opp_moves.is_empty() {
                let is_root = {
                    let node_guard = current_node.lock().unwrap();
                    node_guard.parent.is_none()
                };
                if !is_root {
                    return (current_node, move_history);
                }
            }

            // Handle valid moves based on current state
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

            if let Some(file) = debug_file.as_deref_mut() {
                writeln!(
                    file,
                    "Valid moves after filtering: {}",
                    DebugMCTSNode::format_available_moves(state, &valid_our_moves, true)
                )
                .unwrap();
            }

            // Add move history debugging
            if let Some(file) = debug_file.as_deref_mut() {
                writeln!(file, "\nCurrent move history:").unwrap();
                for entry in &move_history {
                    writeln!(file, "  {}", Self::format_move_history_entry(state, entry)).unwrap();
                }
            }

            // Check for untried moves
            let (untried_move, has_untried) = {
                let node_guard = current_node.lock().unwrap();
                let untried = valid_our_moves
                    .iter()
                    .find(|m| !node_guard.children.contains_key(*m))
                    .cloned();
                (untried, untried.is_some())
            };

            if has_untried {
                let our_move = untried_move.unwrap();

                if let Some(file) = debug_file.as_deref_mut() {
                    writeln!(file, "\n=== Move Selection Debug ===").unwrap();
                    writeln!(
                        file,
                        "Active Pokemon: {:?}",
                        state.side_one.get_active_immutable().id
                    )
                    .unwrap();
                    writeln!(
                        file,
                        "Selected untried move: {}",
                        DebugMCTSNode::format_move_name(state, &our_move, true)
                    )
                    .unwrap();
                    writeln!(
                        file,
                        "Current valid moves: {}",
                        DebugMCTSNode::format_available_moves(state, &valid_our_moves, true)
                    )
                    .unwrap();
                    writeln!(file, "\nMove history before update:").unwrap();
                    for entry in &move_history {
                        writeln!(file, "  {}", Self::format_move_history_entry(state, entry))
                            .unwrap();
                    }
                }

                let opp_move = if valid_opp_moves
                    .iter()
                    .all(|m| matches!(m, MoveChoice::Switch(_)))
                {
                    valid_opp_moves[0].clone()
                } else {
                    current_node.lock().unwrap().select_opponent_move(
                        &valid_opp_moves,
                        state,
                        debug_file.as_deref_mut(),
                    )
                };

                // In select_and_expand, before applying any moves:
                let current_opponent_active = state.side_two.get_active_immutable().id;
                let current_our_active = state.side_one.get_active_immutable().id;

                let unique_move = UniqueMove {
                    move_choice: opp_move.clone(),
                    pokemon_name: current_opponent_active, // Important: Store the active Pokemon at time of move
                    is_switch: matches!(opp_move, MoveChoice::Switch(_)),
                };

                // Apply the moves and update state
                let instructions =
                    generate_instructions_from_move_pair(state, &our_move, &opp_move, true);
                let chosen_inst = sample_instruction(&instructions);
                state.apply_instructions(&chosen_inst.instruction_list);

                // Record move history
                move_history.push(MoveHistoryEntry {
                    our_move: our_move,
                    our_active: current_our_active,
                    opp_move: opp_move,
                    opp_active: current_opponent_active, // Use the Pokemon that was active when move was made
                });
                if let Some(file) = debug_file.as_deref_mut() {
                    writeln!(file, "\nState after move application:").unwrap();
                    writeln!(
                        file,
                        "Active Pokemon: {:?}",
                        state.side_one.get_active_immutable().id
                    )
                    .unwrap();
                    let (curr_moves, _) = state.get_all_options();
                    writeln!(
                        file,
                        "Current moves: {}",
                        DebugMCTSNode::format_available_moves(state, &curr_moves, true)
                    )
                    .unwrap();
                }

                if let Some(file) = debug_file.as_deref_mut() {
                    writeln!(file, "\nMove history after update:").unwrap();
                    for entry in &move_history {
                        writeln!(file, "  {}", Self::format_move_history_entry(state, entry))
                            .unwrap();
                    }
                }

                let new_depth = current_node.lock().unwrap().depth + 1;
                let current_our_active = state.side_one.get_active_immutable().id;

                let mut new_node = DebugMCTSNode::new(new_depth);
                new_node.our_move = Some(our_move.clone());
                new_node.parent = Some(Arc::downgrade(&current_node));
                new_node.actual_opponent_move = Some(unique_move);
                new_node.original_active = Some(current_our_active);

                let new_node_arc = Arc::new(Mutex::new(new_node));
                {
                    let mut node_guard = current_node.lock().unwrap();
                    node_guard
                        .children
                        .insert(our_move, Arc::clone(&new_node_arc));
                }

                return (new_node_arc, move_history);
            }

            // Selection phase
            // if let Some(file) = debug_file.as_deref_mut() {
            //     writeln!(file, "\nCalculating UCB scores:").unwrap();
            // }

            let selection_result = {
                let node_guard = current_node.lock().unwrap();
                let mut best_move = None;
                let mut best_score = f32::NEG_INFINITY;
                let mut best_node = None;

                for (move_choice, child) in &node_guard.children {
                    if valid_our_moves.contains(move_choice) {
                        let child_guard = child.lock().unwrap();
                        let score = child_guard.ucb1_score(node_guard.visits);

                        // if let Some(file) = debug_file.as_deref_mut() {
                        //     writeln!(
                        //         file,
                        //         "Move {} (V:{} W:{:.3} A:{:.3}): UCB={:.3}",
                        //         Self::format_move_name(state, move_choice, true),
                        //         child_guard.visits,
                        //         child_guard.value,
                        //         child_guard.value / child_guard.visits as f32,
                        //         score
                        //     )
                        //     .unwrap();
                        // }

                        if score > best_score {
                            best_score = score;
                            best_move = Some(move_choice.clone());
                            best_node = Some(Arc::clone(child));
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
                current_node.lock().unwrap().select_opponent_move(
                    &valid_opp_moves,
                    state,
                    debug_file.as_deref_mut(),
                )
            };

            // Same changes in the selection phase:
            let current_opponent_active = state.side_two.get_active_immutable().id;
            let current_our_active = state.side_one.get_active_immutable().id;

            let unique_move = UniqueMove {
                move_choice: selected_opp_move.clone(),
                pokemon_name: state.side_two.get_active_immutable().id,
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

            // Update node with the actual opponent move
            {
                let mut next_node_guard = next_node.lock().unwrap();
                next_node_guard.actual_opponent_move = Some(unique_move);
            }

            current_node = next_node;
        }
    }
    // Update backpropagate to handle MoveHistoryEntry
    fn backpropagate(
        node: Arc<Mutex<DebugMCTSNode>>,
        score: f32,
        move_history: &[MoveHistoryEntry],
    ) {
        let mut current = Arc::clone(&node);

        // Update the leaf node
        {
            let mut node_guard = current.lock().unwrap();
            node_guard.visits += 1;
            node_guard.value += score;
            node_guard.last_simulation_score = Some(score);
        }

        // Walk back up the tree
        for entry in move_history.iter().rev() {
            let parent = {
                let node_guard = current.lock().unwrap();
                node_guard.parent.as_ref().and_then(|p| p.upgrade())
            };

            if let Some(parent_node) = parent {
                {
                    let mut parent_guard = parent_node.lock().unwrap();
                    parent_guard.visits += 1;
                    parent_guard.value += score;

                    // Update opponent stats using move history entry
                    let unique_move = UniqueMove {
                        move_choice: entry.opp_move.clone(),
                        pokemon_name: entry.opp_active,
                        is_switch: matches!(entry.opp_move, MoveChoice::Switch(_)),
                    };

                    // // Debug: Check hash and collisions
                    // println!(
                    //     "Inserting UniqueMove: {:?} with hash {:?}",
                    //     unique_move,
                    //     calculate_hash(&unique_move)
                    // );
                    // if let Some(existing) = parent_guard.opponent_move_stats.get(&unique_move) {
                    //     println!(
                    //         "Collision detected: {:?} with hash {:?}. Replacing with: {:?}",
                    //         existing,
                    //         calculate_hash(existing),
                    //         unique_move
                    //     );
                    // }

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

            // Format based on stored active Pokemon in the node if available
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

        // Print opponent move stats if they exist
        if !self.opponent_move_stats.is_empty() {
            self.print_opponent_move_stats(output, &new_prefix, state);
        }

        // Show children
        let mut children: Vec<_> = self.children.iter().collect();
        children.sort_by(|(_, a), (_, b)| {
            let a_lock = a.lock().unwrap();
            let b_lock = b.lock().unwrap();
            b_lock
                .visits
                .cmp(&a_lock.visits)
                .then(b_lock.creation_time.cmp(&a_lock.creation_time))
        });

        for (i, (_, child)) in children.iter().enumerate() {
            let child_ref = child.lock().unwrap();
            let mut child_state = state.clone();

            // Apply moves to get to child state
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

                // Key change here: always use the stored pokemon_name from UniqueMove
                // rather than trying to reconstruct from current state
                let move_desc = if unique_move.is_switch {
                    if let MoveChoice::Switch(idx) = &unique_move.move_choice {
                        format!(
                            "switch {} (from {})",
                            state.side_two.pokemon[*idx].id,
                            unique_move.pokemon_name // Use stored name
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
                        unique_move.pokemon_name // Use stored name
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

    // Preallocate vector with known size
    let mut weights = Vec::with_capacity(instructions.len());
    weights.extend(instructions.iter().map(|i| i.percentage as f64));

    THREAD_RNG.with(|rng| match WeightedIndex::new(&weights) {
        Ok(dist) => &instructions[dist.sample(&mut *rng.borrow_mut())],
        Err(_) => &instructions[0],
    })
}

pub fn perform_mcts_search(
    state: &mut State,
    iterations: Option<u32>,
    time_limit: Option<Duration>,
) -> MoveChoice {
    let start_time = Instant::now();
    let n_threads = rayon::current_num_threads();

    // Create parameters for each thread
    let batch_size = 20;
    let iterations_per_thread = iterations.map(|i| i / n_threads as u32);
    let time_limit_ref = Arc::new(time_limit);

    // Run parallel MCTS
    let trees: Vec<_> = (0..n_threads)
        .into_par_iter()
        .map(|_| {
            // Each thread gets its own MCTS tree and state
            let thread_state = state.clone();
            let mcts = DebugMCTS::new();
            let root_arc = Arc::clone(&mcts.root);
            let max_depth_arc = Arc::clone(&mcts.max_depth_seen);
            let time_limit = Arc::clone(&time_limit_ref);

            while !should_stop(&start_time, iterations_per_thread, *time_limit, &mcts) {
                for _ in 0..batch_size {
                    let mut sim_state = thread_state.clone();
                    let root_eval = evaluate(&thread_state);

                    // Select and expand
                    let (selected_node, move_history) = DebugMCTSNode::select_and_expand(
                        Arc::clone(&root_arc),
                        None,
                        &mut sim_state,
                        &max_depth_arc,
                    );

                    // Compute simulation score
                    let score = if sim_state.battle_is_over() != 0.0 {
                        if sim_state.battle_is_over() > 0.0 {
                            1.0
                        } else {
                            0.0
                        }
                    } else {
                        sigmoid(evaluate(&sim_state) - root_eval)
                    };

                    // Backpropagate the score
                    DebugMCTSNode::backpropagate(selected_node, score, &move_history);
                }
            }
            mcts
        })
        .collect();

    // Aggregate statistics from all trees
    let (our_moves, _) = state.get_all_options();
    let mut combined_stats: HashMap<MoveChoice, (i64, f32)> = HashMap::new();
    let mut max_depth = 0;
    let mut total_visits = 0;

    for tree in &trees {
        let root = tree.root.lock().unwrap();
        max_depth = max_depth.max(*tree.max_depth_seen.lock().unwrap());
        total_visits += root.visits;

        // Combine statistics
        for mov in &our_moves {
            if let Some(child) = root.children.get(mov) {
                let child_guard = child.lock().unwrap();
                let entry = combined_stats.entry(mov.clone()).or_insert((0, 0.0));
                entry.0 += child_guard.visits;
                entry.1 += child_guard.value;
            }
        }
    }

    // Print combined statistics
    println!("\nCombined Move Statistics:");
    println!(
        "{:<35}{:>12}{:>12}{:>10}",
        "Move", "Visits", "Avg Value", "% Visits"
    );
    println!("{:-<69}", "");

    let mut stats: Vec<_> = combined_stats.iter().collect();
    stats.sort_by_key(|(_, (visits, _))| -visits);

    for (mov, (visits, value)) in stats {
        let visit_percentage = if total_visits > 0 {
            (*visits as f32 / total_visits as f32) * 100.0
        } else {
            0.0
        };
        let avg_value = if *visits > 0 {
            value / *visits as f32
        } else {
            0.0
        };

        let move_name = DebugMCTSNode::format_move_name(state, mov, true);
        println!(
            "{:<35}{:>12}{:>12.3}{:>9.1}%",
            move_name, visits, avg_value, visit_percentage
        );
    }

    println!(
        "Time elapsed: {:?}, Total visits: {}, Maximum depth: {}",
        start_time.elapsed(),
        total_visits,
        max_depth
    );

    // Choose best move based on combined statistics
    let mut best_visits = 0;
    let mut best_moves = Vec::new();

    for (mov, (visits, _)) in &combined_stats {
        if *visits > best_visits {
            best_visits = *visits;
            best_moves.clear();
            best_moves.push(mov);
        } else if *visits == best_visits {
            best_moves.push(mov);
        }
    }

    let mut rng = thread_rng();
    best_moves[rng.gen_range(0..best_moves.len())].clone()
}

pub fn debug_mcts(
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

    // Print statistics first without holding any locks
    print_move_statistics(&result.root, &our_moves, state);

    // Then handle file output if needed
    if let Some(ref mut file) = output_file {
        let root_guard = result.root.lock().unwrap();
        print_move_statistics_to_file(&root_guard, &our_moves, state, file);
    }

    // Lock the max_depth_seen to get its inner value
    let max_depth = *result.max_depth_seen.lock().unwrap();
    println!(
        "Time elapsed: {:?}, Maximum depth explored: {}",
        start_time.elapsed(),
        max_depth
    );

    choose_best_move(&result.root, state)
}

fn internal_mcts_search(
    state: &mut State,
    max_iterations: u32,
    print_frequency: Option<u32>,
    debug_file: Option<&mut File>,
) -> DebugMCTS {
    let mcts = DebugMCTS::new();
    let root_eval = evaluate(state);

    println!("{print_frequency:?}");

    for _ in 0..max_iterations {
        let mut sim_state = state.clone();

        let (selected_node, move_history) = DebugMCTSNode::select_and_expand(
            Arc::clone(&mcts.root),
            None, // Don't pass debug file during iterations
            &mut sim_state,
            &mcts.max_depth_seen,
        );

        let score = if sim_state.battle_is_over() == 0.0 {
            sigmoid(evaluate(&sim_state) - root_eval)
        } else if sim_state.battle_is_over() > 0.0 {
            1.0
        } else {
            0.0
        };

        DebugMCTSNode::backpropagate(selected_node, score, &move_history);
    }

    // Show final tree only
    if let Some(file) = debug_file {
        writeln!(file, "\nFinal Tree:").unwrap();
        let root_guard = mcts.root.lock().unwrap();
        root_guard.visualize(state, Some(file));
    }

    mcts
}

// fn internal_mcts_search(
//     state: &mut State,
//     max_iterations: u32,
//     print_frequency: Option<u32>,
//     mut debug_file: Option<&mut File>,
// ) -> DebugMCTS {
//     let mut mcts = DebugMCTS::new();
//     let root_eval = evaluate(state);

//     if let Some(file) = debug_file.as_deref_mut() {
//         writeln!(file, "Starting debug MCTS search").unwrap();
//         writeln!(file, "Initial evaluation: {:.3}", root_eval).unwrap();
//         writeln!(file, "\nInitial State:\n{}", state.visualize()).unwrap();
//     }

//     for iteration in 0..max_iterations {
//         if let Some(file) = debug_file.as_deref_mut() {
//             writeln!(
//                 file,
//                 "Iteration {}, Root visits: {}",
//                 iteration,
//                 mcts.root.lock().unwrap().visits // Changed from borrow()
//             )
//             .unwrap();
//         }

//         if let Some(freq) = print_frequency {
//             if iteration % freq == 0 {
//                 if let Some(file) = debug_file.as_deref_mut() {
//                     writeln!(file, "\n=== Iteration {} ===", iteration).unwrap();
//                     let root_guard = mcts.root.lock().unwrap(); // Changed from borrow()
//                     root_guard.visualize(state, Some(file));
//                 }
//             }
//         }

//         let mut sim_state = state.clone();
//         let (selected_node, move_history) = DebugMCTSNode::select_and_expand(
//             Arc::clone(&mcts.root), // Changed from Rc::clone
//             debug_file.as_deref_mut(),
//             &mut sim_state,
//             &mut mcts.max_depth_seen,
//         );

//         if let Some(file) = debug_file.as_deref_mut() {
//             writeln!(file, "\nState after move sequence:").unwrap();
//             writeln!(file, "{}", sim_state.visualize()).unwrap();
//         }

//         let score = if sim_state.battle_is_over() == 0.0 {
//             let eval = evaluate(&sim_state);

//             if let Some(file) = debug_file.as_deref_mut() {
//                 writeln!(
//                     file,
//                     "Position evaluation: {:.3} (normalized: {:.3})",
//                     eval,
//                     sigmoid(eval - root_eval)
//                 )
//                 .unwrap();
//             }

//             sigmoid(eval - root_eval)
//         } else if sim_state.battle_is_over() > 0.0 {
//             if let Some(file) = debug_file.as_deref_mut() {
//                 writeln!(file, "Won position found").unwrap();
//             }
//             1.0
//         } else {
//             if let Some(file) = debug_file.as_deref_mut() {
//                 writeln!(file, "Lost position found").unwrap();
//             }
//             0.0
//         };

//         if let Some(file) = debug_file.as_deref_mut() {
//             writeln!(file, "battle_is_over: {}", sim_state.battle_is_over()).unwrap();
//             writeln!(file, "Move sequence:").unwrap();
//             for entry in &move_history {
//                 writeln!(
//                     file,
//                     "  {}",
//                     DebugMCTSNode::format_move_history_entry(&sim_state, entry)
//                 )
//                 .unwrap();
//             }
//         }

//         DebugMCTSNode::backpropagate(selected_node, score, &move_history);
//     }

//     if let Some(file) = debug_file.as_deref_mut() {
//         writeln!(file, "\nFinal Tree:").unwrap();
//         let root_guard = mcts.root.lock().unwrap(); // Changed from borrow()
//         root_guard.visualize(state, Some(file));
//     }

//     mcts
// }

fn print_move_statistics(
    root: &Arc<Mutex<DebugMCTSNode>>,
    available_moves: &[MoveChoice],
    state: &State,
) {
    let root = root.lock().unwrap();
    let stats = root.collect_move_statistics(available_moves);

    println!("\nMove Statistics:");
    println!(
        "{:<35}{:>12}{:>12}{:>10}",
        "Move", "Visits", "Avg Value", "% Visits"
    );
    println!("{:-<69}", "");

    let total_visits: i64 = stats.iter().map(|s| s.visits).sum();

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
fn choose_best_move(root: &Arc<Mutex<DebugMCTSNode>>, state: &State) -> MoveChoice {
    let root = root.lock().unwrap(); // Changed from borrow()
    let (our_moves, _) = state.get_all_options();
    let mut best_visits = 0;
    let mut best_moves = Vec::new();

    for mov in &our_moves {
        let visits = match root.children.get(mov) {
            Some(child) => child.lock().unwrap().visits, // Changed from borrow()
            None => 0,
        };

        if visits > best_visits {
            best_visits = visits;
            best_moves.clear();
            best_moves.push(mov);
        } else if visits == best_visits {
            best_moves.push(mov);
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
    let visits = mcts.root.lock().unwrap().visits;

    // Check iteration limit
    if let Some(max_iter) = iterations {
        if visits >= max_iter as i64 {
            return true;
        }
    }

    // Check time limit
    if let Some(limit) = time_limit {
        if start_time.elapsed() >= limit {
            return true;
        }
    }

    // Hard cap on total visits
    visits >= 10_000_000
}
