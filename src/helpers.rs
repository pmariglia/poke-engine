use std::collections::HashMap;
use std::collections::HashSet;

use super::data::moves::VolatileStatus;
use super::data::moves::SideCondition;
use super::data::conditions::Status;
use super::data::pokedex::get_pkmn;
use super::data::pokedex::BaseStats;
use super::data::pokedex::PokedexPokemon;
use super::state::PokemonNatures;
use super::state::State;
use super::state::Side;
use super::state::Pokemon;
use super::state::Terrain;
use super::state::Weather;


#[derive(Clone)]
struct PokemonStats {
    hitpoints: i16,
    attack: i16,
    defense: i16,
    special_attack: i16,
    special_defense: i16,
    speed: i16,
}


fn update_stats_from_nature(old_stats: &PokemonStats, nature: &PokemonNatures) -> PokemonStats {
    let mut stats: PokemonStats = old_stats.clone();

    match nature {
        // + Attack
        PokemonNatures::Lonely => {
            stats.attack = (stats.attack as f32 * 1.1) as i16;
            stats.defense = (stats.defense as f32 * 0.9) as i16;
        }
        PokemonNatures::Adamant => {
            stats.attack = (stats.attack as f32 * 1.1) as i16;
            stats.special_attack = (stats.special_attack as f32 * 0.9) as i16;
        }
        PokemonNatures::Naughty => {
            stats.attack = (stats.attack as f32 * 1.1) as i16;
            stats.special_defense = (stats.special_defense as f32 * 0.9) as i16;
        }
        PokemonNatures::Brave => {
            stats.attack = (stats.attack as f32 * 1.1) as i16;
            stats.speed = (stats.speed as f32 * 0.9) as i16;
        }

        // + Defense
        PokemonNatures::Bold => {
            stats.defense = (stats.defense as f32 * 1.1) as i16;
            stats.attack = (stats.attack as f32 * 0.9) as i16;
        }
        PokemonNatures::Impish => {
            stats.defense = (stats.defense as f32 * 1.1) as i16;
            stats.special_attack = (stats.special_attack as f32 * 0.9) as i16;
        }
        PokemonNatures::Lax => {
            stats.defense = (stats.defense as f32 * 1.1) as i16;
            stats.special_defense = (stats.special_defense as f32 * 0.9) as i16;
        }
        PokemonNatures::Relaxed => {
            stats.defense = (stats.defense as f32 * 1.1) as i16;
            stats.speed = (stats.speed as f32 * 0.9) as i16;
        }

        // + Special Attack
        PokemonNatures::Modest => {
            stats.special_attack = (stats.special_attack as f32 * 1.1) as i16;
            stats.attack = (stats.attack as f32 * 0.9) as i16;
        }
        PokemonNatures::Mild => {
            stats.special_attack = (stats.special_attack as f32 * 1.1) as i16;
            stats.defense = (stats.defense as f32 * 0.9) as i16;
        }
        PokemonNatures::Rash => {
            stats.special_attack = (stats.special_attack as f32 * 1.1) as i16;
            stats.special_defense = (stats.special_defense as f32 * 0.9) as i16;
        }
        PokemonNatures::Quiet => {
            stats.special_attack = (stats.special_attack as f32 * 1.1) as i16;
            stats.speed = (stats.speed as f32 * 0.9) as i16;
        }

        // + Special Defense
        PokemonNatures::Calm => {
            stats.special_defense = (stats.special_defense as f32 * 1.1) as i16;
            stats.attack = (stats.attack as f32 * 0.9) as i16;
        }
        PokemonNatures::Gentle => {
            stats.special_defense = (stats.special_defense as f32 * 1.1) as i16;
            stats.defense = (stats.defense as f32 * 0.9) as i16;
        }
        PokemonNatures::Careful => {
            stats.special_defense = (stats.special_defense as f32 * 1.1) as i16;
            stats.special_attack = (stats.special_attack as f32 * 0.9) as i16;
        }
        PokemonNatures::Sassy => {
            stats.special_defense = (stats.special_defense as f32 * 1.1) as i16;
            stats.speed = (stats.speed as f32 * 0.9) as i16;
        }

        // + Speed
        PokemonNatures::Timid => {
            stats.speed = (stats.speed as f32 * 1.1) as i16;
            stats.attack = (stats.attack as f32 * 0.9) as i16;
        }
        PokemonNatures::Hasty => {
            stats.speed = (stats.speed as f32 * 1.1) as i16;
            stats.defense = (stats.defense as f32 * 0.9) as i16;
        }
        PokemonNatures::Jolly => {
            stats.speed = (stats.speed as f32 * 1.1) as i16;
            stats.special_attack = (stats.special_attack as f32 * 0.9) as i16;
        }
        PokemonNatures::Naive => {
            stats.speed = (stats.speed as f32 * 1.1) as i16;
            stats.special_defense = (stats.special_defense as f32 * 0.9) as i16;
        }

        // Do nothing for the rest
        _ => (),
    }

    return stats;
}

fn common_pkmn_stat_calc(base_stat: i16, iv: i8, ev: i8, level: i8) -> i16 {
    return (((2 * base_stat as i32 + iv as i32 + (ev as i32 / 4) as i32) * level as i32) / 100)
        as i16;
}

fn calculate_stats(
    base_stats: &BaseStats,
    level: i8,
    ivs: (i8, i8, i8, i8, i8, i8),
    evs: (i8, i8, i8, i8, i8, i8),
    nature: &PokemonNatures,
) -> PokemonStats {
    let pkmn_stats = PokemonStats {
        hitpoints: common_pkmn_stat_calc(base_stats.hp, ivs.0, evs.0, level) + level as i16 + 10,
        attack: common_pkmn_stat_calc(base_stats.attack, ivs.1, evs.1, level) + 5,
        defense: common_pkmn_stat_calc(base_stats.defense, ivs.2, evs.2, level) + 5,
        special_attack: common_pkmn_stat_calc(base_stats.special_attack, ivs.3, evs.3, level) + 5,
        special_defense: common_pkmn_stat_calc(base_stats.special_defense, ivs.4, evs.4, level) + 5,
        speed: common_pkmn_stat_calc(base_stats.speed, ivs.5, evs.5, level) + 5,
    };
    let new_stats: PokemonStats = update_stats_from_nature(&pkmn_stats, &nature);
    return new_stats;
}

pub fn create_basic_pokemon(pkmn_name: String, level: i8) -> Pokemon {
    // Creates a pokemon at the given level with
    // 31 IVs in all stats, evenly distributed EVs,
    // a neutral nature, no item, and their 'first' ability

    // A good tool for debugging

    let pokedex_pkmn: &PokedexPokemon = get_pkmn(&pkmn_name);
    let nature: PokemonNatures = PokemonNatures::Serious;

    let pkmn_stats: PokemonStats = calculate_stats(
        &pokedex_pkmn.base_stats,
        level,
        (31, 31, 31, 31, 31, 31),
        (85, 85, 85, 85, 85, 85),
        &nature,
    );

    return Pokemon {
        id: pkmn_name,
        level: 100,
        types: pokedex_pkmn.types.clone(),
        hp: pkmn_stats.hitpoints,
        maxhp: pkmn_stats.hitpoints,
        ability: pokedex_pkmn.abilities.first.clone(),
        item: "none".to_string(),
        attack: pkmn_stats.attack,
        defense: pkmn_stats.defense,
        special_attack: pkmn_stats.special_attack,
        special_defense: pkmn_stats.special_defense,
        speed: pkmn_stats.speed,
        attack_boost: 0,
        defense_boost: 0,
        special_attack_boost: 0,
        special_defense_boost: 0,
        speed_boost: 0,
        accuracy_boost: 0,
        evasion_boost: 0,
        status: Status::None,
        nature: nature,
        volatile_statuses: HashSet::<VolatileStatus>::new(),
        moves: vec![],
    };
}


pub fn create_dummy_state() -> State {
    let pikachu: Pokemon = create_basic_pokemon("pikachu".to_string(), 100);
    let charizard: Pokemon = create_basic_pokemon("charizard".to_string(), 100);
    let blastoise: Pokemon = create_basic_pokemon("blastoise".to_string(), 100);
    let espeon: Pokemon = create_basic_pokemon("espeon".to_string(), 100);
    let snorlax: Pokemon = create_basic_pokemon("snorlax".to_string(), 100);
    let venusaur: Pokemon = create_basic_pokemon("venusaur".to_string(), 100);

    let charmander: Pokemon = create_basic_pokemon("charmander".to_string(), 100);
    let tapulele: Pokemon = create_basic_pokemon("tapulele".to_string(), 100);
    let rillaboom: Pokemon = create_basic_pokemon("rillaboom".to_string(), 100);
    let rhyperior: Pokemon = create_basic_pokemon("rhyperior".to_string(), 100);
    let gengar: Pokemon = create_basic_pokemon("gengar".to_string(), 100);
    let melmetal: Pokemon = create_basic_pokemon("melmetal".to_string(), 100);

    let my_side: Side = Side {
        active_index: 0,
        reserve: [pikachu, charizard, blastoise, espeon, snorlax, venusaur],
        side_conditions: HashMap::<SideCondition, i8>::new(),
        wish: (0, 0),
    };

    let your_side: Side = Side {
        active_index: 0,
        reserve: [charmander, tapulele, rillaboom, rhyperior, gengar, melmetal],
        side_conditions: HashMap::<SideCondition, i8>::new(),
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
