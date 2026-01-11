use crate::engine::evaluate::evaluate;
use crate::engine::generate_instructions::generate_instructions_from_move_pair;
use crate::engine::state::MoveChoice;
use crate::instruction::StateInstructions;
use crate::state::State;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use rand::rng;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn sigmoid(x: f32) -> f32 {
    // Tuned so that ~200 points is very close to 1.0
    1.0 / (1.0 + (-0.0125 * x).exp())
}

// shared move node that can be accessed across threads
#[derive(Debug)]
pub struct SharedMoveNode {
    pub move_choice: MoveChoice,
    pub total_score: f64,
    pub visits: u32,
}

impl SharedMoveNode {
    pub fn ucb1(&self, parent_visits: u32) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }
        let score = (self.total_score / self.visits as f64)
            + (2.0 * (parent_visits as f64).ln() / self.visits as f64).sqrt();
        score
    }
}

// root node with shared s1_options
#[derive(Debug)]
pub struct RootNode {
    pub children: HashMap<(usize, usize), Vec<Node>>,
    pub times_visited: u32,
    pub s1_options: Arc<RwLock<Vec<SharedMoveNode>>>,
    pub s2_options: Option<Vec<MoveNode>>,
}

impl RootNode {
    fn new(s1_options_shared: Arc<RwLock<Vec<SharedMoveNode>>>) -> RootNode {
        RootNode {
            times_visited: 0,
            children: HashMap::new(),
            s1_options: s1_options_shared,
            s2_options: None,
        }
    }

    unsafe fn populate(&mut self, s2_options: Vec<MoveChoice>) {
        let s2_options_vec: Vec<MoveNode> = s2_options
            .iter()
            .map(|x| MoveNode {
                move_choice: x.clone(),
                total_score: 0.0,
                visits: 0,
            })
            .collect();
        self.s2_options = Some(s2_options_vec);
    }

    pub fn maximize_ucb_for_s1(&self) -> usize {
        let s1_options = self.s1_options.read().unwrap();
        let mut choice = 0;
        let mut best_ucb1 = f64::MIN;
        for (index, node) in s1_options.iter().enumerate() {
            let this_ucb1 = node.ucb1(self.times_visited);
            if this_ucb1 > best_ucb1 {
                best_ucb1 = this_ucb1;
                choice = index;
            }
        }
        choice
    }

    pub fn maximize_ucb_for_s2(&self, side_map: &[MoveNode]) -> usize {
        let mut choice = 0;
        let mut best_ucb1 = f32::MIN;
        for (index, node) in side_map.iter().enumerate() {
            let this_ucb1 = node.ucb1(self.times_visited);
            if this_ucb1 > best_ucb1 {
                best_ucb1 = this_ucb1;
                choice = index;
            }
        }
        choice
    }

    pub unsafe fn selection(&mut self, state: &mut State) -> (*mut Node, usize, usize) {
        let s1_mc_index = self.maximize_ucb_for_s1();
        let s2_mc_index = self.maximize_ucb_for_s2(&self.s2_options.as_ref().unwrap());

        let child_vector = self.children.get_mut(&(s1_mc_index, s2_mc_index));
        match child_vector {
            Some(child_vector) => {
                let child_vec_ptr = child_vector as *mut Vec<Node>;
                let chosen_child = Node::sample_node_static(child_vec_ptr);
                state.apply_instructions(&(*chosen_child).instructions.instruction_list);
                (*chosen_child).selection(state)
            }
            None => (std::ptr::null_mut(), s1_mc_index, s2_mc_index),
        }
    }

    pub unsafe fn expand(
        &mut self,
        state: &mut State,
        s1_move_index: usize,
        s2_move_index: usize,
    ) -> *mut Node {
        let s1_move = {
            let s1_options = self.s1_options.read().unwrap();
            s1_options[s1_move_index].move_choice.clone()
        };
        let s2_move = &self.s2_options.as_ref().unwrap()[s2_move_index].move_choice;

        if state.battle_is_over() != 0.0
            || (s1_move == MoveChoice::None && s2_move == &MoveChoice::None)
        {
            return std::ptr::null_mut();
        }

        let should_branch_on_damage = true; // root is always true
        let mut new_instructions =
            generate_instructions_from_move_pair(state, &s1_move, s2_move, should_branch_on_damage);
        let mut this_pair_vec = Vec::with_capacity(new_instructions.len());

        for state_instructions in new_instructions.drain(..) {
            let mut new_node = Node::new();
            new_node.parent = std::ptr::null_mut(); // the parent is the root node, which is handled separately
            new_node.instructions = state_instructions;
            new_node.s1_choice = s1_move_index as u8;
            new_node.s2_choice = s2_move_index as u8;
            new_node.parent_is_root = true;

            this_pair_vec.push(new_node);
        }

        let new_node_ptr = Node::sample_node_static(&mut this_pair_vec);
        state.apply_instructions(&(*new_node_ptr).instructions.instruction_list);
        self.children
            .insert((s1_move_index, s2_move_index), this_pair_vec);
        new_node_ptr
    }

    pub unsafe fn backpropagate_root(&mut self, s1_choice: usize, s2_choice: usize, score: f32) {
        self.times_visited += 1;

        // Update shared s1 options with write lock
        {
            let mut s1_options = self.s1_options.write().unwrap();
            let s1_movenode = &mut s1_options[s1_choice];
            s1_movenode.total_score += score as f64;
            s1_movenode.visits += 1;
        }

        // Update local s2 options
        let s2_movenode = &mut self.s2_options.as_mut().unwrap()[s2_choice];
        s2_movenode.total_score += 1.0 - score;
        s2_movenode.visits += 1;
    }
}

#[derive(Debug)]
pub struct Node {
    pub parent: *mut Node,
    pub parent_is_root: bool,
    pub children: HashMap<(usize, usize), Vec<Node>>,
    pub times_visited: u32,
    pub instructions: StateInstructions,
    pub s1_choice: u8,
    pub s2_choice: u8,
    pub s1_options: Option<Vec<MoveNode>>,
    pub s2_options: Option<Vec<MoveNode>>,
}

impl Node {
    fn new() -> Node {
        Node {
            parent: std::ptr::null_mut(),
            parent_is_root: false,
            instructions: StateInstructions::default(),
            times_visited: 0,
            children: HashMap::new(),
            s1_choice: 0,
            s2_choice: 0,
            s1_options: None,
            s2_options: None,
        }
    }

    unsafe fn populate(&mut self, s1_options: Vec<MoveChoice>, s2_options: Vec<MoveChoice>) {
        let s1_options_vec: Vec<MoveNode> = s1_options
            .iter()
            .map(|x| MoveNode {
                move_choice: x.clone(),
                total_score: 0.0,
                visits: 0,
            })
            .collect();
        let s2_options_vec: Vec<MoveNode> = s2_options
            .iter()
            .map(|x| MoveNode {
                move_choice: x.clone(),
                total_score: 0.0,
                visits: 0,
            })
            .collect();

        self.s1_options = Some(s1_options_vec);
        self.s2_options = Some(s2_options_vec);
    }

    pub fn maximize_ucb_for_side(&self, side_map: &[MoveNode]) -> usize {
        let mut choice = 0;
        let mut best_ucb1 = f32::MIN;
        for (index, node) in side_map.iter().enumerate() {
            let this_ucb1 = node.ucb1(self.times_visited);
            if this_ucb1 > best_ucb1 {
                best_ucb1 = this_ucb1;
                choice = index;
            }
        }
        choice
    }

    pub unsafe fn selection(&mut self, state: &mut State) -> (*mut Node, usize, usize) {
        let return_node = self as *mut Node;
        if self.s1_options.is_none() {
            let (s1_options, s2_options) = state.get_all_options();
            self.populate(s1_options, s2_options);
        }

        let s1_mc_index = self.maximize_ucb_for_side(&self.s1_options.as_ref().unwrap());
        let s2_mc_index = self.maximize_ucb_for_side(&self.s2_options.as_ref().unwrap());
        let child_vector = self.children.get_mut(&(s1_mc_index, s2_mc_index));
        match child_vector {
            Some(child_vector) => {
                let child_vec_ptr = child_vector as *mut Vec<Node>;
                let chosen_child = Self::sample_node_static(child_vec_ptr);
                state.apply_instructions(&(*chosen_child).instructions.instruction_list);
                (*chosen_child).selection(state)
            }
            None => (return_node, s1_mc_index, s2_mc_index),
        }
    }

    unsafe fn sample_node_static(move_vector: *mut Vec<Node>) -> *mut Node {
        let mut rng = rng();
        let weights: Vec<f64> = (*move_vector)
            .iter()
            .map(|x| x.instructions.percentage as f64)
            .collect();
        let dist = WeightedIndex::new(weights).unwrap();
        let chosen_node = &mut (&mut *move_vector)[dist.sample(&mut rng)];
        chosen_node as *mut Node
    }

    pub unsafe fn expand(
        &mut self,
        state: &mut State,
        s1_move_index: usize,
        s2_move_index: usize,
    ) -> *mut Node {
        let s1_move = &self.s1_options.as_ref().unwrap()[s1_move_index].move_choice;
        let s2_move = &self.s2_options.as_ref().unwrap()[s2_move_index].move_choice;

        if state.battle_is_over() != 0.0
            || (s1_move == &MoveChoice::None && s2_move == &MoveChoice::None)
        {
            return self as *mut Node;
        }

        let should_branch_on_damage = self.parent_is_root;
        let mut new_instructions =
            generate_instructions_from_move_pair(state, s1_move, s2_move, should_branch_on_damage);
        let mut this_pair_vec = Vec::with_capacity(new_instructions.len());

        for state_instructions in new_instructions.drain(..) {
            let mut new_node = Node::new();
            new_node.parent = self;
            new_node.instructions = state_instructions;
            new_node.s1_choice = s1_move_index as u8;
            new_node.s2_choice = s2_move_index as u8;
            new_node.parent_is_root = false;

            this_pair_vec.push(new_node);
        }

        let new_node_ptr = Self::sample_node_static(&mut this_pair_vec);
        state.apply_instructions(&(*new_node_ptr).instructions.instruction_list);
        self.children
            .insert((s1_move_index, s2_move_index), this_pair_vec);
        new_node_ptr
    }

    pub unsafe fn backpropagate(&mut self, score: f32, state: &mut State) -> (usize, usize) {
        self.times_visited += 1;

        state.reverse_instructions(&self.instructions.instruction_list);
        if !self.parent.is_null() {
            let parent_s1_movenode =
                &mut (*self.parent).s1_options.as_mut().unwrap()[self.s1_choice as usize];
            parent_s1_movenode.total_score += score;
            parent_s1_movenode.visits += 1;

            let parent_s2_movenode =
                &mut (*self.parent).s2_options.as_mut().unwrap()[self.s2_choice as usize];
            parent_s2_movenode.total_score += 1.0 - score;
            parent_s2_movenode.visits += 1;

            (*self.parent).backpropagate(score, state)
        } else {
            (self.s1_choice as usize, self.s2_choice as usize)
        }
    }

    pub fn rollout(&mut self, state: &mut State, root_eval: &f32) -> f32 {
        let battle_is_over = state.battle_is_over();
        if battle_is_over == 0.0 {
            let eval = evaluate(state);
            sigmoid(eval - root_eval)
        } else {
            if battle_is_over == -1.0 {
                0.0
            } else {
                battle_is_over
            }
        }
    }
}

#[derive(Debug)]
pub struct MoveNode {
    pub move_choice: MoveChoice,
    pub total_score: f32,
    pub visits: u32,
}

impl MoveNode {
    pub fn ucb1(&self, parent_visits: u32) -> f32 {
        if self.visits == 0 {
            return f32::INFINITY;
        }
        let score = (self.total_score / self.visits as f32)
            + (2.0 * (parent_visits as f32).ln() / self.visits as f32).sqrt();
        score
    }

    pub fn average_score(&self) -> f32 {
        self.total_score / self.visits as f32
    }
}

#[derive(Clone)]
pub struct MctsSideResult {
    pub move_choice: MoveChoice,
    pub total_score: f32,
    pub visits: u32,
}

impl MctsSideResult {
    pub fn average_score(&self) -> f32 {
        if self.visits == 0 {
            return 0.0;
        }
        self.total_score / self.visits as f32
    }
}

pub struct MctsResult {
    pub s1: Vec<MctsSideResult>,
    pub s2: Vec<MctsSideResult>,
    pub iteration_count: u32,
}

fn do_mcts_root(root_node: &mut RootNode, state: &mut State, root_eval: &f32) {
    let (mut new_node, s1_move, s2_move) = unsafe { root_node.selection(state) };

    if new_node.is_null() {
        new_node = unsafe { root_node.expand(state, s1_move, s2_move) };
        if new_node.is_null() {
            return;
        }
        let rollout_result = unsafe { (*new_node).rollout(state, root_eval) };
        unsafe {
            let (s1_root_move, s2_root_move) = (*new_node).backpropagate(rollout_result, state);
            root_node.backpropagate_root(s1_root_move, s2_root_move, rollout_result);
        }
    } else {
        new_node = unsafe { (*new_node).expand(state, s1_move, s2_move) };
        let rollout_result = unsafe { (*new_node).rollout(state, root_eval) };
        unsafe {
            let (s1_root_move, s2_root_move) = (*new_node).backpropagate(rollout_result, state);
            root_node.backpropagate_root(s1_root_move, s2_root_move, rollout_result);
        }
    }
}

pub fn perform_many_mcts(
    states: Vec<State>,
    side_one_options: Vec<MoveChoice>,
    side_two_options_vec: Vec<Vec<MoveChoice>>,
    max_time: Duration,
) -> Vec<MctsResult> {
    assert_eq!(states.len(), side_two_options_vec.len());

    // Create shared s1_options
    let s1_options_shared = Arc::new(RwLock::new(
        side_one_options
            .iter()
            .map(|x| SharedMoveNode {
                move_choice: x.clone(),
                total_score: 0.0,
                visits: 0,
            })
            .collect::<Vec<_>>(),
    ));

    let mut handles = vec![];

    for (mut state, s2_options) in states.into_iter().zip(side_two_options_vec.into_iter()) {
        let s1_shared = Arc::clone(&s1_options_shared);

        let handle = thread::spawn(move || {
            let mut root_node = RootNode::new(s1_shared);
            unsafe {
                root_node.populate(s2_options);
            }

            let root_eval = evaluate(&state);
            let start_time = std::time::Instant::now();

            while start_time.elapsed() < max_time {
                for _ in 0..1000 {
                    do_mcts_root(&mut root_node, &mut state, &root_eval);
                }

                if root_node.times_visited == 10_000_000 {
                    break;
                }
            }

            // Convert results
            let s1_results = {
                let s1_options = root_node.s1_options.read().unwrap();
                s1_options
                    .iter()
                    .map(|v| MctsSideResult {
                        move_choice: v.move_choice.clone(),
                        total_score: v.total_score as f32,
                        visits: v.visits,
                    })
                    .collect()
            };

            let s2_results = root_node
                .s2_options
                .as_ref()
                .unwrap()
                .iter()
                .map(|v| MctsSideResult {
                    move_choice: v.move_choice.clone(),
                    total_score: v.total_score,
                    visits: v.visits,
                })
                .collect();

            MctsResult {
                s1: s1_results,
                s2: s2_results,
                iteration_count: root_node.times_visited,
            }
        });

        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

// Keep original single-state version
pub fn perform_mcts(
    state: &mut State,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
    max_time: Duration,
) -> MctsResult {
    // Implementation unchanged from original
    let mut root_node = Node::new();
    unsafe {
        root_node.populate(side_one_options, side_two_options);
    }

    let root_eval = evaluate(state);
    let start_time = std::time::Instant::now();
    while start_time.elapsed() < max_time {
        for _ in 0..1000 {
            let (mut new_node, s1_move, s2_move) = unsafe { root_node.selection(state) };
            new_node = unsafe { (*new_node).expand(state, s1_move, s2_move) };
            let rollout_result = unsafe { (*new_node).rollout(state, &root_eval) };
            unsafe {
                (*new_node).backpropagate(rollout_result, state);
            }
        }

        /*
        Cut off after 10 million iterations

        Under normal circumstances the bot will only run for 2.5-3.5 million iterations
        however towards the end of a battle the bot may perform tens of millions of iterations

        Beyond about 30 million iterations some floating point nonsense happens where
        MoveNode.total_score stops updating because f32 does not have enough precision

        I can push the problem farther out by using f64 but if the bot is running for 10 million iterations
        then it almost certainly sees a forced win
        */
        if root_node.times_visited == 10_000_000 {
            break;
        }
    }

    MctsResult {
        s1: root_node
            .s1_options
            .as_ref()
            .unwrap()
            .iter()
            .map(|v| MctsSideResult {
                move_choice: v.move_choice.clone(),
                total_score: v.total_score,
                visits: v.visits,
            })
            .collect(),
        s2: root_node
            .s2_options
            .as_ref()
            .unwrap()
            .iter()
            .map(|v| MctsSideResult {
                move_choice: v.move_choice.clone(),
                total_score: v.total_score,
                visits: v.visits,
            })
            .collect(),
        iteration_count: root_node.times_visited,
    }
}
