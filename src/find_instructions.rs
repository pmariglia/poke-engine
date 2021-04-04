use super::state::State;
use super::state::Pokemon;
use super::abilities::get_ability;


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


pub fn get_effective_speed(state: &State, pkmn: &Pokemon) -> i16 {
    let mut effective_speed = pkmn.speed;

    effective_speed = (effective_speed as f32 * get_boost_multiplier(pkmn.speed_boost)) as i16;

    match get_ability(pkmn.ability.as_str()).modify_speed {
        Some(ability_func) => {
            effective_speed = effective_speed * ability_func(state, pkmn);
        },
        None => {}
    }
    
    return effective_speed;
}
