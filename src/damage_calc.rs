use crate::abilities::Abilities;
use crate::choices::Choices;
use crate::state::{Side, Terrain};
use crate::{
    choices::{Choice, MoveCategory},
    state::{
        Pokemon, PokemonBoostableStat, PokemonStatus, PokemonType, PokemonVolatileStatus,
        SideReference, State, Weather,
    },
};

#[rustfmt::skip]
#[cfg(any(feature = "gen9",feature = "gen8",feature = "gen7",feature = "gen6"))]
const TYPE_MATCHUP_DAMAGE_MULTIPICATION: [[f32; 19]; 19] = [
/*         0    1    2    3    4    5    6    7    8    9   10   11   12   13   14   15   16   17   18  */
/*  0 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0, 1.0, 0.5, 1.0, 1.0],
/*  1 */ [1.0, 0.5, 0.5, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0, 1.0],
/*  2 */ [1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0],
/*  3 */ [1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0],
/*  4 */ [1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 1.0, 0.5, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0, 1.0],
/*  5 */ [1.0, 0.5, 0.5, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0],
/*  6 */ [2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 2.0, 0.0, 1.0, 2.0, 2.0, 0.5, 1.0],
/*  7 */ [1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 0.0, 2.0, 1.0],
/*  8 */ [1.0, 2.0, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0],
/*  9 */ [1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0],
/* 10 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0, 1.0],
/* 11 */ [1.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 1.0, 0.5, 2.0, 1.0, 1.0, 0.5, 1.0, 2.0, 0.5, 0.5, 1.0],
/* 12 */ [1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0],
/* 13 */ [0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0],
/* 14 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.0, 1.0],
/* 15 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0],
/* 16 */ [1.0, 0.5, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0],
/* 17 */ [1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 0.5, 1.0, 1.0],
/* 18 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
];

#[rustfmt::skip]
#[cfg(any(feature = "gen5",feature = "gen4"))]
const TYPE_MATCHUP_DAMAGE_MULTIPICATION: [[f32; 19]; 19] = [
/*         0    1    2    3    4    5    6    7    8    9   10   11   12   13   14   15   16   17   18  */
/*  0 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0, 1.0, 0.5, 1.0, 1.0],
/*  1 */ [1.0, 0.5, 0.5, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0, 1.0],
/*  2 */ [1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0],
/*  3 */ [1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0],
/*  4 */ [1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 1.0, 0.5, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0, 1.0],
/*  5 */ [1.0, 0.5, 0.5, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0],
/*  6 */ [2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 2.0, 0.0, 1.0, 2.0, 2.0, 0.5, 1.0],
/*  7 */ [1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 0.0, 2.0, 1.0],
/*  8 */ [1.0, 2.0, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0],
/*  9 */ [1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0],
/* 10 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0, 1.0],
/* 11 */ [1.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 1.0, 0.5, 2.0, 1.0, 1.0, 0.5, 1.0, 2.0, 0.5, 0.5, 1.0],
/* 12 */ [1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0],
/* 13 */ [0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 1.0, 1.0],
/* 14 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.0, 1.0],
/* 15 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 0.5, 1.0],
/* 16 */ [1.0, 0.5, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0],
/* 17 */ [1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 0.5, 1.0, 1.0],
/* 18 */ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
];

pub enum DamageRolls {
    Average,
    Min,
    Max,
    MinMax,
    MinMaxAverage,
    All,
}

fn type_enum_to_type_matchup_int(type_enum: &PokemonType) -> usize {
    match type_enum {
        PokemonType::Normal => 0,
        PokemonType::Fire => 1,
        PokemonType::Water => 2,
        PokemonType::Electric => 3,
        PokemonType::Grass => 4,
        PokemonType::Ice => 5,
        PokemonType::Fighting => 6,
        PokemonType::Poison => 7,
        PokemonType::Ground => 8,
        PokemonType::Flying => 9,
        PokemonType::Psychic => 10,
        PokemonType::Bug => 11,
        PokemonType::Rock => 12,
        PokemonType::Ghost => 13,
        PokemonType::Dragon => 14,
        PokemonType::Dark => 15,
        PokemonType::Steel => 16,
        PokemonType::Fairy => 17,
        PokemonType::Typeless => 18,
    }
}

pub fn type_effectiveness_modifier(
    attacking_type: &PokemonType,
    defending_types: &(PokemonType, PokemonType),
) -> f32 {
    let mut modifier = 1.0;
    let attacking_type_index = type_enum_to_type_matchup_int(attacking_type);
    modifier = modifier
        * TYPE_MATCHUP_DAMAGE_MULTIPICATION[attacking_type_index]
            [type_enum_to_type_matchup_int(&defending_types.0)];
    modifier = modifier
        * TYPE_MATCHUP_DAMAGE_MULTIPICATION[attacking_type_index]
            [type_enum_to_type_matchup_int(&defending_types.1)];
    modifier
}

fn weather_modifier(attacking_move_type: &PokemonType, weather: &Weather) -> f32 {
    match weather {
        Weather::Sun => match attacking_move_type {
            PokemonType::Fire => 1.5,
            PokemonType::Water => 0.5,
            _ => 1.0,
        },
        Weather::Rain => match attacking_move_type {
            PokemonType::Water => 1.5,
            PokemonType::Fire => 0.5,
            _ => 1.0,
        },
        Weather::HarshSun => match attacking_move_type {
            PokemonType::Fire => 1.5,
            PokemonType::Water => 0.0,
            _ => 1.0,
        },
        Weather::HeavyRain => match attacking_move_type {
            PokemonType::Water => 1.5,
            PokemonType::Fire => 0.0,
            _ => 1.0,
        },
        _ => 1.0,
    }
}

fn stab_modifier(
    attacking_move_type: &PokemonType,
    active_types: &(PokemonType, PokemonType),
) -> f32 {
    if attacking_move_type != &PokemonType::Typeless
        && (attacking_move_type == &active_types.0 || attacking_move_type == &active_types.1)
    {
        return 1.5;
    }
    1.0
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

    1.0
}

fn terrain_modifier(
    terrain: &Terrain,
    attacker: &Pokemon,
    defender: &Pokemon,
    choice: &Choice,
) -> f32 {
    #[cfg(any(feature = "gen9", feature = "gen8"))]
    let terrain_boost = 1.3;

    #[cfg(any(feature = "gen7", feature = "gen6", feature = "gen5", feature = "gen4"))]
    let terrain_boost = 1.5;

    match terrain {
        Terrain::ElectricTerrain => {
            if choice.move_type == PokemonType::Electric && attacker.is_grounded() {
                terrain_boost
            } else {
                1.0
            }
        }
        Terrain::GrassyTerrain => {
            if choice.move_type == PokemonType::Grass && attacker.is_grounded() {
                terrain_boost
            } else if choice.move_id == Choices::EARTHQUAKE {
                0.5
            } else {
                1.0
            }
        }
        Terrain::MistyTerrain => {
            if choice.move_type == PokemonType::Dragon && defender.is_grounded() {
                0.5
            } else {
                1.0
            }
        }
        Terrain::PsychicTerrain => {
            if choice.move_type == PokemonType::Psychic && attacker.is_grounded() {
                terrain_boost
            } else if choice.priority > 0 && defender.is_grounded() {
                0.5
            } else {
                1.0
            }
        }
        Terrain::None => 1.0,
    }
}

fn volatile_status_modifier(choice: &Choice, attacking_side: &Side, defending_side: &Side) -> f32 {
    let mut modifier = 1.0;
    for vs in attacking_side.volatile_statuses.iter() {
        match vs {
            PokemonVolatileStatus::FlashFire if choice.move_type == PokemonType::Fire => {
                modifier *= 1.5;
            }
            PokemonVolatileStatus::SlowStart if choice.category == MoveCategory::Physical => {
                modifier *= 0.5;
            }
            _ => {}
        }
    }

    for vs in defending_side.volatile_statuses.iter() {
        match vs {
            PokemonVolatileStatus::MagnetRise
                if choice.move_type == PokemonType::Ground
                    && choice.move_id != Choices::THOUSANDARROWS =>
            {
                return 0.0;
            }
            PokemonVolatileStatus::TarShot if choice.move_type == PokemonType::Fire => {
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

    modifier
}

fn _get_damage_rolls(damage: f32, damage_roll_type: DamageRolls) -> Vec<i16> {
    match damage_roll_type {
        DamageRolls::Min => {
            vec![(damage * 0.85) as i16]
        }
        DamageRolls::Average => {
            vec![(damage * 0.925) as i16]
        }
        DamageRolls::Max => {
            vec![damage as i16]
        }
        DamageRolls::MinMax => {
            vec![(damage * 0.85) as i16, damage as i16]
        }
        DamageRolls::MinMaxAverage => {
            vec![
                (damage * 0.85) as i16,
                (damage * 0.925) as i16,
                damage as i16,
            ]
        }
        DamageRolls::All => {
            vec![
                (damage * 0.85) as i16,
                (damage * 0.86) as i16,
                (damage * 0.87) as i16,
                (damage * 0.88) as i16,
                (damage * 0.89) as i16,
                (damage * 0.90) as i16,
                (damage * 0.91) as i16,
                (damage * 0.92) as i16,
                (damage * 0.93) as i16,
                (damage * 0.94) as i16,
                (damage * 0.95) as i16,
                (damage * 0.96) as i16,
                (damage * 0.97) as i16,
                (damage * 0.98) as i16,
                (damage * 0.99) as i16,
                damage as i16,
            ]
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
    attacking_side: &SideReference,
    choice: &Choice,
    _damage_rolls: DamageRolls,
) -> Option<i16> {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side);
    let attacker = attacking_side.get_active_immutable();
    let defender = defending_side.get_active_immutable();

    let attacking_stat;
    let mut defending_stat;
    match choice.category {
        MoveCategory::Physical => {
            if defender.ability == Abilities::UNAWARE {
                attacking_stat = attacker.attack;
            } else {
                attacking_stat =
                    attacking_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
            }
            if attacker.ability == Abilities::UNAWARE {
                defending_stat = defender.defense;
            } else {
                defending_stat =
                    defending_side.calculate_boosted_stat(PokemonBoostableStat::Defense);
            }
        }
        MoveCategory::Special => {
            if defender.ability == Abilities::UNAWARE {
                attacking_stat = attacker.special_attack;
            } else {
                attacking_stat =
                    attacking_side.calculate_boosted_stat(PokemonBoostableStat::SpecialAttack);
            }
            if attacker.ability == Abilities::UNAWARE {
                defending_stat = defender.special_defense;
            } else if choice.move_id == Choices::PSYSHOCK
                || choice.move_id == Choices::SECRETSWORD
                || choice.move_id == Choices::PSYSTRIKE
            {
                defending_stat =
                    defending_side.calculate_boosted_stat(PokemonBoostableStat::Defense);
            } else {
                defending_stat =
                    defending_side.calculate_boosted_stat(PokemonBoostableStat::SpecialDefense);
                if state.weather_is_active(&Weather::Sand) && defender.has_type(&PokemonType::Rock)
                {
                    defending_stat = (defending_stat as f32 * 1.5) as i16;
                }
            }
        }
        _ => return None,
    }

    if choice.base_power <= 0.0 {
        return Some(0);
    }

    let mut damage: f32;
    damage = 2.0 * attacker.level as f32;
    damage = damage.floor() / 5.0;
    damage = damage.floor() + 2.0;
    damage = damage.floor() * choice.base_power;
    damage = damage * attacking_stat as f32 / defending_stat as f32;
    damage = damage.floor() / 50.0;
    damage = damage.floor() + 2.0;

    let mut defender_types = defender.types;
    if defending_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::Roost)
    {
        if defender_types.0 == PokemonType::Flying {
            defender_types = (PokemonType::Typeless, defender_types.1);
        }
        if defender_types.1 == PokemonType::Flying {
            defender_types = (defender_types.0, PokemonType::Typeless);
        }
    }
    if attacker.ability == Abilities::SCRAPPY
        && (choice.move_type == PokemonType::Normal || choice.move_type == PokemonType::Fighting)
    {
        if defender_types.0 == PokemonType::Ghost {
            defender_types = (PokemonType::Typeless, defender_types.1);
        }
        if defender_types.1 == PokemonType::Ghost {
            defender_types = (defender_types.0, PokemonType::Typeless);
        }
    }

    let mut damage_modifier = 1.0;
    damage_modifier *= type_effectiveness_modifier(&choice.move_type, &defender_types);

    if attacker.ability != Abilities::CLOUDNINE
        && attacker.ability != Abilities::AIRLOCK
        && defender.ability != Abilities::CLOUDNINE
        && defender.ability != Abilities::AIRLOCK
    {
        damage_modifier *= weather_modifier(&choice.move_type, &state.weather.weather_type);
    }

    damage_modifier *= stab_modifier(&choice.move_type, &attacker.types);
    damage_modifier *= burn_modifier(&choice.category, &attacker.status);
    damage_modifier *= volatile_status_modifier(&choice, attacking_side, defending_side);
    damage_modifier *= terrain_modifier(&state.terrain.terrain_type, attacker, defender, &choice);

    if attacker.ability != Abilities::INFILTRATOR {
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
    }

    damage = damage * damage_modifier;

    match _damage_rolls {
        DamageRolls::Average => damage = damage.floor() * 0.925,
        DamageRolls::Min => damage = damage.floor() * 0.85,
        DamageRolls::Max => damage = damage.floor(),
        _ => panic!("Not implemented"),
    }

    Some(damage as i16)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use super::*;
    use crate::state::{
        PokemonStatus, PokemonType, PokemonVolatileStatus, SideReference, State, Weather,
    };

    #[test]
    fn test_basic_damaging_move() {
        let state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::Typeless;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        // level 100 tackle with 100 base stats across the board (attacker & defender)
        assert_eq!(32, dmg.unwrap());
    }

    #[test]
    fn test_basic_non_damaging_move() {
        let state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.move_id = Choices::PROTECT;
        choice.category = MoveCategory::Status;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(None, dmg);
    }

    #[test]
    fn test_move_with_zero_base_power() {
        let state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::Typeless;
        choice.base_power = 0.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(0, dmg.unwrap());
    }

    #[test]
    fn test_boosted_damaging_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        state.side_one.attack_boost = 1;
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::Typeless;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(48, dmg.unwrap());
    }

    #[test]
    fn test_unaware_does_not_get_damaged_by_boosted_stats() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        state.side_one.attack_boost = 1;
        state.side_two.get_active().ability = Abilities::UNAWARE;
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::Typeless;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(32, dmg.unwrap());
    }

    #[test]
    fn test_basic_super_effective_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        state.side_two.get_active().types = (PokemonType::Fire, PokemonType::Typeless);
        choice.move_id = Choices::WATERGUN;
        choice.move_type = PokemonType::Water;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Special;
        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(64, dmg.unwrap());
    }

    #[test]
    fn test_basic_not_very_effective_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        state.side_two.get_active().types = (PokemonType::Water, PokemonType::Typeless);
        choice.move_id = Choices::WATERGUN;
        choice.move_type = PokemonType::Water;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Special;
        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(15, dmg.unwrap());
    }

    macro_rules! weather_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (weather_type, move_type, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.weather.weather_type = weather_type;

                    choice.move_type = move_type;
                    choice.base_power = 40.0;
                    choice.category = MoveCategory::Special;
                    let dmg = calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap());
                }
             )*
        }
    }
    weather_tests! {
        test_rain_boosting_water: (Weather::Rain, PokemonType::Water, 48),
        test_rain_not_boosting_normal: (Weather::Rain, PokemonType::Normal, 48),
        test_sun_boosting_fire: (Weather::Sun, PokemonType::Fire, 48),
        test_sun_reducing_water: (Weather::Sun, PokemonType::Water, 15),
        test_sun_not_boosting_normal: (Weather::Sun, PokemonType::Normal, 48),
        test_heavy_rain_makes_fire_do_zero: (Weather::HeavyRain, PokemonType::Fire, 0),
        test_heavy_rain_boost_water: (Weather::HeavyRain, PokemonType::Water, 48),
        test_harsh_sun_makes_water_do_zero: (Weather::HarshSun, PokemonType::Water, 0),
        test_harsh_sun_boosting_fire: (Weather::HarshSun, PokemonType::Fire, 48),
    }

    macro_rules! stab_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacker_types, attacking_move_type, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_one.get_active().types = attacker_types;

                    choice.move_type = attacking_move_type;
                    choice.base_power = 40.0;
                    choice.category = MoveCategory::Special;
                    let dmg = calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap());
                }
             )*
        }
    }
    stab_tests! {
        test_basic_stab: ((PokemonType::Water, PokemonType::Fire), PokemonType::Water, 48),
        test_basic_without_stab: ((PokemonType::Water, PokemonType::Fire), PokemonType::Normal, 32),
    }

    macro_rules! burn_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacking_move_category, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_one.get_active().status = PokemonStatus::Burn;

                    choice.category = attacking_move_category;
                    choice.move_type = PokemonType::Typeless;
                    choice.base_power = 40.0;
                    let dmg = calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap());
                }
             )*
        }
    }
    burn_tests! {
        test_physical_move_when_burned_reduces: (MoveCategory::Physical, 15),
        test_special_move_when_burned_does_not_reduce: (MoveCategory::Special, 32),
    }

    macro_rules! screens_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (reflect_count, lightscreen_count, auroraveil_count, move_category, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_two.side_conditions.reflect = reflect_count;
                    state.side_two.side_conditions.light_screen = lightscreen_count;
                    state.side_two.side_conditions.aurora_veil = auroraveil_count;

                    choice.category = move_category;
                    choice.base_power = 40.0;
                    choice.move_type = PokemonType::Typeless;
                    let dmg = calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap());
                }
             )*
        }
    }
    screens_tests! {
        test_reflect_reduces_physical_damage_by_half: (1, 0, 0, MoveCategory::Physical, 15),
        test_lightscreen_reduces_special_damage_by_half: (0, 1, 0, MoveCategory::Special, 15),
        test_auroraveil_reduces_physical_damage_by_half: (0, 0, 1, MoveCategory::Physical, 15),
        test_auroraveil_reduces_special_damage_by_half: (0, 0, 1, MoveCategory::Special, 15),
        test_reflect_does_not_reduce_special_damage: (1, 0, 0, MoveCategory::Special, 32),
        test_light_screen_does_not_reduce_physical_damage: (0, 1, 0, MoveCategory::Physical, 32),
        test_auroraveil_does_not_stack_with_reflect: (1, 1, 1, MoveCategory::Physical, 15),
        test_auroraveil_does_not_stack_with_lightscreen: (1, 1, 1, MoveCategory::Special, 15),
    }

    macro_rules! volatile_status_tests{
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacking_volatile_status, defending_volatile_status, move_type, move_name, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_one.volatile_statuses = HashSet::from_iter(attacking_volatile_status);
                    state.side_two.volatile_statuses = HashSet::from_iter(defending_volatile_status);

                    choice.move_id = move_name;
                    choice.category = MoveCategory::Physical;
                    choice.move_type = move_type;
                    choice.base_power = 40.0;
                    let dmg = calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap());
                }
             )*
        }
    }
    volatile_status_tests! {
        test_flashfire_boosts_fire_move: (
            vec![PokemonVolatileStatus::FlashFire],
            vec![],
            PokemonType::Fire,
            Choices::NONE,
            48
        ),
        test_flashfire_does_not_boost_normal_move: (
            vec![PokemonVolatileStatus::FlashFire],
            vec![],
            PokemonType::Typeless,
            Choices::NONE,
            32
        ),
        test_magnetrise_makes_pkmn_immune_to_ground_move: (
            vec![],
            vec![PokemonVolatileStatus::MagnetRise],
            PokemonType::Ground,
            Choices::NONE,
            0
        ),
        test_thousandarrows_can_hit_magnetrise_pokemon: (
            vec![],
            vec![PokemonVolatileStatus::MagnetRise],
            PokemonType::Ground,
            Choices::THOUSANDARROWS,
            32
        ),
        test_tarshot_boosts_fire_move: (
            vec![],
            vec![PokemonVolatileStatus::TarShot],
            PokemonType::Fire,
            Choices::NONE,
            64
        ),
        test_slowstart_halves_move: (
            vec![PokemonVolatileStatus::SlowStart],
            vec![],
            PokemonType::Normal,
            Choices::NONE,
            24
        ),
        test_tarshot_and_flashfire_together: (
            vec![PokemonVolatileStatus::FlashFire],
            vec![PokemonVolatileStatus::TarShot],
            PokemonType::Fire,
            Choices::NONE,
            97
        ),
        test_glaiverush_doubles_damage_against: (
            vec![],
            vec![PokemonVolatileStatus::GlaiveRush],
            PokemonType::Normal,
            Choices::NONE,
            97
        ),
        test_phantomforce_on_defender_causes_0_damage: (
            vec![],
            vec![PokemonVolatileStatus::PhantomForce],
            PokemonType::Normal,
            Choices::NONE,
            0
        ),
    }
}
