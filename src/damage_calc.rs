use crate::{
    data::{
        conditions::{PokemonStatus, PokemonVolatileStatus},
        moves::{Choice, MoveCategory},
    },
    state::{Pokemon, PokemonBoostableStat, PokemonTypes, SideReference, State, Weather},
};

#[rustfmt::skip]
const TYPE_MATCHUP_DAMAGE_MULTIPICATION: [[f32; 19]; 19] = [
    [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0, 1.0, 0.5, 1.0, 1.0],
    [1.0, 0.5, 0.5, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0, 1.0],
    [1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0],
    [1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0],
    [1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 1.0, 0.5, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0, 1.0],
    [1.0, 0.5, 0.5, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0],
    [2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 2.0, 0.0, 1.0, 2.0, 2.0, 0.5, 1.0],
    [1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 0.0, 2.0, 1.0],
    [1.0, 2.0, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0],
    [1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0],
    [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0, 1.0],
    [1.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 1.0, 0.5, 2.0, 1.0, 1.0, 0.5, 1.0, 2.0, 0.5, 0.5, 1.0],
    [1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0],
    [0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.0, 1.0],
    [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0],
    [1.0, 0.5, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0],
    [1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 0.5, 1.0, 1.0],
    [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
];

pub enum DamageRolls {
    Average,
    Min,
    Max,
    MinMax,
    MinMaxAverage,
    All,
}

fn type_enum_to_type_matchup_int(type_enum: &PokemonTypes) -> usize {
    match type_enum {
        PokemonTypes::Normal => 0,
        PokemonTypes::Fire => 1,
        PokemonTypes::Water => 2,
        PokemonTypes::Electric => 3,
        PokemonTypes::Grass => 4,
        PokemonTypes::Ice => 5,
        PokemonTypes::Fighting => 6,
        PokemonTypes::Poison => 7,
        PokemonTypes::Ground => 8,
        PokemonTypes::Flying => 9,
        PokemonTypes::Psychic => 10,
        PokemonTypes::Bug => 11,
        PokemonTypes::Rock => 12,
        PokemonTypes::Ghost => 13,
        PokemonTypes::Dragon => 14,
        PokemonTypes::Dark => 15,
        PokemonTypes::Steel => 16,
        PokemonTypes::Fairy => 17,
        PokemonTypes::Typeless => 18,
    }
}

fn type_effectiveness_modifier(
    attacking_type: &PokemonTypes,
    defending_types: &(PokemonTypes, PokemonTypes),
) -> f32 {
    let mut modifier = 1.0;
    let attacking_type_index = type_enum_to_type_matchup_int(attacking_type);
    modifier = modifier
        * TYPE_MATCHUP_DAMAGE_MULTIPICATION[attacking_type_index]
            [type_enum_to_type_matchup_int(&defending_types.0)];
    modifier = modifier
        * TYPE_MATCHUP_DAMAGE_MULTIPICATION[attacking_type_index]
            [type_enum_to_type_matchup_int(&defending_types.1)];
    return modifier;
}

fn weather_modifier(attacking_move_type: &PokemonTypes, weather: &Weather) -> f32 {
    match weather {
        Weather::Sun => match attacking_move_type {
            PokemonTypes::Fire => 1.5,
            PokemonTypes::Water => 0.5,
            _ => 1.0,
        },
        Weather::Rain => match attacking_move_type {
            PokemonTypes::Water => 1.5,
            PokemonTypes::Fire => 0.5,
            _ => 1.0,
        },
        Weather::HarshSun => match attacking_move_type {
            PokemonTypes::Fire => 1.5,
            PokemonTypes::Water => 0.0,
            _ => 1.0,
        },
        Weather::HeavyRain => match attacking_move_type {
            PokemonTypes::Water => 1.5,
            PokemonTypes::Fire => 0.0,
            _ => 1.0,
        },
        _ => 1.0,
    }
}

fn stab_modifier(
    attacking_move_type: &PokemonTypes,
    active_types: &(PokemonTypes, PokemonTypes),
) -> f32 {
    if attacking_move_type == &active_types.0 || attacking_move_type == &active_types.1 {
        return 1.5;
    }
    return 1.0;
}

fn burn_modifier(
    attacking_move_category: &MoveCategory,
    attacking_pokemon_status: &PokemonStatus,
) -> f32 {
    if attacking_pokemon_status == &PokemonStatus::Burn
        && attacking_move_category == &MoveCategory::Physical
    {
        return 0.5;
    }

    return 1.0;
}

fn volatile_status_modifier(
    choice: &Choice,
    attacking_pokemon: &Pokemon,
    defending_pokemon: &Pokemon,
) -> f32 {
    let mut modifier = 1.0;
    for vs in attacking_pokemon.volatile_statuses.iter() {
        match vs {
            PokemonVolatileStatus::FlashFire if choice.move_type == PokemonTypes::Fire => {
                modifier *= 1.5;
            }
            _ => {}
        }
    }

    for vs in defending_pokemon.volatile_statuses.iter() {
        match vs {
            PokemonVolatileStatus::MagnetRise
                if choice.move_type == PokemonTypes::Ground
                    && choice.move_id.as_str() != "thousandarrows" =>
            {
                return 0.0;
            }
            PokemonVolatileStatus::TarShot if choice.move_type == PokemonTypes::Fire => {
                modifier *= 2.0;
            }
            PokemonVolatileStatus::PhantomForce
            | PokemonVolatileStatus::ShadowForce
            | PokemonVolatileStatus::Bounce
            | PokemonVolatileStatus::Dig
            | PokemonVolatileStatus::Dive
            | PokemonVolatileStatus::Fly => {
                return 0.0;
            }
            PokemonVolatileStatus::GlaiveRush => {
                modifier *= 2.0;
            }
            _ => {}
        }
    }

    return modifier;
}

fn get_damage_rolls(damage: f32, damage_roll_type: DamageRolls) -> Vec<i32> {
    match damage_roll_type {
        DamageRolls::Min => {
            return vec![(damage * 0.85) as i32];
        }
        DamageRolls::Average => {
            return vec![(damage * 0.925) as i32];
        }
        DamageRolls::Max => {
            return vec![damage as i32];
        }
        DamageRolls::MinMax => {
            return vec![(damage * 0.85) as i32, damage as i32];
        }
        DamageRolls::MinMaxAverage => {
            return vec![
                (damage * 0.85) as i32,
                (damage * 0.925) as i32,
                damage as i32,
            ];
        }
        DamageRolls::All => {
            return vec![
                (damage * 0.85) as i32,
                (damage * 0.86) as i32,
                (damage * 0.87) as i32,
                (damage * 0.88) as i32,
                (damage * 0.89) as i32,
                (damage * 0.90) as i32,
                (damage * 0.91) as i32,
                (damage * 0.92) as i32,
                (damage * 0.93) as i32,
                (damage * 0.94) as i32,
                (damage * 0.95) as i32,
                (damage * 0.96) as i32,
                (damage * 0.97) as i32,
                (damage * 0.98) as i32,
                (damage * 0.99) as i32,
                damage as i32,
            ];
        }
    }
}

// This is a basic damage calculation function that assumes special effects/modifiers
// are reflected in the `Choice` struct
//
// i.e. if an ability would multiply a move's base-power by 1.3x, that should already
// be reflected in the `Choice`
pub fn calculate_damage(
    state: &State,
    attacking_side: SideReference,
    choice: &Choice,
    damage_rolls: DamageRolls,
) -> Option<Vec<i32>> {
    if choice.base_power <= 0.0 {
        return None;
    }

    let (attacking_side, defending_side) = state.get_both_sides_immutable(&attacking_side);

    let attacking_stat;
    let defending_stat;
    match choice.category {
        MoveCategory::Physical => {
            if defending_side.get_active_immutable().ability.as_str() == "unaware" {
                attacking_stat = attacking_side.get_active_immutable().attack;
            } else {
                attacking_stat = attacking_side
                    .get_active_immutable()
                    .calculate_boosted_stat(PokemonBoostableStat::Attack);
            }
            if attacking_side.get_active_immutable().ability.as_str() == "unaware" {
                defending_stat = defending_side.get_active_immutable().defense;
            } else {
                defending_stat = defending_side
                    .get_active_immutable()
                    .calculate_boosted_stat(PokemonBoostableStat::Defense);
            }
        }
        MoveCategory::Special => {
            if defending_side.get_active_immutable().ability.as_str() == "unaware" {
                attacking_stat = attacking_side.get_active_immutable().special_attack;
            } else {
                attacking_stat = attacking_side
                    .get_active_immutable()
                    .calculate_boosted_stat(PokemonBoostableStat::SpecialAttack);
            }
            if attacking_side.get_active_immutable().ability.as_str() == "unaware" {
                defending_stat = defending_side.get_active_immutable().special_defense;
            } else {
                defending_stat = defending_side
                    .get_active_immutable()
                    .calculate_boosted_stat(PokemonBoostableStat::SpecialDefense);
            }
        }
        _ => return None,
    }

    let mut damage: f32;
    damage = 2.0 * attacking_side.get_active_immutable().level as f32;
    damage = damage.floor() / 5.0;
    damage = damage.floor() + 2.0;
    damage = damage.floor() * choice.base_power;
    damage = damage * attacking_stat as f32 / defending_stat as f32;
    damage = damage.floor() / 50.0;
    damage = damage.floor() + 2.0;

    let mut damage_modifier = 1.0;
    damage_modifier *= type_effectiveness_modifier(
        &choice.move_type,
        &defending_side.get_active_immutable().types,
    );
    damage_modifier *= weather_modifier(&choice.move_type, &state.weather.weather_type);
    damage_modifier *= stab_modifier(
        &choice.move_type,
        &attacking_side.get_active_immutable().types,
    );
    damage_modifier *= burn_modifier(
        &choice.category,
        &attacking_side.get_active_immutable().status,
    );
    damage_modifier *= volatile_status_modifier(
        &choice,
        attacking_side.get_active_immutable(),
        defending_side.get_active_immutable(),
    );

    if defending_side.side_conditions.aurora_veil > 0 {
        damage_modifier *= 0.5
    } else if defending_side.side_conditions.reflect > 0
        && choice.category == MoveCategory::Physical
    {
        damage_modifier *= 0.5
    } else if defending_side.side_conditions.light_screen > 0
        && choice.category == MoveCategory::Special
    {
        damage_modifier *= 0.5
    }

    damage = damage * damage_modifier;

    return Some(get_damage_rolls(damage.floor(), damage_rolls));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use super::*;
    use crate::data::conditions::{PokemonStatus, PokemonVolatileStatus};
    use crate::state::{
        Pokemon, PokemonNatures, PokemonTypes, Side, SideConditions, SideReference, State,
        StateTerrain, StateWeather, Terrain, Weather,
    };

    fn get_dummy_state() -> State {
        return State {
            side_one: Side {
                active_index: 0,
                pokemon: [
                    Pokemon {
                        id: "squirtle".to_string(),
                        level: 100,
                        types: (PokemonTypes::Typeless, PokemonTypes::Typeless),
                        hp: 100,
                        maxhp: 100,
                        ability: "none".to_string(),
                        item: "none".to_string(),
                        attack: 100,
                        defense: 100,
                        special_attack: 100,
                        special_defense: 100,
                        speed: 100,
                        attack_boost: 0,
                        defense_boost: 0,
                        special_attack_boost: 0,
                        special_defense_boost: 0,
                        speed_boost: 0,
                        accuracy_boost: 0,
                        evasion_boost: 0,
                        status: PokemonStatus::None,
                        nature: PokemonNatures::Serious,
                        volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
                        moves: vec![],
                    },
                    Pokemon {
                        ..Pokemon::default()
                    },
                ],
                side_conditions: SideConditions {
                    ..Default::default()
                },
                wish: (0, 0),
            },
            side_two: Side {
                active_index: 0,
                pokemon: [
                    Pokemon {
                        id: "charmander".to_string(),
                        level: 100,
                        types: (PokemonTypes::Typeless, PokemonTypes::Typeless),
                        hp: 100,
                        maxhp: 100,
                        ability: "none".to_string(),
                        item: "none".to_string(),
                        attack: 100,
                        defense: 100,
                        special_attack: 100,
                        special_defense: 100,
                        speed: 100,
                        attack_boost: 0,
                        defense_boost: 0,
                        special_attack_boost: 0,
                        special_defense_boost: 0,
                        speed_boost: 0,
                        accuracy_boost: 0,
                        evasion_boost: 0,
                        status: PokemonStatus::None,
                        nature: PokemonNatures::Serious,
                        volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
                        moves: vec![],
                    },
                    Pokemon {
                        ..Pokemon::default()
                    },
                ],
                side_conditions: SideConditions {
                    ..Default::default()
                },
                wish: (0, 0),
            },
            weather: StateWeather {
                weather_type: Weather::None,
                turns_remaining: 0,
            },
            terrain: StateTerrain {
                terrain_type: Terrain::None,
                turns_remaining: 0,
            },
            trick_room: false,
        };
    }

    #[test]
    fn test_basic_damaging_move() {
        let state = get_dummy_state();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.move_id = "tackle".to_string();
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

        // level 100 tackle with 100 base stats across the board (attacker & defender)
        // should do 35 damage max
        assert_eq!(Some(vec![35]), dmg);
    }

    #[test]
    fn test_move_with_zero_base_power() {
        let state = get_dummy_state();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.move_id = "tackle".to_string();
        choice.base_power = 0.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

        assert_eq!((None), dmg);
    }

    #[test]
    fn test_boosted_damaging_move() {
        let mut state = get_dummy_state();
        let mut choice = Choice {
            ..Default::default()
        };
        state.side_one.get_active().attack_boost = 1;
        choice.move_id = "tackle".to_string();
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

        // 35 * 1.5x attack boost is 52.5 => 52
        assert_eq!(Some(vec![52]), dmg);
    }

    #[test]
    fn test_unaware_does_not_get_damaged_by_boosted_stats() {
        let mut state = get_dummy_state();
        let mut choice = Choice {
            ..Default::default()
        };
        state.side_one.get_active().attack_boost = 1;
        state.side_two.get_active().ability = "unaware".to_string();
        choice.move_id = "tackle".to_string();
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

        assert_eq!(Some(vec![35]), dmg);
    }

    #[test]
    fn test_basic_super_effective_move() {
        let mut state = get_dummy_state();
        let mut choice = Choice {
            ..Default::default()
        };

        state.side_two.get_active().types = (PokemonTypes::Fire, PokemonTypes::Typeless);
        choice.move_id = "watergun".to_string();
        choice.move_type = PokemonTypes::Water;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Special;
        let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

        // 2x damage should be 35 * 2 = 70
        assert_eq!(Some(vec![70]), dmg);
    }

    #[test]
    fn test_basic_not_very_effective_move() {
        let mut state = get_dummy_state();
        let mut choice = Choice {
            ..Default::default()
        };

        state.side_two.get_active().types = (PokemonTypes::Water, PokemonTypes::Typeless);
        choice.move_id = "watergun".to_string();
        choice.move_type = PokemonTypes::Water;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Special;
        let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

        // 0.5x damage should be 35 / 2 = 17.5 => 17
        assert_eq!(Some(vec![17]), dmg);
    }

    macro_rules! weather_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (weather_type, move_type, expected_damage_amount) = $value;
                    let mut state = get_dummy_state();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.weather.weather_type = weather_type;

                    choice.move_type = move_type;
                    choice.base_power = 40.0;
                    choice.category = MoveCategory::Special;
                    let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

                    assert_eq!(Some(vec![expected_damage_amount]), dmg);
                }
             )*
        }
    }
    weather_tests! {
        test_rain_boosting_water: (Weather::Rain, PokemonTypes::Water, 52),
        test_rain_not_boosting_normal: (Weather::Rain, PokemonTypes::Normal, 35),
        test_sun_boosting_fire: (Weather::Sun, PokemonTypes::Fire, 52),
        test_sun_reducing_water: (Weather::Sun, PokemonTypes::Water, 17),
        test_sun_not_boosting_normal: (Weather::Sun, PokemonTypes::Normal, 35),
        test_heavy_rain_makes_fire_do_zero: (Weather::HeavyRain, PokemonTypes::Fire, 0),
        test_heavy_rain_boost_water: (Weather::HeavyRain, PokemonTypes::Water, 52),
        test_harsh_sun_makes_water_do_zero: (Weather::HarshSun, PokemonTypes::Water, 0),
        test_harsh_sun_boosting_fire: (Weather::HarshSun, PokemonTypes::Fire, 52),
    }

    macro_rules! stab_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacker_types, attacking_move_type, expected_damage_amount) = $value;
                    let mut state = get_dummy_state();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_one.get_active().types = attacker_types;

                    choice.move_type = attacking_move_type;
                    choice.base_power = 40.0;
                    choice.category = MoveCategory::Special;
                    let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

                    assert_eq!(Some(vec![expected_damage_amount]), dmg);
                }
             )*
        }
    }
    stab_tests! {
        test_basic_stab: ((PokemonTypes::Water, PokemonTypes::Fire), PokemonTypes::Water, 52),
        test_basic_without_stab: ((PokemonTypes::Water, PokemonTypes::Fire), PokemonTypes::Normal, 35),
    }

    macro_rules! burn_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacking_move_category, expected_damage_amount) = $value;
                    let mut state = get_dummy_state();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_one.get_active().status = PokemonStatus::Burn;

                    choice.category = attacking_move_category;
                    choice.base_power = 40.0;
                    let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

                    assert_eq!(Some(vec![expected_damage_amount]), dmg);
                }
             )*
        }
    }
    burn_tests! {
        test_physical_move_when_burned_reduces: (MoveCategory::Physical, 17),
        test_special_move_when_burned_does_not_reduce: (MoveCategory::Special, 35),
    }

    macro_rules! screens_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (reflect_count, lightscreen_count, auroraveil_count, move_category, expected_damage_amount) = $value;
                    let mut state = get_dummy_state();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_two.side_conditions.reflect = reflect_count;
                    state.side_two.side_conditions.light_screen = lightscreen_count;
                    state.side_two.side_conditions.aurora_veil = auroraveil_count;

                    choice.category = move_category;
                    choice.base_power = 40.0;
                    let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

                    assert_eq!(Some(vec![expected_damage_amount]), dmg);
                }
             )*
        }
    }
    screens_tests! {
        test_reflect_reduces_physical_damage_by_half: (1, 0, 0, MoveCategory::Physical, 17),
        test_lightscreen_reduces_special_damage_by_half: (0, 1, 0, MoveCategory::Special, 17),
        test_auroraveil_reduces_physical_damage_by_half: (0, 0, 1, MoveCategory::Physical, 17),
        test_auroraveil_reduces_special_damage_by_half: (0, 0, 1, MoveCategory::Special, 17),
        test_reflect_does_not_reduce_special_damage: (1, 0, 0, MoveCategory::Special, 35),
        test_light_screen_does_not_reduce_physical_damage: (0, 1, 0, MoveCategory::Physical, 35),
        test_auroraveil_does_not_stack_with_reflect: (1, 1, 1, MoveCategory::Physical, 17),
        test_auroraveil_does_not_stack_with_lightscreen: (1, 1, 1, MoveCategory::Special, 17),
    }

    macro_rules! volatile_status_tests{
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacking_volatile_status, defending_volatile_status, move_type, move_name, expected_damage_amount) = $value;
                    let mut state = get_dummy_state();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_one.get_active().volatile_statuses = HashSet::from_iter(attacking_volatile_status);
                    state.side_two.get_active().volatile_statuses = HashSet::from_iter(defending_volatile_status);

                    choice.move_id = move_name.to_string();
                    choice.category = MoveCategory::Physical;
                    choice.move_type = move_type;
                    choice.base_power = 40.0;
                    let dmg = calculate_damage(&state, SideReference::SideOne, &choice, DamageRolls::Max);

                    assert_eq!(Some(vec![expected_damage_amount]), dmg);
                }
             )*
        }
    }
    volatile_status_tests! {
        test_flashfire_boosts_fire_move: (
            vec![PokemonVolatileStatus::FlashFire],
            vec![],
            PokemonTypes::Fire,
            "",
            52
        ),
        test_flashfire_does_not_boost_normal_move: (
            vec![PokemonVolatileStatus::FlashFire],
            vec![],
            PokemonTypes::Normal,
            "",
            35
        ),
        test_magnetrise_makes_pkmn_immune_to_ground_move: (
            vec![],
            vec![PokemonVolatileStatus::MagnetRise],
            PokemonTypes::Ground,
            "",
            0
        ),
        test_thousandarrows_can_hit_magnetrise_pokemon: (
            vec![],
            vec![PokemonVolatileStatus::MagnetRise],
            PokemonTypes::Ground,
            "thousandarrows",
            35
        ),
        test_tarshot_boosts_fire_move: (
            vec![],
            vec![PokemonVolatileStatus::TarShot],
            PokemonTypes::Fire,
            "",
            70
        ),
        test_tarshot_and_flashfire_together: (
            vec![PokemonVolatileStatus::FlashFire],
            vec![PokemonVolatileStatus::TarShot],
            PokemonTypes::Fire,
            "",
            105
        ),
        test_glaiverush_doubles_damage_against: (
            vec![],
            vec![PokemonVolatileStatus::GlaiveRush],
            PokemonTypes::Normal,
            "",
            70
        ),
        test_phantomforce_on_defender_causes_0_damage: (
            vec![],
            vec![PokemonVolatileStatus::PhantomForce],
            PokemonTypes::Normal,
            "",
            0
        ),
    }
}
