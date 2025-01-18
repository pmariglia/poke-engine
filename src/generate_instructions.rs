use crate::abilities::{
    ability_after_damage_hit, ability_before_move, ability_end_of_turn,
    ability_modify_attack_against, ability_modify_attack_being_used, ability_on_switch_in,
    ability_on_switch_out, Abilities,
};
use crate::choice_effects::{
    charge_choice_to_volatile, choice_after_damage_hit, choice_before_move, choice_hazard_clear,
    choice_special_effect, modify_choice,
};
use crate::choices::{
    Boost, Choices, Effect, Heal, MoveTarget, MultiHitMove, Secondary, SideCondition, StatBoosts,
    Status, VolatileStatus, MOVES,
};
use crate::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeSideConditionInstruction, ChangeTerrain, ChangeWeather, DecrementRestTurnsInstruction,
    DecrementWishInstruction, HealInstruction, RemoveVolatileStatusInstruction,
    SetSecondMoveSwitchOutMoveInstruction, SetSleepTurnsInstruction, ToggleBatonPassingInstruction,
    ToggleTrickRoomInstruction,
};
use crate::instruction::{DecrementFutureSightInstruction, SetDamageDealtSideTwoInstruction};
use crate::instruction::{DecrementPPInstruction, SetLastUsedMoveInstruction};
use crate::instruction::{SetDamageDealtSideOneInstruction, ToggleTerastallizedInstruction};
use crate::state::PokemonMoveIndex;

use crate::damage_calc::calculate_futuresight_damage;
use crate::items::{
    item_before_move, item_end_of_turn, item_modify_attack_against, item_modify_attack_being_used,
    item_on_switch_in, Items,
};
use crate::state::{
    LastUsedMove, MoveChoice, PokemonBoostableStat, PokemonIndex, PokemonSideCondition,
    PokemonType, Side, Terrain,
};
use crate::{
    choices::{Choice, MoveCategory},
    damage_calc::{calculate_damage, type_effectiveness_modifier, DamageRolls},
    instruction::{
        ChangeStatusInstruction, DamageInstruction, Instruction, StateInstructions,
        SwitchInstruction,
    },
    state::{PokemonStatus, PokemonVolatileStatus, SideReference, State, Weather},
};
use std::cmp;

#[cfg(feature = "terastallization")]
use crate::choices::MultiAccuracyMove;

#[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5", feature = "gen6"))]
pub const BASE_CRIT_CHANCE: f32 = 1.0 / 16.0;

#[cfg(any(feature = "gen7", feature = "gen8", feature = "gen9"))]
pub const BASE_CRIT_CHANCE: f32 = 1.0 / 24.0;

#[cfg(any(feature = "gen3", feature = "gen4"))]
pub const MAX_SLEEP_TURNS: i8 = 4;

#[cfg(any(
    feature = "gen5",
    feature = "gen6",
    feature = "gen7",
    feature = "gen8",
    feature = "gen9"
))]
pub const MAX_SLEEP_TURNS: i8 = 3;

#[cfg(any(feature = "gen7", feature = "gen8", feature = "gen9"))]
pub const HIT_SELF_IN_CONFUSION_CHANCE: f32 = 1.0 / 3.0;

#[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5", feature = "gen6"))]
pub const HIT_SELF_IN_CONFUSION_CHANCE: f32 = 1.0 / 2.0;

fn chance_to_wake_up(turns_asleep: i8) -> f32 {
    if turns_asleep == 0 {
        0.0
    } else {
        1.0 / (1 + MAX_SLEEP_TURNS - turns_asleep) as f32
    }
}

fn set_last_used_move_as_switch(
    side: &mut Side,
    new_pokemon_index: PokemonIndex,
    switching_side_ref: SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    incoming_instructions
        .instruction_list
        .push(Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
            side_ref: switching_side_ref,
            last_used_move: LastUsedMove::Switch(new_pokemon_index),
            previous_last_used_move: side.last_used_move,
        }));
    side.last_used_move = LastUsedMove::Switch(new_pokemon_index);
}

fn set_last_used_move_as_move(
    side: &mut Side,
    used_move: PokemonMoveIndex,
    switching_side_ref: SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    if side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::FLINCH)
    {
        return;
    }
    match side.last_used_move {
        LastUsedMove::Move(last_used_move) => {
            if last_used_move == used_move {
                return;
            }
        }
        _ => {}
    }
    incoming_instructions
        .instruction_list
        .push(Instruction::SetLastUsedMove(SetLastUsedMoveInstruction {
            side_ref: switching_side_ref,
            last_used_move: LastUsedMove::Move(used_move),
            previous_last_used_move: side.last_used_move,
        }));
    side.last_used_move = LastUsedMove::Move(used_move);
}

fn generate_instructions_from_switch(
    state: &mut State,
    new_pokemon_index: PokemonIndex,
    switching_side_ref: SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    let should_last_used_move = state.use_last_used_move;
    state.apply_instructions(&incoming_instructions.instruction_list);

    let side = state.get_side(&switching_side_ref);
    if side.force_switch {
        side.force_switch = false;
        match switching_side_ref {
            SideReference::SideOne => {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::ToggleSideOneForceSwitch);
            }
            SideReference::SideTwo => {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::ToggleSideTwoForceSwitch);
            }
        }
    }

    let mut baton_passing = false;
    if side.baton_passing {
        baton_passing = true;
        side.baton_passing = false;
        match switching_side_ref {
            SideReference::SideOne => {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::ToggleBatonPassing(
                        ToggleBatonPassingInstruction {
                            side_ref: SideReference::SideOne,
                        },
                    ));
            }
            SideReference::SideTwo => {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::ToggleBatonPassing(
                        ToggleBatonPassingInstruction {
                            side_ref: SideReference::SideTwo,
                        },
                    ));
            }
        }
    }

    #[cfg(feature = "gen5")]
    if side.get_active_immutable().status == PokemonStatus::SLEEP {
        let current_active_index = side.active_index;
        let active = side.get_active();
        if active.rest_turns > 0 {
            let current_rest_turns = active.rest_turns;
            incoming_instructions
                .instruction_list
                .push(Instruction::SetRestTurns(SetSleepTurnsInstruction {
                    side_ref: switching_side_ref,
                    pokemon_index: current_active_index,
                    new_turns: 3,
                    previous_turns: current_rest_turns,
                }));
            active.rest_turns = 3
        } else {
            let current_sleep_turns = active.sleep_turns;
            incoming_instructions
                .instruction_list
                .push(Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: switching_side_ref,
                    pokemon_index: current_active_index,
                    new_turns: 0,
                    previous_turns: current_sleep_turns,
                }));
            active.sleep_turns = 0
        }
    }

    state.re_enable_disabled_moves(
        &switching_side_ref,
        &mut incoming_instructions.instruction_list,
    );
    state.remove_volatile_statuses_on_switch(
        &switching_side_ref,
        &mut incoming_instructions.instruction_list,
        baton_passing,
    );
    state.reset_toxic_count(
        &switching_side_ref,
        &mut incoming_instructions.instruction_list,
    );
    if !baton_passing {
        state.reset_boosts(
            &switching_side_ref,
            &mut incoming_instructions.instruction_list,
        );
    }

    ability_on_switch_out(state, &switching_side_ref, incoming_instructions);

    let switch_instruction = Instruction::Switch(SwitchInstruction {
        side_ref: switching_side_ref,
        previous_index: state.get_side(&switching_side_ref).active_index,
        next_index: new_pokemon_index,
    });

    let side = state.get_side(&switching_side_ref);
    side.active_index = new_pokemon_index;
    incoming_instructions
        .instruction_list
        .push(switch_instruction);

    if should_last_used_move {
        set_last_used_move_as_switch(
            side,
            new_pokemon_index,
            switching_side_ref,
            incoming_instructions,
        );
    }

    let active = side.get_active_immutable();
    if active.item != Items::HEAVYDUTYBOOTS && active.ability != Abilities::MAGICGUARD {
        if side.side_conditions.stealth_rock == 1 {
            let switched_in_pkmn = side.get_active();
            let multiplier = type_effectiveness_modifier(&PokemonType::ROCK, &switched_in_pkmn);

            let dmg_amount = cmp::min(
                (switched_in_pkmn.maxhp as f32 * multiplier / 8.0) as i16,
                switched_in_pkmn.hp,
            );
            let stealth_rock_dmg_instruction = Instruction::Damage(DamageInstruction {
                side_ref: switching_side_ref,
                damage_amount: dmg_amount,
            });
            switched_in_pkmn.hp -= dmg_amount;
            incoming_instructions
                .instruction_list
                .push(stealth_rock_dmg_instruction);
        }

        let switched_in_pkmn = side.get_active_immutable();
        if side.side_conditions.spikes > 0 && switched_in_pkmn.is_grounded() {
            let dmg_amount = cmp::min(
                switched_in_pkmn.maxhp * side.side_conditions.spikes as i16 / 8,
                switched_in_pkmn.hp,
            );
            let spikes_dmg_instruction = Instruction::Damage(DamageInstruction {
                side_ref: switching_side_ref,
                damage_amount: dmg_amount,
            });
            side.get_active().hp -= dmg_amount;
            incoming_instructions
                .instruction_list
                .push(spikes_dmg_instruction);
        }

        let switched_in_pkmn = side.get_active_immutable();
        if side.side_conditions.sticky_web == 1 && switched_in_pkmn.is_grounded() {
            // a pkmn switching in doesn't have any other speed drops,
            // so no need to check for going below -6
            if let Some(sticky_web_instruction) = get_boost_instruction(
                &side,
                &PokemonBoostableStat::Speed,
                &-1,
                &switching_side_ref,
                &switching_side_ref,
            ) {
                state.apply_one_instruction(&sticky_web_instruction);
                incoming_instructions
                    .instruction_list
                    .push(sticky_web_instruction);
            }
        }

        let side = state.get_side_immutable(&switching_side_ref);
        let switched_in_pkmn = side.get_active_immutable();
        let mut toxic_spike_instruction: Option<Instruction> = None;
        if side.side_conditions.toxic_spikes > 0 && switched_in_pkmn.is_grounded() {
            if !immune_to_status(
                &state,
                &MoveTarget::User,
                &switching_side_ref,
                &PokemonStatus::POISON,
            ) {
                if side.side_conditions.toxic_spikes == 1 {
                    toxic_spike_instruction =
                        Some(Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: switching_side_ref,
                            pokemon_index: side.active_index,
                            old_status: switched_in_pkmn.status,
                            new_status: PokemonStatus::POISON,
                        }))
                } else if side.side_conditions.toxic_spikes == 2 {
                    toxic_spike_instruction =
                        Some(Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: switching_side_ref,
                            pokemon_index: side.active_index,
                            old_status: switched_in_pkmn.status,
                            new_status: PokemonStatus::TOXIC,
                        }))
                }
            } else if switched_in_pkmn.has_type(&PokemonType::POISON) {
                toxic_spike_instruction = Some(Instruction::ChangeSideCondition(
                    ChangeSideConditionInstruction {
                        side_ref: switching_side_ref,
                        side_condition: PokemonSideCondition::ToxicSpikes,
                        amount: -1 * side.side_conditions.toxic_spikes,
                    },
                ))
            }

            if let Some(i) = toxic_spike_instruction {
                state.apply_one_instruction(&i);
                incoming_instructions.instruction_list.push(i);
            }
        }
    }

    ability_on_switch_in(state, &switching_side_ref, incoming_instructions);
    item_on_switch_in(state, &switching_side_ref, incoming_instructions);

    state.reverse_instructions(&incoming_instructions.instruction_list);
}

fn generate_instructions_from_side_conditions(
    state: &mut State,
    side_condition: &SideCondition,
    attacking_side_reference: &SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    let affected_side_ref;
    match side_condition.target {
        MoveTarget::Opponent => affected_side_ref = attacking_side_reference.get_other_side(),
        MoveTarget::User => affected_side_ref = *attacking_side_reference,
    }

    let max_layers;
    match side_condition.condition {
        PokemonSideCondition::Spikes => max_layers = 3,
        PokemonSideCondition::ToxicSpikes => max_layers = 2,
        PokemonSideCondition::AuroraVeil => {
            max_layers = if state.weather_is_active(&Weather::HAIL) {
                1
            } else {
                0
            }
        }
        _ => max_layers = 1,
    }

    let affected_side = state.get_side(&affected_side_ref);
    if affected_side.get_side_condition(side_condition.condition) < max_layers {
        let ins = Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
            side_ref: affected_side_ref,
            side_condition: side_condition.condition,
            amount: 1,
        });
        affected_side.update_side_condition(side_condition.condition, 1);
        incoming_instructions.instruction_list.push(ins);
    }
}

fn get_instructions_from_volatile_statuses(
    state: &mut State,
    attacker_choice: &Choice,
    volatile_status: &VolatileStatus,
    attacking_side_reference: &SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    let target_side: SideReference;
    match volatile_status.target {
        MoveTarget::Opponent => target_side = attacking_side_reference.get_other_side(),
        MoveTarget::User => target_side = *attacking_side_reference,
    }

    if volatile_status.volatile_status == PokemonVolatileStatus::YAWN
        && immune_to_status(
            state,
            &MoveTarget::Opponent,
            &target_side,
            &PokemonStatus::SLEEP,
        )
    {
        return;
    }
    let side = state.get_side(&target_side);
    let affected_pkmn = side.get_active_immutable();
    if affected_pkmn.volatile_status_can_be_applied(
        &volatile_status.volatile_status,
        &side.volatile_statuses,
        attacker_choice.first_move,
    ) {
        let ins = Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
            side_ref: target_side,
            volatile_status: volatile_status.volatile_status,
        });

        side.volatile_statuses
            .insert(volatile_status.volatile_status);
        incoming_instructions.instruction_list.push(ins);

        let affected_pkmn = state.get_side(&target_side).get_active();
        let damage_taken = affected_pkmn.maxhp / 4;
        if volatile_status.volatile_status == PokemonVolatileStatus::SUBSTITUTE {
            let ins = Instruction::Damage(DamageInstruction {
                side_ref: target_side,
                damage_amount: damage_taken,
            });
            affected_pkmn.hp -= damage_taken;
            incoming_instructions.instruction_list.push(ins);
        }
    }
}

pub fn add_remove_status_instructions(
    incoming_instructions: &mut StateInstructions,
    pokemon_index: PokemonIndex,
    side_reference: SideReference,
    side: &mut Side,
) {
    /*
    Single place to check for status removals, add the necessary instructions, and update the pokemon's status

    This is necessary because of some side effects to removing statuses
    i.e. a pre-mature wake-up from rest must set rest_turns to 0
    */
    let pkmn = &mut side.pokemon[pokemon_index];
    incoming_instructions
        .instruction_list
        .push(Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: side_reference,
            pokemon_index: pokemon_index,
            old_status: pkmn.status,
            new_status: PokemonStatus::NONE,
        }));
    match pkmn.status {
        PokemonStatus::SLEEP => {
            if pkmn.rest_turns > 0 {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::SetRestTurns(SetSleepTurnsInstruction {
                        side_ref: side_reference,
                        pokemon_index,
                        new_turns: 0,
                        previous_turns: pkmn.rest_turns,
                    }));
                pkmn.rest_turns = 0;
            } else if pkmn.sleep_turns > 0 {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                        side_ref: side_reference,
                        pokemon_index,
                        new_turns: 0,
                        previous_turns: pkmn.sleep_turns,
                    }));
                pkmn.sleep_turns = 0;
            }
        }
        _ => {}
    }
    pkmn.status = PokemonStatus::NONE;
}

pub fn immune_to_status(
    state: &State,
    status_target: &MoveTarget,
    target_side_ref: &SideReference,
    status: &PokemonStatus,
) -> bool {
    let target_side = state.get_side_immutable(target_side_ref);
    let target_pkmn = target_side.get_active_immutable();

    // General Status Immunity
    match target_pkmn.ability {
        Abilities::SHIELDSDOWN => return target_pkmn.hp > target_pkmn.maxhp / 2,
        Abilities::PURIFYINGSALT => return true,
        Abilities::COMATOSE => return true,
        Abilities::LEAFGUARD => return state.weather_is_active(&Weather::SUN),
        _ => {}
    }

    if target_pkmn.status != PokemonStatus::NONE || target_pkmn.hp <= 0 {
        true
    } else if state.terrain.terrain_type == Terrain::MISTYTERRAIN && target_pkmn.is_grounded() {
        true
    } else if (target_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::SUBSTITUTE)
        || target_side.side_conditions.safeguard > 0)
        && status_target == &MoveTarget::Opponent
    // substitute/safeguard don't block if the target is yourself (eg. rest)
    {
        true
    } else {
        // Specific status immunity
        match status {
            PokemonStatus::BURN => {
                target_pkmn.has_type(&PokemonType::FIRE)
                    || [
                        Abilities::WATERVEIL,
                        Abilities::WATERBUBBLE,
                        Abilities::THERMALEXCHANGE,
                    ]
                    .contains(&target_pkmn.ability)
            }
            PokemonStatus::FREEZE => {
                target_pkmn.has_type(&PokemonType::ICE)
                    || target_pkmn.ability == Abilities::MAGMAARMOR
                    || state.weather_is_active(&Weather::SUN)
                    || state.weather_is_active(&Weather::HARSHSUN)
            }
            PokemonStatus::SLEEP => {
                (state.terrain.terrain_type == Terrain::ELECTRICTERRAIN
                    && target_pkmn.is_grounded())
                    || [
                        Abilities::INSOMNIA,
                        Abilities::SWEETVEIL,
                        Abilities::VITALSPIRIT,
                    ]
                    .contains(&target_pkmn.ability)
                    || (status_target == &MoveTarget::Opponent
                        && target_side.has_alive_non_rested_sleeping_pkmn())
                // sleep clause
            }

            #[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
            PokemonStatus::PARALYZE => {
                target_pkmn.has_type(&PokemonType::ELECTRIC)
                    || target_pkmn.ability == Abilities::LIMBER
            }

            #[cfg(any(feature = "gen4", feature = "gen5", feature = "gen3"))]
            PokemonStatus::PARALYZE => target_pkmn.ability == Abilities::LIMBER,

            PokemonStatus::POISON | PokemonStatus::TOXIC => {
                target_pkmn.has_type(&PokemonType::POISON)
                    || target_pkmn.has_type(&PokemonType::STEEL)
                    || [Abilities::IMMUNITY, Abilities::PASTELVEIL].contains(&target_pkmn.ability)
            }
            _ => false,
        }
    }
}

fn get_instructions_from_status_effects(
    state: &mut State,
    status: &Status,
    attacking_side_reference: &SideReference,
    incoming_instructions: &mut StateInstructions,
    hit_sub: bool,
) {
    let target_side_ref: SideReference;
    match status.target {
        MoveTarget::Opponent => target_side_ref = attacking_side_reference.get_other_side(),
        MoveTarget::User => target_side_ref = *attacking_side_reference,
    }

    if hit_sub || immune_to_status(state, &status.target, &target_side_ref, &status.status) {
        return;
    }

    let target_side = state.get_side(&target_side_ref);
    let target_side_active = target_side.active_index;
    let target_pkmn = target_side.get_active();

    let status_hit_instruction = Instruction::ChangeStatus(ChangeStatusInstruction {
        side_ref: target_side_ref,
        pokemon_index: target_side_active,
        old_status: target_pkmn.status,
        new_status: status.status,
    });
    target_pkmn.status = status.status;
    incoming_instructions
        .instruction_list
        .push(status_hit_instruction);
}

pub fn get_boost_amount(side: &Side, boost: &PokemonBoostableStat, amount: i8) -> i8 {
    /*
    returns that amount that can actually be applied from the attempted boost amount
        e.g. using swordsdance at +5 attack would result in a +1 boost instead of +2
    */
    let current_boost = side.get_boost_from_boost_enum(boost);

    if amount > 0 {
        return cmp::min(6 - current_boost, amount);
    } else if amount < 0 {
        return cmp::max(-6 - current_boost, amount);
    }
    0
}

pub fn get_boost_instruction(
    target_side: &Side,
    stat: &PokemonBoostableStat,
    boost: &i8,
    attacking_side_ref: &SideReference,
    target_side_ref: &SideReference,
) -> Option<Instruction> {
    /*
    Single point for checking whether a boost can be applied to a pokemon
    Returns that boost instruction, if applicable
    */
    let target_pkmn = target_side.get_active_immutable();

    if boost != &0
        && !(target_side_ref != attacking_side_ref
            && target_pkmn
                .immune_to_stats_lowered_by_opponent(&stat, &target_side.volatile_statuses))
        && target_pkmn.hp != 0
    {
        let mut boost_amount = *boost;
        if target_pkmn.ability == Abilities::CONTRARY {
            boost_amount *= -1;
        }
        boost_amount = get_boost_amount(target_side, &stat, boost_amount);
        if boost_amount != 0 {
            return Some(Instruction::Boost(BoostInstruction {
                side_ref: *target_side_ref,
                stat: *stat,
                amount: boost_amount,
            }));
        }
    }
    None
}

fn get_instructions_from_boosts(
    state: &mut State,
    boosts: &Boost,
    attacking_side_reference: &SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    let target_side_ref: SideReference;
    match boosts.target {
        MoveTarget::Opponent => target_side_ref = attacking_side_reference.get_other_side(),
        MoveTarget::User => target_side_ref = *attacking_side_reference,
    }
    let boostable_stats = boosts.boosts.get_as_pokemon_boostable();
    for (pkmn_boostable_stat, boost) in boostable_stats.iter().filter(|(_, b)| b != &0) {
        let side = state.get_side_immutable(&target_side_ref);
        if let Some(boost_instruction) = get_boost_instruction(
            &side,
            pkmn_boostable_stat,
            boost,
            attacking_side_reference,
            &target_side_ref,
        ) {
            state.apply_one_instruction(&boost_instruction);
            incoming_instructions
                .instruction_list
                .push(boost_instruction);
        }
    }
}

fn compare_health_with_damage_multiples(max_damage: i16, health: i16) -> (i16, i16) {
    let max_damage_f32 = max_damage as f32;
    let health_f32 = health as f32;

    let mut total_less_than = 0;
    let mut num_less_than = 0;
    let mut num_greater_than = 0;
    let increment = max_damage as f32 * 0.01;
    let mut damage = max_damage_f32 * 0.85;
    for _ in 0..16 {
        if damage < health_f32 {
            total_less_than += damage as i16;
            num_less_than += 1;
        } else if damage > health_f32 {
            num_greater_than += 1;
        }
        damage += increment;
    }

    (total_less_than / num_less_than, num_greater_than)
}

fn get_instructions_from_secondaries(
    state: &mut State,
    attacker_choice: &Choice,
    secondaries: &Vec<Secondary>,
    side_reference: &SideReference,
    incoming_instructions: StateInstructions,
    hit_sub: bool,
) -> Vec<StateInstructions> {
    let mut return_instruction_list = Vec::with_capacity(16);
    return_instruction_list.push(incoming_instructions);

    for secondary in secondaries {
        if secondary.target == MoveTarget::Opponent && hit_sub {
            continue;
        }
        let secondary_percent_hit = (secondary.chance / 100.0).min(1.0);

        let mut i = 0;
        while i < return_instruction_list.len() {
            let mut secondary_hit_instructions = return_instruction_list.remove(i);

            if secondary_percent_hit < 1.0 {
                let mut secondary_miss_instructions = secondary_hit_instructions.clone();
                secondary_miss_instructions.update_percentage(1.0 - secondary_percent_hit);
                return_instruction_list.insert(i, secondary_miss_instructions);
                i += 1;
            }

            if secondary_percent_hit > 0.0 {
                secondary_hit_instructions.update_percentage(secondary_percent_hit);

                state.apply_instructions(&secondary_hit_instructions.instruction_list);
                match &secondary.effect {
                    Effect::VolatileStatus(volatile_status) => {
                        get_instructions_from_volatile_statuses(
                            state,
                            attacker_choice,
                            &VolatileStatus {
                                target: secondary.target.clone(),
                                volatile_status: volatile_status.clone(),
                            },
                            side_reference,
                            &mut secondary_hit_instructions,
                        );
                    }
                    Effect::Boost(boost) => {
                        get_instructions_from_boosts(
                            state,
                            &Boost {
                                target: secondary.target.clone(),
                                boosts: boost.clone(),
                            },
                            side_reference,
                            &mut secondary_hit_instructions,
                        );
                    }
                    Effect::Status(status) => {
                        get_instructions_from_status_effects(
                            state,
                            &Status {
                                target: secondary.target.clone(),
                                status: status.clone(),
                            },
                            side_reference,
                            &mut secondary_hit_instructions,
                            hit_sub,
                        );
                    }
                    Effect::Heal(heal_amount) => {
                        get_instructions_from_heal(
                            state,
                            &Heal {
                                target: secondary.target.clone(),
                                amount: *heal_amount,
                            },
                            side_reference,
                            &mut secondary_hit_instructions,
                        );
                    }
                    Effect::RemoveItem => {
                        let secondary_target_side_ref: SideReference;
                        match secondary.target {
                            MoveTarget::Opponent => {
                                secondary_target_side_ref = side_reference.get_other_side();
                            }
                            MoveTarget::User => {
                                secondary_target_side_ref = *side_reference;
                            }
                        }
                        let target_pkmn = state.get_side(&secondary_target_side_ref).get_active();
                        secondary_hit_instructions
                            .instruction_list
                            .push(Instruction::ChangeItem(ChangeItemInstruction {
                                side_ref: secondary_target_side_ref,
                                current_item: target_pkmn.item.clone(),
                                new_item: Items::NONE,
                            }));
                        target_pkmn.item = Items::NONE;
                    }
                }
                state.reverse_instructions(&secondary_hit_instructions.instruction_list);
                return_instruction_list.insert(i, secondary_hit_instructions);
                i += 1; // Increment i only if we didn't remove an element
            }
        }
    }

    return_instruction_list
}

fn get_instructions_from_heal(
    state: &mut State,
    heal: &Heal,
    attacking_side_reference: &SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    let target_side_ref: SideReference;
    match heal.target {
        MoveTarget::Opponent => target_side_ref = attacking_side_reference.get_other_side(),
        MoveTarget::User => target_side_ref = *attacking_side_reference,
    }

    let target_pkmn = state.get_side(&target_side_ref).get_active();

    let mut health_recovered = (heal.amount * target_pkmn.maxhp as f32) as i16;
    let final_health = target_pkmn.hp + health_recovered;
    if final_health > target_pkmn.maxhp {
        health_recovered -= final_health - target_pkmn.maxhp;
    } else if final_health < 0 {
        health_recovered -= final_health;
    }

    if health_recovered != 0 {
        let ins = Instruction::Heal(HealInstruction {
            side_ref: target_side_ref,
            heal_amount: health_recovered,
        });
        target_pkmn.hp += health_recovered;
        incoming_instructions.instruction_list.push(ins);
    }
}

fn check_move_hit_or_miss(
    state: &mut State,
    choice: &Choice,
    attacking_side_ref: &SideReference,
    damage: Option<(i16, i16)>,
    incoming_instructions: &mut StateInstructions,
    frozen_instructions: &mut Vec<StateInstructions>,
) {
    /*
    Checks whether a move can miss

    If the move can miss - adds it to `frozen_instructions`, signifying that the rest of the
    half-turn will not run.

    Otherwise, update the incoming instructions' percent_hit to reflect the chance of the move hitting
    */
    let attacking_side = state.get_side_immutable(attacking_side_ref);
    let attacking_pokemon = attacking_side.get_active_immutable();

    let mut percent_hit = (choice.accuracy / 100.0).min(1.0);
    if Some((0, 0)) == damage {
        percent_hit = 0.0;
    }

    if percent_hit < 1.0 {
        let mut move_missed_instruction = incoming_instructions.clone();
        move_missed_instruction.update_percentage(1.0 - percent_hit);
        if let Some(crash_fraction) = choice.crash {
            let crash_amount = (attacking_pokemon.maxhp as f32 * crash_fraction) as i16;
            let crash_instruction = Instruction::Damage(DamageInstruction {
                side_ref: *attacking_side_ref,
                damage_amount: cmp::min(crash_amount, attacking_pokemon.hp),
            });

            move_missed_instruction
                .instruction_list
                .push(crash_instruction);
        }

        if Items::BLUNDERPOLICY == attacking_pokemon.item {
            if let Some(boost_instruction) = get_boost_instruction(
                &attacking_side,
                &PokemonBoostableStat::Speed,
                &2,
                attacking_side_ref,
                attacking_side_ref,
            ) {
                move_missed_instruction
                    .instruction_list
                    .push(Instruction::ChangeItem(ChangeItemInstruction {
                        side_ref: *attacking_side_ref,
                        current_item: attacking_pokemon.item,
                        new_item: Items::NONE,
                    }));
                move_missed_instruction
                    .instruction_list
                    .push(boost_instruction);
            }
        }

        frozen_instructions.push(move_missed_instruction);
    }
    incoming_instructions.update_percentage(percent_hit);
}

fn get_instructions_from_drag(
    state: &mut State,
    attacking_side_reference: &SideReference,
    incoming_instructions: StateInstructions,
    frozen_instructions: &mut Vec<StateInstructions>,
) {
    let defending_side = state.get_side(&attacking_side_reference.get_other_side());
    if defending_side.get_active_immutable().hp == 0 {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        frozen_instructions.push(incoming_instructions);
        return;
    }

    let defending_side_alive_reserve_indices = defending_side.get_alive_pkmn_indices();

    state.reverse_instructions(&incoming_instructions.instruction_list);

    let num_alive_reserve = defending_side_alive_reserve_indices.len();
    if num_alive_reserve == 0 {
        frozen_instructions.push(incoming_instructions);
        return;
    }

    for pkmn_id in defending_side_alive_reserve_indices {
        let mut cloned_instructions = incoming_instructions.clone();
        generate_instructions_from_switch(
            state,
            pkmn_id,
            attacking_side_reference.get_other_side(),
            &mut cloned_instructions,
        );
        cloned_instructions.update_percentage(1.0 / num_alive_reserve as f32);
        frozen_instructions.push(cloned_instructions);
    }
}

fn reset_damage_dealt(
    side: &Side,
    side_reference: &SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    if side.damage_dealt.damage != 0
        || side.damage_dealt.move_category != MoveCategory::Physical
        || side.damage_dealt.hit_substitute
    {
        match side_reference {
            SideReference::SideOne => {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::SetDamageDealtSideOne(
                        SetDamageDealtSideOneInstruction {
                            damage_change: -1 * side.damage_dealt.damage,
                            move_category: MoveCategory::Physical,
                            previous_move_category: side.damage_dealt.move_category,
                            toggle_hit_substitute: side.damage_dealt.hit_substitute,
                        },
                    ));
            }
            SideReference::SideTwo => {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::SetDamageDealtSideTwo(
                        SetDamageDealtSideTwoInstruction {
                            damage_change: -1 * side.damage_dealt.damage,
                            move_category: MoveCategory::Physical,
                            previous_move_category: side.damage_dealt.move_category,
                            toggle_hit_substitute: side.damage_dealt.hit_substitute,
                        },
                    ));
            }
        }
    }
}

fn set_damage_dealt(
    attacking_side: &mut Side,
    attacking_side_ref: &SideReference,
    damage_dealt: i16,
    choice: &Choice,
    hit_substitute: bool,
    incoming_instructions: &mut StateInstructions,
) {
    match attacking_side_ref {
        SideReference::SideOne => {
            incoming_instructions
                .instruction_list
                .push(Instruction::SetDamageDealtSideOne(
                    SetDamageDealtSideOneInstruction {
                        damage_change: damage_dealt - attacking_side.damage_dealt.damage,
                        move_category: choice.category,
                        previous_move_category: attacking_side.damage_dealt.move_category,
                        toggle_hit_substitute: attacking_side.damage_dealt.hit_substitute
                            != hit_substitute,
                    },
                ));
        }
        SideReference::SideTwo => {
            incoming_instructions
                .instruction_list
                .push(Instruction::SetDamageDealtSideTwo(
                    SetDamageDealtSideTwoInstruction {
                        damage_change: damage_dealt - attacking_side.damage_dealt.damage,
                        move_category: choice.category,
                        previous_move_category: attacking_side.damage_dealt.move_category,
                        toggle_hit_substitute: attacking_side.damage_dealt.hit_substitute
                            != hit_substitute,
                    },
                ));
        }
    }
    attacking_side.damage_dealt.damage = damage_dealt;
    attacking_side.damage_dealt.move_category = choice.category;
    attacking_side.damage_dealt.hit_substitute = hit_substitute;
}

fn generate_instructions_from_damage(
    mut state: &mut State,
    choice: &Choice,
    calculated_damage: i16,
    attacking_side_ref: &SideReference,
    mut incoming_instructions: &mut StateInstructions,
) -> bool {
    /*
    TODO:
        - arbitrary other after_move as well from the old engine (triggers on hit OR miss)
            - dig/dive/bounce/fly volatilestatus
    */
    let mut hit_sub = false;
    let attacking_side = state.get_side(attacking_side_ref);
    let attacking_pokemon = attacking_side.get_active();

    if calculated_damage <= 0 {
        if let Some(crash_fraction) = choice.crash {
            let crash_amount = (attacking_pokemon.maxhp as f32 * crash_fraction) as i16;
            let damage_taken = cmp::min(crash_amount, attacking_pokemon.hp);
            let crash_instruction = Instruction::Damage(DamageInstruction {
                side_ref: *attacking_side_ref,
                damage_amount: damage_taken,
            });
            attacking_pokemon.hp -= damage_taken;
            incoming_instructions
                .instruction_list
                .push(crash_instruction);
        }
        return hit_sub;
    }

    let percent_hit = (choice.accuracy / 100.0).min(1.0);

    if percent_hit > 0.0 {
        let should_use_damage_dealt = state.use_damage_dealt;
        let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
        let attacking_pokemon = attacking_side.get_active();
        let mut damage_dealt;
        if defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SUBSTITUTE)
            && !choice.flags.sound
            && attacking_pokemon.ability != Abilities::INFILTRATOR
        {
            damage_dealt = cmp::min(calculated_damage, defending_side.substitute_health);
            let substitute_damage_dealt = cmp::min(calculated_damage, damage_dealt);
            let substitute_instruction = Instruction::DamageSubstitute(DamageInstruction {
                side_ref: attacking_side_ref.get_other_side(),
                damage_amount: substitute_damage_dealt,
            });
            defending_side.substitute_health -= substitute_damage_dealt;
            incoming_instructions
                .instruction_list
                .push(substitute_instruction);

            if should_use_damage_dealt {
                set_damage_dealt(
                    attacking_side,
                    attacking_side_ref,
                    damage_dealt,
                    choice,
                    true,
                    &mut incoming_instructions,
                );
            }

            if defending_side
                .volatile_statuses
                .contains(&PokemonVolatileStatus::SUBSTITUTE)
                && defending_side.substitute_health == 0
            {
                incoming_instructions
                    .instruction_list
                    .push(Instruction::RemoveVolatileStatus(
                        RemoveVolatileStatusInstruction {
                            side_ref: attacking_side_ref.get_other_side(),
                            volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                        },
                    ));
                defending_side
                    .volatile_statuses
                    .remove(&PokemonVolatileStatus::SUBSTITUTE);
            }

            hit_sub = true;
        } else {
            let attacking_pokemon = attacking_side.get_active();
            let defending_pokemon = defending_side.get_active();
            let mut knocked_out = false;
            damage_dealt = cmp::min(calculated_damage, defending_pokemon.hp);
            if damage_dealt != 0 {
                if (defending_pokemon.ability == Abilities::STURDY
                    || defending_pokemon.item == Items::FOCUSSASH)
                    && defending_pokemon.maxhp == defending_pokemon.hp
                {
                    damage_dealt -= 1;
                }

                if damage_dealt >= defending_pokemon.hp {
                    knocked_out = true;
                }

                let damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    damage_amount: damage_dealt,
                });
                defending_pokemon.hp -= damage_dealt;
                incoming_instructions
                    .instruction_list
                    .push(damage_instruction);

                if knocked_out
                    && defending_side
                        .volatile_statuses
                        .contains(&PokemonVolatileStatus::DESTINYBOND)
                {
                    let damage_instruction = Instruction::Damage(DamageInstruction {
                        side_ref: *attacking_side_ref,
                        damage_amount: attacking_pokemon.hp,
                    });
                    attacking_pokemon.hp = 0;
                    incoming_instructions
                        .instruction_list
                        .push(damage_instruction);
                }

                if should_use_damage_dealt {
                    set_damage_dealt(
                        attacking_side,
                        attacking_side_ref,
                        damage_dealt,
                        choice,
                        false,
                        &mut incoming_instructions,
                    );
                }

                ability_after_damage_hit(
                    &mut state,
                    choice,
                    attacking_side_ref,
                    damage_dealt,
                    &mut incoming_instructions,
                );
            }
        }

        let attacking_pokemon = state.get_side(attacking_side_ref).get_active();
        if let Some(drain_fraction) = choice.drain {
            let drain_amount = (damage_dealt as f32 * drain_fraction) as i16;
            let heal_amount =
                cmp::min(drain_amount, attacking_pokemon.maxhp - attacking_pokemon.hp);
            if heal_amount != 0 {
                let drain_instruction = Instruction::Heal(HealInstruction {
                    side_ref: *attacking_side_ref,
                    heal_amount: heal_amount,
                });
                attacking_pokemon.hp += heal_amount;
                incoming_instructions
                    .instruction_list
                    .push(drain_instruction);
            }
        }

        let attacking_pokemon = state.get_side(attacking_side_ref).get_active();
        if let Some(recoil_fraction) = choice.recoil {
            let recoil_amount = (damage_dealt as f32 * recoil_fraction) as i16;
            let damage_amount = cmp::min(recoil_amount, attacking_pokemon.hp);
            let recoil_instruction = Instruction::Damage(DamageInstruction {
                side_ref: *attacking_side_ref,
                damage_amount: damage_amount,
            });
            attacking_pokemon.hp -= damage_amount;
            incoming_instructions
                .instruction_list
                .push(recoil_instruction);
        }
        choice_after_damage_hit(
            &mut state,
            &choice,
            attacking_side_ref,
            &mut incoming_instructions,
            hit_sub,
        );
    }
    hit_sub
}

fn move_has_no_effect(state: &State, choice: &Choice, attacking_side_ref: &SideReference) -> bool {
    let (_attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    let defender = defending_side.get_active_immutable();

    #[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
    if choice.flags.powder
        && choice.target == MoveTarget::Opponent
        && defender.has_type(&PokemonType::GRASS)
    {
        return true;
    }

    if choice.move_type == PokemonType::ELECTRIC
        && choice.target == MoveTarget::Opponent
        && defender.has_type(&PokemonType::GROUND)
    {
        return true;
    } else if choice.move_id == Choices::ENCORE {
        return match state
            .get_side_immutable(&attacking_side_ref.get_other_side())
            .last_used_move
        {
            LastUsedMove::None => true,
            LastUsedMove::Move(_) => false,
            LastUsedMove::Switch(_) => true,
        };
    } else if state.terrain_is_active(&Terrain::PSYCHICTERRAIN)
        && defender.is_grounded()
        && choice.priority > 0
    {
        return true;
    }
    false
}

fn cannot_use_move(state: &State, choice: &Choice, attacking_side_ref: &SideReference) -> bool {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);

    // If the opponent has 0 hp, you can't use a non-status move
    if defending_side.get_active_immutable().hp == 0 && choice.category != MoveCategory::Status {
        return true;
    }

    // If you were taunted, you can't use a Physical/Special move
    if attacking_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::TAUNT)
        && matches!(choice.category, MoveCategory::Status)
    {
        return true;
    } else if attacking_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::FLINCH)
    {
        return true;
    } else if choice.flags.heal
        && attacking_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::HEALBLOCK)
    {
        return true;
    }
    false
}

#[cfg(feature = "terastallization")]
fn terastallized_base_power_floor(
    state: &mut State,
    choice: &mut Choice,
    attacking_side: &SideReference,
) {
    let attacker = state
        .get_side_immutable(attacking_side)
        .get_active_immutable();

    if attacker.terastallized
        && choice.move_type == attacker.tera_type
        && choice.base_power < 60.0
        && choice.priority <= 0
        && choice.multi_hit() == MultiHitMove::None
        && choice.multi_accuracy() == MultiAccuracyMove::None
    {
        choice.base_power = 60.0;
    }
}

fn before_move(
    state: &mut State,
    choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side: &SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    #[cfg(feature = "terastallization")]
    terastallized_base_power_floor(state, choice, attacking_side);

    ability_before_move(state, choice, attacking_side, incoming_instructions);
    item_before_move(state, choice, attacking_side, incoming_instructions);
    choice_before_move(state, choice, attacking_side, incoming_instructions);

    modify_choice(state, choice, defender_choice, attacking_side);

    ability_modify_attack_being_used(state, choice, defender_choice, attacking_side);
    ability_modify_attack_against(state, choice, defender_choice, attacking_side);

    item_modify_attack_being_used(state, choice, attacking_side);
    item_modify_attack_against(state, choice, attacking_side);

    /*
        TODO: this needs to be here because from_drag is called after the substitute volatilestatus
            has already been removed
    */
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side);
    if defending_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::SUBSTITUTE)
        && choice.category != MoveCategory::Status
    {
        choice.flags.drag = false;
    }

    // Update Choice for `charge` moves
    if choice.flags.charge {
        let charge_volatile_status = charge_choice_to_volatile(&choice.move_id);
        if !attacking_side
            .volatile_statuses
            .contains(&charge_volatile_status)
        {
            choice.remove_all_effects();
            choice.volatile_status = Some(VolatileStatus {
                target: MoveTarget::User,
                volatile_status: charge_volatile_status,
            });
        }
    }

    // modify choice if defender has protect active
    if (defending_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::PROTECT)
        || defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SPIKYSHIELD)
        || defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::BANEFULBUNKER)
        || defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::BURNINGBULWARK)
        || defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SILKTRAP))
        && choice.flags.protect
    {
        choice.remove_effects_for_protect();
        if choice.crash.is_some() {
            choice.accuracy = 0.0;
        }

        if defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SPIKYSHIELD)
            && choice.flags.contact
        {
            choice.heal = Some(Heal {
                target: MoveTarget::User,
                amount: -0.125,
            })
        } else if defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::BANEFULBUNKER)
            && choice.flags.contact
        {
            choice.status = Some(Status {
                target: MoveTarget::User,
                status: PokemonStatus::POISON,
            })
        } else if defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::BURNINGBULWARK)
            && choice.flags.contact
        {
            choice.status = Some(Status {
                target: MoveTarget::User,
                status: PokemonStatus::BURN,
            })
        } else if defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SILKTRAP)
            && choice.flags.contact
        {
            choice.boost = Some(Boost {
                target: MoveTarget::User,
                boosts: StatBoosts {
                    attack: 0,
                    defense: 0,
                    special_attack: 0,
                    special_defense: 0,
                    speed: -1,
                    accuracy: 0,
                },
            })
        }
    }
}

fn generate_instructions_from_existing_status_conditions(
    state: &mut State,
    attacking_side_ref: &SideReference,
    attacker_choice: &Choice,
    incoming_instructions: &mut StateInstructions,
    final_instructions: &mut Vec<StateInstructions>,
) {
    let (attacking_side, _defending_side) = state.get_both_sides(attacking_side_ref);
    let current_active_index = attacking_side.active_index;
    let attacker_active = attacking_side.get_active();
    match attacker_active.status {
        PokemonStatus::PARALYZE => {
            // Fully-Paralyzed Branch
            let mut fully_paralyzed_instruction = incoming_instructions.clone();
            fully_paralyzed_instruction.update_percentage(0.25);
            final_instructions.push(fully_paralyzed_instruction);

            // Non-Paralyzed Branch
            incoming_instructions.update_percentage(0.75);
        }
        PokemonStatus::FREEZE => {
            let mut still_frozen_instruction = incoming_instructions.clone();
            still_frozen_instruction.update_percentage(0.80);
            final_instructions.push(still_frozen_instruction);

            incoming_instructions.update_percentage(0.20);
            attacker_active.status = PokemonStatus::NONE;
            incoming_instructions
                .instruction_list
                .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: attacking_side_ref.clone(),
                    pokemon_index: current_active_index,
                    old_status: PokemonStatus::FREEZE,
                    new_status: PokemonStatus::NONE,
                }));
        }
        PokemonStatus::SLEEP => {
            match attacker_active.rest_turns {
                // Pokemon is not asleep because of Rest.
                0 => {
                    let current_sleep_turns = attacker_active.sleep_turns;
                    let chance_to_wake = chance_to_wake_up(current_sleep_turns);
                    if chance_to_wake == 1.0 {
                        attacker_active.status = PokemonStatus::NONE;
                        attacker_active.sleep_turns = 0;
                        incoming_instructions
                            .instruction_list
                            .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                                side_ref: *attacking_side_ref,
                                pokemon_index: current_active_index,
                                old_status: PokemonStatus::SLEEP,
                                new_status: PokemonStatus::NONE,
                            }));
                        incoming_instructions
                            .instruction_list
                            .push(Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                side_ref: *attacking_side_ref,
                                pokemon_index: current_active_index,
                                new_turns: 0,
                                previous_turns: current_sleep_turns,
                            }));
                    } else if chance_to_wake == 0.0 {
                        if attacker_choice.move_id == Choices::SLEEPTALK {
                            // if we are using sleeptalk we want to continue using this move
                            incoming_instructions.instruction_list.push(
                                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                    side_ref: *attacking_side_ref,
                                    pokemon_index: current_active_index,
                                    new_turns: current_sleep_turns + 1,
                                    previous_turns: current_sleep_turns,
                                }),
                            );
                        } else {
                            let mut still_asleep_instruction = incoming_instructions.clone();
                            still_asleep_instruction.update_percentage(1.0);
                            still_asleep_instruction.instruction_list.push(
                                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                    side_ref: *attacking_side_ref,
                                    pokemon_index: current_active_index,
                                    new_turns: current_sleep_turns + 1,
                                    previous_turns: current_sleep_turns,
                                }),
                            );
                            final_instructions.push(still_asleep_instruction);
                            incoming_instructions.update_percentage(0.0);
                        }
                    } else {
                        // This code deals with the situation where there is a chance to wake up
                        // as well as a chance to stay asleep.
                        // This logic will branch the state and one branch will represent where
                        // nothing happens and the other will represent where something happens
                        // Normally "nothing happens" means you stay asleep and "something happens"
                        // means you wake up. If the move is sleeptalk these are reversed.
                        let do_nothing_percentage;
                        let mut do_nothing_instructions = incoming_instructions.clone();
                        if attacker_choice.move_id == Choices::SLEEPTALK {
                            do_nothing_percentage = chance_to_wake;
                            do_nothing_instructions.instruction_list.push(
                                Instruction::ChangeStatus(ChangeStatusInstruction {
                                    side_ref: *attacking_side_ref,
                                    pokemon_index: current_active_index,
                                    old_status: PokemonStatus::SLEEP,
                                    new_status: PokemonStatus::NONE,
                                }),
                            );
                            do_nothing_instructions.instruction_list.push(
                                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                    side_ref: *attacking_side_ref,
                                    pokemon_index: current_active_index,
                                    new_turns: 0,
                                    previous_turns: current_sleep_turns,
                                }),
                            );
                            incoming_instructions.instruction_list.push(
                                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                    side_ref: *attacking_side_ref,
                                    pokemon_index: current_active_index,
                                    new_turns: current_sleep_turns + 1,
                                    previous_turns: current_sleep_turns,
                                }),
                            );
                            attacker_active.sleep_turns += 1;
                        } else {
                            do_nothing_percentage = 1.0 - chance_to_wake;
                            do_nothing_instructions.instruction_list.push(
                                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                    side_ref: *attacking_side_ref,
                                    pokemon_index: current_active_index,
                                    new_turns: current_sleep_turns + 1,
                                    previous_turns: current_sleep_turns,
                                }),
                            );
                            incoming_instructions
                                .instruction_list
                                .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                                    side_ref: *attacking_side_ref,
                                    pokemon_index: current_active_index,
                                    old_status: PokemonStatus::SLEEP,
                                    new_status: PokemonStatus::NONE,
                                }));
                            incoming_instructions.instruction_list.push(
                                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                    side_ref: *attacking_side_ref,
                                    pokemon_index: current_active_index,
                                    new_turns: 0,
                                    previous_turns: current_sleep_turns,
                                }),
                            );
                            attacker_active.status = PokemonStatus::NONE;
                            attacker_active.sleep_turns = 0;
                        }
                        do_nothing_instructions.update_percentage(do_nothing_percentage);
                        incoming_instructions.update_percentage(1.0 - do_nothing_percentage);
                        final_instructions.push(do_nothing_instructions);
                    }
                }
                // Pokemon is asleep because of Rest, and will wake up this turn
                1 => {
                    attacker_active.status = PokemonStatus::NONE;
                    attacker_active.rest_turns -= 1;
                    incoming_instructions
                        .instruction_list
                        .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: *attacking_side_ref,
                            pokemon_index: current_active_index,
                            old_status: PokemonStatus::SLEEP,
                            new_status: PokemonStatus::NONE,
                        }));
                    incoming_instructions
                        .instruction_list
                        .push(Instruction::DecrementRestTurns(
                            DecrementRestTurnsInstruction {
                                side_ref: *attacking_side_ref,
                            },
                        ));
                }
                // Pokemon is asleep because of Rest, and will stay asleep this turn
                2 | 3 => {
                    attacker_active.rest_turns -= 1;
                    incoming_instructions
                        .instruction_list
                        .push(Instruction::DecrementRestTurns(
                            DecrementRestTurnsInstruction {
                                side_ref: *attacking_side_ref,
                            },
                        ));
                }
                _ => panic!("Invalid rest_turns value: {}", attacker_active.rest_turns),
            }
        }
        _ => {}
    }

    if attacking_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::CONFUSION)
    {
        let mut hit_yourself_instruction = incoming_instructions.clone();
        hit_yourself_instruction.update_percentage(HIT_SELF_IN_CONFUSION_CHANCE);

        let attacking_stat = attacking_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
        let defending_stat = attacking_side.calculate_boosted_stat(PokemonBoostableStat::Defense);

        let attacker_active = attacking_side.get_active();
        let mut damage_dealt = 2.0 * attacker_active.level as f32;
        damage_dealt = damage_dealt.floor() / 5.0;
        damage_dealt = damage_dealt.floor() + 2.0;
        damage_dealt = damage_dealt.floor() * 40.0; // 40 is the base power of confusion damage
        damage_dealt = damage_dealt * attacking_stat as f32 / defending_stat as f32;
        damage_dealt = damage_dealt.floor() / 50.0;
        damage_dealt = damage_dealt.floor() + 2.0;
        if attacker_active.status == PokemonStatus::BURN {
            damage_dealt /= 2.0;
        }

        let damage_dealt = cmp::min(damage_dealt as i16, attacker_active.hp);
        let damage_instruction = Instruction::Damage(DamageInstruction {
            side_ref: *attacking_side_ref,
            damage_amount: damage_dealt,
        });
        hit_yourself_instruction
            .instruction_list
            .push(damage_instruction);

        final_instructions.push(hit_yourself_instruction);

        incoming_instructions.update_percentage(1.0 - HIT_SELF_IN_CONFUSION_CHANCE);
    }
}

pub fn generate_instructions_from_move(
    state: &mut State,
    choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side: SideReference,
    mut incoming_instructions: StateInstructions,
    mut final_instructions: &mut Vec<StateInstructions>,
    branch_on_damage: bool,
) {
    if state.use_damage_dealt {
        reset_damage_dealt(
            state.get_side(&attacking_side),
            &attacking_side,
            &mut incoming_instructions,
        );
    }

    if choice.category == MoveCategory::Switch {
        generate_instructions_from_switch(
            state,
            choice.switch_id,
            attacking_side,
            &mut incoming_instructions,
        );
        final_instructions.push(incoming_instructions);
        return;
    }

    let attacker_side = state.get_side(&attacking_side);

    if choice.move_id == Choices::NONE {
        if attacker_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::MUSTRECHARGE)
        {
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: attacking_side,
                        volatile_status: PokemonVolatileStatus::MUSTRECHARGE,
                    },
                ));
        }
        final_instructions.push(incoming_instructions);
        return;
    }

    if attacker_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::TRUANT)
    {
        incoming_instructions
            .instruction_list
            .push(Instruction::RemoveVolatileStatus(
                RemoveVolatileStatusInstruction {
                    side_ref: attacking_side,
                    volatile_status: PokemonVolatileStatus::TRUANT,
                },
            ));
        final_instructions.push(incoming_instructions);
        return;
    }

    // TODO: test first-turn dragontail missing - it should not trigger this early return
    if !choice.first_move && defender_choice.flags.drag {
        final_instructions.push(incoming_instructions);
        return;
    }

    state.apply_instructions(&incoming_instructions.instruction_list);

    let side = state.get_side_immutable(&attacking_side);
    if side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::ENCORE)
    {
        match side.last_used_move {
            LastUsedMove::Move(last_used_move) => {
                if choice.move_index != last_used_move {
                    *choice = MOVES
                        .get(&side.get_active_immutable().moves[&last_used_move].id)
                        .unwrap()
                        .clone();
                    choice.move_index = last_used_move;
                }
            }
            _ => panic!("Encore should not be active when last used move is not a move"),
        }
    }

    if !choice.first_move
        && state
            .get_side(&attacking_side.get_other_side())
            .force_switch
    {
        state
            .get_side(&attacking_side)
            .switch_out_move_second_saved_move = choice.move_id;
        state.reverse_instructions(&incoming_instructions.instruction_list);
        final_instructions.push(incoming_instructions);
        return;
    }

    if state
        .get_side_immutable(&attacking_side)
        .get_active_immutable()
        .hp
        == 0
    {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        final_instructions.push(incoming_instructions);
        return;
    }

    // If the move is a charge move, remove the volatile status if damage was done
    if choice.flags.charge {
        let side = state.get_side(&attacking_side);
        let volatile_status = charge_choice_to_volatile(&choice.move_id);
        if side.volatile_statuses.contains(&volatile_status) {
            choice.flags.charge = false;
            let instruction = Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: attacking_side,
                volatile_status: volatile_status,
            });
            incoming_instructions.instruction_list.push(instruction);
            side.volatile_statuses.remove(&volatile_status);
        }
    }

    if cannot_use_move(state, &choice, &attacking_side) {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        final_instructions.push(incoming_instructions);
        return;
    }

    before_move(
        state,
        choice,
        defender_choice,
        &attacking_side,
        &mut incoming_instructions,
    );
    if incoming_instructions.percentage == 0.0 {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        return;
    }

    // most of the time pp decrement doesn't matter and just adds another instruction
    // so we only decrement pp if the move is at 10 or less pp since that is when it starts
    // to matter
    let (attacker_side, defender_side) = state.get_both_sides(&attacking_side);
    let active = attacker_side.get_active();
    if active.moves[&choice.move_index].pp < 10 {
        let pp_decrement_amount = if choice.target == MoveTarget::Opponent
            && defender_side.get_active_immutable().ability == Abilities::PRESSURE
        {
            2
        } else {
            1
        };
        incoming_instructions
            .instruction_list
            .push(Instruction::DecrementPP(DecrementPPInstruction {
                side_ref: attacking_side,
                move_index: choice.move_index,
                amount: pp_decrement_amount,
            }));
        active.moves[&choice.move_index].pp -= pp_decrement_amount;
    }

    if state.use_last_used_move {
        set_last_used_move_as_move(
            state.get_side(&attacking_side),
            choice.move_index,
            attacking_side,
            &mut incoming_instructions,
        );
    }

    if !choice.sleep_talk_move {
        generate_instructions_from_existing_status_conditions(
            state,
            &attacking_side,
            &choice,
            &mut incoming_instructions,
            &mut final_instructions,
        );
    }
    let attacker = state
        .get_side_immutable(&attacking_side)
        .get_active_immutable();
    if choice.move_id == Choices::SLEEPTALK && attacker.status == PokemonStatus::SLEEP {
        let new_choices = attacker.get_sleep_talk_choices();
        state.reverse_instructions(&incoming_instructions.instruction_list);
        let num_choices = new_choices.len() as f32;
        for mut new_choice in new_choices {
            new_choice.sleep_talk_move = true;
            let mut sleep_talk_instructions = incoming_instructions.clone();
            sleep_talk_instructions.update_percentage(1.0 / num_choices);
            generate_instructions_from_move(
                state,
                &mut new_choice,
                defender_choice,
                attacking_side,
                sleep_talk_instructions,
                &mut final_instructions,
                false,
            );
        }
        return;
    } else if attacker.status == PokemonStatus::SLEEP && !choice.sleep_talk_move {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        if incoming_instructions.percentage > 0.0 {
            final_instructions.push(incoming_instructions);
        }
        return;
    }

    if move_has_no_effect(state, &choice, &attacking_side) {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        final_instructions.push(incoming_instructions);
        return;
    }
    choice_special_effect(state, &choice, &attacking_side, &mut incoming_instructions);
    let damage = calculate_damage(state, &attacking_side, &choice, DamageRolls::Max);
    check_move_hit_or_miss(
        state,
        &choice,
        &attacking_side,
        damage,
        &mut incoming_instructions,
        &mut final_instructions,
    );

    if incoming_instructions.percentage == 0.0 {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        return;
    }

    // start multi-hit
    let hit_count;
    match choice.multi_hit() {
        MultiHitMove::None => {
            hit_count = 1;
        }
        MultiHitMove::DoubleHit => {
            hit_count = 2;
        }
        MultiHitMove::TripleHit => {
            hit_count = 3;
        }
        MultiHitMove::TwoToFiveHits => {
            hit_count =
                if state.get_side(&attacking_side).get_active().ability == Abilities::SKILLLINK {
                    5
                } else if state.get_side(&attacking_side).get_active().item == Items::LOADEDDICE {
                    4
                } else {
                    3 // too lazy to implement branching here. Average is 3.2 so this is a fine approximation
                };
        }
        MultiHitMove::PopulationBomb => {
            // population bomb checks accuracy each time but lets approximate
            hit_count = if state.get_side(&attacking_side).get_active().item == Items::WIDELENS {
                9
            } else {
                6
            };
        }
        MultiHitMove::TripleAxel => {
            // triple axel checks accuracy each time but until multi-accuracy is implemented this
            // is the best we can do
            hit_count = 3
        }
    }

    let (_attacker_side, defender_side) = state.get_both_sides(&attacking_side);
    let defender_active = defender_side.get_active();
    let mut does_damage = false;
    let (mut branch_damage, mut regular_damage) = (0, 0);
    let mut branch_instructions: Option<StateInstructions> = None;
    if let Some((max_damage_dealt, max_crit_damage)) = damage {
        does_damage = true;
        let avg_damage_dealt = (max_damage_dealt as f32 * 0.925) as i16;
        let min_damage_dealt = (max_damage_dealt as f32 * 0.85) as i16;
        if branch_on_damage
            && max_damage_dealt >= defender_active.hp
            && min_damage_dealt < defender_active.hp
        {
            let (average_non_kill_damage, num_kill_rolls) =
                compare_health_with_damage_multiples(max_damage_dealt, defender_active.hp);

            let crit_rate = if defender_active.ability == Abilities::BATTLEARMOR
                || defender_active.ability == Abilities::SHELLARMOR
            {
                0.0
            } else if choice.move_id.guaranteed_crit() {
                1.0
            } else if choice.move_id.increased_crit_ratio() {
                1.0 / 8.0
            } else {
                BASE_CRIT_CHANCE
            };

            // the chance of a branch is the chance of the roll killing + the chance of a crit
            let branch_chance = ((1.0 - crit_rate) * (num_kill_rolls as f32 / 16.0)) + crit_rate;

            let mut branch_ins = incoming_instructions.clone();
            branch_ins.update_percentage(branch_chance);
            branch_instructions = Some(branch_ins);
            branch_damage = defender_active.hp;

            incoming_instructions.update_percentage(1.0 - branch_chance);
            regular_damage = average_non_kill_damage;
        } else if branch_on_damage && max_damage_dealt < defender_active.hp {
            let crit_rate = if defender_active.ability == Abilities::BATTLEARMOR
                || defender_active.ability == Abilities::SHELLARMOR
            {
                0.0
            } else if choice.move_id.guaranteed_crit() {
                1.0
            } else if choice.move_id.increased_crit_ratio() {
                1.0 / 8.0
            } else {
                BASE_CRIT_CHANCE
            };
            let mut branch_ins = incoming_instructions.clone();
            branch_ins.update_percentage(crit_rate);
            branch_instructions = Some(branch_ins);
            branch_damage = (max_crit_damage as f32 * 0.925) as i16;
            incoming_instructions.update_percentage(1.0 - crit_rate);
            regular_damage = (max_damage_dealt as f32 * 0.925) as i16;
        } else {
            regular_damage = avg_damage_dealt;
        }
    }

    if incoming_instructions.percentage != 0.0 {
        run_move(
            state,
            attacking_side,
            incoming_instructions,
            hit_count,
            does_damage,
            regular_damage,
            choice,
            defender_choice,
            &mut final_instructions,
        );
    } else {
        state.reverse_instructions(&incoming_instructions.instruction_list);
    }

    // A branch representing either a roll that kills the opponent or a crit
    if let Some(branch_ins) = branch_instructions {
        if branch_ins.percentage != 0.0 {
            state.apply_instructions(&branch_ins.instruction_list);
            run_move(
                state,
                attacking_side,
                branch_ins,
                hit_count,
                does_damage,
                branch_damage,
                choice,
                defender_choice,
                &mut final_instructions,
            );
        }
    }

    combine_duplicate_instructions(&mut final_instructions);
    return;
}

fn combine_duplicate_instructions(list_of_instructions: &mut Vec<StateInstructions>) {
    for i in 0..list_of_instructions.len() {
        let mut j = i + 1;
        while j < list_of_instructions.len() {
            if list_of_instructions[i].instruction_list == list_of_instructions[j].instruction_list
            {
                list_of_instructions[i].percentage += list_of_instructions[j].percentage;
                list_of_instructions.remove(j);
            } else {
                j += 1;
            }
        }
    }
}

fn get_effective_speed(state: &State, side_reference: &SideReference) -> i16 {
    let side = state.get_side_immutable(side_reference);
    let active_pkmn = side.get_active_immutable();

    let mut boosted_speed = side.calculate_boosted_stat(PokemonBoostableStat::Speed) as f32;

    match state.weather.weather_type {
        Weather::SUN | Weather::HARSHSUN if active_pkmn.ability == Abilities::CHLOROPHYLL => {
            boosted_speed *= 2.0
        }
        Weather::RAIN | Weather::HEAVYRAIN if active_pkmn.ability == Abilities::SWIFTSWIM => {
            boosted_speed *= 2.0
        }
        Weather::SAND if active_pkmn.ability == Abilities::SANDRUSH => boosted_speed *= 2.0,
        Weather::HAIL if active_pkmn.ability == Abilities::SLUSHRUSH => boosted_speed *= 2.0,
        _ => {}
    }

    match active_pkmn.ability {
        Abilities::SURGESURFER if state.terrain.terrain_type == Terrain::ELECTRICTERRAIN => {
            boosted_speed *= 2.0
        }
        Abilities::UNBURDEN
            if side
                .volatile_statuses
                .contains(&PokemonVolatileStatus::UNBURDEN) =>
        {
            boosted_speed *= 2.0
        }
        Abilities::QUICKFEET if active_pkmn.status != PokemonStatus::NONE => boosted_speed *= 1.5,
        _ => {}
    }

    if side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::SLOWSTART)
    {
        boosted_speed *= 0.5;
    }

    if side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::PROTOSYNTHESISSPE)
        || side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::QUARKDRIVESPE)
    {
        boosted_speed *= 1.5;
    }

    if side.side_conditions.tailwind > 0 {
        boosted_speed *= 2.0
    }

    match active_pkmn.item {
        Items::IRONBALL => boosted_speed *= 0.5,
        Items::CHOICESCARF => boosted_speed *= 1.5,
        _ => {}
    }

    #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5", feature = "gen6"))]
    if active_pkmn.status == PokemonStatus::PARALYZE && active_pkmn.ability != Abilities::QUICKFEET
    {
        boosted_speed *= 0.25;
    }

    #[cfg(any(feature = "gen7", feature = "gen8", feature = "gen9"))]
    if active_pkmn.status == PokemonStatus::PARALYZE && active_pkmn.ability != Abilities::QUICKFEET
    {
        boosted_speed *= 0.50;
    }

    boosted_speed as i16
}

fn modify_choice_priority(state: &State, side_reference: &SideReference, choice: &mut Choice) {
    let side = state.get_side_immutable(side_reference);
    let active_pkmn = side.get_active_immutable();

    match active_pkmn.ability {
        Abilities::PRANKSTER if choice.category == MoveCategory::Status => choice.priority += 1,
        Abilities::GALEWINGS
            if choice.move_type == PokemonType::FLYING && active_pkmn.hp == active_pkmn.maxhp =>
        {
            choice.priority += 1
        }
        Abilities::TRIAGE if choice.flags.heal => choice.priority += 3,
        _ => {}
    }
}

fn side_one_moves_first(state: &State, side_one_choice: &Choice, side_two_choice: &Choice) -> bool {
    let side_one_effective_speed = get_effective_speed(&state, &SideReference::SideOne);
    let side_two_effective_speed = get_effective_speed(&state, &SideReference::SideTwo);

    if side_one_choice.category == MoveCategory::Switch
        && side_two_choice.category == MoveCategory::Switch
    {
        return side_one_effective_speed > side_two_effective_speed;
    } else if side_one_choice.category == MoveCategory::Switch {
        return side_two_choice.move_id != Choices::PURSUIT;
    } else if side_two_choice.category == MoveCategory::Switch {
        return side_one_choice.move_id == Choices::PURSUIT;
    }

    let side_one_active = state.side_one.get_active_immutable();
    let side_two_active = state.side_two.get_active_immutable();
    if side_one_choice.priority == side_two_choice.priority {
        if side_one_active.item == Items::CUSTAPBERRY
            && side_one_active.hp < side_one_active.maxhp / 4
        {
            return true;
        } else if side_two_active.item == Items::CUSTAPBERRY
            && side_two_active.hp < side_two_active.maxhp / 4
        {
            return false;
        }
        match state.trick_room.active {
            true => return side_one_effective_speed < side_two_effective_speed,
            false => side_one_effective_speed > side_two_effective_speed,
        }
    } else {
        side_one_choice.priority > side_two_choice.priority
    }
}

fn add_end_of_turn_instructions(
    state: &mut State,
    mut incoming_instructions: &mut StateInstructions,
    _side_one_choice: &Choice,
    _side_two_choice: &Choice,
    first_move_side: &SideReference,
) {
    /*
    Methodology:
        This function is deterministic and will not branch.
        It will apply instructions to the state as it goes, and then reverse them at the end.
    */

    state.apply_instructions(&incoming_instructions.instruction_list);
    if state.side_one.force_switch || state.side_two.force_switch {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        return;
    }

    let sides = [first_move_side, &first_move_side.get_other_side()];

    // Weather decrement / dissipation
    if state.weather.turns_remaining > 0 && state.weather.weather_type != Weather::NONE {
        let weather_dissipate_instruction = Instruction::DecrementWeatherTurnsRemaining;
        incoming_instructions
            .instruction_list
            .push(weather_dissipate_instruction);
        state.weather.turns_remaining -= 1;
        if state.weather.turns_remaining == 0 {
            let weather_end_instruction = Instruction::ChangeWeather(ChangeWeather {
                new_weather: Weather::NONE,
                new_weather_turns_remaining: 0,
                previous_weather: state.weather.weather_type,
                previous_weather_turns_remaining: 0,
            });
            incoming_instructions
                .instruction_list
                .push(weather_end_instruction);
            state.weather.weather_type = Weather::NONE;
        }
    }

    // Trick Room decrement / dissipation
    if state.trick_room.turns_remaining > 0 && state.trick_room.active {
        incoming_instructions
            .instruction_list
            .push(Instruction::DecrementTrickRoomTurnsRemaining);
        state.trick_room.turns_remaining -= 1;
        if state.trick_room.turns_remaining == 0 {
            incoming_instructions
                .instruction_list
                .push(Instruction::ToggleTrickRoom(ToggleTrickRoomInstruction {
                    currently_active: true,
                    new_trickroom_turns_remaining: 0,
                    previous_trickroom_turns_remaining: 0,
                }));
            state.trick_room.active = false;
        }
    }

    // Terrain decrement / dissipation
    if state.terrain.turns_remaining > 0 && state.terrain.terrain_type != Terrain::NONE {
        let terrain_dissipate_instruction = Instruction::DecrementTerrainTurnsRemaining;
        incoming_instructions
            .instruction_list
            .push(terrain_dissipate_instruction);
        state.terrain.turns_remaining -= 1;
        if state.terrain.turns_remaining == 0 {
            let terrain_end_instruction = Instruction::ChangeTerrain(ChangeTerrain {
                new_terrain: Terrain::NONE,
                new_terrain_turns_remaining: 0,
                previous_terrain: state.terrain.terrain_type,
                previous_terrain_turns_remaining: 0,
            });
            incoming_instructions
                .instruction_list
                .push(terrain_end_instruction);
            state.terrain.terrain_type = Terrain::NONE;
        }
    }

    // Weather Damage
    for side_ref in sides {
        if state.weather_is_active(&Weather::HAIL) {
            let active_pkmn = state.get_side(side_ref).get_active();
            if active_pkmn.hp == 0
                || active_pkmn.ability == Abilities::MAGICGUARD
                || active_pkmn.ability == Abilities::OVERCOAT
                || active_pkmn.ability == Abilities::ICEBODY
                || active_pkmn.has_type(&PokemonType::ICE)
            {
                continue;
            }

            let damage_amount =
                cmp::min((active_pkmn.maxhp as f32 * 0.0625) as i16, active_pkmn.hp);
            let hail_damage_instruction = Instruction::Damage(DamageInstruction {
                side_ref: *side_ref,
                damage_amount: damage_amount,
            });

            active_pkmn.hp -= damage_amount;
            incoming_instructions
                .instruction_list
                .push(hail_damage_instruction);
        } else if state.weather_is_active(&Weather::SAND) {
            let active_pkmn = state.get_side(side_ref).get_active();
            if active_pkmn.hp == 0
                || active_pkmn.ability == Abilities::MAGICGUARD
                || active_pkmn.ability == Abilities::OVERCOAT
                || active_pkmn.has_type(&PokemonType::GROUND)
                || active_pkmn.has_type(&PokemonType::STEEL)
                || active_pkmn.has_type(&PokemonType::ROCK)
            {
                continue;
            }
            let damage_amount =
                cmp::min((active_pkmn.maxhp as f32 * 0.0625) as i16, active_pkmn.hp);
            let sand_damage_instruction = Instruction::Damage(DamageInstruction {
                side_ref: *side_ref,
                damage_amount: damage_amount,
            });
            active_pkmn.hp -= damage_amount;
            incoming_instructions
                .instruction_list
                .push(sand_damage_instruction);
        }
    }

    // future sight
    for side_ref in sides {
        let (attacking_side, defending_side) = state.get_both_sides(side_ref);
        if attacking_side.future_sight.0 > 0 {
            let decrement_future_sight_instruction =
                Instruction::DecrementFutureSight(DecrementFutureSightInstruction {
                    side_ref: *side_ref,
                });
            if attacking_side.future_sight.0 == 1 {
                let mut damage = calculate_futuresight_damage(
                    &attacking_side,
                    &defending_side,
                    &attacking_side.future_sight.1,
                );
                let defender = defending_side.get_active();
                damage = cmp::min(damage, defender.hp);
                let future_sight_damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: side_ref.get_other_side(),
                    damage_amount: damage,
                });
                incoming_instructions
                    .instruction_list
                    .push(future_sight_damage_instruction);
                defender.hp -= damage;
            }
            attacking_side.future_sight.0 -= 1;
            incoming_instructions
                .instruction_list
                .push(decrement_future_sight_instruction);
        }
    }

    // wish
    for side_ref in sides {
        let side = state.get_side(side_ref);
        let side_wish = side.wish;
        let active_pkmn = side.get_active();

        if side_wish.0 > 0 {
            let decrement_wish_instruction = Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: *side_ref,
            });
            if side_wish.0 == 1 && 0 < active_pkmn.hp && active_pkmn.hp < active_pkmn.maxhp {
                #[cfg(not(feature = "gen4"))]
                let heal_amount = cmp::min(active_pkmn.maxhp - active_pkmn.hp, side_wish.1);

                #[cfg(feature = "gen4")]
                let heal_amount =
                    cmp::min(active_pkmn.maxhp - active_pkmn.hp, active_pkmn.maxhp / 2);

                let wish_heal_instruction = Instruction::Heal(HealInstruction {
                    side_ref: *side_ref,
                    heal_amount: heal_amount,
                });
                incoming_instructions
                    .instruction_list
                    .push(wish_heal_instruction);
                active_pkmn.hp += heal_amount;
            }
            side.wish.0 -= 1;
            incoming_instructions
                .instruction_list
                .push(decrement_wish_instruction);
        }
    }

    // status damage
    for side_ref in sides {
        let side = state.get_side(side_ref);
        let toxic_count = side.side_conditions.toxic_count as f32;
        let active_pkmn = side.get_active();
        if active_pkmn.hp == 0 || active_pkmn.ability == Abilities::MAGICGUARD {
            continue;
        }

        match active_pkmn.status {
            PokemonStatus::BURN => {
                #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5", feature = "gen6"))]
                let mut damage_factor = 0.125;

                #[cfg(any(feature = "gen7", feature = "gen8", feature = "gen9",))]
                let mut damage_factor = 0.0625;

                if active_pkmn.ability == Abilities::HEATPROOF {
                    damage_factor /= 2.0;
                }
                let damage_amount = cmp::max(
                    cmp::min(
                        (active_pkmn.maxhp as f32 * damage_factor) as i16,
                        active_pkmn.hp,
                    ),
                    1,
                );
                let burn_damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: *side_ref,
                    damage_amount: damage_amount,
                });
                active_pkmn.hp -= damage_amount;
                incoming_instructions
                    .instruction_list
                    .push(burn_damage_instruction);
            }
            PokemonStatus::POISON if active_pkmn.ability != Abilities::POISONHEAL => {
                let damage_amount = cmp::max(
                    1,
                    cmp::min((active_pkmn.maxhp as f32 * 0.125) as i16, active_pkmn.hp),
                );

                let poison_damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: *side_ref,
                    damage_amount: damage_amount,
                });
                active_pkmn.hp -= damage_amount;
                incoming_instructions
                    .instruction_list
                    .push(poison_damage_instruction);
            }
            PokemonStatus::TOXIC if active_pkmn.ability != Abilities::POISONHEAL => {
                let toxic_multiplier = (1.0 / 16.0) * toxic_count + (1.0 / 16.0);
                let damage_amount = cmp::max(
                    cmp::min(
                        (active_pkmn.maxhp as f32 * toxic_multiplier) as i16,
                        active_pkmn.hp,
                    ),
                    1,
                );
                let toxic_damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: *side_ref,
                    damage_amount,
                });
                let toxic_counter_increment_instruction =
                    Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                        side_ref: *side_ref,
                        side_condition: PokemonSideCondition::ToxicCount,
                        amount: 1,
                    });
                active_pkmn.hp -= damage_amount;
                side.side_conditions.toxic_count += 1;
                incoming_instructions
                    .instruction_list
                    .push(toxic_damage_instruction);
                incoming_instructions
                    .instruction_list
                    .push(toxic_counter_increment_instruction);
            }
            _ => {}
        }
    }

    // ability/item end-of-turn effects
    for side_ref in sides {
        let side = state.get_side(side_ref);
        let active_pkmn = side.get_active();
        if active_pkmn.hp == 0 {
            continue;
        }

        item_end_of_turn(state, side_ref, &mut incoming_instructions);
        ability_end_of_turn(state, side_ref, &mut incoming_instructions);
    }

    // leechseed sap
    for side_ref in sides {
        let (leechseed_side, other_side) = state.get_both_sides(side_ref);
        if leechseed_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::LEECHSEED)
        {
            let active_pkmn = leechseed_side.get_active();
            let other_active_pkmn = other_side.get_active();
            if active_pkmn.hp == 0
                || other_active_pkmn.hp == 0
                || active_pkmn.ability == Abilities::MAGICGUARD
            {
                continue;
            }

            let health_sapped = cmp::min((active_pkmn.maxhp as f32 * 0.125) as i16, active_pkmn.hp);
            let damage_ins = Instruction::Damage(DamageInstruction {
                side_ref: *side_ref,
                damage_amount: health_sapped,
            });
            active_pkmn.hp -= health_sapped;
            incoming_instructions.instruction_list.push(damage_ins);

            let health_recovered = cmp::min(
                health_sapped,
                other_active_pkmn.maxhp - other_active_pkmn.hp,
            );
            if health_recovered > 0 {
                let heal_ins = Instruction::Heal(HealInstruction {
                    side_ref: side_ref.get_other_side(),
                    heal_amount: health_recovered,
                });
                other_active_pkmn.hp += health_recovered;
                incoming_instructions.instruction_list.push(heal_ins);
            }
        }
    }

    // volatile statuses
    for side_ref in sides {
        let side = state.get_side(side_ref);
        if side.get_active().hp == 0 {
            continue;
        }

        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::YAWNSLEEPTHISTURN)
        {
            side.volatile_statuses
                .remove(&PokemonVolatileStatus::YAWNSLEEPTHISTURN);
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::YAWNSLEEPTHISTURN,
                    },
                ));

            let active = side.get_active();
            if active.status == PokemonStatus::NONE {
                active.status = PokemonStatus::SLEEP;
                incoming_instructions
                    .instruction_list
                    .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: *side_ref,
                        pokemon_index: side.active_index,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::SLEEP,
                    }));
            }
        }

        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::YAWN)
        {
            side.volatile_statuses.remove(&PokemonVolatileStatus::YAWN);
            side.volatile_statuses
                .insert(PokemonVolatileStatus::YAWNSLEEPTHISTURN);
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::YAWN,
                    },
                ));
            incoming_instructions
                .instruction_list
                .push(Instruction::ApplyVolatileStatus(
                    ApplyVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::YAWNSLEEPTHISTURN,
                    },
                ));
        }

        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::PERISH1)
        {
            let active_pkmn = side.get_active();
            incoming_instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: *side_ref,
                    damage_amount: active_pkmn.hp,
                }));
            active_pkmn.hp = 0;
        }

        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::PERISH2)
        {
            side.volatile_statuses
                .remove(&PokemonVolatileStatus::PERISH2);
            side.volatile_statuses
                .insert(PokemonVolatileStatus::PERISH1);
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::PERISH2,
                    },
                ));
            incoming_instructions
                .instruction_list
                .push(Instruction::ApplyVolatileStatus(
                    ApplyVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::PERISH1,
                    },
                ));
        }
        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::PERISH3)
        {
            side.volatile_statuses
                .remove(&PokemonVolatileStatus::PERISH3);
            side.volatile_statuses
                .insert(PokemonVolatileStatus::PERISH2);
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::PERISH3,
                    },
                ));
            incoming_instructions
                .instruction_list
                .push(Instruction::ApplyVolatileStatus(
                    ApplyVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::PERISH2,
                    },
                ));
        }
        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::PERISH4)
        {
            side.volatile_statuses
                .remove(&PokemonVolatileStatus::PERISH4);
            side.volatile_statuses
                .insert(PokemonVolatileStatus::PERISH3);
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::PERISH4,
                    },
                ));
            incoming_instructions
                .instruction_list
                .push(Instruction::ApplyVolatileStatus(
                    ApplyVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::PERISH3,
                    },
                ));
        }

        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::FLINCH)
        {
            side.volatile_statuses
                .remove(&PokemonVolatileStatus::FLINCH);
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::FLINCH,
                    },
                ));
        }
        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::ROOST)
        {
            side.volatile_statuses.remove(&PokemonVolatileStatus::ROOST);
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::ROOST,
                    },
                ));
        }

        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::PARTIALLYTRAPPED)
        {
            let active_pkmn = side.get_active();

            #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5"))]
            let damage_amount = cmp::min((active_pkmn.maxhp as f32 / 16.0) as i16, active_pkmn.hp);

            #[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
            let damage_amount = cmp::min((active_pkmn.maxhp as f32 / 8.0) as i16, active_pkmn.hp);

            incoming_instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: *side_ref,
                    damage_amount,
                }));
            active_pkmn.hp -= damage_amount;
        }
        if side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SALTCURE)
        {
            let active_pkmn = side.get_active();
            let mut divisor = 8.0;
            if active_pkmn.has_type(&PokemonType::WATER)
                || active_pkmn.has_type(&PokemonType::STEEL)
            {
                divisor = 4.0;
            }
            let damage_amount =
                cmp::min((active_pkmn.maxhp as f32 / divisor) as i16, active_pkmn.hp);
            incoming_instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: *side_ref,
                    damage_amount: damage_amount,
                }));
            active_pkmn.hp -= damage_amount;
        }

        let possible_statuses = [
            PokemonVolatileStatus::PROTECT,
            PokemonVolatileStatus::BANEFULBUNKER,
            PokemonVolatileStatus::BURNINGBULWARK,
            PokemonVolatileStatus::SPIKYSHIELD,
            PokemonVolatileStatus::SILKTRAP,
        ];

        let mut protect_vs = None;
        for status in &possible_statuses {
            if side.volatile_statuses.contains(status) {
                protect_vs = Some(*status);
                break;
            }
        }

        if let Some(protect_vs) = protect_vs {
            incoming_instructions
                .instruction_list
                .push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: protect_vs,
                    },
                ));
            side.volatile_statuses.remove(&protect_vs);
            incoming_instructions
                .instruction_list
                .push(Instruction::ChangeSideCondition(
                    ChangeSideConditionInstruction {
                        side_ref: *side_ref,
                        side_condition: PokemonSideCondition::Protect,
                        amount: 1,
                    },
                ));
            side.side_conditions.protect += 1;
        } else if side.side_conditions.protect > 0 {
            incoming_instructions
                .instruction_list
                .push(Instruction::ChangeSideCondition(
                    ChangeSideConditionInstruction {
                        side_ref: *side_ref,
                        side_condition: PokemonSideCondition::Protect,
                        amount: -1 * side.side_conditions.protect,
                    },
                ));
            side.side_conditions.protect -= side.side_conditions.protect;
        }
    } // end volatile statuses

    state.reverse_instructions(&incoming_instructions.instruction_list);
}

fn end_of_turn_triggered(side_one_move: &MoveChoice, side_two_move: &MoveChoice) -> bool {
    !(matches!(side_one_move, &MoveChoice::Switch(_)) && side_two_move == &MoveChoice::None)
        && !(side_one_move == &MoveChoice::None && matches!(side_two_move, &MoveChoice::Switch(_)))
}

fn run_move(
    state: &mut State,
    attacking_side: SideReference,
    mut instructions: StateInstructions,
    hit_count: i8,
    does_damage: bool,
    damage_amount: i16,
    choice: &Choice,
    defender_choice: &Choice,
    final_instructions: &mut Vec<StateInstructions>,
) {
    let mut hit_sub = false;
    for _ in 0..hit_count {
        if does_damage {
            hit_sub = generate_instructions_from_damage(
                state,
                &choice,
                damage_amount,
                &attacking_side,
                &mut instructions,
            );
        }
        if let Some(side_condition) = &choice.side_condition {
            generate_instructions_from_side_conditions(
                state,
                side_condition,
                &attacking_side,
                &mut instructions,
            );
        }
        choice_hazard_clear(state, &choice, &attacking_side, &mut instructions);
        if let Some(volatile_status) = &choice.volatile_status {
            get_instructions_from_volatile_statuses(
                state,
                &choice,
                volatile_status,
                &attacking_side,
                &mut instructions,
            );
        }
        if let Some(status) = &choice.status {
            get_instructions_from_status_effects(
                state,
                status,
                &attacking_side,
                &mut instructions,
                hit_sub,
            );
        }
        if let Some(heal) = &choice.heal {
            get_instructions_from_heal(state, heal, &attacking_side, &mut instructions);
        }
    } // end multi-hit
      // this is wrong, but I am deciding it is good enough for this engine (for now)
      // each multi-hit move should trigger a chance for a secondary effect,
      // but the way this engine was structured makes it difficult to implement
      // without some performance hits.

    if let Some(boost) = &choice.boost {
        get_instructions_from_boosts(state, boost, &attacking_side, &mut instructions);
    }

    if choice.flags.drag
        && state
            .get_side_immutable(&attacking_side.get_other_side())
            .get_active_immutable()
            .ability
            != Abilities::GUARDDOG
    {
        get_instructions_from_drag(state, &attacking_side, instructions, final_instructions);
        return;
    }

    // Only entered if the move causes a switch-out
    // U-turn, Volt Switch, Baton Pass, etc.
    // This deals with a bunch of flags that are required for the next turn to run properly
    if choice.flags.pivot {
        match attacking_side {
            SideReference::SideOne => {
                if state.side_one.visible_alive_pkmn() > 1 {
                    if choice.move_id == Choices::BATONPASS {
                        state.side_one.baton_passing = !state.side_one.baton_passing;
                        instructions
                            .instruction_list
                            .push(Instruction::ToggleBatonPassing(
                                ToggleBatonPassingInstruction {
                                    side_ref: SideReference::SideOne,
                                },
                            ));
                    }
                    state.side_one.force_switch = !state.side_one.force_switch;
                    instructions
                        .instruction_list
                        .push(Instruction::ToggleSideOneForceSwitch);

                    if choice.first_move {
                        instructions.instruction_list.push(
                            Instruction::SetSideTwoMoveSecondSwitchOutMove(
                                SetSecondMoveSwitchOutMoveInstruction {
                                    new_choice: defender_choice.move_id,
                                    previous_choice: state
                                        .side_two
                                        .switch_out_move_second_saved_move,
                                },
                            ),
                        );
                        state.side_two.switch_out_move_second_saved_move = defender_choice.move_id;
                    } else {
                        instructions.instruction_list.push(
                            Instruction::SetSideTwoMoveSecondSwitchOutMove(
                                SetSecondMoveSwitchOutMoveInstruction {
                                    new_choice: Choices::NONE,
                                    previous_choice: state
                                        .side_two
                                        .switch_out_move_second_saved_move,
                                },
                            ),
                        );
                        state.side_two.switch_out_move_second_saved_move = defender_choice.move_id;
                    }
                }
            }
            SideReference::SideTwo => {
                if state.side_two.visible_alive_pkmn() > 1 {
                    if choice.move_id == Choices::BATONPASS {
                        state.side_two.baton_passing = !state.side_two.baton_passing;
                        instructions
                            .instruction_list
                            .push(Instruction::ToggleBatonPassing(
                                ToggleBatonPassingInstruction {
                                    side_ref: SideReference::SideTwo,
                                },
                            ));
                    }
                    state.side_two.force_switch = !state.side_two.force_switch;
                    instructions
                        .instruction_list
                        .push(Instruction::ToggleSideTwoForceSwitch);

                    if choice.first_move {
                        instructions.instruction_list.push(
                            Instruction::SetSideOneMoveSecondSwitchOutMove(
                                SetSecondMoveSwitchOutMoveInstruction {
                                    new_choice: defender_choice.move_id,
                                    previous_choice: state
                                        .side_one
                                        .switch_out_move_second_saved_move,
                                },
                            ),
                        );
                        state.side_one.switch_out_move_second_saved_move = defender_choice.move_id;
                    } else {
                        instructions.instruction_list.push(
                            Instruction::SetSideOneMoveSecondSwitchOutMove(
                                SetSecondMoveSwitchOutMoveInstruction {
                                    new_choice: Choices::NONE,
                                    previous_choice: state
                                        .side_one
                                        .switch_out_move_second_saved_move,
                                },
                            ),
                        );
                        state.side_one.switch_out_move_second_saved_move = defender_choice.move_id;
                    }
                }
            }
        }
    }

    if let Some(secondaries_vec) = &choice.secondaries {
        state.reverse_instructions(&instructions.instruction_list);
        let instructions_vec_after_secondaries = get_instructions_from_secondaries(
            state,
            &choice,
            secondaries_vec,
            &attacking_side,
            instructions,
            hit_sub,
        );
        final_instructions.extend(instructions_vec_after_secondaries);
    } else {
        state.reverse_instructions(&instructions.instruction_list);
        final_instructions.push(instructions);
    }
}

pub fn generate_instructions_from_move_pair(
    state: &mut State,
    side_one_move: &MoveChoice,
    side_two_move: &MoveChoice,
    branch_on_damage: bool,
) -> Vec<StateInstructions> {
    /*
    - get Choice structs from moves
    - determine who moves first
    - initialize empty instructions
    - run move 1
    - run move 2
    - run end of turn instructions

    NOTE: End of turn instructions will need to generate the removing of certain volatile statuses, like flinched.
          This was done elsewhere in the other bot, but it should be here instead
    */

    let mut side_one_choice;
    let mut s1_tera = false;
    match side_one_move {
        MoveChoice::Switch(switch_id) => {
            side_one_choice = Choice::default();
            side_one_choice.switch_id = *switch_id;
            side_one_choice.category = MoveCategory::Switch;
        }
        MoveChoice::Move(move_index) => {
            side_one_choice = state.side_one.get_active().moves[move_index].choice.clone();
            side_one_choice.move_index = *move_index;
        }
        MoveChoice::MoveTera(move_index) => {
            side_one_choice = state.side_one.get_active().moves[move_index].choice.clone();
            side_one_choice.move_index = *move_index;
            s1_tera = true;
        }
        MoveChoice::None => {
            side_one_choice = Choice::default();
        }
    }

    let mut side_two_choice;
    let mut s2_tera = false;
    match side_two_move {
        MoveChoice::Switch(switch_id) => {
            side_two_choice = Choice::default();
            side_two_choice.switch_id = *switch_id;
            side_two_choice.category = MoveCategory::Switch;
        }
        MoveChoice::Move(move_index) => {
            side_two_choice = state.side_two.get_active().moves[move_index].choice.clone();
            side_two_choice.move_index = *move_index;
        }
        MoveChoice::MoveTera(move_index) => {
            side_two_choice = state.side_two.get_active().moves[move_index].choice.clone();
            side_two_choice.move_index = *move_index;
            s2_tera = true;
        }
        MoveChoice::None => {
            side_two_choice = Choice::default();
        }
    }

    let mut state_instructions_vec: Vec<StateInstructions> = Vec::with_capacity(16);
    let mut incoming_instructions: StateInstructions = StateInstructions::default();

    // Run terstallization type changes
    // Note: only create/apply instructions, don't apply changes
    // generate_instructions_from_move() assumes instructions have not been applied
    if s1_tera {
        incoming_instructions
            .instruction_list
            .push(Instruction::ToggleTerastallized(
                ToggleTerastallizedInstruction {
                    side_ref: SideReference::SideOne,
                },
            ));
    }
    if s2_tera {
        incoming_instructions
            .instruction_list
            .push(Instruction::ToggleTerastallized(
                ToggleTerastallizedInstruction {
                    side_ref: SideReference::SideTwo,
                },
            ));
    }

    let first_move_side;
    modify_choice_priority(&state, &SideReference::SideOne, &mut side_one_choice);
    modify_choice_priority(&state, &SideReference::SideTwo, &mut side_two_choice);
    if side_one_moves_first(&state, &side_one_choice, &side_two_choice) {
        first_move_side = SideReference::SideOne;
        generate_instructions_from_move(
            state,
            &mut side_one_choice,
            &side_two_choice,
            SideReference::SideOne,
            incoming_instructions,
            &mut state_instructions_vec,
            branch_on_damage,
        );
        side_two_choice.first_move = false;
        let mut i = 0;
        let vec_len = state_instructions_vec.len();
        while i < vec_len {
            let state_instruction = state_instructions_vec.remove(0);
            generate_instructions_from_move(
                state,
                &mut side_two_choice.clone(),
                &side_one_choice,
                SideReference::SideTwo,
                state_instruction,
                &mut state_instructions_vec,
                branch_on_damage,
            );
            i += 1;
        }
    } else {
        first_move_side = SideReference::SideTwo;
        generate_instructions_from_move(
            state,
            &mut side_two_choice,
            &side_one_choice,
            SideReference::SideTwo,
            incoming_instructions,
            &mut state_instructions_vec,
            branch_on_damage,
        );
        side_one_choice.first_move = false;
        let mut i = 0;
        let vec_len = state_instructions_vec.len();
        while i < vec_len {
            let state_instruction = state_instructions_vec.remove(0);
            generate_instructions_from_move(
                state,
                &mut side_one_choice.clone(),
                &side_two_choice,
                SideReference::SideOne,
                state_instruction,
                &mut state_instructions_vec,
                branch_on_damage,
            );
            i += 1;
        }
    }

    if end_of_turn_triggered(side_one_move, side_two_move) {
        for state_instruction in state_instructions_vec.iter_mut() {
            add_end_of_turn_instructions(
                state,
                state_instruction,
                &side_one_choice,
                &side_two_choice,
                &first_move_side,
            );
        }
    }

    state_instructions_vec
}

pub fn calculate_damage_rolls(
    mut state: State,
    attacking_side_ref: &SideReference,
    mut choice: Choice,
    mut defending_choice: &Choice,
) -> Option<Vec<i16>> {
    let mut incoming_instructions = StateInstructions::default();

    if choice.flags.charge {
        choice.flags.charge = false;
    }
    if choice.move_id == Choices::FAKEOUT {
        state.get_side(attacking_side_ref).last_used_move = LastUsedMove::Switch(PokemonIndex::P0);
    }

    let attacker_active = state
        .get_side_immutable(attacking_side_ref)
        .get_active_immutable();
    let defender_active = state
        .get_side_immutable(&attacking_side_ref.get_other_side())
        .get_active_immutable();
    match choice.move_id {
        Choices::SEISMICTOSS => {
            if type_effectiveness_modifier(&PokemonType::NORMAL, &defender_active) == 0.0 {
                return None;
            }
            return Some(vec![attacker_active.level as i16]);
        }
        Choices::NIGHTSHADE => {
            if type_effectiveness_modifier(&PokemonType::GHOST, &defender_active) == 0.0 {
                return None;
            }
            return Some(vec![attacker_active.level as i16]);
        }
        Choices::FINALGAMBIT => {
            if type_effectiveness_modifier(&PokemonType::GHOST, &defender_active) == 0.0 {
                return None;
            }
            return Some(vec![attacker_active.hp]);
        }
        Choices::ENDEAVOR => {
            if type_effectiveness_modifier(&PokemonType::GHOST, &defender_active) == 0.0
                || defender_active.hp <= attacker_active.hp
            {
                return None;
            }
            return Some(vec![defender_active.hp - attacker_active.hp]);
        }
        Choices::PAINSPLIT => {
            if type_effectiveness_modifier(&PokemonType::GHOST, &defender_active) == 0.0
                || defender_active.hp <= attacker_active.hp
            {
                return None;
            }
            return Some(vec![
                defender_active.hp - (attacker_active.hp + defender_active.hp) / 2,
            ]);
        }
        Choices::SUPERFANG
            if type_effectiveness_modifier(&PokemonType::NORMAL, &defender_active) == 0.0 =>
        {
            return None;
        }
        Choices::SUPERFANG | Choices::NATURESMADNESS | Choices::RUINATION => {
            return Some(vec![defender_active.hp / 2]);
        }
        Choices::SUCKERPUNCH | Choices::THUNDERCLAP => {
            defending_choice = MOVES.get(&Choices::TACKLE).unwrap();
        }

        _ => {}
    }

    before_move(
        &mut state,
        &mut choice,
        defending_choice,
        attacking_side_ref,
        &mut incoming_instructions,
    );

    if choice.move_id == Choices::FUTURESIGHT {
        choice = MOVES.get(&Choices::FUTURESIGHT)?.clone();
    }

    let mut return_vec = Vec::with_capacity(16);
    if let Some((damage, _crit_damage)) =
        calculate_damage(&state, attacking_side_ref, &choice, DamageRolls::Max)
    {
        let damage = damage as f32;
        return_vec.push((damage * 0.85) as i16);
        return_vec.push((damage * 0.86) as i16);
        return_vec.push((damage * 0.87) as i16);
        return_vec.push((damage * 0.88) as i16);
        return_vec.push((damage * 0.89) as i16);
        return_vec.push((damage * 0.90) as i16);
        return_vec.push((damage * 0.91) as i16);
        return_vec.push((damage * 0.92) as i16);
        return_vec.push((damage * 0.93) as i16);
        return_vec.push((damage * 0.94) as i16);
        return_vec.push((damage * 0.95) as i16);
        return_vec.push((damage * 0.96) as i16);
        return_vec.push((damage * 0.97) as i16);
        return_vec.push((damage * 0.98) as i16);
        return_vec.push((damage * 0.99) as i16);
        return_vec.push(damage as i16);
        Some(return_vec)
    } else {
        None
    }
}

pub fn calculate_both_damage_rolls(
    state: &State,
    mut s1_choice: Choice,
    mut s2_choice: Choice,
    side_one_moves_first: bool,
) -> (Option<Vec<i16>>, Option<Vec<i16>>) {
    if side_one_moves_first {
        s1_choice.first_move = true;
        s2_choice.first_move = false;
    } else {
        s1_choice.first_move = false;
        s2_choice.first_move = true;
    }

    let damages_dealt_s1 = calculate_damage_rolls(
        state.clone(),
        &SideReference::SideOne,
        s1_choice.clone(),
        &s2_choice,
    );
    let damages_dealt_s2 = calculate_damage_rolls(
        state.clone(),
        &SideReference::SideTwo,
        s2_choice,
        &s1_choice,
    );

    (damages_dealt_s1, damages_dealt_s2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abilities::Abilities;
    use crate::choices::{Choices, MOVES};
    use crate::instruction::{
        ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
        ChangeStatusInstruction, ChangeSubsituteHealthInstruction, ChangeTerrain,
        DamageInstruction, EnableMoveInstruction, SwitchInstruction,
    };
    use crate::state::{
        Move, PokemonBoostableStat, PokemonIndex, PokemonMoveIndex, SideReference, State, Terrain,
    };

    #[test]
    fn test_drag_move_as_second_move_exits_early_if_opponent_used_drag_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::DRAGONTAIL).unwrap().to_owned();
        choice.first_move = false;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::DRAGONTAIL).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_electric_move_does_nothing_versus_ground_type() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::THUNDERBOLT).unwrap().to_owned();
        state.side_two.get_active().types = (PokemonType::GROUND, PokemonType::TYPELESS);
        choice.first_move = false;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_grass_type_cannot_have_powder_move_used_against_it() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::SPORE).unwrap().to_owned(); // Spore is a powder move
        state.side_two.get_active().types = (PokemonType::GRASS, PokemonType::TYPELESS);
        choice.first_move = false;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        #[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
        let expected_instructions = vec![StateInstructions::default()];

        #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5"))]
        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::SLEEP,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_spikes_sets_first_layer() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::SPIKES).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: 1,
                },
            )],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_spikes_layers_cannot_exceed_3() {
        let mut state: State = State::default();
        state.side_two.side_conditions.spikes = 3;
        let mut choice = MOVES.get(&Choices::SPIKES).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_auroa_veil_works_in_hail() {
        let mut state: State = State::default();
        state.weather.weather_type = Weather::HAIL;
        let mut choice = MOVES.get(&Choices::AURORAVEIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::AuroraVeil,
                    amount: 1,
                },
            )],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_custap_berry_consumed_when_less_than_25_percent_hp() {
        let mut state: State = State::default();
        state.side_one.get_active().item = Items::CUSTAPBERRY;
        state.side_one.get_active().hp = 24;
        let mut choice = MOVES.get(&Choices::AURORAVEIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::CUSTAPBERRY,
                new_item: Items::NONE,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_auroa_veil_fails_outside_of_hail() {
        let mut state: State = State::default();
        state.weather.weather_type = Weather::NONE;
        let mut choice = MOVES.get(&Choices::AURORAVEIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_stealthrock_cannot_exceed_1_layer() {
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        let mut choice = MOVES.get(&Choices::STEALTHROCK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_stoneaxe_damage_and_stealthrock_setting() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::STONEAXE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                        side_ref: SideReference::SideTwo,
                        side_condition: PokemonSideCondition::Stealthrock,
                        amount: 1,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_ceaselessedge_damage_and_stealthrock_setting() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::CEASELESSEDGE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                        side_ref: SideReference::SideTwo,
                        side_condition: PokemonSideCondition::Spikes,
                        amount: 1,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_100_percent_secondary_volatilestatus() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::CHATTER).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_possible_secondary_volatilestatus() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::CONFUSION).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 40,
                })],
            },
            StateInstructions {
                percentage: 10.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 40,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::CONFUSION,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_possible_secondary_volatilestatus_with_possible_accuracy() {
        let mut state: State = State::default();
        state.side_two.get_active().hp = 400;
        state.side_two.get_active().maxhp = 400;
        let mut choice = MOVES.get(&Choices::AXEKICK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 50, // This move has recoil lol
                })],
            },
            StateInstructions {
                percentage: 63.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 188,
                })],
            },
            StateInstructions {
                percentage: 27.0000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 188,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::CONFUSION,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_volatile_status_applied_to_self() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::AQUARING).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ApplyVolatileStatus(
                ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::AQUARING,
                },
            )],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_volatile_status_applied_to_opponent() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::ATTRACT).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ApplyVolatileStatus(
                ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::ATTRACT,
                },
            )],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_cannot_apply_volatile_status_twice() {
        let mut state: State = State::default();
        state
            .side_two
            .volatile_statuses
            .insert(PokemonVolatileStatus::ATTRACT);
        let mut choice = MOVES.get(&Choices::ATTRACT).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_substitute_failing_if_user_has_less_than_25_percent_hp() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 25;
        let mut choice = MOVES.get(&Choices::SUBSTITUTE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_drag_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::WHIRLWIND).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P2,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P3,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P4,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P5,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_drag_move_with_fainted_reserve() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        let mut choice = MOVES.get(&Choices::WHIRLWIND).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P2,
                })],
            },
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P4,
                })],
            },
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P5,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_damaging_drag_move_with_fainted_reserve() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        let mut choice = MOVES.get(&Choices::DRAGONTAIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.0000019,
                instruction_list: vec![], // The move missed
            },
            StateInstructions {
                percentage: 30.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P2,
                    }),
                ],
            },
            StateInstructions {
                percentage: 30.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P4,
                    }),
                ],
            },
            StateInstructions {
                percentage: 30.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P5,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_damaging_drag_that_knocks_out_defender() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        state.side_two.get_active().hp = 5;
        let mut choice = MOVES.get(&Choices::DRAGONTAIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.0000019,
                instruction_list: vec![], // The move missed
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 5,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_drag_versus_no_alive_reserved() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P2].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        state.side_two.pokemon[PokemonIndex::P4].hp = 0;
        state.side_two.pokemon[PokemonIndex::P5].hp = 0;
        let mut choice = MOVES.get(&Choices::WHIRLWIND).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_drag_move_with_fainted_reserve_and_prior_instruction() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        let mut choice = MOVES.get(&Choices::WHIRLWIND).unwrap().to_owned();

        let previous_instruction = StateInstructions {
            percentage: 50.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            })],
        };

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            previous_instruction,
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 16.666668,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 5,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P2,
                    }),
                ],
            },
            StateInstructions {
                percentage: 16.666668,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 5,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P4,
                    }),
                ],
            },
            StateInstructions {
                percentage: 16.666668,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 5,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P5,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    #[cfg(feature = "gen9")]
    fn test_basic_status_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::GLARE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::PARALYZE,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    #[cfg(feature = "gen9")]
    fn test_status_move_that_can_miss() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::THUNDERWAVE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::PARALYZE,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_status_move_that_can_miss_but_is_blocked_by_ability() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::LIMBER;
        let mut choice = MOVES.get(&Choices::THUNDERWAVE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_conditional_burn_on_contact() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 70.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                })],
            },
            StateInstructions {
                percentage: 30.0000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_protectivepads_stops_flamebody() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        state.side_one.get_active().item = Items::PROTECTIVEPADS;
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_versus_noncontact_move() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = MOVES.get(&Choices::WATERGUN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 32,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_versus_fire_type() {
        let mut state: State = State::default();
        state.side_one.get_active().types.0 = PokemonType::FIRE;
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = MOVES.get(&Choices::WATERGUN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 32,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_move_with_multiple_secondaries() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::FIREFANG).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 5.00000095,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 76.9499969,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                })],
            },
            StateInstructions {
                percentage: 8.55000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::FLINCH,
                    }),
                ],
            },
            StateInstructions {
                percentage: 8.55000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
            StateInstructions {
                percentage: 0.949999988,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::FLINCH,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 70.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                })],
            },
            StateInstructions {
                percentage: 30.000002,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_creating_a_move_with_multiple_secondaries() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = MOVES.get(&Choices::FIREPUNCH).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 63.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 60,
                })],
            },
            StateInstructions {
                percentage: 27.0000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 60,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
            StateInstructions {
                percentage: 7.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 60,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
            StateInstructions {
                percentage: 3.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 60,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
        ];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_substitute_does_not_block_rest() {
        let mut state: State = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::SUBSTITUTE);
        state.side_one.get_active().hp = state.side_one.get_active().maxhp - 1;
        let mut choice = MOVES.get(&Choices::REST).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::SLEEP,
                }),
                Instruction::SetRestTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 3,
                    previous_turns: 0,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 1,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_heal_move() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 1;
        let mut choice = MOVES.get(&Choices::RECOVER).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 50,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_heal_move_generates_no_instruction_at_maxhp() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::RECOVER).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_negative_heal_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::EXPLOSION).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 100,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_negative_heal_move_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 1;
        let mut choice = MOVES.get(&Choices::EXPLOSION).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_heal_move_does_not_overheal() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 55;
        let mut choice = MOVES.get(&Choices::RECOVER).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 45,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_boosting_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::SWORDSDANCE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 2,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_does_not_overboost() {
        let mut state: State = State::default();
        state.side_one.attack_boost = 5;
        let mut choice = MOVES.get(&Choices::SWORDSDANCE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_no_instruction_when_boosting_at_max() {
        let mut state: State = State::default();
        state.side_one.attack_boost = 6;
        let mut choice = MOVES.get(&Choices::SWORDSDANCE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_boost_lowering_that_can_miss() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::KINESIS).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 19.999998,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 80.0,
                instruction_list: vec![Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Accuracy,
                    amount: -1,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_boost_lowering() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::CHARM).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: -2,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_cannot_boost_lower_than_negative_6() {
        let mut state: State = State::default();
        state.side_two.attack_boost = -5;
        let mut choice = MOVES.get(&Choices::CHARM).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: -1,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_no_boost_when_already_at_minimum() {
        let mut state: State = State::default();
        state.side_two.attack_boost = -6;
        let mut choice = MOVES.get(&Choices::CHARM).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_clearbody_blocks_stat_lowering() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::CLEARBODY;
        let mut choice = MOVES.get(&Choices::CHARM).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_clearbody_does_not_block_self_stat_lowering() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::CLEARBODY;
        let mut choice = MOVES.get(&Choices::SHELLSMASH).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: 2,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::SpecialAttack,
                    amount: 2,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::SpecialDefense,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: 2,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_defog_does_not_change_terrain_if_terrain_is_none() {
        let mut state: State = State::default();

        let mut choice = MOVES.get(&Choices::DEFOG).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_defog_clears_terrain() {
        let mut state: State = State::default();
        state.terrain.terrain_type = Terrain::ELECTRICTERRAIN;
        state.terrain.turns_remaining = 1;

        let mut choice = MOVES.get(&Choices::DEFOG).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeTerrain(ChangeTerrain {
                new_terrain: Terrain::NONE,
                new_terrain_turns_remaining: 0,
                previous_terrain: Terrain::ELECTRICTERRAIN,
                previous_terrain_turns_remaining: 1,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_defog_clears_terrain_and_side_conditions() {
        let mut state: State = State::default();
        state.terrain.terrain_type = Terrain::ELECTRICTERRAIN;
        state.terrain.turns_remaining = 1;
        state.side_one.side_conditions.reflect = 1;
        state.side_two.side_conditions.reflect = 1;

        let mut choice = MOVES.get(&Choices::DEFOG).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeTerrain(ChangeTerrain {
                    new_terrain: Terrain::NONE,
                    new_terrain_turns_remaining: 0,
                    previous_terrain: Terrain::ELECTRICTERRAIN,
                    previous_terrain_turns_remaining: 1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Reflect,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Reflect,
                    amount: -1,
                }),
            ],
        }];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_tidyup_clears_side_conditions_and_substitutes() {
        let mut state: State = State::default();
        state.terrain.terrain_type = Terrain::ELECTRICTERRAIN;
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::SUBSTITUTE);
        state
            .side_two
            .volatile_statuses
            .insert(PokemonVolatileStatus::SUBSTITUTE);
        state.side_one.substitute_health = 10;
        state.side_two.substitute_health = 25;
        state.terrain.turns_remaining = 1;
        state.side_one.side_conditions.spikes = 2;
        state.side_two.side_conditions.stealth_rock = 1;

        let mut choice = MOVES.get(&Choices::TIDYUP).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: -2,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::ChangeSubstituteHealth(ChangeSubsituteHealthInstruction {
                    side_ref: SideReference::SideOne,
                    health_change: -10,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                }),
                Instruction::ChangeSubstituteHealth(ChangeSubsituteHealthInstruction {
                    side_ref: SideReference::SideTwo,
                    health_change: -25,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: 1,
                }),
            ],
        }];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    #[cfg(any(feature = "gen8", feature = "gen9"))]
    fn test_rapidspin_clears_hazards() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;

        let mut choice = MOVES.get(&Choices::RAPIDSPIN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 61,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    amount: 1,
                    stat: PokemonBoostableStat::Speed,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_missing_rapidspin_does_not_clear_hazards() {
        let mut state: State = State::default();
        state.side_two.get_active().types = (PokemonType::GHOST, PokemonType::NORMAL);
        state.side_one.side_conditions.stealth_rock = 1;

        let mut choice = MOVES.get(&Choices::RAPIDSPIN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_acid_into_steel_type() {
        let mut state: State = State::default();
        state.side_two.get_active().types = (PokemonType::STEEL, PokemonType::NORMAL);

        let mut choice = MOVES.get(&Choices::ACID).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    #[cfg(any(feature = "gen8", feature = "gen9"))]
    fn test_rapidspin_clears_multiple_hazards() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.side_conditions.spikes = 3;
        state.side_one.side_conditions.sticky_web = 1;

        let mut choice = MOVES.get(&Choices::RAPIDSPIN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 61,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: -3,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: -2,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::StickyWeb,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    amount: 1,
                    stat: PokemonBoostableStat::Speed,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    #[cfg(any(feature = "gen8", feature = "gen9"))]
    fn test_rapidspin_does_not_clear_opponent_hazards() {
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        state.side_two.side_conditions.toxic_spikes = 2;
        state.side_two.side_conditions.spikes = 3;
        state.side_two.side_conditions.sticky_web = 1;

        let mut choice = MOVES.get(&Choices::RAPIDSPIN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 61,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    amount: 1,
                    stat: PokemonBoostableStat::Speed,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_courtchange_basic_swap() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;

        let mut choice = MOVES.get(&Choices::COURTCHANGE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: 1,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_courtchange_complicated_swap() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_two.side_conditions.toxic_spikes = 2;
        state.side_two.side_conditions.spikes = 3;
        state.side_two.side_conditions.sticky_web = 1;

        let mut choice = MOVES.get(&Choices::COURTCHANGE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: 1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: -3,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: 3,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: -2,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: 2,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::StickyWeb,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::StickyWeb,
                    amount: 1,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_stoneaxe_does_not_set_stealthrock_if_already_set() {
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        let mut choice = MOVES.get(&Choices::STONEAXE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flinched_pokemon_cannot_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::FLINCH);

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_taunted_pokemon_cannot_use_status_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::GLARE).unwrap().to_owned();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::TAUNT);

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_pokemon_taunted_on_first_turn_cannot_use_status_move() {
        let mut state: State = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::TAUNT);

        let mut choice = MOVES.get(&Choices::GLARE).unwrap().to_owned();
        choice.first_move = false;

        let mut incoming_instructions = StateInstructions::default();
        incoming_instructions
            .instruction_list
            .push(Instruction::ApplyVolatileStatus(
                ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::TAUNT,
                },
            ));

        let original_incoming_instructions = incoming_instructions.clone();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            incoming_instructions,
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![original_incoming_instructions])
    }

    #[test]
    fn test_dead_pokemon_moving_second_does_nothing() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        choice.first_move = false;
        state.side_one.get_active().hp = 0;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_cannot_ohko_versus_study() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::EARTHQUAKE).unwrap().to_owned();
        state.side_two.get_active().ability = Abilities::STURDY;
        state.side_two.get_active().hp = 50;
        state.side_two.get_active().maxhp = 50;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 49,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_cannot_ohko_versus_sash() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::EARTHQUAKE).unwrap().to_owned();
        state.side_two.get_active().item = Items::FOCUSSASH;
        state.side_two.get_active().hp = 50;
        state.side_two.get_active().maxhp = 50;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 49,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_sturdy_does_not_affect_non_ohko_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::EARTHQUAKE).unwrap().to_owned();
        state.side_two.get_active().ability = Abilities::STURDY;
        state.side_two.get_active().hp = 45;
        state.side_two.get_active().maxhp = 50;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 45,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_boosts_on_kill() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().ability = Abilities::BEASTBOOST;
        state.side_one.get_active().attack = 500; // highest stat
        state.side_two.get_active().hp = 1;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: 1,
                }),
            ],
        };
        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_boosts_different_stat_on_kill() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().ability = Abilities::BEASTBOOST;
        state.side_one.get_active().defense = 500; // highest stat
        state.side_two.get_active().hp = 1;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: 1,
                }),
            ],
        };
        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_does_not_overboost() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().ability = Abilities::BEASTBOOST;
        state.side_one.get_active().attack = 500; // highest stat
        state.side_one.attack_boost = 6; // max boosts already
        state.side_two.get_active().hp = 1;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_does_not_boost_without_kill() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().ability = Abilities::BEASTBOOST;
        state.side_one.get_active().attack = 150; // highest stat
        state.side_two.get_active().hp = 100;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 72,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_move_heals() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::ABSORB).unwrap().to_owned();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().maxhp = 200;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 8,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_move_does_not_overheal() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::ABSORB).unwrap().to_owned();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().maxhp = 105;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_recoil_damage() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::BRAVEBIRD).unwrap().to_owned();
        state.side_one.get_active().hp = 105;
        state.side_one.get_active().maxhp = 105;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 94,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 31,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_recoil_cannot_overkill() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::BRAVEBIRD).unwrap().to_owned();
        state.side_one.get_active().hp = 5;
        state.side_one.get_active().maxhp = 105;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 94,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_and_recoil_together() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::ABSORB).unwrap().to_owned();
        choice.recoil = Some(0.33);
        state.side_one.get_active().hp = 1;
        state.side_one.get_active().maxhp = 105;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 8,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_crash_move_missing() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::JUMPKICK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 5.000001,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 50,
                })],
            },
            StateInstructions {
                percentage: 95.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_crash_move_missing_versus_ghost_type() {
        let mut state: State = State::default();
        state.side_two.get_active().types.0 = PokemonType::GHOST;
        let mut choice = MOVES.get(&Choices::JUMPKICK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: Vec<StateInstructions> = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 50,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_crash_move_missing_cannot_overkill() {
        let mut state: State = State::default();
        state.get_side(&SideReference::SideOne).get_active().hp = 5;
        let mut choice = MOVES.get(&Choices::JUMPKICK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 5.000001,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                })],
            },
            StateInstructions {
                percentage: 95.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    #[cfg(feature = "gen9")]
    fn test_knockoff_removing_item() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::KNOCKOFF).unwrap().to_owned();
        state.get_side(&SideReference::SideTwo).get_active().item = Items::HEAVYDUTYBOOTS;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 76,
                }),
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: SideReference::SideTwo,
                    current_item: Items::HEAVYDUTYBOOTS,
                    new_item: Items::NONE,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_blunderpolicy_boost() {
        let mut state: State = State::default();
        let mut choice = MOVES.get(&Choices::CROSSCHOP).unwrap().to_owned();
        state.get_side(&SideReference::SideOne).get_active().item = Items::BLUNDERPOLICY;

        let mut instructions = vec![];
        generate_instructions_from_move(
            &mut state,
            &mut choice,
            &MOVES.get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 19.999998,
                instruction_list: vec![
                    Instruction::ChangeItem(ChangeItemInstruction {
                        side_ref: SideReference::SideOne,
                        current_item: Items::BLUNDERPOLICY,
                        new_item: Items::NONE,
                    }),
                    Instruction::Boost(BoostInstruction {
                        side_ref: SideReference::SideOne,
                        stat: PokemonBoostableStat::Speed,
                        amount: 2,
                    }),
                ],
            },
            StateInstructions {
                percentage: 80.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn test_basic_switch_functionality_with_no_prior_instructions() {
        let mut state: State = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_volatile_statuses() {
        let mut state: State = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::LEECHSEED,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_toxic_count() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_count = 2;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicCount,
                    amount: -2,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_boost() {
        let mut state: State = State::default();
        state.side_one.attack_boost = 2;
        state.side_one.speed_boost = 5;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: -2,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: -5,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_disabled_move() {
        let mut state: State = State::default();
        state.side_one.get_active().moves.m0 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m1 = Move {
            id: Choices::NONE,
            disabled: false,
            pp: 32,
            ..Default::default()
        };

        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M0,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_multiple_disabled_moves() {
        let mut state: State = State::default();
        state.side_one.get_active().moves.m0 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m1 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m2 = Move {
            id: Choices::NONE,
            disabled: false,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m3 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M0,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M1,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M3,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_functionality_with_a_prior_instruction() {
        let mut state: State = State::default();
        let mut incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };

        choice.switch_id = PokemonIndex::P1;
        incoming_instructions
            .instruction_list
            .push(Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }));

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator() {
        let mut state: State = State::default();
        state.side_one.get_active().hp -= 10;
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 10,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator_plus_move_enabling() {
        let mut state: State = State::default();
        state.side_one.get_active().moves.m0 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m1 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m2 = Move {
            id: Choices::NONE,
            disabled: false,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m3 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().hp -= 10;
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M0,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M1,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M3,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 10,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator_but_no_damage_taken() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_fainted_pokemon_with_regenerator_does_not_heal() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        state.side_one.get_active().hp = 0;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_regenerator_only_heals_one_third() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        state.side_one.get_active().hp = 3;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 33,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_naturalcure() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::NATURALCURE;
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::PARALYZE,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_naturalcure_with_no_status() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::NATURALCURE;
        state.side_one.get_active().status = PokemonStatus::NONE;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: state.side_one.get_active().hp / 8,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_resisted_stealthrock() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.pokemon[PokemonIndex::P1].types = (PokemonType::GROUND, PokemonType::NORMAL);
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: state.side_one.get_active().hp / 16,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.pokemon[PokemonIndex::P1].hp = 5;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stickyweb() {
        let mut state: State = State::default();
        state.side_one.side_conditions.sticky_web = 1;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: -1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stickyweb_with_heavydutyboots() {
        let mut state: State = State::default();
        state.side_one.side_conditions.sticky_web = 1;
        state.side_one.pokemon[PokemonIndex::P1].item = Items::HEAVYDUTYBOOTS;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stickyweb_with_contrary() {
        let mut state: State = State::default();
        state.side_one.side_conditions.sticky_web = 1;
        state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::CONTRARY;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: 1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_single_layer_toxicspikes() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 1;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P1,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::POISON,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P1,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::TOXIC,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_flying_type() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.pokemon[PokemonIndex::P1].types.0 = PokemonType::FLYING;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_poison_and_flying_type() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.pokemon[PokemonIndex::P1].types.0 = PokemonType::FLYING;
        state.side_one.pokemon[PokemonIndex::P1].types.1 = PokemonType::POISON;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate() {
        let mut state: State = State::default();
        state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Attack,
                    amount: -1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate_when_opponent_is_already_lowest_atk_boost() {
        let mut state: State = State::default();
        state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;
        state.side_two.attack_boost = -6;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate_versus_clearbody() {
        let mut state: State = State::default();
        state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;
        state.side_two.get_active().ability = Abilities::CLEARBODY;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_poison_type() {
        let mut state: State = State::default();
        state.side_one.pokemon[PokemonIndex::P1].types.0 = PokemonType::POISON;
        state.side_one.side_conditions.toxic_spikes = 2;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: -2,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_and_spikes_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.spikes = 1;
        state.side_one.pokemon[PokemonIndex::P1].hp = 15;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 3,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_and_multiple_layers_of_spikes_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.spikes = 3;
        state.side_one.pokemon[PokemonIndex::P1].hp = 25;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 13,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_healthy_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions::default();

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            &mut vec![],
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_rest_turns_at_3_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().rest_turns = 3;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementRestTurns(
                DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                },
            )],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];

        let frozen_instructions = &mut vec![];
        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_rest_turns_at_2_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().rest_turns = 2;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementRestTurns(
                DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                },
            )],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_paralyzed_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 75.0,
            instruction_list: vec![],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 25.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_confused_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::CONFUSION);
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0 * (1.0 - HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 100.0 * (HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 35,
            })],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_confused_pokemon_with_prior_instruction() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::CONFUSION);
        let mut incoming_instructions = StateInstructions::default();
        incoming_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })];

        let expected_instructions = StateInstructions {
            percentage: 100.0 * (1.0 - HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 100.0 * HIT_SELF_IN_CONFUSION_CHANCE,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 35,
                }),
            ],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_confused_pokemon_with_prior_instruction_does_not_overkill() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::CONFUSION);
        let mut incoming_instructions = StateInstructions::default();
        state.side_one.get_active().hp = 2;
        incoming_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })];

        let expected_instructions = StateInstructions {
            percentage: 100.0 * (1.0 - HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 100.0 * HIT_SELF_IN_CONFUSION_CHANCE,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 2,
                }),
            ],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_frozen_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::FREEZE;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 20.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: PokemonStatus::FREEZE,
                new_status: PokemonStatus::NONE,
            })],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 80.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().sleep_turns = MAX_SLEEP_TURNS;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: MAX_SLEEP_TURNS,
                }),
            ],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_waking_up_and_confused() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().sleep_turns = MAX_SLEEP_TURNS;
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::CONFUSION);
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0 * (1.0 - HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: MAX_SLEEP_TURNS,
                }),
            ],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 100.0 * HIT_SELF_IN_CONFUSION_CHANCE,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: MAX_SLEEP_TURNS,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 35,
                }),
            ],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_pokemon_waking_up_with_1_rest_turn() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().rest_turns = 1;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];
        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_pokemon_staying_asleep_with_two_rest_turns() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().rest_turns = 1;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];
        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_paralyzed_pokemon_preserves_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        let mut incoming_instructions = StateInstructions::default();
        incoming_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })];

        let expected_instructions = StateInstructions {
            percentage: 75.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 25.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_basic_side_two_moves_first() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            false,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_custap_berry_when_less_than_25_percent_activates() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().item = Items::CUSTAPBERRY;
        state.side_one.get_active().hp = 24;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_quarkdrivespe_boost_works() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::QUARKDRIVESPE);
        state.side_one.get_active().hp = 24;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_protosynthesisspe_boost_works() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::PROTOSYNTHESISSPE);
        state.side_one.get_active().hp = 24;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_custap_berry_when_greater_than_25_percent_does_not_activate() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().item = Items::CUSTAPBERRY;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            false,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_custap_berry_does_not_matter_when_opponent_uses_increased_priority_move() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::QUICKATTACK).unwrap().to_owned();
        state.side_one.get_active().item = Items::CUSTAPBERRY;
        state.side_one.get_active().hp = 24;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            false,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_slowstart_halves_effective_speed() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;
        state
            .side_two
            .volatile_statuses
            .insert(PokemonVolatileStatus::SLOWSTART);

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_basic_side_one_moves_first() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_paralysis_reduces_effective_speed() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();

        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            false,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    #[cfg(any(feature = "gen7", feature = "gen8", feature = "gen9"))]
    fn test_later_gen_speed_cutting_in_half() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        state.side_one.get_active().speed = 100;

        assert_eq!(50, get_effective_speed(&state, &SideReference::SideOne))
    }

    #[test]
    #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5", feature = "gen6"))]
    fn test_earlier_gen_speed_cutting_by_75_percent() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        state.side_one.get_active().speed = 100;

        assert_eq!(25, get_effective_speed(&state, &SideReference::SideOne))
    }

    #[test]
    fn test_choicescarf_multiplying_speed() {
        let mut state = State::default();
        state.side_one.get_active().speed = 100;
        state.side_one.get_active().item = Items::CHOICESCARF;

        assert_eq!(150, get_effective_speed(&state, &SideReference::SideOne))
    }

    #[test]
    fn test_iron_ball_halving_speed() {
        let mut state = State::default();
        state.side_one.get_active().speed = 100;
        state.side_one.get_active().item = Items::IRONBALL;

        assert_eq!(50, get_effective_speed(&state, &SideReference::SideOne))
    }

    #[test]
    fn test_speed_tie_goes_to_side_two() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            false,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_higher_priority_ignores_speed_diff() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::QUICKATTACK).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_side_two_higher_priority_ignores_speed_diff() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::QUICKATTACK).unwrap().to_owned();
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            false,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_both_higher_priority_defaults_back_to_speed() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::QUICKATTACK).unwrap().to_owned();
        let side_two_choice = MOVES.get(&Choices::QUICKATTACK).unwrap().to_owned();
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_switch_always_goes_first() {
        let mut state = State::default();
        let mut side_one_choice = MOVES.get(&Choices::SPLASH).unwrap().to_owned();
        side_one_choice.category = MoveCategory::Switch;
        let side_two_choice = MOVES.get(&Choices::QUICKATTACK).unwrap().to_owned();
        state.side_one.get_active().speed = 99;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_double_switch_checks_higher_speed() {
        let mut state = State::default();
        let mut side_one_choice = MOVES.get(&Choices::SPLASH).unwrap().to_owned();
        side_one_choice.category = MoveCategory::Switch;
        let mut side_two_choice = MOVES.get(&Choices::SPLASH).unwrap().to_owned();
        side_two_choice.category = MoveCategory::Switch;

        state.side_one.get_active().speed = 99;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            false,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_pursuit_goes_before_switch() {
        let mut state = State::default();
        let side_one_choice = MOVES.get(&Choices::PURSUIT).unwrap().to_owned();
        let mut side_two_choice = MOVES.get(&Choices::SPLASH).unwrap().to_owned();
        side_two_choice.category = MoveCategory::Switch;

        state.side_one.get_active().speed = 50;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_end_of_turn_hail_damage() {
        let mut state = State::default();
        state.weather.weather_type = Weather::HAIL;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_hail_damage_against_ice_type() {
        let mut state = State::default();
        state.weather.weather_type = Weather::HAIL;
        state.side_two.get_active().types.0 = PokemonType::ICE;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                // no damage to side_two
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_sand_damage() {
        let mut state = State::default();
        state.weather.weather_type = Weather::SAND;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_sand_damage_against_ground_type() {
        let mut state = State::default();
        state.weather.weather_type = Weather::SAND;
        state.side_two.get_active().types.0 = PokemonType::GROUND;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,

            // no damage to side_two
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_hail_does_not_overkill() {
        let mut state = State::default();
        state.weather.weather_type = Weather::HAIL;
        state.side_one.get_active().hp = 3;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 3,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_fainted_pkmn_does_not_take_hail_dmg() {
        let mut state = State::default();
        state.weather.weather_type = Weather::HAIL;
        state.side_one.get_active().hp = 0;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    #[cfg(not(feature = "gen4"))]
    fn test_wished_pokemon_gets_healed() {
        let mut state = State::default();
        state.side_one.wish = (1, 5);
        state.side_one.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 5,
                }),
                Instruction::DecrementWish(DecrementWishInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wish_does_not_overheal() {
        let mut state = State::default();
        state.side_one.wish = (1, 50);
        state.side_one.get_active().hp = 95;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 5,
                }),
                Instruction::DecrementWish(DecrementWishInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wish_does_nothing_when_maxhp() {
        let mut state = State::default();
        state.side_one.wish = (1, 50);
        state.side_one.get_active().hp = 100;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wish_does_nothing_when_fainted() {
        let mut state = State::default();
        state.side_one.wish = (1, 50);
        state.side_one.get_active().hp = 0;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wish_at_2_does_not_heal() {
        let mut state = State::default();
        state.side_one.wish = (2, 50);
        state.side_one.get_active().hp = 95;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leftovers_heals_at_end_of_turn() {
        let mut state = State::default();
        state.side_one.get_active().hp = 50;
        state.side_one.get_active().item = Items::LEFTOVERS;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leftovers_does_not_overheal() {
        let mut state = State::default();
        state.side_one.get_active().hp = 99;
        state.side_one.get_active().item = Items::LEFTOVERS;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 1,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leftovers_generates_no_instruction_at_maxhp() {
        let mut state = State::default();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().item = Items::LEFTOVERS;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leftovers_generates_no_instruction_when_fainted() {
        let mut state = State::default();
        state.side_one.get_active().hp = 0;
        state.side_one.get_active().item = Items::LEFTOVERS;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_blacksludge_heal_as_poison_type() {
        let mut state = State::default();
        state.side_one.get_active().hp = 50;
        state.side_one.get_active().item = Items::BLACKSLUDGE;
        state.side_one.get_active().types.0 = PokemonType::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_blacksludge_damage_as_non_poison_type() {
        let mut state = State::default();
        state.side_one.get_active().hp = 50;
        state.side_one.get_active().item = Items::BLACKSLUDGE;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_blacksludge_does_not_overheal() {
        let mut state = State::default();
        state.side_one.get_active().hp = 99;
        state.side_one.get_active().item = Items::BLACKSLUDGE;
        state.side_one.get_active().types.0 = PokemonType::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 1,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_flameorb_end_of_turn_burn() {
        let mut state = State::default();
        state.side_one.get_active().item = Items::FLAMEORB;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::BURN,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_fire_type_cannot_be_burned_by_flameorb() {
        let mut state = State::default();
        state.side_one.get_active().item = Items::FLAMEORB;
        state.side_one.get_active().types.0 = PokemonType::FIRE;
        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_toxicorb_applies_status() {
        let mut state = State::default();
        state.side_one.get_active().item = Items::TOXICORB;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::TOXIC,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_toxicorb_does_not_apply_to_poison_type() {
        let mut state = State::default();
        state.side_one.get_active().item = Items::TOXICORB;
        state.side_one.get_active().types.0 = PokemonType::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poisonheal_heals_at_end_of_turn() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::POISONHEAL;
        state.side_one.get_active().status = PokemonStatus::POISON;
        state.side_one.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poisonheal_does_not_overheal() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::POISONHEAL;
        state.side_one.get_active().status = PokemonStatus::POISON;
        state.side_one.get_active().hp = 99;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 1,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poisonheal_does_nothign_at_maxhp() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::POISONHEAL;
        state.side_one.get_active().status = PokemonStatus::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_speedboost() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::SPEEDBOOST;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Speed,
                amount: 1,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_speedboost_does_not_boost_beyond_6() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::SPEEDBOOST;
        state.side_one.speed_boost = 6;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_poison_damage() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poison_damage_does_not_overkill() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::POISON;
        state.side_one.get_active().hp = 5;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    #[cfg(any(feature = "gen9", feature = "gen8", feature = "gen7"))]
    fn test_end_of_turn_burn_damage() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::BURN;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5", feature = "gen6"))]
    fn test_early_generation_burn_one_eigth() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::BURN;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_burn_damage_does_not_overkill() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::BURN;
        state.side_one.get_active().hp = 5;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_burn_damage_ignored_if_has_magicguard() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::BURN;
        state.side_one.get_active().ability = Abilities::MAGICGUARD;
        state.side_one.get_active().hp = 5;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_first_toxic_damage() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::TOXIC;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicCount,
                    amount: 1,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leechseed_sap() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        state.side_one.get_active().hp = 50;
        state.side_two.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideTwo,
                    heal_amount: 12,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leechseed_sap_does_not_heal_if_receiving_side_is_maxhp() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        state.side_one.get_active().hp = 50;
        state.side_two.get_active().hp = 100;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leechseed_sap_does_not_overkill() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        state.side_one.get_active().hp = 5;
        state.side_two.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideTwo,
                    heal_amount: 5,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leechseed_sap_does_not_overheal() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        state.side_one.get_active().hp = 50;
        state.side_two.get_active().hp = 95;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideTwo,
                    heal_amount: 5,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_protect_volatile_being_removed() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::PROTECT);

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::PROTECT,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Protect,
                    amount: 1,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_protect_side_condition_being_removed() {
        let mut state = State::default();
        state.side_one.side_conditions.protect = 2;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Protect,
                    amount: -2,
                },
            )],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_roost_vs_removal() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::ROOST);

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::RemoveVolatileStatus(
                RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::ROOST,
                },
            )],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_partiallytrapped_damage() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::PARTIALLYTRAPPED);

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5"))]
        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            })],
        };

        #[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_saltcure_on_water_type_damage() {
        let mut state = State::default();
        state.side_one.get_active().types.0 = PokemonType::WATER;
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::SALTCURE);

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions(
            &mut state,
            &mut incoming_instructions,
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &MOVES.get(&Choices::TACKLE).unwrap().to_owned(),
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 25,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_chance_to_wake_up_with_no_turns_asleep_is_0() {
        assert_eq!(0.0, chance_to_wake_up(0));
    }

    #[test]
    #[cfg(any(feature = "gen4"))]
    fn test_gen4_25_percent_to_wake_after_1_sleep_turn() {
        assert_eq!(0.25, chance_to_wake_up(1));
    }

    #[test]
    #[cfg(any(feature = "gen4"))]
    fn test_gen4_100_percent_to_wake_after_4_sleep_turn() {
        assert_eq!(1.0, chance_to_wake_up(4));
    }
}
