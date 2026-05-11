use crate::engine::evaluate::evaluate;
use crate::engine::generate_instructions::generate_instructions_from_move_pair;
use crate::engine::state::MoveChoice;
use crate::instruction::StateInstructions;
use crate::mcts::{MctsResult, MctsSideResult};
use crate::state::State;
use rand::prelude::*;
use rand::rng;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::sync::{Arc, OnceLock, RwLock};
use std::thread;
use std::time::{Duration, Instant};

const MCTS_DEADLINE_CHECK_INTERVAL: u32 = 1_000;
const MCTS_MAX_ITERATIONS_PER_TREE: u32 = 10_000_000;
const MCTS_DAMAGE_BRANCH_DEPTH: u8 = 2;
const SCORE_SCALE: f32 = 400.0;
const VIRTUAL_LOSS_VISITS: u32 = 3;

fn sigmoid(x: f32) -> f32 {
    // Tuned so that ~200 points is very close to 1.0
    1.0 / (1.0 + (-0.0125 * x).exp())
}

pub struct MoveNode {
    move_choice: MoveChoice,
    total_score: AtomicU32,
    visits: AtomicU32,
}

impl MoveNode {
    fn new(move_choice: MoveChoice) -> Self {
        Self {
            move_choice,
            total_score: AtomicU32::new(0),
            visits: AtomicU32::new(0),
        }
    }

    #[inline]
    fn add_virtual_loss(&self) {
        self.visits.fetch_add(VIRTUAL_LOSS_VISITS, Ordering::AcqRel);
    }

    #[inline]
    fn remove_virtual_loss(&self) {
        self.visits.fetch_sub(VIRTUAL_LOSS_VISITS, Ordering::AcqRel);
    }

    #[inline]
    fn add_result(&self, score: f32) {
        self.total_score
            .fetch_add((score * SCORE_SCALE).round() as u32, Ordering::AcqRel);
        self.visits.fetch_add(1, Ordering::AcqRel);
    }

    #[inline]
    fn total_score_f32(&self) -> f32 {
        self.total_score.load(Ordering::Acquire) as f32 / SCORE_SCALE
    }

    #[inline]
    fn ucb1_with_parent_exploration(&self, parent_exploration_numerator: f32) -> f32 {
        let visits = self.visits.load(Ordering::Acquire);
        if visits == 0 {
            return f32::INFINITY;
        }
        let total_score = self.total_score_f32();
        let average_score = total_score / visits as f32;
        average_score + (parent_exploration_numerator / visits as f32).sqrt()
    }
}

pub struct SharedNodeOptions {
    s1: Vec<MoveNode>,
    s2: Vec<MoveNode>,
}

impl SharedNodeOptions {
    fn new(s1_options: Vec<MoveChoice>, s2_options: Vec<MoveChoice>) -> Self {
        Self {
            s1: s1_options.into_iter().map(MoveNode::new).collect(),
            s2: s2_options.into_iter().map(MoveNode::new).collect(),
        }
    }
}

pub struct SharedNodeChildren {
    entries: RwLock<HashMap<(usize, usize), SharedBranch>>,
}

impl SharedNodeChildren {
    fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }

    fn get_and_sample<R: Rng + ?Sized>(
        &self,
        s1_index: usize,
        s2_index: usize,
        rng: &mut R,
    ) -> Option<Arc<Node>> {
        self.entries
            .read()
            .unwrap()
            .get(&(s1_index, s2_index))
            .map(|branch| branch.sample(rng))
    }

    fn insert_if_absent_and_sample<R: Rng + ?Sized>(
        &self,
        s1_index: usize,
        s2_index: usize,
        branch: SharedBranch,
        rng: &mut R,
    ) -> Arc<Node> {
        let mut map = self.entries.write().unwrap();
        let branch = map.entry((s1_index, s2_index)).or_insert(branch);
        branch.sample(rng)
    }
}

pub struct SharedBranch {
    nodes: Vec<Arc<Node>>,
    total_weight: f32,
}

impl SharedBranch {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Arc<Node> {
        if self.nodes.len() <= 1 || self.total_weight <= 0.0 {
            return self.nodes[0].clone();
        }

        let mut threshold = rng.random_range(0.0..self.total_weight);
        for node in &self.nodes {
            threshold -= node.instructions.percentage.max(0.0);
            if threshold <= 0.0 {
                return node.clone();
            }
        }
        self.nodes[self.nodes.len() - 1].clone()
    }
}

pub struct Node {
    root: bool,
    instructions: StateInstructions,
    depth: u8,
    times_visited: AtomicU32,
    virtual_losses: AtomicI32,
    options: OnceLock<SharedNodeOptions>,
    children: SharedNodeChildren,
}

impl Node {
    fn new_root(side_one_options: Vec<MoveChoice>, side_two_options: Vec<MoveChoice>) -> Arc<Self> {
        let node = Arc::new(Self {
            root: true,
            instructions: StateInstructions::default(),
            depth: 0,
            times_visited: AtomicU32::new(0),
            virtual_losses: AtomicI32::new(0),
            options: OnceLock::new(),
            children: SharedNodeChildren::new(),
        });
        let _ = node
            .options
            .set(SharedNodeOptions::new(side_one_options, side_two_options));
        node
    }

    fn new_child(
        instructions: StateInstructions,
        _s1_choice: usize,
        _s2_choice: usize,
        depth: u8,
    ) -> Arc<Self> {
        Arc::new(Self {
            root: false,
            instructions,
            depth,
            times_visited: AtomicU32::new(0),
            virtual_losses: AtomicI32::new(0),
            options: OnceLock::new(),
            children: SharedNodeChildren::new(),
        })
    }

    fn ensure_options(&self, state: &State) -> &SharedNodeOptions {
        self.options.get_or_init(|| {
            let (s1_options, s2_options) = state.get_all_options();
            SharedNodeOptions::new(s1_options, s2_options)
        })
    }

    fn maximize_ucb_for_side(&self, side_options: &[MoveNode]) -> usize {
        let mut choice = 0;
        let mut best_ucb1 = f32::MIN;
        let parent_visits = self
            .times_visited
            .load(Ordering::Acquire)
            .saturating_add(self.virtual_losses.load(Ordering::Acquire).max(0) as u32)
            .max(1);
        let parent_exploration_numerator = 2.0 * (parent_visits as f32).ln().max(0.0);

        for (index, node) in side_options.iter().enumerate() {
            let this_ucb1 = node.ucb1_with_parent_exploration(parent_exploration_numerator);
            if this_ucb1 > best_ucb1 {
                best_ucb1 = this_ucb1;
                choice = index;
            }
        }
        choice
    }

    fn select_move_pair(&self, state: &State) -> Option<(usize, usize)> {
        let options = self.ensure_options(state);
        if options.s1.is_empty() || options.s2.is_empty() {
            return None;
        }
        Some((
            self.maximize_ucb_for_side(&options.s1),
            self.maximize_ucb_for_side(&options.s2),
        ))
    }

    fn sample_existing_child<R: Rng + ?Sized>(
        &self,
        s1_index: usize,
        s2_index: usize,
        rng: &mut R,
    ) -> Option<Arc<Node>> {
        self.children.get_and_sample(s1_index, s2_index, rng)
    }

    fn expand_and_sample_child<R: Rng + ?Sized>(
        &self,
        state: &mut State,
        s1_index: usize,
        s2_index: usize,
        rng: &mut R,
    ) -> Option<Arc<Node>> {
        // Check under read lock first
        if let Some(existing) = self.children.get_and_sample(s1_index, s2_index, rng) {
            return Some(existing);
        }

        let options = self
            .options
            .get()
            .expect("node options must be initialized");
        let s1_move = &options.s1[s1_index].move_choice;
        let s2_move = &options.s2[s2_index].move_choice;

        if (state.battle_is_over() != 0.0 && !self.root)
            || (s1_move == &MoveChoice::None && s2_move == &MoveChoice::None)
        {
            return None;
        }

        let should_branch_on_damage = self.depth < MCTS_DAMAGE_BRANCH_DEPTH;
        let instructions =
            generate_instructions_from_move_pair(state, s1_move, s2_move, should_branch_on_damage);
        if instructions.is_empty() {
            return None;
        }

        let nodes = instructions
            .into_iter()
            .map(|state_instructions| {
                Node::new_child(
                    state_instructions,
                    s1_index,
                    s2_index,
                    self.depth.saturating_add(1),
                )
            })
            .collect::<Vec<_>>();
        let total_weight = nodes
            .iter()
            .map(|n| n.instructions.percentage.max(0.0))
            .sum();
        let branch = SharedBranch {
            nodes,
            total_weight,
        };

        // or_insert handles the race — if another thread inserted while we
        // were generating instructions, we just use theirs
        Some(
            self.children
                .insert_if_absent_and_sample(s1_index, s2_index, branch, rng),
        )
    }
}

struct PathStep {
    parent: Arc<Node>,
    child: Arc<Node>,
    s1_index: usize,
    s2_index: usize,
}

fn rollout(state: &State, root_eval: f32) -> f32 {
    let battle_is_over = state.battle_is_over();
    if battle_is_over == 0.0 {
        let eval = evaluate(state);
        sigmoid(eval - root_eval)
    } else if battle_is_over == -1.0 {
        0.0
    } else {
        battle_is_over
    }
}

fn reverse_path(state: &mut State, path: &[PathStep]) {
    for step in path.iter().rev() {
        state.reverse_instructions(&step.child.instructions.instruction_list);
    }
}

fn remove_virtual_losses(path: &[PathStep]) {
    for step in path {
        if let Some(options) = step.parent.options.get() {
            options.s1[step.s1_index].remove_virtual_loss();
            options.s2[step.s2_index].remove_virtual_loss();
        }
        step.child.virtual_losses.fetch_sub(1, Ordering::AcqRel);
    }
}

fn backpropagate(path: &[PathStep], leaf: &Arc<Node>, score: f32) {
    leaf.times_visited.fetch_add(1, Ordering::AcqRel);

    for step in path.iter().rev() {
        let options = step.parent.options.get().expect("path parent has options");
        options.s1[step.s1_index].add_result(score);
        options.s2[step.s2_index].add_result(1.0 - score);
        step.parent.times_visited.fetch_add(1, Ordering::AcqRel);
    }
}

fn do_shared_tree_playout<R: Rng + ?Sized>(
    root: &Arc<Node>,
    state: &mut State,
    root_eval: f32,
    rng: &mut R,
) {
    let mut path = Vec::with_capacity(16);
    let mut current = root.clone();

    loop {
        if current.depth > 0 && state.battle_is_over() != 0.0 {
            break;
        }

        let Some((s1_index, s2_index)) = current.select_move_pair(state) else {
            break;
        };
        let options = current.options.get().expect("selected node has options");
        options.s1[s1_index].add_virtual_loss();
        options.s2[s2_index].add_virtual_loss();

        let child =
            if let Some(existing_child) = current.sample_existing_child(s1_index, s2_index, rng) {
                Some(existing_child)
            } else {
                current.expand_and_sample_child(state, s1_index, s2_index, rng)
            };

        let Some(child) = child else {
            options.s1[s1_index].remove_virtual_loss();
            options.s2[s2_index].remove_virtual_loss();
            break;
        };

        child.virtual_losses.fetch_add(1, Ordering::AcqRel);
        state.apply_instructions(&child.instructions.instruction_list);
        path.push(PathStep {
            parent: current.clone(),
            child: child.clone(),
            s1_index,
            s2_index,
        });

        let child_was_new_leaf = child.times_visited.load(Ordering::Acquire) == 0;
        current = child;
        if child_was_new_leaf {
            break;
        }
    }

    let score = rollout(state, root_eval);
    backpropagate(&path, &current, score);
    remove_virtual_losses(&path);
    reverse_path(state, &path);
}

fn mcts_worker_count() -> usize {
    std::env::var("POKE_ENGINE_MCTS_WORKER_COUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| 4)
}

pub fn perform_mcts_shared_tree(
    state: &mut State,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
    max_time: Duration,
    root_eval: f32,
) -> MctsResult {
    let worker_count = mcts_worker_count();
    let deadline = Instant::now() + max_time;
    let root = Node::new_root(side_one_options, side_two_options);
    let started_iterations = Arc::new(AtomicU32::new(0));
    let max_iterations = MCTS_MAX_ITERATIONS_PER_TREE;

    thread::scope(|scope| {
        for _ in 0..worker_count {
            let root = root.clone();
            let started_iterations = started_iterations.clone();
            let mut worker_state = state.clone();
            scope.spawn(move || {
                let mut rng = rng();
                let mut iterations_until_deadline_check = 0;
                loop {
                    if iterations_until_deadline_check == 0 {
                        if Instant::now() >= deadline {
                            break;
                        }
                        iterations_until_deadline_check = MCTS_DEADLINE_CHECK_INTERVAL;
                    }

                    let iteration = started_iterations.fetch_add(1, Ordering::AcqRel);
                    if iteration >= max_iterations {
                        break;
                    }

                    do_shared_tree_playout(&root, &mut worker_state, root_eval, &mut rng);
                    iterations_until_deadline_check -= 1;
                }
            });
        }
    });

    let options = root.options.get().expect("root options initialized");
    MctsResult {
        s1: options
            .s1
            .iter()
            .map(|v| MctsSideResult {
                move_choice: v.move_choice,
                total_score: v.total_score_f32(),
                visits: v.visits.load(Ordering::Acquire),
            })
            .collect(),
        s2: options
            .s2
            .iter()
            .map(|v| MctsSideResult {
                move_choice: v.move_choice,
                total_score: v.total_score_f32(),
                visits: v.visits.load(Ordering::Acquire),
            })
            .collect(),
        iteration_count: root.times_visited.load(Ordering::Acquire),
    }
}
