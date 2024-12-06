use crate::choice_effects::choice_after_damage_hit;
use crate::choice_effects::{
    charge_choice_to_volatile, choice_before_move, choice_special_effect, modify_choice,
};
use crate::choices::{
    Boost, Choices, Effect, Heal, MoveTarget, MultiHitMove, Secondary, Status, VolatileStatus,
};
use crate::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeSideConditionInstruction,
    DecrementRestTurnsInstruction, HealInstruction, RemoveVolatileStatusInstruction,
    SetSleepTurnsInstruction,
};
use crate::instruction::{
    DecrementPPInstruction, SetDamageDealtSideOneInstruction, SetDamageDealtSideTwoInstruction,
};
use crate::state::{
    MoveChoice, PokemonBoostableStat, PokemonIndex, PokemonSideCondition, PokemonType, Side,
};
use crate::{
    choices::{Choice, MoveCategory},
    damage_calc::{calculate_damage, DamageRolls},
    instruction::{
        ChangeStatusInstruction, DamageInstruction, Instruction, StateInstructions,
        SwitchInstruction,
    },
    state::{PokemonStatus, PokemonVolatileStatus, SideReference, State},
};
use std::cmp;

pub const MAX_SLEEP_TURNS: i8 = 7;

fn chance_to_wake_up(turns_asleep: i8) -> f32 {
    if turns_asleep == 0 {
        0.0
    } else {
        1.0 / (1 + MAX_SLEEP_TURNS - turns_asleep) as f32
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

pub fn generate_instructions_from_switch(
    state: &mut State,
    new_pokemon_index: PokemonIndex,
    switching_side_ref: SideReference,
    incoming_instructions: &mut StateInstructions,
) {
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

    state.re_enable_disabled_moves(
        &switching_side_ref,
        &mut incoming_instructions.instruction_list,
    );
    state.remove_volatile_statuses_on_switch(
        &switching_side_ref,
        &mut incoming_instructions.instruction_list,
    );
    state.reset_toxic(
        &switching_side_ref,
        &mut incoming_instructions.instruction_list,
    );
    state.reset_boosts(
        &switching_side_ref,
        &mut incoming_instructions.instruction_list,
    );

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

    state.reverse_instructions(&incoming_instructions.instruction_list);
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

        if volatile_status.volatile_status == PokemonVolatileStatus::SUBSTITUTE {
            let affected_pkmn = state.get_side(&target_side).get_active();
            let damage_taken = affected_pkmn.maxhp / 4;
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

    if target_pkmn.status != PokemonStatus::NONE || target_pkmn.hp <= 0 {
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
            PokemonStatus::BURN => target_pkmn.has_type(&PokemonType::FIRE),
            PokemonStatus::FREEZE => {
                target_pkmn.has_type(&PokemonType::ICE) || target_side.has_alive_frozen_pokemon()
            }
            PokemonStatus::SLEEP => {
                // sleep clause
                status_target == &MoveTarget::Opponent
                    && target_side.has_alive_non_rested_sleeping_pkmn()
            }

            PokemonStatus::POISON | PokemonStatus::TOXIC => {
                target_pkmn.has_type(&PokemonType::POISON)
                    || target_pkmn.has_type(&PokemonType::STEEL)
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
                    Effect::RemoveItem => {}
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

        frozen_instructions.push(move_missed_instruction);
    }
    incoming_instructions.update_percentage(percent_hit);
}

fn generate_instructions_from_damage(
    state: &mut State,
    choice: &Choice,
    calculated_damage: i16,
    attacking_side_ref: &SideReference,
    should_use_damage_dealt: bool,
    mut incoming_instructions: &mut StateInstructions,
) -> bool {
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
        let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
        let damage_dealt;
        if defending_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SUBSTITUTE)
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
            let defending_pokemon = defending_side.get_active();
            damage_dealt = cmp::min(calculated_damage, defending_pokemon.hp);
            if damage_dealt != 0 {
                let damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    damage_amount: damage_dealt,
                });
                defending_pokemon.hp -= damage_dealt;
                incoming_instructions
                    .instruction_list
                    .push(damage_instruction);
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
            state,
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
    if choice.move_type == PokemonType::ELECTRIC
        && choice.target == MoveTarget::Opponent
        && defender.has_type(&PokemonType::GROUND)
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
    } else if attacking_side
        .volatile_statuses
        .contains(&PokemonVolatileStatus::FLINCH)
    {
        return true;
    }
    false
}

pub fn before_move(
    state: &mut State,
    choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side: &SideReference,
    incoming_instructions: &mut StateInstructions,
) {
    choice_before_move(state, choice, attacking_side, incoming_instructions);
    modify_choice(state, choice, defender_choice, attacking_side);

    let (attacking_side, _defending_side) = state.get_both_sides_immutable(attacking_side);
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
}

fn generate_instructions_from_existing_status_conditions(
    state: &mut State,
    attacking_side_ref: &SideReference,
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
            // Gen1 cannot thaw on your own
            final_instructions.push(incoming_instructions.clone());
            incoming_instructions.update_percentage(0.0);
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
                        final_instructions.push(incoming_instructions.clone());
                        incoming_instructions.update_percentage(0.0);
                    } else if chance_to_wake == 0.0 {
                        let mut still_asleep_instruction = incoming_instructions.clone();
                        still_asleep_instruction.update_percentage(1.0);
                        still_asleep_instruction
                            .instruction_list
                            .push(Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                side_ref: *attacking_side_ref,
                                pokemon_index: current_active_index,
                                new_turns: current_sleep_turns + 1,
                                previous_turns: current_sleep_turns,
                            }));
                        final_instructions.push(still_asleep_instruction);
                        incoming_instructions.update_percentage(0.0);
                    } else {
                        // still asleep
                        let mut still_asleep_instruction = incoming_instructions.clone();
                        still_asleep_instruction.update_percentage(1.0 - chance_to_wake);
                        still_asleep_instruction
                            .instruction_list
                            .push(Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                                side_ref: *attacking_side_ref,
                                pokemon_index: current_active_index,
                                new_turns: current_sleep_turns + 1,
                                previous_turns: current_sleep_turns,
                            }));
                        final_instructions.push(still_asleep_instruction);

                        // wakes up
                        incoming_instructions.update_percentage(chance_to_wake);
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
                        final_instructions.push(incoming_instructions.clone());
                        incoming_instructions.update_percentage(0.0);
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
                    final_instructions.push(incoming_instructions.clone());
                    incoming_instructions.update_percentage(0.0);
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
        hit_yourself_instruction.update_percentage(0.50);

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

        incoming_instructions.update_percentage(0.50);
    }
}

pub fn generate_instructions_from_move(
    state: &mut State,
    choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side: SideReference,
    mut incoming_instructions: StateInstructions,
    mut final_instructions: &mut Vec<StateInstructions>,
    _branch_if_roll_kills: bool,
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

    if choice.move_id == Choices::NONE {
        if state
            .get_side(&attacking_side)
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

    state.apply_instructions(&incoming_instructions.instruction_list);

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
    let attacker_side = state.get_side(&attacking_side);
    let active = attacker_side.get_active();
    if active.moves[&choice.move_index].pp < 10 {
        let pp_decrement_amount = 1;
        incoming_instructions
            .instruction_list
            .push(Instruction::DecrementPP(DecrementPPInstruction {
                side_ref: attacking_side,
                move_index: choice.move_index,
                amount: pp_decrement_amount,
            }));
        active.moves[&choice.move_index].pp -= pp_decrement_amount;
    }

    generate_instructions_from_existing_status_conditions(
        state,
        &attacking_side,
        &mut incoming_instructions,
        &mut final_instructions,
    );
    let attacker = state
        .get_side_immutable(&attacking_side)
        .get_active_immutable();
    if attacker.status == PokemonStatus::SLEEP {
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
    choice_special_effect(
        state,
        &choice,
        &defender_choice,
        &attacking_side,
        &mut incoming_instructions,
    );
    let damage = calculate_damage(state, &attacking_side, &choice, DamageRolls::Average);
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
            hit_count = 3;
        }
        _ => {
            hit_count = 1;
        }
    }
    let mut hit_sub: bool = false;
    let set_damage_dealt = defender_choice.move_id == Choices::COUNTER
        && (choice.move_type == PokemonType::NORMAL || choice.move_type == PokemonType::FIGHTING);

    let (mut average_damage, mut average_crit_damage) = (0, 0);
    let does_damage;
    if let Some((regular_damage, crit_damage)) = damage {
        average_damage = regular_damage;
        average_crit_damage = crit_damage;
        does_damage = true
    } else {
        does_damage = false;
    }

    let (attacker_side, defender_side) = state.get_both_sides(&attacking_side);
    let crit_instructions: Option<StateInstructions>;

    // Only calculate crit if regular damage doesn't KO
    if average_damage < defender_side.get_active().hp {
        let crit_rate = attacker_side.get_active().crit_rate(&choice.move_id);
        let mut crit = incoming_instructions.clone();
        crit.update_percentage(crit_rate);
        crit_instructions = Some(crit);
        incoming_instructions.update_percentage(1.0 - crit_rate);
    } else {
        crit_instructions = None
    }

    // Non-Crit
    if incoming_instructions.percentage != 0.0 {
        for _ in 0..hit_count {
            if does_damage {
                hit_sub = generate_instructions_from_damage(
                    state,
                    &choice,
                    average_damage,
                    &attacking_side,
                    set_damage_dealt,
                    &mut incoming_instructions,
                );
            }
            if let Some(volatile_status) = &choice.volatile_status {
                get_instructions_from_volatile_statuses(
                    state,
                    &choice,
                    volatile_status,
                    &attacking_side,
                    &mut incoming_instructions,
                );
            }
            if let Some(status) = &choice.status {
                get_instructions_from_status_effects(
                    state,
                    status,
                    &attacking_side,
                    &mut incoming_instructions,
                    hit_sub,
                );
            }
            if let Some(heal) = &choice.heal {
                get_instructions_from_heal(
                    state,
                    heal,
                    &attacking_side,
                    &mut incoming_instructions,
                );
            }
        }
        if let Some(boost) = &choice.boost {
            get_instructions_from_boosts(state, boost, &attacking_side, &mut incoming_instructions);
        }

        if let Some(secondaries_vec) = &choice.secondaries {
            state.reverse_instructions(&incoming_instructions.instruction_list);
            let instructions_vec_after_secondaries = get_instructions_from_secondaries(
                state,
                &choice,
                secondaries_vec,
                &attacking_side,
                incoming_instructions,
                hit_sub,
            );
            final_instructions.extend(instructions_vec_after_secondaries);
        } else {
            state.reverse_instructions(&incoming_instructions.instruction_list);
            final_instructions.push(incoming_instructions);
        }
    } else {
        state.reverse_instructions(&incoming_instructions.instruction_list);
    }

    // Crit
    if let Some(mut crit_instructions) = crit_instructions {
        if crit_instructions.percentage != 0.0 {
            state.apply_instructions(&crit_instructions.instruction_list);
            for _ in 0..hit_count {
                if does_damage {
                    hit_sub = generate_instructions_from_damage(
                        state,
                        &choice,
                        average_crit_damage,
                        &attacking_side,
                        set_damage_dealt,
                        &mut crit_instructions,
                    );
                }
                if let Some(volatile_status) = &choice.volatile_status {
                    get_instructions_from_volatile_statuses(
                        state,
                        &choice,
                        volatile_status,
                        &attacking_side,
                        &mut crit_instructions,
                    );
                }
                if let Some(status) = &choice.status {
                    get_instructions_from_status_effects(
                        state,
                        status,
                        &attacking_side,
                        &mut crit_instructions,
                        hit_sub,
                    );
                }
                if let Some(heal) = &choice.heal {
                    get_instructions_from_heal(
                        state,
                        heal,
                        &attacking_side,
                        &mut crit_instructions,
                    );
                }
            }

            if let Some(boost) = &choice.boost {
                get_instructions_from_boosts(state, boost, &attacking_side, &mut crit_instructions);
            }
            if let Some(secondaries_vec) = &choice.secondaries {
                state.reverse_instructions(&crit_instructions.instruction_list);
                let instructions_vec_after_secondaries = get_instructions_from_secondaries(
                    state,
                    &choice,
                    secondaries_vec,
                    &attacking_side,
                    crit_instructions,
                    hit_sub,
                );
                final_instructions.extend(instructions_vec_after_secondaries);
            } else {
                state.reverse_instructions(&crit_instructions.instruction_list);
                final_instructions.push(crit_instructions);
            }
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
    if active_pkmn.status == PokemonStatus::PARALYZE {
        boosted_speed *= 0.25;
    }

    boosted_speed as i16
}

pub fn side_one_moves_first(
    state: &State,
    side_one_choice: &Choice,
    side_two_choice: &Choice,
) -> bool {
    let side_one_effective_speed = get_effective_speed(&state, &SideReference::SideOne);
    let side_two_effective_speed = get_effective_speed(&state, &SideReference::SideTwo);

    if side_one_choice.category == MoveCategory::Switch
        && side_two_choice.category == MoveCategory::Switch
    {
        return side_one_effective_speed > side_two_effective_speed;
    } else if side_one_choice.category == MoveCategory::Switch {
        return true;
    } else if side_two_choice.category == MoveCategory::Switch {
        return false;
    }

    if side_one_choice.priority == side_two_choice.priority {
        side_one_effective_speed > side_two_effective_speed
    } else {
        side_one_choice.priority > side_two_choice.priority
    }
}

pub fn add_end_of_turn_instructions(
    state: &mut State,
    incoming_instructions: &mut StateInstructions,
    _side_one_choice: &Choice,
    _side_two_choice: &Choice,
    first_move_side: &SideReference,
) {
    state.apply_instructions(&incoming_instructions.instruction_list);
    if state.side_one.force_switch || state.side_two.force_switch {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        return;
    }

    let sides = [first_move_side, &first_move_side.get_other_side()];

    // status damage
    for side_ref in sides {
        let side = state.get_side(side_ref);
        let toxic_count = side.side_conditions.toxic_count as f32;
        let active_pkmn = side.get_active();
        if active_pkmn.hp == 0 {
            continue;
        }

        match active_pkmn.status {
            PokemonStatus::BURN => {
                let damage_amount = cmp::max(
                    cmp::min((active_pkmn.maxhp as f32 * 0.0625) as i16, active_pkmn.hp),
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
            PokemonStatus::POISON => {
                let damage_amount = cmp::max(
                    1,
                    cmp::min((active_pkmn.maxhp as f32 * 0.0625) as i16, active_pkmn.hp),
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
            PokemonStatus::TOXIC => {
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

    // leechseed sap
    for side_ref in sides {
        let (leechseed_side, other_side) = state.get_both_sides(side_ref);
        if leechseed_side
            .volatile_statuses
            .contains(&PokemonVolatileStatus::LEECHSEED)
        {
            let active_pkmn = leechseed_side.get_active();
            let other_active_pkmn = other_side.get_active();
            if active_pkmn.hp == 0 || other_active_pkmn.hp == 0 {
                continue;
            }

            let health_sapped =
                cmp::min((active_pkmn.maxhp as f32 * 0.0625) as i16, active_pkmn.hp);
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

    // volatile status damage
    for side_ref in sides {
        let side = state.get_side(side_ref);
        let active_pkmn = side.get_active();
        if active_pkmn.hp == 0 {
            continue;
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
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);
}

fn end_of_turn_triggered(side_one_move: &MoveChoice, side_two_move: &MoveChoice) -> bool {
    !(matches!(side_one_move, &MoveChoice::Switch(_)) && side_two_move == &MoveChoice::None)
        && !(side_one_move == &MoveChoice::None && matches!(side_two_move, &MoveChoice::Switch(_)))
}

pub fn generate_instructions_from_move_pair(
    state: &mut State,
    side_one_move: &MoveChoice,
    side_two_move: &MoveChoice,
    branch_if_roll_kills: bool,
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
        MoveChoice::None => {
            side_one_choice = Choice::default();
        }
        MoveChoice::MoveTera(_) => panic!("Tera not available"),
    }

    let mut side_two_choice;
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
        MoveChoice::None => {
            side_two_choice = Choice::default();
        }
        MoveChoice::MoveTera(_) => panic!("Tera not available"),
    }

    let mut state_instructions_vec: Vec<StateInstructions> = Vec::with_capacity(16);
    let incoming_instructions: StateInstructions = StateInstructions::default();

    let first_move_side;
    if side_one_moves_first(&state, &side_one_choice, &side_two_choice) {
        first_move_side = SideReference::SideOne;
        generate_instructions_from_move(
            state,
            &mut side_one_choice,
            &side_two_choice,
            SideReference::SideOne,
            incoming_instructions,
            &mut state_instructions_vec,
            branch_if_roll_kills,
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
                branch_if_roll_kills,
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
            branch_if_roll_kills,
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
                branch_if_roll_kills,
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
    defending_choice: &Choice,
) -> Option<Vec<i16>> {
    let mut incoming_instructions = StateInstructions::default();

    if choice.flags.charge {
        choice.flags.charge = false;
    }

    let attacker_active = state
        .get_side_immutable(attacking_side_ref)
        .get_active_immutable();
    let defender_active = state
        .get_side_immutable(&attacking_side_ref.get_other_side())
        .get_active_immutable();
    match choice.move_id {
        Choices::SEISMICTOSS | Choices::NIGHTSHADE => {
            // Gen1 Seismic Toss / Night Shade do not check type effectiveness
            return Some(vec![attacker_active.level as i16]);
        }
        Choices::SUPERFANG => {
            // Gen1 Super Fang does not check type effectiveness
            return Some(vec![defender_active.hp / 2]);
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
