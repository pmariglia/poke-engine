use crate::choices::{
    Boost, Effect, HazardClearFn, Heal, MoveTarget, Secondary, SideCondition, Status,
    VolatileStatus,
};
use crate::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeSideConditionInstruction, EnableMoveInstruction, HealInstruction,
    RemoveVolatileStatusInstruction,
};
use crate::state::{PokemonBoostableStat, PokemonSideCondition, PokemonType, Side, Terrain};
use crate::{
    abilities::ABILITIES,
    choices::{Choice, MoveCategory, MOVES},
    damage_calc::{calculate_damage, type_effectiveness_modifier, DamageRolls},
    instruction::{
        ChangeStatusInstruction, DamageInstruction, Instruction, StateInstructions,
        SwitchInstruction,
    },
    items::ITEMS,
    state::{Pokemon, PokemonStatus, PokemonVolatileStatus, SideReference, State, Weather},
};
use std::cmp;

type InstructionGenerationFn =
    fn(&mut State, &Choice, &SideReference, StateInstructions) -> StateInstructions;

fn generate_instructions_from_switch(
    state: &mut State,
    new_pokemon_index: usize,
    switching_side_ref: SideReference,
    incoming_instructions: StateInstructions,
) -> StateInstructions {
    let mut incoming_instructions = incoming_instructions;
    state.apply_instructions(&incoming_instructions.instruction_list);

    let switching_side = state.get_side_immutable(&switching_side_ref);
    let mut additional_instructions = vec![];
    for (pkmn_move_index, _) in switching_side
        .get_active_immutable()
        .moves
        .iter()
        .enumerate()
        .filter(|(_, m)| m.disabled)
    {
        additional_instructions.push(Instruction::EnableMove(EnableMoveInstruction {
            side_ref: switching_side_ref,
            move_index: pkmn_move_index,
        }));
    }

    for i in get_remove_volatile_status_and_boosts_instructions(switching_side, &switching_side_ref)
    {
        additional_instructions.push(i);
    }

    if let Some(ability) = ABILITIES.get(&switching_side.get_active_immutable().ability) {
        if let Some(on_switch_out_fn) = ability.on_switch_out {
            additional_instructions.extend(on_switch_out_fn(&state, &switching_side_ref));
        }
    }

    for i in additional_instructions {
        state.apply_one_instruction(&i);
        incoming_instructions.instruction_list.push(i);
    }

    let switch_instruction = Instruction::Switch(SwitchInstruction {
        side_ref: switching_side_ref,
        previous_index: state.get_side(&switching_side_ref).active_index,
        next_index: new_pokemon_index,
    });

    state.apply_one_instruction(&switch_instruction);
    incoming_instructions
        .instruction_list
        .push(switch_instruction);

    let mut switch_in_hazards_instructions = vec![];
    let switch_in_side = state.get_side_immutable(&switching_side_ref);
    let switched_in_pkmn = state
        .get_side_immutable(&switching_side_ref)
        .get_active_immutable();
    if switched_in_pkmn.item != "heavydutyboots" {
        if switch_in_side.side_conditions.stealth_rock == 1 {
            let multiplier =
                type_effectiveness_modifier(&PokemonType::Rock, &switched_in_pkmn.types) as i16;

            let stealth_rock_dmg_instruction = Instruction::Damage(DamageInstruction {
                side_ref: switching_side_ref,
                damage_amount: cmp::min(
                    switched_in_pkmn.maxhp * multiplier / 8,
                    switched_in_pkmn.hp,
                ),
            });
            state.apply_one_instruction(&stealth_rock_dmg_instruction);
            switch_in_hazards_instructions.push(stealth_rock_dmg_instruction);
        }

        let switch_in_side = state.get_side_immutable(&switching_side_ref);
        let switched_in_pkmn = state
            .get_side_immutable(&switching_side_ref)
            .get_active_immutable();
        if switch_in_side.side_conditions.spikes > 0 && switched_in_pkmn.is_grounded() {
            let spikes_dmg_instruction = Instruction::Damage(DamageInstruction {
                side_ref: switching_side_ref,
                damage_amount: cmp::min(
                    switched_in_pkmn.maxhp * switch_in_side.side_conditions.spikes as i16 / 8,
                    switched_in_pkmn.hp,
                ),
            });
            state.apply_one_instruction(&spikes_dmg_instruction);
            switch_in_hazards_instructions.push(spikes_dmg_instruction);
        }

        let switch_in_side = state.get_side_immutable(&switching_side_ref);
        let switched_in_pkmn = state
            .get_side_immutable(&switching_side_ref)
            .get_active_immutable();
        if switch_in_side.side_conditions.sticky_web == 1 && switched_in_pkmn.is_grounded() {
            // a pkmn switching in don't have any other speed drops,
            // so no need to check for going below -6

            let sticky_web_instruction = Instruction::Boost(BoostInstruction {
                side_ref: switching_side_ref,
                stat: PokemonBoostableStat::Speed,
                amount: -1,
            });

            state.apply_one_instruction(&sticky_web_instruction);
            switch_in_hazards_instructions.push(sticky_web_instruction);
        }

        let switch_in_side = state.get_side_immutable(&switching_side_ref);
        let switched_in_pkmn = state
            .get_side_immutable(&switching_side_ref)
            .get_active_immutable();
        let mut toxic_spike_instruction: Option<Instruction> = None;
        if switch_in_side.side_conditions.toxic_spikes > 0 && switched_in_pkmn.is_grounded() {
            if !immune_to_status(
                &state,
                &MoveTarget::User,
                &switching_side_ref,
                &PokemonStatus::Poison,
            ) {
                if switch_in_side.side_conditions.toxic_spikes == 1 {
                    toxic_spike_instruction =
                        Some(Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: switching_side_ref,
                            pokemon_index: switch_in_side.active_index,
                            old_status: switched_in_pkmn.status,
                            new_status: PokemonStatus::Poison,
                        }))
                } else if switch_in_side.side_conditions.toxic_spikes == 2 {
                    toxic_spike_instruction =
                        Some(Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: switching_side_ref,
                            pokemon_index: switch_in_side.active_index,
                            old_status: switched_in_pkmn.status,
                            new_status: PokemonStatus::Toxic,
                        }))
                }
            } else if switched_in_pkmn.has_type(&PokemonType::Poison) {
                toxic_spike_instruction = Some(Instruction::ChangeSideCondition(
                    ChangeSideConditionInstruction {
                        side_ref: switching_side_ref,
                        side_condition: PokemonSideCondition::ToxicSpikes,
                        amount: -1 * switch_in_side.side_conditions.toxic_spikes,
                    },
                ))
            }

            if let Some(i) = toxic_spike_instruction {
                state.apply_one_instruction(&i);
                switch_in_hazards_instructions.push(i);
            }
        }
    }

    for i in switch_in_hazards_instructions {
        incoming_instructions.instruction_list.push(i)
    }

    let mut additional_instructions = vec![];
    let switching_side = state.get_side_immutable(&switching_side_ref);
    if let Some(ability) = ABILITIES.get(&switching_side.get_active_immutable().ability) {
        if let Some(on_switch_in_fn) = ability.on_switch_in {
            additional_instructions.extend(on_switch_in_fn(&state, &switching_side_ref));
        }
    }
    if let Some(item) = ITEMS.get(&switching_side.get_active_immutable().item) {
        if let Some(on_switch_in_fn) = item.on_switch_in {
            additional_instructions.extend(on_switch_in_fn(&state, &switching_side_ref));
        }
    }

    for i in additional_instructions {
        incoming_instructions.instruction_list.push(i)
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);

    return incoming_instructions;
}

fn get_remove_volatile_status_and_boosts_instructions(
    side: &Side,
    side_ref: &SideReference,
) -> Vec<Instruction> {
    let mut instructions = vec![];
    let pkmn = side.get_active_immutable();
    for pkmn_volatile_status in &pkmn.volatile_statuses {
        instructions.push(Instruction::RemoveVolatileStatus(
            RemoveVolatileStatusInstruction {
                side_ref: *side_ref,
                volatile_status: *pkmn_volatile_status,
            },
        ));
    }

    if side.side_conditions.toxic_count > 0 {
        instructions.push(Instruction::ChangeSideCondition(
            ChangeSideConditionInstruction {
                side_ref: *side_ref,
                side_condition: PokemonSideCondition::ToxicCount,
                amount: -1 * side.side_conditions.toxic_count,
            },
        ))
    }

    for (boostable_stat, boost_val) in pkmn.get_pkmn_boost_enum_pairs() {
        if boost_val > 0 {
            instructions.push(Instruction::Boost(BoostInstruction {
                side_ref: *side_ref,
                stat: boostable_stat,
                amount: -1 * boost_val,
            }))
        }
    }
    return instructions;
}

fn generate_instructions_from_side_conditions(
    state: &mut State,
    side_condition: &SideCondition,
    attacking_side_reference: &SideReference,
    mut incoming_instructions: StateInstructions,
) -> StateInstructions {
    state.apply_instructions(&incoming_instructions.instruction_list);

    let affected_side_ref;
    match side_condition.target {
        MoveTarget::Opponent => affected_side_ref = attacking_side_reference.get_other_side(),
        MoveTarget::User => affected_side_ref = *attacking_side_reference,
    }

    let affected_side = state.get_side_immutable(&affected_side_ref);

    let max_layers;
    match side_condition.condition {
        PokemonSideCondition::Spikes => max_layers = 3,
        PokemonSideCondition::ToxicSpikes => max_layers = 3,
        PokemonSideCondition::AuroraVeil => {
            max_layers = if state.weather.weather_type == Weather::Hail {
                1
            } else {
                0
            }
        }
        _ => max_layers = 1,
    }

    let mut additional_instructions = vec![];
    if affected_side.get_side_condition(side_condition.condition) < max_layers {
        additional_instructions.push(Instruction::ChangeSideCondition(
            ChangeSideConditionInstruction {
                side_ref: affected_side_ref,
                side_condition: side_condition.condition,
                amount: 1,
            },
        ));
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);

    for i in additional_instructions {
        incoming_instructions.instruction_list.push(i)
    }

    return incoming_instructions;
}

fn get_instructions_from_hazard_clearing_moves(
    state: &mut State,
    hazard_clear_fn: &HazardClearFn,
    attacking_side_reference: &SideReference,
    mut incoming_instructions: StateInstructions,
) -> StateInstructions {
    state.apply_instructions(&incoming_instructions.instruction_list);
    let additional_instructions = hazard_clear_fn(state, attacking_side_reference);
    state.reverse_instructions(&incoming_instructions.instruction_list);
    for i in additional_instructions {
        incoming_instructions.instruction_list.push(i)
    }

    return incoming_instructions;
}

fn get_instructions_from_volatile_statuses(
    state: &mut State,
    volatile_status: &VolatileStatus,
    attacking_side_reference: &SideReference,
    mut incoming_instructions: StateInstructions,
) -> StateInstructions {
    state.apply_instructions(&incoming_instructions.instruction_list);

    let target_side: SideReference;
    match volatile_status.target {
        MoveTarget::Opponent => target_side = attacking_side_reference.get_other_side(),
        MoveTarget::User => target_side = *attacking_side_reference,
    }

    let mut additional_instructions = vec![];
    let affected_pkmn = state
        .get_side_immutable(&target_side)
        .get_active_immutable();
    if affected_pkmn.volitile_status_can_be_applied(&volatile_status.volatile_status) {
        additional_instructions.push(Instruction::ApplyVolatileStatus(
            ApplyVolatileStatusInstruction {
                side_ref: target_side,
                volatile_status: volatile_status.volatile_status,
            },
        ));
        if volatile_status.volatile_status == PokemonVolatileStatus::Substitute {
            additional_instructions.push(Instruction::Damage(DamageInstruction {
                side_ref: target_side,
                damage_amount: affected_pkmn.maxhp / 4,
            }));
        }
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);
    for i in additional_instructions {
        incoming_instructions.instruction_list.push(i)
    }
    return incoming_instructions;
}

fn sleep_clause_activated() -> bool {
    return false;
}

fn immune_to_status(
    state: &State,
    status_target: &MoveTarget,
    target_side_ref: &SideReference,
    status: &PokemonStatus,
) -> bool {
    let target_pkmn = state
        .get_side_immutable(target_side_ref)
        .get_active_immutable();

    // General Status Immunity
    match target_pkmn.ability.as_str() {
        "shieldsdown" => return target_pkmn.hp > target_pkmn.maxhp / 2,
        "purifyingsalt" => return true,
        "comatose" => return true,
        _ => {}
    }

    return if target_pkmn.status != PokemonStatus::None || target_pkmn.hp <= 0 {
        true
    } else if state.terrain.terrain_type == Terrain::MistyTerrain && target_pkmn.is_grounded() {
        true
    } else if target_pkmn
        .volatile_statuses
        .contains(&PokemonVolatileStatus::Substitute)
        && status_target == &MoveTarget::Opponent
    // substitute doesn't block if the target is yourself (eg. rest)
    {
        true
    } else {
        // Specific status immunity
        match status {
            PokemonStatus::Burn => {
                target_pkmn.has_type(&PokemonType::Fire)
                    || ["waterveil", "waterbubble"].contains(&target_pkmn.ability.as_str())
            }
            PokemonStatus::Freeze => {
                target_pkmn.has_type(&PokemonType::Ice)
                    || target_pkmn.ability.as_str() == "magmaarmor"
                    || state.weather.weather_type == Weather::HarshSun
            }
            PokemonStatus::Sleep => {
                (state.terrain.terrain_type == Terrain::ElectricTerrain
                    && target_pkmn.is_grounded())
                    || ["insomnia", "sweetveil", "vitalspirit"]
                        .contains(&target_pkmn.ability.as_str())
            }
            PokemonStatus::Paralyze => {
                target_pkmn.has_type(&PokemonType::Electric)
                    || target_pkmn.ability.as_str() == "limber"
            }
            PokemonStatus::Poison | PokemonStatus::Toxic => {
                target_pkmn.has_type(&PokemonType::Poison)
                    || target_pkmn.has_type(&PokemonType::Steel)
                    || ["immunity", "pastelveil"].contains(&target_pkmn.ability.as_str())
            }
            _ => false,
        }
    };
}

fn get_instructions_from_status_effects(
    state: &mut State,
    status: &Status,
    attacking_side_reference: &SideReference,
    mut incoming_instructions: StateInstructions,
) -> StateInstructions {
    state.apply_instructions(&incoming_instructions.instruction_list);

    let target_side_ref: SideReference;
    match status.target {
        MoveTarget::Opponent => target_side_ref = attacking_side_reference.get_other_side(),
        MoveTarget::User => target_side_ref = *attacking_side_reference,
    }

    if immune_to_status(state, &status.target, &target_side_ref, &status.status) {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        return incoming_instructions;
    }

    let mut additional_instructions = vec![];
    let target_side = state.get_side_immutable(&target_side_ref);
    let target_pkmn = target_side.get_active_immutable();

    let status_hit_instruction = Instruction::ChangeStatus(ChangeStatusInstruction {
        side_ref: target_side_ref,
        pokemon_index: target_side.active_index,
        old_status: target_pkmn.status,
        new_status: status.status,
    });
    additional_instructions.push(status_hit_instruction);

    state.reverse_instructions(&incoming_instructions.instruction_list);

    for i in additional_instructions {
        incoming_instructions.instruction_list.push(i)
    }

    return incoming_instructions;
}

pub fn get_boost_amount(pkmn: &Pokemon, boost: &PokemonBoostableStat, amount: &i8) -> i8 {
    /*
    returns that amount that can actually be applied from the attempted boost amount
        e.g. using swordsdance at +5 attack would result in a +1 boost instead of +2
    */
    let current_boost = pkmn.get_boost_from_boost_enum(boost);

    if amount > &0 {
        return cmp::min(6 - current_boost, *amount);
    } else if amount < &0 {
        return cmp::max(-6 - current_boost, *amount);
    }
    return 0;
}

pub fn get_boost_instruction(
    state: &State,
    stat: &PokemonBoostableStat,
    boost: &i8,
    attacking_side_ref: &SideReference,
    target_side_ref: &SideReference,
) -> Option<Instruction> {
    /*
    Single point for checking whether a boost can be applied to a pokemon
    Returns that boost instruction, if applicable
    */

    let target_pkmn = state
        .get_side_immutable(target_side_ref)
        .get_active_immutable();
    let boost_amount = get_boost_amount(target_pkmn, &stat, boost);
    if boost_amount != 0
        && !(target_side_ref != attacking_side_ref
            && target_pkmn.immune_to_stats_lowered_by_opponent())
    {
        return Some(Instruction::Boost(BoostInstruction {
            side_ref: *target_side_ref,
            stat: *stat,
            amount: boost_amount,
        }));
    }
    return None;
}

fn get_instructions_from_boosts(
    state: &mut State,
    boosts: &Boost,
    attacking_side_reference: &SideReference,
    mut incoming_instructions: StateInstructions,
) -> StateInstructions {
    state.apply_instructions(&incoming_instructions.instruction_list);
    let mut additional_instructions = vec![];

    let target_side_ref: SideReference;
    match boosts.target {
        MoveTarget::Opponent => target_side_ref = attacking_side_reference.get_other_side(),
        MoveTarget::User => target_side_ref = *attacking_side_reference,
    }
    let boostable_stats = boosts.boosts.get_as_pokemon_boostable();
    for (pkmn_boostable_stat, boost) in boostable_stats.iter().filter(|(_, b)| b != &0) {
        if let Some(boost_instruction) = get_boost_instruction(
            &state,
            pkmn_boostable_stat,
            boost,
            attacking_side_reference,
            &target_side_ref,
        ) {
            additional_instructions.push(boost_instruction)
        }
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);
    for i in additional_instructions {
        incoming_instructions.instruction_list.push(i)
    }
    return incoming_instructions;
}

fn generate_instructions_from_move_special_effect(
    _state: &mut State,
    choice: &Choice,
    _side_reference: &SideReference,
    incoming_instructions: StateInstructions,
) -> StateInstructions {
    return match choice.move_id.as_str() {
        // "haze" => {},
        _ => incoming_instructions,
    };
}

fn get_instructions_from_secondaries(
    state: &mut State,
    secondaries: &Vec<Secondary>,
    side_reference: &SideReference,
    incoming_instructions: StateInstructions,
) -> Vec<StateInstructions> {
    let mut incoming_instructions = vec![incoming_instructions];

    for secondary in secondaries {
        let mut loop_vec = vec![];
        let secondary_percent_hit = secondary.chance / 100.0;

        for mut ins in incoming_instructions {
            if secondary_percent_hit > 0.0 {
                let mut secondary_hit_instructions = ins.clone();
                secondary_hit_instructions.update_percentage(secondary_percent_hit);
                match &secondary.effect {
                    Effect::VolatileStatus(volatile_status) => {
                        secondary_hit_instructions = get_instructions_from_volatile_statuses(
                            state,
                            &VolatileStatus {
                                target: secondary.target.clone(),
                                volatile_status: volatile_status.clone(),
                            },
                            side_reference,
                            secondary_hit_instructions,
                        );
                    }
                    Effect::Boost(boost) => {
                        secondary_hit_instructions = get_instructions_from_boosts(
                            state,
                            &Boost {
                                target: secondary.target.clone(),
                                boosts: boost.clone(),
                            },
                            side_reference,
                            secondary_hit_instructions,
                        );
                    }
                    Effect::Status(status) => {
                        secondary_hit_instructions = get_instructions_from_status_effects(
                            state,
                            &Status {
                                target: secondary.target.clone(),
                                status: status.clone(),
                            },
                            side_reference,
                            secondary_hit_instructions,
                        );
                    }
                }
                loop_vec.push(secondary_hit_instructions);
            }

            if secondary_percent_hit < 1.0 {
                ins.update_percentage(1.0 - secondary_percent_hit);
                loop_vec.push(ins);
            }
        }
        incoming_instructions = loop_vec
    }

    return incoming_instructions;
}

fn get_instructions_from_heal(
    state: &mut State,
    heal: &Heal,
    attacking_side_reference: &SideReference,
    mut incoming_instructions: StateInstructions,
) -> StateInstructions {
    state.apply_instructions(&incoming_instructions.instruction_list);

    let target_side_ref: SideReference;
    match heal.target {
        MoveTarget::Opponent => target_side_ref = attacking_side_reference.get_other_side(),
        MoveTarget::User => target_side_ref = *attacking_side_reference,
    }

    let target_pkmn = state
        .get_side_immutable(&target_side_ref)
        .get_active_immutable();

    let mut health_recovered = (heal.amount * target_pkmn.maxhp as f32) as i16;
    let final_health = target_pkmn.hp + health_recovered;
    if final_health > target_pkmn.maxhp {
        health_recovered -= final_health - target_pkmn.maxhp;
    } else if final_health < 0 {
        health_recovered -= final_health;
    }

    let mut additional_instructions = vec![];
    if health_recovered != 0 {
        additional_instructions.push(Instruction::Heal(HealInstruction {
            side_ref: target_side_ref,
            heal_amount: health_recovered,
        }))
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);
    for i in additional_instructions {
        incoming_instructions.instruction_list.push(i)
    }

    return incoming_instructions;
}

fn check_move_hit_or_miss(
    state: &mut State,
    choice: &Choice,
    attacking_side_ref: &SideReference,
    incoming_instructions: &mut StateInstructions,
    frozen_instructions: &mut Vec<StateInstructions>,
) -> StateInstructions {
    /*
    Checks whether a move can miss

    If the move can miss - adds it to `frozen_instructions`, signifying that the rest of the
    half-turn will not run.

    Otherwise, return the instructions that the half-turn will continue to iterate on
    */

    state.apply_instructions(&incoming_instructions.instruction_list);

    let attacking_side = state.get_side_immutable(attacking_side_ref);
    let attacking_pokemon = attacking_side.get_active_immutable();

    let percent_hit = choice.accuracy / 100.0;

    let mut move_hit_instructions = incoming_instructions.clone();

    if percent_hit > 0.0 {
        move_hit_instructions.update_percentage(percent_hit);
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

        if attacking_pokemon.item.as_str() == "blunderpolicy"
            && attacking_pokemon.item_can_be_removed()
        {
            move_missed_instruction.instruction_list.extend(vec![
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: *attacking_side_ref,
                    current_item: String::from("blunderpolicy"),
                    new_item: "".to_string(),
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: *attacking_side_ref,
                    stat: PokemonBoostableStat::Speed,
                    amount: 2,
                }),
            ]);
        }

        frozen_instructions.push(move_missed_instruction);
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);

    return move_hit_instructions;
}

fn run_instruction_generation_fn_for_move_hit(
    instruction_generation_fn: InstructionGenerationFn,
    state: &mut State,
    choice: &Choice,
    side_reference: &SideReference,
    incoming_instructions: Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    let mut continuing_instructions: Vec<StateInstructions> = vec![];
    for instruction in incoming_instructions {
        continuing_instructions.push(instruction_generation_fn(
            state,
            choice,
            side_reference,
            instruction,
        ));
    }
    return continuing_instructions;
}

fn get_instructions_from_drag(
    state: &mut State,
    attacking_side_reference: &SideReference,
    incoming_instructions: StateInstructions,
    frozen_instructions: &mut Vec<StateInstructions>,
) {
    state.apply_instructions(&incoming_instructions.instruction_list);

    let defending_side = state.get_side(&attacking_side_reference.get_other_side());
    if defending_side.get_active_immutable().hp == 0 {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        frozen_instructions.push(incoming_instructions);
        return;
    }

    let defending_side_alive_reserve_indices = defending_side
        .pokemon
        .iter()
        .enumerate()
        .filter(|(a, b)| b.hp > 0 && a != &defending_side.active_index)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    state.reverse_instructions(&incoming_instructions.instruction_list);

    let num_alive_reserve = defending_side_alive_reserve_indices.len();
    if num_alive_reserve == 0 {
        frozen_instructions.push(incoming_instructions);
        return;
    }
    for pkmn_id in defending_side_alive_reserve_indices {
        let mut switch_instructions = generate_instructions_from_switch(
            state,
            pkmn_id,
            attacking_side_reference.get_other_side(),
            incoming_instructions.clone(),
        );
        switch_instructions.update_percentage(1.0 / num_alive_reserve as f32);
        frozen_instructions.push(switch_instructions);
    }
}

fn generate_instructions_from_damage(
    state: &mut State,
    choice: &Choice,
    calculated_damage: i16,
    attacking_side_ref: &SideReference,
    incoming_instructions: StateInstructions,
) -> Vec<StateInstructions> {
    /*
    TODO:
        - substitute consideration
            - requires a state change. VolatileStatus for sub & value for sub-health
            - do last tbh
        - arbitrary other after_move as well from the old engine (triggers on hit OR miss)
            - dig/dive/bounce/fly volatilestatus
        - item after damage hit: should generate instructions (rockyhelmet, for example)
    */

    let mut return_instructions: Vec<StateInstructions> = vec![];

    state.apply_instructions(&incoming_instructions.instruction_list);

    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    let attacking_pokemon = attacking_side.get_active_immutable();
    let defending_pokemon = defending_side.get_active_immutable();

    let percent_hit = choice.accuracy / 100.0;
    // Move hit
    if percent_hit > 0.0 {
        let mut move_hit_instructions = incoming_instructions.clone();

        let mut damage_dealt = cmp::min(calculated_damage, defending_pokemon.hp);

        if defending_pokemon.ability.as_str() == "sturdy"
            && defending_pokemon.maxhp == defending_pokemon.hp
        {
            damage_dealt -= 1;
        }

        move_hit_instructions
            .instruction_list
            .push(Instruction::Damage(DamageInstruction {
                side_ref: attacking_side_ref.get_other_side(),
                damage_amount: damage_dealt,
            }));

        if let Some(ability) = ABILITIES.get(&attacking_pokemon.ability) {
            if let Some(after_damage_hit_fn) = ability.after_damage_hit {
                move_hit_instructions
                    .instruction_list
                    .extend(after_damage_hit_fn(
                        state,
                        choice,
                        attacking_side_ref,
                        damage_dealt,
                    ));
            };
        }

        /*
        Generating these instructions does not need to mutate the state, so use
        `attacking_pokemon_health` to keep track of the attacking pokemon's health separately
        */
        let mut attacking_pokemon_health = attacking_pokemon.hp;
        if let Some(drain_fraction) = choice.drain {
            let drain_amount = (damage_dealt as f32 * drain_fraction) as i16;
            let heal_amount = cmp::min(
                drain_amount,
                attacking_pokemon.maxhp - attacking_pokemon_health,
            );
            let drain_instruction = Instruction::Heal(HealInstruction {
                side_ref: *attacking_side_ref,
                heal_amount: heal_amount,
            });
            move_hit_instructions
                .instruction_list
                .push(drain_instruction);
            attacking_pokemon_health += heal_amount;
        }

        if let Some(recoil_fraction) = choice.recoil {
            let recoil_amount = (damage_dealt as f32 * recoil_fraction) as i16;
            let recoil_instruction = Instruction::Damage(DamageInstruction {
                side_ref: *attacking_side_ref,
                damage_amount: cmp::min(recoil_amount, attacking_pokemon_health),
            });
            move_hit_instructions
                .instruction_list
                .push(recoil_instruction);
        }

        if let Some(after_damage_hit_fn) = choice.after_damage_hit {
            move_hit_instructions
                .instruction_list
                .extend(after_damage_hit_fn(&state, &choice, attacking_side_ref));
        }

        return_instructions.push(move_hit_instructions);
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);

    return return_instructions;
}

fn cannot_use_move(state: &State, choice: &Choice, attacking_side_ref: &SideReference) -> bool {
    /*
        Checks for any situation where a move cannot be used.
        Some examples:
            - electric type move versus a ground type
            - you are taunted and are trying to use a non-damaging move
            - you were flinched
            - etc.
    */
    let attacking_pkmn: &Pokemon = state
        .get_side_immutable(attacking_side_ref)
        .get_active_immutable();

    // If you were taunted, you can't use a Physical/Special move
    if attacking_pkmn
        .volatile_statuses
        .contains(&PokemonVolatileStatus::Taunt)
        && matches!(
            choice.category,
            MoveCategory::Physical | MoveCategory::Special
        )
    {
        return true;
    } else if attacking_pkmn
        .volatile_statuses
        .contains(&PokemonVolatileStatus::Flinch)
    {
        return true;
    } else if choice.move_type == PokemonType::Electric
        && state
            .get_side_immutable(&attacking_side_ref.get_other_side())
            .get_active_immutable()
            .has_type(&PokemonType::Ground)
    {
        return true;
    } else if choice.flags.powder
        && state
            .get_side_immutable(&attacking_side_ref.get_other_side())
            .get_active_immutable()
            .has_type(&PokemonType::Grass)
    {
        return true;
    }

    return false;
}

fn before_move(state: &State, choice: &Choice, attacking_side: &SideReference) -> Vec<Instruction> {
    let mut new_instructions = vec![];
    let attacking_pokemon = state
        .get_side_immutable(attacking_side)
        .get_active_immutable();

    if let Some(ability) = ABILITIES.get(&attacking_pokemon.ability) {
        if let Some(before_move_fn) = ability.before_move {
            new_instructions.append(&mut before_move_fn(state, choice, attacking_side));
        };
    }

    return new_instructions;
}

// Updates the attacker's Choice based on some special effects
fn update_choice(
    state: &State,
    attacker_choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side: &SideReference,
) {
    let (attacker_side, defender_side) = state.get_both_sides_immutable(attacking_side);
    let attacking_pokemon = attacker_side.get_active_immutable();
    let defending_pokemon = defender_side.get_active_immutable();

    match attacker_choice.modify_move {
        Some(modify_move_fn) => {
            modify_move_fn(state, attacker_choice, defender_choice, attacking_side);
        }
        None => {}
    }

    if let Some(ability) = ABILITIES.get(&attacking_pokemon.ability) {
        if let Some(modify_move_fn) = ability.modify_attack_being_used {
            modify_move_fn(state, attacker_choice, defender_choice, attacking_side)
        };
    }

    if let Some(ability) = ABILITIES.get(&defending_pokemon.ability) {
        if let Some(modify_move_fn) = ability.modify_attack_against {
            modify_move_fn(state, attacker_choice, defender_choice, attacking_side)
        };
    }

    if let Some(item) = ITEMS.get(&attacking_pokemon.item) {
        if let Some(modify_move_fn) = item.modify_attack_being_used {
            modify_move_fn(state, attacker_choice, attacking_side)
        };
    }

    if let Some(item) = ITEMS.get(&defending_pokemon.item) {
        if let Some(modify_move_fn) = item.modify_attack_against {
            modify_move_fn(state, attacker_choice, attacking_side)
        };
    }
}

fn generate_instructions_from_existing_status_conditions(
    state: &mut State,
    attacking_side_ref: &SideReference,
    mut incoming_instructions: StateInstructions,
    frozen_instructions: &mut Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    // Frozen, Sleep, and Paralysis may cause a Pokemon to not move

    let apply_reverse_instruction_list = incoming_instructions.instruction_list.clone();
    state.apply_instructions(&apply_reverse_instruction_list);

    let (attacking_side, _defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    let attacker_active = attacking_side.get_active_immutable();

    let mut instructions_that_will_proceed = Vec::<StateInstructions>::new();
    match attacker_active.status {
        PokemonStatus::Paralyze => {
            // Fully-Paralyzed Branch
            let mut fully_paralyzed_instruction = incoming_instructions.clone();
            fully_paralyzed_instruction.update_percentage(0.25);
            frozen_instructions.push(fully_paralyzed_instruction);

            // Non-Paralyzed Branch
            incoming_instructions.update_percentage(0.75);
            instructions_that_will_proceed.push(incoming_instructions);
        }
        PokemonStatus::Freeze => {
            // Thawing is a 20% event, and changes a pokemons status
            let mut thaw_instruction = incoming_instructions.clone();
            thaw_instruction.update_percentage(0.20);
            thaw_instruction
                .instruction_list
                .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: attacking_side_ref.clone(),
                    pokemon_index: attacking_side.active_index,
                    old_status: attacker_active.status,
                    new_status: PokemonStatus::None,
                }));
            instructions_that_will_proceed.push(thaw_instruction);

            // staying frozen
            incoming_instructions.update_percentage(0.80);
            frozen_instructions.push(incoming_instructions);
        }
        PokemonStatus::Sleep => {
            // Waking up is a 33% event, and changes the status
            let mut awake_instruction = incoming_instructions.clone();
            awake_instruction.update_percentage(0.33);
            awake_instruction
                .instruction_list
                .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: attacking_side_ref.clone(),
                    pokemon_index: attacking_side.active_index,
                    old_status: attacker_active.status,
                    new_status: PokemonStatus::None,
                }));
            instructions_that_will_proceed.push(awake_instruction);

            // staying asleep
            incoming_instructions.update_percentage(0.67);
            frozen_instructions.push(incoming_instructions);
        }
        _ => {
            instructions_that_will_proceed.push(incoming_instructions);
        }
    }

    state.reverse_instructions(&apply_reverse_instruction_list);
    return instructions_that_will_proceed;
}

// fn move_special_effects(state: &State, choice: &mut Choice) {}

// Interpreting the function arguments/return-value:
//
// This function takes in a mutable StateInstruction,
// and returns a Vector of StateInstructions, which
// represent all the possible branches that can be taken
// given that move being used
pub fn generate_instructions_from_move(
    state: &mut State,
    choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side: SideReference,
    mut incoming_instructions: StateInstructions,
) -> Vec<StateInstructions> {
    /*
    The functions that are called by this function will each take a StateInstruction struct that
    signifies what has already happened. If the function can cause a branch, it will return a
    vector of StateInstructions, otherwise it will return a StateInstruction. In both cases,
    the functions will take ownership of the value, and return a new value.

    Note: end-of-turn instructions are not included here - this is only the instructions from a move

    Order of Operations:
    (*) indicates it can branch, (-) indicates it does not

    - DONE check for if the user is switching - do so & exit early
    - check for short-circuit situations that would exit before doing anything
        - DONE using drag move but you moved 2nd (possible if say both users use dragontail)
        - DONE attacking pokemon is dead (possible if you got KO-ed this turn)
        - DONE attacking pokemon is taunted & chose a non-damaging move
        - DONE attacker was flinched
            - not a branching event because the previous turn would've decided whether a flinch happened or not
    - update choice struct based on special effects
        - protect (or it's variants) nullifying a move
            - this may generate a custom instruction because some protect variants do things (spikyshield, banefulbunker, etc)
            - rather than updating the choice struct, this should be a check that immediately adds the instruction list
              to `final_instructions` after applying the custom instructions from something like spikyshield ofc.
            - this can be done in the move_hit_or_miss function called before other moves
        - charging move that sets some charge flags and exits
            - again.. rather than exit, add the instructions to final instructions
        - DONE move special effect
        - DONE ability special effect (both sides)
        - DONE item special effect (both sides)

    BEGIN THINGS THAT HAPPEN AFTER FIRST POSSIBLE BRANCH
    * DONE attacker is fully-paralyzed, asleep, frozen (the first thing that can branch from the old engine)
    - DONE move has no effect (maybe something like check_if_move_can_hit)
        - i.e. electric-type status move used against a ground type, powder move used against grass / overcoat
        - Normally, the move doing 0 damage would trigger this, but for non-damaging moves there needs to be another
        spot where this is checked. This may be better done elsewhere
        - This HAS to be done after the frozen/sleep/paralyzed check, which is the first possible branch
    - move special effects
        hail, trick, futuresight, trickroom, etc. Anything that cannot be succinctly expressed in a Choice
        these will generate instructions (sometimes conditionally), but should not branch
    - MULTI HIT MOVES?!
    * DONE GOOD ENOUGH - WILL COME BACK TO AFTER ENGINE COMPLETE calculate damage amount(s) and do the damage
    - after-move effects
        * move special effect (both sides)
            - static/flamebody means this needs to possibly branch
            - protect and it's variants, which can generate some custom instructions
        - ability (both sides)
        - item (both sides)
    - DONE side_conditions: spikes, wish, veil. Anything using the `side_condition` section of the Choice
    - DONE hazard clearing: defog, rapidspin, courtchange, etc.
    - DONE volatile_statuses: Anything using the `volatile_status` section of the Choice
    - DONE status effects: Anything using the `status` section of the Choice
    - DONE boosts: Anything using the `boosts` section of the Choice
    - WONT DO boost reset (clearsmog & haze)
        potentially could be an `after_move` for clearsmog, and a move special effect for haze
            ^ will do this
    - DONE heal Anything using the `heal` section of the Choice
    * WONT DO flinching move
        collapse into secondaries
    * DONE drag moves
        potentially could be a move special effect, or even a short-circuit since nothing else could happen?
    * DONE secondaries, which will be one of the following:
        PokemonVolatileStatus
        PokemonSideCondition
        StatBoosts
        Heal
        PokemonStatus

        These secondaries have their own separate chance & target,
        whereas their equivalents above are assumed to be 100% if the
        move hit
        They only are attempted if the move did not miss , so some
        flag will be needed to signify that the move hit/missed
            or will the fact that the instructions are a non-end-of-turn be enough to know that
            a secondary should be attempted?

    - switch-out move
        Will have to come back to this since it breaks a bunch of patterns and stops the turn mid-way through

    */

    if choice.category == MoveCategory::Switch {
        return vec![generate_instructions_from_switch(
            state,
            choice.switch_id,
            attacking_side,
            incoming_instructions,
        )];
    }

    if !choice.first_move && choice.flags.drag {
        return vec![incoming_instructions];
    }

    state.apply_instructions(&incoming_instructions.instruction_list);

    if state
        .get_side_immutable(&attacking_side)
        .get_active_immutable()
        .hp
        == 0
    {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        return vec![incoming_instructions];
    }

    // Before-Move callbacks to update the choice
    update_choice(state, choice, defender_choice, &attacking_side);

    // Before-Move callbacks to generate new instructions
    let before_move_instructions = before_move(state, &choice, &attacking_side);
    state.apply_instructions(&before_move_instructions);
    incoming_instructions
        .instruction_list
        .extend(before_move_instructions);

    let damage = calculate_damage(state, attacking_side, &choice, DamageRolls::Average);

    state.reverse_instructions(&incoming_instructions.instruction_list);

    // The final return-value of this function
    let mut final_instructions: Vec<StateInstructions> = vec![];
    let list_of_instructions = generate_instructions_from_existing_status_conditions(
        state,
        &attacking_side,
        incoming_instructions,
        &mut final_instructions,
    );

    let mut next_instructions = vec![];
    for instruction in list_of_instructions {
        state.apply_instructions(&instruction.instruction_list);
        if cannot_use_move(state, &choice, &attacking_side) {
            state.reverse_instructions(&instruction.instruction_list);
            final_instructions.push(instruction);
        } else {
            state.reverse_instructions(&instruction.instruction_list);
            next_instructions.push(instruction);
        }
    }

    let mut continuing_instructions: Vec<StateInstructions> = vec![];
    for instruction in next_instructions {
        continuing_instructions.push(generate_instructions_from_move_special_effect(
            state,
            &choice,
            &attacking_side,
            instruction,
        ));
    }
    next_instructions = continuing_instructions;

    let mut move_hit_instructions: Vec<StateInstructions> = vec![];
    for mut instruction in next_instructions {
        move_hit_instructions.push(check_move_hit_or_miss(
            state,
            &choice,
            &attacking_side,
            &mut instruction,
            &mut final_instructions,
        ))
    }

    next_instructions = move_hit_instructions;

    // Damage generation gets its own block because it has some special logic
    if let Some(damages_dealt) = damage {
        let mut temp_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            let num_damage_amounts = damages_dealt.len() as f32;
            for dmg in &damages_dealt {
                let mut this_instruction = instruction.clone();
                this_instruction.update_percentage(1.0 / num_damage_amounts);
                println!("Instruction: {:?}, Run dmg: {:?}", this_instruction, dmg);
                temp_instructions.extend(generate_instructions_from_damage(
                    state,
                    &choice,
                    *dmg,
                    &attacking_side,
                    this_instruction,
                ));
            }
        }
        next_instructions = temp_instructions;
    }

    if choice.flags.drag {
        for ins in next_instructions {
            get_instructions_from_drag(state, &attacking_side, ins, &mut final_instructions);
        }
        return combine_duplicate_instructions(final_instructions);
    }

    if let Some(side_condition) = &choice.side_condition {
        let mut continuing_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            continuing_instructions.push(generate_instructions_from_side_conditions(
                state,
                side_condition,
                &attacking_side,
                instruction,
            ));
        }
        next_instructions = continuing_instructions;
    }
    if let Some(hazard_clear) = &choice.hazard_clear {
        let mut continuing_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            continuing_instructions.push(get_instructions_from_hazard_clearing_moves(
                state,
                hazard_clear,
                &attacking_side,
                instruction,
            ));
        }
        next_instructions = continuing_instructions;
    }
    if let Some(volatile_status) = &choice.volatile_status {
        let mut continuing_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            continuing_instructions.push(get_instructions_from_volatile_statuses(
                state,
                volatile_status,
                &attacking_side,
                instruction,
            ));
        }
        next_instructions = continuing_instructions;
    }
    if let Some(status) = &choice.status {
        let mut continuing_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            continuing_instructions.push(get_instructions_from_status_effects(
                state,
                status,
                &attacking_side,
                instruction,
            ));
        }
        next_instructions = continuing_instructions;
    }
    if let Some(boost) = &choice.boost {
        let mut continuing_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            continuing_instructions.push(get_instructions_from_boosts(
                state,
                boost,
                &attacking_side,
                instruction,
            ));
        }
        next_instructions = continuing_instructions;
    }
    if let Some(heal) = &choice.heal {
        let mut continuing_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            continuing_instructions.push(get_instructions_from_heal(
                state,
                heal,
                &attacking_side,
                instruction,
            ));
        }
        next_instructions = continuing_instructions;
    }
    if let Some(secondaries_vec) = &choice.secondaries {
        let mut continuing_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            continuing_instructions.extend(get_instructions_from_secondaries(
                state,
                secondaries_vec,
                &attacking_side,
                instruction,
            ));
        }
        next_instructions = continuing_instructions;
    }

    for instruction in next_instructions {
        final_instructions.push(instruction);
    }

    return combine_duplicate_instructions(final_instructions);
}

fn combine_duplicate_instructions(
    mut list_of_instructions: Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    let mut result = vec![list_of_instructions.remove(0)];

    for instruction_1 in list_of_instructions {
        let mut found_duplicate = false;
        for instruction_2 in result.iter_mut() {
            if instruction_1.instruction_list == instruction_2.instruction_list {
                instruction_2.percentage += instruction_1.percentage;
                found_duplicate = true;
                break;
            }
        }
        if !found_duplicate {
            result.push(instruction_1);
        }
    }

    return result;
}

fn get_effective_speed(state: &State, side_reference: &SideReference) -> i16 {
    let side = state.get_side_immutable(side_reference);
    let active_pkmn = side.get_active_immutable();

    let mut boosted_speed = active_pkmn.calculate_boosted_stat(PokemonBoostableStat::Speed) as f32;

    match state.weather.weather_type {
        Weather::Sun | Weather::HarshSun if active_pkmn.ability.as_str() == "chlorophyll" => {
            boosted_speed *= 2.0
        }
        Weather::Rain | Weather::HeavyRain if active_pkmn.ability.as_str() == "swiftswim" => {
            boosted_speed *= 2.0
        }
        Weather::Sand if active_pkmn.ability.as_str() == "sandrush" => boosted_speed *= 2.0,
        Weather::Hail if active_pkmn.ability.as_str() == "slushrush" => boosted_speed *= 2.0,
        _ => {}
    }

    match active_pkmn.ability.as_str() {
        "surgesurfer" if state.terrain.terrain_type == Terrain::ElectricTerrain => {
            boosted_speed *= 2.0
        }
        "unburden"
            if active_pkmn
                .volatile_statuses
                .contains(&PokemonVolatileStatus::Unburden) =>
        {
            boosted_speed *= 2.0
        }
        "quickfeet" if active_pkmn.status != PokemonStatus::None => boosted_speed *= 1.5,
        _ => {}
    }

    if side.side_conditions.tailwind > 0 {
        boosted_speed *= 2.0
    }

    if active_pkmn.item.as_str() == "choicescarf" {
        boosted_speed *= 1.5
    }

    if active_pkmn.status == PokemonStatus::Paralyze && active_pkmn.ability.as_str() != "quickfeet"
    {
        boosted_speed *= 1.5
    }

    return boosted_speed as i16;
}

fn get_effective_priority(state: &State, side_reference: &SideReference, choice: &Choice) -> i8 {
    let mut priority = choice.priority;
    let side = state.get_side_immutable(side_reference);
    let active_pkmn = side.get_active_immutable();

    match active_pkmn.ability.as_str() {
        "prankster" if choice.category == MoveCategory::Status => priority += 1,
        "galewings"
            if choice.move_type == PokemonType::Flying && active_pkmn.hp == active_pkmn.maxhp =>
        {
            priority += 1
        }
        "triage" if choice.flags.heal => priority += 3,
        _ => {}
    }

    return priority;
}

fn side_one_moves_first(state: &State, side_one_choice: &Choice, side_two_choice: &Choice) -> bool {
    let side_one_effective_speed = get_effective_speed(&state, &SideReference::SideOne);
    let side_two_effective_speed = get_effective_speed(&state, &SideReference::SideTwo);

    if side_one_choice.category == MoveCategory::Switch
        && side_two_choice.category == MoveCategory::Switch
    {
        return side_one_effective_speed > side_two_effective_speed;
    } else if side_one_choice.category == MoveCategory::Switch {
        return side_two_choice.move_id != String::from("pursuit");
    } else if side_two_choice.category == MoveCategory::Switch {
        return side_one_choice.move_id == String::from("pursuit");
    }

    let side_one_effective_priority =
        get_effective_priority(&state, &SideReference::SideOne, &side_one_choice);
    let side_two_effective_priority =
        get_effective_priority(&state, &SideReference::SideTwo, &side_two_choice);

    return if side_one_effective_priority == side_two_effective_priority {
        match state.trick_room {
            true => side_one_effective_speed < side_two_effective_speed,
            false => side_one_effective_speed > side_two_effective_speed,
        }
    } else {
        side_one_effective_priority > side_two_effective_priority
    };
}

fn get_end_of_turn_instructions(
    state: &mut State,
    mut incoming_instructions: StateInstructions,
    _side_one_choice: &Choice,
    _side_two_choice: &Choice,
    first_move_side: &SideReference,
) -> StateInstructions {
    /*

    Methodology:
        This function is deterministic and will not branch.
        It will apply instructions to the state as it goes, and then reverse them at the end.

     TODO:
        - weather damage: sand & hail
        - futuresight damage
        - wish healing
        - items (item callback fn)
        - abilities (ability callback fn)
        - statuses: poison, toxic, burn damage
        - leechseed sap
        - volatile statuses that cause instructions:
            - flinch (removing it)
            - roost (removing it)
            - protect & variants (removing it & incrementing the protect side-condition)
            - partiallytrapped (% damage)
            - saltcure
    */

    state.apply_instructions(&incoming_instructions.instruction_list);

    let sides = [first_move_side, &first_move_side.get_other_side()];

    // Weather Damage
    for side_ref in sides {
        let side = state.get_side_immutable(side_ref);
        let active_pkmn = side.get_active_immutable();

        if active_pkmn.hp == 0 || active_pkmn.ability.as_str() == "magicguard" {
            continue;
        }

        match state.weather.weather_type {
            Weather::Hail => {
                let hail_damage = cmp::min((active_pkmn.maxhp as f32 * 0.0625) as i16, active_pkmn.hp);
                let hail_damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: *side_ref,
                    damage_amount: hail_damage,
                });
                state.apply_one_instruction(&hail_damage_instruction);
                incoming_instructions
                    .instruction_list
                    .push(hail_damage_instruction);
            }
            Weather::Sand => {
                let sand_damage = cmp::min((active_pkmn.maxhp as f32 * 0.0625) as i16, active_pkmn.hp);
                let sand_damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: *side_ref,
                    damage_amount: sand_damage,
                });
                state.apply_one_instruction(&sand_damage_instruction);
                incoming_instructions
                    .instruction_list
                    .push(sand_damage_instruction);
            }
            _ => {}
        }
    }

    // futuresight damage
    // for side_ref in sides {}

    state.reverse_instructions(&incoming_instructions.instruction_list);

    return incoming_instructions;
}

pub fn generate_instructions_from_move_pair(
    state: &mut State,
    side_one_move: &String,
    side_two_move: &String,
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

    let mut side_one_choice = MOVES.get(side_one_move).unwrap().to_owned();
    let mut side_two_choice = MOVES.get(side_two_move).unwrap().to_owned();

    let mut state_instruction_vec: Vec<StateInstructions> = vec![];
    let incoming_instructions: StateInstructions = StateInstructions::default();
    if side_one_moves_first(&state, &side_one_choice, &side_two_choice) {
        let first_move_instructions = generate_instructions_from_move(
            state,
            &mut side_one_choice,
            &side_two_choice,
            SideReference::SideOne,
            incoming_instructions,
        );
        for state_instruction in first_move_instructions {
            state_instruction_vec.extend(generate_instructions_from_move(
                state,
                &mut side_two_choice,
                &side_one_choice,
                SideReference::SideTwo,
                state_instruction,
            ));
        }
    } else {
        let first_move_instructions = generate_instructions_from_move(
            state,
            &mut side_two_choice,
            &side_one_choice,
            SideReference::SideTwo,
            incoming_instructions,
        );
        for state_instruction in first_move_instructions {
            state_instruction_vec.extend(generate_instructions_from_move(
                state,
                &mut side_one_choice,
                &side_two_choice,
                SideReference::SideOne,
                state_instruction,
            ));
        }
    }

    // TODO: End-Of-Turn Instructions

    return state_instruction_vec;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::choices::MOVES;
    use crate::instruction::{
        ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
        ChangeStatusInstruction, ChangeTerrain, DamageInstruction, EnableMoveInstruction,
        SwitchInstruction,
    };
    use crate::state::{Move, PokemonBoostableStat, SideReference, State, Terrain};

    #[test]
    fn test_drag_move_as_second_move_exits_early() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("dragontail").unwrap().to_owned();
        choice.first_move = false;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_electric_move_does_nothing_versus_ground_type() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("thunderbolt").unwrap().to_owned();
        state.side_two.get_active().types = (PokemonType::Ground, PokemonType::Typeless);
        choice.first_move = false;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_grass_type_cannot_have_powder_move_used_against_it() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("spore").unwrap().to_owned(); // Spore is a powder move
        state.side_two.get_active().types = (PokemonType::Grass, PokemonType::Typeless);
        choice.first_move = false;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_spikes_sets_first_layer() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("spikes").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("spikes").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.weather.weather_type = Weather::Hail;
        let mut choice = MOVES.get("auroraveil").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
    fn test_auroa_veil_fails_outside_of_hail() {
        let mut state: State = State::default();
        state.weather.weather_type = Weather::None;
        let mut choice = MOVES.get("auroraveil").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("stealthrock").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("stoneaxe").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
    fn test_100_percent_secondary_volatilestatus() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("chatter").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
                    volatile_status: PokemonVolatileStatus::Confusion,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_possible_secondary_volatilestatus() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("confusion").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 40,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::Confusion,
                    }),
                ],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 40,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_possible_secondary_volatilestatus_with_possible_accuracy() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("axekick").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
                percentage: 27.0000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 100,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::Confusion,
                    }),
                ],
            },
            StateInstructions {
                percentage: 63.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_volatile_status_applied_to_self() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("aquaring").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ApplyVolatileStatus(
                ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::AquaRing,
                },
            )],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_volatile_status_applied_to_opponent() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("attract").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ApplyVolatileStatus(
                ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::Attract,
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
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::Attract);
        let mut choice = MOVES.get("attract").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_substitute_doing_damage_to_user() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 26;
        let mut choice = MOVES.get("substitute").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 25,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_substitute_failing_if_user_has_less_than_25_percent_hp() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 25;
        let mut choice = MOVES.get("substitute").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("whirlwind").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: 0,
                    next_index: 1,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: 0,
                    next_index: 2,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: 0,
                    next_index: 3,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: 0,
                    next_index: 4,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: 0,
                    next_index: 5,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_drag_move_with_fainted_reserve() {
        let mut state: State = State::default();
        state.side_two.pokemon[1].hp = 0;
        state.side_two.pokemon[3].hp = 0;
        let mut choice = MOVES.get("whirlwind").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: 0,
                    next_index: 2,
                })],
            },
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: 0,
                    next_index: 4,
                })],
            },
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: 0,
                    next_index: 5,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_damaging_drag_move_with_fainted_reserve() {
        let mut state: State = State::default();
        state.side_two.pokemon[1].hp = 0;
        state.side_two.pokemon[3].hp = 0;
        let mut choice = MOVES.get("dragontail").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
                        previous_index: 0,
                        next_index: 2,
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
                        previous_index: 0,
                        next_index: 4,
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
                        previous_index: 0,
                        next_index: 5,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_damaging_drag_that_knocks_out_defender() {
        let mut state: State = State::default();
        state.side_two.pokemon[1].hp = 0;
        state.side_two.pokemon[3].hp = 0;
        state.side_two.get_active().hp = 5;
        let mut choice = MOVES.get("dragontail").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_two.pokemon[1].hp = 0;
        state.side_two.pokemon[2].hp = 0;
        state.side_two.pokemon[3].hp = 0;
        state.side_two.pokemon[4].hp = 0;
        state.side_two.pokemon[5].hp = 0;
        let mut choice = MOVES.get("whirlwind").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_two.pokemon[1].hp = 0;
        state.side_two.pokemon[3].hp = 0;
        let mut choice = MOVES.get("whirlwind").unwrap().to_owned();

        let previous_instruction = StateInstructions {
            percentage: 50.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            })],
        };

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            previous_instruction,
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
                        previous_index: 0,
                        next_index: 2,
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
                        previous_index: 0,
                        next_index: 4,
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
                        previous_index: 0,
                        next_index: 5,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_status_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("glare").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: 0,
                old_status: PokemonStatus::None,
                new_status: PokemonStatus::Paralyze,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_status_move_that_can_miss() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("thunderwave").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
                    pokemon_index: 0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Paralyze,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_status_move_that_can_miss_but_is_blocked_by_ability() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = String::from("limber");
        let mut choice = MOVES.get("thunderwave").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_two.get_active().ability = String::from("flamebody");
        let mut choice = MOVES.get("tackle").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 30.0000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: 0,
                        old_status: PokemonStatus::None,
                        new_status: PokemonStatus::Burn,
                    }),
                ],
            },
            StateInstructions {
                percentage: 70.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_protectivepads_stops_flamebody() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = String::from("flamebody");
        state.side_one.get_active().item = String::from("protectivepads");
        let mut choice = MOVES.get("tackle").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_two.get_active().ability = String::from("flamebody");
        let mut choice = MOVES.get("watergun").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_one.get_active().types.0 = PokemonType::Fire;
        state.side_two.get_active().ability = String::from("flamebody");
        let mut choice = MOVES.get("watergun").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("firefang").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 5.00000095,
                instruction_list: vec![],
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
                        pokemon_index: 0,
                        old_status: PokemonStatus::None,
                        new_status: PokemonStatus::Burn,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::Flinch,
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
                        pokemon_index: 0,
                        old_status: PokemonStatus::None,
                        new_status: PokemonStatus::Burn,
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
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::Flinch,
                    }),
                ],
            },
            StateInstructions {
                percentage: 76.9499969,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = String::from("flamebody");
        let mut choice = MOVES.get("tackle").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 30.000002,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: 0,
                        old_status: PokemonStatus::None,
                        new_status: PokemonStatus::Burn,
                    }),
                ],
            },
            StateInstructions {
                percentage: 70.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_creating_a_move_with_multiple_secondaries() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = String::from("flamebody");
        let mut choice = MOVES.get("firepunch").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 3.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 60,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: 0,
                        old_status: PokemonStatus::None,
                        new_status: PokemonStatus::Burn,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: 0,
                        old_status: PokemonStatus::None,
                        new_status: PokemonStatus::Burn,
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
                        pokemon_index: 0,
                        old_status: PokemonStatus::None,
                        new_status: PokemonStatus::Burn,
                    }),
                ],
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
                        pokemon_index: 0,
                        old_status: PokemonStatus::None,
                        new_status: PokemonStatus::Burn,
                    }),
                ],
            },
            StateInstructions {
                percentage: 63.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 60,
                })],
            },
        ];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_substitute_does_not_block_rest() {
        let mut state: State = State::default();
        state
            .side_one
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::Substitute);
        state.side_one.get_active().hp = state.side_one.get_active().maxhp - 1;
        let mut choice = MOVES.get("rest").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: 0,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Sleep,
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
        let mut choice = MOVES.get("recover").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("recover").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("explosion").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: -100,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_negative_heal_move_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 1;
        let mut choice = MOVES.get("explosion").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: -1,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_heal_move_does_not_overheal() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 55;
        let mut choice = MOVES.get("recover").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("swordsdance").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_one.get_active().attack_boost = 5;
        let mut choice = MOVES.get("swordsdance").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_one.get_active().attack_boost = 6;
        let mut choice = MOVES.get("swordsdance").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("kinesis").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("charm").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_two.get_active().attack_boost = -5;
        let mut choice = MOVES.get("charm").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_two.get_active().attack_boost = -6;
        let mut choice = MOVES.get("charm").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_two.get_active().ability = String::from("clearbody");
        let mut choice = MOVES.get("charm").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.side_one.get_active().ability = String::from("clearbody");
        let mut choice = MOVES.get("shellsmash").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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

        let mut choice = MOVES.get("defog").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        state.terrain.terrain_type = Terrain::ElectricTerrain;
        state.terrain.turns_remaining = 1;

        let mut choice = MOVES.get("defog").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeTerrain(ChangeTerrain {
                new_terrain: Terrain::None,
                new_terrain_turns_remaining: 0,
                previous_terrain: Terrain::ElectricTerrain,
                previous_terrain_turns_remaining: 1,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_defog_clears_terrain_and_side_conditions() {
        let mut state: State = State::default();
        state.terrain.terrain_type = Terrain::ElectricTerrain;
        state.terrain.turns_remaining = 1;
        state.side_one.side_conditions.reflect = 1;
        state.side_two.side_conditions.reflect = 1;

        let mut choice = MOVES.get("defog").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeTerrain(ChangeTerrain {
                    new_terrain: Terrain::None,
                    new_terrain_turns_remaining: 0,
                    previous_terrain: Terrain::ElectricTerrain,
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
    fn test_rapidspin_clears_hazards() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;

        let mut choice = MOVES.get("rapidspin").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
    fn test_rapidspin_clears_multiple_hazards() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.side_conditions.spikes = 3;
        state.side_one.side_conditions.sticky_web = 1;

        let mut choice = MOVES.get("rapidspin").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
    fn test_rapidspin_does_not_clear_opponent_hazards() {
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        state.side_two.side_conditions.toxic_spikes = 2;
        state.side_two.side_conditions.spikes = 3;
        state.side_two.side_conditions.sticky_web = 1;

        let mut choice = MOVES.get("rapidspin").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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

        let mut choice = MOVES.get("courtchange").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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

        let mut choice = MOVES.get("courtchange").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("stoneaxe").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("tackle").unwrap().to_owned();
        state
            .side_one
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::Flinch);

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_taunted_pokemon_cannot_use_status_move() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("tackle").unwrap().to_owned();
        state
            .side_one
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::Taunt);

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_pokemon_taunted_on_first_turn_cannot_use_status_move() {
        let mut state: State = State::default();
        state
            .side_one
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::Taunt);

        let mut choice = MOVES.get("tackle").unwrap().to_owned();
        choice.first_move = false;

        let mut incoming_instructions = StateInstructions::default();
        incoming_instructions
            .instruction_list
            .push(Instruction::ApplyVolatileStatus(
                ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::Taunt,
                },
            ));

        let original_incoming_instructions = incoming_instructions.clone();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            incoming_instructions,
        );
        assert_eq!(instructions, vec![original_incoming_instructions])
    }

    #[test]
    fn test_dead_pokemon_moving_second_does_nothing() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("tackle").unwrap().to_owned();
        choice.first_move = false;
        state.side_one.get_active().hp = 0;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_cannot_ohko_versus_study() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("earthquake").unwrap().to_owned();
        state.side_two.get_active().ability = String::from("sturdy");
        state.side_two.get_active().hp = 50;
        state.side_two.get_active().maxhp = 50;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("earthquake").unwrap().to_owned();
        state.side_two.get_active().ability = String::from("sturdy");
        state.side_two.get_active().hp = 45;
        state.side_two.get_active().maxhp = 50;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("tackle").unwrap().to_owned();
        state.side_one.get_active().ability = String::from("beastboost");
        state.side_one.get_active().attack = 500; // highest stat
        state.side_two.get_active().hp = 1;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
    fn test_beastboost_does_not_boost_without_kill() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("tackle").unwrap().to_owned();
        state.side_one.get_active().ability = String::from("beastboost");
        state.side_one.get_active().attack = 150; // highest stat
        state.side_two.get_active().hp = 100;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("absorb").unwrap().to_owned();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().maxhp = 200;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("absorb").unwrap().to_owned();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().maxhp = 105;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("bravebird").unwrap().to_owned();
        state.side_one.get_active().hp = 105;
        state.side_one.get_active().maxhp = 105;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("bravebird").unwrap().to_owned();
        state.side_one.get_active().hp = 5;
        state.side_one.get_active().maxhp = 105;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("absorb").unwrap().to_owned();
        choice.recoil = Some(0.33);
        state.side_one.get_active().hp = 1;
        state.side_one.get_active().maxhp = 105;

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
        let mut choice = MOVES.get("jumpkick").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
    fn test_crash_move_missing_cannot_overkill() {
        let mut state: State = State::default();
        state.get_side(&SideReference::SideOne).get_active().hp = 5;
        let mut choice = MOVES.get("jumpkick").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
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
    fn test_knockoff_removing_item() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("knockoff").unwrap().to_owned();
        state.get_side(&SideReference::SideTwo).get_active().item = String::from("item");

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                }),
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: SideReference::SideTwo,
                    current_item: "item".to_string(),
                    new_item: "".to_string(),
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_blunderpolicy_boost() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("crosschop").unwrap().to_owned();
        state.get_side(&SideReference::SideOne).get_active().item = String::from("blunderpolicy");

        let instructions = generate_instructions_from_move(
            &mut state,
            &mut choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 19.999998,
                instruction_list: vec![
                    Instruction::ChangeItem(ChangeItemInstruction {
                        side_ref: SideReference::SideOne,
                        current_item: "blunderpolicy".to_string(),
                        new_item: "".to_string(),
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

        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            StateInstructions::default(),
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_volatile_statuses() {
        let mut state: State = State::default();
        state
            .side_one
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::LeechSeed);
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::LeechSeed,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            StateInstructions::default(),
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
        choice.switch_id = 1;

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
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            StateInstructions::default(),
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_boost() {
        let mut state: State = State::default();
        state.side_one.get_active().attack_boost = 2;
        state.side_one.get_active().speed_boost = 5;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

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
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            StateInstructions::default(),
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_disabled_move() {
        let mut state: State = State::default();
        state.side_one.get_active().moves = vec![
            Move {
                id: "disabled move".to_string(),
                disabled: true,
                pp: 32,
            },
            Move {
                id: "not disabled move".to_string(),
                disabled: false,
                pp: 32,
            },
        ];
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 0,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            StateInstructions::default(),
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_multiple_disabled_moves() {
        let mut state: State = State::default();
        state.side_one.get_active().moves = vec![
            Move {
                id: "disabled move".to_string(),
                disabled: true,
                pp: 32,
            },
            Move {
                id: "also disabled move".to_string(),
                disabled: true,
                pp: 32,
            },
            Move {
                id: "not disabled move".to_string(),
                disabled: false,
                pp: 32,
            },
            Move {
                id: "also also disabled move".to_string(),
                disabled: true,
                pp: 32,
            },
        ];
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 0,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 1,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 3,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            StateInstructions::default(),
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

        choice.switch_id = 1;
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
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator() {
        let mut state: State = State::default();
        state.side_one.get_active().hp -= 10;
        state.side_one.get_active().ability = String::from("regenerator");
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 10,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator_plus_move_enabling() {
        let mut state: State = State::default();
        state.side_one.get_active().moves = vec![
            Move {
                id: "disabled move".to_string(),
                disabled: true,
                pp: 32,
            },
            Move {
                id: "also disabled move".to_string(),
                disabled: true,
                pp: 32,
            },
            Move {
                id: "not disabled move".to_string(),
                disabled: false,
                pp: 32,
            },
            Move {
                id: "also also disabled move".to_string(),
                disabled: true,
                pp: 32,
            },
        ];
        state.side_one.get_active().hp -= 10;
        state.side_one.get_active().ability = String::from("regenerator");
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 0,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 1,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: 3,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 10,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator_but_no_damage_taken() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = String::from("regenerator");
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_fainted_pokemon_with_regenerator_does_not_heal() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = String::from("regenerator");
        state.side_one.get_active().hp = 0;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_regenerator_only_heals_one_third() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = String::from("regenerator");
        state.side_one.get_active().hp = 3;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 33,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_naturalcure() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = String::from("naturalcure");
        state.side_one.get_active().status = PokemonStatus::Paralyze;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: 0,
                    old_status: PokemonStatus::Paralyze,
                    new_status: PokemonStatus::None,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_naturalcure_with_no_status() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = String::from("naturalcure");
        state.side_one.get_active().status = PokemonStatus::None;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: state.side_one.get_active().hp / 8,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.pokemon[1].hp = 5;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stickyweb() {
        let mut state: State = State::default();
        state.side_one.side_conditions.sticky_web = 1;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: -1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stickyweb_with_heavydutyboots() {
        let mut state: State = State::default();
        state.side_one.side_conditions.sticky_web = 1;
        state.side_one.pokemon[1].item = String::from("heavydutyboots");
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_single_layer_toxicspikes() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 1;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: 1,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Poison,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: 1,
                    old_status: PokemonStatus::None,
                    new_status: PokemonStatus::Toxic,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_flying_type() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.pokemon[1].types.0 = PokemonType::Flying;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_poison_and_flying_type() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.pokemon[1].types.0 = PokemonType::Flying;
        state.side_one.pokemon[1].types.1 = PokemonType::Poison;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate() {
        let mut state: State = State::default();
        state.side_one.pokemon[1].ability = String::from("intimidate");
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Attack,
                    amount: -1,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate_when_opponent_is_already_lowest_atk_boost() {
        let mut state: State = State::default();
        state.side_one.pokemon[1].ability = String::from("intimidate");
        state.side_two.get_active().attack_boost = -6;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate_versus_clearbody() {
        let mut state: State = State::default();
        state.side_one.pokemon[1].ability = String::from("intimidate");
        state.side_two.get_active().ability = String::from("clearbody");
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_poison_type() {
        let mut state: State = State::default();
        state.side_one.pokemon[1].types.0 = PokemonType::Poison;
        state.side_one.side_conditions.toxic_spikes = 2;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: -2,
                }),
            ],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_and_spikes_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.spikes = 1;
        state.side_one.pokemon[1].hp = 15;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
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

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_and_multiple_layers_of_spikes_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.spikes = 3;
        state.side_one.pokemon[1].hp = 25;
        let incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
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

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_healthy_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        let incoming_instructions = StateInstructions::default();

        let expected_instructions = vec![StateInstructions::default()];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            &mut vec![],
        );

        assert_eq!(expected_instructions, actual_instructions);
    }

    #[test]
    fn test_paralyzed_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Paralyze;
        let incoming_instructions = StateInstructions::default();

        let expected_instructions = vec![StateInstructions {
            percentage: 75.0,
            instruction_list: vec![],
        }];

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 25.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, actual_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_frozen_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Freeze;
        let incoming_instructions = StateInstructions::default();

        let expected_instructions = vec![StateInstructions {
            percentage: 20.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: PokemonStatus::Freeze,
                new_status: PokemonStatus::None,
            })],
        }];

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 80.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, actual_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Sleep;
        let incoming_instructions = StateInstructions::default();

        let expected_instructions = vec![StateInstructions {
            percentage: 33.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            })],
        }];

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 67.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, actual_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_paralyzed_pokemon_preserves_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Paralyze;
        let mut incoming_instructions = StateInstructions::default();
        incoming_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })];

        let expected_instructions = vec![StateInstructions {
            percentage: 75.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        }];

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 25.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        }];

        let frozen_instructions = &mut vec![];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, actual_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_previous_instruction_removing_paralysis_stops_the_branch() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Paralyze; // pokemon is paralyzed

        let mut incoming_instructions = StateInstructions::default();
        // there is an incoming instruction to remove the paralysis
        incoming_instructions.instruction_list =
            vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: state.side_one.get_active_immutable().status,
                new_status: PokemonStatus::None,
            })];

        // expected instructions are the incoming instructions, since the paralysis check should
        // fail
        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: state.side_one.get_active_immutable().status,
                new_status: PokemonStatus::None,
            })],
            ..Default::default()
        }];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            &mut vec![],
        );

        assert_eq!(expected_instructions, actual_instructions);
    }

    #[test]
    fn test_basic_side_two_moves_first() {
        let mut state = State::default();
        let side_one_choice = MOVES.get("tackle").unwrap().to_owned();
        let side_two_choice = MOVES.get("tackle").unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            false,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_basic_side_one_moves_first() {
        let mut state = State::default();
        let side_one_choice = MOVES.get("tackle").unwrap().to_owned();
        let side_two_choice = MOVES.get("tackle").unwrap().to_owned();
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_speed_tie_goes_to_side_two() {
        let mut state = State::default();
        let side_one_choice = MOVES.get("tackle").unwrap().to_owned();
        let side_two_choice = MOVES.get("tackle").unwrap().to_owned();
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
        let side_one_choice = MOVES.get("quickattack").unwrap().to_owned();
        let side_two_choice = MOVES.get("tackle").unwrap().to_owned();
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
        let side_one_choice = MOVES.get("tackle").unwrap().to_owned();
        let side_two_choice = MOVES.get("quickattack").unwrap().to_owned();
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
        let side_one_choice = MOVES.get("quickattack").unwrap().to_owned();
        let side_two_choice = MOVES.get("quickattack").unwrap().to_owned();
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
        let mut side_one_choice = MOVES.get("splash").unwrap().to_owned();
        side_one_choice.category = MoveCategory::Switch;
        let side_two_choice = MOVES.get("quickattack").unwrap().to_owned();
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
        let mut side_one_choice = MOVES.get("splash").unwrap().to_owned();
        side_one_choice.category = MoveCategory::Switch;
        let mut side_two_choice = MOVES.get("splash").unwrap().to_owned();
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
        let side_one_choice = MOVES.get("pursuit").unwrap().to_owned();
        let mut side_two_choice = MOVES.get("splash").unwrap().to_owned();
        side_two_choice.category = MoveCategory::Switch;

        state.side_one.get_active().speed = 50;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            true,
            side_one_moves_first(&state, &side_one_choice, &side_two_choice)
        )
    }

    #[test]
    fn test_basic_move_pair_instruction_generation() {
        let mut state = State::default();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 50;
        let side_one_move = String::from("tackle");
        let side_two_move = String::from("tackle");

        let vec_of_instructions =
            generate_instructions_from_move_pair(&mut state, &side_one_move, &side_two_move);

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
            ],
        }];

        assert_eq!(expected_instructions, vec_of_instructions)
    }

    #[test]
    fn test_move_pair_instruction_generation_where_first_move_branches() {
        let mut state = State::default();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 50;
        let side_one_move = String::from("playrough");
        let side_two_move = String::from("tackle");

        let vec_of_instructions =
            generate_instructions_from_move_pair(&mut state, &side_one_move, &side_two_move);

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                })],
            },
            StateInstructions {
                percentage: 9.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 71,
                    }),
                    Instruction::Boost(BoostInstruction {
                        side_ref: SideReference::SideTwo,
                        stat: PokemonBoostableStat::Attack,
                        amount: -1,
                    }),
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        // playrough lowered attack means this does less dmg than other branches
                        damage_amount: 33,
                    }),
                ],
            },
            StateInstructions {
                percentage: 81.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 71,
                    }),
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 48,
                    }),
                ],
            },
        ];

        assert_eq!(expected_instructions, vec_of_instructions)
    }

    #[test]
    fn test_move_pair_instruction_generation_where_second_move_branches() {
        let mut state = State::default();
        state.side_one.get_active().speed = 50;
        state.side_two.get_active().speed = 100;
        let side_one_move = String::from("playrough");
        let side_two_move = String::from("tackle");

        let vec_of_instructions =
            generate_instructions_from_move_pair(&mut state, &side_one_move, &side_two_move);

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                })],
            },
            StateInstructions {
                percentage: 9.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 48,
                    }),
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 71,
                    }),
                    Instruction::Boost(BoostInstruction {
                        side_ref: SideReference::SideTwo,
                        stat: PokemonBoostableStat::Attack,
                        amount: -1,
                    }),
                ],
            },
            StateInstructions {
                percentage: 81.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 48,
                    }),
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 71,
                    }),
                ],
            },
        ];

        assert_eq!(expected_instructions, vec_of_instructions)
    }

    #[test]
    fn test_end_of_turn_hail_damage() {
        let mut state = State::default();
        state.weather.weather_type = Weather::Hail;
        let side_one_move = MOVES.get("tackle").unwrap().to_owned();
        let side_two_move = MOVES.get("tackle").unwrap().to_owned();

        let vec_of_instructions = get_end_of_turn_instructions(
            &mut state,
            StateInstructions::default(),
            &side_one_move,
            &side_two_move,
            &SideReference::SideOne
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
                })
            ],
        };

        assert_eq!(expected_instructions, vec_of_instructions)
    }

    #[test]
    fn test_hail_does_not_overkill() {
        let mut state = State::default();
        state.weather.weather_type = Weather::Hail;
        state.side_one.get_active().hp = 3;
        let side_one_move = MOVES.get("tackle").unwrap().to_owned();
        let side_two_move = MOVES.get("tackle").unwrap().to_owned();

        let vec_of_instructions = get_end_of_turn_instructions(
            &mut state,
            StateInstructions::default(),
            &side_one_move,
            &side_two_move,
            &SideReference::SideOne
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
                })
            ],
        };

        assert_eq!(expected_instructions, vec_of_instructions)
    }

    #[test]
    fn test_fainted_pkmn_does_not_take_hail_dmg() {
        let mut state = State::default();
        state.weather.weather_type = Weather::Hail;
        state.side_one.get_active().hp = 0;
        let side_one_move = MOVES.get("tackle").unwrap().to_owned();
        let side_two_move = MOVES.get("tackle").unwrap().to_owned();

        let vec_of_instructions = get_end_of_turn_instructions(
            &mut state,
            StateInstructions::default(),
            &side_one_move,
            &side_two_move,
            &SideReference::SideOne
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                })
            ],
        };

        assert_eq!(expected_instructions, vec_of_instructions)
    }
}
