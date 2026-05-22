use crate::engine::evaluate::evaluate;
use crate::engine::generate_instructions::generate_instructions_from_move_pair;
use crate::engine::state::MoveChoice;
use crate::instruction::StateInstructions;
use crate::mcts::{MctsResult, MctsSideResult};
use crate::state::State;
use dashmap::DashMap;
use rand::prelude::*;
use rand::rng;
use std::sync::atomic::{AtomicI8, AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

const MCTS_DEADLINE_CHECK_INTERVAL: u32 = 1_000;
const MCTS_MAX_ITERATIONS_PER_TREE: u32 = 10_000_000;
const MCTS_DAMAGE_BRANCH_DEPTH: u8 = 2;
const SCORE_SCALE: f32 = 400.0;
const VIRTUAL_LOSS_VISITS: u32 = 3;

// Node map type alias for clarity.
// key: (parent node address, s1_move_index, s2_move_index)
// value: the branch (weighted list of outcome nodes for that move pair)
type ChildMap = DashMap<(usize, usize, usize), SharedBranch>;

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

    fn add_virtual_loss(&self) {
        self.visits.fetch_add(VIRTUAL_LOSS_VISITS, Ordering::AcqRel);
    }

    fn remove_virtual_loss(&self) {
        self.visits.fetch_sub(VIRTUAL_LOSS_VISITS, Ordering::AcqRel);
    }

    fn add_result(&self, score: f32) {
        self.total_score
            .fetch_add((score * SCORE_SCALE).round() as u32, Ordering::AcqRel);
        self.visits.fetch_add(1, Ordering::AcqRel);
    }

    fn total_score_f32(&self) -> f32 {
        self.total_score.load(Ordering::Acquire) as f32 / SCORE_SCALE
    }

    fn ucb1(&self, parent_visits: u32) -> f32 {
        let visits = self.visits.load(Ordering::Acquire);
        if visits == 0 {
            return f32::INFINITY;
        }
        let average_score = self.total_score_f32() / visits as f32;
        let exploration = 2.0 * (parent_visits as f32).ln().max(0.0) / visits as f32;
        average_score + exploration.sqrt()
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

pub struct SharedBranch {
    nodes: Arc<[Node]>,
    total_weight: f32,
}

impl SharedBranch {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> *const Node {
        if self.nodes.len() <= 1 || self.total_weight <= 0.0 {
            return &self.nodes[0];
        }
        let mut threshold = rng.random_range(0.0..self.total_weight);
        for node in self.nodes.iter() {
            threshold -= node.instructions.percentage.max(0.0);
            if threshold <= 0.0 {
                return node;
            }
        }
        &self.nodes[self.nodes.len() - 1]
    }
}

struct PathStep {
    parent: *const Node,
    child: *const Node,
    s1_index: usize,
    s2_index: usize,
}

pub struct Node {
    root: bool,
    instructions: StateInstructions,
    depth: u8,
    times_visited: AtomicU32,
    virtual_losses: AtomicI8,
    options: OnceLock<SharedNodeOptions>,
}

impl Node {
    fn new_root(s1_options: Vec<MoveChoice>, s2_options: Vec<MoveChoice>) -> Arc<Self> {
        let node = Arc::new(Self {
            root: true,
            instructions: StateInstructions::default(),
            depth: 0,
            times_visited: AtomicU32::new(0),
            virtual_losses: AtomicI8::new(0),
            options: OnceLock::new(),
        });
        let _ = node
            .options
            .set(SharedNodeOptions::new(s1_options, s2_options));
        node
    }

    fn new_child(instructions: StateInstructions, depth: u8) -> Self {
        Self {
            root: false,
            instructions,
            depth,
            times_visited: AtomicU32::new(0),
            virtual_losses: AtomicI8::new(0),
            options: OnceLock::new(),
        }
    }

    fn as_key(&self) -> usize {
        self as *const Node as usize
    }

    fn ensure_options(&self, state: &State) -> &SharedNodeOptions {
        self.options.get_or_init(|| {
            let (s1, s2) = state.get_all_options();
            SharedNodeOptions::new(s1, s2)
        })
    }

    fn select_move_pair(&self, state: &State) -> (usize, usize) {
        let options = self.ensure_options(state);
        let parent_visits = self
            .times_visited
            .load(Ordering::Acquire)
            .saturating_add(self.virtual_losses.load(Ordering::Acquire).max(0) as u32)
            .max(1);
        (
            self.maximize_ucb_for_side(&options.s1, parent_visits),
            self.maximize_ucb_for_side(&options.s2, parent_visits),
        )
    }

    fn selection<R: Rng + ?Sized>(
        root: &Arc<Node>,
        state: &mut State,
        rng: &mut R,
        children: &ChildMap,
        path: &mut Vec<PathStep>,
    ) -> (*const Node, usize, usize) {
        // raw pointers walk both the root (a standalone Arc<Node>) and children
        // (Nodes living inside a branch's Arc<[Node]>) uniformly. every node is
        // owned by children/root for the whole search, so the pointers stay
        // valid
        let mut current: *const Node = Arc::as_ptr(root);
        loop {
            let node = unsafe { &*current };
            let (s1_index, s2_index) = node.select_move_pair(state);
            let options = node.options.get().expect("options set during selection");

            let key = (node.as_key(), s1_index, s2_index);
            match children.get(&key) {
                Some(branch) => {
                    let child = branch.sample(rng);

                    // drop the DashMap ref before mutating state to avoid
                    // holding the lock any longer than necessary. the sampled
                    // node stays alive via the branch's Arc<[Node]> in the
                    // ChildMap
                    drop(branch);

                    let child_ref = unsafe { &*child };
                    options.s1[s1_index].add_virtual_loss();
                    options.s2[s2_index].add_virtual_loss();
                    child_ref.virtual_losses.fetch_add(1, Ordering::AcqRel);
                    state.apply_instructions(&child_ref.instructions.instruction_list);
                    path.push(PathStep {
                        parent: current,
                        child,
                        s1_index,
                        s2_index,
                    });
                    current = child;
                }
                None => {
                    // this is the leaf, stop selection
                    return (current, s1_index, s2_index);
                }
            }
        }
    }

    fn maximize_ucb_for_side(&self, side_options: &[MoveNode], parent_visits: u32) -> usize {
        side_options
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.ucb1(parent_visits)
                    .partial_cmp(&b.ucb1(parent_visits))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// looks up or creates the child branch for `(s1_index, s2_index)` and
    /// returns one sampled child, applying virtual loss bookkeeping.  Returns
    /// `None` when the node should not be expanded (battle over, both-None).
    fn expand<R: Rng + ?Sized>(
        &self,
        state: &mut State,
        s1_index: usize,
        s2_index: usize,
        rng: &mut R,
        children: &ChildMap,
    ) -> Option<*const Node> {
        let options = self
            .options
            .get()
            .expect("options initialised before expand");
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

        let mut total_weight = 0.0f32;
        let nodes = instructions
            .into_iter()
            .map(|instr| {
                total_weight += instr.percentage.max(0.0);
                Node::new_child(instr, self.depth.saturating_add(1))
            })
            .collect::<Arc<[Node]>>();
        let branch = SharedBranch {
            nodes,
            total_weight,
        };

        let key = (self.as_key(), s1_index, s2_index);
        // entry() on DashMap is atomic per-shard: only one thread will
        // construct the branch; all others get the winner's branch.
        let branch_ref = children.entry(key).or_insert(branch);

        Some(branch_ref.sample(rng))
    }

    fn rollout(&self, state: &State, root_eval: f32) -> f32 {
        let battle_is_over = state.battle_is_over();
        if battle_is_over == 0.0 {
            sigmoid(evaluate(state) - root_eval)
        } else if battle_is_over == -1.0 {
            0.0
        } else {
            battle_is_over
        }
    }

    // walk `path` in reverse, updating visit counts and scores,
    // removes virtual losses, and reverse-applying instructions to restore `state` to how it
    // was in the root
    fn backpropagate(path: &[PathStep], leaf: &Node, score: f32, state: &mut State) {
        leaf.times_visited.fetch_add(1, Ordering::AcqRel);

        for step in path.iter().rev() {
            let (parent, child) = unsafe { (&*step.parent, &*step.child) };
            let options = parent.options.get().expect("path parent has options");
            options.s1[step.s1_index].add_result(score);
            options.s1[step.s1_index].remove_virtual_loss();
            options.s2[step.s2_index].add_result(1.0 - score);
            options.s2[step.s2_index].remove_virtual_loss();
            parent.times_visited.fetch_add(1, Ordering::AcqRel);
            child.virtual_losses.fetch_sub(1, Ordering::AcqRel);
            state.reverse_instructions(&child.instructions.instruction_list);
        }
    }
}

fn do_mcts<R: Rng + ?Sized>(
    root: &Arc<Node>,
    state: &mut State,
    root_eval: f32,
    rng: &mut R,
    children: &ChildMap,
    path: &mut Vec<PathStep>,
) {
    path.clear();

    let (leaf, s1_index, s2_index) = Node::selection(root, state, rng, children, path);
    let leaf = unsafe { &*leaf };

    let options = leaf.options.get().expect("options set during selection");
    options.s1[s1_index].add_virtual_loss();
    options.s2[s2_index].add_virtual_loss();
    let expanded = leaf.expand(state, s1_index, s2_index, rng, children);
    match expanded {
        Some(child) => {
            let child = unsafe { &*child };
            child.virtual_losses.fetch_add(1, Ordering::AcqRel);
            state.apply_instructions(&child.instructions.instruction_list);
            path.push(PathStep {
                parent: leaf,
                child,
                s1_index,
                s2_index,
            });

            let score = child.rollout(state, root_eval);

            Node::backpropagate(path, child, score, state);
        }

        // if expansion returns None,
        // the battle is either over or both sides have no valid moves
        // so no child is added to the tree
        // we do a rollout on the leaf and backpropagate without adding a child to the tree
        None => {
            // remove the virtual loss we added before expansion, since we're not actually expanding
            options.s1[s1_index].remove_virtual_loss();
            options.s2[s2_index].remove_virtual_loss();

            let score = leaf.rollout(state, root_eval);

            Node::backpropagate(path, leaf, score, state);
        }
    }
}

pub fn perform_mcts_shared_tree(
    state: &mut State,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
    max_time: Duration,
    worker_count: usize,
) -> MctsResult {
    let root_eval = evaluate(state);
    let deadline = Instant::now() + max_time;
    let root = Node::new_root(side_one_options, side_two_options);
    let started_iterations = Arc::new(AtomicU32::new(0));

    // global map shared by all threads.
    let children: Arc<ChildMap> = Arc::new(DashMap::with_capacity(1 << 16));

    thread::scope(|scope| {
        for _ in 0..worker_count {
            let root = root.clone();
            let started_iterations = started_iterations.clone();
            let children = children.clone();
            let mut worker_state = state.clone();
            scope.spawn(move || {
                let mut rng = rng();
                let mut iterations_until_deadline_check = 0u32;
                let mut path = Vec::with_capacity(16);

                loop {
                    if iterations_until_deadline_check == 0 {
                        if Instant::now() >= deadline {
                            break;
                        }
                        iterations_until_deadline_check = MCTS_DEADLINE_CHECK_INTERVAL;
                    }
                    if started_iterations.fetch_add(1, Ordering::AcqRel)
                        >= MCTS_MAX_ITERATIONS_PER_TREE
                    {
                        break;
                    }

                    do_mcts(
                        &root,
                        &mut worker_state,
                        root_eval,
                        &mut rng,
                        &children,
                        &mut path,
                    );
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
