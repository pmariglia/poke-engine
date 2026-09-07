use crate::engine::evaluate::evaluate;
use crate::engine::generate_instructions::generate_instructions_from_move_pair;
use crate::engine::state::MoveChoice;
use crate::instruction::StateInstructions;
use crate::state::State;
use rand::prelude::*;
use rand::rng;
use std::collections::HashMap;
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

    // in a multi-determinization search the root nodes of every determinization
    // share a single s1 table: s1 cannot distinguish the determinizations, so it
    // must play one strategy across all of them. non-null only on such roots
    pub shared_s1: *mut Vec<ActionStats>,
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
            shared_s1: std::ptr::null_mut(),
        }
    }

    unsafe fn s1_stats_mut(&mut self) -> &mut Vec<ActionStats> {
        if self.shared_s1.is_null() {
            self.s1_options.as_mut().unwrap()
        } else {
            &mut *self.shared_s1
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
        // s2_options rather than s1_options: a multi-determinization root has its
        // s2 table populated but keeps s1_options empty in favour of shared_s1
        if self.s2_options.is_none() {
            let (s1_options, s2_options) = state.get_all_options();
            self.populate(s1_options, s2_options);
        }

        let (s1_index, s1_prob) = sample_regret_matching(self.s1_stats_mut(), rng);
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
        let s1_move = self.s1_stats_mut()[s1_move_index].move_choice.clone();
        let s2_move = self.s2_options.as_ref().unwrap()[s2_move_index]
            .move_choice
            .clone();
        // if the battle is over or both moves are none there is no need to expand
        if (state.battle_is_over() != 0.0 && !self.root)
            || (s1_move == MoveChoice::None && s2_move == MoveChoice::None)
        {
            return self as *mut Node;
        }
        let should_branch_on_damage = self.root || (*self.parent).root;
        let mut new_instructions = generate_instructions_from_move_pair(
            state,
            &s1_move,
            &s2_move,
            should_branch_on_damage,
        );
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
            parent.s1_stats_mut(),
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

// one determinization's search tree, kept as a self-contained unit so that a
// future threaded version can hand whole determinizations to worker threads,
// leaving the shared s1 table as the only cross-thread state
struct DeterminizationTree {
    root: Box<Node>,
    children: HashMap<(usize, usize, usize), Box<[Node]>>,
    root_eval: f32,
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
// answer that does well against the weighted mixture
pub fn perform_cfr_multi(
    states: &mut [State],
    weights: &[f32],
    max_time: Duration,
    max_iterations: u32,
) -> CfrMultiResult {
    assert!(!states.is_empty());
    assert_eq!(states.len(), weights.len());

    // the shared table is indexed by option position, so s1's options must be
    // identical in every determinization. s1's own side is fully known, so this
    // only breaks if a sampled opponent set restricts s1 (e.g. trapping), which
    // callers are expected to avoid
    let (s1_options, _) = states[0].root_get_all_options();
    let mut shared_s1: Box<Vec<ActionStats>> = Box::new(
        s1_options
            .iter()
            .map(|x| ActionStats::new(x.clone()))
            .collect(),
    );
    let shared_s1_ptr: *mut Vec<ActionStats> = &mut *shared_s1;

    let mut trees: Vec<DeterminizationTree> = Vec::with_capacity(states.len());
    for state in states.iter_mut() {
        let (this_s1_options, s2_options) = state.root_get_all_options();
        debug_assert_eq!(this_s1_options, s1_options);
        let mut root = Box::new(Node::new());
        root.root = true;
        root.shared_s1 = shared_s1_ptr;
        root.s2_options = Some(
            s2_options
                .iter()
                .map(|x| ActionStats::new(x.clone()))
                .collect(),
        );
        trees.push(DeterminizationTree {
            root,
            children: HashMap::new(),
            root_eval: evaluate(state),
        });
    }

    let total_weight: f32 = weights.iter().map(|w| w.max(0.0)).sum();
    let mut rng = rng();
    let start_time = std::time::Instant::now();
    let mut total_iterations: u32 = 0;
    loop {
        for _ in 0..1000 {
            let k = sample_weighted(weights, total_weight, &mut rng);
            let tree = &mut trees[k];
            cfr_iteration(
                &mut tree.root,
                &mut states[k],
                &tree.root_eval,
                &mut tree.children,
                &mut rng,
            );
        }
        total_iterations += 1000;
        if total_iterations >= 10_000_000 {
            break;
        }
        if max_iterations > 0 {
            if total_iterations >= max_iterations {
                break;
            }
        } else if start_time.elapsed() >= max_time {
            break;
        }
    }

    CfrMultiResult {
        s1: side_result(&shared_s1),
        iteration_count: total_iterations,
        determinization_iterations: trees.iter().map(|t| t.root.times_visited).collect(),
    }
}
