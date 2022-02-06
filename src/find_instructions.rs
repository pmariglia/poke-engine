use std::process;

use ipc_channel::ipc::{self, IpcOneShotServer, IpcReceiver, IpcSender};
use nix::unistd::{fork, ForkResult};
use serde::{Deserialize, Serialize};

use super::instruction::Instruction;
use super::instruction::SwitchInstruction;
use super::state::Pokemon;
use super::state::Side;
use super::state::State;
use crate::data::abilities::get_ability;
use crate::data::conditions::Status;
use crate::data::items::get_item;
use crate::data::moves::get_move;
use crate::data::moves::Move;
use crate::data::moves::SideCondition;

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Move,
    Switch,
}

#[derive(Debug)]
pub struct MoveChoice {
    // Specifies the move used on a turn
    // Names are hard
    pub move_type: MoveType,
    pub choice: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransposeInstruction {
    pub state: State,
    pub percentage: f32,
    pub instructions: Vec<Instruction>,
}

pub fn forking_random_chance(
    transpose_instruction: &mut TransposeInstruction,
    chance: f32,
) -> bool {
    /*
    Forks and returns both `true` and `false` in the parent and child respectively

    Modifies the `percentage` attribute of the TransposeInstruction based on the `chance` parameter

    This is used when a random event occurs in pokemon so that two processes can explore both options.
    */

    if chance >= 1.0 {
        return true;
    } else if chance <= 0.0 {
        return false;
    }

    unsafe {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                transpose_instruction.percentage *= chance;
                return true;
            }
            Ok(ForkResult::Child) => {
                transpose_instruction.percentage *= 1.0 - chance;
                return false;
            }
            Err(_) => {
                panic!("Fork failed");
            }
        }
    }
}

fn get_boost_multiplier(boost: i8) -> f32 {
    match boost {
        i if i < 0 => {
            return 2.0 / (i.abs() + 2) as f32;
        }
        i if i == 0 => {
            return 1.0;
        }
        i if i > 0 => {
            return ((i + 2) as f32) / 2.0;
        }
        _ => panic!("Got bad value for boost: {}", boost),
    }
}

pub fn get_effective_speed(state: &State, side: &Side) -> i16 {
    let side_active: &Pokemon = side.get_active_immutable();

    let mut effective_speed = side_active.speed as f32;

    effective_speed = (effective_speed * get_boost_multiplier(side_active.speed_boost)).floor();

    match get_ability(side_active.ability.as_str()).modify_speed {
        Some(ability_func) => {
            effective_speed = (effective_speed * ability_func(state, side_active)).floor();
        }
        None => {}
    }

    match get_item(side_active.item.as_str()).modify_speed {
        Some(item_func) => {
            effective_speed = (effective_speed * item_func(state, side_active)).floor();
        }
        None => {}
    }

    if side
        .side_conditions
        .get(&SideCondition::Tailwind)
        .unwrap_or(&0)
        > &0
    {
        effective_speed = effective_speed * 2.0
    }

    if side_active.status == Status::Paralyze && side_active.ability != "quickfeet" {
        effective_speed = (effective_speed / 2.0).floor()
    }

    return effective_speed as i16;
}

pub fn get_effective_priority(state: &State, side: &Side, move_name: &str) -> i8 {
    let move_obj: &Move = get_move(move_name);
    let side_active: &Pokemon = side.get_active_immutable();
    let mut priority = move_obj.priority;

    match get_ability(side_active.ability.as_str()).modify_priority {
        Some(modify_priority_fn) => {
            priority += modify_priority_fn(move_name, side_active);
        }
        None => {}
    }

    match get_move(move_name).modify_priority {
        Some(modify_priority_fn) => {
            priority += modify_priority_fn(&state);
        }
        None => {}
    }

    return priority;
}

pub fn side_one_moves_first(
    state: &State,
    side_one_move: &MoveChoice,
    side_two_move: &MoveChoice,
) -> bool {
    let side_one_effective_speed: i16 = get_effective_speed(state, &state.side_one);
    let side_two_effective_speed: i16 = get_effective_speed(state, &state.side_two);

    if side_one_move.move_type == MoveType::Switch && side_two_move.move_type == MoveType::Switch {
        return side_one_effective_speed > side_two_effective_speed;
    } else if side_one_move.move_type == MoveType::Switch {
        if side_two_move.choice == "pursuit" {
            return false;
        }
        return true;
    } else if side_two_move.move_type == MoveType::Switch {
        if side_one_move.choice == "pursuit" {
            return true;
        }
        return false;
    }

    let side_one_priority = get_effective_priority(&state, &state.side_one, &side_one_move.choice);
    let side_two_priority = get_effective_priority(&state, &state.side_two, &side_two_move.choice);

    if side_one_priority == side_two_priority {
        if state.trick_room {
            return side_two_effective_speed >= side_one_effective_speed;
        } else {
            return side_one_effective_speed > side_two_effective_speed;
        }
    } else {
        return side_one_priority > side_two_priority;
    }
}

pub fn run_switch(
    transpose_instruction: &mut TransposeInstruction,
    is_side_one: bool,
    switch_pokemon: &String,
) {
    /*
    Events handlers for switching:
        - before_switch
        - after_switch

    */

    let side = if is_side_one {
        &mut transpose_instruction.state.side_one
    } else {
        &mut transpose_instruction.state.side_two
    };

    let previous_index = side.active_index;
    side.switch_to_name(switch_pokemon);
    let new_instructions = SwitchInstruction {
        is_side_one: is_side_one,
        previous_index: previous_index,
        next_index: side.active_index,
    };
    transpose_instruction
        .instructions
        .push(Instruction::SwitchInstruction(new_instructions));
}

pub fn run_move(
    transpose_instruction: &mut TransposeInstruction,
    is_side_one: bool,
    move_choice: &MoveChoice,
) {
    /*
    run switch (if it is a switch)
    run before_move (fully paralyzed, flinch, asleep, burned)
    run modify_move (moves can change based on special-effects (weatherball basepower/type change) )
    run move_hit_chance (stop in the "miss" scenario)
    run get_damage
        (the entire damage calc algorithm, ideally self-contained and is accurate on it's own)
            run get_boosted_stats (change stats based on boosts, abilities (solarpower), etc)
            run get_stab
    run apply_damage
    run heal
    run status
    run move_special_effect (requires some sort of module with code for moves' special effects)
        hazard clear
        weather setting
        terrain setting
        trickroom setting
        trick/switcheroo
        boost-clearing (haze)
    run recoil
    run drain
    if (move_hit):
        run apply_secondary
    */

    if move_choice.move_type == MoveType::Switch {
        run_switch(transpose_instruction, is_side_one, &move_choice.choice);
    }
}

pub fn run_turn(
    transpose_instruction: &mut TransposeInstruction,
    side_one_move: MoveChoice,
    side_two_move: MoveChoice,
) {
    let side_one_moves_first =
        side_one_moves_first(&transpose_instruction.state, &side_one_move, &side_two_move);

    if side_one_moves_first {
        run_move(transpose_instruction, true, &side_one_move);
        run_move(transpose_instruction, false, &side_two_move);
    } else {
        run_move(transpose_instruction, false, &side_two_move);
        run_move(transpose_instruction, true, &side_one_move);
    }
    /*
    Do end-of-turn shenanigans
    */
}

pub fn find_all_instructions(
    state: State,
    side_one_move: MoveChoice,
    side_two_move: MoveChoice,
) -> Vec<TransposeInstruction> {
    let mut transpose_instruction: TransposeInstruction = TransposeInstruction {
        state: state,
        percentage: 1.0,
        instructions: vec![],
    };

    let (server, name) = IpcOneShotServer::new().unwrap();

    unsafe {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                /*
                Parent waits for all children to send their results via IPC
                Parent knows the children are complete when the cumulative change reaches 1.0
                */
                let (tx1, rx1): (
                    IpcSender<TransposeInstruction>,
                    IpcReceiver<TransposeInstruction>,
                ) = ipc::channel().unwrap();
                let tx0 = IpcSender::connect(name).unwrap();
                tx0.send(tx1).unwrap();

                let mut list_of_instructions: Vec<TransposeInstruction> =
                    Vec::<TransposeInstruction>::new();
                let mut cumulative_chance: f32 = 0.0;
                while cumulative_chance < 1.0 {
                    let result = rx1.recv().unwrap();
                    cumulative_chance += result.percentage;
                    list_of_instructions.push(result);
                }

                return list_of_instructions;
            }
            Ok(ForkResult::Child) => {
                /*
                Child is responsible for running the turn.
                The child may fork itself again if an event occurs with more than one path.
                */
                let (_, tx1): (_, IpcSender<TransposeInstruction>) = server.accept().unwrap();
                run_turn(&mut transpose_instruction, side_one_move, side_two_move);
                tx1.send(transpose_instruction).unwrap();
                process::exit(0);
            }
            Err(_) => {
                panic!("Fork failed");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::helpers::create_dummy_state;

    use super::super::find_instructions::Instruction;
    use super::super::find_instructions::SwitchInstruction;
    use super::super::state::State;
    use super::super::state::Terrain;

    use crate::data::conditions::Status;
    use crate::data::moves::SideCondition;

    use super::get_effective_priority;
    use super::get_effective_speed;
    use super::run_switch;
    use super::side_one_moves_first;
    use super::MoveChoice;
    use super::MoveType;
    use super::TransposeInstruction;

    #[test]
    fn test_get_effective_priority_returns_zero_for_typical_move() {
        let state: State = create_dummy_state();

        let effective_priority = get_effective_priority(&state, &state.side_one, "tackle");

        assert_eq!(effective_priority, 0);
    }

    #[test]
    fn test_get_effective_priority_returns_one_for_quickattack() {
        let state: State = create_dummy_state();

        let effective_priority = get_effective_priority(&state, &state.side_one, "quickattack");

        assert_eq!(effective_priority, 1);
    }

    #[test]
    fn test_prankster_increases_priority_of_status_move_to_1() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].ability = "prankster".to_string();

        let effective_priority = get_effective_priority(&state, &state.side_one, "thunderwave");

        assert_eq!(effective_priority, 1);
    }

    #[test]
    fn test_prankster_does_not_increase_priority_of_physical_move() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].ability = "prankster".to_string();

        let effective_priority = get_effective_priority(&state, &state.side_one, "tackle");

        assert_eq!(effective_priority, 0);
    }

    #[test]
    fn test_triage_increase_priority_of_drain_move_by_3() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].ability = "triage".to_string();

        let effective_priority = get_effective_priority(&state, &state.side_one, "drainingkiss");

        assert_eq!(effective_priority, 3);
    }

    #[test]
    fn test_prankster_increases_priority_status_move_by_one() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].ability = "prankster".to_string();

        let effective_priority = get_effective_priority(&state, &state.side_one, "babydolleyes");

        assert_eq!(effective_priority, 2);
    }

    #[test]
    fn test_galewings_does_not_incrase_priority_of_flying_move_when_user_is_damaged() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].ability = "galewings".to_string();
        state.side_one.reserve[state.side_one.active_index].hp -= 1;

        let effective_priority = get_effective_priority(&state, &state.side_one, "wingattack");

        assert_eq!(effective_priority, 0);
    }

    #[test]
    fn test_get_effective_speed_returns_actual_speed_when_there_are_no_modifiers() {
        let state: State = create_dummy_state();

        let actual_speed = state.side_one.reserve[state.side_one.active_index].speed;

        let effective_speed = get_effective_speed(&state, &state.side_one);

        assert_eq!(effective_speed, actual_speed);
    }

    #[test]
    fn test_get_effective_speed_returns_increased_speed_when_pkmn_has_speed_boost() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.reserve[state.side_one.active_index].speed;
        state.side_one.reserve[state.side_one.active_index].speed_boost = 1;

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let expected_speed = (1.5 * base_speed as f32) as i16;

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_get_effective_speed_increases_speed_when_tailwind_is_active() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.reserve[state.side_one.active_index].speed;
        state
            .side_one
            .side_conditions
            .insert(SideCondition::Tailwind, 1);

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let expected_speed = (2.0 * base_speed as f32) as i16;

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_tailwind_and_speed_boost_together_when_checking_speed() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.reserve[state.side_one.active_index].speed;
        state.side_one.reserve[state.side_one.active_index].speed_boost = 1;
        state
            .side_one
            .side_conditions
            .insert(SideCondition::Tailwind, 1);

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let mut expected_speed = (1.5 * base_speed as f32) as i16; // speed boost
        expected_speed = (2.0 * expected_speed as f32) as i16; // tailwind

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_paralysis_halves_speed() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.reserve[state.side_one.active_index].speed;
        state.side_one.reserve[state.side_one.active_index].status = Status::Paralyze;

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let expected_speed = (0.5 * base_speed as f32) as i16;

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_paralysis_and_speedboost_together() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.reserve[state.side_one.active_index].speed;
        state.side_one.reserve[state.side_one.active_index].speed_boost = 1;
        state.side_one.reserve[state.side_one.active_index].status = Status::Paralyze;

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let mut expected_speed = (1.5 * base_speed as f32) as i16; // Speed Boost
        expected_speed = (0.5 * expected_speed as f32) as i16; // Paralyzed

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_quickfeet_and_paralyzed_does_not_halve_speed() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.reserve[state.side_one.active_index].speed;
        state.side_one.reserve[state.side_one.active_index].status = Status::Paralyze;
        state.side_one.reserve[state.side_one.active_index].ability = "quickfeet".to_string();

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let expected_speed = (1.5 * base_speed as f32) as i16;

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_side_one_moves_first_when_it_is_faster() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 2;
        state.side_two.reserve[state.side_two.active_index].speed = 1;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_side_two_moves_first_when_it_is_faster() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_sideone_moves_first_using_priority_move_when_it_is_slower() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "quickattack".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_sideone_moves_first_using_grassyglide_in_grassyterrain() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;
        state.terrain.terrain_type = Terrain::GrassyTerrain;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "grassyglide".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_sideone_does_not_move_first_using_grassyglide_in_no_terrain() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;
        state.terrain.terrain_type = Terrain::None;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "grassyglide".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_switch_goes_before_priority_move() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;
        state.terrain.terrain_type = Terrain::None;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "quickattack".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Switch,
            choice: "switch_target".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_faster_pkmn_switches_first() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Switch,
            choice: "switcH_target".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Switch,
            choice: "switch_target".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_pursuit_moves_before_switch() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "pursuit".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Switch,
            choice: "switch_target".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_pursuit_goes_second_when_slower_and_opponent_does_not_switch() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "pursuit".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_slower_pkmn_goes_first_in_trickroom() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.side_two.reserve[state.side_two.active_index].speed = 2;
        state.trick_room = true;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_priority_move_goes_first_in_trickroom() {
        let mut state: State = create_dummy_state();

        state.side_one.reserve[state.side_one.active_index].speed = 2;
        state.side_one.reserve[state.side_one.active_index].speed = 1;
        state.trick_room = true;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "quickattack".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, &s1_move, &s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_run_switch_properly_swaps_pokemon() {
        let state: State = create_dummy_state();

        let mut transpose_instruction: TransposeInstruction = TransposeInstruction {
            state: state,
            percentage: 1.0,
            instructions: Vec::<Instruction>::new(),
        };

        assert_eq!(0, transpose_instruction.state.side_one.active_index);

        run_switch(&mut transpose_instruction, true, &"charizard".to_string());

        assert_eq!(1, transpose_instruction.state.side_one.active_index);
    }

    #[test]
    fn test_run_switch_properly_sets_switchinstruction() {
        let state: State = create_dummy_state();

        let mut transpose_instruction: TransposeInstruction = TransposeInstruction {
            state: state,
            percentage: 1.0,
            instructions: Vec::<Instruction>::new(),
        };

        run_switch(&mut transpose_instruction, true, &"charizard".to_string());

        let expected_instruction_enum = Instruction::SwitchInstruction(SwitchInstruction {
            is_side_one: true,
            previous_index: 0,
            next_index: 1,
        });

        assert_eq!(
            expected_instruction_enum,
            transpose_instruction.instructions[0]
        );
    }
}
