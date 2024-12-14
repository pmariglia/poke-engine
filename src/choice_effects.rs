use crate::abilities::Abilities;
use crate::choices::{Boost, Choice, Choices, Heal, MoveCategory, MoveTarget, StatBoosts};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::{add_remove_status_instructions, get_boost_instruction};
use crate::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeSideConditionInstruction, ChangeStatusInstruction, ChangeSubsituteHealthInstruction,
    ChangeTerrain, ChangeWeather, ChangeWishInstruction, DamageInstruction, HealInstruction,
    Instruction, RemoveVolatileStatusInstruction, SetFutureSightInstruction,
    SetSleepTurnsInstruction, StateInstructions, ToggleTrickRoomInstruction,
};
use crate::items::{get_choice_move_disable_instructions, Items};
use crate::pokemon::PokemonName;
use crate::state::{
    pokemon_index_iter, LastUsedMove, PokemonBoostableStat, PokemonSideCondition, PokemonStatus,
    PokemonType, PokemonVolatileStatus, SideReference, State, Terrain, Weather,
};
use std::cmp;

#[cfg(feature = "terastallization")]
use crate::choices::{MultiAccuracyMove, MultiHitMove};

pub fn modify_choice(
    state: &State,
    attacker_choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
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
        Choices::AURAWHEEL => {
            if attacking_side.get_active_immutable().id == PokemonName::MORPEKOHANGRY {
                attacker_choice.move_type = PokemonType::DARK;
            }
        }
        Choices::IVYCUDGEL => {
            let attacker = attacking_side.get_active_immutable();
            match attacker.item {
                Items::WELLSPRINGMASK => {
                    attacker_choice.move_type = PokemonType::WATER;
                }
                Items::HEARTHFLAMEMASK => {
                    attacker_choice.move_type = PokemonType::FIRE;
                }
                Items::CORNERSTONEMASK => {
                    attacker_choice.move_type = PokemonType::ROCK;
                }
                _ => {}
            }
        }
        Choices::RAGINGBULL => {
            // this gives the correct result even though it's not the "correct" way to implement it
            // reflect is only removed if the move hits, but I don't have a way to check that
            // doubling the power ensures the same damage calculation
            if defending_side.side_conditions.reflect > 0 {
                attacker_choice.base_power *= 2.0;
            }
            match attacking_side.get_active_immutable().id {
                PokemonName::TAUROSPALDEACOMBAT => {
                    attacker_choice.move_type = PokemonType::FIGHTING;
                }
                PokemonName::TAUROSPALDEABLAZE => {
                    attacker_choice.move_type = PokemonType::FIRE;
                }
                PokemonName::TAUROSPALDEAAQUA => {
                    attacker_choice.move_type = PokemonType::WATER;
                }
                _ => {}
            }
        }
        Choices::BOLTBEAK | Choices::FISHIOUSREND => {
            if attacker_choice.first_move {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::HARDPRESS => {
            let defender = defending_side.get_active_immutable();
            attacker_choice.base_power = 100.0 * (defender.hp as f32 / defender.maxhp as f32);
        }
        Choices::LASTRESPECTS => {
            // Technically not correct because of reviving moves but good enough
            let mut bp = 50.0;
            for pkmn in attacking_side.pokemon.into_iter() {
                if pkmn.hp == 0 && pkmn.level != 1 {
                    bp += 50.0;
                }
            }
            attacker_choice.base_power = bp
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
            if state.terrain.terrain_type == Terrain::PSYCHICTERRAIN {
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
            if state.weather_is_active(&Weather::SUN) {
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
            if defending_side.get_active_immutable().status != PokemonStatus::NONE {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::HYDROSTEAM => {
            if state.weather_is_active(&Weather::SUN) {
                attacker_choice.base_power *= 3.0; // 1.5x for being in sun, 2x for cancelling out rain debuff
            }
        }
        Choices::JUDGMENT => {
            attacker_choice.move_type = attacking_side.get_active_immutable().types.0;
        }
        Choices::MULTIATTACK => {
            attacker_choice.move_type = attacking_side.get_active_immutable().types.0;
        }
        Choices::MISTYEXPLOSION => {
            if state.terrain.terrain_type == Terrain::MISTYTERRAIN {
                attacker_choice.base_power *= 1.5;
            }
        }
        #[cfg(any(feature = "gen3", feature = "gen4"))]
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
        Choices::NORETREAT => {
            if attacking_side
                .volatile_statuses
                .contains(&PokemonVolatileStatus::NORETREAT)
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
            if state.terrain.terrain_type == Terrain::ELECTRICTERRAIN {
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
            if state.terrain.terrain_type == Terrain::ELECTRICTERRAIN {
                attacker_choice.base_power *= 1.5;
            }
        }
        Choices::SHOREUP => {
            if state.weather_is_active(&Weather::SAND) {
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.667,
                });
            }
        }
        Choices::STEELROLLER => {
            if state.terrain.terrain_type == Terrain::NONE {
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
        Choices::TERABLAST => {
            let active = attacking_side.get_active_immutable();
            if active.terastallized {
                attacker_choice.move_type = active.tera_type;
                if attacking_side.calculate_boosted_stat(PokemonBoostableStat::Attack)
                    > attacking_side.calculate_boosted_stat(PokemonBoostableStat::SpecialAttack)
                {
                    attacker_choice.category = MoveCategory::Physical;
                }
            }
        }
        Choices::TERRAINPULSE => match state.terrain.terrain_type {
            Terrain::ELECTRICTERRAIN => {
                attacker_choice.move_type = PokemonType::ELECTRIC;
                attacker_choice.base_power *= 2.0;
            }
            Terrain::GRASSYTERRAIN => {
                attacker_choice.move_type = PokemonType::GRASS;
                attacker_choice.base_power *= 2.0;
            }
            Terrain::MISTYTERRAIN => {
                attacker_choice.move_type = PokemonType::FAIRY;
                attacker_choice.base_power *= 2.0;
            }
            Terrain::PSYCHICTERRAIN => {
                attacker_choice.move_type = PokemonType::PSYCHIC;
                attacker_choice.base_power *= 2.0;
            }
            Terrain::NONE => {}
        },
        Choices::TOXIC => {
            if attacking_side
                .get_active_immutable()
                .has_type(&PokemonType::POISON)
            {
                attacker_choice.accuracy = 100.0;
            }
        }
        Choices::WEATHERBALL => match state.weather.weather_type {
            Weather::SUN | Weather::HARSHSUN => {
                attacker_choice.base_power = 100.0;
                attacker_choice.move_type = PokemonType::FIRE;
            }
            Weather::RAIN | Weather::HEAVYRAIN => {
                attacker_choice.base_power = 100.0;
                attacker_choice.move_type = PokemonType::WATER;
            }
            Weather::SAND => {
                attacker_choice.base_power = 100.0;
                attacker_choice.move_type = PokemonType::ROCK;
            }
            Weather::HAIL | Weather::SNOW => {
                attacker_choice.base_power = 100.0;
                attacker_choice.move_type = PokemonType::ICE;
            }
            Weather::NONE => {}
        },
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
        Choices::BLIZZARD => {
            if state.weather_is_active(&Weather::HAIL) {
                attacker_choice.accuracy = 100.0;
            }
        }
        Choices::HURRICANE | Choices::THUNDER => {
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

        #[cfg(any(feature = "gen6", feature = "gen7", feature = "gen8", feature = "gen9"))]
        Choices::KNOCKOFF => {
            // Bonus damage still applies if substitute is hit
            let defender = defending_side.get_active_immutable();
            if !defender.item_is_permanent() && defender.item != Items::NONE {
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

        #[cfg(any(feature = "gen3", feature = "gen4"))]
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
            if attacking_side.get_active_immutable().status != PokemonStatus::NONE {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::STOREDPOWER => {
            let total_boosts = attacking_side.attack_boost.max(0)
                + attacking_side.defense_boost.max(0)
                + attacking_side.special_attack_boost.max(0)
                + attacking_side.special_defense_boost.max(0)
                + attacking_side.speed_boost.max(0)
                + attacking_side.accuracy_boost.max(0)
                + attacking_side.evasion_boost.max(0);
            if total_boosts > 0 {
                attacker_choice.base_power += 20.0 * total_boosts as f32;
            }
        }
        Choices::BARBBARRAGE => {
            let defending_pkmn_status = defending_side.get_active_immutable().status;
            if defending_pkmn_status == PokemonStatus::POISON
                || defending_pkmn_status == PokemonStatus::TOXIC
            {
                attacker_choice.base_power *= 2.0;
            }
        }
        Choices::FREEZEDRY => {
            if defending_side
                .get_active_immutable()
                .has_type(&PokemonType::WATER)
            {
                attacker_choice.base_power *= 4.0; // 2x for being super effective, 2x for nullifying water resistance
            }
        }
        Choices::ERUPTION | Choices::WATERSPOUT | Choices::DRAGONENERGY => {
            let attacker = attacking_side.get_active_immutable();
            let hp_ratio = attacker.hp as f32 / attacker.maxhp as f32;
            attacker_choice.base_power *= hp_ratio;
        }
        Choices::SUCKERPUNCH | Choices::THUNDERCLAP => {
            if !attacker_choice.first_move || defender_choice.category == MoveCategory::Status {
                attacker_choice.base_power = 0.0;
            }
        }
        Choices::UPPERHAND => {
            if !(attacker_choice.first_move && defender_choice.priority > 0) {
                attacker_choice.remove_all_effects()
            }
        }
        Choices::COLLISIONCOURSE | Choices::ELECTRODRIFT => {
            let defender_active = defending_side.get_active_immutable();
            if type_effectiveness_modifier(&attacker_choice.move_type, &defender_active) > 1.0 {
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
    let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
    let attacker_active = attacking_side.get_active();
    if choice.flags.recharge {
        let instruction = Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
            side_ref: attacking_side_ref.clone(),
            volatile_status: PokemonVolatileStatus::MUSTRECHARGE,
        });
        instructions.instruction_list.push(instruction);
        attacking_side
            .volatile_statuses
            .insert(PokemonVolatileStatus::MUSTRECHARGE);

    // Recharging and truant are mutually exclusive, with recharge taking priority
    } else if attacker_active.ability == Abilities::TRUANT {
        let instruction = Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
            side_ref: attacking_side_ref.clone(),
            volatile_status: PokemonVolatileStatus::TRUANT,
        });
        instructions.instruction_list.push(instruction);
        attacking_side
            .volatile_statuses
            .insert(PokemonVolatileStatus::TRUANT);
    }
    match choice.move_id {
        Choices::RAGINGBULL => {
            if defending_side.side_conditions.reflect > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: attacking_side_ref.get_other_side(),
                            side_condition: PokemonSideCondition::Reflect,
                            amount: -1 * defending_side.side_conditions.reflect,
                        },
                    ));
                defending_side.side_conditions.reflect = 0;
            }
            if defending_side.side_conditions.light_screen > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: attacking_side_ref.get_other_side(),
                            side_condition: PokemonSideCondition::LightScreen,
                            amount: -1 * defending_side.side_conditions.light_screen,
                        },
                    ));
                defending_side.side_conditions.light_screen = 0;
            }
            if defending_side.side_conditions.aurora_veil > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: attacking_side_ref.get_other_side(),
                            side_condition: PokemonSideCondition::AuroraVeil,
                            amount: -1 * defending_side.side_conditions.aurora_veil,
                        },
                    ));
                defending_side.side_conditions.aurora_veil = 0;
            }
        }
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
        Choices::CLEARSMOG => {
            state.reset_boosts(
                &attacking_side_ref.get_other_side(),
                &mut instructions.instruction_list,
            );
        }
        Choices::ICESPINNER => {
            if state.terrain.terrain_type != Terrain::NONE && state.terrain.turns_remaining > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeTerrain(ChangeTerrain {
                        new_terrain: Terrain::NONE,
                        new_terrain_turns_remaining: 0,
                        previous_terrain: state.terrain.terrain_type,
                        previous_terrain_turns_remaining: state.terrain.turns_remaining,
                    }));
                state.terrain.terrain_type = Terrain::NONE;
                state.terrain.turns_remaining = 0;
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
    let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
    let attacker = attacking_side.get_active();
    let defender = defending_side.get_active_immutable();

    #[cfg(feature = "terastallization")]
    if attacker.terastallized
        && choice.move_type == attacker.tera_type
        && choice.base_power < 60.0
        && choice.priority <= 0
        && choice.multi_hit() == MultiHitMove::None
        && choice.multi_accuracy() == MultiAccuracyMove::None
    {
        choice.base_power = 60.0;
    }

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
        Choices::EXPLOSION | Choices::SELFDESTRUCT | Choices::MISTYEXPLOSION
            if defender.ability != Abilities::DAMP =>
        {
            let damage_amount = attacker.hp;
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: *attacking_side_ref,
                    damage_amount,
                }));
            attacker.hp = 0;
        }
        Choices::MINDBLOWN if defender.ability != Abilities::DAMP => {
            let damage_amount = cmp::min(attacker.maxhp / 2, attacker.hp);
            instructions
                .instruction_list
                .push(Instruction::Damage(DamageInstruction {
                    side_ref: *attacking_side_ref,
                    damage_amount,
                }));
            attacker.hp -= damage_amount;
        }
        Choices::METEORBEAM | Choices::ELECTROSHOT if choice.flags.charge => {
            if let Some(boost_instruction) = get_boost_instruction(
                &attacking_side,
                &PokemonBoostableStat::SpecialAttack,
                &1,
                attacking_side_ref,
                attacking_side_ref,
            ) {
                state.apply_one_instruction(&boost_instruction);
                instructions.instruction_list.push(boost_instruction);
            }
        }
        _ => {}
    }
    let attacking_side = state.get_side(attacking_side_ref);
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
            if state.terrain.terrain_type != Terrain::NONE {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeTerrain(ChangeTerrain {
                        new_terrain: Terrain::NONE,
                        new_terrain_turns_remaining: 0,
                        previous_terrain: state.terrain.terrain_type,
                        previous_terrain_turns_remaining: state.terrain.turns_remaining,
                    }));
                state.terrain.terrain_type = Terrain::NONE;
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
        Choices::TIDYUP => {
            let side_condition_clears = [
                PokemonSideCondition::Stealthrock,
                PokemonSideCondition::Spikes,
                PokemonSideCondition::ToxicSpikes,
                PokemonSideCondition::StickyWeb,
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
            if state
                .side_one
                .volatile_statuses
                .contains(&PokemonVolatileStatus::SUBSTITUTE)
            {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSubstituteHealth(
                        ChangeSubsituteHealthInstruction {
                            side_ref: SideReference::SideOne,
                            health_change: -1 * state.side_one.substitute_health,
                        },
                    ));
                instructions
                    .instruction_list
                    .push(Instruction::RemoveVolatileStatus(
                        RemoveVolatileStatusInstruction {
                            side_ref: SideReference::SideOne,
                            volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                        },
                    ));
                state.side_one.substitute_health = 0;
                state
                    .side_one
                    .volatile_statuses
                    .remove(&PokemonVolatileStatus::SUBSTITUTE);
            }
            if state
                .side_two
                .volatile_statuses
                .contains(&PokemonVolatileStatus::SUBSTITUTE)
            {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSubstituteHealth(
                        ChangeSubsituteHealthInstruction {
                            side_ref: SideReference::SideTwo,
                            health_change: -1 * state.side_two.substitute_health,
                        },
                    ));
                instructions
                    .instruction_list
                    .push(Instruction::RemoveVolatileStatus(
                        RemoveVolatileStatusInstruction {
                            side_ref: SideReference::SideTwo,
                            volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                        },
                    ));
                state.side_two.substitute_health = 0;
                state
                    .side_two
                    .volatile_statuses
                    .remove(&PokemonVolatileStatus::SUBSTITUTE);
            }
        }
        Choices::RAPIDSPIN | Choices::MORTALSPIN => {
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
            }
        }
        Choices::COUNTER => {
            if defending_side.damage_dealt.move_category == MoveCategory::Physical
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
        Choices::METALBURST | Choices::COMEUPPANCE => {
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
                let previous_wish_amount = attacking_side.wish.1;
                instructions.instruction_list.push(Instruction::ChangeWish(
                    ChangeWishInstruction {
                        side_ref: *attacking_side_ref,
                        wish_amount_change: attacking_side.get_active_immutable().maxhp / 2
                            - previous_wish_amount,
                    },
                ));
                attacking_side.wish = (2, attacking_side.get_active_immutable().maxhp / 2);
            }
        }
        Choices::REFRESH => {
            let active_index = attacking_side.active_index;
            let active_pkmn = attacking_side.get_active();
            if active_pkmn.status != PokemonStatus::NONE {
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
            if active_pkmn.status != PokemonStatus::SLEEP {
                let heal_amount = active_pkmn.maxhp - active_pkmn.hp;
                instructions
                    .instruction_list
                    .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: *attacking_side_ref,
                        pokemon_index: active_index,
                        old_status: active_pkmn.status,
                        new_status: PokemonStatus::SLEEP,
                    }));
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
        Choices::TRICKROOM => {
            let new_turns_remaining;
            if state.trick_room.active {
                new_turns_remaining = 0;
            } else {
                new_turns_remaining = 5;
            }
            instructions
                .instruction_list
                .push(Instruction::ToggleTrickRoom(ToggleTrickRoomInstruction {
                    currently_active: state.trick_room.active,
                    new_trickroom_turns_remaining: new_turns_remaining,
                    previous_trickroom_turns_remaining: state.trick_room.turns_remaining,
                }));
            state.trick_room.active = !state.trick_room.active;
        }
        Choices::SUPERFANG | Choices::NATURESMADNESS | Choices::RUINATION => {
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
        Choices::ENDEAVOR => {
            let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
            let attacker = attacking_side.get_active();
            let defender = defending_side.get_active();

            if type_effectiveness_modifier(&PokemonType::NORMAL, &defender) == 0.0
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

            if type_effectiveness_modifier(&PokemonType::NORMAL, &defender) == 0.0 {
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
                    && pkmn.ability != Abilities::SOUNDPROOF
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
        Choices::TRICK | Choices::SWITCHEROO => {
            let defender_has_sub = defending_side
                .volatile_statuses
                .contains(&PokemonVolatileStatus::SUBSTITUTE);
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
        Choices::HAIL => {
            if state.weather.weather_type != Weather::HAIL {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::HAIL,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::HAIL;
                state.weather.turns_remaining = 5;
            }
        }
        Choices::SNOWSCAPE | Choices::CHILLYRECEPTION => {
            if state.weather.weather_type != Weather::SNOW {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::SNOW,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::SNOW;
                state.weather.turns_remaining = 5;
            }
        }
        _ => {}
    }
}

pub fn charge_choice_to_volatile(choice: &Choices) -> PokemonVolatileStatus {
    // Panics if you pass a choice that does not have a corresponding volatile status
    match choice {
        Choices::BOUNCE => PokemonVolatileStatus::BOUNCE,
        Choices::DIG => PokemonVolatileStatus::DIG,
        Choices::DIVE => PokemonVolatileStatus::DIVE,
        Choices::FLY => PokemonVolatileStatus::FLY,
        Choices::FREEZESHOCK => PokemonVolatileStatus::FREEZESHOCK,
        Choices::GEOMANCY => PokemonVolatileStatus::GEOMANCY,
        Choices::ICEBURN => PokemonVolatileStatus::ICEBURN,
        Choices::METEORBEAM => PokemonVolatileStatus::METEORBEAM,
        Choices::ELECTROSHOT => PokemonVolatileStatus::ELECTROSHOT,
        Choices::PHANTOMFORCE => PokemonVolatileStatus::PHANTOMFORCE,
        Choices::RAZORWIND => PokemonVolatileStatus::RAZORWIND,
        Choices::SHADOWFORCE => PokemonVolatileStatus::SHADOWFORCE,
        Choices::SKULLBASH => PokemonVolatileStatus::SKULLBASH,
        Choices::SKYATTACK => PokemonVolatileStatus::SKYATTACK,
        Choices::SKYDROP => PokemonVolatileStatus::SKYDROP,
        Choices::SOLARBEAM => PokemonVolatileStatus::SOLARBEAM,
        Choices::SOLARBLADE => PokemonVolatileStatus::SOLARBLADE,
        _ => {
            panic!("Invalid choice for charge: {:?}", choice)
        }
    }
}
