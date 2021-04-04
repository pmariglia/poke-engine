use std::collections::HashMap;

use lazy_static::lazy_static;

use super::state::State;
use super::state::Side;
use super::state::Pokemon;
use super::state::Weather;

type ModifySpeedFn = fn(&State, &Pokemon) -> i16;

fn modify_speed_swiftswim(state: &State, pkmn: &Pokemon) -> i16 {
    if state.weather == Weather::Rain || state.weather == Weather::HeavyRain {
        return 2
    };

    return 1;
}

lazy_static! {
    static ref ABILITIES: HashMap<String, Ability> = {
        let mut abilities: HashMap<String, Ability> = HashMap::new();

        abilities.insert(
            "swiftswim".to_string(),
            Ability {
                modify_speed: Some(modify_speed_swiftswim)
            }
        );

        abilities
    };
}

pub fn get_ability(ability_name: &str) -> &'static Ability {
    return ABILITIES.get(ability_name).unwrap_or_else(
        || panic!("Could not get ability {}", ability_name)
    )
}

// #[derive(Debug)]
pub struct Ability {
    pub modify_speed: Option<ModifySpeedFn>
}