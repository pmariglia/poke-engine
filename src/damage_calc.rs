use crate::abilities::Abilities;
use crate::choices::{Choices, MOVES};
use crate::state::{PokemonIndex, Side, Terrain};
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
#[cfg(any(feature = "gen5",feature = "gen4", feature = "gen3"))]
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

#[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5"))]
pub const CRIT_MULTIPLIER: f32 = 2.0;

#[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
pub const CRIT_MULTIPLIER: f32 = 1.5;

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
        PokemonType::STELLAR => 18, // Stellar is typeless for type effectiveness
    }
}

pub fn type_effectiveness_modifier(attacking_type: &PokemonType, defender: &Pokemon) -> f32 {
    #[cfg(not(feature = "terastallization"))]
    let defending_types = defender.types;
    #[cfg(feature = "terastallization")]
    let defending_types = if defender.terastallized {
        (defender.tera_type, PokemonType::TYPELESS)
    } else {
        defender.types
    };
    _type_effectiveness_modifier(attacking_type, &defending_types)
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
    let move_has_basic_stab =
        attacking_move_type == &active_types.0 || attacking_move_type == &active_types.1;
    if active_pkmn.terastallized {
        if &active_pkmn.tera_type == attacking_move_type && move_has_basic_stab {
            return 2.0;
        } else if &active_pkmn.tera_type == attacking_move_type || move_has_basic_stab {
            return 1.5;
        }
    } else if move_has_basic_stab {
        return 1.5;
    }
    1.0
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

fn terrain_modifier(
    terrain: &Terrain,
    attacker: &Pokemon,
    defender: &Pokemon,
    choice: &Choice,
) -> f32 {
    #[cfg(any(feature = "gen9", feature = "gen8"))]
    let terrain_boost = 1.3;

    #[cfg(not(any(feature = "gen9", feature = "gen8")))]
    let terrain_boost = 1.5;

    match terrain {
        Terrain::ELECTRICTERRAIN => {
            if choice.move_type == PokemonType::ELECTRIC && attacker.is_grounded() {
                terrain_boost
            } else {
                1.0
            }
        }
        Terrain::GRASSYTERRAIN => {
            if choice.move_type == PokemonType::GRASS && attacker.is_grounded() {
                terrain_boost
            } else if choice.move_id == Choices::EARTHQUAKE {
                0.5
            } else {
                1.0
            }
        }
        Terrain::MISTYTERRAIN => {
            if choice.move_type == PokemonType::DRAGON && defender.is_grounded() {
                0.5
            } else {
                1.0
            }
        }
        Terrain::PSYCHICTERRAIN => {
            if choice.move_type == PokemonType::PSYCHIC && attacker.is_grounded() {
                terrain_boost
            } else {
                1.0
            }
        }
        Terrain::NONE => 1.0,
    }
}

fn volatile_status_modifier(choice: &Choice, attacking_side: &Side, defending_side: &Side) -> f32 {
    let mut modifier = 1.0;
    for vs in attacking_side.volatile_statuses.iter() {
        match vs {
            PokemonVolatileStatus::FLASHFIRE if choice.move_type == PokemonType::FIRE => {
                modifier *= 1.5;
            }
            PokemonVolatileStatus::SLOWSTART if choice.category == MoveCategory::Physical => {
                modifier *= 0.5;
            }
            PokemonVolatileStatus::CHARGE if choice.move_type == PokemonType::ELECTRIC => {
                modifier *= 2.0;
            }
            PokemonVolatileStatus::PROTOSYNTHESISATK | PokemonVolatileStatus::QUARKDRIVEATK
                if choice.category == MoveCategory::Physical =>
            {
                modifier *= 1.3;
            }
            PokemonVolatileStatus::PROTOSYNTHESISSPA | PokemonVolatileStatus::QUARKDRIVESPA
                if choice.category == MoveCategory::Special =>
            {
                modifier *= 1.3;
            }
            _ => {}
        }
    }

    for vs in defending_side.volatile_statuses.iter() {
        match vs {
            PokemonVolatileStatus::MAGNETRISE
                if choice.move_type == PokemonType::GROUND
                    && choice.move_id != Choices::THOUSANDARROWS =>
            {
                return 0.0;
            }
            PokemonVolatileStatus::TARSHOT if choice.move_type == PokemonType::FIRE => {
                modifier *= 2.0;
            }
            PokemonVolatileStatus::PHANTOMFORCE
            | PokemonVolatileStatus::SHADOWFORCE
            | PokemonVolatileStatus::BOUNCE
            | PokemonVolatileStatus::DIG
            | PokemonVolatileStatus::DIVE
            | PokemonVolatileStatus::FLY => {
                return 0.0;
            }
            PokemonVolatileStatus::GLAIVERUSH => {
                modifier *= 2.0;
            }
            PokemonVolatileStatus::PROTOSYNTHESISDEF | PokemonVolatileStatus::QUARKDRIVEDEF
                if choice.category == MoveCategory::Physical =>
            {
                modifier /= 1.3;
            }
            PokemonVolatileStatus::PROTOSYNTHESISSPD | PokemonVolatileStatus::QUARKDRIVESPD
                if choice.category == MoveCategory::Special =>
            {
                modifier /= 1.3;
            }
            _ => {}
        }
    }

    modifier
}

fn get_defending_types(
    side: &Side,
    defending_pkmn: &Pokemon,
    attacking_pkmn: &Pokemon,
    attacking_choice: &Choice,
) -> (PokemonType, PokemonType) {
    if defending_pkmn.terastallized && !(defending_pkmn.tera_type == PokemonType::STELLAR) {
        return (defending_pkmn.tera_type, PokemonType::TYPELESS);
    }
    let mut defender_types = defending_pkmn.types;
    if side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::ROOST)
    {
        if defender_types.0 == PokemonType::FLYING {
            defender_types = (PokemonType::TYPELESS, defender_types.1);
        }
        if defender_types.1 == PokemonType::FLYING {
            defender_types = (defender_types.0, PokemonType::TYPELESS);
        }
    }
    if (attacking_pkmn.ability == Abilities::SCRAPPY
        || attacking_pkmn.ability == Abilities::MINDSEYE)
        && (attacking_choice.move_type == PokemonType::NORMAL
            || attacking_choice.move_type == PokemonType::FIGHTING)
    {
        if defender_types.0 == PokemonType::GHOST {
            defender_types = (PokemonType::TYPELESS, defender_types.1);
        }
        if defender_types.1 == PokemonType::GHOST {
            defender_types = (defender_types.0, PokemonType::TYPELESS);
        }
    }
    defender_types
}

fn get_attacking_and_defending_stats(
    attacker: &Pokemon,
    defender: &Pokemon,
    attacking_side: &Side,
    defending_side: &Side,
    state: &State,
    choice: &Choice,
) -> (i16, i16, i16, i16) {
    let mut should_calc_attacker_boost = true;
    let mut should_calc_defender_boost = true;
    let defending_stat;
    let (
        attacking_final_stat,
        mut defending_final_stat,
        mut crit_attacking_stat,
        mut crit_defending_stat,
    );

    match choice.category {
        MoveCategory::Physical => {
            if attacking_side.attack_boost > 0 {
                crit_attacking_stat =
                    attacking_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
            } else {
                crit_attacking_stat = attacker.attack;
            }
            if defending_side.defense_boost <= 0 {
                crit_defending_stat =
                    defending_side.calculate_boosted_stat(PokemonBoostableStat::Defense);
            } else {
                crit_defending_stat = defender.defense;
            }

            // Unaware checks
            if defender.ability == Abilities::UNAWARE {
                should_calc_attacker_boost = false;
            }
            if attacker.ability == Abilities::UNAWARE {
                should_calc_defender_boost = false;
            }

            // Get the attacking stat

            // checks for moves that change which stat is used for the attacking_stat
            if choice.move_id == Choices::FOULPLAY {
                if should_calc_attacker_boost {
                    attacking_final_stat =
                        defending_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
                } else {
                    attacking_final_stat = defender.attack;
                }
                crit_attacking_stat = defending_side.get_active_immutable().attack;
            } else if choice.move_id == Choices::BODYPRESS {
                if should_calc_attacker_boost {
                    attacking_final_stat =
                        attacking_side.calculate_boosted_stat(PokemonBoostableStat::Defense);
                } else {
                    attacking_final_stat = attacker.defense;
                }
                crit_attacking_stat = attacking_side.get_active_immutable().defense;
            } else if should_calc_attacker_boost {
                attacking_final_stat =
                    attacking_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
            } else {
                attacking_final_stat = attacker.attack;
            }

            // Get the defending stat
            defending_stat = PokemonBoostableStat::Defense;
            if should_calc_defender_boost {
                defending_final_stat =
                    defending_side.calculate_boosted_stat(PokemonBoostableStat::Defense);
            } else {
                defending_final_stat = defender.defense;
            }
        }
        MoveCategory::Special => {
            if attacking_side.special_attack_boost > 0 {
                crit_attacking_stat =
                    attacking_side.calculate_boosted_stat(PokemonBoostableStat::SpecialAttack);
            } else {
                crit_attacking_stat = attacker.special_attack;
            }
            if defending_side.special_defense_boost <= 0 {
                crit_defending_stat =
                    defending_side.calculate_boosted_stat(PokemonBoostableStat::SpecialDefense);
            } else {
                crit_defending_stat = defender.special_defense;
            }

            // Unaware checks
            if defender.ability == Abilities::UNAWARE {
                should_calc_attacker_boost = false;
            }
            if attacker.ability == Abilities::UNAWARE {
                should_calc_defender_boost = false;
            }

            // Get the attacking stat
            if should_calc_attacker_boost {
                attacking_final_stat =
                    attacking_side.calculate_boosted_stat(PokemonBoostableStat::SpecialAttack);
            } else {
                attacking_final_stat = attacker.special_attack;
            }

            // Get the defending stat
            // check for moves that change which stat is used for the defending_stat
            if choice.move_id == Choices::PSYSHOCK
                || choice.move_id == Choices::SECRETSWORD
                || choice.move_id == Choices::PSYSTRIKE
            {
                if defending_side.defense_boost <= 0 {
                    crit_defending_stat =
                        defending_side.calculate_boosted_stat(PokemonBoostableStat::Defense);
                } else {
                    crit_defending_stat = defender.defense;
                }

                defending_stat = PokemonBoostableStat::Defense;
                if should_calc_defender_boost {
                    defending_final_stat =
                        defending_side.calculate_boosted_stat(PokemonBoostableStat::Defense);
                } else {
                    defending_final_stat = defender.defense;
                }
            } else {
                defending_stat = PokemonBoostableStat::SpecialDefense;
                if should_calc_defender_boost {
                    defending_final_stat =
                        defending_side.calculate_boosted_stat(PokemonBoostableStat::SpecialDefense);
                } else {
                    defending_final_stat = defender.special_defense;
                }
            }
        }
        _ => panic!("Can only calculate damage for physical or special moves"),
    }

    #[cfg(any(
        feature = "gen4",
        feature = "gen5",
        feature = "gen6",
        feature = "gen7",
        feature = "gen8",
        feature = "gen9"
    ))]
    if state.weather_is_active(&Weather::SNOW)
        && defender.has_type(&PokemonType::ICE)
        && defending_stat == PokemonBoostableStat::Defense
    {
        defending_final_stat = (defending_final_stat as f32 * 1.5) as i16;
    } else if state.weather_is_active(&Weather::SAND)
        && defender.has_type(&PokemonType::ROCK)
        && defending_stat == PokemonBoostableStat::SpecialDefense
    {
        defending_final_stat = (defending_final_stat as f32 * 1.5) as i16;
    }

    (
        attacking_final_stat,
        defending_final_stat,
        crit_attacking_stat,
        crit_defending_stat,
    )
}

fn common_pkmn_damage_calc(
    attacking_side: &Side,
    attacker: &Pokemon,
    attacking_stat: i16,
    defending_side: &Side,
    defender: &Pokemon,
    defending_stat: i16,
    weather: &Weather,
    terrain: &Terrain,
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

    let defender_types = get_defending_types(&defending_side, defender, attacker, choice);

    let mut damage_modifier = 1.0;
    damage_modifier *= _type_effectiveness_modifier(&choice.move_type, &defender_types);

    if attacker.ability != Abilities::CLOUDNINE
        && attacker.ability != Abilities::AIRLOCK
        && defender.ability != Abilities::CLOUDNINE
        && defender.ability != Abilities::AIRLOCK
    {
        damage_modifier *= weather_modifier(&choice.move_type, weather);
    }

    damage_modifier *= stab_modifier(&choice.move_type, &attacker);
    damage_modifier *= burn_modifier(&choice.category, &attacker.status);
    damage_modifier *= volatile_status_modifier(&choice, attacking_side, defending_side);
    damage_modifier *= terrain_modifier(terrain, attacker, defender, &choice);

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
    let (attacking_stat, defending_stat, crit_attacking_stat, crit_defending_stat) =
        get_attacking_and_defending_stats(
            attacker,
            defender,
            attacking_side,
            defending_side,
            state,
            &choice,
        );

    let mut damage = common_pkmn_damage_calc(
        attacking_side,
        attacker,
        attacking_stat,
        defending_side,
        defender,
        defending_stat,
        &state.weather.weather_type,
        &state.terrain.terrain_type,
        choice,
    );
    if attacker.ability != Abilities::INFILTRATOR {
        if defending_side.side_conditions.aurora_veil > 0 {
            damage *= 0.5
        } else if defending_side.side_conditions.reflect > 0
            && choice.category == MoveCategory::Physical
        {
            damage *= 0.5
        } else if defending_side.side_conditions.light_screen > 0
            && choice.category == MoveCategory::Special
        {
            damage *= 0.5
        }
    }

    let mut crit_damage = common_pkmn_damage_calc(
        attacking_side,
        attacker,
        crit_attacking_stat,
        defending_side,
        defender,
        crit_defending_stat,
        &state.weather.weather_type,
        &state.terrain.terrain_type,
        choice,
    );
    crit_damage *= CRIT_MULTIPLIER;

    match _damage_rolls {
        DamageRolls::Average => {
            damage = damage.floor() * 0.925;
            crit_damage = crit_damage.floor() * 0.925;
        }
        DamageRolls::Min => {
            damage = damage.floor() * 0.85;
            crit_damage = crit_damage.floor() * 0.85;
        }
        DamageRolls::Max => {
            damage = damage.floor();
            crit_damage = crit_damage.floor();
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
    let mut damage = common_pkmn_damage_calc(
        attacking_side,
        attacker,
        attacking_stat,
        defending_side,
        defending_side.get_active_immutable(),
        defending_stat,
        &Weather::NONE,
        &Terrain::NONE,
        MOVES.get(&Choices::FUTURESIGHT).unwrap(),
    );
    if attacker.ability != Abilities::INFILTRATOR {
        if defending_side.side_conditions.light_screen > 0 {
            damage *= 0.5
        }
    }

    (damage * 0.925) as i16
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
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        // level 100 tackle with 100 base stats across the board (attacker & defender)
        assert_eq!(32, dmg.unwrap().0);
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
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 0.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(0, dmg.unwrap().0);
    }

    #[test]
    fn test_boosted_damaging_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        state.side_one.attack_boost = 1;
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(48, dmg.unwrap().0);
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
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(32, dmg.unwrap().0);
    }

    #[test]
    fn test_basic_super_effective_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        state.side_two.get_active().types = (PokemonType::FIRE, PokemonType::TYPELESS);
        choice.move_id = Choices::WATERGUN;
        choice.move_type = PokemonType::WATER;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Special;
        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(64, dmg.unwrap().0);
    }

    #[test]
    fn test_basic_not_very_effective_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        state.side_two.get_active().types = (PokemonType::WATER, PokemonType::TYPELESS);
        choice.move_id = Choices::WATERGUN;
        choice.move_type = PokemonType::WATER;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Special;
        let dmg = calculate_damage(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(15, dmg.unwrap().0);
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

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
                }
             )*
        }
    }
    weather_tests! {
        test_rain_boosting_water: (Weather::RAIN, PokemonType::WATER, 48),
        test_rain_not_boosting_normal: (Weather::RAIN, PokemonType::NORMAL, 48),
        test_sun_boosting_fire: (Weather::SUN, PokemonType::FIRE, 48),
        test_sun_reducing_water: (Weather::SUN, PokemonType::WATER, 15),
        test_sun_not_boosting_normal: (Weather::SUN, PokemonType::NORMAL, 48),
        test_heavy_rain_makes_fire_do_zero: (Weather::HEAVYRAIN, PokemonType::FIRE, 0),
        test_heavy_rain_boost_water: (Weather::HEAVYRAIN, PokemonType::WATER, 48),
        test_harsh_sun_makes_water_do_zero: (Weather::HARSHSUN, PokemonType::WATER, 0),
        test_harsh_sun_boosting_fire: (Weather::HARSHSUN, PokemonType::FIRE, 48),
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

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
                }
             )*
        }
    }
    stab_tests! {
        test_basic_stab: ((PokemonType::WATER, PokemonType::FIRE), PokemonType::WATER, 48),
        test_basic_without_stab: ((PokemonType::WATER, PokemonType::FIRE), PokemonType::NORMAL, 32),
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
                    state.side_one.get_active().status = PokemonStatus::BURN;

                    choice.category = attacking_move_category;
                    choice.move_type = PokemonType::TYPELESS;
                    choice.base_power = 40.0;
                    let dmg = calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
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
                    choice.move_type = PokemonType::TYPELESS;
                    let dmg = calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
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

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
                }
             )*
        }
    }
    volatile_status_tests! {
        test_flashfire_boosts_fire_move: (
            vec![PokemonVolatileStatus::FLASHFIRE],
            vec![],
            PokemonType::FIRE,
            Choices::NONE,
            48
        ),
        test_flashfire_does_not_boost_normal_move: (
            vec![PokemonVolatileStatus::FLASHFIRE],
            vec![],
            PokemonType::TYPELESS,
            Choices::NONE,
            32
        ),
        test_magnetrise_makes_pkmn_immune_to_ground_move: (
            vec![],
            vec![PokemonVolatileStatus::MAGNETRISE],
            PokemonType::GROUND,
            Choices::NONE,
            0
        ),
        test_thousandarrows_can_hit_magnetrise_pokemon: (
            vec![],
            vec![PokemonVolatileStatus::MAGNETRISE],
            PokemonType::GROUND,
            Choices::THOUSANDARROWS,
            32
        ),
        test_tarshot_boosts_fire_move: (
            vec![],
            vec![PokemonVolatileStatus::TARSHOT],
            PokemonType::FIRE,
            Choices::NONE,
            64
        ),
        test_slowstart_halves_move: (
            vec![PokemonVolatileStatus::SLOWSTART],
            vec![],
            PokemonType::NORMAL,
            Choices::NONE,
            24
        ),
        test_tarshot_and_flashfire_together: (
            vec![PokemonVolatileStatus::FLASHFIRE],
            vec![PokemonVolatileStatus::TARSHOT],
            PokemonType::FIRE,
            Choices::NONE,
            97
        ),
        test_glaiverush_doubles_damage_against: (
            vec![],
            vec![PokemonVolatileStatus::GLAIVERUSH],
            PokemonType::NORMAL,
            Choices::NONE,
            97
        ),
        test_phantomforce_on_defender_causes_0_damage: (
            vec![],
            vec![PokemonVolatileStatus::PHANTOMFORCE],
            PokemonType::NORMAL,
            Choices::NONE,
            0
        ),
    }
}
