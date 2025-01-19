use crate::define_enum_with_from_str;
use crate::state::PokemonVolatileStatus;
use crate::state::{PokemonBoostableStat, PokemonSideCondition};
use crate::state::{PokemonIndex, PokemonStatus};
use crate::state::{PokemonMoveIndex, PokemonType};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

lazy_static! {
    pub static ref MOVES: HashMap<Choices, Choice> = {
        let mut moves: HashMap<Choices, Choice> = HashMap::new();
        moves.insert(
            Choices::NONE,
            Choice {
                move_id: Choices::NONE,
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ABSORB,
            Choice {
                move_id: Choices::ABSORB,
                base_power: 20.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ACCELEROCK,
            Choice {
                move_id: Choices::ACCELEROCK,
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::ACID,
                Choice {
                    move_id: Choices::ACID,
                    base_power: 40.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 33.2,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::ACID,
                Choice {
                    move_id: Choices::ACID,
                    base_power: 40.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ACID,
                Choice {
                    move_id: Choices::ACID,
                    base_power: 40.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ACIDARMOR,
            Choice {
                move_id: Choices::ACIDARMOR,
                target: MoveTarget::User,
                move_type: PokemonType::POISON,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ACIDSPRAY,
            Choice {
                move_id: Choices::ACIDSPRAY,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -2,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ACROBATICS,
            Choice {
                move_id: Choices::ACROBATICS,
                base_power: 55.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ACUPRESSURE,
            Choice {
                move_id: Choices::ACUPRESSURE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AERIALACE,
            Choice {
                move_id: Choices::AERIALACE,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AEROBLAST,
            Choice {
                move_id: Choices::AEROBLAST,
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AFTERYOU,
            Choice {
                move_id: Choices::AFTERYOU,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AGILITY,
            Choice {
                move_id: Choices::AGILITY,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::AIRCUTTER,
                Choice {
                    move_id: Choices::AIRCUTTER,
                    accuracy: 95.0,
                    base_power: 55.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                        protect: true,
                        slicing: true,
                        wind: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::AIRCUTTER,
                Choice {
                    move_id: Choices::AIRCUTTER,
                    accuracy: 95.0,
                    base_power: 60.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::AIRSLASH,
            Choice {
                move_id: Choices::AIRSLASH,
                accuracy: 95.0,
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ALLURINGVOICE,
            Choice {
                base_power: 80.0,
                category: MoveCategory::Special,
                move_id: Choices::ALLURINGVOICE,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    sound: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ALLYSWITCH,
            Choice {
                move_id: Choices::ALLYSWITCH,
                priority: 2,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::AMNESIA,
                Choice {
                    move_id: Choices::AMNESIA,
                    target: MoveTarget::User,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 2,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::AMNESIA,
                Choice {
                    move_id: Choices::AMNESIA,
                    target: MoveTarget::User,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 2,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ANCHORSHOT,
            Choice {
                move_id: Choices::ANCHORSHOT,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ANCIENTPOWER,
            Choice {
                move_id: Choices::ANCIENTPOWER,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::APPLEACID,
            Choice {
                move_id: Choices::APPLEACID,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AQUACUTTER,
            Choice {
                move_id: Choices::AQUACUTTER,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AQUAJET,
            Choice {
                move_id: Choices::AQUAJET,
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AQUARING,
            Choice {
                move_id: Choices::AQUARING,
                target: MoveTarget::User,
                move_type: PokemonType::WATER,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::AQUARING,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AQUASTEP,
            Choice {
                move_id: Choices::AQUASTEP,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AQUATAIL,
            Choice {
                move_id: Choices::AQUATAIL,
                accuracy: 90.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ARMORCANNON,
            Choice {
                move_id: Choices::ARMORCANNON,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ARMTHRUST,
            Choice {
                move_id: Choices::ARMTHRUST,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AROMATHERAPY,
            Choice {
                move_id: Choices::AROMATHERAPY,
                target: MoveTarget::User,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AROMATICMIST,
            Choice {
                move_id: Choices::AROMATICMIST,
                target: MoveTarget::User,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ASSIST,
            Choice {
                move_id: Choices::ASSIST,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::ASSURANCE,
                Choice {
                    move_id: Choices::ASSURANCE,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ASSURANCE,
                Choice {
                    move_id: Choices::ASSURANCE,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ASTONISH,
            Choice {
                move_id: Choices::ASTONISH,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ASTRALBARRAGE,
            Choice {
                move_id: Choices::ASTRALBARRAGE,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ATTACKORDER,
            Choice {
                move_id: Choices::ATTACKORDER,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ATTRACT,
            Choice {
                move_id: Choices::ATTRACT,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::ATTRACT,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::AURASPHERE,
                Choice {
                    move_id: Choices::AURASPHERE,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        bullet: true,
                                protect: true,
                        pulse: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::AURASPHERE,
                Choice {
                    move_id: Choices::AURASPHERE,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        bullet: true,
                                protect: true,
                        pulse: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::AURAWHEEL,
            Choice {
                move_id: Choices::AURAWHEEL,
                base_power: 110.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::AURORABEAM,
                Choice {
                    move_id: Choices::AURORABEAM,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 33.2,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::AURORABEAM,
                Choice {
                    move_id: Choices::AURORABEAM,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::AURORAVEIL,
            Choice {
                move_id: Choices::AURORAVEIL,
                target: MoveTarget::User,
                move_type: PokemonType::ICE,
                flags: Flags {
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::AuroraVeil,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AUTOTOMIZE,
            Choice {
                move_id: Choices::AUTOTOMIZE,
                target: MoveTarget::User,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AVALANCHE,
            Choice {
                move_id: Choices::AVALANCHE,
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: -4,
                move_type: PokemonType::ICE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::AXEKICK,
            Choice {
                move_id: Choices::AXEKICK,
                accuracy: 90.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                crash: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BABYDOLLEYES,
            Choice {
                move_id: Choices::BABYDOLLEYES,
                priority: 1,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BADDYBAD,
            Choice {
                move_id: Choices::BADDYBAD,
                accuracy: 95.0,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BANEFULBUNKER,
            Choice {
                move_id: Choices::BANEFULBUNKER,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::POISON,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::BANEFULBUNKER,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BARBBARRAGE,
            Choice {
                move_id: Choices::BARBBARRAGE,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BARRAGE,
            Choice {
                move_id: Choices::BARRAGE,
                accuracy: 85.0,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BARRIER,
            Choice {
                move_id: Choices::BARRIER,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BATONPASS,
            Choice {
                move_id: Choices::BATONPASS,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    pivot: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BEAKBLAST,
            Choice {
                move_id: Choices::BEAKBLAST,
                base_power: 100.0,
                category: MoveCategory::Physical,
                priority: -3,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::BEATUP,
                Choice {
                    base_power: 10.0,
                    move_id: Choices::BEATUP,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::BEATUP,
                Choice {
                    move_id: Choices::BEATUP,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::BEHEMOTHBASH,
            Choice {
                move_id: Choices::BEHEMOTHBASH,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BEHEMOTHBLADE,
            Choice {
                move_id: Choices::BEHEMOTHBLADE,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BELCH,
            Choice {
                move_id: Choices::BELCH,
                accuracy: 90.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BELLYDRUM,
            Choice {
                move_id: Choices::BELLYDRUM,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BESTOW,
            Choice {
                move_id: Choices::BESTOW,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BIDE,
            Choice {
                move_id: Choices::BIDE,
                category: MoveCategory::Physical,
                priority: 1,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::BIDE,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::BIND,
                Choice {
                    move_id: Choices::BIND,
                    accuracy: 75.0,
                    base_power: 15.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::BIND,
                Choice {
                    move_id: Choices::BIND,
                    accuracy: 85.0,
                    base_power: 15.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::BITE,
                Choice {
                    move_id: Choices::BITE,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        bite: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::BITE,
                Choice {
                    move_id: Choices::BITE,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        bite: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::BITTERBLADE,
            Choice {
                move_id: Choices::BITTERBLADE,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    heal: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BITTERMALICE,
            Choice {
                move_id: Choices::BITTERMALICE,
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BLASTBURN,
            Choice {
                move_id: Choices::BLASTBURN,
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BLAZEKICK,
            Choice {
                move_id: Choices::BLAZEKICK,
                accuracy: 90.0,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BLAZINGTORQUE,
            Choice {
                move_id: Choices::BLAZINGTORQUE,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BLEAKWINDSTORM,
            Choice {
                move_id: Choices::BLEAKWINDSTORM,
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::BLIZZARD,
                Choice {
                    move_id: Choices::BLIZZARD,
                    accuracy: 90.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                        protect: true,
                        wind: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::FREEZE),
                    }]),
                    ..Default::default()
                },
            );
        }
        else if cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::BLIZZARD,
                Choice {
                    move_id: Choices::BLIZZARD,
                    accuracy: 70.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                        protect: true,
                        wind: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::FREEZE),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::BLIZZARD,
                Choice {
                    move_id: Choices::BLIZZARD,
                    accuracy: 70.0,
                    base_power: 110.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::FREEZE),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::BLOCK,
            Choice {
                move_id: Choices::BLOCK,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BLOODMOON,
            Choice {
                move_id: Choices::BLOODMOON,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BLUEFLARE,
            Choice {
                move_id: Choices::BLUEFLARE,
                accuracy: 85.0,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BODYPRESS,
            Choice {
                move_id: Choices::BODYPRESS,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BODYSLAM,
            Choice {
                move_id: Choices::BODYSLAM,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BOLTBEAK,
            Choice {
                move_id: Choices::BOLTBEAK,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BOLTSTRIKE,
            Choice {
                move_id: Choices::BOLTSTRIKE,
                accuracy: 85.0,
                base_power: 130.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BONECLUB,
            Choice {
                move_id: Choices::BONECLUB,
                accuracy: 85.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BONEMERANG,
            Choice {
                move_id: Choices::BONEMERANG,
                accuracy: 90.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::BONERUSH,
                Choice {
                    move_id: Choices::BONERUSH,
                    accuracy: 80.0,
                    base_power: 25.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GROUND,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::BONERUSH,
                Choice {
                    move_id: Choices::BONERUSH,
                    accuracy: 90.0,
                    base_power: 25.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GROUND,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::BOOMBURST,
            Choice {
                move_id: Choices::BOOMBURST,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BOUNCE,
            Choice {
                move_id: Choices::BOUNCE,
                accuracy: 85.0,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    charge: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BOUNCYBUBBLE,
            Choice {
                move_id: Choices::BOUNCYBUBBLE,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BRANCHPOKE,
            Choice {
                move_id: Choices::BRANCHPOKE,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BRAVEBIRD,
            Choice {
                move_id: Choices::BRAVEBIRD,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BREAKINGSWIPE,
            Choice {
                move_id: Choices::BREAKINGSWIPE,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BRICKBREAK,
            Choice {
                move_id: Choices::BRICKBREAK,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BRINE,
            Choice {
                move_id: Choices::BRINE,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BRUTALSWING,
            Choice {
                move_id: Choices::BRUTALSWING,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::BUBBLE,
                Choice {
                    move_id: Choices::BUBBLE,
                    base_power: 20.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 33.2,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::BUBBLE,
                Choice {
                    move_id: Choices::BUBBLE,
                    base_power: 20.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::BUBBLE,
                Choice {
                    move_id: Choices::BUBBLE,
                    base_power: 40.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::BUBBLEBEAM,
                Choice {
                    move_id: Choices::BUBBLEBEAM,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 33.2,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::BUBBLEBEAM,
                Choice {
                    move_id: Choices::BUBBLEBEAM,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::BUGBITE,
            Choice {
                move_id: Choices::BUGBITE,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BUGBUZZ,
            Choice {
                move_id: Choices::BUGBUZZ,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BULKUP,
            Choice {
                move_id: Choices::BULKUP,
                target: MoveTarget::User,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BULLDOZE,
            Choice {
                move_id: Choices::BULLDOZE,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BULLETPUNCH,
            Choice {
                move_id: Choices::BULLETPUNCH,
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::BULLETSEED,
                Choice {
                    move_id: Choices::BULLETSEED,
                    base_power: 10.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        bullet: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::BULLETSEED,
                Choice {
                    move_id: Choices::BULLETSEED,
                    base_power: 25.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        bullet: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::BURNINGBULWARK,
            Choice {
                move_id: Choices::BURNINGBULWARK,
                category: MoveCategory::Status,
                move_type: PokemonType::FIRE,
                priority: 4,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::BURNINGBULWARK,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BURNINGJEALOUSY,
            Choice {
                move_id: Choices::BURNINGJEALOUSY,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BURNUP,
            Choice {
                move_id: Choices::BURNUP,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::BUZZYBUZZ,
            Choice {
                move_id: Choices::BUZZYBUZZ,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CALMMIND,
            Choice {
                move_id: Choices::CALMMIND,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CAMOUFLAGE,
            Choice {
                move_id: Choices::CAMOUFLAGE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CAPTIVATE,
            Choice {
                move_id: Choices::CAPTIVATE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CEASELESSEDGE,
            Choice {
                move_id: Choices::CEASELESSEDGE,
                accuracy: 90.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::Opponent,
                    condition: PokemonSideCondition::Spikes,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CELEBRATE,
            Choice {
                move_id: Choices::CELEBRATE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CHARGE,
            Choice {
                move_id: Choices::CHARGE,
                target: MoveTarget::User,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::CHARGE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CHARGEBEAM,
            Choice {
                move_id: Choices::CHARGEBEAM,
                accuracy: 90.0,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 70.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CHARM,
            Choice {
                move_id: Choices::CHARM,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -2,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::CHATTER,
                Choice {
                    move_id: Choices::CHATTER,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                                protect: true,
                        sound: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::CHATTER,
                Choice {
                    move_id: Choices::CHATTER,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                                protect: true,
                        sound: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::CHILLINGWATER,
            Choice {
                move_id: Choices::CHILLINGWATER,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CHILLYRECEPTION,
            Choice {
                move_id: Choices::CHILLYRECEPTION,
                move_type: PokemonType::ICE,
                flags: Flags {
                    pivot: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CHIPAWAY,
            Choice {
                move_id: Choices::CHIPAWAY,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CHLOROBLAST,
            Choice {
                move_id: Choices::CHLOROBLAST,
                accuracy: 95.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CIRCLETHROW,
            Choice {
                move_id: Choices::CIRCLETHROW,
                accuracy: 90.0,
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: -6,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    drag: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::CLAMP,
                Choice {
                    move_id: Choices::CLAMP,
                    accuracy: 75.0,
                    base_power: 35.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::CLAMP,
                Choice {
                    move_id: Choices::CLAMP,
                    accuracy: 85.0,
                    base_power: 35.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::CLANGINGSCALES,
            Choice {
                move_id: Choices::CLANGINGSCALES,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CLANGOROUSSOUL,
            Choice {
                move_id: Choices::CLANGOROUSSOUL,
                target: MoveTarget::User,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CLEARSMOG,
            Choice {
                move_id: Choices::CLEARSMOG,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CLOSECOMBAT,
            Choice {
                move_id: Choices::CLOSECOMBAT,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COACHING,
            Choice {
                move_id: Choices::COACHING,
                target: MoveTarget::User,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COIL,
            Choice {
                move_id: Choices::COIL,
                target: MoveTarget::User,
                move_type: PokemonType::POISON,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 1,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COLLISIONCOURSE,
            Choice {
                move_id: Choices::COLLISIONCOURSE,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COMBATTORQUE,
            Choice {
                move_id: Choices::COMBATTORQUE,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COMETPUNCH,
            Choice {
                move_id: Choices::COMETPUNCH,
                accuracy: 85.0,
                base_power: 18.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COMEUPPANCE,
            Choice {
                move_id: Choices::COMEUPPANCE,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CONFIDE,
            Choice {
                move_id: Choices::CONFIDE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CONFUSERAY,
            Choice {
                move_id: Choices::CONFUSERAY,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CONFUSION,
            Choice {
                move_id: Choices::CONFUSION,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::CONSTRICT,
                Choice {
                    move_id: Choices::CONSTRICT,
                    base_power: 10.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 33.2,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
            Choices::CONSTRICT,
            Choice {
                move_id: Choices::CONSTRICT,
                base_power: 10.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        }
        moves.insert(
            Choices::CONVERSION,
            Choice {
                move_id: Choices::CONVERSION,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CONVERSION2,
            Choice {
                move_id: Choices::CONVERSION2,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COPYCAT,
            Choice {
                move_id: Choices::COPYCAT,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COREENFORCER,
            Choice {
                move_id: Choices::COREENFORCER,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CORROSIVEGAS,
            Choice {
                move_id: Choices::CORROSIVEGAS,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COSMICPOWER,
            Choice {
                move_id: Choices::COSMICPOWER,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COTTONGUARD,
            Choice {
                move_id: Choices::COTTONGUARD,
                target: MoveTarget::User,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 3,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::COTTONSPORE,
                Choice {
                    move_id: Choices::COTTONSPORE,
                    accuracy: 85.0,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                            powder: true,
                        protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::Opponent,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -2,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::COTTONSPORE,
                Choice {
                    move_id: Choices::COTTONSPORE,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                            powder: true,
                        protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::Opponent,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -2,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::COUNTER,
            Choice {
                move_id: Choices::COUNTER,
                category: MoveCategory::Physical,
                priority: -5,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::COURTCHANGE,
            Choice {
                move_id: Choices::COURTCHANGE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::COVET,
                Choice {
                    move_id: Choices::COVET,
                    base_power: 40.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::COVET,
                Choice {
                    move_id: Choices::COVET,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::CRABHAMMER,
                Choice {
                    move_id: Choices::CRABHAMMER,
                    accuracy: 85.0,
                    base_power: 90.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::CRABHAMMER,
                Choice {
                    move_id: Choices::CRABHAMMER,
                    accuracy: 90.0,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::CRAFTYSHIELD,
            Choice {
                move_id: Choices::CRAFTYSHIELD,
                priority: 3,
                target: MoveTarget::User,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::CraftyShield,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CROSSCHOP,
            Choice {
                move_id: Choices::CROSSCHOP,
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CROSSPOISON,
            Choice {
                move_id: Choices::CROSSPOISON,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::POISON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::CRUNCH,
                Choice {
                    move_id: Choices::CRUNCH,
                    base_power: 80.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        bite: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 20.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        else {
            moves.insert(
                Choices::CRUNCH,
                Choice {
                    move_id: Choices::CRUNCH,
                    base_power: 80.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        bite: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 20.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::CRUSHCLAW,
            Choice {
                move_id: Choices::CRUSHCLAW,
                accuracy: 95.0,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CRUSHGRIP,
            Choice {
                move_id: Choices::CRUSHGRIP,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CURSE,
            Choice {
                move_id: Choices::CURSE,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                }),
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::CURSE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::CUT,
            Choice {
                move_id: Choices::CUT,
                accuracy: 95.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DARKESTLARIAT,
            Choice {
                move_id: Choices::DARKESTLARIAT,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DARKPULSE,
            Choice {
                move_id: Choices::DARKPULSE,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::DARKVOID,
                Choice {
                    move_id: Choices::DARKVOID,
                    accuracy: 80.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::SLEEP,
                    }),
                    move_type: PokemonType::DARK,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DARKVOID,
                Choice {
                    move_id: Choices::DARKVOID,
                    accuracy: 50.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::SLEEP,
                    }),
                    move_type: PokemonType::DARK,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::DAZZLINGGLEAM,
            Choice {
                move_id: Choices::DAZZLINGGLEAM,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DECORATE,
            Choice {
                move_id: Choices::DECORATE,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DEFENDORDER,
            Choice {
                move_id: Choices::DEFENDORDER,
                target: MoveTarget::User,
                move_type: PokemonType::BUG,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DEFENSECURL,
            Choice {
                move_id: Choices::DEFENSECURL,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::DEFENSECURL,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DEFOG,
            Choice {
                move_id: Choices::DEFOG,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DESTINYBOND,
            Choice {
                move_id: Choices::DESTINYBOND,
                target: MoveTarget::User,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::DESTINYBOND,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DETECT,
            Choice {
                move_id: Choices::DETECT,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::PROTECT,
                }),
                ..Default::default()
            },
        );

        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::DIAMONDSTORM,
                Choice {
                    move_id: Choices::DIAMONDSTORM,
                    accuracy: 95.0,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 50.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DIAMONDSTORM,
                Choice {
                    move_id: Choices::DIAMONDSTORM,
                    accuracy: 95.0,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 50.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 2,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::DIG,
                Choice {
                    move_id: Choices::DIG,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GROUND,
                    flags: Flags {
                        charge: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::DIG,
                Choice {
                    move_id: Choices::DIG,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GROUND,
                    flags: Flags {
                        charge: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DIG,
                Choice {
                    move_id: Choices::DIG,
                    base_power: 80.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GROUND,
                    flags: Flags {
                        charge: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::DIRECLAW,
            Choice {
                move_id: Choices::DIRECLAW,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::POISON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::DISABLE,
                Choice {
                    move_id: Choices::DISABLE,
                    accuracy: 80.0,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::DISABLE,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DISABLE,
                Choice {
                    move_id: Choices::DISABLE,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::DISABLE,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::DISARMINGVOICE,
            Choice {
                move_id: Choices::DISARMINGVOICE,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DISCHARGE,
            Choice {
                move_id: Choices::DISCHARGE,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::DIVE,
                Choice {
                    move_id: Choices::DIVE,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                        charge: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DIVE,
                Choice {
                    move_id: Choices::DIVE,
                    base_power: 80.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                        charge: true,
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::DIZZYPUNCH,
                Choice {
                    move_id: Choices::DIZZYPUNCH,
                    base_power: 70.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        punch: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DIZZYPUNCH,
                Choice {
                    move_id: Choices::DIZZYPUNCH,
                    base_power: 70.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        punch: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 20.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::DOODLE,
            Choice {
                move_id: Choices::DOODLE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::DOOMDESIRE,
                Choice {
                    move_id: Choices::DOOMDESIRE,
                    accuracy: 85.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::STEEL,
                    flags: Flags {
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DOOMDESIRE,
                Choice {
                    move_id: Choices::DOOMDESIRE,
                    base_power: 140.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::STEEL,
                    flags: Flags {
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::DOUBLEEDGE,
                Choice {
                    move_id: Choices::DOUBLEEDGE,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    recoil: Some(0.33),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DOUBLEEDGE,
                Choice {
                    move_id: Choices::DOUBLEEDGE,
                    base_power: 120.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    recoil: Some(0.33),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::DOUBLEHIT,
            Choice {
                move_id: Choices::DOUBLEHIT,
                accuracy: 90.0,
                base_power: 35.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DOUBLEIRONBASH,
            Choice {
                move_id: Choices::DOUBLEIRONBASH,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DOUBLEKICK,
            Choice {
                move_id: Choices::DOUBLEKICK,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DOUBLESHOCK,
            Choice {
                move_id: Choices::DOUBLESHOCK,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DOUBLESLAP,
            Choice {
                move_id: Choices::DOUBLESLAP,
                accuracy: 85.0,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DOUBLETEAM,
            Choice {
                move_id: Choices::DOUBLETEAM,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::DRACOMETEOR,
                Choice {
                    move_id: Choices::DRACOMETEOR,
                    accuracy: 90.0,
                    base_power: 140.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::DRAGON,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -2,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DRACOMETEOR,
                Choice {
                    move_id: Choices::DRACOMETEOR,
                    accuracy: 90.0,
                    base_power: 130.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::DRAGON,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -2,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::DRAGONASCENT,
            Choice {
                move_id: Choices::DRAGONASCENT,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONBREATH,
            Choice {
                move_id: Choices::DRAGONBREATH,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONCHEER,
            Choice {
                move_id: Choices::DRAGONCHEER,
                base_power: 60.0,
                category: MoveCategory::Status,
                move_type: PokemonType::DRAGON,
                target: MoveTarget::User,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONCLAW,
            Choice {
                move_id: Choices::DRAGONCLAW,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONDANCE,
            Choice {
                move_id: Choices::DRAGONDANCE,
                target: MoveTarget::User,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONDARTS,
            Choice {
                move_id: Choices::DRAGONDARTS,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONENERGY,
            Choice {
                move_id: Choices::DRAGONENERGY,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONHAMMER,
            Choice {
                move_id: Choices::DRAGONHAMMER,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::DRAGONPULSE,
                Choice {
                    move_id: Choices::DRAGONPULSE,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::DRAGON,
                    flags: Flags {
                                protect: true,
                        pulse: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DRAGONPULSE,
                Choice {
                    move_id: Choices::DRAGONPULSE,
                    base_power: 85.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::DRAGON,
                    flags: Flags {
                                protect: true,
                        pulse: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::DRAGONRAGE,
            Choice {
                move_id: Choices::DRAGONRAGE,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONRUSH,
            Choice {
                move_id: Choices::DRAGONRUSH,
                accuracy: 75.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAGONTAIL,
            Choice {
                move_id: Choices::DRAGONTAIL,
                accuracy: 90.0,
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: -6,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    contact: true,
                    drag: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRAININGKISS,
            Choice {
                move_id: Choices::DRAININGKISS,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    contact: true,
                    heal: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.75),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::DRAINPUNCH,
                Choice {
                    move_id: Choices::DRAINPUNCH,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        heal: true,
                            protect: true,
                        punch: true,
                        ..Default::default()
                    },
                    drain: Some(0.5),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::DRAINPUNCH,
                Choice {
                    move_id: Choices::DRAINPUNCH,
                    base_power: 75.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        heal: true,
                            protect: true,
                        punch: true,
                        ..Default::default()
                    },
                    drain: Some(0.5),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::DREAMEATER,
            Choice {
                move_id: Choices::DREAMEATER,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRILLPECK,
            Choice {
                move_id: Choices::DRILLPECK,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRILLRUN,
            Choice {
                move_id: Choices::DRILLRUN,
                accuracy: 95.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DRUMBEATING,
            Choice {
                move_id: Choices::DRUMBEATING,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DUALCHOP,
            Choice {
                move_id: Choices::DUALCHOP,
                accuracy: 90.0,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DUALWINGBEAT,
            Choice {
                move_id: Choices::DUALWINGBEAT,
                accuracy: 90.0,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DYNAMAXCANNON,
            Choice {
                move_id: Choices::DYNAMAXCANNON,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::DYNAMICPUNCH,
            Choice {
                move_id: Choices::DYNAMICPUNCH,
                accuracy: 50.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EARTHPOWER,
            Choice {
                move_id: Choices::EARTHPOWER,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EARTHQUAKE,
            Choice {
                move_id: Choices::EARTHQUAKE,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ECHOEDVOICE,
            Choice {
                move_id: Choices::ECHOEDVOICE,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EERIEIMPULSE,
            Choice {
                move_id: Choices::EERIEIMPULSE,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EERIESPELL,
            Choice {
                move_id: Choices::EERIESPELL,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EGGBOMB,
            Choice {
                move_id: Choices::EGGBOMB,
                accuracy: 75.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ELECTRICTERRAIN,
            Choice {
                move_id: Choices::ELECTRICTERRAIN,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ELECTRIFY,
            Choice {
                move_id: Choices::ELECTRIFY,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::ELECTRIFY,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ELECTROBALL,
            Choice {
                move_id: Choices::ELECTROBALL,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ELECTRODRIFT,
            Choice {
                move_id: Choices::ELECTRODRIFT,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ELECTROSHOT,
            Choice {
                move_id: Choices::ELECTROSHOT,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ELECTROWEB,
            Choice {
                move_id: Choices::ELECTROWEB,
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EMBARGO,
            Choice {
                move_id: Choices::EMBARGO,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::EMBARGO,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EMBER,
            Choice {
                move_id: Choices::EMBER,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ENCORE,
            Choice {
                move_id: Choices::ENCORE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::ENCORE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ENDEAVOR,
            Choice {
                move_id: Choices::ENDEAVOR,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ENDURE,
            Choice {
                move_id: Choices::ENDURE,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::ENDURE,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::ENERGYBALL,
                Choice {
                    move_id: Choices::ENERGYBALL,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        bullet: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ENERGYBALL,
                Choice {
                    move_id: Choices::ENERGYBALL,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        bullet: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ENTRAINMENT,
            Choice {
                move_id: Choices::ENTRAINMENT,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ERUPTION,
            Choice {
                move_id: Choices::ERUPTION,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ESPERWING,
            Choice {
                move_id: Choices::ESPERWING,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ETERNABEAM,
            Choice {
                move_id: Choices::ETERNABEAM,
                accuracy: 90.0,
                base_power: 160.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EXPANDINGFORCE,
            Choice {
                move_id: Choices::EXPANDINGFORCE,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::EXPLOSION,
                Choice {
                    move_id: Choices::EXPLOSION,
                    base_power: 170.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::EXPLOSION,
                Choice {
                    move_id: Choices::EXPLOSION,
                    base_power: 250.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::EXTRASENSORY,
            Choice {
                move_id: Choices::EXTRASENSORY,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::EXTREMESPEED,
            Choice {
                move_id: Choices::EXTREMESPEED,
                base_power: 80.0,
                category: MoveCategory::Physical,
                priority: 2,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FACADE,
            Choice {
                move_id: Choices::FACADE,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FAIRYLOCK,
            Choice {
                move_id: Choices::FAIRYLOCK,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FAIRYWIND,
            Choice {
                move_id: Choices::FAIRYWIND,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FAKEOUT,
            Choice {
                move_id: Choices::FAKEOUT,
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 3,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FAKETEARS,
            Choice {
                move_id: Choices::FAKETEARS,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -2,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FALSESURRENDER,
            Choice {
                move_id: Choices::FALSESURRENDER,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FALSESWIPE,
            Choice {
                move_id: Choices::FALSESWIPE,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FEATHERDANCE,
            Choice {
                move_id: Choices::FEATHERDANCE,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -2,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::FEINT,
                Choice {
                    move_id: Choices::FEINT,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    priority: 2,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FEINT,
                Choice {
                    move_id: Choices::FEINT,
                    base_power: 30.0,
                    category: MoveCategory::Physical,
                    priority: 2,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FEINTATTACK,
            Choice {
                move_id: Choices::FEINTATTACK,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::FELLSTINGER,
                Choice {
                    move_id: Choices::FELLSTINGER,
                    base_power: 30.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FELLSTINGER,
                Choice {
                    move_id: Choices::FELLSTINGER,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FICKLEBEAM,
            Choice {
                move_id: Choices::FICKLEBEAM,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FIERYDANCE,
            Choice {
                move_id: Choices::FIERYDANCE,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FIERYWRATH,
            Choice {
                move_id: Choices::FIERYWRATH,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FILLETAWAY,
            Choice {
                move_id: Choices::FILLETAWAY,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FINALGAMBIT,
            Choice {
                move_id: Choices::FINALGAMBIT,
                category: MoveCategory::Special,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::FIREBLAST,
                Choice {
                    move_id: Choices::FIREBLAST,
                    accuracy: 85.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::BURN),
                    }]),
                    ..Default::default()
                },
            );
        }
        else if cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::FIREBLAST,
                Choice {
                    move_id: Choices::FIREBLAST,
                    accuracy: 85.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::BURN),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FIREBLAST,
                Choice {
                    move_id: Choices::FIREBLAST,
                    accuracy: 85.0,
                    base_power: 110.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::BURN),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FIREFANG,
            Choice {
                move_id: Choices::FIREFANG,
                accuracy: 95.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::BURN),
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    },
                ]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FIRELASH,
            Choice {
                move_id: Choices::FIRELASH,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::FIREPLEDGE,
                Choice {
                    move_id: Choices::FIREPLEDGE,
                    base_power: 50.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FIREPLEDGE,
                Choice {
                    move_id: Choices::FIREPLEDGE,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FIREPUNCH,
            Choice {
                move_id: Choices::FIREPUNCH,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::FIRESPIN,
                Choice {
                    move_id: Choices::FIRESPIN,
                    accuracy: 70.0,
                    base_power: 15.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FIRESPIN,
                Choice {
                    move_id: Choices::FIRESPIN,
                    accuracy: 85.0,
                    base_power: 35.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FIRSTIMPRESSION,
            Choice {
                move_id: Choices::FIRSTIMPRESSION,
                base_power: 0.0,
                category: MoveCategory::Physical,
                priority: 2,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FISHIOUSREND,
            Choice {
                move_id: Choices::FISHIOUSREND,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FISSURE,
            Choice {
                move_id: Choices::FISSURE,
                accuracy: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLAIL,
            Choice {
                move_id: Choices::FLAIL,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLAMEBURST,
            Choice {
                move_id: Choices::FLAMEBURST,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLAMECHARGE,
            Choice {
                move_id: Choices::FLAMECHARGE,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::FLAMETHROWER,
                Choice {
                    move_id: Choices::FLAMETHROWER,
                    base_power: 95.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::BURN),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FLAMETHROWER,
                Choice {
                    move_id: Choices::FLAMETHROWER,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::BURN),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FLAMEWHEEL,
            Choice {
                move_id: Choices::FLAMEWHEEL,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLAREBLITZ,
            Choice {
                move_id: Choices::FLAREBLITZ,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLASH,
            Choice {
                move_id: Choices::FLASH,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLASHCANNON,
            Choice {
                move_id: Choices::FLASHCANNON,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLATTER,
            Choice {
                move_id: Choices::FLATTER,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLEURCANNON,
            Choice {
                move_id: Choices::FLEURCANNON,
                accuracy: 90.0,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLING,
            Choice {
                move_id: Choices::FLING,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLIPTURN,
            Choice {
                move_id: Choices::FLIPTURN,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    pivot: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLOATYFALL,
            Choice {
                move_id: Choices::FLOATYFALL,
                accuracy: 95.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLORALHEALING,
            Choice {
                move_id: Choices::FLORALHEALING,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    heal: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLOWERSHIELD,
            Choice {
                move_id: Choices::FLOWERSHIELD,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLOWERTRICK,
            Choice {
                move_id: Choices::FLOWERTRICK,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FLY,
            Choice {
                move_id: Choices::FLY,
                accuracy: 95.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    charge: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::FLYINGPRESS,
                Choice {
                    move_id: Choices::FLYINGPRESS,
                    accuracy: 95.0,
                    base_power: 80.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FLYINGPRESS,
                Choice {
                    move_id: Choices::FLYINGPRESS,
                    accuracy: 95.0,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FOCUSBLAST,
            Choice {
                move_id: Choices::FOCUSBLAST,
                accuracy: 70.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FOCUSENERGY,
            Choice {
                move_id: Choices::FOCUSENERGY,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::FOCUSENERGY,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FOCUSPUNCH,
            Choice {
                move_id: Choices::FOCUSPUNCH,
                base_power: 150.0,
                category: MoveCategory::Physical,
                priority: -3,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FOLLOWME,
            Choice {
                move_id: Choices::FOLLOWME,
                priority: 2,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::FOLLOWME,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FORCEPALM,
            Choice {
                move_id: Choices::FORCEPALM,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FORESIGHT,
            Choice {
                move_id: Choices::FORESIGHT,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::FORESIGHT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FORESTSCURSE,
            Choice {
                move_id: Choices::FORESTSCURSE,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FOULPLAY,
            Choice {
                move_id: Choices::FOULPLAY,
                base_power: 95.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FREEZEDRY,
            Choice {
                move_id: Choices::FREEZEDRY,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::FREEZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FREEZESHOCK,
            Choice {
                move_id: Choices::FREEZESHOCK,
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FREEZINGGLARE,
            Choice {
                move_id: Choices::FREEZINGGLARE,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::FREEZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FREEZYFROST,
            Choice {
                move_id: Choices::FREEZYFROST,
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FRENZYPLANT,
            Choice {
                move_id: Choices::FRENZYPLANT,
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::FROSTBREATH,
                Choice {
                    move_id: Choices::FROSTBREATH,
                    accuracy: 90.0,
                    base_power: 40.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FROSTBREATH,
                Choice {
                    move_id: Choices::FROSTBREATH,
                    accuracy: 90.0,
                    base_power: 60.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FRUSTRATION,
            Choice {
                move_id: Choices::FRUSTRATION,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FURYATTACK,
            Choice {
                move_id: Choices::FURYATTACK,
                accuracy: 85.0,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::FURYCUTTER,
                Choice {
                    move_id: Choices::FURYCUTTER,
                    accuracy: 95.0,
                    base_power: 10.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        slicing: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen5") {
            moves.insert(
                Choices::FURYCUTTER,
                Choice {
                    move_id: Choices::FURYCUTTER,
                    accuracy: 95.0,
                    base_power: 20.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FURYCUTTER,
                Choice {
                    move_id: Choices::FURYCUTTER,
                    accuracy: 95.0,
                    base_power: 40.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::FURYSWIPES,
            Choice {
                move_id: Choices::FURYSWIPES,
                accuracy: 80.0,
                base_power: 18.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FUSIONBOLT,
            Choice {
                move_id: Choices::FUSIONBOLT,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::FUSIONFLARE,
            Choice {
                move_id: Choices::FUSIONFLARE,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::FUTURESIGHT,
                Choice {
                    accuracy: 90.0,
                    move_id: Choices::FUTURESIGHT,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen5") {
            moves.insert(
                Choices::FUTURESIGHT,
                Choice {
                    move_id: Choices::FUTURESIGHT,
                    base_power: 100.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::FUTURESIGHT,
                Choice {
                    move_id: Choices::FUTURESIGHT,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::GASTROACID,
            Choice {
                move_id: Choices::GASTROACID,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::GASTROACID,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GEARGRIND,
            Choice {
                move_id: Choices::GEARGRIND,
                accuracy: 85.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GEARUP,
            Choice {
                move_id: Choices::GEARUP,
                target: MoveTarget::User,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GEOMANCY,
            Choice {
                move_id: Choices::GEOMANCY,
                target: MoveTarget::User,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    charge: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 2,
                        speed: 2,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::GIGADRAIN,
                Choice {
                    move_id: Choices::GIGADRAIN,
                    base_power: 60.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        heal: true,
                            protect: true,
                        ..Default::default()
                    },
                    drain: Some(0.5),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::GIGADRAIN,
                Choice {
                    move_id: Choices::GIGADRAIN,
                    base_power: 75.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        heal: true,
                            protect: true,
                        ..Default::default()
                    },
                    drain: Some(0.5),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::GIGAIMPACT,
            Choice {
                move_id: Choices::GIGAIMPACT,
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GIGATONHAMMER,
            Choice {
                move_id: Choices::GIGATONHAMMER,
                base_power: 160.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        if cfg!(feature = "gen9") {
            moves.insert(
                Choices::GLACIALLANCE,
                Choice {
                    move_id: Choices::GLACIALLANCE,
                    base_power: 120.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::GLACIALLANCE,
                Choice {
                    move_id: Choices::GLACIALLANCE,
                    base_power: 130.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::GLACIATE,
            Choice {
                move_id: Choices::GLACIATE,
                accuracy: 95.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GLAIVERUSH,
            Choice {
                move_id: Choices::GLAIVERUSH,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::GLARE,
                Choice {
                    accuracy: 75.0,
                    move_id: Choices::GLARE,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::PARALYZE,
                    }),
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen5") {
            moves.insert(
                Choices::GLARE,
                Choice {
                    accuracy: 90.0,
                    move_id: Choices::GLARE,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::PARALYZE,
                    }),
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::GLARE,
                Choice {
                    move_id: Choices::GLARE,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::PARALYZE,
                    }),
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::GLITZYGLOW,
            Choice {
                move_id: Choices::GLITZYGLOW,
                accuracy: 95.0,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GRASSKNOT,
            Choice {
                move_id: Choices::GRASSKNOT,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::GRASSPLEDGE,
                Choice {
                    move_id: Choices::GRASSPLEDGE,
                    base_power: 50.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::GRASSPLEDGE,
                Choice {
                    move_id: Choices::GRASSPLEDGE,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::GRASSWHISTLE,
            Choice {
                move_id: Choices::GRASSWHISTLE,
                accuracy: 55.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::SLEEP,
                }),
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        if cfg!(feature = "gen9") {
            moves.insert(
                Choices::GRASSYGLIDE,
                Choice {
                    move_id: Choices::GRASSYGLIDE,
                    base_power: 55.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::GRASSYGLIDE,
                Choice {
                    move_id: Choices::GRASSYGLIDE,
                    base_power: 70.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::GRASSYTERRAIN,
            Choice {
                move_id: Choices::GRASSYTERRAIN,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GRAVAPPLE,
            Choice {
                move_id: Choices::GRAVAPPLE,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GRAVITY,
            Choice {
                move_id: Choices::GRAVITY,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GROWL,
            Choice {
                move_id: Choices::GROWL,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::GROWTH,
                Choice {
                    move_id: Choices::GROWTH,
                    target: MoveTarget::User,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::GROWTH,
                Choice {
                    move_id: Choices::GROWTH,
                    target: MoveTarget::User,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 1,
                            defense: 0,
                            special_attack: 1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::GRUDGE,
            Choice {
                move_id: Choices::GRUDGE,
                target: MoveTarget::User,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::GRUDGE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GUARDSPLIT,
            Choice {
                move_id: Choices::GUARDSPLIT,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GUARDSWAP,
            Choice {
                move_id: Choices::GUARDSWAP,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::GUILLOTINE,
            Choice {
                move_id: Choices::GUILLOTINE,
                accuracy: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::GUNKSHOT,
                Choice {
                    move_id: Choices::GUNKSHOT,
                    accuracy: 70.0,
                    base_power: 120.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::POISON),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::GUNKSHOT,
                Choice {
                    move_id: Choices::GUNKSHOT,
                    accuracy: 80.0,
                    base_power: 120.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::POISON),
                    }]),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::GUST,
                Choice {
                    move_id: Choices::GUST,
                    base_power: 40.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        protect: true,
                        wind: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::GUST,
                Choice {
                    move_id: Choices::GUST,
                    base_power: 40.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                        protect: true,
                        wind: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::GYROBALL,
            Choice {
                move_id: Choices::GYROBALL,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    bullet: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HAIL,
            Choice {
                move_id: Choices::HAIL,
                move_type: PokemonType::ICE,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HAMMERARM,
            Choice {
                move_id: Choices::HAMMERARM,
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HAPPYHOUR,
            Choice {
                move_id: Choices::HAPPYHOUR,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HARDEN,
            Choice {
                move_id: Choices::HARDEN,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HARDPRESS,
            Choice {
                move_id: Choices::HARDPRESS,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HAZE,
            Choice {
                move_id: Choices::HAZE,
                move_type: PokemonType::ICE,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEADBUTT,
            Choice {
                move_id: Choices::HEADBUTT,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEADCHARGE,
            Choice {
                move_id: Choices::HEADCHARGE,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.25),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEADLONGRUSH,
            Choice {
                move_id: Choices::HEADLONGRUSH,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEADSMASH,
            Choice {
                move_id: Choices::HEADSMASH,
                accuracy: 80.0,
                base_power: 150.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEALBELL,
            Choice {
                move_id: Choices::HEALBELL,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEALBLOCK,
            Choice {
                move_id: Choices::HEALBLOCK,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::HEALBLOCK,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEALINGWISH,
            Choice {
                move_id: Choices::HEALINGWISH,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::HealingWish,
                }),
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -1.0,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEALORDER,
            Choice {
                move_id: Choices::HEALORDER,
                target: MoveTarget::User,
                move_type: PokemonType::BUG,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEALPULSE,
            Choice {
                move_id: Choices::HEALPULSE,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    heal: true,
                    protect: true,
                    pulse: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEARTSTAMP,
            Choice {
                move_id: Choices::HEARTSTAMP,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEARTSWAP,
            Choice {
                move_id: Choices::HEARTSWAP,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HEATCRASH,
            Choice {
                move_id: Choices::HEATCRASH,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::HEATWAVE,
                Choice {
                    move_id: Choices::HEATWAVE,
                    accuracy: 90.0,
                    base_power: 100.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                        protect: true,
                        wind: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::BURN),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::HEATWAVE,
                Choice {
                    move_id: Choices::HEATWAVE,
                    accuracy: 90.0,
                    base_power: 95.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                        protect: true,
                        wind: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::BURN),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::HEAVYSLAM,
            Choice {
                move_id: Choices::HEAVYSLAM,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HELPINGHAND,
            Choice {
                move_id: Choices::HELPINGHAND,
                priority: 5,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::HELPINGHAND,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::HEX,
                Choice {
                    move_id: Choices::HEX,
                    base_power: 50.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GHOST,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::HEX,
                Choice {
                    move_id: Choices::HEX,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GHOST,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::HIDDENPOWER,
            Choice {
                move_id: Choices::HIDDENPOWER,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERBUG60,
            Choice {
                move_id: Choices::HIDDENPOWERBUG60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERBUG70,
            Choice {
                move_id: Choices::HIDDENPOWERBUG70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERDARK60,
            Choice {
                move_id: Choices::HIDDENPOWERDARK60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERDARK70,
            Choice {
                move_id: Choices::HIDDENPOWERDARK70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERDRAGON60,
            Choice {
                move_id: Choices::HIDDENPOWERDRAGON60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERDRAGON70,
            Choice {
                move_id: Choices::HIDDENPOWERDRAGON70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERELECTRIC60,
            Choice {
                move_id: Choices::HIDDENPOWERELECTRIC60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERELECTRIC70,
            Choice {
                move_id: Choices::HIDDENPOWERELECTRIC70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERFIGHTING60,
            Choice {
                move_id: Choices::HIDDENPOWERFIGHTING60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERFIGHTING70,
            Choice {
                move_id: Choices::HIDDENPOWERFIGHTING70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERFIRE60,
            Choice {
                move_id: Choices::HIDDENPOWERFIRE60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERFIRE70,
            Choice {
                move_id: Choices::HIDDENPOWERFIRE70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERFLYING60,
            Choice {
                move_id: Choices::HIDDENPOWERFLYING60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERFLYING70,
            Choice {
                move_id: Choices::HIDDENPOWERFLYING70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERGHOST60,
            Choice {
                move_id: Choices::HIDDENPOWERGHOST60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERGHOST70,
            Choice {
                move_id: Choices::HIDDENPOWERGHOST70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERGRASS60,
            Choice {
                move_id: Choices::HIDDENPOWERGRASS60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERGRASS70,
            Choice {
                move_id: Choices::HIDDENPOWERGRASS70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERGROUND60,
            Choice {
                move_id: Choices::HIDDENPOWERGROUND60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERGROUND70,
            Choice {
                move_id: Choices::HIDDENPOWERGROUND70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERICE60,
            Choice {
                move_id: Choices::HIDDENPOWERICE60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERICE70,
            Choice {
                move_id: Choices::HIDDENPOWERICE70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERPOISON60,
            Choice {
                move_id: Choices::HIDDENPOWERPOISON60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERPOISON70,
            Choice {
                move_id: Choices::HIDDENPOWERPOISON70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERPSYCHIC60,
            Choice {
                move_id: Choices::HIDDENPOWERPSYCHIC60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERPSYCHIC70,
            Choice {
                move_id: Choices::HIDDENPOWERPSYCHIC70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERROCK60,
            Choice {
                move_id: Choices::HIDDENPOWERROCK60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERROCK70,
            Choice {
                move_id: Choices::HIDDENPOWERROCK70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERSTEEL60,
            Choice {
                move_id: Choices::HIDDENPOWERSTEEL60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERSTEEL70,
            Choice {
                move_id: Choices::HIDDENPOWERSTEEL70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERWATER60,
            Choice {
                move_id: Choices::HIDDENPOWERWATER60,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIDDENPOWERWATER70,
            Choice {
                move_id: Choices::HIDDENPOWERWATER70,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HIGHHORSEPOWER,
            Choice {
                move_id: Choices::HIGHHORSEPOWER,
                accuracy: 95.0,
                base_power: 95.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::HIGHJUMPKICK,
                Choice {
                    move_id: Choices::HIGHJUMPKICK,
                    accuracy: 90.0,
                    base_power: 85.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    crash: Some(0.5),
                    ..Default::default()
                },
            );
        }
        else if cfg!(feature = "gen4") {
            moves.insert(
                Choices::HIGHJUMPKICK,
                Choice {
                    move_id: Choices::HIGHJUMPKICK,
                    accuracy: 90.0,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    crash: Some(0.5),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::HIGHJUMPKICK,
                Choice {
                    move_id: Choices::HIGHJUMPKICK,
                    accuracy: 90.0,
                    base_power: 130.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    crash: Some(0.5),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::HOLDBACK,
            Choice {
                move_id: Choices::HOLDBACK,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HOLDHANDS,
            Choice {
                move_id: Choices::HOLDHANDS,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HONECLAWS,
            Choice {
                move_id: Choices::HONECLAWS,
                target: MoveTarget::User,
                move_type: PokemonType::DARK,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 1,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HORNATTACK,
            Choice {
                move_id: Choices::HORNATTACK,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HORNDRILL,
            Choice {
                move_id: Choices::HORNDRILL,
                accuracy: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HORNLEECH,
            Choice {
                move_id: Choices::HORNLEECH,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    heal: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HOWL,
            Choice {
                move_id: Choices::HOWL,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    sound: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::HURRICANE,
                Choice {
                    move_id: Choices::HURRICANE,
                    accuracy: 70.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                        protect: true,
                        wind: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::HURRICANE,
                Choice {
                    move_id: Choices::HURRICANE,
                    accuracy: 70.0,
                    base_power: 110.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                        protect: true,
                        wind: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::HYDROCANNON,
            Choice {
                move_id: Choices::HYDROCANNON,
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::HYDROPUMP,
                Choice {
                    move_id: Choices::HYDROPUMP,
                    accuracy: 80.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::HYDROPUMP,
                Choice {
                    move_id: Choices::HYDROPUMP,
                    accuracy: 80.0,
                    base_power: 110.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::HYDROSTEAM,
            Choice {
                move_id: Choices::HYDROSTEAM,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HYPERBEAM,
            Choice {
                move_id: Choices::HYPERBEAM,
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HYPERDRILL,
            Choice {
                move_id: Choices::HYPERDRILL,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HYPERFANG,
            Choice {
                move_id: Choices::HYPERFANG,
                accuracy: 90.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HYPERSPACEFURY,
            Choice {
                move_id: Choices::HYPERSPACEFURY,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HYPERSPACEHOLE,
            Choice {
                move_id: Choices::HYPERSPACEHOLE,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HYPERVOICE,
            Choice {
                move_id: Choices::HYPERVOICE,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::HYPNOSIS,
            Choice {
                move_id: Choices::HYPNOSIS,
                accuracy: 60.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::SLEEP,
                }),
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ICEBALL,
            Choice {
                move_id: Choices::ICEBALL,
                accuracy: 90.0,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    bullet: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::ICEBEAM,
                Choice {
                    move_id: Choices::ICEBEAM,
                    base_power: 95.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::FREEZE),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ICEBEAM,
                Choice {
                    move_id: Choices::ICEBEAM,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::FREEZE),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ICEBURN,
            Choice {
                move_id: Choices::ICEBURN,
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ICEFANG,
            Choice {
                move_id: Choices::ICEFANG,
                accuracy: 95.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::FREEZE),
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    },
                ]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ICEHAMMER,
            Choice {
                move_id: Choices::ICEHAMMER,
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ICEPUNCH,
            Choice {
                move_id: Choices::ICEPUNCH,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::FREEZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ICESHARD,
            Choice {
                move_id: Choices::ICESHARD,
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ICESPINNER,
            Choice {
                move_id: Choices::ICESPINNER,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ICICLECRASH,
            Choice {
                move_id: Choices::ICICLECRASH,
                accuracy: 90.0,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::ICICLESPEAR,
                Choice {
                    move_id: Choices::ICICLESPEAR,
                    base_power: 10.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ICICLESPEAR,
                Choice {
                    move_id: Choices::ICICLESPEAR,
                    base_power: 25.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ICE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ICYWIND,
            Choice {
                move_id: Choices::ICYWIND,
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::IMPRISON,
            Choice {
                move_id: Choices::IMPRISON,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::IMPRISON,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::INCINERATE,
                Choice {
                    move_id: Choices::INCINERATE,
                    base_power: 30.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::INCINERATE,
                Choice {
                    move_id: Choices::INCINERATE,
                    base_power: 60.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::INFERNALPARADE,
            Choice {
                move_id: Choices::INFERNALPARADE,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::INFERNO,
            Choice {
                move_id: Choices::INFERNO,
                accuracy: 50.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::INFESTATION,
            Choice {
                move_id: Choices::INFESTATION,
                base_power: 20.0,
                category: MoveCategory::Special,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::INGRAIN,
            Choice {
                move_id: Choices::INGRAIN,
                target: MoveTarget::User,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::INGRAIN,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::INSTRUCT,
            Choice {
                move_id: Choices::INSTRUCT,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::IONDELUGE,
            Choice {
                move_id: Choices::IONDELUGE,
                priority: 1,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::IRONDEFENSE,
            Choice {
                move_id: Choices::IRONDEFENSE,
                target: MoveTarget::User,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::IRONHEAD,
            Choice {
                move_id: Choices::IRONHEAD,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::IRONTAIL,
            Choice {
                move_id: Choices::IRONTAIL,
                accuracy: 75.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::IVYCUDGEL,
            Choice {
                move_id: Choices::IVYCUDGEL,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::JAWLOCK,
            Choice {
                move_id: Choices::JAWLOCK,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::JETPUNCH,
            Choice {
                move_id: Choices::JETPUNCH,
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::JUDGMENT,
            Choice {
                move_id: Choices::JUDGMENT,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::JUMPKICK,
                Choice {
                    move_id: Choices::JUMPKICK,
                    accuracy: 95.0,
                    base_power: 70.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                                protect: true,
                        ..Default::default()
                    },
                    crash: Some(0.5),
                    ..Default::default()
                },
            );
        }
        else if cfg!(feature = "gen4") {
            moves.insert(
                Choices::JUMPKICK,
                Choice {
                    move_id: Choices::JUMPKICK,
                    accuracy: 95.0,
                    base_power: 85.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                                protect: true,
                        ..Default::default()
                    },
                    crash: Some(0.5),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::JUMPKICK,
                Choice {
                    move_id: Choices::JUMPKICK,
                    accuracy: 95.0,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                                protect: true,
                        ..Default::default()
                    },
                    crash: Some(0.5),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::JUNGLEHEALING,
            Choice {
                move_id: Choices::JUNGLEHEALING,
                target: MoveTarget::User,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.25,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::KARATECHOP,
                Choice {
                    move_id: Choices::KARATECHOP,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::KARATECHOP,
                Choice {
                    move_id: Choices::KARATECHOP,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::KINESIS,
            Choice {
                move_id: Choices::KINESIS,
                accuracy: 80.0,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::KINGSSHIELD,
            Choice {
                move_id: Choices::KINGSSHIELD,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::KINGSSHIELD,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::KNOCKOFF,
                Choice {
                    move_id: Choices::KNOCKOFF,
                    base_power: 20.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::KNOCKOFF,
                Choice {
                    move_id: Choices::KNOCKOFF,
                    base_power: 65.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::KOWTOWCLEAVE,
            Choice {
                move_id: Choices::KOWTOWCLEAVE,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LANDSWRATH,
            Choice {
                move_id: Choices::LANDSWRATH,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LASERFOCUS,
            Choice {
                move_id: Choices::LASERFOCUS,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::LASERFOCUS,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LASHOUT,
            Choice {
                move_id: Choices::LASHOUT,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::LASTRESORT,
                Choice {
                    move_id: Choices::LASTRESORT,
                    base_power: 130.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::LASTRESORT,
                Choice {
                    move_id: Choices::LASTRESORT,
                    base_power: 140.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::LASTRESPECTS,
            Choice {
                move_id: Choices::LASTRESPECTS,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LAVAPLUME,
            Choice {
                move_id: Choices::LAVAPLUME,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LEAFAGE,
            Choice {
                move_id: Choices::LEAFAGE,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::LEAFBLADE,
                Choice {
                    move_id: Choices::LEAFBLADE,
                    base_power: 70.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        slicing: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::LEAFBLADE,
                Choice {
                    move_id: Choices::LEAFBLADE,
                    base_power: 90.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        slicing: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::LEAFSTORM,
                Choice {
                    move_id: Choices::LEAFSTORM,
                    accuracy: 90.0,
                    base_power: 140.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -2,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::LEAFSTORM,
                Choice {
                    move_id: Choices::LEAFSTORM,
                    accuracy: 90.0,
                    base_power: 130.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -2,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::LEAFTORNADO,
            Choice {
                move_id: Choices::LEAFTORNADO,
                accuracy: 90.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::LEECHLIFE,
                Choice {
                    move_id: Choices::LEECHLIFE,
                    base_power: 20.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                        contact: true,
                        heal: true,
                            protect: true,
                        ..Default::default()
                    },
                    drain: Some(0.5),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::LEECHLIFE,
                Choice {
                    move_id: Choices::LEECHLIFE,
                    base_power: 80.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                        contact: true,
                        heal: true,
                            protect: true,
                        ..Default::default()
                    },
                    drain: Some(0.5),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::LEECHSEED,
            Choice {
                move_id: Choices::LEECHSEED,
                accuracy: 90.0,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::LEECHSEED,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LEER,
            Choice {
                move_id: Choices::LEER,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::LICK,
                Choice {
                    move_id: Choices::LICK,
                    base_power: 20.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GHOST,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::LICK,
                Choice {
                    move_id: Choices::LICK,
                    base_power: 30.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GHOST,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::LIFEDEW,
            Choice {
                move_id: Choices::LIFEDEW,
                target: MoveTarget::User,
                move_type: PokemonType::WATER,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.25,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LIGHTOFRUIN,
            Choice {
                move_id: Choices::LIGHTOFRUIN,
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.5),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::LIGHTSCREEN,
                Choice {
                    move_id: Choices::LIGHTSCREEN,
                    target: MoveTarget::User,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::User,
                        volatile_status: PokemonVolatileStatus::LIGHTSCREEN,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::LIGHTSCREEN,
                Choice {
                    move_id: Choices::LIGHTSCREEN,
                    target: MoveTarget::User,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    side_condition: Some(SideCondition {
                        target: MoveTarget::User,
                        condition: PokemonSideCondition::LightScreen,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::LIQUIDATION,
            Choice {
                move_id: Choices::LIQUIDATION,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LOCKON,
            Choice {
                move_id: Choices::LOCKON,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LOVELYKISS,
            Choice {
                move_id: Choices::LOVELYKISS,
                accuracy: 75.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::SLEEP,
                }),
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") {
            moves.insert(
                Choices::LOWKICK,
                Choice {
                    move_id: Choices::LOWKICK,
                    category: MoveCategory::Physical,
                    base_power: 50.0,
                    accuracy: 90.0,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::LOWKICK,
                Choice {
                    move_id: Choices::LOWKICK,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::LOWSWEEP,
                Choice {
                    move_id: Choices::LOWSWEEP,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::LOWSWEEP,
                Choice {
                    move_id: Choices::LOWSWEEP,
                    base_power: 65.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::LUCKYCHANT,
            Choice {
                move_id: Choices::LUCKYCHANT,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::LuckyChant,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LUMINACRASH,
            Choice {
                move_id: Choices::LUMINACRASH,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -2,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LUNARBLESSING,
            Choice {
                move_id: Choices::LUNARBLESSING,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.25,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LUNARDANCE,
            Choice {
                move_id: Choices::LUNARDANCE,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::LUNGE,
            Choice {
                move_id: Choices::LUNGE,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen9") {
            moves.insert(
            Choices::LUSTERPURGE,
            Choice {
                move_id: Choices::LUSTERPURGE,
                base_power: 95.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        } else {
            moves.insert(
                Choices::LUSTERPURGE,
                Choice {
                    move_id: Choices::LUSTERPURGE,
                    base_power: 70.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 50.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::MACHPUNCH,
            Choice {
                move_id: Choices::MACHPUNCH,
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAGICALLEAF,
            Choice {
                move_id: Choices::MAGICALLEAF,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAGICALTORQUE,
            Choice {
                move_id: Choices::MAGICALTORQUE,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAGICCOAT,
            Choice {
                move_id: Choices::MAGICCOAT,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::MAGICCOAT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAGICPOWDER,
            Choice {
                move_id: Choices::MAGICPOWDER,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAGICROOM,
            Choice {
                move_id: Choices::MAGICROOM,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::MAGMASTORM,
                Choice {
                    move_id: Choices::MAGMASTORM,
                    accuracy: 70.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen5") {
            moves.insert(
                Choices::MAGMASTORM,
                Choice {
                    move_id: Choices::MAGMASTORM,
                    accuracy: 75.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::MAGMASTORM,
                Choice {
                    move_id: Choices::MAGMASTORM,
                    accuracy: 75.0,
                    base_power: 100.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::MAGNETBOMB,
            Choice {
                move_id: Choices::MAGNETBOMB,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAGNETICFLUX,
            Choice {
                move_id: Choices::MAGNETICFLUX,
                target: MoveTarget::User,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAGNETRISE,
            Choice {
                move_id: Choices::MAGNETRISE,
                target: MoveTarget::User,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::MAGNETRISE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAGNITUDE,
            Choice {
                move_id: Choices::MAGNITUDE,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MAKEITRAIN,
            Choice {
                move_id: Choices::MAKEITRAIN,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MALIGNANTCHAIN,
            Choice {
                move_id: Choices::MALIGNANTCHAIN,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::TOXIC),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MATBLOCK,
            Choice {
                move_id: Choices::MATBLOCK,
                target: MoveTarget::User,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::MatBlock,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MATCHAGOTCHA,
            Choice {
                move_id: Choices::MATCHAGOTCHA,
                accuracy: 90.0,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    heal: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MEANLOOK,
            Choice {
                move_id: Choices::MEANLOOK,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MEDITATE,
            Choice {
                move_id: Choices::MEDITATE,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MEFIRST,
            Choice {
                move_id: Choices::MEFIRST,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MEGADRAIN,
            Choice {
                move_id: Choices::MEGADRAIN,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MEGAHORN,
            Choice {
                move_id: Choices::MEGAHORN,
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MEGAKICK,
            Choice {
                move_id: Choices::MEGAKICK,
                accuracy: 75.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MEGAPUNCH,
            Choice {
                move_id: Choices::MEGAPUNCH,
                accuracy: 85.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MEMENTO,
            Choice {
                move_id: Choices::MEMENTO,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -2,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -1.0,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::METALBURST,
            Choice {
                move_id: Choices::METALBURST,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::METALCLAW,
            Choice {
                move_id: Choices::METALCLAW,
                accuracy: 95.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::METALSOUND,
            Choice {
                move_id: Choices::METALSOUND,
                accuracy: 85.0,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -2,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::METEORASSAULT,
            Choice {
                move_id: Choices::METEORASSAULT,
                base_power: 150.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::METEORBEAM,
            Choice {
                move_id: Choices::METEORBEAM,
                accuracy: 90.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::METEORMASH,
                Choice {
                    move_id: Choices::METEORMASH,
                    accuracy: 85.0,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::STEEL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        punch: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 20.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::METEORMASH,
                Choice {
                    move_id: Choices::METEORMASH,
                    accuracy: 90.0,
                    base_power: 90.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::STEEL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        punch: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 20.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::METRONOME,
            Choice {
                move_id: Choices::METRONOME,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MIGHTYCLEAVE,
            Choice {
                move_id: Choices::MIGHTYCLEAVE,
                base_power: 95.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    contact: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MILKDRINK,
            Choice {
                move_id: Choices::MILKDRINK,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MIMIC,
            Choice {
                move_id: Choices::MIMIC,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MINDBLOWN,
            Choice {
                move_id: Choices::MINDBLOWN,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MINDREADER,
            Choice {
                move_id: Choices::MINDREADER,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MINIMIZE,
            Choice {
                move_id: Choices::MINIMIZE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::MINIMIZE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MIRACLEEYE,
            Choice {
                move_id: Choices::MIRACLEEYE,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::MIRACLEEYE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MIRRORCOAT,
            Choice {
                move_id: Choices::MIRRORCOAT,
                category: MoveCategory::Special,
                priority: -5,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MIRRORMOVE,
            Choice {
                move_id: Choices::MIRRORMOVE,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MIRRORSHOT,
            Choice {
                move_id: Choices::MIRRORSHOT,
                accuracy: 85.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MIST,
            Choice {
                move_id: Choices::MIST,
                target: MoveTarget::User,
                move_type: PokemonType::ICE,
                flags: Flags {
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Mist,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen9") {
            moves.insert(
                Choices::MISTBALL,
                Choice {
                    move_id: Choices::MISTBALL,
                    base_power: 95.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        bullet: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 50.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::MISTBALL,
                Choice {
                    move_id: Choices::MISTBALL,
                    base_power: 70.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        bullet: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 50.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::MISTYEXPLOSION,
            Choice {
                move_id: Choices::MISTYEXPLOSION,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MISTYTERRAIN,
            Choice {
                move_id: Choices::MISTYTERRAIN,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MOONBLAST,
            Choice {
                move_id: Choices::MOONBLAST,
                base_power: 95.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MOONGEISTBEAM,
            Choice {
                move_id: Choices::MOONGEISTBEAM,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MOONLIGHT,
            Choice {
                move_id: Choices::MOONLIGHT,
                target: MoveTarget::User,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MORNINGSUN,
            Choice {
                move_id: Choices::MORNINGSUN,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MORTALSPIN,
            Choice {
                move_id: Choices::MORTALSPIN,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::POISON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MOUNTAINGALE,
            Choice {
                move_id: Choices::MOUNTAINGALE,
                accuracy: 85.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MUDBOMB,
            Choice {
                move_id: Choices::MUDBOMB,
                accuracy: 85.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::MUDDYWATER,
                Choice {
                    move_id: Choices::MUDDYWATER,
                    accuracy: 85.0,
                    base_power: 95.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::MUDDYWATER,
                Choice {
                    move_id: Choices::MUDDYWATER,
                    accuracy: 85.0,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::MUDSHOT,
            Choice {
                move_id: Choices::MUDSHOT,
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MUDSLAP,
            Choice {
                move_id: Choices::MUDSLAP,
                base_power: 20.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::MUDSPORT,
            Choice {
                move_id: Choices::MUDSPORT,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen9") || cfg!(feature = "gen8") {
            moves.insert(
                Choices::MULTIATTACK,
                Choice {
                    move_id: Choices::MULTIATTACK,
                    base_power: 120.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::MULTIATTACK,
                Choice {
                    move_id: Choices::MULTIATTACK,
                    base_power: 90.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::MYSTICALFIRE,
                Choice {
                    move_id: Choices::MYSTICALFIRE,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::MYSTICALFIRE,
                Choice {
                    move_id: Choices::MYSTICALFIRE,
                    base_power: 75.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::MYSTICALPOWER,
            Choice {
                move_id: Choices::MYSTICALPOWER,
                accuracy: 90.0,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NASTYPLOT,
            Choice {
                move_id: Choices::NASTYPLOT,
                target: MoveTarget::User,
                move_type: PokemonType::DARK,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NATURALGIFT,
            Choice {
                move_id: Choices::NATURALGIFT,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NATUREPOWER,
            Choice {
                move_id: Choices::NATUREPOWER,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NATURESMADNESS,
            Choice {
                move_id: Choices::NATURESMADNESS,
                accuracy: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NEEDLEARM,
            Choice {
                move_id: Choices::NEEDLEARM,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NIGHTDAZE,
            Choice {
                move_id: Choices::NIGHTDAZE,
                accuracy: 95.0,
                base_power: 85.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 40.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NIGHTMARE,
            Choice {
                move_id: Choices::NIGHTMARE,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::NIGHTMARE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NIGHTSHADE,
            Choice {
                move_id: Choices::NIGHTSHADE,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NIGHTSLASH,
            Choice {
                move_id: Choices::NIGHTSLASH,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NOBLEROAR,
            Choice {
                move_id: Choices::NOBLEROAR,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NORETREAT,
            Choice {
                move_id: Choices::NORETREAT,
                target: MoveTarget::User,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 1,
                        accuracy: 0,
                    },
                }),
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::NORETREAT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NOTHING,
            Choice {
                move_id: Choices::NOTHING,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NOXIOUSTORQUE,
            Choice {
                move_id: Choices::NOXIOUSTORQUE,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::NUZZLE,
            Choice {
                move_id: Choices::NUZZLE,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::OBLIVIONWING,
            Choice {
                move_id: Choices::OBLIVIONWING,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.75),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::OBSTRUCT,
            Choice {
                move_id: Choices::OBSTRUCT,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::DARK,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::PROTECT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::OCTAZOOKA,
            Choice {
                move_id: Choices::OCTAZOOKA,
                accuracy: 85.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::OCTOLOCK,
            Choice {
                move_id: Choices::OCTOLOCK,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::OCTOLOCK,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ODORSLEUTH,
            Choice {
                move_id: Choices::ODORSLEUTH,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::FORESIGHT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::OMINOUSWIND,
            Choice {
                move_id: Choices::OMINOUSWIND,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ORDERUP,
            Choice {
                move_id: Choices::ORDERUP,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ORIGINPULSE,
            Choice {
                move_id: Choices::ORIGINPULSE,
                accuracy: 85.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::OUTRAGE,
                Choice {
                    move_id: Choices::OUTRAGE,
                    base_power: 90.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DRAGON,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::User,
                        volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::OUTRAGE,
                Choice {
                    move_id: Choices::OUTRAGE,
                    base_power: 120.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DRAGON,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::User,
                        volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::OVERDRIVE,
            Choice {
                move_id: Choices::OVERDRIVE,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::OVERHEAT,
                Choice {
                    move_id: Choices::OVERHEAT,
                    accuracy: 90.0,
                    base_power: 140.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::OVERHEAT,
                Choice {
                    move_id: Choices::OVERHEAT,
                    accuracy: 90.0,
                    base_power: 130.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -2,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::PAINSPLIT,
            Choice {
                move_id: Choices::PAINSPLIT,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PALEOWAVE,
            Choice {
                move_id: Choices::PALEOWAVE,
                base_power: 85.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::PARABOLICCHARGE,
                Choice {
                    move_id: Choices::PARABOLICCHARGE,
                    base_power: 50.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                        heal: true,
                            protect: true,
                        ..Default::default()
                    },
                    drain: Some(0.5),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::PARABOLICCHARGE,
                Choice {
                    move_id: Choices::PARABOLICCHARGE,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                        heal: true,
                            protect: true,
                        ..Default::default()
                    },
                    drain: Some(0.5),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::PARTINGSHOT,
            Choice {
                move_id: Choices::PARTINGSHOT,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    pivot: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PAYBACK,
            Choice {
                move_id: Choices::PAYBACK,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PAYDAY,
            Choice {
                move_id: Choices::PAYDAY,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PECK,
            Choice {
                move_id: Choices::PECK,
                base_power: 35.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PERISHSONG,
            Choice {
                move_id: Choices::PERISHSONG,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PETALBLIZZARD,
            Choice {
                move_id: Choices::PETALBLIZZARD,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::PETALDANCE,
                Choice {
                    move_id: Choices::PETALDANCE,
                    base_power: 70.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::User,
                        volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                    }),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen4") {
            moves.insert(
                Choices::PETALDANCE,
                Choice {
                    move_id: Choices::PETALDANCE,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::User,
                        volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::PETALDANCE,
                Choice {
                    move_id: Choices::PETALDANCE,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                                protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::User,
                        volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::PHANTOMFORCE,
            Choice {
                move_id: Choices::PHANTOMFORCE,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    charge: true,
                    contact: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PHOTONGEYSER,
            Choice {
                move_id: Choices::PHOTONGEYSER,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PIKAPAPOW,
            Choice {
                move_id: Choices::PIKAPAPOW,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::PINMISSILE,
                Choice {
                    move_id: Choices::PINMISSILE,
                    accuracy: 85.0,
                    base_power: 14.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::PINMISSILE,
                Choice {
                    move_id: Choices::PINMISSILE,
                    accuracy: 95.0,
                    base_power: 25.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::PLASMAFISTS,
            Choice {
                move_id: Choices::PLASMAFISTS,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PLAYNICE,
            Choice {
                move_id: Choices::PLAYNICE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PLAYROUGH,
            Choice {
                move_id: Choices::PLAYROUGH,
                accuracy: 90.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PLUCK,
            Choice {
                move_id: Choices::PLUCK,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::POISONFANG,
                Choice {
                    move_id: Choices::POISONFANG,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        bite: true,
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::TOXIC),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::POISONFANG,
                Choice {
                    move_id: Choices::POISONFANG,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        bite: true,
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 50.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::TOXIC),
                    }]),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::POISONGAS,
                Choice {
                    move_id: Choices::POISONGAS,
                    accuracy: 55.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::POISON,
                    }),
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen5") {
            moves.insert(
                Choices::POISONGAS,
                Choice {
                    move_id: Choices::POISONGAS,
                    accuracy: 80.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::POISON,
                    }),
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::POISONGAS,
                Choice {
                    move_id: Choices::POISONGAS,
                    accuracy: 90.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::POISON,
                    }),
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::POISONJAB,
            Choice {
                move_id: Choices::POISONJAB,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::POISON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POISONPOWDER,
            Choice {
                move_id: Choices::POISONPOWDER,
                accuracy: 75.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::POISON,
                }),
                move_type: PokemonType::POISON,
                flags: Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::POISONSTING,
                Choice {
                    move_id: Choices::POISONSTING,
                    base_power: 15.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 20.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::POISON),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::POISONSTING,
                Choice {
                    move_id: Choices::POISONSTING,
                    base_power: 15.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::POISON),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::POISONTAIL,
            Choice {
                move_id: Choices::POISONTAIL,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::POISON,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POLLENPUFF,
            Choice {
                move_id: Choices::POLLENPUFF,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::BUG,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POLTERGEIST,
            Choice {
                move_id: Choices::POLTERGEIST,
                accuracy: 90.0,
                base_power: 110.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POPULATIONBOMB,
            Choice {
                move_id: Choices::POPULATIONBOMB,
                accuracy: 90.0,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POUNCE,
            Choice {
                move_id: Choices::POUNCE,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POUND,
            Choice {
                move_id: Choices::POUND,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POWDER,
            Choice {
                move_id: Choices::POWDER,
                priority: 1,
                move_type: PokemonType::BUG,
                flags: Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::POWDER,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POWDERSNOW,
            Choice {
                move_id: Choices::POWDERSNOW,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::FREEZE),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::POWERGEM,
                Choice {
                    move_id: Choices::POWERGEM,
                    base_power: 70.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::POWERGEM,
                Choice {
                    move_id: Choices::POWERGEM,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::POWERSHIFT,
            Choice {
                move_id: Choices::POWERSHIFT,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::POWERSHIFT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POWERSPLIT,
            Choice {
                move_id: Choices::POWERSPLIT,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POWERSWAP,
            Choice {
                move_id: Choices::POWERSWAP,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POWERTRICK,
            Choice {
                move_id: Choices::POWERTRICK,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::POWERTRICK,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POWERTRIP,
            Choice {
                move_id: Choices::POWERTRIP,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POWERUPPUNCH,
            Choice {
                move_id: Choices::POWERUPPUNCH,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::POWERWHIP,
            Choice {
                move_id: Choices::POWERWHIP,
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PRECIPICEBLADES,
            Choice {
                move_id: Choices::PRECIPICEBLADES,
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PRESENT,
            Choice {
                move_id: Choices::PRESENT,
                accuracy: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PRISMATICLASER,
            Choice {
                move_id: Choices::PRISMATICLASER,
                base_power: 160.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PROTECT,
            Choice {
                move_id: Choices::PROTECT,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::PROTECT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYBEAM,
            Choice {
                move_id: Choices::PSYBEAM,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYBLADE,
            Choice {
                move_id: Choices::PSYBLADE,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::PSYCHIC,
                Choice {
                    move_id: Choices::PSYCHIC,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 33.2,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::PSYCHIC,
                Choice {
                    move_id: Choices::PSYCHIC,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::PSYCHICFANGS,
            Choice {
                move_id: Choices::PSYCHICFANGS,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYCHICNOISE,
            Choice {
                move_id: Choices::PSYCHICNOISE,
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::HEALBLOCK,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYCHICTERRAIN,
            Choice {
                move_id: Choices::PSYCHICTERRAIN,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYCHOBOOST,
            Choice {
                move_id: Choices::PSYCHOBOOST,
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYCHOCUT,
            Choice {
                move_id: Choices::PSYCHOCUT,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::PSYCHOSHIFT,
                Choice {
                    accuracy: 90.0,
                    move_id: Choices::PSYCHOSHIFT,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::PSYCHOSHIFT,
                Choice {
                    move_id: Choices::PSYCHOSHIFT,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::PSYCHUP,
            Choice {
                move_id: Choices::PSYCHUP,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYSHIELDBASH,
            Choice {
                move_id: Choices::PSYSHIELDBASH,
                accuracy: 90.0,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYSHOCK,
            Choice {
                move_id: Choices::PSYSHOCK,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PSYSTRIKE,
            Choice {
                move_id: Choices::PSYSTRIKE,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::PSYWAVE,
                Choice {
                    accuracy: 90.0,
                    move_id: Choices::PSYWAVE,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::PSYWAVE,
                Choice {
                    move_id: Choices::PSYWAVE,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::PUNISHMENT,
            Choice {
                move_id: Choices::PUNISHMENT,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PURIFY,
            Choice {
                move_id: Choices::PURIFY,
                move_type: PokemonType::POISON,
                flags: Flags {
                    heal: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PURSUIT,
            Choice {
                move_id: Choices::PURSUIT,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::PYROBALL,
            Choice {
                move_id: Choices::PYROBALL,
                accuracy: 90.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::QUASH,
            Choice {
                move_id: Choices::QUASH,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::QUICKATTACK,
            Choice {
                move_id: Choices::QUICKATTACK,
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::QUICKGUARD,
            Choice {
                move_id: Choices::QUICKGUARD,
                priority: 3,
                target: MoveTarget::User,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::QuickGuard,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::QUIVERDANCE,
            Choice {
                move_id: Choices::QUIVERDANCE,
                target: MoveTarget::User,
                move_type: PokemonType::BUG,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RAGE,
            Choice {
                move_id: Choices::RAGE,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RAGEFIST,
            Choice {
                move_id: Choices::RAGEFIST,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RAGEPOWDER,
            Choice {
                move_id: Choices::RAGEPOWDER,
                priority: 2,
                target: MoveTarget::User,
                move_type: PokemonType::BUG,
                flags: Flags {
                    powder: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::RAGEPOWDER,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RAGINGBULL,
            Choice {
                move_id: Choices::RAGINGBULL,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RAGINGFURY,
            Choice {
                move_id: Choices::RAGINGFURY,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RAINDANCE,
            Choice {
                move_id: Choices::RAINDANCE,
                move_type: PokemonType::WATER,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        if cfg!(feature = "gen9") || cfg!(feature = "gen8") {
            moves.insert(
                Choices::RAPIDSPIN,
                Choice {
                    move_id: Choices::RAPIDSPIN,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::RAPIDSPIN,
                Choice {
                    move_id: Choices::RAPIDSPIN,
                    base_power: 20.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: None,
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::RAZORLEAF,
            Choice {
                move_id: Choices::RAZORLEAF,
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RAZORSHELL,
            Choice {
                move_id: Choices::RAZORSHELL,
                accuracy: 95.0,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RAZORWIND,
            Choice {
                move_id: Choices::RAZORWIND,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RECHARGE,
            Choice {
                move_id: Choices::RECHARGE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RECOVER,
            Choice {
                move_id: Choices::RECOVER,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RECYCLE,
            Choice {
                move_id: Choices::RECYCLE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::REFLECT,
                Choice {
                    move_id: Choices::REFLECT,
                    target: MoveTarget::User,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::REFLECT,
                }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::REFLECT,
                Choice {
                    move_id: Choices::REFLECT,
                    target: MoveTarget::User,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    side_condition: Some(SideCondition {
                        target: MoveTarget::User,
                        condition: PokemonSideCondition::Reflect,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::REFLECTTYPE,
            Choice {
                move_id: Choices::REFLECTTYPE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::REFRESH,
            Choice {
                move_id: Choices::REFRESH,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RELICSONG,
            Choice {
                move_id: Choices::RELICSONG,
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::SLEEP),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::REST,
            Choice {
                move_id: Choices::REST,
                status: None,  // Rest implemented in choice_special_effect()
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: None,  // Rest implemented in choice_special_effect()
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RETALIATE,
            Choice {
                move_id: Choices::RETALIATE,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RETURN,
            Choice {
                move_id: Choices::RETURN,
                base_power: 102.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RETURN102,
            Choice {
                move_id: Choices::RETURN102,
                base_power: 102.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::REVELATIONDANCE,
            Choice {
                move_id: Choices::REVELATIONDANCE,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::REVENGE,
            Choice {
                move_id: Choices::REVENGE,
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: -4,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::REVERSAL,
            Choice {
                move_id: Choices::REVERSAL,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::REVIVALBLESSING,
            Choice {
                move_id: Choices::REVIVALBLESSING,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RISINGVOLTAGE,
            Choice {
                move_id: Choices::RISINGVOLTAGE,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROAR,
            Choice {
                move_id: Choices::ROAR,
                priority: -6,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    drag: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROAROFTIME,
            Choice {
                move_id: Choices::ROAROFTIME,
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::ROCKBLAST,
                Choice {
                    move_id: Choices::ROCKBLAST,
                    accuracy: 80.0,
                    base_power: 25.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                        bullet: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ROCKBLAST,
                Choice {
                    move_id: Choices::ROCKBLAST,
                    accuracy: 90.0,
                    base_power: 25.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                        bullet: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ROCKCLIMB,
            Choice {
                move_id: Choices::ROCKCLIMB,
                accuracy: 85.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROCKPOLISH,
            Choice {
                move_id: Choices::ROCKPOLISH,
                target: MoveTarget::User,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::ROCKSLIDE,
                Choice {
                    move_id: Choices::ROCKSLIDE,
                    accuracy: 90.0,
                    base_power: 75.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ROCKSLIDE,
                Choice {
                    move_id: Choices::ROCKSLIDE,
                    accuracy: 90.0,
                    base_power: 75.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    }]),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
            Choices::ROCKSMASH,
            Choice {
                move_id: Choices::ROCKSMASH,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        } else {
            moves.insert(
                Choices::ROCKSMASH,
                Choice {
                    move_id: Choices::ROCKSMASH,
                    base_power: 40.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 50.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::ROCKTHROW,
                Choice {
                    move_id: Choices::ROCKTHROW,
                    accuracy: 65.0,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ROCKTHROW,
                Choice {
                    move_id: Choices::ROCKTHROW,
                    accuracy: 90.0,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::ROCKTOMB,
                Choice {
                    move_id: Choices::ROCKTOMB,
                    accuracy: 80.0,
                    base_power: 50.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ROCKTOMB,
                Choice {
                    move_id: Choices::ROCKTOMB,
                    accuracy: 95.0,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::ROCK,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ROCKWRECKER,
            Choice {
                move_id: Choices::ROCKWRECKER,
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROLEPLAY,
            Choice {
                move_id: Choices::ROLEPLAY,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROLLINGKICK,
            Choice {
                move_id: Choices::ROLLINGKICK,
                accuracy: 85.0,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROLLOUT,
            Choice {
                move_id: Choices::ROLLOUT,
                accuracy: 90.0,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROOST,
            Choice {
                move_id: Choices::ROOST,
                target: MoveTarget::User,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::ROOST,
                }),
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROTOTILLER,
            Choice {
                move_id: Choices::ROTOTILLER,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ROUND,
            Choice {
                move_id: Choices::ROUND,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::RUINATION,
            Choice {
                move_id: Choices::RUINATION,
                accuracy: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SACREDFIRE,
            Choice {
                move_id: Choices::SACREDFIRE,
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SACREDSWORD,
            Choice {
                move_id: Choices::SACREDSWORD,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SAFEGUARD,
            Choice {
                move_id: Choices::SAFEGUARD,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Safeguard,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SALTCURE,
            Choice {
                move_id: Choices::SALTCURE,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::SALTCURE,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::SANDATTACK,
                Choice {
                    move_id: Choices::SANDATTACK,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::Opponent,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        },
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SANDATTACK,
                Choice {
                    move_id: Choices::SANDATTACK,
                    move_type: PokemonType::GROUND,
                    flags: Flags {
                        protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::Opponent,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SANDSEARSTORM,
            Choice {
                move_id: Choices::SANDSEARSTORM,
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SANDSTORM,
            Choice {
                move_id: Choices::SANDSTORM,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    wind: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::SANDTOMB,
                Choice {
                    move_id: Choices::SANDTOMB,
                    accuracy: 70.0,
                    base_power: 15.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GROUND,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SANDTOMB,
                Choice {
                    move_id: Choices::SANDTOMB,
                    accuracy: 85.0,
                    base_power: 35.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GROUND,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SAPPYSEED,
            Choice {
                move_id: Choices::SAPPYSEED,
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SCALD,
            Choice {
                move_id: Choices::SCALD,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SCALESHOT,
            Choice {
                move_id: Choices::SCALESHOT,
                accuracy: 90.0,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::SCARYFACE,
                Choice {
                    move_id: Choices::SCARYFACE,
                    accuracy: 90.0,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::Opponent,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -2,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SCARYFACE,
                Choice {
                    move_id: Choices::SCARYFACE,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::Opponent,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -2,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SCORCHINGSANDS,
            Choice {
                move_id: Choices::SCORCHINGSANDS,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SCRATCH,
            Choice {
                move_id: Choices::SCRATCH,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SCREECH,
            Choice {
                move_id: Choices::SCREECH,
                accuracy: 85.0,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SEARINGSHOT,
            Choice {
                move_id: Choices::SEARINGSHOT,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SECRETPOWER,
            Choice {
                move_id: Choices::SECRETPOWER,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SECRETSWORD,
            Choice {
                move_id: Choices::SECRETSWORD,
                base_power: 85.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SEEDBOMB,
            Choice {
                move_id: Choices::SEEDBOMB,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SEEDFLARE,
            Choice {
                move_id: Choices::SEEDFLARE,
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 40.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -2,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SEISMICTOSS,
            Choice {
                move_id: Choices::SEISMICTOSS,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::SELFDESTRUCT,
                Choice {
                    move_id: Choices::SELFDESTRUCT,
                    base_power: 130.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SELFDESTRUCT,
                Choice {
                    move_id: Choices::SELFDESTRUCT,
                    base_power: 200.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SHADOWBALL,
            Choice {
                move_id: Choices::SHADOWBALL,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHADOWBONE,
            Choice {
                move_id: Choices::SHADOWBONE,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHADOWCLAW,
            Choice {
                move_id: Choices::SHADOWCLAW,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHADOWFORCE,
            Choice {
                move_id: Choices::SHADOWFORCE,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    charge: true,
                    contact: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHADOWPUNCH,
            Choice {
                move_id: Choices::SHADOWPUNCH,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHADOWSNEAK,
            Choice {
                move_id: Choices::SHADOWSNEAK,
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHADOWSTRIKE,
            Choice {
                move_id: Choices::SHADOWSTRIKE,
                accuracy: 95.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHARPEN,
            Choice {
                move_id: Choices::SHARPEN,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHEDTAIL,
            Choice {
                move_id: Choices::SHEDTAIL,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    pivot: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHEERCOLD,
            Choice {
                move_id: Choices::SHEERCOLD,
                accuracy: 30.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ICE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHELLSIDEARM,
            Choice {
                move_id: Choices::SHELLSIDEARM,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHELLSMASH,
            Choice {
                move_id: Choices::SHELLSMASH,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: -1,
                        special_attack: 2,
                        special_defense: -1,
                        speed: 2,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHELLTRAP,
            Choice {
                move_id: Choices::SHELLTRAP,
                base_power: 150.0,
                category: MoveCategory::Special,
                priority: -3,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHELTER,
            Choice {
                move_id: Choices::SHELTER,
                target: MoveTarget::User,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHIFTGEAR,
            Choice {
                move_id: Choices::SHIFTGEAR,
                target: MoveTarget::User,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHOCKWAVE,
            Choice {
                move_id: Choices::SHOCKWAVE,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SHOREUP,
            Choice {
                move_id: Choices::SHOREUP,
                target: MoveTarget::User,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SIGNALBEAM,
            Choice {
                move_id: Choices::SIGNALBEAM,
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SILKTRAP,
            Choice {
                move_id: Choices::SILKTRAP,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::BUG,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SILKTRAP,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SILVERWIND,
            Choice {
                move_id: Choices::SILVERWIND,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SIMPLEBEAM,
            Choice {
                move_id: Choices::SIMPLEBEAM,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SING,
            Choice {
                move_id: Choices::SING,
                accuracy: 55.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::SLEEP,
                }),
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SIZZLYSLIDE,
            Choice {
                move_id: Choices::SIZZLYSLIDE,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SKETCH,
            Choice {
                move_id: Choices::SKETCH,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SKILLSWAP,
            Choice {
                move_id: Choices::SKILLSWAP,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SKITTERSMACK,
            Choice {
                move_id: Choices::SKITTERSMACK,
                accuracy: 90.0,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::SKULLBASH,
                Choice {
                    move_id: Choices::SKULLBASH,
                    base_power: 100.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        charge: true,
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SKULLBASH,
                Choice {
                    move_id: Choices::SKULLBASH,
                    base_power: 130.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        charge: true,
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SKYATTACK,
            Choice {
                move_id: Choices::SKYATTACK,
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SKYDROP,
            Choice {
                move_id: Choices::SKYDROP,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    charge: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SKYUPPERCUT,
            Choice {
                move_id: Choices::SKYUPPERCUT,
                accuracy: 90.0,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SLACKOFF,
            Choice {
                move_id: Choices::SLACKOFF,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SLAM,
            Choice {
                move_id: Choices::SLAM,
                accuracy: 75.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SLASH,
            Choice {
                move_id: Choices::SLASH,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SLEEPPOWDER,
            Choice {
                move_id: Choices::SLEEPPOWDER,
                accuracy: 75.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::SLEEP,
                }),
                move_type: PokemonType::GRASS,
                flags: Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SLEEPTALK,
            Choice {
                move_id: Choices::SLEEPTALK,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::SLUDGE,
                Choice {
                    move_id: Choices::SLUDGE,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 40.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::POISON),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SLUDGE,
                Choice {
                    move_id: Choices::SLUDGE,
                    base_power: 65.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::POISON),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SLUDGEBOMB,
            Choice {
                move_id: Choices::SLUDGEBOMB,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SLUDGEWAVE,
            Choice {
                move_id: Choices::SLUDGEWAVE,
                base_power: 95.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SMACKDOWN,
            Choice {
                move_id: Choices::SMACKDOWN,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::SMACKDOWN,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SMARTSTRIKE,
            Choice {
                move_id: Choices::SMARTSTRIKE,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::SMELLINGSALTS,
                Choice {
                    move_id: Choices::SMELLINGSALTS,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SMELLINGSALTS,
                Choice {
                    move_id: Choices::SMELLINGSALTS,
                    base_power: 70.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::SMOG,
                Choice {
                    move_id: Choices::SMOG,
                    accuracy: 70.0,
                    base_power: 20.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 40.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::POISON),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SMOG,
                Choice {
                    move_id: Choices::SMOG,
                    accuracy: 70.0,
                    base_power: 30.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 40.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::POISON),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SMOKESCREEN,
            Choice {
                move_id: Choices::SMOKESCREEN,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SNAPTRAP,
            Choice {
                move_id: Choices::SNAPTRAP,
                base_power: 35.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SNARL,
            Choice {
                move_id: Choices::SNARL,
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SNATCH,
            Choice {
                move_id: Choices::SNATCH,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::DARK,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SNATCH,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SNIPESHOT,
            Choice {
                move_id: Choices::SNIPESHOT,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::SNORE,
                Choice {
                    move_id: Choices::SNORE,
                    base_power: 40.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        sound: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SNORE,
                Choice {
                    move_id: Choices::SNORE,
                    base_power: 50.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        sound: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SNOWSCAPE,
            Choice {
                move_id: Choices::SNOWSCAPE,
                move_type: PokemonType::ICE,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SOAK,
            Choice {
                move_id: Choices::SOAK,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SOFTBOILED,
            Choice {
                move_id: Choices::SOFTBOILED,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SOLARBEAM,
            Choice {
                move_id: Choices::SOLARBEAM,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SOLARBLADE,
            Choice {
                move_id: Choices::SOLARBLADE,
                base_power: 125.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    charge: true,
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SONICBOOM,
            Choice {
                move_id: Choices::SONICBOOM,
                accuracy: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPACIALREND,
            Choice {
                move_id: Choices::SPACIALREND,
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPARK,
            Choice {
                move_id: Choices::SPARK,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPARKLINGARIA,
            Choice {
                move_id: Choices::SPARKLINGARIA,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::SPARKLINGARIA),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPARKLYSWIRL,
            Choice {
                move_id: Choices::SPARKLYSWIRL,
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPECTRALTHIEF,
            Choice {
                move_id: Choices::SPECTRALTHIEF,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPEEDSWAP,
            Choice {
                move_id: Choices::SPEEDSWAP,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPICYEXTRACT,
            Choice {
                move_id: Choices::SPICYEXTRACT,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: -2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPIDERWEB,
            Choice {
                move_id: Choices::SPIDERWEB,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPIKECANNON,
            Choice {
                move_id: Choices::SPIKECANNON,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPIKES,
            Choice {
                move_id: Choices::SPIKES,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    reflectable: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::Opponent,
                    condition: PokemonSideCondition::Spikes,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPIKYSHIELD,
            Choice {
                move_id: Choices::SPIKYSHIELD,
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SPIKYSHIELD,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPINOUT,
            Choice {
                move_id: Choices::SPINOUT,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -2,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPIRITBREAK,
            Choice {
                move_id: Choices::SPIRITBREAK,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPIRITSHACKLE,
            Choice {
                move_id: Choices::SPIRITSHACKLE,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPITE,
            Choice {
                move_id: Choices::SPITE,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPITUP,
            Choice {
                move_id: Choices::SPITUP,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPLASH,
            Choice {
                move_id: Choices::SPLASH,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPLISHYSPLASH,
            Choice {
                move_id: Choices::SPLISHYSPLASH,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPORE,
            Choice {
                move_id: Choices::SPORE,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::SLEEP,
                }),
                move_type: PokemonType::GRASS,
                flags: Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPOTLIGHT,
            Choice {
                move_id: Choices::SPOTLIGHT,
                priority: 3,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::SPOTLIGHT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SPRINGTIDESTORM,
            Choice {
                move_id: Choices::SPRINGTIDESTORM,
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STEALTHROCK,
            Choice {
                move_id: Choices::STEALTHROCK,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    reflectable: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::Opponent,
                    condition: PokemonSideCondition::Stealthrock,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STEAMERUPTION,
            Choice {
                move_id: Choices::STEAMERUPTION,
                accuracy: 95.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::BURN),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STEAMROLLER,
            Choice {
                move_id: Choices::STEAMROLLER,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STEELBEAM,
            Choice {
                move_id: Choices::STEELBEAM,
                accuracy: 95.0,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STEELROLLER,
            Choice {
                move_id: Choices::STEELROLLER,
                base_power: 130.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STEELWING,
            Choice {
                move_id: Choices::STEELWING,
                accuracy: 90.0,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STICKYWEB,
            Choice {
                move_id: Choices::STICKYWEB,
                move_type: PokemonType::BUG,
                flags: Flags {
                    reflectable: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::Opponent,
                    condition: PokemonSideCondition::StickyWeb,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STOCKPILE,
            Choice {
                move_id: Choices::STOCKPILE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::STOCKPILE,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STOMP,
            Choice {
                move_id: Choices::STOMP,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STOMPINGTANTRUM,
            Choice {
                move_id: Choices::STOMPINGTANTRUM,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STONEAXE,
            Choice {
                move_id: Choices::STONEAXE,
                accuracy: 90.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::Opponent,
                    condition: PokemonSideCondition::Stealthrock,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STONEEDGE,
            Choice {
                move_id: Choices::STONEEDGE,
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STOREDPOWER,
            Choice {
                move_id: Choices::STOREDPOWER,
                base_power: 20.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::STORMTHROW,
                Choice {
                    move_id: Choices::STORMTHROW,
                    base_power: 40.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::STORMTHROW,
                Choice {
                    move_id: Choices::STORMTHROW,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::STRANGESTEAM,
            Choice {
                move_id: Choices::STRANGESTEAM,
                accuracy: 95.0,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STRENGTH,
            Choice {
                move_id: Choices::STRENGTH,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STRENGTHSAP,
            Choice {
                move_id: Choices::STRENGTHSAP,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    heal: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STRINGSHOT,
            Choice {
                move_id: Choices::STRINGSHOT,
                accuracy: 95.0,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -2,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STRUGGLE,
            Choice {
                move_id: Choices::STRUGGLE,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::STRUGGLEBUG,
                Choice {
                    move_id: Choices::STRUGGLEBUG,
                    base_power: 30.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::STRUGGLEBUG,
                Choice {
                    move_id: Choices::STRUGGLEBUG,
                    base_power: 50.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::STUFFCHEEKS,
            Choice {
                move_id: Choices::STUFFCHEEKS,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::STUNSPORE,
            Choice {
                move_id: Choices::STUNSPORE,
                accuracy: 75.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::PARALYZE,
                }),
                move_type: PokemonType::GRASS,
                flags: Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SUBMISSION,
            Choice {
                move_id: Choices::SUBMISSION,
                accuracy: 80.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.25),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SUBSTITUTE,
            Choice {
                move_id: Choices::SUBSTITUTE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::SUCKERPUNCH,
                Choice {
                    move_id: Choices::SUCKERPUNCH,
                    base_power: 80.0,
                    category: MoveCategory::Physical,
                    priority: 1,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SUCKERPUNCH,
                Choice {
                    move_id: Choices::SUCKERPUNCH,
                    base_power: 70.0,
                    category: MoveCategory::Physical,
                    priority: 1,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SUNNYDAY,
            Choice {
                move_id: Choices::SUNNYDAY,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SUNSTEELSTRIKE,
            Choice {
                move_id: Choices::SUNSTEELSTRIKE,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SUPERCELLSLAM,
            Choice {
                accuracy: 95.0,
                move_id: Choices::SUPERCELLSLAM,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                crash: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SUPERFANG,
            Choice {
                move_id: Choices::SUPERFANG,
                accuracy: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SUPERPOWER,
            Choice {
                move_id: Choices::SUPERPOWER,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SUPERSONIC,
            Choice {
                move_id: Choices::SUPERSONIC,
                accuracy: 55.0,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::SURF,
                Choice {
                    move_id: Choices::SURF,
                    base_power: 95.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SURF,
                Choice {
                    move_id: Choices::SURF,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SURGINGSTRIKES,
            Choice {
                move_id: Choices::SURGINGSTRIKES,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::SWAGGER,
                Choice {
                    move_id: Choices::SWAGGER,
                    accuracy: 85.0,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::Opponent,
                        boosts: StatBoosts {
                            attack: 2,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::CONFUSION,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SWAGGER,
                Choice {
                    move_id: Choices::SWAGGER,
                    accuracy: 90.0,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::Opponent,
                        boosts: StatBoosts {
                            attack: 2,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::CONFUSION,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SWALLOW,
            Choice {
                move_id: Choices::SWALLOW,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SWEETKISS,
            Choice {
                move_id: Choices::SWEETKISS,
                accuracy: 75.0,
                move_type: PokemonType::FAIRY,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SWEETSCENT,
            Choice {
                move_id: Choices::SWEETSCENT,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SWIFT,
            Choice {
                move_id: Choices::SWIFT,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SWITCHEROO,
            Choice {
                move_id: Choices::SWITCHEROO,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SWORDSDANCE,
            Choice {
                move_id: Choices::SWORDSDANCE,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::SYNCHRONOISE,
                Choice {
                    move_id: Choices::SYNCHRONOISE,
                    base_power: 70.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::SYNCHRONOISE,
                Choice {
                    move_id: Choices::SYNCHRONOISE,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::SYNTHESIS,
            Choice {
                move_id: Choices::SYNTHESIS,
                target: MoveTarget::User,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::SYRUPBOMB,
            Choice {
                move_id: Choices::SYRUPBOMB,
                accuracy: 85.0,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::SYRUPBOMB),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TACHYONCUTTER,
            Choice {
                move_id: Choices::TACHYONCUTTER,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::STEEL,
                flags: Flags {
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TACKLE,
            Choice {
                move_id: Choices::TACKLE,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::TAILGLOW,
                Choice {
                    move_id: Choices::TAILGLOW,
                    target: MoveTarget::User,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                            ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 2,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::TAILGLOW,
                Choice {
                    move_id: Choices::TAILGLOW,
                    target: MoveTarget::User,
                    move_type: PokemonType::BUG,
                    flags: Flags {
                            ..Default::default()
                    },
                    boost: Some(Boost {
                        target: MoveTarget::User,
                        boosts: StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 3,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        },
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::TAILSLAP,
            Choice {
                move_id: Choices::TAILSLAP,
                accuracy: 85.0,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TAILWHIP,
            Choice {
                move_id: Choices::TAILWHIP,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TAILWIND,
            Choice {
                move_id: Choices::TAILWIND,
                target: MoveTarget::User,
                move_type: PokemonType::FLYING,
                flags: Flags {
                    wind: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Tailwind,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TAKEDOWN,
            Choice {
                move_id: Choices::TAKEDOWN,
                accuracy: 85.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TAKEHEART,
            Choice {
                move_id: Choices::TAKEHEART,
                target: MoveTarget::User,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TARSHOT,
            Choice {
                move_id: Choices::TARSHOT,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                }),
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::TARSHOT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TAUNT,
            Choice {
                move_id: Choices::TAUNT,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::TAUNT,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TEARFULLOOK,
            Choice {
                move_id: Choices::TEARFULLOOK,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TEATIME,
            Choice {
                move_id: Choices::TEATIME,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::TECHNOBLAST,
                Choice {
                    move_id: Choices::TECHNOBLAST,
                    base_power: 85.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::TECHNOBLAST,
                Choice {
                    move_id: Choices::TECHNOBLAST,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::TEETERDANCE,
            Choice {
                move_id: Choices::TEETERDANCE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TELEKINESIS,
            Choice {
                move_id: Choices::TELEKINESIS,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::TELEKINESIS,
                }),
                ..Default::default()
            },
        );

        if cfg!(feature = "gen9") || cfg!(feature = "gen8") {
            moves.insert(
                Choices::TELEPORT,
                Choice {
                    move_id: Choices::TELEPORT,
                    priority: -6,
                    target: MoveTarget::User,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        pivot: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::TELEPORT,
                Choice {
                    move_id: Choices::TELEPORT,
                    priority: 0,
                    target: MoveTarget::User,
                    move_type: PokemonType::PSYCHIC,
                    flags: Flags {
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::TEMPERFLARE,
            Choice {
                move_id: Choices::TEMPERFLARE,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TERABLAST,
            Choice {
                move_id: Choices::TERABLAST,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TERASTARSTORM,
            Choice {
                move_id: Choices::TERASTARSTORM,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TERRAINPULSE,
            Choice {
                move_id: Choices::TERRAINPULSE,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::THIEF,
                Choice {
                    move_id: Choices::THIEF,
                    base_power: 40.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::THIEF,
                Choice {
                    move_id: Choices::THIEF,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::THOUSANDARROWS,
            Choice {
                move_id: Choices::THOUSANDARROWS,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::SMACKDOWN,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::THOUSANDWAVES,
            Choice {
                move_id: Choices::THOUSANDWAVES,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GROUND,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::THRASH,
                Choice {
                    move_id: Choices::THRASH,
                    base_power: 90.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::User,
                        volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::THRASH,
                Choice {
                    move_id: Choices::THRASH,
                    base_power: 120.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::User,
                        volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::THROATCHOP,
            Choice {
                move_id: Choices::THROATCHOP,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::THROATCHOP,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::THUNDER,
                Choice {
                    move_id: Choices::THUNDER,
                    accuracy: 70.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        } else if cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::THUNDER,
                Choice {
                    move_id: Choices::THUNDER,
                    accuracy: 70.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::THUNDER,
                Choice {
                    move_id: Choices::THUNDER,
                    accuracy: 70.0,
                    base_power: 110.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::THUNDERBOLT,
                Choice {
                    move_id: Choices::THUNDERBOLT,
                    base_power: 95.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::THUNDERBOLT,
                Choice {
                    move_id: Choices::THUNDERBOLT,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::THUNDERCAGE,
            Choice {
                move_id: Choices::THUNDERCAGE,
                accuracy: 90.0,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::THUNDERCLAP,
            Choice {
                move_id: Choices::THUNDERCLAP,
                base_power: 70.0,
                priority: 1,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::THUNDERFANG,
            Choice {
                move_id: Choices::THUNDERFANG,
                accuracy: 95.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    },
                ]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::THUNDEROUSKICK,
            Choice {
                move_id: Choices::THUNDEROUSKICK,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::THUNDERPUNCH,
            Choice {
                move_id: Choices::THUNDERPUNCH,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::THUNDERSHOCK,
            Choice {
                move_id: Choices::THUNDERSHOCK,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::THUNDERWAVE,
                Choice {
                    move_id: Choices::THUNDERWAVE,
                    accuracy: 100.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::PARALYZE,
                    }),
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::THUNDERWAVE,
                Choice {
                    move_id: Choices::THUNDERWAVE,
                    accuracy: 90.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::PARALYZE,
                    }),
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::TICKLE,
            Choice {
                move_id: Choices::TICKLE,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TIDYUP,
            Choice {
                move_id: Choices::TIDYUP,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TOPSYTURVY,
            Choice {
                move_id: Choices::TOPSYTURVY,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TORCHSONG,
            Choice {
                move_id: Choices::TORCHSONG,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TORMENT,
            Choice {
                move_id: Choices::TORMENT,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::TORMENT,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::TOXIC,
                Choice {
                    move_id: Choices::TOXIC,
                    accuracy: 85.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::TOXIC,
                    }),
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::TOXIC,
                Choice {
                    move_id: Choices::TOXIC,
                    accuracy: 90.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::TOXIC,
                    }),
                    move_type: PokemonType::POISON,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::TOXICSPIKES,
            Choice {
                move_id: Choices::TOXICSPIKES,
                move_type: PokemonType::POISON,
                flags: Flags {
                    reflectable: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::Opponent,
                    condition: PokemonSideCondition::ToxicSpikes,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TOXICTHREAD,
            Choice {
                move_id: Choices::TOXICTHREAD,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::POISON,
                }),
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRAILBLAZE,
            Choice {
                move_id: Choices::TRAILBLAZE,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRANSFORM,
            Choice {
                move_id: Choices::TRANSFORM,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::TRIATTACK,
                Choice {
                    move_id: Choices::TRIATTACK,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::TRIATTACK,
                Choice {
                    move_id: Choices::TRIATTACK,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        protect: true,
                        ..Default::default()
                    },
                    // not technically correct but idgaf
                    // this should roll 20% chance to inflict, and then roll for the status
                    secondaries: Some(
                        vec![
                            Secondary {
                                chance: 6.67,
                                target: MoveTarget::Opponent,
                                effect: Effect::Status(PokemonStatus::PARALYZE),
                            },
                            Secondary {
                                chance: 6.67,
                                target: MoveTarget::Opponent,
                                effect: Effect::Status(PokemonStatus::BURN),
                            },
                            Secondary {
                                chance: 6.67,
                                target: MoveTarget::Opponent,
                                effect: Effect::Status(PokemonStatus::FREEZE),
                            },
                        ]
                    ),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::TRICK,
            Choice {
                move_id: Choices::TRICK,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRICKORTREAT,
            Choice {
                move_id: Choices::TRICKORTREAT,
                move_type: PokemonType::GHOST,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRICKROOM,
            Choice {
                move_id: Choices::TRICKROOM,
                priority: -7,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRIPLEARROWS,
            Choice {
                move_id: Choices::TRIPLEARROWS,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        }),
                    },
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                    },
                ]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRIPLEAXEL,
            Choice {
                move_id: Choices::TRIPLEAXEL,
                accuracy: 90.0,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ICE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRIPLEDIVE,
            Choice {
                move_id: Choices::TRIPLEDIVE,
                accuracy: 95.0,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRIPLEKICK,
            Choice {
                move_id: Choices::TRIPLEKICK,
                accuracy: 90.0,
                base_power: 10.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TROPKICK,
            Choice {
                move_id: Choices::TROPKICK,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Boost(StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TRUMPCARD,
            Choice {
                move_id: Choices::TRUMPCARD,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TWINBEAM,
            Choice {
                move_id: Choices::TWINBEAM,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TWINEEDLE,
            Choice {
                move_id: Choices::TWINEEDLE,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::POISON),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::TWISTER,
            Choice {
                move_id: Choices::TWISTER,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::DRAGON,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::UPPERHAND,
            Choice {
                move_id: Choices::UPPERHAND,
                base_power: 65.0,
                priority: 3,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::UPROAR,
                Choice {
                    move_id: Choices::UPROAR,
                    base_power: 50.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        sound: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::UPROAR,
                Choice {
                    move_id: Choices::UPROAR,
                    base_power: 90.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                            protect: true,
                        sound: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::UTURN,
            Choice {
                move_id: Choices::UTURN,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    pivot: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VACUUMWAVE,
            Choice {
                move_id: Choices::VACUUMWAVE,
                base_power: 40.0,
                category: MoveCategory::Special,
                priority: 1,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VCREATE,
            Choice {
                move_id: Choices::VCREATE,
                accuracy: 95.0,
                base_power: 180.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::FIRE,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: -1,
                        speed: -1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VEEVEEVOLLEY,
            Choice {
                move_id: Choices::VEEVEEVOLLEY,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VENOMDRENCH,
            Choice {
                move_id: Choices::VENOMDRENCH,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VENOSHOCK,
            Choice {
                move_id: Choices::VENOSHOCK,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::POISON,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VICTORYDANCE,
            Choice {
                move_id: Choices::VICTORYDANCE,
                target: MoveTarget::User,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::VINEWHIP,
                Choice {
                    move_id: Choices::VINEWHIP,
                    base_power: 35.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::VINEWHIP,
                Choice {
                    move_id: Choices::VINEWHIP,
                    base_power: 45.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::GRASS,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::VISEGRIP,
            Choice {
                move_id: Choices::VISEGRIP,
                base_power: 55.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VITALTHROW,
            Choice {
                move_id: Choices::VITALTHROW,
                base_power: 70.0,
                category: MoveCategory::Physical,
                priority: -1,
                move_type: PokemonType::FIGHTING,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VOLTSWITCH,
            Choice {
                move_id: Choices::VOLTSWITCH,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    pivot: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::VOLTTACKLE,
            Choice {
                move_id: Choices::VOLTTACKLE,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::WAKEUPSLAP,
                Choice {
                    move_id: Choices::WAKEUPSLAP,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::WAKEUPSLAP,
                Choice {
                    move_id: Choices::WAKEUPSLAP,
                    base_power: 70.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FIGHTING,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::WATERFALL,
            Choice {
                move_id: Choices::WATERFALL,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WATERGUN,
            Choice {
                move_id: Choices::WATERGUN,
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::WATERPLEDGE,
                Choice {
                    move_id: Choices::WATERPLEDGE,
                    base_power: 50.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::WATERPLEDGE,
                Choice {
                    move_id: Choices::WATERPLEDGE,
                    base_power: 80.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                                protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::WATERPULSE,
            Choice {
                move_id: Choices::WATERPULSE,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION),
                }]),
                ..Default::default()
            },
        );

        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") || cfg!(feature = "gen6") {
            moves.insert(
                Choices::WATERSHURIKEN,
                Choice {
                    move_id: Choices::WATERSHURIKEN,
                    base_power: 15.0,
                    category: MoveCategory::Physical,
                    priority: 1,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::WATERSHURIKEN,
                Choice {
                    move_id: Choices::WATERSHURIKEN,
                    base_power: 15.0,
                    category: MoveCategory::Special,
                    priority: 1,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }

        moves.insert(
            Choices::WATERSPORT,
            Choice {
                move_id: Choices::WATERSPORT,
                move_type: PokemonType::WATER,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WATERSPOUT,
            Choice {
                move_id: Choices::WATERSPOUT,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::WATER,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WAVECRASH,
            Choice {
                move_id: Choices::WAVECRASH,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::WATER,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WEATHERBALL,
            Choice {
                move_id: Choices::WEATHERBALL,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::WHIRLPOOL,
                Choice {
                    move_id: Choices::WHIRLPOOL,
                    accuracy: 70.0,
                    base_power: 15.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::WHIRLPOOL,
                Choice {
                    move_id: Choices::WHIRLPOOL,
                    accuracy: 85.0,
                    base_power: 35.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::WATER,
                    flags: Flags {
                            protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::WHIRLWIND,
            Choice {
                move_id: Choices::WHIRLWIND,
                priority: -6,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    drag: true,
                    reflectable: true,
                    wind: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        if cfg!(feature = "gen9") {
            moves.insert(
                Choices::WICKEDBLOW,
                Choice {
                    move_id: Choices::WICKEDBLOW,
                    base_power: 75.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        punch: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::WICKEDBLOW,
                Choice {
                    move_id: Choices::WICKEDBLOW,
                    base_power: 80.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::DARK,
                    flags: Flags {
                        contact: true,
                            protect: true,
                        punch: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }

        moves.insert(
            Choices::WICKEDTORQUE,
            Choice {
                move_id: Choices::WICKEDTORQUE,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::DARK,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::SLEEP),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WIDEGUARD,
            Choice {
                move_id: Choices::WIDEGUARD,
                priority: 3,
                target: MoveTarget::User,
                move_type: PokemonType::ROCK,
                flags: Flags {
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::WideGuard,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WILDBOLTSTORM,
            Choice {
                move_id: Choices::WILDBOLTSTORM,
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::PARALYZE),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WILDCHARGE,
            Choice {
                move_id: Choices::WILDCHARGE,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.25),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") || cfg!(feature = "gen5") {
            moves.insert(
                Choices::WILLOWISP,
                Choice {
                    move_id: Choices::WILLOWISP,
                    accuracy: 75.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::BURN,
                    }),
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::WILLOWISP,
                Choice {
                    move_id: Choices::WILLOWISP,
                    accuracy: 85.0,
                    status: Some(Status {
                        target: MoveTarget::Opponent,
                        status: PokemonStatus::BURN,
                    }),
                    move_type: PokemonType::FIRE,
                    flags: Flags {
                            protect: true,
                        reflectable: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        if cfg!(feature = "gen1") {
            moves.insert(
                Choices::WINGATTACK,
                Choice {
                    move_id: Choices::WINGATTACK,
                    base_power: 35.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::WINGATTACK,
                Choice {
                    move_id: Choices::WINGATTACK,
                    base_power: 60.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::FLYING,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::WISH,
            Choice {
                move_id: Choices::WISH,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    heal: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WITHDRAW,
            Choice {
                move_id: Choices::WITHDRAW,
                target: MoveTarget::User,
                move_type: PokemonType::WATER,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WONDERROOM,
            Choice {
                move_id: Choices::WONDERROOM,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WOODHAMMER,
            Choice {
                move_id: Choices::WOODHAMMER,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WORKUP,
            Choice {
                move_id: Choices::WORKUP,
                target: MoveTarget::User,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::WORRYSEED,
            Choice {
                move_id: Choices::WORRYSEED,
                move_type: PokemonType::GRASS,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") || cfg!(feature = "gen4") {
            moves.insert(
                Choices::WRAP,
                Choice {
                    move_id: Choices::WRAP,
                    accuracy: 85.0,
                    base_power: 15.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::WRAP,
                Choice {
                    move_id: Choices::WRAP,
                    accuracy: 90.0,
                    base_power: 15.0,
                    category: MoveCategory::Physical,
                    move_type: PokemonType::NORMAL,
                    flags: Flags {
                        contact: true,
                        protect: true,
                        ..Default::default()
                    },
                    volatile_status: Some(VolatileStatus {
                        target: MoveTarget::Opponent,
                        volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                    }),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::WRINGOUT,
            Choice {
                move_id: Choices::WRINGOUT,
                category: MoveCategory::Special,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::XSCISSOR,
            Choice {
                move_id: Choices::XSCISSOR,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::BUG,
                flags: Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            Choices::YAWN,
            Choice {
                move_id: Choices::YAWN,
                move_type: PokemonType::NORMAL,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::YAWN,
                }),
                ..Default::default()
            },
        );
        if cfg!(feature = "gen1") || cfg!(feature = "gen2") || cfg!(feature = "gen3") {
            moves.insert(
                Choices::ZAPCANNON,
                Choice {
                    move_id: Choices::ZAPCANNON,
                    accuracy: 50.0,
                    base_power: 100.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                        bullet: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        } else {
            moves.insert(
                Choices::ZAPCANNON,
                Choice {
                    move_id: Choices::ZAPCANNON,
                    accuracy: 50.0,
                    base_power: 120.0,
                    category: MoveCategory::Special,
                    move_type: PokemonType::ELECTRIC,
                    flags: Flags {
                        bullet: true,
                        protect: true,
                        ..Default::default()
                    },
                    secondaries: Some(vec![Secondary {
                        chance: 100.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }]),
                    ..Default::default()
                },
            );
        }
        moves.insert(
            Choices::ZENHEADBUTT,
            Choice {
                move_id: Choices::ZENHEADBUTT,
                accuracy: 90.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::PSYCHIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ZINGZAP,
            Choice {
                move_id: Choices::ZINGZAP,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            Choices::ZIPPYZAP,
            Choice {
                move_id: Choices::ZIPPYZAP,
                base_power: 80.0,
                category: MoveCategory::Physical,
                priority: 2,
                move_type: PokemonType::ELECTRIC,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::User,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );

        #[cfg(any(feature = "gen1", feature = "gen2", feature = "gen3"))]
        undo_physical_special_split(&mut moves);

        moves
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
    Switch,
}

#[derive(PartialEq, Debug, Clone)]
pub enum MoveTarget {
    User,
    Opponent,
}

#[derive(Debug, Clone)]
pub struct VolatileStatus {
    pub target: MoveTarget,
    pub volatile_status: PokemonVolatileStatus,
}

#[derive(Debug, Clone)]
pub struct SideCondition {
    pub target: MoveTarget,
    pub condition: PokemonSideCondition,
}

#[derive(Debug, Clone)]
pub struct Boost {
    pub target: MoveTarget,
    pub boosts: StatBoosts,
}

#[derive(Debug, Clone)]
pub struct Heal {
    pub target: MoveTarget,
    pub amount: f32,
}

#[derive(Debug, Clone)]
pub struct Status {
    pub target: MoveTarget,
    pub status: PokemonStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatBoosts {
    pub attack: i8,
    pub defense: i8,
    pub special_attack: i8,
    pub special_defense: i8,
    pub speed: i8,
    pub accuracy: i8,
}

impl Default for StatBoosts {
    fn default() -> StatBoosts {
        StatBoosts {
            attack: 0,
            defense: 0,
            special_attack: 0,
            special_defense: 0,
            speed: 0,
            accuracy: 0,
        }
    }
}

impl StatBoosts {
    pub fn get_as_pokemon_boostable(&self) -> [(PokemonBoostableStat, i8); 6] {
        [
            (PokemonBoostableStat::Attack, self.attack),
            (PokemonBoostableStat::Defense, self.defense),
            (PokemonBoostableStat::SpecialAttack, self.special_attack),
            (PokemonBoostableStat::SpecialDefense, self.special_defense),
            (PokemonBoostableStat::Speed, self.speed),
            (PokemonBoostableStat::Accuracy, self.accuracy),
        ]
    }
}

#[derive(Debug)]
pub struct Myself {
    pub volatile_status: Option<VolatileStatus>,
    pub boosts: StatBoosts,
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub bite: bool,
    pub bullet: bool,
    pub charge: bool,
    pub contact: bool,
    pub drag: bool,
    pub heal: bool,
    pub powder: bool,
    pub protect: bool,
    pub pulse: bool,
    pub punch: bool,
    pub recharge: bool,
    pub reflectable: bool,
    pub slicing: bool,
    pub sound: bool,
    pub pivot: bool,
    pub wind: bool,
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            bite: false,
            bullet: false,
            charge: false,
            contact: false,
            drag: false,
            heal: false,
            powder: false,
            protect: false,
            pulse: false,
            punch: false,
            recharge: false,
            reflectable: false,
            slicing: false,
            sound: false,
            pivot: false,
            wind: false,
        }
    }
}

impl Flags {
    pub fn clear_all(&mut self) {
        self.bite = false;
        self.bullet = false;
        self.charge = false;
        self.contact = false;
        self.drag = false;
        self.heal = false;
        self.powder = false;
        self.protect = false;
        self.pulse = false;
        self.punch = false;
        self.recharge = false;
        self.reflectable = false;
        self.slicing = false;
        self.sound = false;
        self.pivot = false;
        self.wind = false;
    }
}

#[derive(Debug, Clone)]
pub struct Secondary {
    pub chance: f32,
    pub target: MoveTarget,
    pub effect: Effect,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    VolatileStatus(PokemonVolatileStatus),
    Boost(StatBoosts),
    Status(PokemonStatus),
    Heal(f32),
    RemoveItem,
}

#[derive(PartialEq)]
pub enum MultiHitMove {
    None,
    DoubleHit,
    TripleHit,
    TwoToFiveHits,
    PopulationBomb,
    TripleAxel,
}

#[derive(PartialEq)]
pub enum MultiAccuracyMove {
    None,
    TripleHit,
    TenHits,
}

define_enum_with_from_str! {
    #[repr(u16)]
    #[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
    Choices {
        NONE,
        ABSORB,
        ACCELEROCK,
        ACID,
        ACIDARMOR,
        ACIDSPRAY,
        ACROBATICS,
        ACUPRESSURE,
        AERIALACE,
        AEROBLAST,
        AFTERYOU,
        AGILITY,
        AIRCUTTER,
        AIRSLASH,
        ALLURINGVOICE,
        ALLYSWITCH,
        AMNESIA,
        ANCHORSHOT,
        ANCIENTPOWER,
        APPLEACID,
        AQUACUTTER,
        AQUAJET,
        AQUARING,
        AQUASTEP,
        AQUATAIL,
        ARMORCANNON,
        ARMTHRUST,
        AROMATHERAPY,
        AROMATICMIST,
        ASSIST,
        ASSURANCE,
        ASTONISH,
        ASTRALBARRAGE,
        ATTACKORDER,
        ATTRACT,
        AURASPHERE,
        AURAWHEEL,
        AURORABEAM,
        AURORAVEIL,
        AUTOTOMIZE,
        AVALANCHE,
        AXEKICK,
        BABYDOLLEYES,
        BADDYBAD,
        BANEFULBUNKER,
        BARBBARRAGE,
        BARRAGE,
        BARRIER,
        BATONPASS,
        BEAKBLAST,
        BEATUP,
        BEHEMOTHBASH,
        BEHEMOTHBLADE,
        BELCH,
        BELLYDRUM,
        BESTOW,
        BIDE,
        BIND,
        BITE,
        BITTERBLADE,
        BITTERMALICE,
        BLASTBURN,
        BLAZEKICK,
        BLAZINGTORQUE,
        BLEAKWINDSTORM,
        BLIZZARD,
        BLOCK,
        BLOODMOON,
        BLUEFLARE,
        BODYPRESS,
        BODYSLAM,
        BOLTBEAK,
        BOLTSTRIKE,
        BONECLUB,
        BONEMERANG,
        BONERUSH,
        BOOMBURST,
        BOUNCE,
        BOUNCYBUBBLE,
        BRANCHPOKE,
        BRAVEBIRD,
        BREAKINGSWIPE,
        BRICKBREAK,
        BRINE,
        BRUTALSWING,
        BUBBLE,
        BUBBLEBEAM,
        BUGBITE,
        BUGBUZZ,
        BULKUP,
        BULLDOZE,
        BULLETPUNCH,
        BULLETSEED,
        BURNINGBULWARK,
        BURNINGJEALOUSY,
        BURNUP,
        BUZZYBUZZ,
        CALMMIND,
        CAMOUFLAGE,
        CAPTIVATE,
        CEASELESSEDGE,
        CELEBRATE,
        CHARGE,
        CHARGEBEAM,
        CHARM,
        CHATTER,
        CHILLINGWATER,
        CHILLYRECEPTION,
        CHIPAWAY,
        CHLOROBLAST,
        CIRCLETHROW,
        CLAMP,
        CLANGINGSCALES,
        CLANGOROUSSOUL,
        CLEARSMOG,
        CLOSECOMBAT,
        COACHING,
        COIL,
        COLLISIONCOURSE,
        COMBATTORQUE,
        COMETPUNCH,
        COMEUPPANCE,
        CONFIDE,
        CONFUSERAY,
        CONFUSION,
        CONSTRICT,
        CONVERSION,
        CONVERSION2,
        COPYCAT,
        COREENFORCER,
        CORROSIVEGAS,
        COSMICPOWER,
        COTTONGUARD,
        COTTONSPORE,
        COUNTER,
        COURTCHANGE,
        COVET,
        CRABHAMMER,
        CRAFTYSHIELD,
        CROSSCHOP,
        CROSSPOISON,
        CRUNCH,
        CRUSHCLAW,
        CRUSHGRIP,
        CURSE,
        CUT,
        DARKESTLARIAT,
        DARKPULSE,
        DARKVOID,
        DAZZLINGGLEAM,
        DECORATE,
        DEFENDORDER,
        DEFENSECURL,
        DEFOG,
        DESTINYBOND,
        DETECT,
        DIAMONDSTORM,
        DIG,
        DIRECLAW,
        DISABLE,
        DISARMINGVOICE,
        DISCHARGE,
        DIVE,
        DIZZYPUNCH,
        DOODLE,
        DOOMDESIRE,
        DOUBLEEDGE,
        DOUBLEHIT,
        DOUBLEIRONBASH,
        DOUBLEKICK,
        DOUBLESHOCK,
        DOUBLESLAP,
        DOUBLETEAM,
        DRACOMETEOR,
        DRAGONASCENT,
        DRAGONBREATH,
        DRAGONCHEER,
        DRAGONCLAW,
        DRAGONDANCE,
        DRAGONDARTS,
        DRAGONENERGY,
        DRAGONHAMMER,
        DRAGONPULSE,
        DRAGONRAGE,
        DRAGONRUSH,
        DRAGONTAIL,
        DRAININGKISS,
        DRAINPUNCH,
        DREAMEATER,
        DRILLPECK,
        DRILLRUN,
        DRUMBEATING,
        DUALCHOP,
        DUALWINGBEAT,
        DYNAMAXCANNON,
        DYNAMICPUNCH,
        EARTHPOWER,
        EARTHQUAKE,
        ECHOEDVOICE,
        EERIEIMPULSE,
        EERIESPELL,
        EGGBOMB,
        ELECTRICTERRAIN,
        ELECTRIFY,
        ELECTROBALL,
        ELECTRODRIFT,
        ELECTROSHOT,
        ELECTROWEB,
        EMBARGO,
        EMBER,
        ENCORE,
        ENDEAVOR,
        ENDURE,
        ENERGYBALL,
        ENTRAINMENT,
        ERUPTION,
        ESPERWING,
        ETERNABEAM,
        EXPANDINGFORCE,
        EXPLOSION,
        EXTRASENSORY,
        EXTREMESPEED,
        FACADE,
        FAIRYLOCK,
        FAIRYWIND,
        FAKEOUT,
        FAKETEARS,
        FALSESURRENDER,
        FALSESWIPE,
        FEATHERDANCE,
        FEINT,
        FEINTATTACK,
        FELLSTINGER,
        FICKLEBEAM,
        FIERYDANCE,
        FIERYWRATH,
        FILLETAWAY,
        FINALGAMBIT,
        FIREBLAST,
        FIREFANG,
        FIRELASH,
        FIREPLEDGE,
        FIREPUNCH,
        FIRESPIN,
        FIRSTIMPRESSION,
        FISHIOUSREND,
        FISSURE,
        FLAIL,
        FLAMEBURST,
        FLAMECHARGE,
        FLAMETHROWER,
        FLAMEWHEEL,
        FLAREBLITZ,
        FLASH,
        FLASHCANNON,
        FLATTER,
        FLEURCANNON,
        FLING,
        FLIPTURN,
        FLOATYFALL,
        FLORALHEALING,
        FLOWERSHIELD,
        FLOWERTRICK,
        FLY,
        FLYINGPRESS,
        FOCUSBLAST,
        FOCUSENERGY,
        FOCUSPUNCH,
        FOLLOWME,
        FORCEPALM,
        FORESIGHT,
        FORESTSCURSE,
        FOULPLAY,
        FREEZEDRY,
        FREEZESHOCK,
        FREEZINGGLARE,
        FREEZYFROST,
        FRENZYPLANT,
        FROSTBREATH,
        FRUSTRATION,
        FURYATTACK,
        FURYCUTTER,
        FURYSWIPES,
        FUSIONBOLT,
        FUSIONFLARE,
        FUTURESIGHT,
        GASTROACID,
        GEARGRIND,
        GEARUP,
        GEOMANCY,
        GIGADRAIN,
        GIGAIMPACT,
        GIGATONHAMMER,
        GLACIALLANCE,
        GLACIATE,
        GLAIVERUSH,
        GLARE,
        GLITZYGLOW,
        GRASSKNOT,
        GRASSPLEDGE,
        GRASSWHISTLE,
        GRASSYGLIDE,
        GRASSYTERRAIN,
        GRAVAPPLE,
        GRAVITY,
        GROWL,
        GROWTH,
        GRUDGE,
        GUARDSPLIT,
        GUARDSWAP,
        GUILLOTINE,
        GUNKSHOT,
        GUST,
        GYROBALL,
        HAIL,
        HAMMERARM,
        HAPPYHOUR,
        HARDEN,
        HARDPRESS,
        HAZE,
        HEADBUTT,
        HEADCHARGE,
        HEADLONGRUSH,
        HEADSMASH,
        HEALBELL,
        HEALBLOCK,
        HEALINGWISH,
        HEALORDER,
        HEALPULSE,
        HEARTSTAMP,
        HEARTSWAP,
        HEATCRASH,
        HEATWAVE,
        HEAVYSLAM,
        HELPINGHAND,
        HEX,
        HIDDENPOWER,
        HIDDENPOWERBUG60,
        HIDDENPOWERBUG70,
        HIDDENPOWERDARK60,
        HIDDENPOWERDARK70,
        HIDDENPOWERDRAGON60,
        HIDDENPOWERDRAGON70,
        HIDDENPOWERELECTRIC60,
        HIDDENPOWERELECTRIC70,
        HIDDENPOWERFIGHTING60,
        HIDDENPOWERFIGHTING70,
        HIDDENPOWERFIRE60,
        HIDDENPOWERFIRE70,
        HIDDENPOWERFLYING60,
        HIDDENPOWERFLYING70,
        HIDDENPOWERGHOST60,
        HIDDENPOWERGHOST70,
        HIDDENPOWERGRASS60,
        HIDDENPOWERGRASS70,
        HIDDENPOWERGROUND60,
        HIDDENPOWERGROUND70,
        HIDDENPOWERICE60,
        HIDDENPOWERICE70,
        HIDDENPOWERPOISON60,
        HIDDENPOWERPOISON70,
        HIDDENPOWERPSYCHIC60,
        HIDDENPOWERPSYCHIC70,
        HIDDENPOWERROCK60,
        HIDDENPOWERROCK70,
        HIDDENPOWERSTEEL60,
        HIDDENPOWERSTEEL70,
        HIDDENPOWERWATER60,
        HIDDENPOWERWATER70,
        HIGHHORSEPOWER,
        HIGHJUMPKICK,
        HOLDBACK,
        HOLDHANDS,
        HONECLAWS,
        HORNATTACK,
        HORNDRILL,
        HORNLEECH,
        HOWL,
        HURRICANE,
        HYDROCANNON,
        HYDROPUMP,
        HYDROSTEAM,
        HYPERBEAM,
        HYPERDRILL,
        HYPERFANG,
        HYPERSPACEFURY,
        HYPERSPACEHOLE,
        HYPERVOICE,
        HYPNOSIS,
        ICEBALL,
        ICEBEAM,
        ICEBURN,
        ICEFANG,
        ICEHAMMER,
        ICEPUNCH,
        ICESHARD,
        ICESPINNER,
        ICICLECRASH,
        ICICLESPEAR,
        ICYWIND,
        IMPRISON,
        INCINERATE,
        INFERNALPARADE,
        INFERNO,
        INFESTATION,
        INGRAIN,
        INSTRUCT,
        IONDELUGE,
        IRONDEFENSE,
        IRONHEAD,
        IRONTAIL,
        IVYCUDGEL,
        JAWLOCK,
        JETPUNCH,
        JUDGMENT,
        JUMPKICK,
        JUNGLEHEALING,
        KARATECHOP,
        KINESIS,
        KINGSSHIELD,
        KNOCKOFF,
        KOWTOWCLEAVE,
        LANDSWRATH,
        LASERFOCUS,
        LASHOUT,
        LASTRESORT,
        LASTRESPECTS,
        LAVAPLUME,
        LEAFAGE,
        LEAFBLADE,
        LEAFSTORM,
        LEAFTORNADO,
        LEECHLIFE,
        LEECHSEED,
        LEER,
        LICK,
        LIFEDEW,
        LIGHTOFRUIN,
        LIGHTSCREEN,
        LIQUIDATION,
        LOCKON,
        LOVELYKISS,
        LOWKICK,
        LOWSWEEP,
        LUCKYCHANT,
        LUMINACRASH,
        LUNARBLESSING,
        LUNARDANCE,
        LUNGE,
        LUSTERPURGE,
        MACHPUNCH,
        MAGICALLEAF,
        MAGICALTORQUE,
        MAGICCOAT,
        MAGICPOWDER,
        MAGICROOM,
        MAGMASTORM,
        MAGNETBOMB,
        MAGNETICFLUX,
        MAGNETRISE,
        MAGNITUDE,
        MAKEITRAIN,
        MALIGNANTCHAIN,
        MATBLOCK,
        MATCHAGOTCHA,
        MEANLOOK,
        MEDITATE,
        MEFIRST,
        MEGADRAIN,
        MEGAHORN,
        MEGAKICK,
        MEGAPUNCH,
        MEMENTO,
        METALBURST,
        METALCLAW,
        METALSOUND,
        METEORASSAULT,
        METEORBEAM,
        METEORMASH,
        METRONOME,
        MIGHTYCLEAVE,
        MILKDRINK,
        MIMIC,
        MINDBLOWN,
        MINDREADER,
        MINIMIZE,
        MIRACLEEYE,
        MIRRORCOAT,
        MIRRORMOVE,
        MIRRORSHOT,
        MIST,
        MISTBALL,
        MISTYEXPLOSION,
        MISTYTERRAIN,
        MOONBLAST,
        MOONGEISTBEAM,
        MOONLIGHT,
        MORNINGSUN,
        MORTALSPIN,
        MOUNTAINGALE,
        MUDBOMB,
        MUDDYWATER,
        MUDSHOT,
        MUDSLAP,
        MUDSPORT,
        MULTIATTACK,
        MYSTICALFIRE,
        MYSTICALPOWER,
        NASTYPLOT,
        NATURALGIFT,
        NATUREPOWER,
        NATURESMADNESS,
        NEEDLEARM,
        NIGHTDAZE,
        NIGHTMARE,
        NIGHTSHADE,
        NIGHTSLASH,
        NOBLEROAR,
        NORETREAT,
        NOTHING,
        NOXIOUSTORQUE,
        NUZZLE,
        OBLIVIONWING,
        OBSTRUCT,
        OCTAZOOKA,
        OCTOLOCK,
        ODORSLEUTH,
        OMINOUSWIND,
        ORDERUP,
        ORIGINPULSE,
        OUTRAGE,
        OVERDRIVE,
        OVERHEAT,
        PAINSPLIT,
        PALEOWAVE,
        PARABOLICCHARGE,
        PARTINGSHOT,
        PAYBACK,
        PAYDAY,
        PECK,
        PERISHSONG,
        PETALBLIZZARD,
        PETALDANCE,
        PHANTOMFORCE,
        PHOTONGEYSER,
        PIKAPAPOW,
        PINMISSILE,
        PLASMAFISTS,
        PLAYNICE,
        PLAYROUGH,
        PLUCK,
        POISONFANG,
        POISONGAS,
        POISONJAB,
        POISONPOWDER,
        POISONSTING,
        POISONTAIL,
        POLLENPUFF,
        POLTERGEIST,
        POPULATIONBOMB,
        POUNCE,
        POUND,
        POWDER,
        POWDERSNOW,
        POWERGEM,
        POWERSHIFT,
        POWERSPLIT,
        POWERSWAP,
        POWERTRICK,
        POWERTRIP,
        POWERUPPUNCH,
        POWERWHIP,
        PRECIPICEBLADES,
        PRESENT,
        PRISMATICLASER,
        PROTECT,
        PSYBEAM,
        PSYBLADE,
        PSYCHIC,
        PSYCHICFANGS,
        PSYCHICNOISE,
        PSYCHICTERRAIN,
        PSYCHOBOOST,
        PSYCHOCUT,
        PSYCHOSHIFT,
        PSYCHUP,
        PSYSHIELDBASH,
        PSYSHOCK,
        PSYSTRIKE,
        PSYWAVE,
        PUNISHMENT,
        PURIFY,
        PURSUIT,
        PYROBALL,
        QUASH,
        QUICKATTACK,
        QUICKGUARD,
        QUIVERDANCE,
        RAGE,
        RAGEFIST,
        RAGEPOWDER,
        RAGINGBULL,
        RAGINGFURY,
        RAINDANCE,
        RAPIDSPIN,
        RAZORLEAF,
        RAZORSHELL,
        RAZORWIND,
        RECHARGE,
        RECOVER,
        RECYCLE,
        REFLECT,
        REFLECTTYPE,
        REFRESH,
        RELICSONG,
        REST,
        RETALIATE,
        RETURN,
        RETURN102,
        REVELATIONDANCE,
        REVENGE,
        REVERSAL,
        REVIVALBLESSING,
        RISINGVOLTAGE,
        ROAR,
        ROAROFTIME,
        ROCKBLAST,
        ROCKCLIMB,
        ROCKPOLISH,
        ROCKSLIDE,
        ROCKSMASH,
        ROCKTHROW,
        ROCKTOMB,
        ROCKWRECKER,
        ROLEPLAY,
        ROLLINGKICK,
        ROLLOUT,
        ROOST,
        ROTOTILLER,
        ROUND,
        RUINATION,
        SACREDFIRE,
        SACREDSWORD,
        SAFEGUARD,
        SALTCURE,
        SANDATTACK,
        SANDSEARSTORM,
        SANDSTORM,
        SANDTOMB,
        SAPPYSEED,
        SCALD,
        SCALESHOT,
        SCARYFACE,
        SCORCHINGSANDS,
        SCRATCH,
        SCREECH,
        SEARINGSHOT,
        SECRETPOWER,
        SECRETSWORD,
        SEEDBOMB,
        SEEDFLARE,
        SEISMICTOSS,
        SELFDESTRUCT,
        SHADOWBALL,
        SHADOWBONE,
        SHADOWCLAW,
        SHADOWFORCE,
        SHADOWPUNCH,
        SHADOWSNEAK,
        SHADOWSTRIKE,
        SHARPEN,
        SHEDTAIL,
        SHEERCOLD,
        SHELLSIDEARM,
        SHELLSMASH,
        SHELLTRAP,
        SHELTER,
        SHIFTGEAR,
        SHOCKWAVE,
        SHOREUP,
        SIGNALBEAM,
        SILKTRAP,
        SILVERWIND,
        SIMPLEBEAM,
        SING,
        SIZZLYSLIDE,
        SKETCH,
        SKILLSWAP,
        SKITTERSMACK,
        SKULLBASH,
        SKYATTACK,
        SKYDROP,
        SKYUPPERCUT,
        SLACKOFF,
        SLAM,
        SLASH,
        SLEEPPOWDER,
        SLEEPTALK,
        SLUDGE,
        SLUDGEBOMB,
        SLUDGEWAVE,
        SMACKDOWN,
        SMARTSTRIKE,
        SMELLINGSALTS,
        SMOG,
        SMOKESCREEN,
        SNAPTRAP,
        SNARL,
        SNATCH,
        SNIPESHOT,
        SNORE,
        SNOWSCAPE,
        SOAK,
        SOFTBOILED,
        SOLARBEAM,
        SOLARBLADE,
        SONICBOOM,
        SPACIALREND,
        SPARK,
        SPARKLINGARIA,
        SPARKLYSWIRL,
        SPECTRALTHIEF,
        SPEEDSWAP,
        SPICYEXTRACT,
        SPIDERWEB,
        SPIKECANNON,
        SPIKES,
        SPIKYSHIELD,
        SPINOUT,
        SPIRITBREAK,
        SPIRITSHACKLE,
        SPITE,
        SPITUP,
        SPLASH,
        SPLISHYSPLASH,
        SPORE,
        SPOTLIGHT,
        SPRINGTIDESTORM,
        STEALTHROCK,
        STEAMERUPTION,
        STEAMROLLER,
        STEELBEAM,
        STEELROLLER,
        STEELWING,
        STICKYWEB,
        STOCKPILE,
        STOMP,
        STOMPINGTANTRUM,
        STONEAXE,
        STONEEDGE,
        STOREDPOWER,
        STORMTHROW,
        STRANGESTEAM,
        STRENGTH,
        STRENGTHSAP,
        STRINGSHOT,
        STRUGGLE,
        STRUGGLEBUG,
        STUFFCHEEKS,
        STUNSPORE,
        SUBMISSION,
        SUBSTITUTE,
        SUCKERPUNCH,
        SUNNYDAY,
        SUNSTEELSTRIKE,
        SUPERCELLSLAM,
        SUPERFANG,
        SUPERPOWER,
        SUPERSONIC,
        SURF,
        SURGINGSTRIKES,
        SWAGGER,
        SWALLOW,
        SWEETKISS,
        SWEETSCENT,
        SWIFT,
        SWITCHEROO,
        SWORDSDANCE,
        SYNCHRONOISE,
        SYNTHESIS,
        SYRUPBOMB,
        TACHYONCUTTER,
        TACKLE,
        TAILGLOW,
        TAILSLAP,
        TAILWHIP,
        TAILWIND,
        TAKEDOWN,
        TAKEHEART,
        TARSHOT,
        TAUNT,
        TEARFULLOOK,
        TEATIME,
        TECHNOBLAST,
        TEETERDANCE,
        TELEKINESIS,
        TELEPORT,
        TEMPERFLARE,
        TERABLAST,
        TERASTARSTORM,
        TERRAINPULSE,
        THIEF,
        THOUSANDARROWS,
        THOUSANDWAVES,
        THRASH,
        THROATCHOP,
        THUNDER,
        THUNDERBOLT,
        THUNDERCAGE,
        THUNDERCLAP,
        THUNDERFANG,
        THUNDEROUSKICK,
        THUNDERPUNCH,
        THUNDERSHOCK,
        THUNDERWAVE,
        TICKLE,
        TIDYUP,
        TOPSYTURVY,
        TORCHSONG,
        TORMENT,
        TOXIC,
        TOXICSPIKES,
        TOXICTHREAD,
        TRAILBLAZE,
        TRANSFORM,
        TRIATTACK,
        TRICK,
        TRICKORTREAT,
        TRICKROOM,
        TRIPLEARROWS,
        TRIPLEAXEL,
        TRIPLEDIVE,
        TRIPLEKICK,
        TROPKICK,
        TRUMPCARD,
        TWINBEAM,
        TWINEEDLE,
        TWISTER,
        UPPERHAND,
        UPROAR,
        UTURN,
        VACUUMWAVE,
        VCREATE,
        VEEVEEVOLLEY,
        VENOMDRENCH,
        VENOSHOCK,
        VICTORYDANCE,
        VINEWHIP,
        VISEGRIP,
        VITALTHROW,
        VOLTSWITCH,
        VOLTTACKLE,
        WAKEUPSLAP,
        WATERFALL,
        WATERGUN,
        WATERPLEDGE,
        WATERPULSE,
        WATERSHURIKEN,
        WATERSPORT,
        WATERSPOUT,
        WAVECRASH,
        WEATHERBALL,
        WHIRLPOOL,
        WHIRLWIND,
        WICKEDBLOW,
        WICKEDTORQUE,
        WIDEGUARD,
        WILDBOLTSTORM,
        WILDCHARGE,
        WILLOWISP,
        WINGATTACK,
        WISH,
        WITHDRAW,
        WONDERROOM,
        WOODHAMMER,
        WORKUP,
        WORRYSEED,
        WRAP,
        WRINGOUT,
        XSCISSOR,
        YAWN,
        ZAPCANNON,
        ZENHEADBUTT,
        ZINGZAP,
        ZIPPYZAP,
    },
    default = NONE
}

impl Choices {
    pub fn is_hiddenpower(&self) -> bool {
        match self {
            Choices::HIDDENPOWER
            | Choices::HIDDENPOWERBUG60
            | Choices::HIDDENPOWERBUG70
            | Choices::HIDDENPOWERDARK60
            | Choices::HIDDENPOWERDARK70
            | Choices::HIDDENPOWERDRAGON60
            | Choices::HIDDENPOWERDRAGON70
            | Choices::HIDDENPOWERELECTRIC60
            | Choices::HIDDENPOWERELECTRIC70
            | Choices::HIDDENPOWERFIGHTING60
            | Choices::HIDDENPOWERFIGHTING70
            | Choices::HIDDENPOWERFIRE60
            | Choices::HIDDENPOWERFIRE70
            | Choices::HIDDENPOWERFLYING60
            | Choices::HIDDENPOWERFLYING70
            | Choices::HIDDENPOWERGHOST60
            | Choices::HIDDENPOWERGHOST70
            | Choices::HIDDENPOWERGRASS60
            | Choices::HIDDENPOWERGRASS70
            | Choices::HIDDENPOWERGROUND60
            | Choices::HIDDENPOWERGROUND70
            | Choices::HIDDENPOWERICE60
            | Choices::HIDDENPOWERICE70
            | Choices::HIDDENPOWERPOISON60
            | Choices::HIDDENPOWERPOISON70
            | Choices::HIDDENPOWERPSYCHIC60
            | Choices::HIDDENPOWERPSYCHIC70
            | Choices::HIDDENPOWERROCK60
            | Choices::HIDDENPOWERROCK70
            | Choices::HIDDENPOWERSTEEL60
            | Choices::HIDDENPOWERSTEEL70
            | Choices::HIDDENPOWERWATER60
            | Choices::HIDDENPOWERWATER70 => true,
            _ => false,
        }
    }
    pub fn increased_crit_ratio(&self) -> bool {
        match self {
            Choices::AEROBLAST
            | Choices::AIRCUTTER
            | Choices::AQUACUTTER
            | Choices::ATTACKORDER
            | Choices::BLAZEKICK
            | Choices::CRABHAMMER
            | Choices::CROSSCHOP
            | Choices::CROSSPOISON
            | Choices::DIRECLAW
            | Choices::DRILLRUN
            | Choices::ESPERWING
            | Choices::IVYCUDGEL
            | Choices::KARATECHOP
            | Choices::LEAFBLADE
            | Choices::NIGHTSLASH
            | Choices::POISONTAIL
            | Choices::PSYCHOCUT
            | Choices::RAZORLEAF
            | Choices::RAZORWIND
            | Choices::SHADOWCLAW
            | Choices::SKYATTACK
            | Choices::SLASH
            | Choices::SNIPESHOT
            | Choices::SPACIALREND
            | Choices::STONEEDGE
            | Choices::TRIPLEARROWS => true,
            _ => false,
        }
    }
    pub fn guaranteed_crit(&self) -> bool {
        match self {
            Choices::WICKEDBLOW
            | Choices::SURGINGSTRIKES
            | Choices::FLOWERTRICK
            | Choices::STORMTHROW
            | Choices::FROSTBREATH => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Choice {
    // Basic move information
    pub move_id: Choices, // in the case of category::Switch, this is not used
    pub move_index: PokemonMoveIndex,
    pub switch_id: PokemonIndex,
    pub move_type: PokemonType,
    pub accuracy: f32,
    pub category: MoveCategory,
    pub base_power: f32,
    pub boost: Option<Boost>,
    pub priority: i8,
    pub flags: Flags,
    pub drain: Option<f32>,
    pub recoil: Option<f32>,
    pub crash: Option<f32>,
    pub heal: Option<Heal>,
    pub status: Option<Status>,
    pub volatile_status: Option<VolatileStatus>,
    pub side_condition: Option<SideCondition>,
    pub secondaries: Option<Vec<Secondary>>,

    pub target: MoveTarget,

    pub first_move: bool,
    pub sleep_talk_move: bool,
}

impl fmt::Debug for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Choice: {:?}", self.move_id)
    }
}

impl Choice {
    pub fn multi_accuracy(&self) -> MultiAccuracyMove {
        match self.move_id {
            Choices::TRIPLEAXEL => MultiAccuracyMove::TripleHit,
            Choices::TRIPLEKICK => MultiAccuracyMove::TripleHit,
            Choices::POPULATIONBOMB => MultiAccuracyMove::TenHits,
            _ => MultiAccuracyMove::None,
        }
    }
    pub fn multi_hit(&self) -> MultiHitMove {
        match self.move_id {
            Choices::ARMTHRUST => MultiHitMove::TwoToFiveHits,
            Choices::BARRAGE => MultiHitMove::TwoToFiveHits,
            Choices::BONEMERANG => MultiHitMove::DoubleHit,
            Choices::BONERUSH => MultiHitMove::TwoToFiveHits,
            Choices::BULLETSEED => MultiHitMove::TwoToFiveHits,
            Choices::COMETPUNCH => MultiHitMove::TwoToFiveHits,
            Choices::DOUBLEHIT => MultiHitMove::DoubleHit,
            Choices::DOUBLEIRONBASH => MultiHitMove::DoubleHit,
            Choices::DOUBLEKICK => MultiHitMove::DoubleHit,
            Choices::DOUBLESLAP => MultiHitMove::TwoToFiveHits,
            Choices::DRAGONDARTS => MultiHitMove::DoubleHit,
            Choices::DUALCHOP => MultiHitMove::DoubleHit,
            Choices::DUALWINGBEAT => MultiHitMove::DoubleHit,
            Choices::FURYATTACK => MultiHitMove::TwoToFiveHits,
            Choices::FURYSWIPES => MultiHitMove::TwoToFiveHits,
            Choices::GEARGRIND => MultiHitMove::DoubleHit,
            Choices::ICICLESPEAR => MultiHitMove::TwoToFiveHits,
            Choices::PINMISSILE => MultiHitMove::TwoToFiveHits,
            Choices::ROCKBLAST => MultiHitMove::TwoToFiveHits,
            Choices::SCALESHOT => MultiHitMove::TwoToFiveHits,
            Choices::SPIKECANNON => MultiHitMove::TwoToFiveHits,
            Choices::SURGINGSTRIKES => MultiHitMove::TripleHit,
            Choices::TACHYONCUTTER => MultiHitMove::DoubleHit,
            Choices::TAILSLAP => MultiHitMove::TwoToFiveHits,
            Choices::TRIPLEDIVE => MultiHitMove::TripleHit,
            Choices::TWINBEAM => MultiHitMove::DoubleHit,
            Choices::TWINEEDLE => MultiHitMove::DoubleHit,
            Choices::WATERSHURIKEN => MultiHitMove::TwoToFiveHits,

            // These are multi-accuracy
            // but until that is implemented we approximate them as multi-hit
            Choices::POPULATIONBOMB => MultiHitMove::PopulationBomb,
            Choices::TRIPLEAXEL => MultiHitMove::TripleAxel,
            _ => MultiHitMove::None,
        }
    }
    pub fn targets_special_defense(&self) -> bool {
        self.category == MoveCategory::Special
            && !(self.move_id == Choices::PSYSHOCK
                || self.move_id == Choices::SECRETSWORD
                || self.move_id == Choices::PSYSTRIKE)
    }
    pub fn add_or_create_secondaries(&mut self, secondary: Secondary) {
        if let Some(secondaries) = &mut self.secondaries {
            secondaries.push(secondary);
        } else {
            self.secondaries = Some(vec![secondary]);
        }
    }
    pub fn remove_effects_for_protect(&mut self) {
        // Crash is not removed

        self.base_power = 0.0;
        self.category = MoveCategory::Status;
        self.accuracy = 100.0;
        self.flags.drag = false;
        self.flags.pivot = false;
        self.heal = None;
        self.drain = None;
        self.recoil = None;
        self.boost = None;
        self.status = None;
        self.volatile_status = None;
        self.side_condition = None;
        self.secondaries = None;
    }
    pub fn remove_all_effects(&mut self) {
        self.category = MoveCategory::Status;
        self.flags.clear_all();
        self.base_power = 0.0;
        self.accuracy = 100.0;
        self.heal = None;
        self.drain = None;
        self.recoil = None;
        self.crash = None;
        self.heal = None;
        self.boost = None;
        self.status = None;
        self.volatile_status = None;
        self.side_condition = None;
        self.secondaries = None;
    }
}

impl Default for Choice {
    fn default() -> Choice {
        Choice {
            move_id: Choices::NONE,
            move_index: PokemonMoveIndex::M0,
            switch_id: PokemonIndex::P0,
            move_type: PokemonType::NORMAL,
            accuracy: 100.0,
            category: MoveCategory::Status,
            base_power: 0.0,
            boost: None,
            priority: 0,
            flags: Flags {
                ..Default::default()
            },
            drain: None,
            recoil: None,
            crash: None,
            heal: None,
            status: None,
            volatile_status: None,
            side_condition: None,
            secondaries: None,
            target: MoveTarget::Opponent,
            first_move: true,
            sleep_talk_move: false,
        }
    }
}

pub fn undo_physical_special_split(moves: &mut HashMap<Choices, Choice>) {
    for (_, choice) in moves.iter_mut() {
        if choice.category == MoveCategory::Status {
            continue;
        }
        match choice.move_type {
            PokemonType::NORMAL
            | PokemonType::FIGHTING
            | PokemonType::POISON
            | PokemonType::GROUND
            | PokemonType::FLYING
            | PokemonType::BUG
            | PokemonType::ROCK
            | PokemonType::GHOST
            | PokemonType::STEEL => {
                choice.category = MoveCategory::Physical;
            }
            _ => {
                choice.category = MoveCategory::Special;
            }
        }
    }
}
