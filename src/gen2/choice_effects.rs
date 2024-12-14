use crate::choices::{Choice, Choices, Heal, MoveCategory, MoveTarget};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::{add_remove_status_instructions, get_boost_amount};
use crate::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeSideConditionInstruction, ChangeStatusInstruction, ChangeSubsituteHealthInstruction,
    ChangeWeather, DamageInstruction, HealInstruction, Instruction, SetFutureSightInstruction,
    SetSleepTurnsInstruction, StateInstructions,
};
use crate::items::{get_choice_move_disable_instructions, Items};
use crate::state::{
    pokemon_index_iter, PokemonBoostableStat, PokemonSideCondition, PokemonStatus, PokemonType,
    PokemonVolatileStatus, SideReference, State, Weather,
};
use std::cmp;

pub fn modify_choice(
    state: &State,
    attacker_choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, _defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    match attacker_choice.move_id {
        Choices::REVERSAL => {
            let attacker = attacking_side.get_active_immutable();
            let hp_ratio = attacker.hp as f32 / attacker.maxhp as f32;
            if hp_ratio >= 0.688 {
                attacker_choice.base_power = 20.0;
            } else if hp_ratio >= 0.354 {
                attacker_choice.base_power = 40.0;
            } else if hp_ratio >= 0.208 {
                attacker_choice.base_power = 80.0;
            } else if hp_ratio >= 0.104 {
                attacker_choice.base_power = 100.0;
            } else if hp_ratio >= 0.042 {
                attacker_choice.base_power = 150.0;
            } else {
                attacker_choice.base_power = 200.0;
            }
        }
        Choices::EXPLOSION | Choices::SELFDESTRUCT => {
            attacker_choice.base_power *= 2.0;
        }
        Choices::MORNINGSUN | Choices::MOONLIGHT | Choices::SYNTHESIS => {
            match state.weather.weather_type {
                Weather::SUN => {
                    attacker_choice.heal = Some(Heal {
                        target: MoveTarget::User,
                        amount: 0.667,
                    })
                }
                Weather::NONE => {}
                _ => {
                    attacker_choice.heal = Some(Heal {
                        target: MoveTarget::User,
                        amount: 0.25,
                    })
                }
            }
        }
        Choices::PROTECT => {
            if attacking_side.side_conditions.protect > 0 {
                // for now, the engine doesn't support consecutive protects
                // 2nd protect will always fail
                attacker_choice.volatile_status = None;
            }
        }
        Choices::PURSUIT => {
            if defender_choice.category == MoveCategory::Switch {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::SOLARBEAM => {
            if state.weather_is_active(&Weather::SUN) || state.weather_is_active(&Weather::HARSHSUN)
            {
                attacker_choice.flags.charge = false;
            } else if !state.weather_is_active(&Weather::SUN)
                && state.weather.weather_type != Weather::NONE
            {
                attacker_choice.base_power /= 2.0;
            }
        }
        Choices::THUNDER => {
            if state.weather_is_active(&Weather::RAIN)
                || state.weather_is_active(&Weather::HEAVYRAIN)
            {
                attacker_choice.accuracy = 100.0;
            } else if state.weather_is_active(&Weather::SUN)
                || state.weather_is_active(&Weather::HARSHSUN)
            {
                attacker_choice.accuracy = 50.0;
            }
        }
        _ => {}
    }
}

pub fn choice_after_damage_hit(
    state: &mut State,
    choice: &Choice,
    attacking_side_ref: &SideReference,
    instructions: &mut StateInstructions,
    hit_sub: bool,
) {
    let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
    if choice.flags.recharge {
        let instruction = Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
            side_ref: attacking_side_ref.clone(),
            volatile_status: PokemonVolatileStatus::MUSTRECHARGE,
        });
        instructions.instruction_list.push(instruction);
        attacking_side
            .volatile_statuses
            .insert(PokemonVolatileStatus::MUSTRECHARGE);
    }
    match choice.move_id {
        Choices::THIEF => {
            let attacker_active = attacking_side.get_active();
            let defender_active = defending_side.get_active();
            if defender_active.item_can_be_removed()
                && defender_active.item != Items::NONE
                && attacker_active.item == Items::NONE
                && !hit_sub
            {
                let defender_item = defender_active.item;

                let instruction = Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    current_item: defender_item,
                    new_item: Items::NONE,
                });
                instructions.instruction_list.push(instruction);
                defender_active.item = Items::NONE;

                let instruction = Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: *attacking_side_ref,
                    current_item: Items::NONE,
                    new_item: defender_item,
                });
                instructions.instruction_list.push(instruction);
                attacker_active.item = defender_item;
            }
        }
        _ => {}
    }
}

pub fn choice_before_move(
    state: &mut State,
    choice: &mut Choice,
    attacking_side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let attacking_side = state.get_side(attacking_side_ref);
    let attacker = attacking_side.get_active();

    match choice.move_id {
        Choices::FUTURESIGHT => {
            choice.remove_all_effects();
            if attacking_side.future_sight.0 == 0 {
                instructions
                    .instruction_list
                    .push(Instruction::SetFutureSight(SetFutureSightInstruction {
                        side_ref: *attacking_side_ref,
                        pokemon_index: attacking_side.active_index,
                        previous_pokemon_index: attacking_side.future_sight.1,
                    }));
                attacking_side.future_sight = (3, attacking_side.active_index);
            }
        }
        Choices::EXPLOSION | Choices::SELFDESTRUCT => {
            let damage_amount = attacker.hp;
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: *attacking_side_ref,
                    damage_amount,
                }));
            attacker.hp = 0;
        }
        _ => {}
    }
    let attacking_side = state.get_side(attacking_side_ref);
    let attacker = attacking_side.get_active();
    if let Some(choice_volatile_status) = &choice.volatile_status {
        if choice_volatile_status.volatile_status == PokemonVolatileStatus::LOCKEDMOVE
            && choice_volatile_status.target == MoveTarget::User
        {
            let ins =
                get_choice_move_disable_instructions(attacker, attacking_side_ref, &choice.move_id);
            for i in ins {
                state.apply_one_instruction(&i);
                instructions.instruction_list.push(i);
            }
        }
    }
}

pub fn choice_hazard_clear(
    state: &mut State,
    choice: &Choice,
    attacking_side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, _defending_side) = state.get_both_sides(attacking_side_ref);
    match choice.move_id {
        Choices::RAPIDSPIN => {
            if attacking_side.side_conditions.stealth_rock > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: *attacking_side_ref,
                            side_condition: PokemonSideCondition::Stealthrock,
                            amount: -1 * attacking_side.side_conditions.stealth_rock,
                        },
                    ));
                attacking_side.side_conditions.stealth_rock = 0;
            }
            if attacking_side.side_conditions.spikes > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: *attacking_side_ref,
                            side_condition: PokemonSideCondition::Spikes,
                            amount: -1 * attacking_side.side_conditions.spikes,
                        },
                    ));
                attacking_side.side_conditions.spikes = 0;
            }
            if attacking_side.side_conditions.toxic_spikes > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: *attacking_side_ref,
                            side_condition: PokemonSideCondition::ToxicSpikes,
                            amount: -1 * attacking_side.side_conditions.toxic_spikes,
                        },
                    ));
                attacking_side.side_conditions.toxic_spikes = 0;
            }
            if attacking_side.side_conditions.sticky_web > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: *attacking_side_ref,
                            side_condition: PokemonSideCondition::StickyWeb,
                            amount: -1 * attacking_side.side_conditions.sticky_web,
                        },
                    ));
                attacking_side.side_conditions.sticky_web = 0;
            }
        }
        _ => {}
    }
}

pub fn choice_special_effect(
    state: &mut State,
    choice: &Choice,
    defender_choice: &Choice,
    attacking_side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
    match choice.move_id {
        Choices::BELLYDRUM => {
            let boost_amount = 6 - attacking_side.attack_boost;
            let attacker = attacking_side.get_active();
            if attacker.hp > attacker.maxhp / 2 {
                instructions
                    .instruction_list
                    .push(Instruction::Damage(DamageInstruction {
                        side_ref: *attacking_side_ref,
                        damage_amount: attacker.maxhp / 2,
                    }));
                instructions
                    .instruction_list
                    .push(Instruction::Boost(BoostInstruction {
                        side_ref: *attacking_side_ref,
                        stat: PokemonBoostableStat::Attack,
                        amount: boost_amount,
                    }));
                attacker.hp -= attacker.maxhp / 2;
                attacking_side.attack_boost = 6;
            } else {
                let boost_amount =
                    get_boost_amount(attacking_side, &PokemonBoostableStat::Attack, 2);
                if boost_amount != 0 {
                    instructions
                        .instruction_list
                        .push(Instruction::Boost(BoostInstruction {
                            side_ref: *attacking_side_ref,
                            stat: PokemonBoostableStat::Attack,
                            amount: boost_amount,
                        }));
                    attacking_side.attack_boost += boost_amount;
                }
            }
        }
        Choices::COUNTER => {
            if (defending_side.damage_dealt.move_category == MoveCategory::Physical
                || defender_choice.move_id.is_hiddenpower())
                && !defending_side
                    .get_active_immutable()
                    .has_type(&PokemonType::GHOST)
            {
                let damage_amount = cmp::min(
                    defending_side.damage_dealt.damage * 2,
                    defending_side.get_active_immutable().hp,
                );
                if damage_amount > 0 {
                    instructions
                        .instruction_list
                        .push(Instruction::Damage(DamageInstruction {
                            side_ref: attacking_side_ref.get_other_side(),
                            damage_amount: damage_amount,
                        }));
                    defending_side.get_active().hp -= damage_amount;
                }
            }
        }
        Choices::MIRRORCOAT => {
            if defending_side.damage_dealt.move_category == MoveCategory::Special
                && !defending_side
                    .get_active_immutable()
                    .has_type(&PokemonType::DARK)
                && !defender_choice.move_id.is_hiddenpower()
            {
                let damage_amount = cmp::min(
                    defending_side.damage_dealt.damage * 2,
                    defending_side.get_active_immutable().hp,
                );
                if damage_amount > 0 {
                    instructions
                        .instruction_list
                        .push(Instruction::Damage(DamageInstruction {
                            side_ref: attacking_side_ref.get_other_side(),
                            damage_amount: damage_amount,
                        }));
                    defending_side.get_active().hp -= damage_amount;
                }
            }
        }
        Choices::HEALBELL => {
            for pkmn_index in pokemon_index_iter() {
                if attacking_side.pokemon[pkmn_index].status != PokemonStatus::NONE {
                    add_remove_status_instructions(
                        instructions,
                        pkmn_index,
                        *attacking_side_ref,
                        attacking_side,
                    );
                }
            }
        }
        Choices::HAZE => {
            state.reset_boosts(&SideReference::SideOne, &mut instructions.instruction_list);
            state.reset_boosts(&SideReference::SideTwo, &mut instructions.instruction_list);
        }
        Choices::REST => {
            let active_index = attacking_side.active_index;
            let active_pkmn = attacking_side.get_active();
            if active_pkmn.hp != active_pkmn.maxhp {
                let heal_amount = active_pkmn.maxhp - active_pkmn.hp;
                if active_pkmn.status != PokemonStatus::SLEEP {
                    instructions
                        .instruction_list
                        .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                            side_ref: *attacking_side_ref,
                            pokemon_index: active_index,
                            old_status: active_pkmn.status,
                            new_status: PokemonStatus::SLEEP,
                        }));
                }
                instructions
                    .instruction_list
                    .push(Instruction::SetRestTurns(SetSleepTurnsInstruction {
                        side_ref: *attacking_side_ref,
                        pokemon_index: active_index,
                        new_turns: 3,
                        previous_turns: active_pkmn.rest_turns,
                    }));
                instructions
                    .instruction_list
                    .push(Instruction::Heal(HealInstruction {
                        side_ref: *attacking_side_ref,
                        heal_amount: heal_amount,
                    }));
                active_pkmn.hp = active_pkmn.maxhp;
                active_pkmn.status = PokemonStatus::SLEEP;
                active_pkmn.rest_turns = 3;
            }
        }
        Choices::SUPERFANG => {
            let target_pkmn = defending_side.get_active();
            if target_pkmn.hp == 1 {
                return;
            }
            if choice.move_id == Choices::SUPERFANG
                && type_effectiveness_modifier(&PokemonType::NORMAL, &target_pkmn) == 0.0
            {
                return;
            }
            let target_hp = target_pkmn.hp / 2;
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    damage_amount: target_pkmn.hp - target_hp,
                }));
            target_pkmn.hp = target_hp;
        }
        Choices::NIGHTSHADE => {
            let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
            let attacker_level = attacking_side.get_active_immutable().level;
            let defender_active = defending_side.get_active();
            if type_effectiveness_modifier(&PokemonType::GHOST, &defender_active) == 0.0 {
                return;
            }

            let damage_amount = cmp::min(attacker_level as i16, defender_active.hp);
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    damage_amount: damage_amount,
                }));
            defender_active.hp -= damage_amount;
        }
        Choices::SEISMICTOSS => {
            let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
            let attacker_level = attacking_side.get_active_immutable().level;
            let defender_active = defending_side.get_active();
            if type_effectiveness_modifier(&PokemonType::NORMAL, &defender_active) == 0.0 {
                return;
            }

            let damage_amount = cmp::min(attacker_level as i16, defender_active.hp);
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    damage_amount: damage_amount,
                }));
            defender_active.hp -= damage_amount;
        }
        Choices::PAINSPLIT => {
            let target_hp = (attacking_side.get_active_immutable().hp
                + defending_side.get_active_immutable().hp)
                / 2;
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: *attacking_side_ref,
                    damage_amount: attacking_side.get_active_immutable().hp - target_hp,
                }));
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    damage_amount: defending_side.get_active_immutable().hp - target_hp,
                }));

            attacking_side.get_active().hp = target_hp;
            defending_side.get_active().hp = target_hp;
        }
        Choices::SUBSTITUTE => {
            if attacking_side
                .volatile_statuses
                .contains(&PokemonVolatileStatus::SUBSTITUTE)
            {
                return;
            }
            let sub_current_health = attacking_side.substitute_health;
            let active_pkmn = attacking_side.get_active();
            let sub_target_health = active_pkmn.maxhp / 4;
            if active_pkmn.hp > sub_target_health {
                let damage_instruction = Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.clone(),
                    damage_amount: sub_target_health,
                });
                let set_sub_health_instruction =
                    Instruction::ChangeSubstituteHealth(ChangeSubsituteHealthInstruction {
                        side_ref: attacking_side_ref.clone(),
                        health_change: sub_target_health - sub_current_health,
                    });
                let apply_vs_instruction =
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: attacking_side_ref.clone(),
                        volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                    });
                active_pkmn.hp -= sub_target_health;
                attacking_side.substitute_health = sub_target_health;
                attacking_side
                    .volatile_statuses
                    .insert(PokemonVolatileStatus::SUBSTITUTE);
                instructions.instruction_list.push(damage_instruction);
                instructions
                    .instruction_list
                    .push(set_sub_health_instruction);
                instructions.instruction_list.push(apply_vs_instruction);
            }
        }
        Choices::PERISHSONG => {
            for side_ref in [SideReference::SideOne, SideReference::SideTwo] {
                let side = state.get_side(&side_ref);
                let pkmn = side.get_active();
                if pkmn.hp != 0
                    && !(side
                        .volatile_statuses
                        .contains(&PokemonVolatileStatus::PERISH4)
                        || side
                            .volatile_statuses
                            .contains(&PokemonVolatileStatus::PERISH3)
                        || side
                            .volatile_statuses
                            .contains(&PokemonVolatileStatus::PERISH2)
                        || side
                            .volatile_statuses
                            .contains(&PokemonVolatileStatus::PERISH1))
                {
                    instructions
                        .instruction_list
                        .push(Instruction::ApplyVolatileStatus(
                            ApplyVolatileStatusInstruction {
                                side_ref: side_ref,
                                volatile_status: PokemonVolatileStatus::PERISH4,
                            },
                        ));
                    side.volatile_statuses
                        .insert(PokemonVolatileStatus::PERISH4);
                }
            }
        }
        Choices::SUNNYDAY => {
            if state.weather.weather_type != Weather::SUN {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::SUN,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::SUN;
                state.weather.turns_remaining = 5;
            }
        }
        Choices::RAINDANCE => {
            if state.weather.weather_type != Weather::RAIN {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::RAIN,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::RAIN;
                state.weather.turns_remaining = 5;
            }
        }
        Choices::SANDSTORM => {
            if state.weather.weather_type != Weather::SAND {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::SAND,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::SAND;
                state.weather.turns_remaining = 5;
            }
        }
        _ => {}
    }
}

pub fn charge_choice_to_volatile(choice: &Choices) -> PokemonVolatileStatus {
    // Panics if you pass a choice that does not have a corresponding volatile status
    match choice {
        Choices::DIG => PokemonVolatileStatus::DIG,
        Choices::FLY => PokemonVolatileStatus::FLY,
        Choices::RAZORWIND => PokemonVolatileStatus::RAZORWIND,
        Choices::SKULLBASH => PokemonVolatileStatus::SKULLBASH,
        Choices::SKYATTACK => PokemonVolatileStatus::SKYATTACK,
        Choices::SOLARBEAM => PokemonVolatileStatus::SOLARBEAM,
        _ => {
            panic!("Invalid choice for charge: {:?}", choice)
        }
    }
}
