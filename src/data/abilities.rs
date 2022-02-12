use std::collections::HashMap;

use lazy_static::lazy_static;

use super::moves::get_move;
use super::moves::MoveCategory;

use crate::instruction::Instruction;
use crate::instruction::ChangeStatusInstruction;
use crate::state::Pokemon;
use crate::state::State;
use crate::state::Side;
use crate::data::conditions::Status;
use crate::state::PokemonTypes;
use crate::state::Terrain;
use crate::state::Weather;

type ModifySpeedFn = fn(&State, &Pokemon) -> f32;
type ModifyPriorityFn = fn(&str, &Pokemon) -> i8;
type BeforeSwitchOutFn = fn(
    &mut Side,
    is_side_one: bool
) -> Option<Vec<Instruction>>;

lazy_static! {
    static ref ABILITIES: HashMap<String, Ability> = {
        let mut abilities: HashMap<String, Ability> = HashMap::new();

        abilities.insert(
            "ripen".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "tangledfeet".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "dragonsmaw".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "clearbody".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "galvanize".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "vitalspirit".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "aerilate".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "defiant".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "cutecharm".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "neuroforce".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "soundproof".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "rkssystem".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "poisonpoint".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "stakeout".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "unnerve".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "rockhead".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "aurabreak".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "mimicry".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "bulletproof".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "powerofalchemy".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "technician".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "multiscale".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "arenatrap".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "battlebond".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "disguise".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "earlybird".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "lightningrod".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "magician".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "refrigerate".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "friendguard".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "noability".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "gulpmissile".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "powerconstruct".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "forecast".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "prankster".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: Some(|move_name, _pkmn| {
                    if get_move(move_name).category == MoveCategory::Status {
                        return 1;
                    }
                    return 0;
                }),
                before_switch_out: None,
            },
        );

        abilities.insert(
            "protean".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "asoneglastrier".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "shadowtag".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "skilllink".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "intrepidsword".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "soulheart".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "swiftswim".to_string(),
            Ability {
                modify_speed: Some(|state, _pkmn| {
                    if state.weather.weather_type == Weather::Rain
                        || state.weather.weather_type == Weather::HeavyRain
                    {
                        return 2.0;
                    };
                    return 1.0;
                }),
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "superluck".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "insomnia".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "dancer".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "steamengine".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "angerpoint".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "contrary".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "magmaarmor".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "hungerswitch".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "receiver".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "zenmode".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "emergencyexit".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "illusion".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "weakarmor".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "drought".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "innardsout".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "shieldsdown".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "adaptability".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "corrosion".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "longreach".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "purepower".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "tintedlens".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "queenlymajesty".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "desolateland".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "moxie".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sapsipper".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "slushrush".to_string(),
            Ability {
                modify_speed: Some(|state, _pkmn| {
                    if state.weather.weather_type == Weather::Hail {
                        return 2.0;
                    };
                    return 1.0;
                }),
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "bigpecks".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "stall".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "whitesmoke".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "flareboost".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "shadowshield".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "liquidvoice".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "mistysurge".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "multitype".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "noguard".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "torrent".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "deltastream".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "klutz".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "libero".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "serenegrace".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "cursedbody".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "unaware".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "lightmetal".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "marvelscale".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "telepathy".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "quickdraw".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "hypercutter".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "symbiosis".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "plus".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "mirrorarmor".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "pastelveil".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "toughclaws".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "effectspore".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "mummy".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "baddreams".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "magicguard".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sandstream".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "powerspot".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "flamebody".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "reckless".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "pressure".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "gooey".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "immunity".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "leafguard".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "hugepower".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "solarpower".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "schooling".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "motordrive".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "anticipation".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "merciless".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "trace".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "naturalcure".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: Some(|side, is_side_one| {
                    if side.reserve[side.active_index].status != Status::None {
                        let old_status = side.reserve[side.active_index].status;
                        side.reserve[side.active_index].status = Status::None;
                        return Some(
                            vec![
                                Instruction::ChangeStatus(
                                    ChangeStatusInstruction {
                                        is_side_one: is_side_one,
                                        pokemon_index: side.active_index,
                                        old_status: old_status,
                                        new_status: Status::None,
                                    }
                                )
                            ]
                        );
                    }
                    return None;
                }),
            },
        );

        abilities.insert(
            "harvest".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "suctioncups".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "iceface".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "roughskin".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "wonderguard".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "waterveil".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "fairyaura".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sandspit".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "intimidate".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "dauntlessshield".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "aromaveil".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "airlock".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "normalize".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "darkaura".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "victorystar".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "grassysurge".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sturdy".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "pickpocket".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "electricsurge".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "runaway".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "oblivious".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "surgesurfer".to_string(),
            Ability {
                modify_speed: Some(|state, _pkmn| {
                    if state.terrain.terrain_type == Terrain::ElectricTerrain {
                        return 2.0;
                    };
                    return 1.0;
                }),
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "levitate".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "asonespectrier".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "pickup".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "icebody".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "curiousmedicine".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "flowerveil".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "static".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "wonderskin".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "overgrow".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "propellertail".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "thickfat".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "gluttony".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "keeneye".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "mountaineer".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "flashfire".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "compoundeyes".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "steelworker".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "comatose".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "ballfetch".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "dazzling".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "download".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "transistor".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "moldbreaker".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "liquidooze".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "poisonheal".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "prismarmor".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sniper".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "stench".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "competitive".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "swarm".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "stalwart".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "illuminate".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "turboblaze".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "gorillatactics".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "speedboost".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "heatproof".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "snowcloak".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "teravolt".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "chillingneigh".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "shielddust".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "rivalry".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "primordialsea".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "screencleaner".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "magnetpull".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "honeygather".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "cottondown".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "grasspelt".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "battlearmor".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "beastboost".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "berserk".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "minus".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "raindish".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "synchronize".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "filter".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "truant".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "furcoat".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "fullmetalbody".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "regenerator".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "forewarn".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "ironbarbs".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "stamina".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sandrush".to_string(),
            Ability {
                modify_speed: Some(|state, _pkmn| {
                    if state.weather.weather_type == Weather::Sand {
                        return 2.0;
                    };
                    return 1.0;
                }),
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "colorchange".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "blaze".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "analytic".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "tanglinghair".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "cloudnine".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "steelyspirit".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "quickfeet".to_string(),
            Ability {
                modify_speed: Some(|_state, pkmn| {
                    if pkmn.status != Status::None {
                        return 1.5;
                    };
                    return 1.0;
                }),
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "magicbounce".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "megalauncher".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "heavymetal".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "stormdrain".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "pixilate".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "watercompaction".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "justified".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "slowstart".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "snowwarning".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "flowergift".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "shedskin".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "wimpout".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "icescales".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "infiltrator".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "limber".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "psychicsurge".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "defeatist".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "waterabsorb".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "imposter".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "dryskin".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "fluffy".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "unburden".to_string(),
            Ability {
                modify_speed: Some(|_state, pkmn| {
                    if pkmn.item == "none" {
                        return 2.0;
                    };
                    return 1.0;
                }),
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "cheekpouch".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "stancechange".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "moody".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "punkrock".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sandveil".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "parentalbond".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "strongjaw".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "battery".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "healer".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "steadfast".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "damp".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "perishbody".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "triage".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: Some(|move_name, _pkmn| {
                    if get_move(move_name).flags.heal {
                        return 3;
                    }
                    return 0;
                }),
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sheerforce".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "owntempo".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "frisk".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "voltabsorb".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "galewings".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: Some(|move_name, pkmn| {
                    if pkmn.hp == pkmn.maxhp
                        && get_move(move_name).move_type == PokemonTypes::Flying
                    {
                        return 1;
                    }
                    return 0;
                }),
                before_switch_out: None,
            },
        );

        abilities.insert(
            "aftermath".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "stickyhold".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "grimneigh".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "ironfist".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "rebound".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "unseenfist".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "solidrock".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "hustle".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "hydration".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "scrappy".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "overcoat".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "neutralizinggas".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sweetveil".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "drizzle".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "innerfocus".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "poisontouch".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "wanderingspirit".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "guts".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "shellarmor".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "rattled".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "waterbubble".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "sandforce".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "toxicboost".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "persistent".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "chlorophyll".to_string(),
            Ability {
                modify_speed: Some(|state, _pkmn| {
                    if state.weather.weather_type == Weather::Sun
                        || state.weather.weather_type == Weather::HarshSun
                    {
                        return 2.0;
                    };
                    return 1.0;
                }),
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities.insert(
            "simple".to_string(),
            Ability {
                modify_speed: None,
                modify_priority: None,
                before_switch_out: None,
            },
        );

        abilities
    };
}

pub fn get_ability(ability_name: &str) -> &'static Ability {
    return ABILITIES
        .get(ability_name)
        .unwrap_or_else(|| panic!("Could not get ability {}", ability_name));
}

pub struct Ability {
    pub modify_speed: Option<ModifySpeedFn>,
    pub modify_priority: Option<ModifyPriorityFn>,
    pub before_switch_out: Option<BeforeSwitchOutFn>,
}
