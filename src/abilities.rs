#![allow(unused_variables)]
use crate::choices::{
    Boost, Choice, Choices, Effect, Heal, MoveCategory, MoveTarget, Secondary, StatBoosts,
    VolatileStatus,
};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::{add_remove_status_instructions, get_boost_instruction};
use crate::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeItemInstruction,
    ChangeSideConditionInstruction, ChangeStatusInstruction, ChangeTerrain, ChangeType,
    ChangeWeather, DamageInstruction, HealInstruction, Instruction, StateInstructions,
};
use crate::items::Items;
use crate::state::{PokemonBoostableStat, PokemonSideCondition, PokemonType, Terrain};
use crate::state::{PokemonStatus, State};
use crate::state::{PokemonVolatileStatus, SideReference, Weather};
use std::cmp;

#[derive(PartialEq, Debug, Clone)]
pub enum Abilities {
    RIPEN,
    TANGLEDFEET,
    DRAGONSMAW,
    CLEARBODY,
    GALVANIZE,
    VITALSPIRIT,
    AERILATE,
    DEFIANT,
    CUTECHARM,
    NEUROFORCE,
    SOUNDPROOF,
    RKSSYSTEM,
    POISONPOINT,
    STAKEOUT,
    UNNERVE,
    ROCKHEAD,
    AURABREAK,
    MIMICRY,
    BULLETPROOF,
    POWEROFALCHEMY,
    TECHNICIAN,
    MULTISCALE,
    ARENATRAP,
    BATTLEBOND,
    DISGUISE,
    EARLYBIRD,
    LIGHTNINGROD,
    MAGICIAN,
    REFRIGERATE,
    FRIENDGUARD,
    NOABILITY,
    GULPMISSILE,
    POWERCONSTRUCT,
    FORECAST,
    PRANKSTER,
    PROTEAN,
    ASONEGLASTRIER,
    SHADOWTAG,
    SKILLLINK,
    INTREPIDSWORD,
    SOULHEART,
    SWIFTSWIM,
    EARTHEATER,
    SUPERLUCK,
    SUPREMEOVERLORD,
    INSOMNIA,
    DANCER,
    STEAMENGINE,
    ANGERPOINT,
    CONTRARY,
    MAGMAARMOR,
    HUNGERSWITCH,
    RECEIVER,
    ZENMODE,
    EMERGENCYEXIT,
    ILLUSION,
    WEAKARMOR,
    DROUGHT,
    INNARDSOUT,
    SHIELDSDOWN,
    ADAPTABILITY,
    CORROSION,
    LONGREACH,
    PUREPOWER,
    TINTEDLENS,
    QUEENLYMAJESTY,
    DESOLATELAND,
    MOXIE,
    SAPSIPPER,
    SLUSHRUSH,
    BIGPECKS,
    STALL,
    WHITESMOKE,
    FLAREBOOST,
    SHADOWSHIELD,
    LIQUIDVOICE,
    MISTYSURGE,
    MULTITYPE,
    NOGUARD,
    TORRENT,
    DELTASTREAM,
    KLUTZ,
    LIBERO,
    SERENEGRACE,
    CURSEDBODY,
    UNAWARE,
    LIGHTMETAL,
    MARVELSCALE,
    TELEPATHY,
    QUICKDRAW,
    HYPERCUTTER,
    SYMBIOSIS,
    PLUS,
    MIRRORARMOR,
    PASTELVEIL,
    TOUGHCLAWS,
    EFFECTSPORE,
    MUMMY,
    BADDREAMS,
    MAGICGUARD,
    SANDSTREAM,
    POWERSPOT,
    FLAMEBODY,
    RECKLESS,
    PRESSURE,
    GOOEY,
    IMMUNITY,
    LEAFGUARD,
    HUGEPOWER,
    SOLARPOWER,
    SCHOOLING,
    MOTORDRIVE,
    ANTICIPATION,
    MERCILESS,
    TRACE,
    NATURALCURE,
    HARVEST,
    SUCTIONCUPS,
    ICEFACE,
    ROUGHSKIN,
    WONDERGUARD,
    WATERVEIL,
    FAIRYAURA,
    SANDSPIT,
    INTIMIDATE,
    DAUNTLESSSHIELD,
    AROMAVEIL,
    AIRLOCK,
    NORMALIZE,
    DARKAURA,
    VICTORYSTAR,
    GRASSYSURGE,
    STURDY,
    PICKPOCKET,
    ELECTRICSURGE,
    RUNAWAY,
    OBLIVIOUS,
    SURGESURFER,
    LEVITATE,
    ASONESPECTRIER,
    PICKUP,
    ICEBODY,
    CURIOUSMEDICINE,
    FLOWERVEIL,
    STATIC,
    WONDERSKIN,
    OVERGROW,
    PROPELLERTAIL,
    THICKFAT,
    GLUTTONY,
    KEENEYE,
    MOUNTAINEER,
    FLASHFIRE,
    COMPOUNDEYES,
    STEELWORKER,
    COMATOSE,
    BALLFETCH,
    DAZZLING,
    DOWNLOAD,
    TRANSISTOR,
    MOLDBREAKER,
    LIQUIDOOZE,
    POISONHEAL,
    PRISMARMOR,
    SNIPER,
    STENCH,
    COMPETITIVE,
    SWARM,
    STALWART,
    ILLUMINATE,
    TURBOBLAZE,
    GORILLATACTICS,
    SPEEDBOOST,
    HEATPROOF,
    SNOWCLOAK,
    TERAVOLT,
    CHILLINGNEIGH,
    SHIELDDUST,
    RIVALRY,
    PRIMORDIALSEA,
    SCREENCLEANER,
    MAGNETPULL,
    HONEYGATHER,
    COTTONDOWN,
    GRASSPELT,
    BATTLEARMOR,
    BEASTBOOST,
    BERSERK,
    MINUS,
    RAINDISH,
    SYNCHRONIZE,
    FILTER,
    TRUANT,
    FURCOAT,
    FULLMETALBODY,
    REGENERATOR,
    FOREWARN,
    IRONBARBS,
    STAMINA,
    SANDRUSH,
    COLORCHANGE,
    BLAZE,
    ANALYTIC,
    TANGLINGHAIR,
    CLOUDNINE,
    STEELYSPIRIT,
    QUICKFEET,
    MAGICBOUNCE,
    MEGALAUNCHER,
    HEAVYMETAL,
    STORMDRAIN,
    PIXILATE,
    WATERCOMPACTION,
    JUSTIFIED,
    SLOWSTART,
    SNOWWARNING,
    FLOWERGIFT,
    SHEDSKIN,
    WIMPOUT,
    ICESCALES,
    INFILTRATOR,
    LIMBER,
    PSYCHICSURGE,
    DEFEATIST,
    WATERABSORB,
    IMPOSTER,
    DRYSKIN,
    FLUFFY,
    UNBURDEN,
    CHEEKPOUCH,
    STANCECHANGE,
    MOODY,
    ROCKYPAYLOAD,
    PUNKROCK,
    SANDVEIL,
    PARENTALBOND,
    STRONGJAW,
    BATTERY,
    HEALER,
    STEADFAST,
    DAMP,
    PERISHBODY,
    TRIAGE,
    SHEERFORCE,
    OWNTEMPO,
    FRISK,
    VOLTABSORB,
    GALEWINGS,
    AFTERMATH,
    STICKYHOLD,
    GRIMNEIGH,
    IRONFIST,
    REBOUND,
    UNSEENFIST,
    SOLIDROCK,
    HUSTLE,
    HYDRATION,
    SCRAPPY,
    OVERCOAT,
    NEUTRALIZINGGAS,
    SWEETVEIL,
    DRIZZLE,
    INNERFOCUS,
    POISONTOUCH,
    WANDERINGSPIRIT,
    GUTS,
    SHELLARMOR,
    RATTLED,
    WATERBUBBLE,
    SANDFORCE,
    TOXICBOOST,
    PERSISTENT,
    CHLOROPHYLL,
    SIMPLE,
    NONE,
    PURIFYINGSALT,
}

// https://bulbapedia.bulbagarden.net/wiki/Ignoring_Abilities#Ignorable_Abilities
fn mold_breaker_ignores(ability: &Abilities) -> bool {
    match ability {
        Abilities::BATTLEARMOR
        | Abilities::CLEARBODY
        | Abilities::DAMP
        | Abilities::DRYSKIN
        | Abilities::FILTER
        | Abilities::FLASHFIRE
        | Abilities::FLOWERGIFT
        | Abilities::HEATPROOF
        | Abilities::HYPERCUTTER
        | Abilities::IMMUNITY
        | Abilities::INNERFOCUS
        | Abilities::INSOMNIA
        | Abilities::KEENEYE
        | Abilities::LEAFGUARD
        | Abilities::LEVITATE
        | Abilities::LIGHTNINGROD
        | Abilities::LIMBER
        | Abilities::MAGMAARMOR
        | Abilities::MARVELSCALE
        | Abilities::MOTORDRIVE
        | Abilities::OBLIVIOUS
        | Abilities::OWNTEMPO
        | Abilities::SANDVEIL
        | Abilities::SHELLARMOR
        | Abilities::SHIELDDUST
        | Abilities::SIMPLE
        | Abilities::SNOWCLOAK
        | Abilities::SOLIDROCK
        | Abilities::SOUNDPROOF
        | Abilities::STICKYHOLD
        | Abilities::STORMDRAIN
        | Abilities::STURDY
        | Abilities::SUCTIONCUPS
        | Abilities::TANGLEDFEET
        | Abilities::THICKFAT
        | Abilities::UNAWARE
        | Abilities::VITALSPIRIT
        | Abilities::VOLTABSORB
        | Abilities::WATERABSORB
        | Abilities::WATERVEIL
        | Abilities::WHITESMOKE
        | Abilities::WONDERGUARD
        | Abilities::BIGPECKS
        | Abilities::CONTRARY
        | Abilities::FRIENDGUARD
        | Abilities::HEAVYMETAL
        | Abilities::LIGHTMETAL
        | Abilities::MAGICBOUNCE
        | Abilities::MULTISCALE
        | Abilities::SAPSIPPER
        | Abilities::TELEPATHY
        | Abilities::WONDERSKIN
        | Abilities::AURABREAK
        | Abilities::AROMAVEIL
        | Abilities::BULLETPROOF
        | Abilities::FLOWERVEIL
        | Abilities::FURCOAT
        | Abilities::OVERCOAT
        | Abilities::SWEETVEIL
        | Abilities::DAZZLING
        | Abilities::DISGUISE
        | Abilities::FLUFFY
        | Abilities::QUEENLYMAJESTY
        | Abilities::WATERBUBBLE
        | Abilities::ICESCALES
        | Abilities::ICEFACE
        | Abilities::MIRRORARMOR
        | Abilities::PASTELVEIL
        | Abilities::PUNKROCK
        | Abilities::FAIRYAURA
        | Abilities::DARKAURA => true,
        _ => false,
    }
}

pub fn ability_before_move(
    state: &mut State,
    choice: &Choice,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let active_pkmn = attacking_side.get_active();
    match active_pkmn.ability {
        Abilities::PROTEAN | Abilities::LIBERO => {
            if !active_pkmn.has_type(&choice.move_type) {
                let ins = Instruction::ChangeType(ChangeType {
                    side_ref: *side_ref,
                    new_types: (choice.move_type, PokemonType::Typeless),
                    old_types: active_pkmn.types,
                });
                active_pkmn.types = (choice.move_type, PokemonType::Typeless);
                instructions.instruction_list.push(ins);
            }
        }
        _ => {}
    }
}

pub fn ability_after_damage_hit(
    state: &mut State,
    choice: &Choice,
    side_ref: &SideReference,
    damage_dealt: i16,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let active_pkmn = attacking_side.get_active();
    match active_pkmn.ability {
        Abilities::MAGICIAN | Abilities::PICKPOCKET => {
            let defending_pkmn = defending_side.get_active();
            if damage_dealt > 0
                && defending_pkmn.item_can_be_removed()
                && active_pkmn.item == Items::NONE
            {
                instructions.instruction_list.push(Instruction::ChangeItem(
                    ChangeItemInstruction {
                        side_ref: *side_ref,
                        current_item: active_pkmn.item,
                        new_item: defending_pkmn.item,
                    },
                ));
                active_pkmn.item = defending_pkmn.item;
                instructions.instruction_list.push(Instruction::ChangeItem(
                    ChangeItemInstruction {
                        side_ref: side_ref.get_other_side(),
                        current_item: defending_pkmn.item,
                        new_item: Items::NONE,
                    },
                ));
                defending_pkmn.item = Items::NONE;
            }
        }
        Abilities::MOXIE => {
            if damage_dealt > 0 && defending_side.get_active_immutable().hp == 0 {
                if let Some(boost_instruction) = get_boost_instruction(
                    &attacking_side,
                    &PokemonBoostableStat::Attack,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                }
            }
        }
        Abilities::BEASTBOOST => {
            if damage_dealt > 0 && defending_side.get_active_immutable().hp == 0 {
                let highest_stat = &active_pkmn.calculate_highest_stat();
                if let Some(boost_instruction) =
                    get_boost_instruction(&attacking_side, highest_stat, &1, side_ref, side_ref)
                {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                }
            }
        }
        _ => {}
    }
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
                    new_types: (choice.move_type, PokemonType::Typeless),
                    old_types: defending_pkmn.types,
                });
                defending_pkmn.types = (choice.move_type, PokemonType::Typeless);
                instructions.instruction_list.push(change_type_instruction);
            }
        }
        Abilities::STAMINA => {
            if damage_dealt > 0 && defending_pkmn.hp != 0 {
                if let Some(boost_instruction) = get_boost_instruction(
                    &defending_side,
                    &PokemonBoostableStat::Defense,
                    &1,
                    side_ref,
                    &side_ref.get_other_side(),
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                }
            }
        }
        Abilities::COTTONDOWN => {
            if damage_dealt > 0 {
                if let Some(boost_instruction) = get_boost_instruction(
                    &attacking_side,
                    &PokemonBoostableStat::Speed,
                    &-1,
                    &side_ref.get_other_side(),
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                }
            }
        }
        Abilities::BERSERK => {
            if damage_dealt > 0
                && defending_pkmn.hp < defending_pkmn.maxhp / 2
                && defending_pkmn.hp + damage_dealt >= defending_pkmn.maxhp / 2
            {
                if let Some(boost_instruction) = get_boost_instruction(
                    &defending_side,
                    &PokemonBoostableStat::SpecialAttack,
                    &1,
                    &side_ref.get_other_side(),
                    &side_ref.get_other_side(),
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                }
            }
        }
        Abilities::ROUGHSKIN | Abilities::IRONBARBS => {
            if damage_dealt > 0 && choice.flags.contact {
                let damage_dealt = cmp::min(attacking_pkmn.maxhp / 8, attacking_pkmn.hp);
                instructions
                    .instruction_list
                    .push(Instruction::Damage(DamageInstruction {
                        side_ref: *side_ref,
                        damage_amount: damage_dealt,
                    }));
                attacking_pkmn.hp -= damage_dealt;
            }
        }
        Abilities::AFTERMATH => {
            if damage_dealt > 0
                && defending_side.get_active_immutable().hp == 0
                && choice.flags.contact
            {
                let damage_dealt = cmp::min(attacking_pkmn.maxhp / 4, attacking_pkmn.hp);
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
            if active_pkmn.status != PokemonStatus::None {
                let status = active_pkmn.status.clone();
                active_pkmn.status = PokemonStatus::None;
                instructions
                    .instruction_list
                    .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: *side_ref,
                        pokemon_index: attacking_side.active_index,
                        old_status: status,
                        new_status: PokemonStatus::None,
                    }));
            }
        }
        Abilities::REGENERATOR => {
            let hp_recovered = cmp::min(active_pkmn.maxhp / 3, active_pkmn.maxhp - active_pkmn.hp);

            if hp_recovered > 0 && active_pkmn.hp > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::Heal(HealInstruction {
                        side_ref: *side_ref,
                        heal_amount: hp_recovered,
                    }));
                active_pkmn.hp += hp_recovered;
            }
        }
        _ => {}
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
        Abilities::BADDREAMS => {
            let defender = defending_side.get_active();
            if defender.status == PokemonStatus::Sleep {
                let damage_dealt = cmp::min(defender.maxhp / 8, defender.hp);
                instructions
                    .instruction_list
                    .push(Instruction::Damage(DamageInstruction {
                        side_ref: side_ref.get_other_side(),
                        damage_amount: damage_dealt,
                    }));
                defender.hp -= damage_dealt;
            }
        }
        Abilities::SOLARPOWER => {
            if state.weather_is_active(&Weather::HarshSun) || state.weather_is_active(&Weather::Sun)
            {
                let active_pkmn = state.get_side(side_ref).get_active();
                let damage_dealt =
                    cmp::min(active_pkmn.maxhp / 8, active_pkmn.maxhp - active_pkmn.hp);
                if damage_dealt > 0 {
                    instructions
                        .instruction_list
                        .push(Instruction::Damage(DamageInstruction {
                            side_ref: *side_ref,
                            damage_amount: damage_dealt,
                        }));
                    active_pkmn.hp -= damage_dealt;
                }
            }
        }
        Abilities::ICEBODY => {
            if state.weather_is_active(&Weather::Hail) {
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
        Abilities::POISONHEAL => {
            if active_pkmn.hp < active_pkmn.maxhp
                && (active_pkmn.status == PokemonStatus::Poison
                    || active_pkmn.status == PokemonStatus::Toxic)
            {
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
            if state.weather_is_active(&Weather::Rain)
                || state.weather_is_active(&Weather::HeavyRain)
            {
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
            if state.weather_is_active(&Weather::Rain) {
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
        Abilities::HYDRATION => {
            if active_pkmn.status != PokemonStatus::None
                && (state.weather_is_active(&Weather::Rain)
                    || state.weather_is_active(&Weather::HeavyRain))
            {
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
        // Shed skin only has a 1/3 chance of activating at the end of the turn
        // but I'm not going to branch on that here
        Abilities::SHEDSKIN => {
            if active_pkmn.status != PokemonStatus::None {
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
    match active_pkmn.ability {
        Abilities::INTREPIDSWORD => {
            // no need to check for boost at +6 because we are switching in
            attacking_side.attack_boost += 1;
            instructions
                .instruction_list
                .push(Instruction::Boost(BoostInstruction {
                    side_ref: *side_ref,
                    stat: PokemonBoostableStat::Attack,
                    amount: 1,
                }));
        }
        Abilities::SLOWSTART => {
            attacking_side
                .volatile_statuses
                .insert(PokemonVolatileStatus::SlowStart);
            instructions
                .instruction_list
                .push(Instruction::ApplyVolatileStatus(
                    ApplyVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: PokemonVolatileStatus::SlowStart,
                    },
                ));
        }
        Abilities::DROUGHT => {
            if state.weather.weather_type != Weather::Sun {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::Sun,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::Sun;
                state.weather.turns_remaining = 5;
            }
        }
        Abilities::DESOLATELAND => {
            if state.weather.weather_type != Weather::HarshSun {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::HarshSun,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::HarshSun;
                state.weather.turns_remaining = 5;
            }
        }
        Abilities::MISTYSURGE => {
            if state.terrain.terrain_type != Terrain::MistyTerrain {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeTerrain(ChangeTerrain {
                        new_terrain: Terrain::MistyTerrain,
                        new_terrain_turns_remaining: 5,
                        previous_terrain: state.terrain.terrain_type,
                        previous_terrain_turns_remaining: state.terrain.turns_remaining,
                    }));
                state.terrain.terrain_type = Terrain::MistyTerrain;
                state.terrain.turns_remaining = 5;
            }
        }
        Abilities::SANDSTREAM => {
            if state.weather.weather_type != Weather::Sand {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::Sand,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::Sand;
                state.weather.turns_remaining = 5;
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
                match defending_side.get_active_immutable().ability {
                    Abilities::OWNTEMPO
                    | Abilities::OBLIVIOUS
                    | Abilities::INNERFOCUS
                    | Abilities::SCRAPPY => {}
                    _ => {
                        state.apply_one_instruction(&boost_instruction);
                        instructions.instruction_list.push(boost_instruction);
                    }
                }
            }
        }
        Abilities::DAUNTLESSSHIELD => {
            // no need to check for boost at +6 because we are switching in
            attacking_side.defense_boost += 1;
            instructions
                .instruction_list
                .push(Instruction::Boost(BoostInstruction {
                    side_ref: *side_ref,
                    stat: PokemonBoostableStat::Defense,
                    amount: 1,
                }));
        }
        Abilities::GRASSYSURGE => {
            if state.terrain.terrain_type != Terrain::GrassyTerrain {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeTerrain(ChangeTerrain {
                        new_terrain: Terrain::GrassyTerrain,
                        new_terrain_turns_remaining: 5,
                        previous_terrain: state.terrain.terrain_type,
                        previous_terrain_turns_remaining: state.terrain.turns_remaining,
                    }));
                state.terrain.terrain_type = Terrain::GrassyTerrain;
                state.terrain.turns_remaining = 5;
            }
        }
        Abilities::ELECTRICSURGE => {
            if state.terrain.terrain_type != Terrain::ElectricTerrain {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeTerrain(ChangeTerrain {
                        new_terrain: Terrain::ElectricTerrain,
                        new_terrain_turns_remaining: 5,
                        previous_terrain: state.terrain.terrain_type,
                        previous_terrain_turns_remaining: state.terrain.turns_remaining,
                    }));
                state.terrain.terrain_type = Terrain::ElectricTerrain;
                state.terrain.turns_remaining = 5;
            }
        }
        Abilities::DOWNLOAD => {
            if defending_side.calculate_boosted_stat(PokemonBoostableStat::Defense)
                < defending_side.calculate_boosted_stat(PokemonBoostableStat::SpecialDefense)
            {
                if let Some(boost_instruction) = get_boost_instruction(
                    &attacking_side,
                    &PokemonBoostableStat::Attack,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                }
            } else {
                if let Some(boost_instruction) = get_boost_instruction(
                    &attacking_side,
                    &PokemonBoostableStat::SpecialAttack,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                }
            }
        }
        Abilities::PRIMORDIALSEA => {
            if state.weather.weather_type != Weather::HeavyRain {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::HeavyRain,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::HeavyRain;
                state.weather.turns_remaining = 5;
            }
        }
        Abilities::SCREENCLEANER => {
            if state.side_one.side_conditions.reflect > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: SideReference::SideOne,
                            side_condition: PokemonSideCondition::Reflect,
                            amount: -1 * state.side_one.side_conditions.reflect,
                        },
                    ));
                state.side_one.side_conditions.reflect = 0;
            }
            if state.side_two.side_conditions.reflect > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: SideReference::SideTwo,
                            side_condition: PokemonSideCondition::Reflect,
                            amount: -1 * state.side_two.side_conditions.reflect,
                        },
                    ));
                state.side_two.side_conditions.reflect = 0;
            }
            if state.side_one.side_conditions.light_screen > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: SideReference::SideOne,
                            side_condition: PokemonSideCondition::LightScreen,
                            amount: -1 * state.side_one.side_conditions.light_screen,
                        },
                    ));
                state.side_one.side_conditions.light_screen = 0;
            }
            if state.side_two.side_conditions.light_screen > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: SideReference::SideTwo,
                            side_condition: PokemonSideCondition::LightScreen,
                            amount: -1 * state.side_two.side_conditions.light_screen,
                        },
                    ));
                state.side_two.side_conditions.light_screen = 0;
            }
            if state.side_one.side_conditions.aurora_veil > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: SideReference::SideOne,
                            side_condition: PokemonSideCondition::AuroraVeil,
                            amount: -1 * state.side_one.side_conditions.aurora_veil,
                        },
                    ));
                state.side_one.side_conditions.aurora_veil = 0;
            }
            if state.side_two.side_conditions.aurora_veil > 0 {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeSideCondition(
                        ChangeSideConditionInstruction {
                            side_ref: SideReference::SideTwo,
                            side_condition: PokemonSideCondition::AuroraVeil,
                            amount: -1 * state.side_two.side_conditions.aurora_veil,
                        },
                    ));
                state.side_two.side_conditions.aurora_veil = 0;
            }
        }
        Abilities::SNOWWARNING => {
            if state.weather.weather_type != Weather::Hail {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::Hail,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::Hail;
                state.weather.turns_remaining = 5;
            }
        }
        Abilities::PSYCHICSURGE => {
            if state.terrain.terrain_type != Terrain::PsychicTerrain {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeTerrain(ChangeTerrain {
                        new_terrain: Terrain::PsychicTerrain,
                        new_terrain_turns_remaining: 5,
                        previous_terrain: state.terrain.terrain_type,
                        previous_terrain_turns_remaining: state.terrain.turns_remaining,
                    }));
                state.terrain.terrain_type = Terrain::PsychicTerrain;
                state.terrain.turns_remaining = 5;
            }
        }
        Abilities::DRIZZLE => {
            if state.weather.weather_type != Weather::Rain {
                instructions
                    .instruction_list
                    .push(Instruction::ChangeWeather(ChangeWeather {
                        new_weather: Weather::Rain,
                        new_weather_turns_remaining: 5,
                        previous_weather: state.weather.weather_type,
                        previous_weather_turns_remaining: state.weather.turns_remaining,
                    }));
                state.weather.weather_type = Weather::Rain;
                state.weather.turns_remaining = 5;
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
        Abilities::DRAGONSMAW => {
            if attacker_choice.move_type == PokemonType::Dragon {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::GALVANIZE => {
            if attacker_choice.move_type == PokemonType::Normal {
                attacker_choice.move_type = PokemonType::Electric;
                attacker_choice.base_power *= 1.2;
            }
        }
        #[cfg(any(feature = "gen9", feature = "gen8", feature = "gen7"))]
        Abilities::AERILATE => {
            if attacker_choice.move_type == PokemonType::Normal {
                attacker_choice.move_type = PokemonType::Flying;
                attacker_choice.base_power *= 1.2;
            }
        }
        #[cfg(any(feature = "gen6", feature = "gen5", feature = "gen4"))]
        Abilities::AERILATE => {
            if attacker_choice.move_type == PokemonType::Normal {
                attacker_choice.move_type = PokemonType::Flying;
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::NEUROFORCE => {
            if type_effectiveness_modifier(
                &attacker_choice.move_type,
                &defending_side.get_active_immutable().types,
            ) > 1.0
            {
                attacker_choice.base_power *= 1.25;
            }
        }
        Abilities::STAKEOUT => {
            if defender_choice.category == MoveCategory::Switch {
                attacker_choice.base_power *= 2.0;
            }
        }
        Abilities::TECHNICIAN => {
            if attacker_choice.base_power <= 60.0 {
                attacker_choice.base_power *= 1.5;
            }
        }
        #[cfg(any(feature = "gen9", feature = "gen8", feature = "gen7"))]
        Abilities::REFRIGERATE => {
            if attacker_choice.move_type == PokemonType::Normal {
                attacker_choice.move_type = PokemonType::Ice;
                attacker_choice.base_power *= 1.2;
            }
        }
        #[cfg(any(feature = "gen6", feature = "gen5", feature = "gen4"))]
        Abilities::REFRIGERATE => {
            if attacker_choice.move_type == PokemonType::Normal {
                attacker_choice.move_type = PokemonType::Ice;
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::SUPREMEOVERLORD => {
            let mut boost_amount = 1.0;
            boost_amount += 0.1 * attacking_side.num_alive_pkmn() as f32;
            attacker_choice.base_power *= boost_amount;
        }
        Abilities::ADAPTABILITY => {
            if attacking_pkmn.has_type(&attacker_choice.move_type) {
                attacker_choice.base_power *= 4.0 / 3.0;
            }
        }
        Abilities::LONGREACH => {
            attacker_choice.flags.contact = false;
        }
        Abilities::PUREPOWER => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.base_power *= 2.0;
            }
        }
        Abilities::TINTEDLENS => {
            if type_effectiveness_modifier(
                &attacker_choice.move_type,
                &defending_side.get_active_immutable().types,
            ) < 1.0
            {
                attacker_choice.base_power *= 2.0;
            }
        }
        Abilities::FLAREBOOST => {
            if attacking_pkmn.status == PokemonStatus::Burn {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::LIQUIDVOICE => {
            if attacker_choice.flags.sound {
                attacker_choice.move_type = PokemonType::Water;
            }
        }
        Abilities::NOGUARD => attacker_choice.accuracy = 100.0,
        Abilities::TORRENT => {
            if attacker_choice.move_type == PokemonType::Water
                && attacking_pkmn.hp < attacking_pkmn.maxhp / 3
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
        Abilities::TOUGHCLAWS => {
            if attacker_choice.flags.contact {
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::RECKLESS => {
            if attacker_choice.crash.is_some() || attacker_choice.recoil.is_some() {
                attacker_choice.base_power *= 1.2;
            }
        }
        Abilities::HUGEPOWER => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.base_power *= 2.0;
            }
        }
        Abilities::SOLARPOWER => {
            if state.weather_is_active(&Weather::Sun) {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::FAIRYAURA => {
            if attacker_choice.move_type == PokemonType::Fairy {
                attacker_choice.base_power *= 1.33;
            }
        }
        Abilities::NORMALIZE => {
            attacker_choice.move_type = PokemonType::Normal;
        }
        Abilities::DARKAURA => {
            if attacker_choice.move_type == PokemonType::Dark {
                attacker_choice.base_power *= 1.33;
            }
        }
        Abilities::VICTORYSTAR => {
            attacker_choice.accuracy *= 1.1;
        }
        Abilities::COMPOUNDEYES => {
            attacker_choice.accuracy *= 1.3;
        }
        Abilities::STEELWORKER => {
            if attacker_choice.move_type == PokemonType::Steel {
                attacker_choice.base_power *= 1.5;
            }
        }
        #[cfg(any(
            feature = "gen8",
            feature = "gen7",
            feature = "gen6",
            feature = "gen5",
            feature = "gen4"
        ))]
        Abilities::TRANSISTOR => {
            if attacker_choice.move_type == PokemonType::Electric {
                attacker_choice.base_power *= 1.5;
            }
        }
        #[cfg(any(feature = "gen9"))]
        Abilities::TRANSISTOR => {
            if attacker_choice.move_type == PokemonType::Electric {
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::STENCH => {
            let mut already_flinches = false;
            if let Some(secondaries) = &mut attacker_choice.secondaries {
                for secondary in secondaries.iter() {
                    if secondary.effect == Effect::VolatileStatus(PokemonVolatileStatus::Flinch) {
                        already_flinches = true;
                    }
                }
            }
            if !already_flinches {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                })
            }
        }
        Abilities::SWARM => {
            if attacker_choice.move_type == PokemonType::Bug
                && attacking_pkmn.hp < attacking_pkmn.maxhp / 3
            {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::GORILLATACTICS => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::BLAZE => {
            if attacker_choice.move_type == PokemonType::Fire
                && attacking_pkmn.hp < attacking_pkmn.maxhp / 3
            {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::OVERGROW => {
            if attacker_choice.move_type == PokemonType::Grass
                && attacking_pkmn.hp < attacking_pkmn.maxhp / 3
            {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::ANALYTIC => {
            if !attacker_choice.first_move {
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::MEGALAUNCHER => {
            if attacker_choice.flags.pulse {
                attacker_choice.base_power *= 1.5;
            };
        }
        #[cfg(any(feature = "gen9", feature = "gen8", feature = "gen7"))]
        Abilities::PIXILATE => {
            if attacker_choice.move_type == PokemonType::Normal {
                attacker_choice.move_type = PokemonType::Fairy;
                attacker_choice.base_power *= 1.2;
            }
        }
        #[cfg(any(feature = "gen6", feature = "gen5", feature = "gen4"))]
        Abilities::PIXILATE => {
            if attacker_choice.move_type == PokemonType::Normal {
                attacker_choice.move_type = PokemonType::Fairy;
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::DEFEATIST => {
            if attacking_pkmn.hp < attacking_pkmn.maxhp / 2 {
                attacker_choice.base_power *= 0.5;
            }
        }
        Abilities::ROCKYPAYLOAD => {
            if attacker_choice.move_type == PokemonType::Rock {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::PUNKROCK => {
            if attacker_choice.flags.sound {
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::STRONGJAW => {
            if attacker_choice.flags.bite {
                attacker_choice.base_power *= 1.5;
            }
        }
        Abilities::BATTERY => {
            if attacker_choice.category == MoveCategory::Special {
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::SHEERFORCE => {
            if attacker_choice.secondaries.is_some() {
                attacker_choice.base_power *= 1.3;
                attacker_choice.secondaries = None
            }
        }
        Abilities::IRONFIST => {
            if attacker_choice.flags.punch {
                attacker_choice.base_power *= 1.2;
            }
        }
        Abilities::UNSEENFIST => {
            if attacker_choice.flags.contact {
                attacker_choice.flags.protect = false
            }
        }
        Abilities::HUSTLE => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.base_power *= 1.5;
                attacker_choice.accuracy *= 0.80
            }
        }
        Abilities::POISONTOUCH => {
            if attacker_choice.flags.contact {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                })
            }
        }
        Abilities::GUTS => {
            if attacking_pkmn.status != PokemonStatus::None {
                attacker_choice.base_power *= 1.5;

                // not the right place to put this, but good enough
                if attacking_pkmn.status == PokemonStatus::Burn {
                    attacker_choice.base_power *= 2.0;
                }
            }
        }
        Abilities::SANDFORCE => {
            if state.weather_is_active(&Weather::Sand)
                && (attacker_choice.move_type == PokemonType::Rock
                    || attacker_choice.move_type == PokemonType::Ground
                    || attacker_choice.move_type == PokemonType::Steel)
            {
                attacker_choice.base_power *= 1.3;
            }
        }
        Abilities::TOXICBOOST => {
            if attacking_pkmn.status == PokemonStatus::Poison
                || attacking_pkmn.status == PokemonStatus::Toxic
            {
                attacker_choice.base_power *= 1.5;
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
    if (attacking_pkmn.ability == Abilities::MOLDBREAKER
        || attacking_pkmn.ability == Abilities::TERAVOLT
        || attacking_pkmn.ability == Abilities::TURBOBLAZE)
        && mold_breaker_ignores(&target_pkmn.ability)
    {
        return;
    }

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
                    effect: Effect::Status(PokemonStatus::Poison),
                })
            }
        }
        Abilities::BULLETPROOF => {
            if attacker_choice.flags.bullet {
                attacker_choice.remove_all_effects();
                attacker_choice.accuracy = 0.0;
            }
        }
        Abilities::MULTISCALE => {
            if target_pkmn.hp == target_pkmn.maxhp {
                attacker_choice.base_power /= 2.0;
            }
        }
        Abilities::LIGHTNINGROD => {
            if attacker_choice.move_type == PokemonType::Electric {
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
        Abilities::EARTHEATER => {
            if attacker_choice.move_type == PokemonType::Ground {
                attacker_choice.remove_all_effects();
                attacker_choice.base_power = 0.0;
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::Opponent,
                    amount: 0.25,
                });
                attacker_choice.category = MoveCategory::Status;
            }
        }
        Abilities::STEAMENGINE => {
            if attacker_choice.move_type == PokemonType::Water
                || attacker_choice.move_type == PokemonType::Fire
            {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 6,
                        accuracy: 0,
                    }),
                });
            }
        }
        #[cfg(any(feature = "gen9", feature = "gen8", feature = "gen7"))]
        Abilities::WEAKARMOR => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    }),
                });
            }
        }
        #[cfg(any(feature = "gen6", feature = "gen5", feature = "gen4"))]
        Abilities::WEAKARMOR => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    }),
                });
            }
        }
        Abilities::QUEENLYMAJESTY => {
            if attacker_choice.priority > 0 {
                attacker_choice.remove_all_effects();
                attacker_choice.accuracy = 0.0;
            }
        }
        Abilities::SAPSIPPER => {
            if attacker_choice.move_type == PokemonType::Grass {
                attacker_choice.remove_all_effects();
                attacker_choice.accuracy = 100.0;
                attacker_choice.target = MoveTarget::Opponent;
                attacker_choice.boost = Some(Boost {
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                    target: MoveTarget::Opponent,
                });
                attacker_choice.category = MoveCategory::Status;
            }
        }
        Abilities::SHADOWSHIELD => {
            if target_pkmn.hp == target_pkmn.maxhp {
                attacker_choice.base_power /= 2.0;
            }
        }
        Abilities::NOGUARD => {
            attacker_choice.accuracy = 100.0;
        }
        Abilities::MARVELSCALE => {
            if target_pkmn.status != PokemonStatus::None
                && attacker_choice.category == MoveCategory::Physical
            {
                attacker_choice.base_power /= 1.5;
            }
        }
        Abilities::EFFECTSPORE => {
            if attacker_choice.flags.contact {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 9.0,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::Poison),
                });
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 10.0,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                });
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 11.0,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::Sleep),
                });
            }
        }
        Abilities::FLAMEBODY => {
            if state.move_makes_contact(&attacker_choice, attacking_side_ref) {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 30.0,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::Burn),
                });
            }
        }
        Abilities::GOOEY => {
            if attacker_choice.flags.contact {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                })
            }
        }
        Abilities::MOTORDRIVE => {
            if attacker_choice.move_type == PokemonType::Electric {
                attacker_choice.remove_all_effects();
                attacker_choice.accuracy = 100.0;
                attacker_choice.target = MoveTarget::Opponent;
                attacker_choice.boost = Some(Boost {
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                    target: MoveTarget::Opponent,
                });
                attacker_choice.category = MoveCategory::Status;
            }
        }
        Abilities::SUCTIONCUPS => {
            attacker_choice.flags.drag = false;
        }
        Abilities::WONDERGUARD => {
            if attacker_choice.category != MoveCategory::Status
                && type_effectiveness_modifier(&attacker_choice.move_type, &target_pkmn.types)
                    <= 1.0
            {
                attacker_choice.remove_all_effects();
                attacker_choice.base_power = 0.0;
            }
        }
        Abilities::FAIRYAURA => {
            if attacker_choice.move_type == PokemonType::Fairy {
                attacker_choice.base_power *= 1.33;
            }
        }
        Abilities::LEVITATE => {
            if attacker_choice.move_type == PokemonType::Ground
                && attacker_choice.target == MoveTarget::Opponent
                && attacker_choice.move_id != Choices::THOUSANDARROWS
            {
                attacker_choice.base_power = 0.0;
            }
        }
        Abilities::STATIC => {
            if state.move_makes_contact(&attacker_choice, attacking_side_ref) {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 30.0,
                    target: MoveTarget::User,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                })
            }
        }
        Abilities::WONDERSKIN => {
            if attacker_choice.category == MoveCategory::Status && attacker_choice.accuracy > 50.0 {
                attacker_choice.accuracy = 50.0;
            }
        }
        Abilities::THICKFAT => {
            if attacker_choice.move_type == PokemonType::Fire
                || attacker_choice.move_type == PokemonType::Ice
            {
                attacker_choice.base_power /= 2.0;
            }
        }
        Abilities::FLASHFIRE => {
            if attacker_choice.move_type == PokemonType::Fire {
                attacker_choice.remove_all_effects();
                attacker_choice.volatile_status = Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::FlashFire,
                });
            }
        }
        Abilities::DAZZLING => {
            if attacker_choice.priority > 0 {
                attacker_choice.accuracy = 0.0;
            }
        }
        Abilities::LIQUIDOOZE => {
            if let Some(drain) = attacker_choice.drain {
                attacker_choice.drain = Some(-1.0 * drain);
            }
        }
        Abilities::PRISMARMOR => {
            if type_effectiveness_modifier(&attacker_choice.move_type, &target_pkmn.types) > 1.0 {
                attacker_choice.base_power *= 0.75;
            }
        }
        Abilities::HEATPROOF => {
            if attacker_choice.move_type == PokemonType::Fire {
                attacker_choice.base_power *= 0.5;
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
        Abilities::GRASSPELT => {
            if state.terrain_is_active(&Terrain::GrassyTerrain)
                && attacker_choice.category == MoveCategory::Physical
            {
                attacker_choice.base_power /= 1.5;
            }
        }
        Abilities::FILTER => {
            if type_effectiveness_modifier(&attacker_choice.move_type, &target_pkmn.types) > 1.0 {
                attacker_choice.base_power *= 0.75;
            }
        }
        Abilities::FURCOAT => {
            if attacker_choice.category == MoveCategory::Physical {
                attacker_choice.base_power *= 0.5;
            }
        }
        Abilities::TANGLINGHAIR => {
            if attacker_choice.flags.contact {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                })
            }
        }
        Abilities::MAGICBOUNCE => {
            if attacker_choice.flags.reflectable {
                attacker_choice.target = MoveTarget::User;
                if let Some(side_condition) = &mut attacker_choice.side_condition {
                    if side_condition.target == MoveTarget::Opponent {
                        side_condition.target = MoveTarget::User;
                    }
                }
                if let Some(status) = &mut attacker_choice.status {
                    if status.target == MoveTarget::Opponent {
                        status.target = MoveTarget::User;
                    }
                }
            }
        }
        Abilities::STORMDRAIN => {
            if attacker_choice.move_type == PokemonType::Water {
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
        Abilities::WATERCOMPACTION => {
            if attacker_choice.move_type == PokemonType::Water {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                });
            }
        }
        Abilities::JUSTIFIED => {
            if attacker_choice.move_type == PokemonType::Dark {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                })
            }
        }
        Abilities::ICESCALES => {
            if attacker_choice.category == MoveCategory::Special {
                attacker_choice.base_power *= 0.5;
            }
        }
        Abilities::WATERABSORB => {
            if attacker_choice.move_type == PokemonType::Water {
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
            if attacker_choice.move_type == PokemonType::Water {
                attacker_choice.remove_all_effects();
                attacker_choice.base_power = 0.0;
                attacker_choice.heal = Some(Heal {
                    target: MoveTarget::Opponent,
                    amount: 0.25,
                });
                attacker_choice.category = MoveCategory::Status;
            } else if attacker_choice.move_type == PokemonType::Fire {
                attacker_choice.base_power *= 1.25;
            }
        }
        Abilities::FLUFFY => {
            if attacker_choice.flags.contact {
                attacker_choice.base_power *= 0.5;
            }
            if attacker_choice.move_type == PokemonType::Fire {
                attacker_choice.base_power *= 2.0;
            }
        }
        Abilities::PUNKROCK => {
            if attacker_choice.flags.sound {
                attacker_choice.base_power /= 2.0;
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
            if attacker_choice.move_type == PokemonType::Electric {
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
        Abilities::SOLIDROCK => {
            if type_effectiveness_modifier(&attacker_choice.move_type, &target_pkmn.types) > 1.0 {
                attacker_choice.base_power *= 0.75;
            }
        }
        Abilities::OVERCOAT => {
            if attacker_choice.flags.powder {
                attacker_choice.remove_all_effects();
                attacker_choice.accuracy = 0.0
            }
        }
        Abilities::RATTLED => {
            if attacker_choice.move_type == PokemonType::Bug
                || attacker_choice.move_type == PokemonType::Dark
                || attacker_choice.move_type == PokemonType::Ghost
            {
                attacker_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    }),
                });
            }
        }
        Abilities::WATERBUBBLE => {
            if attacker_choice.move_type == PokemonType::Fire {
                attacker_choice.base_power /= 2.0;
            }
        }
        Abilities::PURIFYINGSALT => {
            if attacker_choice.move_type == PokemonType::Ghost {
                attacker_choice.base_power /= 2.0;
            }
        }
        _ => {}
    }
}
