use crate::choices::{Choices, MOVES};
use crate::state::{PokemonIndex, Side};
use crate::{
    choices::{Choice, MoveCategory},
    state::{
        Pokemon, PokemonBoostableStat, PokemonStatus, PokemonType, PokemonVolatileStatus,
        SideReference, State, Weather,
    },
};

#[rustfmt::skip]
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

#[allow(dead_code)]
pub enum DamageRolls {
    Average,
    Min,
    Max,
}

fn type_enum_to_type_matchup_int(type_enum: &PokemonType) -> usize {
    match type_enum {
        PokemonType::NORMAL => 0,
        PokemonType::FIRE => 1,
        PokemonType::WATER => 2,
        PokemonType::ELECTRIC => 3,
        PokemonType::GRASS => 4,
        PokemonType::ICE => 5,
        PokemonType::FIGHTING => 6,
        PokemonType::POISON => 7,
        PokemonType::GROUND => 8,
        PokemonType::FLYING => 9,
        PokemonType::PSYCHIC => 10,
        PokemonType::BUG => 11,
        PokemonType::ROCK => 12,
        PokemonType::GHOST => 13,
        PokemonType::DRAGON => 14,
        PokemonType::DARK => 15,
        PokemonType::STEEL => 16,
        PokemonType::FAIRY => 17,
        PokemonType::TYPELESS => 18,
        PokemonType::STELLAR => panic!("Stellar is not a valid type for type calculations"),
    }
}

pub fn type_effectiveness_modifier(attacking_type: &PokemonType, defender: &Pokemon) -> f32 {
    _type_effectiveness_modifier(attacking_type, &defender.types)
}

fn _type_effectiveness_modifier(
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
        Weather::SUN => match attacking_move_type {
            PokemonType::FIRE => 1.5,
            PokemonType::WATER => 0.5,
            _ => 1.0,
        },
        Weather::RAIN => match attacking_move_type {
            PokemonType::WATER => 1.5,
            PokemonType::FIRE => 0.5,
            _ => 1.0,
        },
        Weather::HARSHSUN => match attacking_move_type {
            PokemonType::FIRE => 1.5,
            PokemonType::WATER => 0.0,
            _ => 1.0,
        },
        Weather::HEAVYRAIN => match attacking_move_type {
            PokemonType::WATER => 1.5,
            PokemonType::FIRE => 0.0,
            _ => 1.0,
        },
        _ => 1.0,
    }
}

fn stab_modifier(attacking_move_type: &PokemonType, active_pkmn: &Pokemon) -> f32 {
    if attacking_move_type == &PokemonType::TYPELESS {
        return 1.0;
    }
    let active_types = active_pkmn.types;
    if attacking_move_type == &active_types.0 || attacking_move_type == &active_types.1 {
        1.5
    } else {
        1.0
    }
}

fn burn_modifier(
    attacking_move_category: &MoveCategory,
    attacking_pokemon_status: &PokemonStatus,
) -> f32 {
    if attacking_pokemon_status == &PokemonStatus::BURN
        && attacking_move_category == &MoveCategory::Physical
    {
        return 0.5;
    }

    1.0
}

fn volatile_status_modifier(defending_side: &Side) -> f32 {
    if defending_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::DIG)
        || defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::FLY)
    {
        0.0
    } else {
        1.0
    }
}

fn common_pkmn_damage_calc(
    attacker: &Pokemon,
    attacking_stat: i16,
    defending_side: &Side,
    defender: &Pokemon,
    defending_stat: i16,
    weather: &Weather,
    choice: &Choice,
) -> f32 {
    let mut damage: f32;
    damage = 2.0 * attacker.level as f32;
    damage = damage.floor() / 5.0;
    damage = damage.floor() + 2.0;
    damage = damage.floor() * choice.base_power;
    damage = damage * attacking_stat as f32 / defending_stat as f32;
    damage = damage.floor() / 50.0;
    damage = damage.floor() + 2.0;

    let defender_types = defender.types;

    let mut damage_modifier = 1.0;
    damage_modifier *= _type_effectiveness_modifier(&choice.move_type, &defender_types);
    damage_modifier *= weather_modifier(&choice.move_type, weather);
    damage_modifier *= stab_modifier(&choice.move_type, &attacker);
    damage_modifier *= volatile_status_modifier(defending_side);

    damage * damage_modifier
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
) -> Option<(i16, i16)> {
    if choice.category == MoveCategory::Status || choice.category == MoveCategory::Switch {
        return None;
    } else if choice.base_power == 0.0 {
        return Some((0, 0));
    }
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side);
    let attacker = attacking_side.get_active_immutable();
    let defender = defending_side.get_active_immutable();

    let (crit_attacking_stat, crit_defending_stat, boosted_attacking_stat, boosted_defending_stat);
    let mut gen2_crit_ignore_effects = false;
    if choice.category == MoveCategory::Physical {
        boosted_attacking_stat =
            attacking_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
        boosted_defending_stat =
            defending_side.calculate_boosted_stat(PokemonBoostableStat::Defense);
        if defending_side.defense_boost >= attacking_side.attack_boost {
            gen2_crit_ignore_effects = true;
            crit_attacking_stat = boosted_attacking_stat;
            crit_defending_stat = boosted_defending_stat;
        } else {
            crit_attacking_stat = attacker.attack;
            crit_defending_stat = defender.defense;
        }
    } else {
        boosted_attacking_stat =
            attacking_side.calculate_boosted_stat(PokemonBoostableStat::SpecialAttack);
        boosted_defending_stat =
            defending_side.calculate_boosted_stat(PokemonBoostableStat::SpecialDefense);
        if defending_side.special_defense_boost >= attacking_side.special_attack_boost {
            gen2_crit_ignore_effects = true;
            crit_attacking_stat = boosted_attacking_stat;
            crit_defending_stat = boosted_defending_stat;
        } else {
            crit_attacking_stat = attacker.special_attack;
            crit_defending_stat = defender.special_defense;
        }
    }

    let mut damage = common_pkmn_damage_calc(
        attacker,
        boosted_attacking_stat,
        defending_side,
        defender,
        boosted_defending_stat,
        &state.weather.weather_type,
        choice,
    );
    // burn modifier is not common when considering crit
    damage *= burn_modifier(&choice.category, &attacker.status);
    if defending_side.side_conditions.reflect > 0 && choice.category == MoveCategory::Physical {
        damage *= 0.5
    } else if defending_side.side_conditions.light_screen > 0
        && choice.category == MoveCategory::Special
    {
        damage *= 0.5
    }

    let mut crit_damage = common_pkmn_damage_calc(
        attacker,
        crit_attacking_stat,
        defending_side,
        defender,
        crit_defending_stat,
        &state.weather.weather_type,
        choice,
    );
    if !gen2_crit_ignore_effects {
        crit_damage *= burn_modifier(&choice.category, &attacker.status);
        if defending_side.side_conditions.reflect > 0 && choice.category == MoveCategory::Physical {
            crit_damage *= 0.5
        } else if defending_side.side_conditions.light_screen > 0
            && choice.category == MoveCategory::Special
        {
            crit_damage *= 0.5
        }
    }
    crit_damage *= 2.0;

    match _damage_rolls {
        DamageRolls::Average => {
            damage = damage.floor() * 0.925;
            crit_damage = crit_damage.floor() * 0.925
        }
        DamageRolls::Min => {
            damage = damage.floor() * 0.85;
            crit_damage = crit_damage.floor() * 0.85
        }
        DamageRolls::Max => {
            damage = damage.floor();
            crit_damage = crit_damage.floor()
        }
    }

    Some((damage as i16, crit_damage as i16))
}

pub fn calculate_futuresight_damage(
    attacking_side: &Side,
    defending_side: &Side,
    attacking_side_pokemon_index: &PokemonIndex,
) -> i16 {
    let attacking_stat = attacking_side.pokemon[attacking_side_pokemon_index].special_attack;
    let defending_stat = defending_side.get_active_immutable().special_defense;
    let attacker = attacking_side.get_active_immutable();
    let damage = common_pkmn_damage_calc(
        attacker,
        attacking_stat,
        defending_side,
        defending_side.get_active_immutable(),
        defending_stat,
        &Weather::NONE,
        MOVES.get(&Choices::FUTURESIGHT).unwrap(),
    );

    (damage * 0.925) as i16
}
