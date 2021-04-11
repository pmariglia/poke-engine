use super::state::State;
use super::state::Side;
use super::state::Pokemon;
use super::state::Terrain;
use super::state::Status;
use super::moves::Move;
use super::moves::get_move;
use super::abilities::get_ability;
use super::items::get_item;


fn get_boost_multiplier(boost: i8) -> f32 {
    match boost {
        i if i < 0 => {
            return 2.0 / (i.abs() + 2) as f32;
        },
        i if i == 0 => {
            return 1.0;
        },
        i if i > 0 => {
            return ((i + 2) as f32) / 2.0;
        },
        _ => panic!("Got bad value for boost: {}", boost)
    }
}


pub fn get_effective_speed(state: &State, side: &Side) -> i16 {
    let mut effective_speed = side.active.speed as f32;

    effective_speed = (effective_speed * get_boost_multiplier(side.active.speed_boost)).floor();

    match get_ability(side.active.ability.as_str()).modify_speed {
        Some(ability_func) => {
            effective_speed = (effective_speed * ability_func(state, &side.active)).floor();
        },
        None => {}
    }

    match get_item(side.active.item.as_str()).modify_speed {
        Some(item_func) => {
            effective_speed = (effective_speed * item_func(state, &side.active)).floor();
        },
        None => {}
    }

    if side.side_conditions.get("tailwind").unwrap_or(&0) > &0 {
        effective_speed = effective_speed * 2.0
    }

    if side.active.status == Status::Paralyze {
        effective_speed = (effective_speed / 2.0).floor()
    }
    
    return effective_speed as i16;
}


pub fn get_effective_priority(state: &State, side: &Side, move_name: &str) -> i8{
    let move_obj: &Move = get_move(move_name);
    let mut priority = move_obj.priority;
    
    match get_ability(side.active.ability.as_str()).modify_priority {
        Some(modify_priority_fn) => {
            priority += modify_priority_fn(move_name);
        },
        None => {}
    }

    match get_move(move_name).modify_priority {
        Some(modify_priority_fn) => {
            priority += modify_priority_fn(&state);
        },
        None => {}
    }

    return priority;
}


pub fn side_one_moves_first(state: &State, side_one_move: &str, side_two_move: &str) -> bool {
    let side_one_effective_speed: i16 = get_effective_speed(state, &state.side_one);
    let side_two_effective_speed: i16 = get_effective_speed(state, &state.side_two);

    if side_one_move.starts_with("switch ") && side_two_move.starts_with("switch ") {
        return side_one_effective_speed > side_two_effective_speed;
    }

    else if side_one_move.starts_with("switch ") {
        if side_two_move == "pursuit" {
            return false;
        }
        return true;
    }
    
    else if side_two_move.starts_with("switch ") {
        if side_one_move == "pursuit" {
            return true;
        }
        return false;
    }
    
    let side_one_priority = get_effective_priority(&state, &state.side_one, side_one_move);
    let side_two_priority = get_effective_priority(&state, &state.side_two, side_two_move);

    if side_one_priority == side_two_priority {
        if state.trick_room {
            return side_two_effective_speed >= side_one_effective_speed;
        }
        else {
            return side_one_effective_speed > side_two_effective_speed;
        }
    } else {
        return side_one_priority > side_two_priority;
    }
}
