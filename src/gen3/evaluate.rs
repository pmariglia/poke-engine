use super::abilities::Abilities;
use super::items::Items;
use super::state::PokemonVolatileStatus;
use crate::choices::MoveCategory;
use crate::state::{Pokemon, PokemonStatus, Side, State};

const POKEMON_ALIVE: f32 = 30.0;
const POKEMON_HP: f32 = 100.0;

const POKEMON_ATTACK_BOOST: f32 = 30.0;
const POKEMON_DEFENSE_BOOST: f32 = 15.0;
const POKEMON_SPECIAL_ATTACK_BOOST: f32 = 30.0;
const POKEMON_SPECIAL_DEFENSE_BOOST: f32 = 15.0;
const POKEMON_SPEED_BOOST: f32 = 30.0;

const POKEMON_BOOST_MULTIPLIER_6: f32 = 3.3;
const POKEMON_BOOST_MULTIPLIER_5: f32 = 3.15;
const POKEMON_BOOST_MULTIPLIER_4: f32 = 3.0;
const POKEMON_BOOST_MULTIPLIER_3: f32 = 2.5;
const POKEMON_BOOST_MULTIPLIER_2: f32 = 2.0;
const POKEMON_BOOST_MULTIPLIER_1: f32 = 1.0;
const POKEMON_BOOST_MULTIPLIER_0: f32 = 0.0;
const POKEMON_BOOST_MULTIPLIER_NEG_1: f32 = -1.0;
const POKEMON_BOOST_MULTIPLIER_NEG_2: f32 = -2.0;
const POKEMON_BOOST_MULTIPLIER_NEG_3: f32 = -2.5;
const POKEMON_BOOST_MULTIPLIER_NEG_4: f32 = -3.0;
const POKEMON_BOOST_MULTIPLIER_NEG_5: f32 = -3.15;
const POKEMON_BOOST_MULTIPLIER_NEG_6: f32 = -3.3;

const POKEMON_FROZEN: f32 = -40.0;
const POKEMON_ASLEEP: f32 = -25.0;
const POKEMON_PARALYZED: f32 = -25.0;
const POKEMON_TOXIC: f32 = -30.0;
const POKEMON_POISONED: f32 = -10.0;
const POKEMON_BURNED: f32 = -25.0;

const LEECH_SEED: f32 = -30.0;
const SUBSTITUTE: f32 = 40.0;
const CONFUSION: f32 = -20.0;

const REFLECT: f32 = 20.0;
const LIGHT_SCREEN: f32 = 20.0;

const SPIKES_ONE_LAYER: f32 = -12.0;
const SPIKES_TWO_LAYER: f32 = -16.0;
const SPIKES_THREE_LAYER: f32 = -25.0;

fn evaluate_poison(pokemon: &Pokemon, base_score: f32) -> f32 {
    match pokemon.ability {
        Abilities::GUTS | Abilities::MARVELSCALE => 10.0,
        _ => base_score,
    }
}

fn evaluate_burned(pokemon: &Pokemon) -> f32 {
    // burn is not as punishing in certain situations

    // guts, marvel scale, quick feet will result in a positive evaluation
    match pokemon.ability {
        Abilities::GUTS | Abilities::MARVELSCALE => return -2.0 * POKEMON_BURNED,
        _ => {}
    }

    let mut multiplier = 0.0;
    for mv in pokemon.moves.into_iter() {
        if mv.choice.category == MoveCategory::Physical {
            multiplier += 1.0;
        }
    }

    // don't make burn as punishing for special attackers
    if pokemon.special_attack > pokemon.attack {
        multiplier /= 2.0;
    }

    multiplier * POKEMON_BURNED
}

fn get_boost_multiplier(boost: i8) -> f32 {
    match boost {
        6 => POKEMON_BOOST_MULTIPLIER_6,
        5 => POKEMON_BOOST_MULTIPLIER_5,
        4 => POKEMON_BOOST_MULTIPLIER_4,
        3 => POKEMON_BOOST_MULTIPLIER_3,
        2 => POKEMON_BOOST_MULTIPLIER_2,
        1 => POKEMON_BOOST_MULTIPLIER_1,
        0 => POKEMON_BOOST_MULTIPLIER_0,
        -1 => POKEMON_BOOST_MULTIPLIER_NEG_1,
        -2 => POKEMON_BOOST_MULTIPLIER_NEG_2,
        -3 => POKEMON_BOOST_MULTIPLIER_NEG_3,
        -4 => POKEMON_BOOST_MULTIPLIER_NEG_4,
        -5 => POKEMON_BOOST_MULTIPLIER_NEG_5,
        -6 => POKEMON_BOOST_MULTIPLIER_NEG_6,
        _ => panic!("Invalid boost value: {}", boost),
    }
}

fn evaluate_hazards(pokemon: &Pokemon, side: &Side) -> f32 {
    let mut score = 0.0;
    if pokemon.is_grounded() {
        match side.side_conditions.spikes {
            1 => score += SPIKES_ONE_LAYER,
            2 => score += SPIKES_TWO_LAYER,
            3 => score += SPIKES_THREE_LAYER,
            _ => {}
        }
    }

    score
}

fn evaluate_pokemon(pokemon: &Pokemon) -> f32 {
    let mut score = 0.0;
    score += POKEMON_HP * pokemon.hp as f32 / pokemon.maxhp as f32;

    match pokemon.status {
        PokemonStatus::BURN => score += evaluate_burned(pokemon),
        PokemonStatus::FREEZE => score += POKEMON_FROZEN,
        PokemonStatus::SLEEP => score += POKEMON_ASLEEP,
        PokemonStatus::PARALYZE => score += POKEMON_PARALYZED,
        PokemonStatus::TOXIC => score += evaluate_poison(pokemon, POKEMON_TOXIC),
        PokemonStatus::POISON => score += evaluate_poison(pokemon, POKEMON_POISONED),
        PokemonStatus::NONE => {}
    }

    if pokemon.item != Items::NONE {
        score += 10.0;
    }

    for mv in pokemon.moves.into_iter() {
        // at 10 PP: (10*3) - 30 = 0  -> no penalty
        // at 1 pp: (1*3) - 30 = -27  -> -27 penalty
        if mv.pp <= 10 {
            let penalty = (mv.pp * 3) as f32 - 30.0;
            score += penalty;
        }
    }

    // without this a low hp pokemon could get a negative score and incentivize the other side
    // to keep it alive
    if score < 0.0 {
        score = 0.0;
    }

    score += POKEMON_ALIVE;

    score
}

pub fn evaluate(state: &State) -> f32 {
    let mut score = 0.0;

    let mut iter = state.side_one.pokemon.into_iter();
    while let Some(pkmn) = iter.next() {
        if pkmn.hp > 0 {
            score += evaluate_pokemon(pkmn);
            score += evaluate_hazards(pkmn, &state.side_one);
            if iter.pokemon_index == state.side_one.active_index {
                for vs in state.side_one.volatile_statuses.iter() {
                    match vs {
                        PokemonVolatileStatus::LEECHSEED => score += LEECH_SEED,
                        PokemonVolatileStatus::SUBSTITUTE => score += SUBSTITUTE,
                        PokemonVolatileStatus::CONFUSION => score += CONFUSION,
                        _ => {}
                    }
                }

                score += get_boost_multiplier(state.side_one.attack_boost) * POKEMON_ATTACK_BOOST;
                score += get_boost_multiplier(state.side_one.defense_boost) * POKEMON_DEFENSE_BOOST;
                score += get_boost_multiplier(state.side_one.special_attack_boost)
                    * POKEMON_SPECIAL_ATTACK_BOOST;
                score += get_boost_multiplier(state.side_one.special_defense_boost)
                    * POKEMON_SPECIAL_DEFENSE_BOOST;
                score += get_boost_multiplier(state.side_one.speed_boost) * POKEMON_SPEED_BOOST;
            }
        }
    }
    let mut iter = state.side_two.pokemon.into_iter();
    while let Some(pkmn) = iter.next() {
        if pkmn.hp > 0 {
            score -= evaluate_pokemon(pkmn);
            score -= evaluate_hazards(pkmn, &state.side_two);

            if iter.pokemon_index == state.side_two.active_index {
                for vs in state.side_two.volatile_statuses.iter() {
                    match vs {
                        PokemonVolatileStatus::LEECHSEED => score -= LEECH_SEED,
                        PokemonVolatileStatus::SUBSTITUTE => score -= SUBSTITUTE,
                        PokemonVolatileStatus::CONFUSION => score -= CONFUSION,
                        _ => {}
                    }
                }

                score -= get_boost_multiplier(state.side_two.attack_boost) * POKEMON_ATTACK_BOOST;
                score -= get_boost_multiplier(state.side_two.defense_boost) * POKEMON_DEFENSE_BOOST;
                score -= get_boost_multiplier(state.side_two.special_attack_boost)
                    * POKEMON_SPECIAL_ATTACK_BOOST;
                score -= get_boost_multiplier(state.side_two.special_defense_boost)
                    * POKEMON_SPECIAL_DEFENSE_BOOST;
                score -= get_boost_multiplier(state.side_two.speed_boost) * POKEMON_SPEED_BOOST;
            }
        }
    }

    score += state.side_one.side_conditions.reflect as f32 * REFLECT;
    score += state.side_one.side_conditions.light_screen as f32 * LIGHT_SCREEN;

    score -= state.side_two.side_conditions.reflect as f32 * REFLECT;
    score -= state.side_two.side_conditions.light_screen as f32 * LIGHT_SCREEN;

    score
}
