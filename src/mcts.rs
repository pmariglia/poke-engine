use crate::evaluate::evaluate;
use crate::generate_instructions::generate_instructions_from_move_pair;
use crate::instruction::StateInstructions;
use crate::state::{MoveChoice, State};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;
use std::collections::HashMap;
use std::time::Duration;

fn sigmoid(x: f32) -> f32 {
    // Tuned so that ~200 points is very close to 1.0
    return 1.0 / (1.0 + (-0.0125 * x).exp());
}

#[derive(Debug)]
pub struct Node {
    pub root: bool,
    pub parent: *mut Node,
    pub children: HashMap<(MoveChoice, MoveChoice), Vec<Node>>,
    pub times_visited: i64,

    // represents the instructions & s1/s2 moves that led to this node from the parent
    pub instructions: StateInstructions,
    pub s1_choice: MoveChoice,
    pub s2_choice: MoveChoice,

    // represents the total score and number of visits for this node
    // de-coupled for s1 and s2
    pub s1_options: HashMap<MoveChoice, MoveNode>,
    pub s2_options: HashMap<MoveChoice, MoveNode>,
}

impl Node {
    pub fn generate_options(&mut self, state: &State) {
        let (s1_options, s2_options) = state.get_all_options();
        for op in s1_options.iter() {
            self.s1_options.insert(
                op.clone(),
                MoveNode {
                    move_choice: op.clone(),
                    total_score: 0.0,
                    visits: 0,
                },
            );
        }
        for op in s2_options.iter() {
            self.s2_options.insert(
                op.clone(),
                MoveNode {
                    move_choice: op.clone(),
                    total_score: 0.0,
                    visits: 0,
                },
            );
        }
    }

    pub fn maximize_ucb_for_side(&self, side_map: &HashMap<MoveChoice, MoveNode>) -> MoveChoice {
        let mut choice = MoveChoice::None;
        let mut best_ucb1 = f32::MIN;
        for (_, node) in side_map.iter() {
            let this_ucb1 = node.ucb1(self.times_visited);
            if this_ucb1 > best_ucb1 {
                best_ucb1 = this_ucb1;
                choice = node.move_choice;
            }
        }
        return choice;
    }

    pub unsafe fn selection(&mut self, state: &mut State) -> (*mut Node, MoveChoice, MoveChoice) {
        let return_node = self as *mut Node;

        let s1_move_choice = self.maximize_ucb_for_side(&self.s1_options);
        let s2_move_choice = self.maximize_ucb_for_side(&self.s2_options);
        let child_vector = self.children.get_mut(&(s1_move_choice, s2_move_choice));
        match child_vector {
            Some(child_vector) => {
                let child_vec_ptr = child_vector as *mut Vec<Node>;
                let chosen_child = self.sample_node(child_vec_ptr);
                state.apply_instructions(&(*chosen_child).instructions.instruction_list);
                return (*chosen_child).selection(state);
            }
            None => {
                return (return_node, s1_move_choice, s2_move_choice);
            }
        }
    }

    unsafe fn sample_node(&self, move_vector: *mut Vec<Node>) -> *mut Node {
        let mut rng = thread_rng();
        let weights: Vec<f64> = (*move_vector)
            .iter()
            .map(|x| x.instructions.percentage as f64)
            .collect();
        let dist = WeightedIndex::new(weights).unwrap();
        let chosen_node = &mut (*move_vector)[dist.sample(&mut rng)];
        let chosen_node_ptr = chosen_node as *mut Node;
        return chosen_node_ptr;
    }

    pub unsafe fn expand(
        &mut self,
        state: &mut State,
        s1_move: &MoveChoice,
        s2_move: &MoveChoice,
    ) -> *mut Node {
        // if the battle is over, no need to expand
        if state.battle_is_over() != 0.0 && !self.root {
            return self as *mut Node;
        }
        let mut new_instructions = generate_instructions_from_move_pair(state, s1_move, s2_move);
        let mut this_pair_vec = Vec::with_capacity(2);
        for state_instructions in new_instructions.drain(..) {
            let mut new_node = Node::default();
            new_node.parent = self;
            new_node.instructions = state_instructions;
            new_node.s1_choice = s1_move.clone();
            new_node.s2_choice = s2_move.clone();
            state.apply_instructions(&new_node.instructions.instruction_list);
            new_node.generate_options(state);
            state.reverse_instructions(&new_node.instructions.instruction_list);
            this_pair_vec.push(new_node);
        }

        // sample a node from the new instruction list.
        // this is the node that the rollout will be done on
        let new_node_ptr = self.sample_node(&mut this_pair_vec);
        state.apply_instructions(&(*new_node_ptr).instructions.instruction_list);
        self.children
            .insert((s1_move.clone(), s2_move.clone()), this_pair_vec);
        return new_node_ptr;
    }

    pub unsafe fn backpropagate(&mut self, score: f32, state: &mut State) {
        self.times_visited += 1;
        if self.root {
            return;
        }
        (*self.parent)
            .s1_options
            .get_mut(&self.s1_choice)
            .unwrap()
            .total_score += score;
        (*self.parent)
            .s1_options
            .get_mut(&self.s1_choice)
            .unwrap()
            .visits += 1;
        (*self.parent)
            .s2_options
            .get_mut(&self.s2_choice)
            .unwrap()
            .total_score += 1.0 - score;
        (*self.parent)
            .s2_options
            .get_mut(&self.s2_choice)
            .unwrap()
            .visits += 1;

        state.reverse_instructions(&self.instructions.instruction_list);
        (*self.parent).backpropagate(score, state);
    }

    pub fn rollout(&mut self, state: &mut State) -> f32 {
        let battle_is_over = state.battle_is_over();
        if battle_is_over == 0.0 {
            let eval = evaluate(state);
            return sigmoid(eval);
        } else {
            if battle_is_over == -1.0 {
                return 0.0;
            } else {
                return battle_is_over;
            }
        }
    }
}

impl Default for Node {
    fn default() -> Node {
        return Node {
            root: false,
            parent: std::ptr::null_mut(),
            instructions: StateInstructions::default(),
            times_visited: 0,
            children: HashMap::new(),
            s1_choice: MoveChoice::None,
            s2_choice: MoveChoice::None,
            s1_options: HashMap::new(),
            s2_options: HashMap::new(),
        };
    }
}

#[derive(Debug)]
pub struct MoveNode {
    pub move_choice: MoveChoice,
    pub total_score: f32,
    pub visits: i64,
}

impl MoveNode {
    pub fn ucb1(&self, parent_visits: i64) -> f32 {
        if self.visits == 0 {
            return f32::INFINITY;
        }
        let score = (self.total_score / self.visits as f32)
            + (2.0 * (parent_visits as f32).ln() / self.visits as f32).sqrt();
        return score;
    }
    pub fn average_score(&self) -> f32 {
        let score = self.total_score / self.visits as f32;
        return score;
    }
}

#[derive(Clone)]
pub struct MctsSideResult {
    pub move_choice: MoveChoice,
    pub total_score: f32,
    pub visits: i64,
}

impl MctsSideResult {
    pub fn average_score(&self) -> f32 {
        if self.visits == 0 {
            return 0.0;
        }
        let score = self.total_score / self.visits as f32;
        return score;
    }
}

pub struct MctsResult {
    pub s1: Vec<MctsSideResult>,
    pub s2: Vec<MctsSideResult>,
    pub iteration_count: i64,
}

fn do_mcts(root_node: &mut Node, state: &mut State) {
    let (mut new_node, s1_move, s2_move) = unsafe { root_node.selection(state) };
    new_node = unsafe { (*new_node).expand(state, &s1_move, &s2_move) };
    let rollout_result = unsafe { (*new_node).rollout(state) };
    unsafe { (*new_node).backpropagate(rollout_result, state) }
}

pub fn perform_mcts(
    state: &mut State,
    mut side_one_options: Vec<MoveChoice>,
    mut side_two_options: Vec<MoveChoice>,
    max_time: Duration,
) -> MctsResult {
    let mut root_node = Node::default();
    root_node.root = true;
    for op in side_one_options.drain(..) {
        root_node.s1_options.insert(
            op.clone(),
            MoveNode {
                move_choice: op,
                total_score: 0.0,
                visits: 0,
            },
        );
    }
    for op in side_two_options.drain(..) {
        root_node.s2_options.insert(
            op.clone(),
            MoveNode {
                move_choice: op,
                total_score: 0.0,
                visits: 0,
            },
        );
    }

    let start_time = std::time::Instant::now();
    while start_time.elapsed() < max_time {
        for _ in 0..1000 {
            do_mcts(&mut root_node, state);
        }
    }

    let result = MctsResult {
        s1: root_node
            .s1_options
            .iter()
            .map(|(k, v)| MctsSideResult {
                move_choice: k.clone(),
                total_score: v.total_score,
                visits: v.visits,
            })
            .collect(),
        s2: root_node
            .s2_options
            .iter()
            .map(|(k, v)| MctsSideResult {
                move_choice: k.clone(),
                total_score: v.total_score,
                visits: v.visits,
            })
            .collect(),
        iteration_count: root_node.times_visited,
    };

    return result;
}
