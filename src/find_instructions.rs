use super::state::State;
use super::state::Pokemon;
use super::abilities::get_ability;

pub fn get_effective_speed(state: &State, pkmn: &Pokemon) -> i16 {
    let mut effective_speed = pkmn.speed;

    match get_ability(pkmn.ability.as_str()).modify_speed {
        Some(ability_func) => {
            effective_speed = effective_speed * ability_func(state, pkmn);
        },
        None => {}
    }
    
    return effective_speed;
}
