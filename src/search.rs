use crate::generate_instructions::generate_instructions_from_move_pair;
use crate::state::{MoveChoice, Pokemon, State};
use std::collections::HashMap;
use crate::evaluate::evaluate;

/*
TODO:
- evaluation fn
- pick_safest fn
- alpha beta pruning
*/

const _WIN_BONUS: f32 = 1000.0;

pub fn expectiminimax_search(
    state: &mut State,
    depth: i8,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
    ab_prune: bool,
) -> Vec<f32> {
    let num_s1_moves = side_one_options.len();
    let num_s2_moves = side_two_options.len();
    let mut score_lookup: Vec<f32> = Vec::with_capacity(num_s1_moves * num_s2_moves);

    let battle_is_over = state.battle_is_over();
    if battle_is_over != 0.0 {
        evaluate(state) * 100.0;
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

                    let (_, safest) = pick_safest(
                        next_turn_side_one_options.len(),
                        next_turn_side_two_options.len(),
                        expectiminimax_search(
                            state,
                            depth - 1,
                            next_turn_side_one_options,
                            next_turn_side_two_options,
                            ab_prune,
                        )
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
    return score_lookup;
}

// fn get_vec_index(s1_index: usize, s2_index: usize, s2_len: usize) -> usize {
//     return s1_index * s2_len + s2_index;
// }

pub fn pick_safest(num_s1_moves: usize, num_s2_moves: usize, score_lookup: Vec<f32>) -> (usize, f32) {
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

    return (best_worst_case_s1_index, best_worst_case)
}
