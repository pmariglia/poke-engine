use super::state::Side;
use super::state::State;
// use super::state::Status;
use crate::data::abilities::get_ability;
use crate::data::conditions::Status;
use crate::data::items::get_item;
use crate::data::moves::get_move;
use crate::data::moves::Move;

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
    let mut effective_speed = side.active.speed as f32;

    effective_speed = (effective_speed * get_boost_multiplier(side.active.speed_boost)).floor();

    match get_ability(side.active.ability.as_str()).modify_speed {
        Some(ability_func) => {
            effective_speed = (effective_speed * ability_func(state, &side.active)).floor();
        }
        None => {}
    }

    match get_item(side.active.item.as_str()).modify_speed {
        Some(item_func) => {
            effective_speed = (effective_speed * item_func(state, &side.active)).floor();
        }
        None => {}
    }

    if side.side_conditions.get("tailwind").unwrap_or(&0) > &0 {
        effective_speed = effective_speed * 2.0
    }

    if side.active.status == Status::Paralyze && side.active.ability != "quickfeet" {
        effective_speed = (effective_speed / 2.0).floor()
    }

    return effective_speed as i16;
}

pub fn get_effective_priority(state: &State, side: &Side, move_name: &str) -> i8 {
    let move_obj: &Move = get_move(move_name);
    let mut priority = move_obj.priority;

    match get_ability(side.active.ability.as_str()).modify_priority {
        Some(modify_priority_fn) => {
            priority += modify_priority_fn(move_name, &side.active);
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
    side_one_move: MoveChoice,
    side_two_move: MoveChoice,
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

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use super::super::state::Pokemon;
    use super::super::state::Side;
    use super::super::state::State;
    use super::super::state::Terrain;
    use super::super::state::Weather;
    // use super::super::state::Status;
    use super::super::state::create_basic_pokemon;
    use crate::data::conditions::Status;

    use super::get_effective_priority;
    use super::get_effective_speed;
    use super::side_one_moves_first;
    use super::MoveChoice;
    use super::MoveType;

    fn create_dummy_state() -> State {
        let pikachu: Pokemon = create_basic_pokemon("pikachu".to_string(), 100);
        let charizard: Pokemon = create_basic_pokemon("charizard".to_string(), 100);
        let blastoise: Pokemon = create_basic_pokemon("blastoise".to_string(), 100);
        let espeon: Pokemon = create_basic_pokemon("espeon".to_string(), 100);
        let snorlax: Pokemon = create_basic_pokemon("snorlax".to_string(), 100);
        let venusaur: Pokemon = create_basic_pokemon("venusaur".to_string(), 100);

        let landorustherian: Pokemon = create_basic_pokemon("landorustherian".to_string(), 100);
        let tapulele: Pokemon = create_basic_pokemon("tapulele".to_string(), 100);
        let rillaboom: Pokemon = create_basic_pokemon("rillaboom".to_string(), 100);
        let rhyperior: Pokemon = create_basic_pokemon("rhyperior".to_string(), 100);
        let gengar: Pokemon = create_basic_pokemon("gengar".to_string(), 100);
        let melmetal: Pokemon = create_basic_pokemon("melmetal".to_string(), 100);

        let my_side: Side = Side {
            active: pikachu,
            reserve: vec![charizard, blastoise, espeon, snorlax, venusaur],
            side_conditions: HashMap::<String, i8>::new(),
            wish: (0, 0),
        };

        let your_side: Side = Side {
            active: landorustherian,
            reserve: vec![tapulele, rillaboom, rhyperior, gengar, melmetal],
            side_conditions: HashMap::<String, i8>::new(),
            wish: (0, 0),
        };

        let state: State = State {
            side_one: my_side,
            side_two: your_side,
            weather: Weather::None,
            terrain: Terrain::None,
            trick_room: false,
        };

        return state;
    }

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

        state.side_one.active.ability = "prankster".to_string();

        let effective_priority = get_effective_priority(&state, &state.side_one, "thunderwave");

        assert_eq!(effective_priority, 1);
    }

    #[test]
    fn test_prankster_does_not_increase_priority_of_physical_move() {
        let mut state: State = create_dummy_state();

        state.side_one.active.ability = "prankster".to_string();

        let effective_priority = get_effective_priority(&state, &state.side_one, "tackle");

        assert_eq!(effective_priority, 0);
    }

    #[test]
    fn test_triage_increase_priority_of_drain_move_by_3() {
        let mut state: State = create_dummy_state();

        state.side_one.active.ability = "triage".to_string();

        let effective_priority = get_effective_priority(&state, &state.side_one, "drainingkiss");

        assert_eq!(effective_priority, 3);
    }

    #[test]
    fn test_prankster_increases_priority_status_move_by_one() {
        let mut state: State = create_dummy_state();

        state.side_one.active.ability = "prankster".to_string();

        let effective_priority = get_effective_priority(&state, &state.side_one, "babydolleyes");

        assert_eq!(effective_priority, 2);
    }

    #[test]
    fn test_galewings_does_not_incrase_priority_of_flying_move_when_user_is_damaged() {
        let mut state: State = create_dummy_state();

        state.side_one.active.ability = "galewings".to_string();
        state.side_one.active.hp -= 1;

        let effective_priority = get_effective_priority(&state, &state.side_one, "wingattack");

        assert_eq!(effective_priority, 0);
    }

    #[test]
    fn test_get_effective_speed_returns_actual_speed_when_there_are_no_modifiers() {
        let state: State = create_dummy_state();

        let actual_speed = state.side_one.active.speed;

        let effective_speed = get_effective_speed(&state, &state.side_one);

        assert_eq!(effective_speed, actual_speed);
    }

    #[test]
    fn test_get_effective_speed_returns_increased_speed_when_pkmn_has_speed_boost() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.active.speed;
        state.side_one.active.speed_boost = 1;

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let expected_speed = (1.5 * base_speed as f32) as i16;

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_get_effective_speed_increases_speed_when_tailwind_is_active() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.active.speed;
        state
            .side_one
            .side_conditions
            .insert("tailwind".to_string(), 1);

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let expected_speed = (2.0 * base_speed as f32) as i16;

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_tailwind_and_speed_boost_together_when_checking_speed() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.active.speed;
        state.side_one.active.speed_boost = 1;
        state
            .side_one
            .side_conditions
            .insert("tailwind".to_string(), 1);

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let mut expected_speed = (1.5 * base_speed as f32) as i16; // speed boost
        expected_speed = (2.0 * expected_speed as f32) as i16; // tailwind

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_paralysis_halves_speed() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.active.speed;
        state.side_one.active.status = Status::Paralyze;

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let expected_speed = (0.5 * base_speed as f32) as i16;

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_paralysis_and_speedboost_together() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.active.speed;
        state.side_one.active.speed_boost = 1;
        state.side_one.active.status = Status::Paralyze;

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let mut expected_speed = (1.5 * base_speed as f32) as i16; // Speed Boost
        expected_speed = (0.5 * expected_speed as f32) as i16; // Paralyzed

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_quickfeet_and_paralyzed_does_not_halve_speed() {
        let mut state: State = create_dummy_state();

        let base_speed = state.side_one.active.speed;
        state.side_one.active.status = Status::Paralyze;
        state.side_one.active.ability = "quickfeet".to_string();

        let actual_speed = get_effective_speed(&state, &state.side_one);
        let expected_speed = (1.5 * base_speed as f32) as i16;

        assert_eq!(expected_speed, actual_speed);
    }

    #[test]
    fn test_side_one_moves_first_when_it_is_faster() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 2;
        state.side_two.active.speed = 1;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_side_two_moves_first_when_it_is_faster() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_sideone_moves_first_using_priority_move_when_it_is_slower() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "quickattack".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_sideone_moves_first_using_grassyglide_in_grassyterrain() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;
        state.terrain = Terrain::GrassyTerrain;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "grassyglide".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_sideone_does_not_move_first_using_grassyglide_in_no_terrain() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;
        state.terrain = Terrain::None;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "grassyglide".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_switch_goes_before_priority_move() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;
        state.terrain = Terrain::None;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "quickattack".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Switch,
            choice: "switch_target".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_faster_pkmn_switches_first() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Switch,
            choice: "switcH_target".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Switch,
            choice: "switch_target".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_pursuit_moves_before_switch() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "pursuit".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Switch,
            choice: "switch_target".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_pursuit_goes_second_when_slower_and_opponent_does_not_switch() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "pursuit".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(false, s1_moves_first);
    }

    #[test]
    fn test_slower_pkmn_goes_first_in_trickroom() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 1;
        state.side_two.active.speed = 2;
        state.trick_room = true;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(true, s1_moves_first);
    }

    #[test]
    fn test_priority_move_goes_first_in_trickroom() {
        let mut state: State = create_dummy_state();

        state.side_one.active.speed = 2;
        state.side_two.active.speed = 1;
        state.trick_room = true;

        let s1_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "quickattack".to_string(),
        };

        let s2_move = MoveChoice {
            move_type: MoveType::Move,
            choice: "tackle".to_string(),
        };

        let s1_moves_first = side_one_moves_first(&state, s1_move, s2_move);

        assert_eq!(true, s1_moves_first);
    }
}
