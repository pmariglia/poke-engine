use crate::engine::evaluate::evaluate;
use crate::engine::generate_instructions::generate_instructions_from_move_pair;
use crate::engine::state::MoveChoice;
use crate::instruction::StateInstructions;
use crate::state::State;
use rand::prelude::*;
use rand::rng;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::Duration;

// exploration floor mixed into the sampling policy. every action is sampled with
// probability at least GAMMA / n, which bounds the importance weights in the
// regret update by n / GAMMA
const GAMMA: f64 = 0.1;

fn sigmoid(x: f32) -> f32 {
    // Tuned so that ~200 points is very close to 1.0
    1.0 / (1.0 + (-0.0125 * x).exp())
}

// computes the regret-matching strategy, accumulates it into the average
// strategy (whose normalization is what converges to equilibrium), and samples
// an action from the exploration-mixed policy.
// returns the sampled index and the probability it was sampled with
fn sample_regret_matching(stats: &mut [ActionStats], rng: &mut impl Rng) -> (usize, f64) {
    let n = stats.len();
    let uniform = 1.0 / n as f64;
    let positive_regret_sum: f64 = stats.iter().map(|s| s.cumulative_regret.max(0.0)).sum();

    if positive_regret_sum > 0.0 {
        for s in stats.iter_mut() {
            s.cumulative_strategy += s.cumulative_regret.max(0.0) / positive_regret_sum;
        }
    } else {
        for s in stats.iter_mut() {
            s.cumulative_strategy += uniform;
        }
    }

    let index = if positive_regret_sum <= 0.0 || rng.random_bool(GAMMA) {
        rng.random_range(0..n)
    } else {
        let mut threshold = rng.random_range(0.0..positive_regret_sum);
        let mut chosen = n - 1;
        for (i, s) in stats.iter().enumerate() {
            threshold -= s.cumulative_regret.max(0.0);
            if threshold <= 0.0 {
                chosen = i;
                break;
            }
        }
        chosen
    };

    let sigma = if positive_regret_sum > 0.0 {
        stats[index].cumulative_regret.max(0.0) / positive_regret_sum
    } else {
        uniform
    };
    let prob = (1.0 - GAMMA) * sigma + GAMMA * uniform;
    (index, prob)
}

// payoff * I(a == sampled) / sample_prob is an unbiased estimate of each
// action's value; the raw payoff estimates the value of the current policy.
// their difference is the sampled instantaneous regret
fn update_regrets(stats: &mut [ActionStats], sampled: usize, sample_prob: f64, payoff: f64) {
    for (i, s) in stats.iter_mut().enumerate() {
        if i == sampled {
            s.cumulative_regret += payoff / sample_prob - payoff;
        } else {
            s.cumulative_regret -= payoff;
        }
    }
    stats[sampled].total_score += payoff as f32;
    stats[sampled].visits += 1;
}

#[derive(Debug)]
pub struct Node {
    pub root: bool,
    pub parent: *mut Node,
    pub times_visited: u32,

    // represents the instructions & s1/s2 moves that led to this node from the parent
    pub instructions: StateInstructions,
    pub s1_choice: u8,
    pub s2_choice: u8,

    // the probabilities the s1/s2 choices were sampled with when this node was
    // last traversed, needed for the regret update during backpropagation
    pub s1_prob: f64,
    pub s2_prob: f64,

    // regret-matching statistics, de-coupled for s1 and s2
    pub s1_options: Option<Vec<ActionStats>>,
    pub s2_options: Option<Vec<ActionStats>>,
}

impl Node {
    fn new() -> Node {
        Node {
            root: false,
            parent: std::ptr::null_mut(),
            instructions: StateInstructions::default(),
            times_visited: 0,
            s1_choice: 0,
            s2_choice: 0,
            s1_prob: 1.0,
            s2_prob: 1.0,
            s1_options: None,
            s2_options: None,
        }
    }

    fn populate(&mut self, s1_options: Vec<MoveChoice>, s2_options: Vec<MoveChoice>) {
        let s1_options_vec: Vec<ActionStats> = s1_options
            .iter()
            .map(|x| ActionStats::new(x.clone()))
            .collect();
        let s2_options_vec: Vec<ActionStats> = s2_options
            .iter()
            .map(|x| ActionStats::new(x.clone()))
            .collect();

        self.s1_options = Some(s1_options_vec);
        self.s2_options = Some(s2_options_vec);
    }

    pub unsafe fn selection(
        &mut self,
        state: &mut State,
        children: &mut HashMap<(usize, usize, usize), Box<[Node]>>,
        rng: &mut impl Rng,
    ) -> (*mut Node, usize, usize, f64, f64) {
        if self.s1_options.is_none() {
            let (s1_options, s2_options) = state.get_all_options();
            self.populate(s1_options, s2_options);
        }

        let (s1_index, s1_prob) = sample_regret_matching(self.s1_options.as_mut().unwrap(), rng);
        let (s2_index, s2_prob) = sample_regret_matching(self.s2_options.as_mut().unwrap(), rng);
        let key = (self as *mut Node as usize, s1_index, s2_index);
        match children.get_mut(&key) {
            Some(child_vector) => {
                let child_vec_ptr = child_vector as *mut Box<[Node]>;
                let chosen_child = self.sample_node(child_vec_ptr, rng);
                (*chosen_child).s1_prob = s1_prob;
                (*chosen_child).s2_prob = s2_prob;
                state.apply_instructions(&(*chosen_child).instructions.instruction_list);
                (*chosen_child).selection(state, children, rng)
            }
            None => (self as *mut Node, s1_index, s2_index, s1_prob, s2_prob),
        }
    }

    unsafe fn sample_node(&self, move_vector: *mut Box<[Node]>, rng: &mut impl Rng) -> *mut Node {
        let nodes = &mut **move_vector;

        let total_weight: f32 = nodes
            .iter()
            .map(|n| n.instructions.percentage.max(0.0))
            .sum();

        let mut threshold = rng.random_range(0.0..total_weight);

        for node in nodes.iter_mut() {
            threshold -= node.instructions.percentage.max(0.0);
            if threshold <= 0.0 {
                return node as *mut Node;
            }
        }

        // fallback: return last node (handles float rounding issues that can come up)
        &mut nodes[nodes.len() - 1] as *mut Node
    }

    pub unsafe fn expand(
        &mut self,
        state: &mut State,
        s1_move_index: usize,
        s2_move_index: usize,
        s1_prob: f64,
        s2_prob: f64,
        children: &mut HashMap<(usize, usize, usize), Box<[Node]>>,
        rng: &mut impl Rng,
    ) -> *mut Node {
        let s1_move = &self.s1_options.as_ref().unwrap()[s1_move_index].move_choice;
        let s2_move = &self.s2_options.as_ref().unwrap()[s2_move_index].move_choice;
        // if the battle is over or both moves are none there is no need to expand
        if (state.battle_is_over() != 0.0 && !self.root)
            || (s1_move == &MoveChoice::None && s2_move == &MoveChoice::None)
        {
            return self as *mut Node;
        }
        let should_branch_on_damage = self.root || (*self.parent).root;
        let mut new_instructions =
            generate_instructions_from_move_pair(state, s1_move, s2_move, should_branch_on_damage);
        let mut this_pair_vec = Vec::with_capacity(new_instructions.len());
        for state_instructions in new_instructions.drain(..) {
            let mut new_node = Node::new();
            new_node.parent = self;
            new_node.instructions = state_instructions;
            new_node.s1_choice = s1_move_index as u8;
            new_node.s2_choice = s2_move_index as u8;
            new_node.s1_prob = s1_prob;
            new_node.s2_prob = s2_prob;
            this_pair_vec.push(new_node);
        }

        // sample a node from the new instruction list.
        // this is the node that the rollout will be done on.
        // into_boxed_slice drops the Vec's spare capacity and, more importantly,
        // makes it a type that cannot be resized, which ensures the node
        // addresses are stable for the children map keys
        let mut boxed = this_pair_vec.into_boxed_slice();
        let new_node_ptr = self.sample_node(&mut boxed, rng);
        state.apply_instructions(&(*new_node_ptr).instructions.instruction_list);

        let key = (self as *mut Node as usize, s1_move_index, s2_move_index);
        children.insert(key, boxed);
        new_node_ptr
    }

    pub unsafe fn backpropagate(&mut self, score: f32, state: &mut State) {
        self.times_visited += 1;
        if self.root {
            return;
        }

        let parent = &mut *self.parent;
        update_regrets(
            parent.s1_options.as_mut().unwrap(),
            self.s1_choice as usize,
            self.s1_prob,
            score as f64,
        );
        update_regrets(
            parent.s2_options.as_mut().unwrap(),
            self.s2_choice as usize,
            self.s2_prob,
            1.0 - score as f64,
        );

        state.reverse_instructions(&self.instructions.instruction_list);
        parent.backpropagate(score, state);
    }

    // backpropagation for the multi-determinization search: identical to
    // backpropagate except the root ply's updates are skipped, because the
    // root's s1 strategy lives in the shared atomic table owned by the driver
    pub unsafe fn backpropagate_below_root(&mut self, score: f32, state: &mut State) {
        if self.root {
            return;
        }
        self.times_visited += 1;

        let parent = &mut *self.parent;
        if parent.root {
            state.reverse_instructions(&self.instructions.instruction_list);
            return;
        }
        update_regrets(
            parent.s1_options.as_mut().unwrap(),
            self.s1_choice as usize,
            self.s1_prob,
            score as f64,
        );
        update_regrets(
            parent.s2_options.as_mut().unwrap(),
            self.s2_choice as usize,
            self.s2_prob,
            1.0 - score as f64,
        );

        state.reverse_instructions(&self.instructions.instruction_list);
        parent.backpropagate_below_root(score, state);
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
pub struct ActionStats {
    pub move_choice: MoveChoice,
    pub cumulative_regret: f64,
    pub cumulative_strategy: f64,
    pub total_score: f32,
    pub visits: u32,
}

impl ActionStats {
    fn new(move_choice: MoveChoice) -> ActionStats {
        ActionStats {
            move_choice,
            cumulative_regret: 0.0,
            cumulative_strategy: 0.0,
            total_score: 0.0,
            visits: 0,
        }
    }
}

#[derive(Clone)]
pub struct CfrSideResult {
    pub move_choice: MoveChoice,

    // normalized average strategy: the probability this move should be
    // played with. sample from this rather than taking the argmax to
    // retain the equilibrium properties
    pub strategy: f32,

    pub total_score: f32,
    pub visits: u32,
}

impl CfrSideResult {
    pub fn average_score(&self) -> f32 {
        if self.visits == 0 {
            return 0.0;
        }
        self.total_score / self.visits as f32
    }
}

pub struct CfrResult {
    pub s1: Vec<CfrSideResult>,
    pub s2: Vec<CfrSideResult>,
    pub iteration_count: u32,
}

fn side_result(options: &[ActionStats]) -> Vec<CfrSideResult> {
    let strategy_sum: f64 = options.iter().map(|v| v.cumulative_strategy).sum();
    options
        .iter()
        .map(|v| CfrSideResult {
            move_choice: v.move_choice.clone(),
            strategy: if strategy_sum > 0.0 {
                (v.cumulative_strategy / strategy_sum) as f32
            } else {
                1.0 / options.len() as f32
            },
            total_score: v.total_score,
            visits: v.visits,
        })
        .collect()
}

fn cfr_iteration(
    root_node: &mut Node,
    state: &mut State,
    root_eval: &f32,
    children: &mut HashMap<(usize, usize, usize), Box<[Node]>>,
    rng: &mut impl Rng,
) {
    let (mut new_node, s1_move, s2_move, s1_prob, s2_prob) =
        unsafe { root_node.selection(state, children, rng) };
    new_node =
        unsafe { (*new_node).expand(state, s1_move, s2_move, s1_prob, s2_prob, children, rng) };
    let rollout_result = unsafe { (*new_node).rollout(state, root_eval) };
    unsafe { (*new_node).backpropagate(rollout_result, state) }
}

enum SearchLimit {
    Time(Duration),
    Iterations(u32),
}

fn run_cfr_loop(
    root_node: &mut Node,
    state: &mut State,
    root_eval: &f32,
    children: &mut HashMap<(usize, usize, usize), Box<[Node]>>,
    limit: SearchLimit,
) {
    let mut rng = rng();
    let start_time = std::time::Instant::now();
    loop {
        for _ in 0..1000 {
            cfr_iteration(root_node, state, root_eval, children, &mut rng);
        }
        if root_node.times_visited >= 10_000_000 {
            break;
        }
        match limit {
            SearchLimit::Time(max_time) => {
                if start_time.elapsed() >= max_time {
                    break;
                }
            }
            SearchLimit::Iterations(n) => {
                if root_node.times_visited >= n {
                    break;
                }
            }
        }
    }
}

pub fn perform_cfr(
    state: &mut State,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
    max_time: Duration,
    max_iterations: u32,
) -> CfrResult {
    let mut root_node = Node::new();
    root_node.populate(side_one_options, side_two_options);
    root_node.root = true;
    let mut children: HashMap<(usize, usize, usize), Box<[Node]>> = HashMap::new();

    let root_eval = evaluate(state);
    let search_limit = if max_iterations > 0 {
        SearchLimit::Iterations(max_iterations)
    } else {
        SearchLimit::Time(max_time)
    };
    run_cfr_loop(
        &mut root_node,
        state,
        &root_eval,
        &mut children,
        search_limit,
    );

    CfrResult {
        s1: side_result(root_node.s1_options.as_ref().unwrap()),
        s2: side_result(root_node.s2_options.as_ref().unwrap()),
        iteration_count: root_node.times_visited,
    }
}

// fixed-point scale for the shared atomic regrets. regrets reach magnitudes of
// ~1e9 in long searches; at this scale that is ~1e15, well within i64 range
const REGRET_SCALE: f64 = 1_000_000.0;

// one determinization's search tree. a self-contained unit of work: a worker
// thread locks a tree, runs iterations on it, and touches nothing outside it
// except the shared s1 table
struct DeterminizationTree<'a> {
    state: &'a mut State,
    root: Box<Node>,
    children: HashMap<(usize, usize, usize), Box<[Node]>>,
    root_eval: f32,
}

// SAFETY: the raw pointers inside a tree's nodes only ever reference other
// nodes owned by that same tree, and a tree is only accessed by the thread
// currently holding its mutex
unsafe impl Send for DeterminizationTree<'_> {}

// s1's root strategy, shared by every determinization. only the regrets are
// shared: they are the only field read during the search. strategy/score/visit
// accumulation is write-only until the end, so it stays thread-local
struct SharedS1 {
    regrets: Vec<AtomicI64>,
}

impl SharedS1 {
    fn snapshot_into(&self, buf: &mut Vec<f64>) {
        buf.clear();
        buf.extend(
            self.regrets
                .iter()
                .map(|r| r.load(Ordering::Relaxed) as f64 / REGRET_SCALE),
        );
    }

    fn update(&self, sampled: usize, sample_prob: f64, payoff: f64) {
        for (i, r) in self.regrets.iter().enumerate() {
            let delta = if i == sampled {
                payoff / sample_prob - payoff
            } else {
                -payoff
            };
            r.fetch_add((delta * REGRET_SCALE) as i64, Ordering::Relaxed);
        }
    }
}

#[derive(Clone, Default)]
struct LocalS1Stats {
    cumulative_strategy: f64,
    total_score: f32,
    visits: u32,
}

// mirrors sample_regret_matching against a snapshot of the shared regrets.
// concurrent workers may update the shared table between snapshot and update,
// so the sampling distribution can be slightly stale; regret matching is
// tolerant of this
fn sample_shared_s1(
    snapshot: &[f64],
    local: &mut [LocalS1Stats],
    rng: &mut impl Rng,
) -> (usize, f64) {
    let n = snapshot.len();
    let uniform = 1.0 / n as f64;
    let positive_regret_sum: f64 = snapshot.iter().map(|r| r.max(0.0)).sum();

    if positive_regret_sum > 0.0 {
        for (l, r) in local.iter_mut().zip(snapshot.iter()) {
            l.cumulative_strategy += r.max(0.0) / positive_regret_sum;
        }
    } else {
        for l in local.iter_mut() {
            l.cumulative_strategy += uniform;
        }
    }

    let index = if positive_regret_sum <= 0.0 || rng.random_bool(GAMMA) {
        rng.random_range(0..n)
    } else {
        let mut threshold = rng.random_range(0.0..positive_regret_sum);
        let mut chosen = n - 1;
        for (i, r) in snapshot.iter().enumerate() {
            threshold -= r.max(0.0);
            if threshold <= 0.0 {
                chosen = i;
                break;
            }
        }
        chosen
    };

    let sigma = if positive_regret_sum > 0.0 {
        snapshot[index].max(0.0) / positive_regret_sum
    } else {
        uniform
    };
    (index, (1.0 - GAMMA) * sigma + GAMMA * uniform)
}

// one cfr iteration on a single determinization. the root ply is handled here
// rather than by Node::selection because the root's s1 strategy lives in the
// shared table rather than in the node
unsafe fn multi_cfr_iteration(
    tree: &mut DeterminizationTree,
    shared: &SharedS1,
    local: &mut [LocalS1Stats],
    scratch: &mut Vec<f64>,
    rng: &mut impl Rng,
) {
    shared.snapshot_into(scratch);
    let (s1_index, s1_prob) = sample_shared_s1(scratch, local, rng);
    let root: *mut Node = &mut *tree.root;
    let (s2_index, s2_prob) = sample_regret_matching((*root).s2_options.as_mut().unwrap(), rng);

    let key = (root as usize, s1_index, s2_index);
    let leaf = match tree.children.get_mut(&key) {
        Some(child_vector) => {
            let child_vec_ptr = child_vector as *mut Box<[Node]>;
            let chosen_child = (*root).sample_node(child_vec_ptr, rng);
            (*chosen_child).s1_prob = s1_prob;
            (*chosen_child).s2_prob = s2_prob;
            tree.state
                .apply_instructions(&(*chosen_child).instructions.instruction_list);
            let (node, s1, s2, p1, p2) =
                (*chosen_child).selection(tree.state, &mut tree.children, rng);
            (*node).expand(tree.state, s1, s2, p1, p2, &mut tree.children, rng)
        }
        None => (*root).expand(
            tree.state,
            s1_index,
            s2_index,
            s1_prob,
            s2_prob,
            &mut tree.children,
            rng,
        ),
    };

    let score = (*leaf).rollout(tree.state, &tree.root_eval);
    (*leaf).backpropagate_below_root(score, tree.state);

    (*root).times_visited += 1;
    // expand can return the root itself when there is nothing to expand; the
    // single-determinization search records no updates in that case either
    if leaf != root {
        shared.update(s1_index, s1_prob, score as f64);
        local[s1_index].total_score += score;
        local[s1_index].visits += 1;
        update_regrets(
            (*root).s2_options.as_mut().unwrap(),
            s2_index,
            s2_prob,
            1.0 - score as f64,
        );
    }
}

pub struct CfrMultiResult {
    // s1's single strategy across all determinizations. sample from it
    pub s1: Vec<CfrSideResult>,
    pub iteration_count: u32,
    pub determinization_iterations: Vec<u32>,
}

fn sample_weighted(weights: &[f32], total_weight: f32, rng: &mut impl Rng) -> usize {
    if total_weight <= 0.0 {
        return rng.random_range(0..weights.len());
    }
    let mut threshold = rng.random_range(0.0..total_weight);
    for (i, w) in weights.iter().enumerate() {
        threshold -= w.max(0.0);
        if threshold <= 0.0 {
            return i;
        }
    }
    weights.len() - 1
}

// runs cfr across several possible states at once, weighted by belief. every
// determinization gets its own tree, but the roots share one s1 strategy table:
// s1 does not know which state is real, so its root strategy must be a single
// answer that does well against the weighted mixture.
// worker threads each sample a determinization, lock its tree, and run one
// iteration; the shared s1 regrets are the only cross-thread state
pub fn perform_cfr_multi(
    states: &mut [State],
    weights: &[f32],
    max_time: Duration,
    max_iterations: u32,
    threads: usize,
) -> CfrMultiResult {
    assert!(!states.is_empty());
    assert_eq!(states.len(), weights.len());

    // the shared table is indexed by option position, so s1's options must be
    // identical in every determinization. s1's own side is fully known, so this
    // only breaks if a sampled opponent set restricts s1 (e.g. trapping), which
    // callers are expected to avoid
    let (s1_options, _) = states[0].root_get_all_options();
    let shared = SharedS1 {
        regrets: (0..s1_options.len()).map(|_| AtomicI64::new(0)).collect(),
    };

    let trees: Vec<Mutex<DeterminizationTree>> = states
        .iter_mut()
        .map(|state| {
            let (this_s1_options, s2_options) = state.root_get_all_options();
            debug_assert_eq!(this_s1_options, s1_options);
            let mut root = Box::new(Node::new());
            root.root = true;
            // the root's own s1 stats are never read or updated: expand only
            // needs the move choices, and the strategy lives in the shared table
            root.populate(this_s1_options, s2_options);
            let root_eval = evaluate(state);
            Mutex::new(DeterminizationTree {
                state,
                root,
                children: HashMap::new(),
                root_eval,
            })
        })
        .collect();

    let total_weight: f32 = weights.iter().map(|w| w.max(0.0)).sum();
    let total_iterations = AtomicU32::new(0);
    let start_time = std::time::Instant::now();
    let worker_count = threads.max(1);

    let locals: Vec<Vec<LocalS1Stats>> = std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(worker_count);
        for _ in 0..worker_count {
            handles.push(s.spawn(|| {
                let mut rng = rng();
                let mut local = vec![LocalS1Stats::default(); shared.regrets.len()];
                let mut scratch: Vec<f64> = Vec::with_capacity(shared.regrets.len());
                'search: loop {
                    for _ in 0..100 {
                        let k = sample_weighted(weights, total_weight, &mut rng);
                        // block rather than resample: skipping busy trees would
                        // skew iterations away from the weight distribution and
                        // bias the shared regret updates. locks are only ever
                        // held for a single iteration
                        let mut tree = trees[k].lock().unwrap();
                        unsafe {
                            multi_cfr_iteration(
                                &mut tree,
                                &shared,
                                &mut local,
                                &mut scratch,
                                &mut rng,
                            );
                        }
                    }
                    let done = total_iterations.fetch_add(100, Ordering::Relaxed) + 100;
                    if done >= 10_000_000 {
                        break 'search;
                    }
                    if max_iterations > 0 {
                        if done >= max_iterations {
                            break 'search;
                        }
                    } else if start_time.elapsed() >= max_time {
                        break 'search;
                    }
                }
                local
            }));
        }
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    let mut merged: Vec<LocalS1Stats> = vec![LocalS1Stats::default(); shared.regrets.len()];
    for local in &locals {
        for (m, l) in merged.iter_mut().zip(local.iter()) {
            m.cumulative_strategy += l.cumulative_strategy;
            m.total_score += l.total_score;
            m.visits += l.visits;
        }
    }

    let strategy_sum: f64 = merged.iter().map(|m| m.cumulative_strategy).sum();
    let s1 = s1_options
        .iter()
        .zip(merged.iter())
        .map(|(move_choice, m)| CfrSideResult {
            move_choice: move_choice.clone(),
            strategy: if strategy_sum > 0.0 {
                (m.cumulative_strategy / strategy_sum) as f32
            } else {
                1.0 / s1_options.len() as f32
            },
            total_score: m.total_score,
            visits: m.visits,
        })
        .collect();

    CfrMultiResult {
        s1,
        iteration_count: total_iterations.load(Ordering::Relaxed),
        determinization_iterations: trees
            .iter()
            .map(|t| t.lock().unwrap().root.times_visited)
            .collect(),
    }
}
