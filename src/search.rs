use crate::evaluate::evaluate;
use crate::generate_instructions::generate_instructions_from_move_pair;
use crate::state::{MoveChoice, State};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

enum IterativeDeependingThreadMessage {
    Stop((Vec<MoveChoice>, Vec<MoveChoice>, Vec<f32>, i8)),
}

pub fn expectiminimax_search(
    state: &mut State,
    mut depth: i8,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
    ab_prune: bool,
    mtx: &Arc<Mutex<bool>>,
) -> Vec<f32> {
    depth -= 1;
    let num_s1_moves = side_one_options.len();
    let num_s2_moves = side_two_options.len();
    let mut score_lookup: Vec<f32> = Vec::with_capacity(num_s1_moves * num_s2_moves);

    if *mtx.lock().unwrap() == false {
        for _ in 0..(num_s1_moves * num_s2_moves) {
            score_lookup.push(0.0);
        }
        return score_lookup;
    }

    let battle_is_over = state.battle_is_over();
    if battle_is_over != 0.0 {
        for _ in 0..(num_s1_moves * num_s2_moves) {
            score_lookup.push(((100.0 * depth as f32) * battle_is_over) + evaluate(state));
        }
        return score_lookup;
    }

    let mut skip;
    let mut alpha = f32::MIN;
    for side_one_move in side_one_options.iter().as_ref() {
        let mut beta = f32::MAX;
        skip = false;

        for side_two_move in side_two_options.iter().as_ref() {
            if skip {
                score_lookup.push(f32::NAN);
                continue;
            }

            let mut score = 0.0;
            let instructions =
                generate_instructions_from_move_pair(state, &side_one_move, &side_two_move);
            if depth == 0 {
                for instruction in instructions.iter() {
                    state.apply_instructions(&instruction.instruction_list);
                    score += instruction.percentage * evaluate(state) / 100.0;
                    state.reverse_instructions(&instruction.instruction_list);
                }
            } else {
                for instruction in instructions.iter() {
                    state.apply_instructions(&instruction.instruction_list);
                    let (next_turn_side_one_options, next_turn_side_two_options) =
                        state.get_all_options();

                    let next_turn_side_one_options_len = next_turn_side_one_options.len();
                    let next_turn_side_two_options_len = next_turn_side_two_options.len();
                    let (_, safest) = pick_safest(
                        &expectiminimax_search(
                            state,
                            depth,
                            next_turn_side_one_options,
                            next_turn_side_two_options,
                            true, // until there is something better than `pick_safest` for evaluating a sub-game, there is no point in this being anything other than `true`
                            &mtx,
                        ),
                        next_turn_side_one_options_len,
                        next_turn_side_two_options_len,
                    );
                    score += instruction.percentage * safest / 100.0;

                    state.reverse_instructions(&instruction.instruction_list);
                }
            }
            score_lookup.push(score);

            if ab_prune {
                if score < beta {
                    beta = score;
                }
                if score <= alpha {
                    skip = true;
                }
            }
        }
        if beta > alpha {
            alpha = beta;
        }
    }
    score_lookup
}

pub fn pick_safest(
    score_lookup: &Vec<f32>,
    num_s1_moves: usize,
    num_s2_moves: usize,
) -> (usize, f32) {
    let mut best_worst_case = f32::MIN;
    let mut best_worst_case_s1_index = 0;
    let mut vec_index = 0;

    for s1_index in 0..num_s1_moves {
        let mut worst_case_this_row = f32::MAX;
        for _ in 0..num_s2_moves {
            let score = score_lookup[vec_index];
            vec_index += 1;
            if score < worst_case_this_row {
                worst_case_this_row = score;
            }
        }
        if worst_case_this_row > best_worst_case {
            best_worst_case_s1_index = s1_index;
            best_worst_case = worst_case_this_row;
        }
    }

    (best_worst_case_s1_index, best_worst_case)
}

fn re_order_moves_for_iterative_deepening(
    last_search_result: &Vec<f32>,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
    let num_s1_moves = side_one_options.len();
    let num_s2_moves = side_two_options.len();
    let mut worst_case_s1_scores: Vec<(MoveChoice, f32)> = vec![];
    let mut vec_index = 0;

    for s1_index in 0..num_s1_moves {
        let mut worst_case_this_row = f32::MAX;
        for _ in 0..num_s2_moves {
            let score = last_search_result[vec_index];
            vec_index += 1;
            if score < worst_case_this_row {
                worst_case_this_row = score;
            }
        }
        worst_case_s1_scores.push((side_one_options[s1_index].clone(), worst_case_this_row));
    }

    worst_case_s1_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let new_s1_vec = worst_case_s1_scores.iter().map(|x| x.0.clone()).collect();

    (new_s1_vec, side_two_options)
}

pub fn iterative_deepen_expectiminimax(
    state: &mut State,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
    max_time: Duration,
) -> (Vec<MoveChoice>, Vec<MoveChoice>, Vec<f32>, i8) {
    let mut state_clone = state.clone();

    let mut result = expectiminimax_search(
        state,
        1,
        side_one_options.clone(),
        side_two_options.clone(),
        true,
        &Arc::new(Mutex::new(true)),
    );
    let (mut re_ordered_s1_options, mut re_ordered_s2_options) =
        re_order_moves_for_iterative_deepening(&result, side_one_options, side_two_options);
    let mut i = 1;
    let running = Arc::new(Mutex::new(true));
    let running_clone = Arc::clone(&running);

    let (sender, receiver): (
        Sender<IterativeDeependingThreadMessage>,
        Receiver<IterativeDeependingThreadMessage>,
    ) = channel();

    let handle = thread::spawn(move || {
        let mut previous_turn_s1_options = re_ordered_s1_options.clone();
        let mut previous_turn_s2_options = re_ordered_s2_options.clone();
        loop {
            let previous_result = result;
            i += 1;
            result = expectiminimax_search(
                &mut state_clone,
                i,
                re_ordered_s1_options.clone(),
                re_ordered_s2_options.clone(),
                true,
                &running_clone,
            );

            // when we are told to stop, return the *previous* result.
            // the current result will be invalid
            if *running_clone.lock().unwrap() == false {
                sender
                    .send(IterativeDeependingThreadMessage::Stop((
                        previous_turn_s1_options,
                        previous_turn_s2_options,
                        previous_result,
                        i - 1,
                    )))
                    .unwrap();
                break;
            }
            previous_turn_s1_options = re_ordered_s1_options.clone();
            previous_turn_s2_options = re_ordered_s2_options.clone();
            (re_ordered_s1_options, re_ordered_s2_options) = re_order_moves_for_iterative_deepening(
                &result,
                re_ordered_s1_options,
                re_ordered_s2_options,
            );
        }
    });

    thread::sleep(max_time);
    *running.lock().unwrap() = false;
    match receiver.recv() {
        Ok(IterativeDeependingThreadMessage::Stop(result)) => {
            handle.join().unwrap();
            result
        }
        _ => panic!("Failed to receive stop message"),
    }
}
