use crate::abilities::Abilities;
use crate::choices::{Boost, Choice, Choices, Heal, MoveCategory, MoveTarget, StatBoosts};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::add_remove_status_instructions;
use crate::instruction::{
    ApplyVolatileStatusInstruction, ChangeItemInstruction, ChangeSideConditionInstruction,
    ChangeStatusInstruction, ChangeTerrain, ChangeWeather, DamageInstruction, HealInstruction,
    Instruction, SetRestTurnsInstruction, SetSubstituteHealthInstruction, SetWishInstruction,
    StateInstructions,
};
use crate::items::{get_choice_move_disable_instructions, Items};
use crate::state::{
    pokemon_index_iter, LastUsedMove, PokemonBoostableStat, PokemonSideCondition, PokemonStatus,
    PokemonType, PokemonVolatileStatus, SideReference, State, Terrain, Weather,
};
use std::cmp;

pub fn modify_choice(
    state: &State,
    attacker_choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    match attacker_choice.move_id {
        Choices::BODYPRESS => {
            attacker_choice.base_power *=
                attacking_side.calculate_boosted_stat(PokemonBoostableStat::Defense) as f32
                    / attacking_side.calculate_boosted_stat(PokemonBoostableStat::Attack) as f32;
        }
        Choices::BOLTBEAK | Choices::FISHIOUSREND => {
            if attacker_choice.first_move {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::CLANGOROUSSOUL => {
            let attacker = attacking_side.get_active_immutable();
            if attacker.hp > attacker.maxhp / 3 {
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::User,
                    amount: -0.33,
                });
                attacker_choice.boost = Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 1,
                        accuracy: 0,
                    },
                });
            }
        }
        Choices::EXPANDINGFORCE => {
            if state.terrain.terrain_type == Terrain::PsychicTerrain {
                attacker_choice.base_power *= 1.5;
            }
        }
        Choices::FILLETAWAY => {
            let attacker = attacking_side.get_active_immutable();
            if attacker.hp > attacker.maxhp / 2 {
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::User,
                    amount: -0.5,
                });
                attacker_choice.boost = Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                });
            }
        }
        Choices::FAKEOUT => match attacking_side.last_used_move {
            LastUsedMove::Move(_) => attacker_choice.remove_all_effects(),
            _ => {}
        },
        Choices::GROWTH => {
            if state.weather_is_active(&Weather::Sun) {
                attacker_choice.boost = Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                });
            }
        }
        Choices::HEX => {
            if defending_side.get_active_immutable().status != PokemonStatus::None {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::HYDROSTEAM => {
            if state.weather_is_active(&Weather::Sun) {
                attacker_choice.base_power *= 3.0; // 1.5x for being in sun, 2x for cancelling out rain debuff
            }
        }
        Choices::JUDGMENT => {
            attacker_choice.move_type = attacking_side.get_active_immutable().types.0;
        }
        Choices::MISTYEXPLOSION => {
            if state.terrain.terrain_type == Terrain::MistyTerrain {
                attacker_choice.base_power *= 1.5;
            }
        }
        #[cfg(any(feature = "gen4"))]
        Choices::EXPLOSION | Choices::SELFDESTRUCT => {
            attacker_choice.base_power *= 2.0;
        }

        Choices::MORNINGSUN | Choices::MOONLIGHT | Choices::SYNTHESIS => {
            match state.weather.weather_type {
                Weather::Sun => {
                    attacker_choice.heal = Some(Heal {
                        target: MoveTarget::User,
                        amount: 0.667,
                    })
                }
                Weather::None => {}
                _ => {
                    attacker_choice.heal = Some(Heal {
                        target: MoveTarget::User,
                        amount: 0.25,
                    })
                }
            }
        }
        Choices::NORETREAT => {
            if attacking_side
                .volatile_statuses
                .contains(&PokemonVolatileStatus::NoRetreat)
            {
                attacker_choice.boost = None;
            }
        }
        Choices::POLTERGEIST => {
            if defending_side.get_active_immutable().item == Items::NONE {
                attacker_choice.base_power = 0.0;
            }
        }
        Choices::PROTECT => {
            if attacking_side.side_conditions.protect > 0 {
                // for now, the engine doesn't support consecutive protects
                // 2nd protect will always fail
                attacker_choice.volatile_status = None;
            }
        }
        Choices::PSYBLADE => {
            if state.terrain.terrain_type == Terrain::ElectricTerrain {
                attacker_choice.base_power *= 1.5;
            }
        }
        Choices::PURSUIT => {
            if defender_choice.category == MoveCategory::Switch {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::REVELATIONDANCE => {
            attacker_choice.move_type = attacking_side.get_active_immutable().types.0;
        }
        Choices::RISINGVOLTAGE => {
            if state.terrain.terrain_type == Terrain::ElectricTerrain {
                attacker_choice.base_power *= 1.5;
            }
        }
        Choices::SHOREUP => {
            if state.weather_is_active(&Weather::Sand) {
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.667,
                });
            }
        }
        Choices::STEELROLLER => {
            if state.terrain.terrain_type == Terrain::None {
                attacker_choice.base_power = 0.0;
            }
        }
        Choices::STRENGTHSAP => {
            attacker_choice.boost = Some(Boost {
                target: MoveTarget::Opponent,
                boosts: StatBoosts {
                    attack: -1,
                    defense: 0,
                    special_attack: 0,
                    special_defense: 0,
                    speed: 0,
                    accuracy: 0,
                },
            });
            let defender_attack =
                defending_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
            let attacker_maxhp = attacking_side.get_active_immutable().maxhp;
            attacker_choice.heal = Some(Heal {
                target: MoveTarget::User,
                amount: defender_attack as f32 / attacker_maxhp as f32,
            });
        }
        Choices::TERRAINPULSE => match state.terrain.terrain_type {
            Terrain::ElectricTerrain => {
                attacker_choice.move_type = PokemonType::Electric;
                attacker_choice.base_power *= 2.0;
            }
            Terrain::GrassyTerrain => {
                attacker_choice.move_type = PokemonType::Grass;
                attacker_choice.base_power *= 2.0;
            }
            Terrain::MistyTerrain => {
                attacker_choice.move_type = PokemonType::Fairy;
                attacker_choice.base_power *= 2.0;
            }
            Terrain::PsychicTerrain => {
                attacker_choice.move_type = PokemonType::Psychic;
                attacker_choice.base_power *= 2.0;
            }
            Terrain::None => {}
        },
        Choices::TOXIC => {
            if attacking_side
                .get_active_immutable()
                .has_type(&PokemonType::Poison)
            {
                attacker_choice.accuracy = 100.0;
            }
        }
        Choices::WEATHERBALL => match state.weather.weather_type {
            Weather::Sun | Weather::HarshSun => {
                attacker_choice.base_power = 100.0;
                attacker_choice.move_type = PokemonType::Fire;
            }
            Weather::Rain | Weather::HeavyRain => {
                attacker_choice.base_power = 100.0;
                attacker_choice.move_type = PokemonType::Water;
            }
            Weather::Sand => {
                attacker_choice.base_power = 100.0;
                attacker_choice.move_type = PokemonType::Rock;
            }
            Weather::Hail => {
                attacker_choice.base_power = 100.0;
                attacker_choice.move_type = PokemonType::Ice;
            }
            Weather::None => {}
        },
        Choices::SOLARBEAM => {
            if state.weather_is_active(&Weather::Sun) || state.weather_is_active(&Weather::HarshSun)
            {
                attacker_choice.flags.charge = false;
            }
        }
        Choices::BLIZZARD => {
            if state.weather_is_active(&Weather::Hail) {
                attacker_choice.accuracy = 100.0;
            }
        }
        Choices::HURRICANE | Choices::THUNDER => {
            if state.weather_is_active(&Weather::Rain)
                || state.weather_is_active(&Weather::HeavyRain)
            {
                attacker_choice.accuracy = 100.0;
            } else if state.weather_is_active(&Weather::Sun)
                || state.weather_is_active(&Weather::HarshSun)
            {
                attacker_choice.accuracy = 50.0;
            }
        }

        #[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
        Choices::KNOCKOFF => {
            // Note: Bonus damage still applies if substitute is hit
            let defender = defending_side.get_active_immutable();
            if defender.item_can_be_removed() && defender.item != Items::NONE {
                attacker_choice.base_power *= 1.5;
            }
        }

        Choices::ACROBATICS => {
            if attacking_side.get_active_immutable().item == Items::NONE {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::FOCUSPUNCH => {
            if (defending_side.damage_dealt.move_category == MoveCategory::Physical
                || defending_side.damage_dealt.move_category == MoveCategory::Special)
                && !defending_side.damage_dealt.hit_substitute
                && defending_side.damage_dealt.damage > 0
            {
                attacker_choice.remove_all_effects();
            }
        }
        Choices::ELECTROBALL => {
            let attacker_speed = attacking_side.calculate_boosted_stat(PokemonBoostableStat::Speed);
            let defender_speed = defending_side.calculate_boosted_stat(PokemonBoostableStat::Speed);
            let speed_ratio = attacker_speed as f32 / defender_speed as f32;
            if speed_ratio >= 4.0 {
                attacker_choice.base_power = 150.0;
            } else if speed_ratio >= 3.0 {
                attacker_choice.base_power = 120.0;
            } else if speed_ratio >= 2.0 {
                attacker_choice.base_power = 80.0;
            } else if speed_ratio >= 1.0 {
                attacker_choice.base_power = 60.0;
            } else {
                attacker_choice.base_power = 40.0;
            }
        }
        Choices::GYROBALL => {
            let attacker_speed = attacking_side.calculate_boosted_stat(PokemonBoostableStat::Speed);
            let defender_speed = defending_side.calculate_boosted_stat(PokemonBoostableStat::Speed);

            attacker_choice.base_power =
                ((25.0 * defender_speed as f32 / attacker_speed as f32) + 1.0).min(150.0);
        }
        Choices::AVALANCHE => {
            if !attacker_choice.first_move
                && (defender_choice.category == MoveCategory::Physical
                    || defender_choice.category == MoveCategory::Special)
            {
                attacker_choice.base_power *= 2.0;
            }
        }

        #[cfg(any(feature = "gen4"))]
        Choices::PAYBACK => {
            if !attacker_choice.first_move {
                attacker_choice.base_power *= 2.0;
            }
        }

        #[cfg(any(
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Choices::PAYBACK => {
            if !attacker_choice.first_move && defender_choice.category != MoveCategory::Switch {
                attacker_choice.base_power *= 2.0;
            }
        }

        Choices::FACADE => {
            if attacking_side.get_active_immutable().status != PokemonStatus::None {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::STOREDPOWER => {
            let total_boosts = attacking_side.attack_boost
                + attacking_side.defense_boost
                + attacking_side.special_attack_boost
                + attacking_side.special_defense_boost
                + attacking_side.speed_boost
                + attacking_side.accuracy_boost
                + attacking_side.evasion_boost;
            if total_boosts > 0 {
                attacker_choice.base_power *= total_boosts as f32;
            }
        }
        Choices::FOULPLAY => {
            let defender_attack =
                defending_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
            let attacker_attack =
                attacking_side.calculate_boosted_stat(PokemonBoostableStat::Attack);
            attacker_choice.base_power *= defender_attack as f32 / attacker_attack as f32;
        }
        Choices::BARBBARRAGE => {
            let defending_pkmn_status = defending_side.get_active_immutable().status;
            if defending_pkmn_status == PokemonStatus::Poison
                || defending_pkmn_status == PokemonStatus::Toxic
            {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::FREEZEDRY => {
            if defending_side
                .get_active_immutable()
                .has_type(&PokemonType::Water)
            {
                attacker_choice.base_power *= 4.0; // 2x for being super effective, 2x for nullifying water resistance
            }
        }
        Choices::ERUPTION | Choices::WATERSPOUT => {
            let attacker = attacking_side.get_active_immutable();
            let hp_ratio = attacker.hp as f32 / attacker.maxhp as f32;
            attacker_choice.base_power *= hp_ratio;
        }
        Choices::SUCKERPUNCH => {
            if !attacker_choice.first_move || defender_choice.category == MoveCategory::Status {
                attacker_choice.base_power = 0.0;
            }
        }
        Choices::COLLISIONCOURSE => {
            let defender_active = defending_side.get_active_immutable();
            if type_effectiveness_modifier(&attacker_choice.move_type, &defender_active.types) > 1.0
            {
                attacker_choice.base_power *= 1.3;
            }
        }
        Choices::GRASSKNOT | Choices::LOWKICK => {
            let defender_active = defending_side.get_active_immutable();
            if defender_active.weight_kg < 10.0 {
                attacker_choice.base_power = 20.0;
            } else if defender_active.weight_kg < 25.0 {
                attacker_choice.base_power = 40.0;
            } else if defender_active.weight_kg < 50.0 {
                attacker_choice.base_power = 60.0;
            } else if defender_active.weight_kg < 100.0 {
                attacker_choice.base_power = 80.0;
            } else if defender_active.weight_kg < 200.0 {
                attacker_choice.base_power = 100.0;
            } else {
                attacker_choice.base_power = 120.0;
            }
        }
        Choices::HEATCRASH | Choices::HEAVYSLAM => {
            let attacker = attacking_side.get_active_immutable();
            let defender = defending_side.get_active_immutable();
            let weight_ratio = defender.weight_kg / attacker.weight_kg;
            if weight_ratio > 0.5 {
                attacker_choice.base_power = 40.0;
            } else if weight_ratio > 0.3335 {
                attacker_choice.base_power = 60.0;
            } else if weight_ratio >= 0.2501 {
                attacker_choice.base_power = 80.0;
            } else if weight_ratio >= 0.2001 {
                attacker_choice.base_power = 100.0;
            } else {
                attacker_choice.base_power = 120.0;
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
    let (_attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
    match choice.move_id {
        Choices::KNOCKOFF => {
            let defender_active = defending_side.get_active();
            if defender_active.item_can_be_removed()
                && defender_active.item != Items::NONE
                && !hit_sub
            {
                let instruction = Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    current_item: defender_active.item,
                    new_item: Items::NONE,
                });
                instructions.instruction_list.push(instruction);
                defender_active.item = Items::NONE;
            }
        }
        Choices::CLEARSMOG => {
            state.reset_boosts(
                &attacking_side_ref.get_other_side(),
                &mut instructions.instruction_list,
            );
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
    let (attacking_side, _) = state.get_both_sides(attacking_side_ref);
    let attacker = attacking_side.get_active();
    if choice.flags.charge
        && attacker.item == Items::POWERHERB
        && choice.move_id != Choices::SKYDROP
    {
        let instruction = Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: *attacking_side_ref,
            current_item: Items::POWERHERB,
            new_item: Items::NONE,
        });
        attacker.item = Items::NONE;
        choice.flags.charge = false;
        instructions.instruction_list.push(instruction);
    }
    if let Some(choice_volatile_status) = &choice.volatile_status {
        if choice_volatile_status.volatile_status == PokemonVolatileStatus::LockedMove
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
        Choices::COURTCHANGE => {
            let mut instruction_list = vec![];
            let courtchange_swaps = [
                PokemonSideCondition::Stealthrock,
                PokemonSideCondition::Spikes,
                PokemonSideCondition::ToxicSpikes,
                PokemonSideCondition::StickyWeb,
                PokemonSideCondition::Reflect,
                PokemonSideCondition::LightScreen,
                PokemonSideCondition::AuroraVeil,
                PokemonSideCondition::Tailwind,
            ];

            for side in [SideReference::SideOne, SideReference::SideTwo] {
                for side_condition in courtchange_swaps {
                    let side_condition_num = state
                        .get_side_immutable(&side)
                        .get_side_condition(side_condition);
                    if side_condition_num > 0 {
                        instruction_list.push(Instruction::ChangeSideCondition(
                            ChangeSideConditionInstruction {
                                side_ref: side,
                                side_condition: side_condition,
                                amount: -1 * side_condition_num,
                            },
                        ));
                        instruction_list.push(Instruction::ChangeSideCondition(
                            ChangeSideConditionInstruction {
                                side_ref: side.get_other_side(),
                                side_condition: side_condition,
                                amount: side_condition_num,
                            },
                        ));
                    }
                }
            }
            state.apply_instructions(&instruction_list);
            for i in instruction_list {
                instructions.instruction_list.push(i)
            }
        }
        Choices::DEFOG => {
            if state.terrain.terrain_type != Terrain::None {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeTerrain(ChangeTerrain {
                        new_terrain: Terrain::None,
                        new_terrain_turns_remaining: 0,
                        previous_terrain: state.terrain.terrain_type,
                        previous_terrain_turns_remaining: state.terrain.turns_remaining,
                    }));
                state.terrain.terrain_type = Terrain::None;
                state.terrain.turns_remaining = 0;
            }
            let side_condition_clears = [
                PokemonSideCondition::Stealthrock,
                PokemonSideCondition::Spikes,
                PokemonSideCondition::ToxicSpikes,
                PokemonSideCondition::StickyWeb,
                PokemonSideCondition::Reflect,
                PokemonSideCondition::LightScreen,
                PokemonSideCondition::AuroraVeil,
            ];

            for side in [SideReference::SideOne, SideReference::SideTwo] {
                for side_condition in side_condition_clears {
                    let side_condition_num = state
                        .get_side_immutable(&side)
                        .get_side_condition(side_condition);
                    if side_condition_num > 0 {
                        let i = Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                            side_ref: side,
                            side_condition: side_condition,
                            amount: -1 * side_condition_num,
                        });
                        state.apply_one_instruction(&i);
                        instructions.instruction_list.push(i)
                    }
                }
            }
        }
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
    attacking_side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
    match choice.move_id {
        Choices::COUNTER => {
            if defending_side.damage_dealt.move_category == MoveCategory::Physical
                && !defending_side
                    .get_active_immutable()
                    .has_type(&PokemonType::Fighting)
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
                    .has_type(&PokemonType::Dark)
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
        Choices::METALBURST => {
            if defending_side.damage_dealt.move_category != MoveCategory::Status
                && !defending_side.damage_dealt.hit_substitute
                && !choice.first_move
            {
                let damage_amount = cmp::min(
                    (defending_side.damage_dealt.damage * 3) / 2,
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
        Choices::WISH => {
            if attacking_side.wish.0 == 0 {
                instructions
                    .instruction_list
                    .push(Instruction::SetWish(SetWishInstruction {
                        side_ref: *attacking_side_ref,
                        wish_amount: attacking_side.get_active_immutable().maxhp / 2,
                        previous_wish_amount: 0,
                    }));
                attacking_side.wish = (2, attacking_side.get_active_immutable().maxhp / 2);
            }
        }
        Choices::REFRESH => {
            let active_index = attacking_side.active_index;
            let active_pkmn = attacking_side.get_active();
            if active_pkmn.status != PokemonStatus::None {
                add_remove_status_instructions(
                    instructions,
                    active_index,
                    *attacking_side_ref,
                    attacking_side,
                );
            }
        }
        Choices::HEALBELL | Choices::AROMATHERAPY => {
            for pkmn_index in pokemon_index_iter() {
                if attacking_side.pokemon[pkmn_index].status != PokemonStatus::None {
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
            if active_pkmn.status != PokemonStatus::Sleep {
                let heal_amount = active_pkmn.maxhp - active_pkmn.hp;
                instructions
                    .instruction_list
                    .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: *attacking_side_ref,
                        pokemon_index: active_index,
                        old_status: active_pkmn.status,
                        new_status: PokemonStatus::Sleep,
                    }));
                instructions
                    .instruction_list
                    .push(Instruction::SetRestTurns(SetRestTurnsInstruction {
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
                active_pkmn.status = PokemonStatus::Sleep;
                active_pkmn.rest_turns = 3;
            }
        }
        Choices::TRICKROOM => {
            state.trick_room = !state.trick_room;
            instructions
                .instruction_list
                .push(Instruction::ToggleTrickRoom);
        }
        Choices::SUPERFANG | Choices::NATURESMADNESS | Choices::RUINATION => {
            let target_pkmn = defending_side.get_active();
            if target_pkmn.hp == 1 {
                return;
            }
            if choice.move_id == Choices::SUPERFANG
                && type_effectiveness_modifier(&PokemonType::Normal, &target_pkmn.types) == 0.0
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
            if type_effectiveness_modifier(&PokemonType::Ghost, &defender_active.types) == 0.0 {
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
            if type_effectiveness_modifier(&PokemonType::Normal, &defender_active.types) == 0.0 {
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
        Choices::ENDEAVOR => {
            let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
            let attacker = attacking_side.get_active();
            let defender = defending_side.get_active();

            if type_effectiveness_modifier(&PokemonType::Normal, &defender.types) == 0.0
                || attacker.hp >= defender.hp
            {
                return;
            }

            let damage_amount = defender.hp - attacker.hp;
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    damage_amount: damage_amount,
                }));
            defender.hp -= damage_amount;
        }
        Choices::FINALGAMBIT => {
            let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
            let attacker = attacking_side.get_active();
            let defender = defending_side.get_active();

            if type_effectiveness_modifier(&PokemonType::Normal, &defender.types) == 0.0 {
                return;
            }

            let damage_amount = attacker.hp;
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: attacking_side_ref.get_other_side(),
                    damage_amount: damage_amount,
                }));
            defender.hp -= damage_amount;

            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: *attacking_side_ref,
                    damage_amount: attacker.hp,
                }));
            attacker.hp = 0;
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
                .contains(&PokemonVolatileStatus::Substitute)
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
                    Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                        side_ref: attacking_side_ref.clone(),
                        new_health: sub_target_health,
                        old_health: sub_current_health,
                    });
                let apply_vs_instruction =
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: attacking_side_ref.clone(),
                        volatile_status: PokemonVolatileStatus::Substitute,
                    });
                active_pkmn.hp -= sub_target_health;
                attacking_side.substitute_health = sub_target_health;
                attacking_side
                    .volatile_statuses
                    .insert(PokemonVolatileStatus::Substitute);
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
                    && pkmn.ability != Abilities::SOUNDPROOF
                    && !(side
                        .volatile_statuses
                        .contains(&PokemonVolatileStatus::Perish4)
                        || side
                            .volatile_statuses
                            .contains(&PokemonVolatileStatus::Perish3)
                        || side
                            .volatile_statuses
                            .contains(&PokemonVolatileStatus::Perish2)
                        || side
                            .volatile_statuses
                            .contains(&PokemonVolatileStatus::Perish1))
                {
                    instructions
                        .instruction_list
                        .push(Instruction::ApplyVolatileStatus(
                            ApplyVolatileStatusInstruction {
                                side_ref: side_ref,
                                volatile_status: PokemonVolatileStatus::Perish4,
                            },
                        ));
                    side.volatile_statuses
                        .insert(PokemonVolatileStatus::Perish4);
                }
            }
        }
        Choices::TRICK | Choices::SWITCHEROO => {
            let defender_has_sub = defending_side
                .volatile_statuses
                .contains(&PokemonVolatileStatus::Substitute);
            let attacker = attacking_side.get_active();
            let defender = defending_side.get_active();
            let attacker_item = attacker.item;
            let defender_item = defender.item;
            if attacker_item == defender_item || !defender.item_can_be_removed() || defender_has_sub
            {
                return;
            }
            let change_attacker_item_instruction = Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: *attacking_side_ref,
                current_item: attacker_item,
                new_item: defender_item,
            });
            let change_defender_item_instruction = Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: attacking_side_ref.get_other_side(),
                current_item: defender_item,
                new_item: attacker_item,
            });
            attacker.item = defender_item;
            defender.item = attacker_item;
            instructions
                .instruction_list
                .push(change_attacker_item_instruction);
            instructions
                .instruction_list
                .push(change_defender_item_instruction);
        }
        Choices::SUNNYDAY => {
            if state.weather.weather_type != Weather::Sun {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::Sun,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: 0,
                    }));
                state.weather.weather_type = Weather::Sun;
                state.weather.turns_remaining = 5;
            }
        }
        Choices::RAINDANCE => {
            if state.weather.weather_type != Weather::Rain {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::Rain,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: 0,
                    }));
                state.weather.weather_type = Weather::Rain;
                state.weather.turns_remaining = 5;
            }
        }
        Choices::SANDSTORM => {
            if state.weather.weather_type != Weather::Sand {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::Sand,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: 0,
                    }));
                state.weather.weather_type = Weather::Sand;
                state.weather.turns_remaining = 5;
            }
        }
        Choices::HAIL => {
            if state.weather.weather_type != Weather::Hail {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::Hail,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: 0,
                    }));
                state.weather.weather_type = Weather::Hail;
                state.weather.turns_remaining = 5;
            }
        }
        _ => {}
    }
}

pub fn charge_choice_to_volatile(choice: &Choices) -> PokemonVolatileStatus {
    // Panics if you pass a choice that does not have a corresponding volatile status
    match choice {
        Choices::BOUNCE => PokemonVolatileStatus::Bounce,
        Choices::DIG => PokemonVolatileStatus::Dig,
        Choices::DIVE => PokemonVolatileStatus::Dive,
        Choices::FLY => PokemonVolatileStatus::Fly,
        Choices::FREEZESHOCK => PokemonVolatileStatus::Freezeshock,
        Choices::GEOMANCY => PokemonVolatileStatus::Geomancy,
        Choices::ICEBURN => PokemonVolatileStatus::IceBurn,
        Choices::METEORBEAM => PokemonVolatileStatus::MeteorBeam,
        Choices::PHANTOMFORCE => PokemonVolatileStatus::PhantomForce,
        Choices::RAZORWIND => PokemonVolatileStatus::RazorWind,
        Choices::SHADOWFORCE => PokemonVolatileStatus::ShadowForce,
        Choices::SKULLBASH => PokemonVolatileStatus::SkullBash,
        Choices::SKYATTACK => PokemonVolatileStatus::SkyAttack,
        Choices::SKYDROP => PokemonVolatileStatus::SkyDrop,
        Choices::SOLARBEAM => PokemonVolatileStatus::SolarBeam,
        Choices::SOLARBLADE => PokemonVolatileStatus::SolarBlade,
        _ => {
            panic!("Invalid choice for charge: {:?}", choice)
        }
    }
}
