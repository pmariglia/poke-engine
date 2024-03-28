use crate::generate_instructions::generate_instructions_from_move_pair;
use crate::state::{MoveChoice, State};
use std::collections::HashMap;

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
) -> HashMap<(MoveChoice, MoveChoice), f32> {
    let score_lookup: HashMap<(MoveChoice, MoveChoice), f32> = HashMap::new();

    let battle_is_over = state.battle_is_over();
    if battle_is_over != 0.0 {
        // score_lookup.insert(
        // (MoveChoice::NONE, MoveChoice::NONE),
        // evaluate(state) * WIN_BONUS * battle_is_over,
        // );
        return score_lookup;
    }

    for side_one_move in side_one_options.iter().as_ref() {
        for side_two_move in side_two_options.iter().as_ref() {
            let mut score = 0.0;
            let instructions =
                generate_instructions_from_move_pair(state, &side_one_move, &side_two_move);
            if depth == 0 {
                for instruction in instructions.iter() {
                    state.apply_instructions(&instruction.instruction_list);
                    score += instruction.percentage * evaluate(state);
                    state.reverse_instructions(&instruction.instruction_list);
                }
            } else {
                for instruction in instructions.iter() {
                    state.apply_instructions(&instruction.instruction_list);
                    let (next_turn_side_one_options, next_turn_side_two_options) =
                        state.get_all_options();

                    expectiminimax_search(
                        state,
                        depth - 1,
                        next_turn_side_one_options,
                        next_turn_side_two_options,
                        ab_prune,
                    );

                    state.reverse_instructions(&instruction.instruction_list);
                }
            }
            // score_lookup.insert((*side_one_move, *side_two_move), score);
        }
    }

    return score_lookup;
}

fn evaluate(_state: &State) -> f32 {
    return 0.0;
}

// fn pick_safest(score_lookup: HashMap<(MoveChoice, MoveChoice), f32>) -> f32 {
//     let mut worst_cases: HashMap<MoveChoice, f32> = HashMap::new();
//     for (key, value) in &score_lookup {
//         let this_worst_case = worst_cases.get(&key.0);
//         match this_worst_case {
//             Some(x) => {
//                 if value < x {
//                     worst_cases.insert(key.0.clone(), value.clone());
//                 }
//             }
//             NONE => {
//                 worst_cases.insert(key.0.clone(), value.clone());
//             }
//         }
//     }
//     return worst_cases
//         .values()
//         .max_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap()
//         .clone();
// }
