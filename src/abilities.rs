#![allow(unused_variables)]
use std::cmp;

use lazy_static::lazy_static;

use crate::choices::{
    Boost, Choice, Effect, Heal, MoveCategory, MoveTarget, Secondary, StatBoosts, VolatileStatus,
};
use crate::damage_calc::type_effectiveness_modifier;
use crate::generate_instructions::get_boost_instruction;
use crate::instruction::{
    BoostInstruction, ChangeStatusInstruction, ChangeTerrain, ChangeType, ChangeWeather,
    DamageInstruction, HealInstruction, Instruction, StateInstructions,
};
use crate::state::{PokemonBoostableStat, PokemonType, Terrain};
use crate::state::{PokemonStatus, State};
use crate::state::{PokemonVolatileStatus, SideReference, Weather};

type ModifyAttackBeingUsed = fn(&State, &mut Choice, &Choice, &SideReference);
type ModifyAttackAgainst = fn(&State, &mut Choice, &Choice, &SideReference);
type AbilityBeforeMove = fn(&mut State, &Choice, &SideReference, &mut StateInstructions);
type AbilityAfterDamageHit = fn(&mut State, &Choice, &SideReference, i16, &mut StateInstructions);
type AbilityOnSwitchOut = fn(&mut State, &SideReference, &mut StateInstructions);
type AbilityOnSwitchIn = fn(&mut State, &SideReference, &mut StateInstructions);
type AbilityEndOfTurn = fn(&mut State, &SideReference, &mut StateInstructions);

#[derive(PartialEq, Debug)]
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

pub struct AbilitiesStruct {
    ripen: Ability,
    tangledfeet: Ability,
    dragonsmaw: Ability,
    clearbody: Ability,
    galvanize: Ability,
    vitalspirit: Ability,
    aerilate: Ability,
    defiant: Ability,
    cutecharm: Ability,
    neuroforce: Ability,
    soundproof: Ability,
    rkssystem: Ability,
    poisonpoint: Ability,
    stakeout: Ability,
    unnerve: Ability,
    rockhead: Ability,
    aurabreak: Ability,
    mimicry: Ability,
    bulletproof: Ability,
    powerofalchemy: Ability,
    technician: Ability,
    multiscale: Ability,
    arenatrap: Ability,
    battlebond: Ability,
    disguise: Ability,
    earlybird: Ability,
    lightningrod: Ability,
    magician: Ability,
    refrigerate: Ability,
    friendguard: Ability,
    noability: Ability,
    gulpmissile: Ability,
    powerconstruct: Ability,
    forecast: Ability,
    prankster: Ability,
    protean: Ability,
    asoneglastrier: Ability,
    shadowtag: Ability,
    skilllink: Ability,
    intrepidsword: Ability,
    soulheart: Ability,
    swiftswim: Ability,
    eartheater: Ability,
    superluck: Ability,
    supremeoverlord: Ability,
    insomnia: Ability,
    dancer: Ability,
    steamengine: Ability,
    angerpoint: Ability,
    contrary: Ability,
    magmaarmor: Ability,
    hungerswitch: Ability,
    receiver: Ability,
    zenmode: Ability,
    emergencyexit: Ability,
    illusion: Ability,
    weakarmor: Ability,
    drought: Ability,
    innardsout: Ability,
    shieldsdown: Ability,
    adaptability: Ability,
    corrosion: Ability,
    longreach: Ability,
    purepower: Ability,
    tintedlens: Ability,
    queenlymajesty: Ability,
    desolateland: Ability,
    moxie: Ability,
    sapsipper: Ability,
    slushrush: Ability,
    bigpecks: Ability,
    stall: Ability,
    whitesmoke: Ability,
    flareboost: Ability,
    shadowshield: Ability,
    liquidvoice: Ability,
    mistysurge: Ability,
    multitype: Ability,
    noguard: Ability,
    torrent: Ability,
    deltastream: Ability,
    klutz: Ability,
    libero: Ability,
    serenegrace: Ability,
    cursedbody: Ability,
    unaware: Ability,
    lightmetal: Ability,
    marvelscale: Ability,
    telepathy: Ability,
    quickdraw: Ability,
    hypercutter: Ability,
    symbiosis: Ability,
    plus: Ability,
    mirrorarmor: Ability,
    pastelveil: Ability,
    toughclaws: Ability,
    effectspore: Ability,
    mummy: Ability,
    baddreams: Ability,
    magicguard: Ability,
    sandstream: Ability,
    powerspot: Ability,
    flamebody: Ability,
    reckless: Ability,
    pressure: Ability,
    gooey: Ability,
    immunity: Ability,
    leafguard: Ability,
    hugepower: Ability,
    solarpower: Ability,
    schooling: Ability,
    motordrive: Ability,
    anticipation: Ability,
    merciless: Ability,
    trace: Ability,
    naturalcure: Ability,
    harvest: Ability,
    suctioncups: Ability,
    iceface: Ability,
    roughskin: Ability,
    wonderguard: Ability,
    waterveil: Ability,
    fairyaura: Ability,
    sandspit: Ability,
    intimidate: Ability,
    dauntlessshield: Ability,
    aromaveil: Ability,
    airlock: Ability,
    normalize: Ability,
    darkaura: Ability,
    victorystar: Ability,
    grassysurge: Ability,
    sturdy: Ability,
    pickpocket: Ability,
    electricsurge: Ability,
    runaway: Ability,
    oblivious: Ability,
    surgesurfer: Ability,
    levitate: Ability,
    asonespectrier: Ability,
    pickup: Ability,
    icebody: Ability,
    curiousmedicine: Ability,
    flowerveil: Ability,
    _static: Ability,
    wonderskin: Ability,
    overgrow: Ability,
    propellertail: Ability,
    thickfat: Ability,
    gluttony: Ability,
    keeneye: Ability,
    mountaineer: Ability,
    flashfire: Ability,
    compoundeyes: Ability,
    steelworker: Ability,
    comatose: Ability,
    ballfetch: Ability,
    dazzling: Ability,
    download: Ability,
    transistor: Ability,
    moldbreaker: Ability,
    liquidooze: Ability,
    poisonheal: Ability,
    prismarmor: Ability,
    sniper: Ability,
    stench: Ability,
    competitive: Ability,
    swarm: Ability,
    stalwart: Ability,
    illuminate: Ability,
    turboblaze: Ability,
    gorillatactics: Ability,
    speedboost: Ability,
    heatproof: Ability,
    snowcloak: Ability,
    teravolt: Ability,
    chillingneigh: Ability,
    shielddust: Ability,
    rivalry: Ability,
    primordialsea: Ability,
    screencleaner: Ability,
    magnetpull: Ability,
    honeygather: Ability,
    cottondown: Ability,
    grasspelt: Ability,
    battlearmor: Ability,
    beastboost: Ability,
    berserk: Ability,
    minus: Ability,
    raindish: Ability,
    synchronize: Ability,
    filter: Ability,
    truant: Ability,
    furcoat: Ability,
    fullmetalbody: Ability,
    regenerator: Ability,
    forewarn: Ability,
    ironbarbs: Ability,
    stamina: Ability,
    sandrush: Ability,
    colorchange: Ability,
    blaze: Ability,
    analytic: Ability,
    tanglinghair: Ability,
    cloudnine: Ability,
    steelyspirit: Ability,
    quickfeet: Ability,
    magicbounce: Ability,
    megalauncher: Ability,
    heavymetal: Ability,
    stormdrain: Ability,
    pixilate: Ability,
    watercompaction: Ability,
    justified: Ability,
    slowstart: Ability,
    snowwarning: Ability,
    flowergift: Ability,
    shedskin: Ability,
    wimpout: Ability,
    icescales: Ability,
    infiltrator: Ability,
    limber: Ability,
    psychicsurge: Ability,
    defeatist: Ability,
    waterabsorb: Ability,
    imposter: Ability,
    dryskin: Ability,
    fluffy: Ability,
    unburden: Ability,
    cheekpouch: Ability,
    stancechange: Ability,
    moody: Ability,
    rockypayload: Ability,
    punkrock: Ability,
    sandveil: Ability,
    parentalbond: Ability,
    strongjaw: Ability,
    battery: Ability,
    healer: Ability,
    steadfast: Ability,
    damp: Ability,
    perishbody: Ability,
    triage: Ability,
    sheerforce: Ability,
    owntempo: Ability,
    frisk: Ability,
    voltabsorb: Ability,
    galewings: Ability,
    aftermath: Ability,
    stickyhold: Ability,
    grimneigh: Ability,
    ironfist: Ability,
    rebound: Ability,
    unseenfist: Ability,
    solidrock: Ability,
    hustle: Ability,
    hydration: Ability,
    scrappy: Ability,
    overcoat: Ability,
    neutralizinggas: Ability,
    sweetveil: Ability,
    drizzle: Ability,
    innerfocus: Ability,
    poisontouch: Ability,
    wanderingspirit: Ability,
    guts: Ability,
    shellarmor: Ability,
    rattled: Ability,
    waterbubble: Ability,
    sandforce: Ability,
    toxicboost: Ability,
    persistent: Ability,
    chlorophyll: Ability,
    simple: Ability,
    none: Ability,
    purifyingsalt: Ability,
}

lazy_static! {
    static ref ALL_ABILITIES: AbilitiesStruct = AbilitiesStruct {
        none: Ability {
            id: "none".to_string(),
            index: 0,
            ..Default::default()
        },
        ripen: Ability {
            id: "ripen".to_string(),
            index: 0,
            ..Default::default()
        },
        tangledfeet: Ability {
            id: "tangledfeet".to_string(),
            index: 1,
            ..Default::default()
        },
        dragonsmaw: Ability {
            id: "dragonsmaw".to_string(),
            index: 2,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Dragon {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        clearbody: Ability {
            id: "clearbody".to_string(),
            index: 3,
            ..Default::default()
        },
        galvanize: Ability {
            id: "galvanize".to_string(),
            index: 4,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Normal {
                        attacking_choice.move_type = PokemonType::Electric;
                        attacking_choice.base_power *= 1.2;
                    }
                },
            ),
            ..Default::default()
        },
        vitalspirit: Ability {
            id: "vitalspirit".to_string(),
            index: 5,
            ..Default::default()
        },
        aerilate: Ability {
            id: "aerilate".to_string(),
            index: 6,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Normal {
                        attacking_choice.move_type = PokemonType::Flying;
                        attacking_choice.base_power *= 1.2;
                    }
                },
            ),
            ..Default::default()
        },
        defiant: Ability {
            id: "defiant".to_string(),
            index: 7,
            ..Default::default()
        },
        cutecharm: Ability {
            id: "cutecharm".to_string(),
            index: 8,
            ..Default::default()
        },
        neuroforce: Ability {
            id: "neuroforce".to_string(),
            index: 9,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if type_effectiveness_modifier(
                        &attacking_choice.move_type,
                        &state
                            .get_side_immutable(&attacking_side.get_other_side())
                            .get_active_immutable()
                            .types,
                    ) > 1.0
                    {
                        attacking_choice.base_power *= 1.25;
                    }
                },
            ),
            ..Default::default()
        },
        soundproof: Ability {
            id: "soundproof".to_string(),
            index: 10,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.sound {
                        attacker_choice.remove_all_effects();
                        attacker_choice.accuracy = 0.0;
                    }
                },
            ),
            ..Default::default()
        },
        rkssystem: Ability {
            id: "rkssystem".to_string(),
            index: 11,
            ..Default::default()
        },
        poisonpoint: Ability {
            id: "poisonpoint".to_string(),
            index: 12,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.contact {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 33.0,
                                target: MoveTarget::User,
                                effect: Effect::Status(PokemonStatus::Poison),
                            }
                        )
                    }
                },
            ),
            ..Default::default()
        },
        stakeout: Ability {
            id: "stakeout".to_string(),
            index: 13,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if defender_choice.category == MoveCategory::Switch {
                        attacking_choice.base_power *= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        unnerve: Ability {
            id: "unnerve".to_string(),
            index: 14,
            ..Default::default()
        },
        rockhead: Ability {
            id: "rockhead".to_string(),
            index: 15,
            ..Default::default()
        },
        aurabreak: Ability {
            id: "aurabreak".to_string(),
            index: 16,
            ..Default::default()
        },
        mimicry: Ability {
            id: "mimicry".to_string(),
            index: 17,
            ..Default::default()
        },
        bulletproof: Ability {
            id: "bulletproof".to_string(),
            index: 18,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.bullet {
                        attacker_choice.accuracy = 0.0;
                    }
                },
            ),
            ..Default::default()
        },
        powerofalchemy: Ability {
            id: "powerofalchemy".to_string(),
            index: 19,
            ..Default::default()
        },
        technician: Ability {
            id: "technician".to_string(),
            index: 20,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.base_power <= 60.0 {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        multiscale: Ability {
            id: "multiscale".to_string(),
            index: 21,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    let target_pkmn = state.get_side_immutable(&attacking_side.get_other_side()).get_active_immutable();
                    if target_pkmn.hp == target_pkmn.maxhp {
                        attacker_choice.base_power /= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        arenatrap: Ability {
            id: "arenatrap".to_string(),
            index: 22,
            ..Default::default()
        },
        battlebond: Ability {
            id: "battlebond".to_string(),
            index: 23,
            ..Default::default()
        },
        disguise: Ability {
            id: "disguise".to_string(),
            index: 24,
            ..Default::default()
        },
        earlybird: Ability {
            id: "earlybird".to_string(),
            index: 25,
            ..Default::default()
        },
        lightningrod: Ability {
            id: "lightningrod".to_string(),
            index: 26,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
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
                },
            ),
            ..Default::default()
        },
        magician: Ability {
            id: "magician".to_string(),
            index: 27,
            ..Default::default()
        },
        refrigerate: Ability {
            id: "refrigerate".to_string(),
            index: 28,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Normal {
                        attacking_choice.move_type = PokemonType::Ice;
                        attacking_choice.base_power *= 1.2;
                    }
                },
            ),
            ..Default::default()
        },
        friendguard: Ability {
            id: "friendguard".to_string(),
            index: 29,
            ..Default::default()
        },
        noability: Ability {
            id: "noability".to_string(),
            index: 30,
            ..Default::default()
        },
        gulpmissile: Ability {
            id: "gulpmissile".to_string(),
            index: 31,
            ..Default::default()
        },
        powerconstruct: Ability {
            id: "powerconstruct".to_string(),
            index: 32,
            ..Default::default()
        },
        forecast: Ability {
            id: "forecast".to_string(),
            index: 33,
            ..Default::default()
        },
        prankster: Ability {
            id: "prankster".to_string(),
            index: 34,
            ..Default::default()
        },
        protean: Ability {
            id: "protean".to_string(),
            index: 35,
            before_move: Some(|state: &mut State, choice: &Choice, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                let active_pkmn = state.get_side(side_ref).get_active();
                if !active_pkmn.has_type(&choice.move_type) {
                    let ins = Instruction::ChangeType(ChangeType {
                        side_ref: *side_ref,
                        new_types: (choice.move_type, PokemonType::Typeless),
                        old_types: active_pkmn.types,
                    });
                    active_pkmn.types = (choice.move_type, PokemonType::Typeless);
                    incoming_instructions.instruction_list.push(ins);
                }
            }),
            ..Default::default()
        },
        asoneglastrier: Ability {
            id: "asoneglastrier".to_string(),
            index: 36,
            ..Default::default()
        },
        shadowtag: Ability {
            id: "shadowtag".to_string(),
            index: 37,
            ..Default::default()
        },
        skilllink: Ability {
            id: "skilllink".to_string(),
            index: 38,
            ..Default::default()
        },
        intrepidsword: Ability {
            id: "intrepidsword".to_string(),
            index: 39,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                // no need to check for boost at +6 because we are switching in
                state.get_side(side_ref).get_active().attack += 1;
                instructions.instruction_list.push(Instruction::Boost(BoostInstruction {
                    side_ref: *side_ref,
                    stat: PokemonBoostableStat::Attack,
                    amount: 1,
                }));
            }),
            ..Default::default()
        },
        soulheart: Ability {
            id: "soulheart".to_string(),
            index: 40,
            ..Default::default()
        },
        swiftswim: Ability {
            id: "swiftswim".to_string(),
            index: 41,
            ..Default::default()
        },
        eartheater: Ability {
            id: "eartheater".to_string(),
            index: 42,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Ground {
                        attacker_choice.base_power = 0.0;
                        attacker_choice.heal = Some(Heal {
                            target: MoveTarget::Opponent,
                            amount: 0.25
                        });
                        attacker_choice.category = MoveCategory::Status;
                    }
                },
            ),
            ..Default::default()
        },
        superluck: Ability {
            id: "superluck".to_string(),
            index: 43,
            ..Default::default()
        },
        supremeoverlord: Ability {
            id: "supremeoverlord".to_string(),
            index: 44,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let mut boost_amount = 1.0;
                    let side = state.get_side_immutable(attacking_side);
                    boost_amount += 0.1 * side.num_alive_pkmn() as f32;
                    attacking_choice.base_power *= boost_amount;
                },
            ),
            ..Default::default()
        },
        insomnia: Ability {
            id: "insomnia".to_string(),
            index: 45,
            ..Default::default()
        },
        dancer: Ability {
            id: "dancer".to_string(),
            index: 46,
            ..Default::default()
        },
        steamengine: Ability {
            id: "steamengine".to_string(),
            index: 47,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Water || attacker_choice.move_type == PokemonType::Fire {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 100.0,
                                target: MoveTarget::Opponent,
                                effect: Effect::Boost(
                                    StatBoosts {
                                        attack: 0,
                                        defense: 0,
                                        special_attack: 0,
                                        special_defense: 0,
                                        speed: 6,
                                        accuracy: 0,
                                    }
                                ),
                            }
                        );
                    }
                },
            ),
            ..Default::default()
        },
        angerpoint: Ability {
            id: "angerpoint".to_string(),
            index: 48,
            ..Default::default()
        },
        contrary: Ability {
            id: "contrary".to_string(),
            index: 49,
            ..Default::default()
        },
        magmaarmor: Ability {
            id: "magmaarmor".to_string(),
            index: 50,
            ..Default::default()
        },
        hungerswitch: Ability {
            id: "hungerswitch".to_string(),
            index: 51,
            ..Default::default()
        },
        receiver: Ability {
            id: "receiver".to_string(),
            index: 52,
            ..Default::default()
        },
        zenmode: Ability {
            id: "zenmode".to_string(),
            index: 53,
            ..Default::default()
        },
        emergencyexit: Ability {
            id: "emergencyexit".to_string(),
            index: 54,
            ..Default::default()
        },
        illusion: Ability {
            id: "illusion".to_string(),
            index: 55,
            ..Default::default()
        },
        weakarmor: Ability {
            id: "weakarmor".to_string(),
            index: 56,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.category == MoveCategory::Physical {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 100.0,
                                target: MoveTarget::Opponent,
                                effect: Effect::Boost(
                                    StatBoosts {
                                        attack: 0,
                                        defense: -1,
                                        special_attack: 0,
                                        special_defense: 0,
                                        speed: 2,
                                        accuracy: 0,
                                    }
                                ),
                            }

                        );
                    }
                },
            ),
            ..Default::default()
        },
        drought: Ability {
            id: "drought".to_string(),
            index: 57,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.weather.weather_type != Weather::Sun {
                    instructions.instruction_list.push(
                        Instruction::ChangeWeather(ChangeWeather {
                            new_weather: Weather::Sun,
                            new_weather_turns_remaining: 5,
                            previous_weather: state.weather.weather_type,
                            previous_weather_turns_remaining: state.weather.turns_remaining,
                        }));
                        state.weather.weather_type = Weather::Sun;
                        state.weather.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        innardsout: Ability {
            id: "innardsout".to_string(),
            index: 58,
            ..Default::default()
        },
        shieldsdown: Ability {
            id: "shieldsdown".to_string(),
            index: 59,
            ..Default::default()
        },
        adaptability: Ability {
            id: "adaptability".to_string(),
            index: 60,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if state
                        .get_side_immutable(attacking_side)
                        .get_active_immutable()
                        .has_type(&attacking_choice.move_type)
                    {
                        attacking_choice.base_power *= 4.0 / 3.0;
                    }
                },
            ),
            ..Default::default()
        },
        corrosion: Ability {
            id: "corrosion".to_string(),
            index: 61,
            ..Default::default()
        },
        longreach: Ability {
            id: "longreach".to_string(),
            index: 62,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    attacking_choice.flags.contact = false;
                },
            ),
            ..Default::default()
        },
        purepower: Ability {
            id: "purepower".to_string(),
            index: 63,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.category == MoveCategory::Physical {
                        attacking_choice.base_power *= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        tintedlens: Ability {
            id: "tintedlens".to_string(),
            index: 64,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if type_effectiveness_modifier(
                        &attacking_choice.move_type,
                        &state
                            .get_side_immutable(&attacking_side.get_other_side())
                            .get_active_immutable()
                            .types,
                    ) < 1.0
                    {
                        attacking_choice.base_power *= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        queenlymajesty: Ability {
            id: "queenlymajesty".to_string(),
            index: 65,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.priority > 0 {
                        attacker_choice.remove_all_effects();
                        attacker_choice.accuracy = 0.0;
                    }
                },
            ),
            ..Default::default()
        },
        desolateland: Ability {
            id: "desolateland".to_string(),
            index: 66,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.weather.weather_type != Weather::HarshSun {
                    instructions.instruction_list.push(
                        Instruction::ChangeWeather(ChangeWeather {
                            new_weather: Weather::HarshSun,
                            new_weather_turns_remaining: 5,
                            previous_weather: state.weather.weather_type,
                            previous_weather_turns_remaining: state.weather.turns_remaining,
                        }));
                        state.weather.weather_type = Weather::HarshSun;
                        state.weather.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        moxie: Ability {
            id: "moxie".to_string(),
            index: 67,
            ..Default::default()
        },
        sapsipper: Ability {
            id: "sapsipper".to_string(),
            index: 68,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
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
                },
            ),
            ..Default::default()
        },
        slushrush: Ability {
            id: "slushrush".to_string(),
            index: 69,
            ..Default::default()
        },
        bigpecks: Ability {
            id: "bigpecks".to_string(),
            index: 70,
            ..Default::default()
        },
        stall: Ability {
            id: "stall".to_string(),
            index: 71,
            ..Default::default()
        },
        whitesmoke: Ability {
            id: "whitesmoke".to_string(),
            index: 72,
            ..Default::default()
        },
        flareboost: Ability {
            id: "flareboost".to_string(),
            index: 73,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if state.get_side_immutable(attacking_side).get_active_immutable().status == PokemonStatus::Burn {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        shadowshield: Ability {
            id: "shadowshield".to_string(),
            index: 74,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    let target_pkmn = state.get_side_immutable(&attacking_side.get_other_side()).get_active_immutable();
                    if target_pkmn.hp == target_pkmn.maxhp {
                        attacker_choice.base_power /= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        liquidvoice: Ability {
            id: "liquidvoice".to_string(),
            index: 75,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.flags.sound {
                        attacking_choice.move_type = PokemonType::Water;
                    }
                },
            ),
            ..Default::default()
        },
        mistysurge: Ability {
            id: "mistysurge".to_string(),
            index: 76,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.terrain.terrain_type != Terrain::MistyTerrain {
                    instructions.instruction_list.push(
                        Instruction::ChangeTerrain(ChangeTerrain {
                            new_terrain: Terrain::MistyTerrain,
                            new_terrain_turns_remaining: 5,
                            previous_terrain: state.terrain.terrain_type,
                            previous_terrain_turns_remaining: state.terrain.turns_remaining,
                        }));
                        state.terrain.terrain_type = Terrain::MistyTerrain;
                        state.terrain.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        multitype: Ability {
            id: "multitype".to_string(),
            index: 77,
            ..Default::default()
        },
        noguard: Ability {
            id: "noguard".to_string(),
            index: 78,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    attacking_choice.accuracy = 100.0
                },
            ),
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    attacker_choice.accuracy = 100.0
                },
            ),
            ..Default::default()
        },
        torrent: Ability {
            id: "torrent".to_string(),
            index: 79,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                    if attacking_choice.move_type == PokemonType::Water && attacking_pokemon.hp < attacking_pokemon.maxhp / 3 {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            ..Default::default()
        },
        deltastream: Ability {
            id: "deltastream".to_string(),
            index: 80,
            ..Default::default()
        },
        klutz: Ability {
            id: "klutz".to_string(),
            index: 81,
            ..Default::default()
        },
        libero: Ability {
            id: "libero".to_string(),
            index: 82,
            before_move: Some(|state: &mut State, choice: &Choice, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                let active_pkmn = state.get_side(side_ref).get_active();
                if !active_pkmn.has_type(&choice.move_type) {
                    let ins = Instruction::ChangeType(ChangeType {
                        side_ref: *side_ref,
                        new_types: (choice.move_type, PokemonType::Typeless),
                        old_types: active_pkmn.types,
                    });
                    active_pkmn.types = (choice.move_type, PokemonType::Typeless);
                    incoming_instructions.instruction_list.push(ins);
                }
            }),
            ..Default::default()
        },
        serenegrace: Ability {
            id: "serenegrace".to_string(),
            index: 83,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if let Some(secondaries) = &mut attacking_choice.secondaries {
                        for secondary in secondaries.iter_mut() {
                            secondary.chance *= 2.0;
                        }
                    }
                },
            ),
            ..Default::default()
        },
        cursedbody: Ability {
            id: "cursedbody".to_string(),
            index: 84,
            ..Default::default()
        },
        unaware: Ability {
            id: "unaware".to_string(),
            index: 85,
            ..Default::default()
        },
        lightmetal: Ability {
            id: "lightmetal".to_string(),
            index: 86,
            ..Default::default()
        },
        marvelscale: Ability {
            id: "marvelscale".to_string(),
            index: 87,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if state.get_side_immutable(&attacking_side.get_other_side()).get_active_immutable().status != PokemonStatus::None && attacker_choice.category == MoveCategory::Physical {
                        attacker_choice.base_power /= 1.5;
                    }
                },
            ),
            ..Default::default()
            },
        telepathy: Ability {
            id: "telepathy".to_string(),
            index: 88,
            ..Default::default()
        },
        quickdraw: Ability {
            id: "quickdraw".to_string(),
            index: 89,
            ..Default::default()
        },
        hypercutter: Ability {
            id: "hypercutter".to_string(),
            index: 90,
            ..Default::default()
        },
        symbiosis: Ability {
            id: "symbiosis".to_string(),
            index: 91,
            ..Default::default()
        },
        plus: Ability {
            id: "plus".to_string(),
            index: 92,
            ..Default::default()
        },
        mirrorarmor: Ability {
            id: "mirrorarmor".to_string(),
            index: 93,
            ..Default::default()
        },
        pastelveil: Ability {
            id: "pastelveil".to_string(),
            index: 94,
            ..Default::default()
        },
        toughclaws: Ability {
            id: "toughclaws".to_string(),
            index: 95,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.flags.contact {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            ..Default::default()
        },
        effectspore: Ability {
            id: "effectspore".to_string(),
            index: 96,
            modify_attack_against: Some(
                |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                    if attacker_choice.flags.contact {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 9.0,
                                target: MoveTarget::User,
                                effect: Effect::Status(PokemonStatus::Poison),
                            }
                        );
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 10.0,
                                target: MoveTarget::User,
                                effect: Effect::Status(PokemonStatus::Paralyze),
                            }
                        );
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 11.0,
                                target: MoveTarget::User,
                                effect: Effect::Status(PokemonStatus::Sleep),
                            }
                        );
                    }
                },
            ),
            ..Default::default()
        },
        mummy: Ability {
            id: "mummy".to_string(),
            index: 97,
            ..Default::default()
        },
        baddreams: Ability {
            id: "baddreams".to_string(),
            index: 98,
            end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                let defender = state.get_side(&side_ref.get_other_side()).get_active();
                if defender.status == PokemonStatus::Sleep {
                    let damage_dealt = cmp::min(defender.maxhp / 8, defender.hp);
                    incoming_instructions.instruction_list.push(Instruction::Damage(DamageInstruction {
                        side_ref: side_ref.get_other_side(),
                        damage_amount: damage_dealt,
                    }));
                    defender.hp -= damage_dealt;
                }

            }),
            ..Default::default()
        },
        magicguard: Ability {
            id: "magicguard".to_string(),
            index: 99,
            ..Default::default()
        },
        sandstream: Ability {
            id: "sandstream".to_string(),
            index: 100,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.weather.weather_type != Weather::Sand {
                    instructions.instruction_list.push(
                        Instruction::ChangeWeather(ChangeWeather {
                            new_weather: Weather::Sand,
                            new_weather_turns_remaining: 5,
                            previous_weather: state.weather.weather_type,
                            previous_weather_turns_remaining: state.weather.turns_remaining,
                        }));
                        state.weather.weather_type = Weather::Sand;
                        state.weather.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        powerspot: Ability {
            id: "powerspot".to_string(),
            index: 101,
            ..Default::default()
        },
        flamebody: Ability {
            id: "flamebody".to_string(),
            index: 102,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if state.move_makes_contact(&attacker_choice, attacking_side) {
                        let burn_secondary = Secondary {
                            chance: 30.0,
                            target: MoveTarget::User,
                            effect: Effect::Status(PokemonStatus::Burn),
                        };

                        if attacker_choice.secondaries.is_none() {
                            attacker_choice.secondaries = Some(vec![burn_secondary]);
                        } else {
                            attacker_choice
                                .secondaries
                                .as_mut()
                                .unwrap()
                                .push(burn_secondary);
                        }
                    }
                },
            ),
            ..Default::default()
        },
        reckless: Ability {
            id: "reckless".to_string(),
            index: 103,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.crash.is_some() {
                        attacking_choice.base_power *= 1.2;
                    }
                },
            ),
            ..Default::default()
        },
        pressure: Ability {
            id: "pressure".to_string(),
            index: 104,
            ..Default::default()
        },
        gooey: Ability {
            id: "gooey".to_string(),
            index: 105,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.contact {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 100.0,
                                target: MoveTarget::User,
                                effect: Effect::Boost(
                                    StatBoosts {
                                        attack: 0,
                                        defense: 0,
                                        special_attack: 0,
                                        special_defense: 0,
                                        speed: -1,
                                        accuracy: 0,
                                    }
                                ),
                            }
                        )
                    }
                },
            ),
            ..Default::default()
        },
        immunity: Ability {
            id: "immunity".to_string(),
            index: 106,
            ..Default::default()
        },
        leafguard: Ability {
            id: "leafguard".to_string(),
            index: 107,
            ..Default::default()
        },
        hugepower: Ability {
            id: "hugepower".to_string(),
            index: 108,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.category == MoveCategory::Physical {
                        attacking_choice.base_power *= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        solarpower: Ability {
            id: "solarpower".to_string(),
            index: 109,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if state.weather_is_active(&Weather::Sun) {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                if state.weather.weather_type == Weather::HarshSun || state.weather.weather_type == Weather::Sun {
                    let side = state.get_side(side_ref);
                    let active = side.get_active();
                    let damage_dealt = cmp::min(active.maxhp / 8, active.maxhp - active.hp);
                    if damage_dealt > 0 {
                        incoming_instructions.instruction_list.push(Instruction::Damage(DamageInstruction {
                        side_ref: *side_ref,
                        damage_amount: damage_dealt,
                    }));
                    active.hp -= damage_dealt;
                    }
                }
            }),
            ..Default::default()
        },
        schooling: Ability {
            id: "schooling".to_string(),
            index: 110,
            ..Default::default()
        },
        motordrive: Ability {
            id: "motordrive".to_string(),
            index: 111,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
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
                },
            ),
            ..Default::default()
        },
        anticipation: Ability {
            id: "anticipation".to_string(),
            index: 112,
            ..Default::default()
        },
        merciless: Ability {
            id: "merciless".to_string(),
            index: 113,
            ..Default::default()
        },
        trace: Ability {
            id: "trace".to_string(),
            index: 114,
            ..Default::default()
        },
        naturalcure: Ability {
            id: "naturalcure".to_string(),
            index: 115,
            on_switch_out: Some(|state: &mut State, side_reference: &SideReference, instructions: &mut StateInstructions| {
                let side = state.get_side(side_reference);
                let active_index = side.active_index;
                let active = side.get_active();
                if active.status != PokemonStatus::None {
                    instructions.instruction_list.push(
                        Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: *side_reference,
                        pokemon_index: active_index,
                        old_status: active.status,
                        new_status: PokemonStatus::None,
                        })
                    );
                    active.status = PokemonStatus::None;
                }
            }),
            ..Default::default()
        },
        harvest: Ability {
            id: "harvest".to_string(),
            index: 116,
            ..Default::default()
        },
        suctioncups: Ability {
            id: "suctioncups".to_string(),
            index: 117,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    attacker_choice.flags.drag = false;
                },
            ),
            ..Default::default()
        },
        iceface: Ability {
            id: "iceface".to_string(),
            index: 118,
            ..Default::default()
        },
        roughskin: Ability {
            id: "roughskin".to_string(),
            index: 119,
            ..Default::default()
        },
        wonderguard: Ability {
            id: "wonderguard".to_string(),
            index: 120,
            modify_attack_against: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.category != MoveCategory::Status && type_effectiveness_modifier(
                        &attacking_choice.move_type,
                        &state
                            .get_side_immutable(&attacking_side.get_other_side())
                            .get_active_immutable()
                            .types,
                    ) <= 1.0
                    {
                        attacking_choice.remove_all_effects();
                        attacking_choice.base_power = 0.0;
                    }
                },
            ),
            ..Default::default()
        },
        waterveil: Ability {
            id: "waterveil".to_string(),
            index: 121,
            ..Default::default()
        },
        fairyaura: Ability {
            id: "fairyaura".to_string(),
            index: 122,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Fairy {
                        attacking_choice.base_power *= 1.33;
                    }
                },
            ),
            modify_attack_against: Some(
                |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                    if attacker_choice.move_type == PokemonType::Fairy {
                        attacker_choice.base_power *= 1.33;
                    }
                },
            ),
            ..Default::default()
        },
        sandspit: Ability {
            id: "sandspit".to_string(),
            index: 123,
            ..Default::default()
        },
        intimidate: Ability {
            id: "intimidate".to_string(),
            index: 124,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                let target_side_ref = side_ref.get_other_side();
                let target_pkmn = state.get_side_immutable(&target_side_ref).get_active_immutable();
                if let Some(boost_instruction) = get_boost_instruction(
                    target_pkmn,
                    &PokemonBoostableStat::Attack,
                    &-1,
                    side_ref,
                    &target_side_ref,
                ) {
                    match target_pkmn.ability {
                        Abilities::OWNTEMPO | Abilities::OBLIVIOUS | Abilities::INNERFOCUS | Abilities::SCRAPPY => {}
                        _ => {
                            state.apply_one_instruction(&boost_instruction);
                            instructions.instruction_list.push(boost_instruction);
                        }
                    }
                }
            }),
            ..Default::default()
        },
        dauntlessshield: Ability {
            id: "dauntlessshield".to_string(),
            index: 125,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                // no need to check for boost at +6 because we are switching in
                state.get_side(side_ref).get_active().defense_boost += 1;
                instructions.instruction_list.push(Instruction::Boost(BoostInstruction {
                    side_ref: *side_ref,
                    stat: PokemonBoostableStat::Defense,
                    amount: 1,
                }));
            }),
            ..Default::default()
        },
        aromaveil: Ability {
            id: "aromaveil".to_string(),
            index: 126,
            ..Default::default()
        },
        airlock: Ability {
            id: "airlock".to_string(),
            index: 127,
            ..Default::default()
        },
        normalize: Ability {
            id: "normalize".to_string(),
            index: 128,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    attacking_choice.move_type = PokemonType::Normal;
                },
            ),
            ..Default::default()
        },
        darkaura: Ability {
            id: "darkaura".to_string(),
            index: 129,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Dark {
                        attacking_choice.base_power *= 1.33;
                    }
                },
            ),
            ..Default::default()
        },
        victorystar: Ability {
            id: "victorystar".to_string(),
            index: 130,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    attacking_choice.accuracy *= 1.1;
                },
            ),
            ..Default::default()
        },
        grassysurge: Ability {
            id: "grassysurge".to_string(),
            index: 131,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.terrain.terrain_type != Terrain::GrassyTerrain {
                    instructions.instruction_list.push(
                        Instruction::ChangeTerrain(ChangeTerrain {
                            new_terrain: Terrain::GrassyTerrain,
                            new_terrain_turns_remaining: 5,
                            previous_terrain: state.terrain.terrain_type,
                            previous_terrain_turns_remaining: state.terrain.turns_remaining,
                        }));
                        state.terrain.terrain_type = Terrain::GrassyTerrain;
                        state.terrain.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        sturdy: Ability {
            id: "sturdy".to_string(),
            index: 132,
            ..Default::default()
        },
        pickpocket: Ability {
            id: "pickpocket".to_string(),
            index: 133,
            ..Default::default()
        },
        electricsurge: Ability {
            id: "electricsurge".to_string(),
            index: 134,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.terrain.terrain_type != Terrain::ElectricTerrain {
                    instructions.instruction_list.push(
                        Instruction::ChangeTerrain(ChangeTerrain {
                            new_terrain: Terrain::ElectricTerrain,
                            new_terrain_turns_remaining: 5,
                            previous_terrain: state.terrain.terrain_type,
                            previous_terrain_turns_remaining: state.terrain.turns_remaining,
                        }));
                        state.terrain.terrain_type = Terrain::ElectricTerrain;
                        state.terrain.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        runaway: Ability {
            id: "runaway".to_string(),
            index: 135,
            ..Default::default()
        },
        oblivious: Ability {
            id: "oblivious".to_string(),
            index: 136,
            ..Default::default()
        },
        surgesurfer: Ability {
            id: "surgesurfer".to_string(),
            index: 137,
            ..Default::default()
        },
        levitate: Ability {
            id: "levitate".to_string(),
            index: 138,
            modify_attack_against: Some(
                |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                    if attacker_choice.move_type == PokemonType::Ground
                        && attacker_choice.target == MoveTarget::Opponent
                        && attacker_choice.move_id != "thousandarrows"
                    {
                        attacker_choice.base_power = 0.0;
                    }
                },
            ),
            ..Default::default()
        },
        asonespectrier: Ability {
            id: "asonespectrier".to_string(),
            index: 139,
            ..Default::default()
        },
        pickup: Ability {
            id: "pickup".to_string(),
            index: 140,
            ..Default::default()
        },
        icebody: Ability {
            id: "icebody".to_string(),
            index: 141,
            end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                if state.weather.weather_type == Weather::Hail {
                    let side = state.get_side(side_ref);
                    let active = side.get_active();
                    let health_recovered = cmp::min(active.maxhp / 16, active.maxhp - active.hp);
                    if health_recovered > 0 {
                        incoming_instructions.instruction_list.push(Instruction::Heal(HealInstruction {
                        side_ref: *side_ref,
                        heal_amount: health_recovered,
                    }));
                    active.hp += health_recovered;
                    }
                }
            }),
            ..Default::default()
        },
        curiousmedicine: Ability {
            id: "curiousmedicine".to_string(),
            index: 142,
            ..Default::default()
        },
        flowerveil: Ability {
            id: "flowerveil".to_string(),
            index: 143,
            ..Default::default()
        },
        _static: Ability {
            id: "static".to_string(),
            index: 144,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if state.move_makes_contact(&attacker_choice, attacking_side) {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 30.0,
                                target: MoveTarget::User,
                                effect: Effect::Status(PokemonStatus::Paralyze),
                            }
                        )
                    }
                },
            ),
            ..Default::default()
        },
        wonderskin: Ability {
            id: "wonderskin".to_string(),
            index: 145,
            modify_attack_against: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.category == MoveCategory::Status && attacking_choice.accuracy > 50.0{
                        attacking_choice.accuracy = 50.0;
                    }
                },
            ),
            ..Default::default()
        },
        overgrow: Ability {
            id: "overgrow".to_string(),
            index: 146,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                    if attacking_choice.move_type == PokemonType::Grass && attacking_pokemon.hp < attacking_pokemon.maxhp / 3 {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            ..Default::default()
        },
        propellertail: Ability {
            id: "propellertail".to_string(),
            index: 147,
            ..Default::default()
        },
        thickfat: Ability {
            id: "thickfat".to_string(),
            index: 148,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Fire || attacker_choice.move_type == PokemonType::Ice {
                        attacker_choice.base_power /= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        gluttony: Ability {
            id: "gluttony".to_string(),
            index: 149,
            ..Default::default()
        },
        keeneye: Ability {
            id: "keeneye".to_string(),
            index: 150,
            ..Default::default()
        },
        mountaineer: Ability {
            id: "mountaineer".to_string(),
            index: 151,
            ..Default::default()
        },
        flashfire: Ability {
            id: "flashfire".to_string(),
            index: 152,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Fire {
                        attacker_choice.base_power = 0.0;
                        attacker_choice.volatile_status = Some(VolatileStatus {
                            target: MoveTarget::Opponent,
                            volatile_status: PokemonVolatileStatus::FlashFire,
                        });
                    }
                },
            ),
            ..Default::default()
        },
        compoundeyes: Ability {
            id: "compoundeyes".to_string(),
            index: 153,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    attacking_choice.accuracy *= 1.3;
                },
            ),
            ..Default::default()
        },
        steelworker: Ability {
            id: "steelworker".to_string(),
            index: 154,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if defender_choice.move_type == PokemonType::Steel {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        comatose: Ability {
            id: "comatose".to_string(),
            index: 155,
            ..Default::default()
        },
        ballfetch: Ability {
            id: "ballfetch".to_string(),
            index: 156,
            ..Default::default()
        },
        dazzling: Ability {
            id: "dazzling".to_string(),
            index: 157,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.priority > 0 {
                        attacker_choice.accuracy = 0.0;
                    }
                },
            ),
            ..Default::default()
        },
        download: Ability {
            id: "download".to_string(),
            index: 158,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                let opposing_pokemon = state.get_side_immutable(&side_ref.get_other_side()).get_active_immutable();
                if opposing_pokemon.calculate_boosted_stat(PokemonBoostableStat::Defense) < opposing_pokemon.calculate_boosted_stat(PokemonBoostableStat::SpecialDefense) {
                    instructions.instruction_list.push(Instruction::Boost(BoostInstruction {
                        side_ref: side_ref.clone(),
                        stat: PokemonBoostableStat::Attack,
                        amount: 1,
                    }));
                    state.get_side(side_ref).get_active().attack_boost += 1;
                } else {
                    instructions.instruction_list.push(Instruction::Boost(BoostInstruction {
                        side_ref: side_ref.clone(),
                        stat: PokemonBoostableStat::SpecialAttack,
                        amount: 1,
                    }));
                    state.get_side(side_ref).get_active().special_attack_boost += 1;
                }
            }),
            ..Default::default()
        },
        transistor: Ability {
            id: "transistor".to_string(),
            index: 159,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Electric {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        moldbreaker: Ability {
            id: "moldbreaker".to_string(),
            index: 160,
            ..Default::default()
        },
        liquidooze: Ability {
            id: "liquidooze".to_string(),
            index: 161,
            modify_attack_against: Some(
                |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                    if let Some(drain) = attacker_choice.drain {
                        attacker_choice.drain = Some(-1.0 * drain);
                    }
                },
            ),
            ..Default::default()
        },
        poisonheal: Ability {
            id: "poisonheal".to_string(),
            index: 162,
            end_of_turn: Some(|state: &mut State,
             side_ref: &SideReference,
             incoming_instructions: &mut StateInstructions| {
                let attacker = state.get_side(side_ref).get_active();
                if attacker.hp < attacker.maxhp
                    && (attacker.status == PokemonStatus::Poison
                        || attacker.status == PokemonStatus::Toxic)
                {
                    let heal_amount = cmp::min(attacker.maxhp / 8, attacker.maxhp - attacker.hp);
                    let ins = Instruction::Heal(HealInstruction {
                        side_ref: side_ref.clone(),
                        heal_amount: heal_amount,
                    });
                    attacker.hp += heal_amount;
                    incoming_instructions.instruction_list.push(ins);

                }
            }),
            ..Default::default()
        },
        prismarmor: Ability {
            id: "prismarmor".to_string(),
            index: 163,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if type_effectiveness_modifier(
                        &attacker_choice.move_type,
                        &state
                            .get_side_immutable(&attacking_side.get_other_side())
                            .get_active_immutable()
                            .types,
                    ) > 1.0
                    {
                        attacker_choice.base_power *= 0.75;
                    }
                },
            ),
            ..Default::default()
        },
        sniper: Ability {
            id: "sniper".to_string(),
            index: 164,
            ..Default::default()
        },
        stench: Ability {
            id: "stench".to_string(),
            index: 165,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let mut already_flinches = false;
                    if let Some(secondaries) = &mut attacking_choice.secondaries {
                        for secondary in secondaries.iter() {
                            if secondary.effect == Effect::VolatileStatus(PokemonVolatileStatus::Flinch) {
                                already_flinches = true;
                            }
                        }
                    }
                    if !already_flinches {
                        attacking_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 10.0,
                                target: MoveTarget::Opponent,
                                effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                            }
                        )
                    }
                },
            ),
            ..Default::default()
        },
        competitive: Ability {
            id: "competitive".to_string(),
            index: 166,
            ..Default::default()
        },
        swarm: Ability {
            id: "swarm".to_string(),
            index: 167,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                    if attacking_choice.move_type == PokemonType::Bug && attacking_pokemon.hp < attacking_pokemon.maxhp / 3 {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            ..Default::default()
        },
        stalwart: Ability {
            id: "stalwart".to_string(),
            index: 168,
            ..Default::default()
        },
        illuminate: Ability {
            id: "illuminate".to_string(),
            index: 169,
            ..Default::default()
        },
        turboblaze: Ability {
            id: "turboblaze".to_string(),
            index: 170,
            ..Default::default()
        },
        gorillatactics: Ability {
            id: "gorillatactics".to_string(),
            index: 171,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.category == MoveCategory::Physical {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        speedboost: Ability {
            id: "speedboost".to_string(),
            index: 172,
            end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                let attacker = state.get_side(side_ref).get_active();
                if attacker.speed_boost < 6 {
                    let ins = Instruction::Boost(BoostInstruction {
                        side_ref: side_ref.clone(),
                        stat: PokemonBoostableStat::Speed,
                        amount: 1,
                    });
                    attacker.speed_boost += 1;
                    incoming_instructions.instruction_list.push(ins);
                }
            }),
            ..Default::default()
        },
        heatproof: Ability {
            id: "heatproof".to_string(),
            index: 173,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Fire {
                        attacker_choice.base_power *= 0.5 ;
                    }
                },
            ),
            ..Default::default()
        },
        snowcloak: Ability {
            id: "snowcloak".to_string(),
            index: 174,
            ..Default::default()
        },
        teravolt: Ability {
            id: "teravolt".to_string(),
            index: 175,
            ..Default::default()
        },
        chillingneigh: Ability {
            id: "chillingneigh".to_string(),
            index: 176,
            ..Default::default()
        },
        shielddust: Ability {
            id: "shielddust".to_string(),
            index: 177,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if let Some(secondaries) = &mut attacker_choice.secondaries {
                        for secondary in secondaries.iter_mut() {
                            if secondary.target == MoveTarget::Opponent {
                                secondary.chance = 0.0;
                            }
                        }
                    }
                },
            ),
            ..Default::default()
        },
        rivalry: Ability {
            id: "rivalry".to_string(),
            index: 178,
            ..Default::default()
        },
        primordialsea: Ability {
            id: "primordialsea".to_string(),
            index: 179,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.weather.weather_type != Weather::HeavyRain {
                    instructions.instruction_list.push(
                        Instruction::ChangeWeather(ChangeWeather {
                            new_weather: Weather::HeavyRain,
                            new_weather_turns_remaining: 5,
                            previous_weather: state.weather.weather_type,
                            previous_weather_turns_remaining: state.weather.turns_remaining,
                        }));
                        state.weather.weather_type = Weather::HeavyRain;
                        state.weather.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        screencleaner: Ability {
            id: "screencleaner".to_string(),
            index: 180,
            ..Default::default()
        },
        magnetpull: Ability {
            id: "magnetpull".to_string(),
            index: 181,
            ..Default::default()
        },
        honeygather: Ability {
            id: "honeygather".to_string(),
            index: 182,
            ..Default::default()
        },
        cottondown: Ability {
            id: "cottondown".to_string(),
            index: 183,
            ..Default::default()
        },
        grasspelt: Ability {
            id: "grasspelt".to_string(),
            index: 184,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if state.terrain_is_active(&Terrain::GrassyTerrain) && attacker_choice.category == MoveCategory::Physical {
                        attacker_choice.base_power /= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        battlearmor: Ability {
            id: "battlearmor".to_string(),
            index: 185,
            ..Default::default()
        },
        beastboost: Ability {
            id: "beastboost".to_string(),
            index: 186,
            after_damage_hit: Some(|state, _, attacking_side, damage_dealt, instructions| {
                let (attacker_side, defender_side) =
                    state.get_both_sides(attacking_side);
                if damage_dealt > 0 && defender_side.get_active_immutable().hp == 0 {
                    let highest_stat = &attacker_side
                        .get_active_immutable()
                        .calculate_highest_stat();
                    if let Some(boost_instruction) = get_boost_instruction(
                        state.get_side_immutable(attacking_side).get_active_immutable(),
                        highest_stat,
                        &1,
                        attacking_side,
                        attacking_side,
                    ) {
                        state.apply_one_instruction(&boost_instruction);
                        instructions.instruction_list.push(boost_instruction);
                    }
                }
            }),
            ..Default::default()
        },
        berserk: Ability {
            id: "berserk".to_string(),
            index: 187,
            ..Default::default()
        },
        minus: Ability {
            id: "minus".to_string(),
            index: 188,
            ..Default::default()
        },
        raindish: Ability {
            id: "raindish".to_string(),
            index: 189,
            end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                if state.weather.weather_type == Weather::Rain || state.weather.weather_type == Weather::HeavyRain {
                    let side = state.get_side(side_ref);
                    let active = side.get_active();
                    let health_recovered = cmp::min(active.maxhp / 16, active.maxhp - active.hp);
                    if health_recovered > 0 {
                        incoming_instructions.instruction_list.push(Instruction::Heal(HealInstruction {
                        side_ref: *side_ref,
                        heal_amount: health_recovered,
                    }));
                    active.hp += health_recovered;
                    }
                }
            }),
            ..Default::default()
        },
        synchronize: Ability {
            id: "synchronize".to_string(),
            index: 190,
            ..Default::default()
        },
        filter: Ability {
            id: "filter".to_string(),
            index: 191,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if type_effectiveness_modifier(
                        &attacker_choice.move_type,
                        &state
                            .get_side_immutable(&attacking_side.get_other_side())
                            .get_active_immutable()
                            .types,
                    ) > 1.0
                    {
                        attacker_choice.base_power *= 0.75;
                    }
                },
            ),
            ..Default::default()
        },
        truant: Ability {
            id: "truant".to_string(),
            index: 192,
            ..Default::default()
        },
        furcoat: Ability {
            id: "furcoat".to_string(),
            index: 193,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.category == MoveCategory::Physical {
                        attacker_choice.base_power *= 0.5;
                    }
                },
            ),
            ..Default::default()
        },
        fullmetalbody: Ability {
            id: "fullmetalbody".to_string(),
            index: 194,
            ..Default::default()
        },
        regenerator: Ability {
            id: "regenerator".to_string(),
            index: 195,
            on_switch_out: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                let switching_out_pkmn =
                    state.get_side(side_ref).get_active();
                let hp_recovered = cmp::min(
                    switching_out_pkmn.maxhp / 3,
                    switching_out_pkmn.maxhp - switching_out_pkmn.hp,
                );

                if hp_recovered > 0 && switching_out_pkmn.hp > 0 {
                    instructions.instruction_list.push(Instruction::Heal(HealInstruction {
                        side_ref: *side_ref,
                        heal_amount: hp_recovered,
                    }));
                    switching_out_pkmn.hp += hp_recovered;
                }
            }),
            ..Default::default()
        },
        forewarn: Ability {
            id: "forewarn".to_string(),
            index: 196,
            ..Default::default()
        },
        ironbarbs: Ability {
            id: "ironbarbs".to_string(),
            index: 197,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.contact {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 100.0,
                                target: MoveTarget::User,
                                effect: Effect::Heal(-0.125),
                            }
                        );
                    }
                },
            ),
            ..Default::default()
        },
        stamina: Ability {
            id: "stamina".to_string(),
            index: 198,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.category != MoveCategory::Status {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 100.0,
                                target: MoveTarget::Opponent,
                                effect: Effect::Boost(
                                    StatBoosts {
                                        attack: 0,
                                        defense: 1,
                                        special_attack: 0,
                                        special_defense: 0,
                                        speed: 0,
                                        accuracy: 0,
                                    }
                                ),
                            }
                        );
                    }
                },
            ),
            ..Default::default()
        },
        sandrush: Ability {
            id: "sandrush".to_string(),
            index: 199,
            ..Default::default()
        },
        colorchange: Ability {
            id: "colorchange".to_string(),
            index: 200,
            ..Default::default()
        },
        blaze: Ability {
            id: "blaze".to_string(),
            index: 201,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                    if attacking_choice.move_type == PokemonType::Fire && attacking_pokemon.hp < attacking_pokemon.maxhp / 3 {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            ..Default::default()
        },
        analytic: Ability {
            id: "analytic".to_string(),
            index: 202,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if !attacking_choice.first_move {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            ..Default::default()
        },
        tanglinghair: Ability {
            id: "tanglinghair".to_string(),
            index: 203,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.contact {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 100.0,
                                target: MoveTarget::User,
                                effect: Effect::Boost(
                                    StatBoosts {
                                        attack: 0,
                                        defense: 0,
                                        special_attack: 0,
                                        special_defense: 0,
                                        speed: -1,
                                        accuracy: 0,
                                    }
                                ),
                            }
                        )
                    }
                },
            ),
            ..Default::default()
        },
        cloudnine: Ability {
            id: "cloudnine".to_string(),
            index: 204,
            ..Default::default()
        },
        steelyspirit: Ability {
            id: "steelyspirit".to_string(),
            index: 205,
            ..Default::default()
        },
        quickfeet: Ability {
            id: "quickfeet".to_string(),
            index: 206,
            ..Default::default()
        },
        magicbounce: Ability {
            id: "magicbounce".to_string(),
            index: 207,
            modify_attack_against: Some(
                |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
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
                },
            ),
            ..Default::default()
        },
        megalauncher: Ability {
            id: "megalauncher".to_string(),
            index: 208,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.flags.pulse {
                        attacking_choice.base_power *= 1.5;
                    };
                },
            ),
            ..Default::default()
        },
        heavymetal: Ability {
            id: "heavymetal".to_string(),
            index: 209,
            ..Default::default()
        },
        stormdrain: Ability {
            id: "stormdrain".to_string(),
            index: 210,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
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
                },
            ),
            ..Default::default()
        },
        pixilate: Ability {
            id: "pixilate".to_string(),
            index: 211,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Normal {
                        attacking_choice.move_type = PokemonType::Fairy;
                        attacking_choice.base_power *= 1.2;
                    }
                },
            ),
            ..Default::default()
        },
        watercompaction: Ability {
            id: "watercompaction".to_string(),
            index: 212,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Water {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 100.0,
                                target: MoveTarget::Opponent,
                                effect: Effect::Boost(
                                    StatBoosts {
                                        attack: 0,
                                        defense: 2,
                                        special_attack: 0,
                                        special_defense: 0,
                                        speed: 0,
                                        accuracy: 0,
                                    }
                                ),
                            }

                        );
                    }
                },
            ),
            ..Default::default()
        },
        justified: Ability {
            id: "justified".to_string(),
            index: 213,
            modify_attack_against: Some(
                |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                    if attacker_choice.move_type == PokemonType::Dark {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
                                chance: 100.0,
                                target: MoveTarget::Opponent,
                                effect: Effect::Boost(
                                    StatBoosts {
                                        attack: 1,
                                        defense: 0,
                                        special_attack: 0,
                                        special_defense: 0,
                                        speed: 0,
                                        accuracy: 0,
                                    }
                                ),
                            }
                        )
                    }
                },
            ),
            ..Default::default()
        },
        slowstart: Ability {
            id: "slowstart".to_string(),
            index: 214,
            ..Default::default()
        },
        snowwarning: Ability {
            id: "snowwarning".to_string(),
            index: 215,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.weather.weather_type != Weather::Hail {
                    instructions.instruction_list.push(
                        Instruction::ChangeWeather(ChangeWeather {
                            new_weather: Weather::Hail,
                            new_weather_turns_remaining: 5,
                            previous_weather: state.weather.weather_type,
                            previous_weather_turns_remaining: state.weather.turns_remaining,
                        }));
                        state.weather.weather_type = Weather::Hail;
                        state.weather.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        flowergift: Ability {
            id: "flowergift".to_string(),
            index: 216,
            ..Default::default()
        },
        shedskin: Ability {
            id: "shedskin".to_string(),
            index: 217,
            ..Default::default()
        },
        wimpout: Ability {
            id: "wimpout".to_string(),
            index: 218,
            ..Default::default()
        },
        icescales: Ability {
            id: "icescales".to_string(),
            index: 219,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.category == MoveCategory::Special {
                        attacker_choice.base_power *= 0.5;
                    }
                },
            ),
            ..Default::default()
        },
        infiltrator: Ability {
            id: "infiltrator".to_string(),
            index: 220,
            ..Default::default()
        },
        limber: Ability {
            id: "limber".to_string(),
            index: 221,
            ..Default::default()
        },
        psychicsurge: Ability {
            id: "psychicsurge".to_string(),
            index: 222,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.terrain.terrain_type != Terrain::PsychicTerrain {
                    instructions.instruction_list.push(
                        Instruction::ChangeTerrain(ChangeTerrain {
                            new_terrain: Terrain::PsychicTerrain,
                            new_terrain_turns_remaining: 5,
                            previous_terrain: state.terrain.terrain_type,
                            previous_terrain_turns_remaining: state.terrain.turns_remaining,
                        }));
                        state.terrain.terrain_type = Terrain::PsychicTerrain;
                        state.terrain.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        defeatist: Ability {
            id: "defeatist".to_string(),
            index: 223,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let attacking_pokemon = state.get_side_immutable(attacking_side).get_active_immutable();
                    if attacking_pokemon.hp < attacking_pokemon.maxhp / 2 {
                        attacking_choice.base_power *= 0.5;
                    }
                },
            ),
            ..Default::default()
        },
        waterabsorb: Ability {
            id: "waterabsorb".to_string(),
            index: 224,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Water {
                        attacker_choice.base_power = 0.0;
                        attacker_choice.heal = Some(Heal {
                            target: MoveTarget::Opponent,
                            amount: 0.25
                        });
                        attacker_choice.category = MoveCategory::Status;
                    }
                },
            ),
            ..Default::default()
        },
        imposter: Ability {
            id: "imposter".to_string(),
            index: 225,
            ..Default::default()
        },
        dryskin: Ability {
            id: "dryskin".to_string(),
            index: 226,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Water {
                        attacker_choice.base_power = 0.0;
                        attacker_choice.heal = Some(Heal {
                            target: MoveTarget::Opponent,
                            amount: 0.25
                        });
                        attacker_choice.category = MoveCategory::Status;
                    } else if attacker_choice.move_type == PokemonType::Fire {
                        attacker_choice.base_power *= 1.25;
                    }
                },
            ),
            end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                if state.weather_is_active(&Weather::Rain) {
                    let active_pkmn = state.get_side(side_ref).get_active();

                    if active_pkmn.hp < active_pkmn.maxhp {
                        let heal_amount = cmp::min(active_pkmn.maxhp / 8, active_pkmn.maxhp - active_pkmn.hp);
                        let ins = Instruction::Heal(HealInstruction {
                            side_ref: side_ref.clone(),
                            heal_amount: heal_amount,
                        });
                        active_pkmn.hp += heal_amount;
                        incoming_instructions.instruction_list.push(ins);
                    }
                }
            }),
            ..Default::default()
        },
        fluffy: Ability {
            id: "fluffy".to_string(),
            index: 227,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.contact {
                        attacker_choice.base_power *= 0.5;
                    }
                    if attacker_choice.move_type == PokemonType::Fire {
                        attacker_choice.base_power *= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        unburden: Ability {
            id: "unburden".to_string(),
            index: 228,
            ..Default::default()
        },
        cheekpouch: Ability {
            id: "cheekpouch".to_string(),
            index: 229,
            ..Default::default()
        },
        stancechange: Ability {
            id: "stancechange".to_string(),
            index: 230,
            ..Default::default()
        },
        moody: Ability {
            id: "moody".to_string(),
            index: 231,
            ..Default::default()
        },
        rockypayload: Ability {
            id: "rockypayload".to_string(),
            index: 232,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.move_type == PokemonType::Rock {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        punkrock: Ability {
            id: "punkrock".to_string(),
            index: 233,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.flags.sound {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.sound {
                        attacker_choice.base_power /= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        sandveil: Ability {
            id: "sandveil".to_string(),
            index: 234,
            ..Default::default()
        },
        parentalbond: Ability {
            id: "parentalbond".to_string(),
            index: 235,
            ..Default::default()
        },
        strongjaw: Ability {
            id: "strongjaw".to_string(),
            index: 236,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.flags.bite {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        battery: Ability {
            id: "battery".to_string(),
            index: 237,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.category == MoveCategory::Special {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            ..Default::default()
        },
        healer: Ability {
            id: "healer".to_string(),
            index: 238,
            ..Default::default()
        },
        steadfast: Ability {
            id: "steadfast".to_string(),
            index: 239,
            ..Default::default()
        },
        damp: Ability {
            id: "damp".to_string(),
            index: 240,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if ["selfdestruct", "explosion", "mindblown", "mistyexplosion"].contains(&attacker_choice.move_id.as_str()) {
                        attacker_choice.accuracy = 0.0;
                        attacker_choice.heal = None;
                    }
                },
            ),
            ..Default::default()
        },
        perishbody: Ability {
            id: "perishbody".to_string(),
            index: 241,
            ..Default::default()
        },
        triage: Ability {
            id: "triage".to_string(),
            index: 242,
            ..Default::default()
        },
        sheerforce: Ability {
            id: "sheerforce".to_string(),
            index: 243,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.secondaries.is_some() {
                        attacking_choice.base_power *= 1.3;
                        attacking_choice.secondaries = None
                    }
                },
            ),
            ..Default::default()
        },
        owntempo: Ability {
            id: "owntempo".to_string(),
            index: 244,
            ..Default::default()
        },
        frisk: Ability {
            id: "frisk".to_string(),
            index: 245,
            ..Default::default()
        },
        voltabsorb: Ability {
            id: "voltabsorb".to_string(),
            index: 246,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Electric {
                        attacker_choice.remove_all_effects();
                        attacker_choice.accuracy = 100.0;
                        attacker_choice.base_power = 0.0;
                        attacker_choice.heal = Some(Heal {
                            target: MoveTarget::Opponent,
                            amount: 0.25
                        });
                        attacker_choice.category = MoveCategory::Status;
                    }
                },
            ),
            ..Default::default()
        },
        galewings: Ability {
            id: "galewings".to_string(),
            index: 247,
            ..Default::default()
        },
        aftermath: Ability {
            id: "aftermath".to_string(),
            index: 248,
            ..Default::default()
        },
        stickyhold: Ability {
            id: "stickyhold".to_string(),
            index: 249,
            ..Default::default()
        },
        grimneigh: Ability {
            id: "grimneigh".to_string(),
            index: 250,
            ..Default::default()
        },
        ironfist: Ability {
            id: "ironfist".to_string(),
            index: 251,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.flags.punch {
                        attacking_choice.base_power *= 1.2;
                    }
                },
            ),
            ..Default::default()
        },
        rebound: Ability {
            id: "rebound".to_string(),
            index: 252,
            ..Default::default()
        },
        unseenfist: Ability {
            id: "unseenfist".to_string(),
            index: 253,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.flags.contact {
                        attacking_choice.flags.protect = false
                    }
                },
            ),
            ..Default::default()
        },
        solidrock: Ability {
            id: "solidrock".to_string(),
            index: 254,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if type_effectiveness_modifier(
                        &attacker_choice.move_type,
                        &state
                            .get_side_immutable(&attacking_side.get_other_side())
                            .get_active_immutable()
                            .types,
                    ) > 1.0
                    {
                        attacker_choice.base_power *= 0.75;
                    }
                },
            ),
            ..Default::default()
        },
        hustle: Ability {
            id: "hustle".to_string(),
            index: 255,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if attacking_choice.category == MoveCategory::Physical {
                        attacking_choice.base_power *= 1.5;
                        attacking_choice.accuracy *= 0.80
                    }
                },
            ),
            ..Default::default()
        },
        hydration: Ability {
            id: "hydration".to_string(),
            index: 256,
            end_of_turn: Some(|state: &mut State, side_ref: &SideReference, incoming_instructions: &mut StateInstructions| {
                let attacker = state.get_side(side_ref).get_active();
                let attacker_status = attacker.status;
                if attacker_status != PokemonStatus::None {
                    attacker.status = PokemonStatus::None;
                    let ins = Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: *side_ref,
                        pokemon_index: state.get_side_immutable(side_ref).active_index,
                        old_status: attacker_status,
                        new_status: PokemonStatus::None,
                    });
                    incoming_instructions.instruction_list.push(ins);
                }
            }),
            ..Default::default()
        },
        scrappy: Ability {
            id: "scrappy".to_string(),
            index: 257,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if state.get_side_immutable(&attacking_side.get_other_side()).get_active_immutable().has_type(&PokemonType::Ghost) {
                        // Technically wrong, come back to this later
                        attacking_choice.move_type = PokemonType::Typeless;
                    }
                },
            ),
            ..Default::default()
        },
        overcoat: Ability {
            id: "overcoat".to_string(),
            index: 258,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.flags.powder {
                        attacker_choice.remove_all_effects();
                        attacker_choice.accuracy = 0.0
                    }
                },
            ),
            ..Default::default()
        },
        neutralizinggas: Ability {
            id: "neutralizinggas".to_string(),
            index: 259,
            ..Default::default()
        },
        sweetveil: Ability {
            id: "sweetveil".to_string(),
            index: 260,
            ..Default::default()
        },
        drizzle: Ability {
            id: "drizzle".to_string(),
            index: 261,
            on_switch_in: Some(|state: &mut State, side_ref: &SideReference, instructions: &mut StateInstructions| {
                if state.weather.weather_type != Weather::Rain {
                    instructions.instruction_list.push(
                        Instruction::ChangeWeather(ChangeWeather {
                            new_weather: Weather::Rain,
                            new_weather_turns_remaining: 5,
                            previous_weather: state.weather.weather_type,
                            previous_weather_turns_remaining: state.weather.turns_remaining,
                        }));
                        state.weather.weather_type = Weather::Rain;
                        state.weather.turns_remaining = 5;
                }
            }),
            ..Default::default()
        },
        innerfocus: Ability {
            id: "innerfocus".to_string(),
            index: 262,
            ..Default::default()
        },
        poisontouch: Ability {
            id: "poisontouch".to_string(),
            index: 263,
            modify_attack_being_used: Some(
                    |state, attacking_choice, defender_choice, attacking_side| {
                        if attacking_choice.flags.contact {
                            attacking_choice.add_or_create_secondaries(
                                Secondary {
                                    chance: 30.0,
                                    target: MoveTarget::Opponent,
                                    effect: Effect::Status(PokemonStatus::Poison),
                                }
                            )
                        }
                    },
                ),
            ..Default::default()
        },
        wanderingspirit: Ability {
            id: "wanderingspirit".to_string(),
            index: 264,
            ..Default::default()
        },
        guts: Ability {
            id: "guts".to_string(),
            index: 265,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let attacking_pkmn = state.get_side_immutable(attacking_side).get_active_immutable();
                    if attacking_pkmn.status != PokemonStatus::None {
                        attacking_choice.base_power *= 1.5;

                        // not the right place to put this, but good enough
                        if attacking_pkmn.status == PokemonStatus::Burn {
                            attacking_choice.base_power *= 2.0;
                        }
                    }
                },
            ),
            ..Default::default()
        },
        shellarmor: Ability {
            id: "shellarmor".to_string(),
            index: 266,
            ..Default::default()
        },
        rattled: Ability {
            id: "rattled".to_string(),
            index: 267,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Bug
                    || attacker_choice.move_type == PokemonType::Dark
                    || attacker_choice.move_type == PokemonType::Ghost {
                        attacker_choice.add_or_create_secondaries(
                            Secondary {
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
                            }
                        );
                    }
                },
            ),
            ..Default::default()
        },
        waterbubble: Ability {
            id: "waterbubble".to_string(),
            index: 268,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Fire {
                        attacker_choice.base_power /= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
        sandforce: Ability {
            id: "sandforce".to_string(),
            index: 269,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    if state.weather_is_active(&Weather::Sand)
                        && (attacking_choice.move_type == PokemonType::Rock
                            || attacking_choice.move_type == PokemonType::Ground
                            || attacking_choice.move_type == PokemonType::Steel)
                    {
                        attacking_choice.base_power *= 1.3;
                    }
                },
            ),
            ..Default::default()
        },
        toxicboost: Ability {
            id: "toxicboost".to_string(),
            index: 270,
            modify_attack_being_used: Some(
                |state, attacking_choice, defender_choice, attacking_side| {
                    let active_pkmn = state.get_side_immutable(attacking_side).get_active_immutable();
                    if active_pkmn.status == PokemonStatus::Poison
                    || active_pkmn.status == PokemonStatus::Toxic {
                        attacking_choice.base_power *= 1.5;
                    }
                },
            ),
            ..Default::default()
        },
        persistent: Ability {
            id: "persistent".to_string(),
            index: 271,
            ..Default::default()
        },
        chlorophyll: Ability {
            id: "chlorophyll".to_string(),
            index: 272,
            ..Default::default()
        },
        simple: Ability {
            id: "simple".to_string(),
            index: 273,
            ..Default::default()
        },
        purifyingsalt: Ability {
            id: "purifyingsalt".to_string(),
            index: 275,
            modify_attack_against: Some(
                |state, attacker_choice: &mut Choice, _defender_choice, attacking_side| {
                    if attacker_choice.move_type == PokemonType::Ghost {
                        attacker_choice.base_power /= 2.0;
                    }
                },
            ),
            ..Default::default()
        },
    };
}

pub fn ability_from_index<'a>(index: &Abilities) -> &'a Ability {
    match index {
        Abilities::NONE => &ALL_ABILITIES.none,
        Abilities::RIPEN => &ALL_ABILITIES.ripen,
        Abilities::TANGLEDFEET => &ALL_ABILITIES.tangledfeet,
        Abilities::DRAGONSMAW => &ALL_ABILITIES.dragonsmaw,
        Abilities::CLEARBODY => &ALL_ABILITIES.clearbody,
        Abilities::GALVANIZE => &ALL_ABILITIES.galvanize,
        Abilities::VITALSPIRIT => &ALL_ABILITIES.vitalspirit,
        Abilities::AERILATE => &ALL_ABILITIES.aerilate,
        Abilities::DEFIANT => &ALL_ABILITIES.defiant,
        Abilities::CUTECHARM => &ALL_ABILITIES.cutecharm,
        Abilities::NEUROFORCE => &ALL_ABILITIES.neuroforce,
        Abilities::SOUNDPROOF => &ALL_ABILITIES.soundproof,
        Abilities::RKSSYSTEM => &ALL_ABILITIES.rkssystem,
        Abilities::POISONPOINT => &ALL_ABILITIES.poisonpoint,
        Abilities::STAKEOUT => &ALL_ABILITIES.stakeout,
        Abilities::UNNERVE => &ALL_ABILITIES.unnerve,
        Abilities::ROCKHEAD => &ALL_ABILITIES.rockhead,
        Abilities::AURABREAK => &ALL_ABILITIES.aurabreak,
        Abilities::MIMICRY => &ALL_ABILITIES.mimicry,
        Abilities::BULLETPROOF => &ALL_ABILITIES.bulletproof,
        Abilities::POWEROFALCHEMY => &ALL_ABILITIES.powerofalchemy,
        Abilities::TECHNICIAN => &ALL_ABILITIES.technician,
        Abilities::MULTISCALE => &ALL_ABILITIES.multiscale,
        Abilities::ARENATRAP => &ALL_ABILITIES.arenatrap,
        Abilities::BATTLEBOND => &ALL_ABILITIES.battlebond,
        Abilities::DISGUISE => &ALL_ABILITIES.disguise,
        Abilities::EARLYBIRD => &ALL_ABILITIES.earlybird,
        Abilities::LIGHTNINGROD => &ALL_ABILITIES.lightningrod,
        Abilities::MAGICIAN => &ALL_ABILITIES.magician,
        Abilities::REFRIGERATE => &ALL_ABILITIES.refrigerate,
        Abilities::FRIENDGUARD => &ALL_ABILITIES.friendguard,
        Abilities::NOABILITY => &ALL_ABILITIES.noability,
        Abilities::GULPMISSILE => &ALL_ABILITIES.gulpmissile,
        Abilities::POWERCONSTRUCT => &ALL_ABILITIES.powerconstruct,
        Abilities::FORECAST => &ALL_ABILITIES.forecast,
        Abilities::PRANKSTER => &ALL_ABILITIES.prankster,
        Abilities::PROTEAN => &ALL_ABILITIES.protean,
        Abilities::ASONEGLASTRIER => &ALL_ABILITIES.asoneglastrier,
        Abilities::SHADOWTAG => &ALL_ABILITIES.shadowtag,
        Abilities::SKILLLINK => &ALL_ABILITIES.skilllink,
        Abilities::INTREPIDSWORD => &ALL_ABILITIES.intrepidsword,
        Abilities::SOULHEART => &ALL_ABILITIES.soulheart,
        Abilities::SWIFTSWIM => &ALL_ABILITIES.swiftswim,
        Abilities::EARTHEATER => &ALL_ABILITIES.eartheater,
        Abilities::SUPERLUCK => &ALL_ABILITIES.superluck,
        Abilities::SUPREMEOVERLORD => &ALL_ABILITIES.supremeoverlord,
        Abilities::INSOMNIA => &ALL_ABILITIES.insomnia,
        Abilities::DANCER => &ALL_ABILITIES.dancer,
        Abilities::STEAMENGINE => &ALL_ABILITIES.steamengine,
        Abilities::ANGERPOINT => &ALL_ABILITIES.angerpoint,
        Abilities::CONTRARY => &ALL_ABILITIES.contrary,
        Abilities::MAGMAARMOR => &ALL_ABILITIES.magmaarmor,
        Abilities::HUNGERSWITCH => &ALL_ABILITIES.hungerswitch,
        Abilities::RECEIVER => &ALL_ABILITIES.receiver,
        Abilities::ZENMODE => &ALL_ABILITIES.zenmode,
        Abilities::EMERGENCYEXIT => &ALL_ABILITIES.emergencyexit,
        Abilities::ILLUSION => &ALL_ABILITIES.illusion,
        Abilities::WEAKARMOR => &ALL_ABILITIES.weakarmor,
        Abilities::DROUGHT => &ALL_ABILITIES.drought,
        Abilities::INNARDSOUT => &ALL_ABILITIES.innardsout,
        Abilities::SHIELDSDOWN => &ALL_ABILITIES.shieldsdown,
        Abilities::ADAPTABILITY => &ALL_ABILITIES.adaptability,
        Abilities::CORROSION => &ALL_ABILITIES.corrosion,
        Abilities::LONGREACH => &ALL_ABILITIES.longreach,
        Abilities::PUREPOWER => &ALL_ABILITIES.purepower,
        Abilities::TINTEDLENS => &ALL_ABILITIES.tintedlens,
        Abilities::QUEENLYMAJESTY => &ALL_ABILITIES.queenlymajesty,
        Abilities::DESOLATELAND => &ALL_ABILITIES.desolateland,
        Abilities::MOXIE => &ALL_ABILITIES.moxie,
        Abilities::SAPSIPPER => &ALL_ABILITIES.sapsipper,
        Abilities::SLUSHRUSH => &ALL_ABILITIES.slushrush,
        Abilities::BIGPECKS => &ALL_ABILITIES.bigpecks,
        Abilities::STALL => &ALL_ABILITIES.stall,
        Abilities::WHITESMOKE => &ALL_ABILITIES.whitesmoke,
        Abilities::FLAREBOOST => &ALL_ABILITIES.flareboost,
        Abilities::SHADOWSHIELD => &ALL_ABILITIES.shadowshield,
        Abilities::LIQUIDVOICE => &ALL_ABILITIES.liquidvoice,
        Abilities::MISTYSURGE => &ALL_ABILITIES.mistysurge,
        Abilities::MULTITYPE => &ALL_ABILITIES.multitype,
        Abilities::NOGUARD => &ALL_ABILITIES.noguard,
        Abilities::TORRENT => &ALL_ABILITIES.torrent,
        Abilities::DELTASTREAM => &ALL_ABILITIES.deltastream,
        Abilities::KLUTZ => &ALL_ABILITIES.klutz,
        Abilities::LIBERO => &ALL_ABILITIES.libero,
        Abilities::SERENEGRACE => &ALL_ABILITIES.serenegrace,
        Abilities::CURSEDBODY => &ALL_ABILITIES.cursedbody,
        Abilities::UNAWARE => &ALL_ABILITIES.unaware,
        Abilities::LIGHTMETAL => &ALL_ABILITIES.lightmetal,
        Abilities::MARVELSCALE => &ALL_ABILITIES.marvelscale,
        Abilities::TELEPATHY => &ALL_ABILITIES.telepathy,
        Abilities::QUICKDRAW => &ALL_ABILITIES.quickdraw,
        Abilities::HYPERCUTTER => &ALL_ABILITIES.hypercutter,
        Abilities::SYMBIOSIS => &ALL_ABILITIES.symbiosis,
        Abilities::PLUS => &ALL_ABILITIES.plus,
        Abilities::MIRRORARMOR => &ALL_ABILITIES.mirrorarmor,
        Abilities::PASTELVEIL => &ALL_ABILITIES.pastelveil,
        Abilities::TOUGHCLAWS => &ALL_ABILITIES.toughclaws,
        Abilities::EFFECTSPORE => &ALL_ABILITIES.effectspore,
        Abilities::MUMMY => &ALL_ABILITIES.mummy,
        Abilities::BADDREAMS => &ALL_ABILITIES.baddreams,
        Abilities::MAGICGUARD => &ALL_ABILITIES.magicguard,
        Abilities::SANDSTREAM => &ALL_ABILITIES.sandstream,
        Abilities::POWERSPOT => &ALL_ABILITIES.powerspot,
        Abilities::FLAMEBODY => &ALL_ABILITIES.flamebody,
        Abilities::RECKLESS => &ALL_ABILITIES.reckless,
        Abilities::PRESSURE => &ALL_ABILITIES.pressure,
        Abilities::GOOEY => &ALL_ABILITIES.gooey,
        Abilities::IMMUNITY => &ALL_ABILITIES.immunity,
        Abilities::LEAFGUARD => &ALL_ABILITIES.leafguard,
        Abilities::HUGEPOWER => &ALL_ABILITIES.hugepower,
        Abilities::SOLARPOWER => &ALL_ABILITIES.solarpower,
        Abilities::SCHOOLING => &ALL_ABILITIES.schooling,
        Abilities::MOTORDRIVE => &ALL_ABILITIES.motordrive,
        Abilities::ANTICIPATION => &ALL_ABILITIES.anticipation,
        Abilities::MERCILESS => &ALL_ABILITIES.merciless,
        Abilities::TRACE => &ALL_ABILITIES.trace,
        Abilities::NATURALCURE => &ALL_ABILITIES.naturalcure,
        Abilities::HARVEST => &ALL_ABILITIES.harvest,
        Abilities::SUCTIONCUPS => &ALL_ABILITIES.suctioncups,
        Abilities::ICEFACE => &ALL_ABILITIES.iceface,
        Abilities::ROUGHSKIN => &ALL_ABILITIES.roughskin,
        Abilities::WONDERGUARD => &ALL_ABILITIES.wonderguard,
        Abilities::WATERVEIL => &ALL_ABILITIES.waterveil,
        Abilities::FAIRYAURA => &ALL_ABILITIES.fairyaura,
        Abilities::SANDSPIT => &ALL_ABILITIES.sandspit,
        Abilities::INTIMIDATE => &ALL_ABILITIES.intimidate,
        Abilities::DAUNTLESSSHIELD => &ALL_ABILITIES.dauntlessshield,
        Abilities::AROMAVEIL => &ALL_ABILITIES.aromaveil,
        Abilities::AIRLOCK => &ALL_ABILITIES.airlock,
        Abilities::NORMALIZE => &ALL_ABILITIES.normalize,
        Abilities::DARKAURA => &ALL_ABILITIES.darkaura,
        Abilities::VICTORYSTAR => &ALL_ABILITIES.victorystar,
        Abilities::GRASSYSURGE => &ALL_ABILITIES.grassysurge,
        Abilities::STURDY => &ALL_ABILITIES.sturdy,
        Abilities::PICKPOCKET => &ALL_ABILITIES.pickpocket,
        Abilities::ELECTRICSURGE => &ALL_ABILITIES.electricsurge,
        Abilities::RUNAWAY => &ALL_ABILITIES.runaway,
        Abilities::OBLIVIOUS => &ALL_ABILITIES.oblivious,
        Abilities::SURGESURFER => &ALL_ABILITIES.surgesurfer,
        Abilities::LEVITATE => &ALL_ABILITIES.levitate,
        Abilities::ASONESPECTRIER => &ALL_ABILITIES.asonespectrier,
        Abilities::PICKUP => &ALL_ABILITIES.pickup,
        Abilities::ICEBODY => &ALL_ABILITIES.icebody,
        Abilities::CURIOUSMEDICINE => &ALL_ABILITIES.curiousmedicine,
        Abilities::FLOWERVEIL => &ALL_ABILITIES.flowerveil,
        Abilities::STATIC => &ALL_ABILITIES._static,
        Abilities::WONDERSKIN => &ALL_ABILITIES.wonderskin,
        Abilities::OVERGROW => &ALL_ABILITIES.overgrow,
        Abilities::PROPELLERTAIL => &ALL_ABILITIES.propellertail,
        Abilities::THICKFAT => &ALL_ABILITIES.thickfat,
        Abilities::GLUTTONY => &ALL_ABILITIES.gluttony,
        Abilities::KEENEYE => &ALL_ABILITIES.keeneye,
        Abilities::MOUNTAINEER => &ALL_ABILITIES.mountaineer,
        Abilities::FLASHFIRE => &ALL_ABILITIES.flashfire,
        Abilities::COMPOUNDEYES => &ALL_ABILITIES.compoundeyes,
        Abilities::STEELWORKER => &ALL_ABILITIES.steelworker,
        Abilities::COMATOSE => &ALL_ABILITIES.comatose,
        Abilities::BALLFETCH => &ALL_ABILITIES.ballfetch,
        Abilities::DAZZLING => &ALL_ABILITIES.dazzling,
        Abilities::DOWNLOAD => &ALL_ABILITIES.download,
        Abilities::TRANSISTOR => &ALL_ABILITIES.transistor,
        Abilities::MOLDBREAKER => &ALL_ABILITIES.moldbreaker,
        Abilities::LIQUIDOOZE => &ALL_ABILITIES.liquidooze,
        Abilities::POISONHEAL => &ALL_ABILITIES.poisonheal,
        Abilities::PRISMARMOR => &ALL_ABILITIES.prismarmor,
        Abilities::SNIPER => &ALL_ABILITIES.sniper,
        Abilities::STENCH => &ALL_ABILITIES.stench,
        Abilities::COMPETITIVE => &ALL_ABILITIES.competitive,
        Abilities::SWARM => &ALL_ABILITIES.swarm,
        Abilities::STALWART => &ALL_ABILITIES.stalwart,
        Abilities::ILLUMINATE => &ALL_ABILITIES.illuminate,
        Abilities::TURBOBLAZE => &ALL_ABILITIES.turboblaze,
        Abilities::GORILLATACTICS => &ALL_ABILITIES.gorillatactics,
        Abilities::SPEEDBOOST => &ALL_ABILITIES.speedboost,
        Abilities::HEATPROOF => &ALL_ABILITIES.heatproof,
        Abilities::SNOWCLOAK => &ALL_ABILITIES.snowcloak,
        Abilities::TERAVOLT => &ALL_ABILITIES.teravolt,
        Abilities::CHILLINGNEIGH => &ALL_ABILITIES.chillingneigh,
        Abilities::SHIELDDUST => &ALL_ABILITIES.shielddust,
        Abilities::RIVALRY => &ALL_ABILITIES.rivalry,
        Abilities::PRIMORDIALSEA => &ALL_ABILITIES.primordialsea,
        Abilities::SCREENCLEANER => &ALL_ABILITIES.screencleaner,
        Abilities::MAGNETPULL => &ALL_ABILITIES.magnetpull,
        Abilities::HONEYGATHER => &ALL_ABILITIES.honeygather,
        Abilities::COTTONDOWN => &ALL_ABILITIES.cottondown,
        Abilities::GRASSPELT => &ALL_ABILITIES.grasspelt,
        Abilities::BATTLEARMOR => &ALL_ABILITIES.battlearmor,
        Abilities::BEASTBOOST => &ALL_ABILITIES.beastboost,
        Abilities::BERSERK => &ALL_ABILITIES.berserk,
        Abilities::MINUS => &ALL_ABILITIES.minus,
        Abilities::RAINDISH => &ALL_ABILITIES.raindish,
        Abilities::SYNCHRONIZE => &ALL_ABILITIES.synchronize,
        Abilities::FILTER => &ALL_ABILITIES.filter,
        Abilities::TRUANT => &ALL_ABILITIES.truant,
        Abilities::FURCOAT => &ALL_ABILITIES.furcoat,
        Abilities::FULLMETALBODY => &ALL_ABILITIES.fullmetalbody,
        Abilities::REGENERATOR => &ALL_ABILITIES.regenerator,
        Abilities::FOREWARN => &ALL_ABILITIES.forewarn,
        Abilities::IRONBARBS => &ALL_ABILITIES.ironbarbs,
        Abilities::STAMINA => &ALL_ABILITIES.stamina,
        Abilities::SANDRUSH => &ALL_ABILITIES.sandrush,
        Abilities::COLORCHANGE => &ALL_ABILITIES.colorchange,
        Abilities::BLAZE => &ALL_ABILITIES.blaze,
        Abilities::ANALYTIC => &ALL_ABILITIES.analytic,
        Abilities::TANGLINGHAIR => &ALL_ABILITIES.tanglinghair,
        Abilities::CLOUDNINE => &ALL_ABILITIES.cloudnine,
        Abilities::STEELYSPIRIT => &ALL_ABILITIES.steelyspirit,
        Abilities::QUICKFEET => &ALL_ABILITIES.quickfeet,
        Abilities::MAGICBOUNCE => &ALL_ABILITIES.magicbounce,
        Abilities::MEGALAUNCHER => &ALL_ABILITIES.megalauncher,
        Abilities::HEAVYMETAL => &ALL_ABILITIES.heavymetal,
        Abilities::STORMDRAIN => &ALL_ABILITIES.stormdrain,
        Abilities::PIXILATE => &ALL_ABILITIES.pixilate,
        Abilities::WATERCOMPACTION => &ALL_ABILITIES.watercompaction,
        Abilities::JUSTIFIED => &ALL_ABILITIES.justified,
        Abilities::SLOWSTART => &ALL_ABILITIES.slowstart,
        Abilities::SNOWWARNING => &ALL_ABILITIES.snowwarning,
        Abilities::FLOWERGIFT => &ALL_ABILITIES.flowergift,
        Abilities::SHEDSKIN => &ALL_ABILITIES.shedskin,
        Abilities::WIMPOUT => &ALL_ABILITIES.wimpout,
        Abilities::ICESCALES => &ALL_ABILITIES.icescales,
        Abilities::INFILTRATOR => &ALL_ABILITIES.infiltrator,
        Abilities::LIMBER => &ALL_ABILITIES.limber,
        Abilities::PSYCHICSURGE => &ALL_ABILITIES.psychicsurge,
        Abilities::DEFEATIST => &ALL_ABILITIES.defeatist,
        Abilities::WATERABSORB => &ALL_ABILITIES.waterabsorb,
        Abilities::IMPOSTER => &ALL_ABILITIES.imposter,
        Abilities::DRYSKIN => &ALL_ABILITIES.dryskin,
        Abilities::FLUFFY => &ALL_ABILITIES.fluffy,
        Abilities::UNBURDEN => &ALL_ABILITIES.unburden,
        Abilities::CHEEKPOUCH => &ALL_ABILITIES.cheekpouch,
        Abilities::STANCECHANGE => &ALL_ABILITIES.stancechange,
        Abilities::MOODY => &ALL_ABILITIES.moody,
        Abilities::ROCKYPAYLOAD => &ALL_ABILITIES.rockypayload,
        Abilities::PUNKROCK => &ALL_ABILITIES.punkrock,
        Abilities::SANDVEIL => &ALL_ABILITIES.sandveil,
        Abilities::PARENTALBOND => &ALL_ABILITIES.parentalbond,
        Abilities::STRONGJAW => &ALL_ABILITIES.strongjaw,
        Abilities::BATTERY => &ALL_ABILITIES.battery,
        Abilities::HEALER => &ALL_ABILITIES.healer,
        Abilities::STEADFAST => &ALL_ABILITIES.steadfast,
        Abilities::DAMP => &ALL_ABILITIES.damp,
        Abilities::PERISHBODY => &ALL_ABILITIES.perishbody,
        Abilities::TRIAGE => &ALL_ABILITIES.triage,
        Abilities::SHEERFORCE => &ALL_ABILITIES.sheerforce,
        Abilities::OWNTEMPO => &ALL_ABILITIES.owntempo,
        Abilities::FRISK => &ALL_ABILITIES.frisk,
        Abilities::VOLTABSORB => &ALL_ABILITIES.voltabsorb,
        Abilities::GALEWINGS => &ALL_ABILITIES.galewings,
        Abilities::AFTERMATH => &ALL_ABILITIES.aftermath,
        Abilities::STICKYHOLD => &ALL_ABILITIES.stickyhold,
        Abilities::GRIMNEIGH => &ALL_ABILITIES.grimneigh,
        Abilities::IRONFIST => &ALL_ABILITIES.ironfist,
        Abilities::REBOUND => &ALL_ABILITIES.rebound,
        Abilities::UNSEENFIST => &ALL_ABILITIES.unseenfist,
        Abilities::SOLIDROCK => &ALL_ABILITIES.solidrock,
        Abilities::HUSTLE => &ALL_ABILITIES.hustle,
        Abilities::HYDRATION => &ALL_ABILITIES.hydration,
        Abilities::SCRAPPY => &ALL_ABILITIES.scrappy,
        Abilities::OVERCOAT => &ALL_ABILITIES.overcoat,
        Abilities::NEUTRALIZINGGAS => &ALL_ABILITIES.neutralizinggas,
        Abilities::SWEETVEIL => &ALL_ABILITIES.sweetveil,
        Abilities::DRIZZLE => &ALL_ABILITIES.drizzle,
        Abilities::INNERFOCUS => &ALL_ABILITIES.innerfocus,
        Abilities::POISONTOUCH => &ALL_ABILITIES.poisontouch,
        Abilities::WANDERINGSPIRIT => &ALL_ABILITIES.wanderingspirit,
        Abilities::GUTS => &ALL_ABILITIES.guts,
        Abilities::SHELLARMOR => &ALL_ABILITIES.shellarmor,
        Abilities::RATTLED => &ALL_ABILITIES.rattled,
        Abilities::WATERBUBBLE => &ALL_ABILITIES.waterbubble,
        Abilities::SANDFORCE => &ALL_ABILITIES.sandforce,
        Abilities::TOXICBOOST => &ALL_ABILITIES.toxicboost,
        Abilities::PERSISTENT => &ALL_ABILITIES.persistent,
        Abilities::CHLOROPHYLL => &ALL_ABILITIES.chlorophyll,
        Abilities::SIMPLE => &ALL_ABILITIES.simple,
        Abilities::PURIFYINGSALT => &ALL_ABILITIES.purifyingsalt,
    }
}

pub struct Ability {
    pub id: String,
    pub index: usize,
    pub modify_attack_being_used: Option<ModifyAttackBeingUsed>,
    pub modify_attack_against: Option<ModifyAttackAgainst>,
    pub before_move: Option<AbilityBeforeMove>,
    pub after_damage_hit: Option<AbilityAfterDamageHit>,
    pub on_switch_out: Option<AbilityOnSwitchOut>,
    pub on_switch_in: Option<AbilityOnSwitchIn>,
    pub end_of_turn: Option<AbilityEndOfTurn>,
}

impl Default for Ability {
    fn default() -> Ability {
        return Ability {
            id: "".to_string(),
            index: 0,
            modify_attack_being_used: None,
            modify_attack_against: None,
            before_move: None,
            after_damage_hit: None,
            on_switch_out: None,
            on_switch_in: None,
            end_of_turn: None,
        };
    }
}
