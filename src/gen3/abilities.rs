#![allow(unused_variables)]
use super::damage_calc::type_effectiveness_modifier;
use super::generate_instructions::{add_remove_status_instructions, get_boost_instruction};
use super::state::{PokemonVolatileStatus, Weather};
use crate::choices::{
    Boost, Choice, Choices, Effect, Heal, MoveCategory, MoveTarget, Secondary, StatBoosts,
    VolatileStatus,
};
use crate::define_enum_with_from_str;
use crate::instruction::{
    BoostInstruction, ChangeAbilityInstruction, ChangeStatusInstruction, ChangeType, ChangeWeather,
    DamageInstruction, HealInstruction, Instruction, StateInstructions,
};
use crate::state::{PokemonBoostableStat, PokemonStatus, PokemonType, SideReference, State};
use std::cmp;

pub const WEATHER_ABILITY_TURNS: i8 = -1;

define_enum_with_from_str! {
    #[repr(i16)]
    #[derive(PartialEq, Debug, Clone, Copy)]
    Abilities {
        NONE,
        AIRLOCK,
        ARENATRAP,
        BATTLEARMOR,
        BLAZE,
        CACOPHONY,
        CHLOROPHYLL,
        CLEARBODY,
        CLOUDNINE,
        COLORCHANGE,
        COMPOUNDEYES,
        CUTECHARM,
        DAMP,
        DRIZZLE,
        DROUGHT,
        DRYSKIN,
        EARLYBIRD,
        EFFECTSPORE,
        FLAMEBODY,
        FLASHFIRE,
        FORECAST,
        GUTS,
        HUGEPOWER,
        HUSTLE,
        HYPERCUTTER,
        ILLUMINATE,
        IMMUNITY,
        INNERFOCUS,
        INSOMNIA,
        INTIMIDATE,
        KEENEYE,
        LEVITATE,
        LIGHTNINGROD,
        LIMBER,
        LIQUIDOOZE,
        MAGMAARMOR,
        MAGNETPULL,
        MARVELSCALE,
        MINUS,
        NATURALCURE,
        OBLIVIOUS,
        OVERGROW,
        OWNTEMPO,
        PICKUP,
        PLUS,
        POISONPOINT,
        PRESSURE,
        PUREPOWER,
        RAINDISH,
        ROCKHEAD,
        ROUGHSKIN,
        RUNAWAY,
        SANDSTREAM,
        SANDVEIL,
        SERENEGRACE,
        SHADOWTAG,
        SHEDSKIN,
        SHELLARMOR,
        SHIELDDUST,
        SOUNDPROOF,
        SPEEDBOOST,
        STATIC,
        STENCH,
        STICKYHOLD,
        STURDY,
        SUCTIONCUPS,
        SWARM,
        SWIFTSWIM,
        SYNCHRONIZE,
        THICKFAT,
        TORRENT,
        TRACE,
        TRUANT,
        VITALSPIRIT,
        VOLTABSORB,
        WATERABSORB,
        WATERVEIL,
        WHITESMOKE,
        WONDERGUARD,
    },
    default = NONE
}

pub fn ability_after_damage_hit(
    state: &mut State,
    choice: &mut Choice,
    side_ref: &SideReference,
    damage_dealt: i16,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let active_pkmn = attacking_side.get_active();
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let attacking_pkmn = attacking_side.get_active();
    let defending_pkmn = defending_side.get_active();
    match defending_pkmn.ability {
        Abilities::COLORCHANGE => {
            if damage_dealt > 0
                && defending_pkmn.hp != 0
                && !defending_pkmn.has_type(&choice.move_type)
            {
                let change_type_instruction = Instruction::ChangeType(ChangeType {
                    side_ref: side_ref.get_other_side(),
                    new_types: (choice.move_type, PokemonType::TYPELESS),
                    old_types: defending_pkmn.types,
                });
                defending_pkmn.types = (choice.move_type, PokemonType::TYPELESS);
                instructions.instruction_list.push(change_type_instruction);
            }
        }
        Abilities::ROUGHSKIN => {
            if damage_dealt > 0 && choice.flags.contact {
                let damage_dealt = cmp::min(attacking_pkmn.maxhp / 16, attacking_pkmn.hp);
                instructions
                    .instruction_list
                    .push(Instruction::Damage(DamageInstruction {
                        side_ref: *side_ref,
                        damage_amount: damage_dealt,
                    }));
                attacking_pkmn.hp -= damage_dealt;
            }
        }
        _ => {}
    }
}

pub fn ability_on_switch_out(
    state: &mut State,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let active_pkmn = attacking_side.get_active();
    match active_pkmn.ability {
        Abilities::NATURALCURE => {
            if active_pkmn.status != PokemonStatus::NONE {
                let status = active_pkmn.status.clone();
                active_pkmn.status = PokemonStatus::NONE;
                instructions
                    .instruction_list
                    .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: *side_ref,
                        pokemon_index: attacking_side.active_index,
                        old_status: status,
                        new_status: PokemonStatus::NONE,
                    }));
            }
        }
        _ => {}
    }

    // revert ability on switch-out to base_ability if they are not the same
    let active_pkmn = state.get_side(side_ref).get_active();
    if active_pkmn.ability != active_pkmn.base_ability {
        instructions
            .instruction_list
            .push(Instruction::ChangeAbility(ChangeAbilityInstruction {
                side_ref: *side_ref,
                ability_change: active_pkmn.base_ability as i16 - active_pkmn.ability as i16,
            }));
        active_pkmn.ability = active_pkmn.base_ability;
    }
}

pub fn ability_end_of_turn(
    state: &mut State,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let active_pkmn = attacking_side.get_active();
    match active_pkmn.ability {
        Abilities::SPEEDBOOST => {
            if attacking_side.speed_boost < 6 {
                let ins = Instruction::Boost(BoostInstruction {
                    side_ref: side_ref.clone(),
                    stat: PokemonBoostableStat::Speed,
                    amount: 1,
                });
                attacking_side.speed_boost += 1;
                instructions.instruction_list.push(ins);
            }
        }
        Abilities::RAINDISH => {
            if state.weather_is_active(&Weather::RAIN) {
                let active_pkmn = state.get_side(side_ref).get_active();
                let health_recovered =
                    cmp::min(active_pkmn.maxhp / 16, active_pkmn.maxhp - active_pkmn.hp);
                if health_recovered > 0 {
                    instructions
                        .instruction_list
                        .push(Instruction::Heal(HealInstruction {
                            side_ref: *side_ref,
                            heal_amount: health_recovered,
                        }));
                    active_pkmn.hp += health_recovered;
                }
            }
        }
        Abilities::DRYSKIN => {
            if state.weather_is_active(&Weather::RAIN) {
                let active_pkmn = state.get_side(side_ref).get_active();
                if active_pkmn.hp < active_pkmn.maxhp {
                    let heal_amount =
                        cmp::min(active_pkmn.maxhp / 8, active_pkmn.maxhp - active_pkmn.hp);
                    let ins = Instruction::Heal(HealInstruction {
                        side_ref: side_ref.clone(),
                        heal_amount: heal_amount,
                    });
                    active_pkmn.hp += heal_amount;
                    instructions.instruction_list.push(ins);
                }
            }
        }
        // Shed skin only has a 1/3 chance of activating at the end of the turn
        // but I'm not going to branch on that here
        Abilities::SHEDSKIN => {
            if active_pkmn.status != PokemonStatus::NONE {
                let attacking_side = state.get_side(side_ref);
                let active_index = attacking_side.active_index;
                let active_pkmn = attacking_side.get_active();

                add_remove_status_instructions(
                    instructions,
                    active_index,
                    *side_ref,
                    attacking_side,
                );
            }
        }
        _ => {}
    }
}

pub fn ability_on_switch_in(
    state: &mut State,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let active_pkmn = attacking_side.get_active();
    let defending_pkmn = defending_side.get_active_immutable();

    // trace copying an ability needs to happen before the ability check to activate on switch-in
    // e.g. tracing intimidate will activate intimidate
    if active_pkmn.ability == Abilities::TRACE && active_pkmn.ability != defending_pkmn.ability {
        instructions
            .instruction_list
            .push(Instruction::ChangeAbility(ChangeAbilityInstruction {
                side_ref: *side_ref,
                ability_change: defending_pkmn.ability as i16 - active_pkmn.ability as i16,
            }));
        active_pkmn.ability = defending_pkmn.ability;
    }

    match active_pkmn.ability {
        Abilities::DROUGHT => {
            if state.weather.weather_type != Weather::SUN {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::SUN,
                        new_weather_turns_remaining: WEATHER_ABILITY_TURNS,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::SUN;
                state.weather.turns_remaining = WEATHER_ABILITY_TURNS;
            }
        }
        Abilities::SANDSTREAM => {
            if state.weather.weather_type != Weather::SAND {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::SAND,
                        new_weather_turns_remaining: WEATHER_ABILITY_TURNS,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::SAND;
                state.weather.turns_remaining = WEATHER_ABILITY_TURNS;
            }
        }
        Abilities::INTIMIDATE => {
            if let Some(boost_instruction) = get_boost_instruction(
                &defending_side,
                &PokemonBoostableStat::Attack,
                &-1,
                side_ref,
                &side_ref.get_other_side(),
            ) {
                state.apply_one_instruction(&boost_instruction);
                instructions.instruction_list.push(boost_instruction);
            }
        }
        Abilities::DRIZZLE => {
            if state.weather.weather_type != Weather::RAIN {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::RAIN,
                        new_weather_turns_remaining: WEATHER_ABILITY_TURNS,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::RAIN;
                state.weather.turns_remaining = WEATHER_ABILITY_TURNS;
            }
        }
        _ => {}
    }
}

pub fn ability_modify_attack_being_used(
    state: &State,
    attacker_choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    let attacking_pkmn = attacking_side.get_active_immutable();
    match attacking_pkmn.ability {
        Abilities::PUREPOWER => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.base_power *= 2.0;
            }
        }
        Abilities::TORRENT => {
            if attacker_choice.move_type == PokemonType::WATER
                && attacking_pkmn.hp <= attacking_pkmn.maxhp / 3
            {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::SERENEGRACE => {
            if let Some(secondaries) = &mut attacker_choice.secondaries {
                for secondary in secondaries.iter_mut() {
                    secondary.chance *= 2.0;
                }
            }
        }
        Abilities::HUGEPOWER => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.base_power *= 2.0;
            }
        }
        Abilities::COMPOUNDEYES => {
            attacker_choice.accuracy *= 1.3;
        }
        Abilities::STENCH => {
            let mut already_flinches = false;
            if let Some(secondaries) = &mut attacker_choice.secondaries {
                for secondary in secondaries.iter() {
                    if secondary.effect == Effect::VolatileStatus(PokemonVolatileStatus::FLINCH) {
                        already_flinches = true;
                    }
                }
            }
            if !already_flinches {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                })
            }
        }
        Abilities::SWARM => {
            if attacker_choice.move_type == PokemonType::BUG
                && attacking_pkmn.hp <= attacking_pkmn.maxhp / 3
            {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::BLAZE => {
            if attacker_choice.move_type == PokemonType::FIRE
                && attacking_pkmn.hp <= attacking_pkmn.maxhp / 3
            {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::OVERGROW => {
            if attacker_choice.move_type == PokemonType::GRASS
                && attacking_pkmn.hp <= attacking_pkmn.maxhp / 3
            {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::HUSTLE => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.base_power *= 1.5;
                attacker_choice.accuracy *= 0.80
            }
        }
        Abilities::GUTS => {
            if attacking_pkmn.status != PokemonStatus::NONE {
                attacker_choice.base_power *= 1.5;

                // not the right place to put this, but good enough
                if attacking_pkmn.status == PokemonStatus::BURN
                    && attacker_choice.category == MoveCategory::Physical
                {
                    attacker_choice.base_power *= 2.0;
                }
            }
        }
        _ => {}
    }
}

pub fn ability_modify_attack_against(
    state: &State,
    attacker_choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    let attacking_pkmn = attacking_side.get_active_immutable();
    let target_pkmn = defending_side.get_active_immutable();

    match target_pkmn.ability {
        Abilities::SOUNDPROOF => {
            if attacker_choice.flags.sound {
                attacker_choice.remove_all_effects();
                attacker_choice.accuracy = 0.0;
            }
        }
        Abilities::POISONPOINT => {
            if attacker_choice.flags.contact {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 33.0,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::POISON),
                })
            }
        }
        Abilities::LIGHTNINGROD => {
            if attacker_choice.move_type == PokemonType::ELECTRIC {
                attacker_choice.remove_all_effects();
                attacker_choice.accuracy = 100.0;
                attacker_choice.target = MoveTarget::Opponent;
                attacker_choice.boost = Some(Boost {
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                    target: MoveTarget::Opponent,
                });
                attacker_choice.category = MoveCategory::Status;
            }
        }
        Abilities::MARVELSCALE => {
            if target_pkmn.status != PokemonStatus::NONE
                && attacker_choice.category == MoveCategory::Physical
            {
                attacker_choice.base_power /= 1.5;
            }
        }
        Abilities::EFFECTSPORE => {
            if attacker_choice.flags.contact {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 3.30,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::POISON),
                });
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 3.30,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                });
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 3.30,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::SLEEP),
                });
            }
        }
        Abilities::FLAMEBODY => {
            if attacker_choice.flags.contact {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 30.0,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::BURN),
                });
            }
        }
        Abilities::SUCTIONCUPS => {
            attacker_choice.flags.drag = false;
        }
        Abilities::WONDERGUARD => {
            if attacker_choice.category != MoveCategory::Status
                && type_effectiveness_modifier(&attacker_choice.move_type, &target_pkmn) <= 1.0
            {
                attacker_choice.remove_all_effects();
                attacker_choice.base_power = 0.0;
            }
        }
        Abilities::LEVITATE => {
            if attacker_choice.move_type == PokemonType::GROUND
                && attacker_choice.target == MoveTarget::Opponent
                && attacker_choice.move_id != Choices::THOUSANDARROWS
            {
                attacker_choice.base_power = 0.0;
            }
        }
        Abilities::STATIC => {
            if attacker_choice.flags.contact {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 30.0,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                })
            }
        }
        Abilities::THICKFAT => {
            if attacker_choice.move_type == PokemonType::FIRE
                || attacker_choice.move_type == PokemonType::ICE
            {
                attacker_choice.base_power /= 2.0;
            }
        }
        Abilities::FLASHFIRE => {
            if attacker_choice.move_type == PokemonType::FIRE {
                attacker_choice.remove_all_effects();
                attacker_choice.volatile_status = Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::FLASHFIRE,
                });
            }
        }
        Abilities::LIQUIDOOZE => {
            if let Some(drain) = attacker_choice.drain {
                attacker_choice.drain = Some(-1.0 * drain);
            }
        }
        Abilities::SHIELDDUST => {
            if let Some(secondaries) = &mut attacker_choice.secondaries {
                for secondary in secondaries.iter_mut() {
                    if secondary.target == MoveTarget::Opponent {
                        secondary.chance = 0.0;
                    }
                }
            }
        }
        Abilities::WATERABSORB => {
            if attacker_choice.move_type == PokemonType::WATER {
                attacker_choice.remove_all_effects();
                attacker_choice.base_power = 0.0;
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::Opponent,
                    amount: 0.25,
                });
                attacker_choice.category = MoveCategory::Status;
            }
        }
        Abilities::DRYSKIN => {
            if attacker_choice.move_type == PokemonType::WATER {
                attacker_choice.remove_all_effects();
                attacker_choice.base_power = 0.0;
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::Opponent,
                    amount: 0.25,
                });
                attacker_choice.category = MoveCategory::Status;
            } else if attacker_choice.move_type == PokemonType::FIRE {
                attacker_choice.base_power *= 1.25;
            }
        }
        Abilities::DAMP => {
            if [
                Choices::SELFDESTRUCT,
                Choices::EXPLOSION,
                Choices::MINDBLOWN,
                Choices::MISTYEXPLOSION,
            ]
            .contains(&attacker_choice.move_id)
            {
                attacker_choice.accuracy = 0.0;
                attacker_choice.heal = None;
            }
        }
        Abilities::VOLTABSORB => {
            #[cfg(feature = "gen3")]
            let activate = attacker_choice.move_type == PokemonType::ELECTRIC
                && attacker_choice.category != MoveCategory::Status;

            #[cfg(not(feature = "gen3"))]
            let activate = attacker_choice.move_type == PokemonType::ELECTRIC;

            if activate {
                attacker_choice.remove_all_effects();
                attacker_choice.accuracy = 100.0;
                attacker_choice.base_power = 0.0;
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::Opponent,
                    amount: 0.25,
                });
                attacker_choice.category = MoveCategory::Status;
            }
        }
        _ => {}
    }
}
