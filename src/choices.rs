use crate::instruction::ApplyVolatileStatusInstruction;
use crate::instruction::ChangeItemInstruction;
use crate::instruction::ChangeSideConditionInstruction;
use crate::instruction::ChangeTerrain;
use crate::instruction::DamageInstruction;
use crate::instruction::Instruction;
use crate::instruction::SetSubstituteHealthInstruction;
use crate::state::PokemonStatus;
use crate::state::PokemonType;
use crate::state::PokemonVolatileStatus;
use crate::state::SideReference;
use crate::state::State;
use crate::state::Terrain;
use crate::state::{PokemonBoostableStat, PokemonSideCondition};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type ModifyChoiceFn = fn(&State, &mut Choice, &Choice, &SideReference);
pub type AfterDamageHitFn = fn(&State, &Choice, &SideReference) -> Vec<Instruction>;
pub type HazardClearFn = fn(&State, &SideReference) -> Vec<Instruction>;
pub type MoveSpecialEffectFn = fn(&State, &SideReference) -> Vec<Instruction>;

lazy_static! {
    pub static ref MOVES: HashMap<String, Choice> = {
        let mut moves: HashMap<String, Choice> = HashMap::new();

        moves.insert(
            String::from("absorb"),
            Choice {
                move_id: String::from("absorb"),
                base_power: 20.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("accelerock"),
            Choice {
                move_id: String::from("accelerock"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Rock,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("acid"),
            Choice {
                move_id: String::from("acid"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
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
            String::from("acidarmor"),
            Choice {
                move_id: String::from("acidarmor"),
                target: MoveTarget::User,
                move_type: PokemonType::Poison,
                flags: Flags {
                    snatch: true,
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
            String::from("acidspray"),
            Choice {
                move_id: String::from("acidspray"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    bullet: true,
                    mirror: true,
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
            String::from("acrobatics"),
            Choice {
                move_id: String::from("acrobatics"),
                base_power: 110.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("acupressure"),
            Choice {
                move_id: String::from("acupressure"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aerialace"),
            Choice {
                move_id: String::from("aerialace"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aeroblast"),
            Choice {
                move_id: String::from("aeroblast"),
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("afteryou"),
            Choice {
                move_id: String::from("afteryou"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("agility"),
            Choice {
                move_id: String::from("agility"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
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
            String::from("aircutter"),
            Choice {
                move_id: String::from("aircutter"),
                accuracy: 95.0,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("airslash"),
            Choice {
                move_id: String::from("airslash"),
                accuracy: 95.0,
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("allyswitch"),
            Choice {
                move_id: String::from("allyswitch"),
                priority: 2,
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("amnesia"),
            Choice {
                move_id: String::from("amnesia"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
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
        moves.insert(
            String::from("anchorshot"),
            Choice {
                move_id: String::from("anchorshot"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ancientpower"),
            Choice {
                move_id: String::from("ancientpower"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
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
            String::from("appleacid"),
            Choice {
                move_id: String::from("appleacid"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
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
            String::from("aquacutter"),
            Choice {
                move_id: String::from("aquacutter"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aquajet"),
            Choice {
                move_id: String::from("aquajet"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aquaring"),
            Choice {
                move_id: String::from("aquaring"),
                target: MoveTarget::User,
                move_type: PokemonType::Water,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::AquaRing,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aquastep"),
            Choice {
                move_id: String::from("aquastep"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    dance: true,
                    mirror: true,
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
            String::from("aquatail"),
            Choice {
                move_id: String::from("aquatail"),
                accuracy: 90.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("armorcannon"),
            Choice {
                move_id: String::from("armorcannon"),
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("armthrust"),
            Choice {
                move_id: String::from("armthrust"),
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aromatherapy"),
            Choice {
                move_id: String::from("aromatherapy"),
                target: MoveTarget::User,
                move_type: PokemonType::Grass,
                flags: Flags {
                    distance: true,
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aromaticmist"),
            Choice {
                move_id: String::from("aromaticmist"),
                target: MoveTarget::User,
                move_type: PokemonType::Fairy,
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
            String::from("assist"),
            Choice {
                move_id: String::from("assist"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("assurance"),
            Choice {
                move_id: String::from("assurance"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("astonish"),
            Choice {
                move_id: String::from("astonish"),
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("astralbarrage"),
            Choice {
                move_id: String::from("astralbarrage"),
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("attackorder"),
            Choice {
                move_id: String::from("attackorder"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("attract"),
            Choice {
                move_id: String::from("attract"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Attract,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aurasphere"),
            Choice {
                move_id: String::from("aurasphere"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    bullet: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("aurawheel"),
            Choice {
                move_id: String::from("aurawheel"),
                base_power: 110.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
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
            String::from("aurorabeam"),
            Choice {
                move_id: String::from("aurorabeam"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
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
            String::from("auroraveil"),
            Choice {
                move_id: String::from("auroraveil"),
                target: MoveTarget::User,
                move_type: PokemonType::Ice,
                flags: Flags {
                    snatch: true,
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
            String::from("autotomize"),
            Choice {
                move_id: String::from("autotomize"),
                target: MoveTarget::User,
                move_type: PokemonType::Steel,
                flags: Flags {
                    snatch: true,
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
            String::from("avalanche"),
            Choice {
                move_id: String::from("avalanche"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: -4,
                move_type: PokemonType::Ice,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("axekick"),
            Choice {
                move_id: String::from("axekick"),
                accuracy: 90.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                crash: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("babydolleyes"),
            Choice {
                move_id: String::from("babydolleyes"),
                priority: 1,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
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
            String::from("baddybad"),
            Choice {
                move_id: String::from("baddybad"),
                accuracy: 95.0,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("banefulbunker"),
            Choice {
                move_id: String::from("banefulbunker"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Poison,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::BanefulBunker,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("barbbarrage"),
            Choice {
                move_id: String::from("barbbarrage"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("barrage"),
            Choice {
                move_id: String::from("barrage"),
                accuracy: 85.0,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("barrier"),
            Choice {
                move_id: String::from("barrier"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
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
            String::from("batonpass"),
            Choice {
                move_id: String::from("batonpass"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("beakblast"),
            Choice {
                move_id: String::from("beakblast"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                priority: -3,
                move_type: PokemonType::Flying,
                flags: Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("beatup"),
            Choice {
                move_id: String::from("beatup"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("behemothbash"),
            Choice {
                move_id: String::from("behemothbash"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("behemothblade"),
            Choice {
                move_id: String::from("behemothblade"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("belch"),
            Choice {
                move_id: String::from("belch"),
                accuracy: 90.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bellydrum"),
            Choice {
                move_id: String::from("bellydrum"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 6,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                }),
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bestow"),
            Choice {
                move_id: String::from("bestow"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bide"),
            Choice {
                move_id: String::from("bide"),
                category: MoveCategory::Physical,
                priority: 1,
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Bide,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bind"),
            Choice {
                move_id: String::from("bind"),
                accuracy: 85.0,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bite"),
            Choice {
                move_id: String::from("bite"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bitterblade"),
            Choice {
                move_id: String::from("bitterblade"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bittermalice"),
            Choice {
                move_id: String::from("bittermalice"),
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
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
            String::from("blastburn"),
            Choice {
                move_id: String::from("blastburn"),
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("blazekick"),
            Choice {
                move_id: String::from("blazekick"),
                accuracy: 90.0,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("blazingtorque"),
            Choice {
                move_id: String::from("blazingtorque"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bleakwindstorm"),
            Choice {
                move_id: String::from("bleakwindstorm"),
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    mirror: true,
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
                        speed: -1,
                        accuracy: 0,
                    }),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("blizzard"),
            Choice {
                move_id: String::from("blizzard"),
                accuracy: 70.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Freeze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("block"),
            Choice {
                move_id: String::from("block"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bloodmoon"),
            Choice {
                move_id: String::from("bloodmoon"),
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("blueflare"),
            Choice {
                move_id: String::from("blueflare"),
                accuracy: 85.0,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bodypress"),
            Choice {
                move_id: String::from("bodypress"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bodyslam"),
            Choice {
                move_id: String::from("bodyslam"),
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("boltbeak"),
            Choice {
                move_id: String::from("boltbeak"),
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("boltstrike"),
            Choice {
                move_id: String::from("boltstrike"),
                accuracy: 85.0,
                base_power: 130.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("boneclub"),
            Choice {
                move_id: String::from("boneclub"),
                accuracy: 85.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bonemerang"),
            Choice {
                move_id: String::from("bonemerang"),
                accuracy: 90.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bonerush"),
            Choice {
                move_id: String::from("bonerush"),
                accuracy: 90.0,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("boomburst"),
            Choice {
                move_id: String::from("boomburst"),
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bounce"),
            Choice {
                move_id: String::from("bounce"),
                accuracy: 85.0,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    charge: true,
                    contact: true,
                    distance: true,
                    gravity: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bouncybubble"),
            Choice {
                move_id: String::from("bouncybubble"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("branchpoke"),
            Choice {
                move_id: String::from("branchpoke"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bravebird"),
            Choice {
                move_id: String::from("bravebird"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("breakingswipe"),
            Choice {
                move_id: String::from("breakingswipe"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("brickbreak"),
            Choice {
                move_id: String::from("brickbreak"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("brine"),
            Choice {
                move_id: String::from("brine"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("brutalswing"),
            Choice {
                move_id: String::from("brutalswing"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bubble"),
            Choice {
                move_id: String::from("bubble"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
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
        moves.insert(
            String::from("bubblebeam"),
            Choice {
                move_id: String::from("bubblebeam"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
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
        moves.insert(
            String::from("bugbite"),
            Choice {
                move_id: String::from("bugbite"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bugbuzz"),
            Choice {
                move_id: String::from("bugbuzz"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
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
            String::from("bulkup"),
            Choice {
                move_id: String::from("bulkup"),
                target: MoveTarget::User,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    snatch: true,
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
            String::from("bulldoze"),
            Choice {
                move_id: String::from("bulldoze"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
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
            String::from("bulletpunch"),
            Choice {
                move_id: String::from("bulletpunch"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("bulletseed"),
            Choice {
                move_id: String::from("bulletseed"),
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("burningjealousy"),
            Choice {
                move_id: String::from("burningjealousy"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("burnup"),
            Choice {
                move_id: String::from("burnup"),
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("buzzybuzz"),
            Choice {
                move_id: String::from("buzzybuzz"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("calmmind"),
            Choice {
                move_id: String::from("calmmind"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
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
            String::from("camouflage"),
            Choice {
                move_id: String::from("camouflage"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("captivate"),
            Choice {
                move_id: String::from("captivate"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("ceaselessedge"),
            Choice {
                move_id: String::from("ceaselessedge"),
                accuracy: 90.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
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
            String::from("celebrate"),
            Choice {
                move_id: String::from("celebrate"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("charge"),
            Choice {
                move_id: String::from("charge"),
                target: MoveTarget::User,
                move_type: PokemonType::Electric,
                flags: Flags {
                    snatch: true,
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
                    volatile_status: PokemonVolatileStatus::Charge,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("chargebeam"),
            Choice {
                move_id: String::from("chargebeam"),
                accuracy: 90.0,
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
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
            String::from("charm"),
            Choice {
                move_id: String::from("charm"),
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
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
        moves.insert(
            String::from("chatter"),
            Choice {
                move_id: String::from("chatter"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    distance: true,
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("chillingwater"),
            Choice {
                move_id: String::from("chillingwater"),
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
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
            String::from("chillyreception"),
            Choice {
                move_id: String::from("chillyreception"),
                move_type: PokemonType::Ice,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("chipaway"),
            Choice {
                move_id: String::from("chipaway"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("chloroblast"),
            Choice {
                move_id: String::from("chloroblast"),
                accuracy: 95.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
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
            String::from("circlethrow"),
            Choice {
                move_id: String::from("circlethrow"),
                accuracy: 90.0,
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: -6,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    drag: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("clamp"),
            Choice {
                move_id: String::from("clamp"),
                accuracy: 85.0,
                base_power: 35.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("clangingscales"),
            Choice {
                move_id: String::from("clangingscales"),
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("clangoroussoul"),
            Choice {
                move_id: String::from("clangoroussoul"),
                target: MoveTarget::User,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    dance: true,
                    snatch: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("clearsmog"),
            Choice {
                move_id: String::from("clearsmog"),
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("closecombat"),
            Choice {
                move_id: String::from("closecombat"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("coaching"),
            Choice {
                move_id: String::from("coaching"),
                target: MoveTarget::User,
                move_type: PokemonType::Fighting,
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
            String::from("coil"),
            Choice {
                move_id: String::from("coil"),
                target: MoveTarget::User,
                move_type: PokemonType::Poison,
                flags: Flags {
                    snatch: true,
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
            String::from("collisioncourse"),
            Choice {
                move_id: String::from("collisioncourse"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("combattorque"),
            Choice {
                move_id: String::from("combattorque"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("cometpunch"),
            Choice {
                move_id: String::from("cometpunch"),
                accuracy: 85.0,
                base_power: 18.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("comeuppance"),
            Choice {
                move_id: String::from("comeuppance"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("confide"),
            Choice {
                move_id: String::from("confide"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("confuseray"),
            Choice {
                move_id: String::from("confuseray"),
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Confusion,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("confusion"),
            Choice {
                move_id: String::from("confusion"),
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("constrict"),
            Choice {
                move_id: String::from("constrict"),
                base_power: 10.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
        moves.insert(
            String::from("conversion"),
            Choice {
                move_id: String::from("conversion"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("conversion2"),
            Choice {
                move_id: String::from("conversion2"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("copycat"),
            Choice {
                move_id: String::from("copycat"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("coreenforcer"),
            Choice {
                move_id: String::from("coreenforcer"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("corrosivegas"),
            Choice {
                move_id: String::from("corrosivegas"),
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("cosmicpower"),
            Choice {
                move_id: String::from("cosmicpower"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
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
            String::from("cottonguard"),
            Choice {
                move_id: String::from("cottonguard"),
                target: MoveTarget::User,
                move_type: PokemonType::Grass,
                flags: Flags {
                    snatch: true,
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
        moves.insert(
            String::from("cottonspore"),
            Choice {
                move_id: String::from("cottonspore"),
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
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
        moves.insert(
            String::from("counter"),
            Choice {
                move_id: String::from("counter"),
                category: MoveCategory::Physical,
                priority: -5,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("courtchange"),
            Choice {
                move_id: String::from("courtchange"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                hazard_clear: Some(|state: &State, _: &SideReference| {
                    let mut instructions = vec![];
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
                                instructions.push(Instruction::ChangeSideCondition(
                                    ChangeSideConditionInstruction {
                                        side_ref: side,
                                        side_condition: side_condition,
                                        amount: -1 * side_condition_num,
                                    },
                                ));
                                instructions.push(Instruction::ChangeSideCondition(
                                    ChangeSideConditionInstruction {
                                        side_ref: side.get_other_side(),
                                        side_condition: side_condition,
                                        amount: side_condition_num,
                                    },
                                ));
                            }
                        }
                    }
                    return instructions;
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("covet"),
            Choice {
                move_id: String::from("covet"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("crabhammer"),
            Choice {
                move_id: String::from("crabhammer"),
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("craftyshield"),
            Choice {
                move_id: String::from("craftyshield"),
                priority: 3,
                target: MoveTarget::User,
                move_type: PokemonType::Fairy,
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
            String::from("crosschop"),
            Choice {
                move_id: String::from("crosschop"),
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("crosspoison"),
            Choice {
                move_id: String::from("crosspoison"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("crunch"),
            Choice {
                move_id: String::from("crunch"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
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
            String::from("crushclaw"),
            Choice {
                move_id: String::from("crushclaw"),
                accuracy: 95.0,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("crushgrip"),
            Choice {
                move_id: String::from("crushgrip"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("curse"),
            Choice {
                move_id: String::from("curse"),
                move_type: PokemonType::Ghost,
                flags: Flags {
                    ..Default::default()
                },
                boost: Some(Boost {
                    target: MoveTarget::Opponent,
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
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Curse,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("cut"),
            Choice {
                move_id: String::from("cut"),
                accuracy: 95.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("darkestlariat"),
            Choice {
                move_id: String::from("darkestlariat"),
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("darkpulse"),
            Choice {
                move_id: String::from("darkpulse"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dark,
                flags: Flags {
                    distance: true,
                    mirror: true,
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("darkvoid"),
            Choice {
                move_id: String::from("darkvoid"),
                accuracy: 50.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Sleep,
                }),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dazzlinggleam"),
            Choice {
                move_id: String::from("dazzlinggleam"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("decorate"),
            Choice {
                move_id: String::from("decorate"),
                move_type: PokemonType::Fairy,
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
            String::from("defendorder"),
            Choice {
                move_id: String::from("defendorder"),
                target: MoveTarget::User,
                move_type: PokemonType::Bug,
                flags: Flags {
                    snatch: true,
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
            String::from("defensecurl"),
            Choice {
                move_id: String::from("defensecurl"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
                    volatile_status: PokemonVolatileStatus::DefenseCurl,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("defog"),
            Choice {
                move_id: String::from("defog"),
                move_type: PokemonType::Flying,
                flags: Flags {
                    mirror: true,
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
                hazard_clear: Some(|state: &State, _: &SideReference| {
                    let mut instructions = vec![];
                    if state.terrain.terrain_type != Terrain::None {
                        instructions.push(Instruction::ChangeTerrain(ChangeTerrain {
                            new_terrain: Terrain::None,
                            new_terrain_turns_remaining: 0,
                            previous_terrain: state.terrain.terrain_type,
                            previous_terrain_turns_remaining: state.terrain.turns_remaining,
                        }));
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
                                instructions.push(Instruction::ChangeSideCondition(
                                    ChangeSideConditionInstruction {
                                        side_ref: side,
                                        side_condition: side_condition,
                                        amount: -1 * side_condition_num,
                                    },
                                ))
                            }
                        }
                    }
                    return instructions;
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("destinybond"),
            Choice {
                move_id: String::from("destinybond"),
                target: MoveTarget::User,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::DestinyBond,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("detect"),
            Choice {
                move_id: String::from("detect"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Protect,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("diamondstorm"),
            Choice {
                move_id: String::from("diamondstorm"),
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
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
        moves.insert(
            String::from("dig"),
            Choice {
                move_id: String::from("dig"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    charge: true,
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("direclaw"),
            Choice {
                move_id: String::from("direclaw"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("disable"),
            Choice {
                move_id: String::from("disable"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Disable,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("disarmingvoice"),
            Choice {
                move_id: String::from("disarmingvoice"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("discharge"),
            Choice {
                move_id: String::from("discharge"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dive"),
            Choice {
                move_id: String::from("dive"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    charge: true,
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dizzypunch"),
            Choice {
                move_id: String::from("dizzypunch"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doodle"),
            Choice {
                move_id: String::from("doodle"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doomdesire"),
            Choice {
                move_id: String::from("doomdesire"),
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Steel,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doubleedge"),
            Choice {
                move_id: String::from("doubleedge"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doublehit"),
            Choice {
                move_id: String::from("doublehit"),
                accuracy: 90.0,
                base_power: 35.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doubleironbash"),
            Choice {
                move_id: String::from("doubleironbash"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doublekick"),
            Choice {
                move_id: String::from("doublekick"),
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doubleshock"),
            Choice {
                move_id: String::from("doubleshock"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doubleslap"),
            Choice {
                move_id: String::from("doubleslap"),
                accuracy: 85.0,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("doubleteam"),
            Choice {
                move_id: String::from("doubleteam"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
        moves.insert(
            String::from("dracometeor"),
            Choice {
                move_id: String::from("dracometeor"),
                accuracy: 90.0,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragonascent"),
            Choice {
                move_id: String::from("dragonascent"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragonbreath"),
            Choice {
                move_id: String::from("dragonbreath"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragonclaw"),
            Choice {
                move_id: String::from("dragonclaw"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragondance"),
            Choice {
                move_id: String::from("dragondance"),
                target: MoveTarget::User,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    dance: true,
                    snatch: true,
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
            String::from("dragondarts"),
            Choice {
                move_id: String::from("dragondarts"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragonenergy"),
            Choice {
                move_id: String::from("dragonenergy"),
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragonhammer"),
            Choice {
                move_id: String::from("dragonhammer"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragonpulse"),
            Choice {
                move_id: String::from("dragonpulse"),
                base_power: 85.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    distance: true,
                    mirror: true,
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragonrage"),
            Choice {
                move_id: String::from("dragonrage"),
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragonrush"),
            Choice {
                move_id: String::from("dragonrush"),
                accuracy: 75.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dragontail"),
            Choice {
                move_id: String::from("dragontail"),
                accuracy: 90.0,
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: -6,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    contact: true,
                    drag: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("drainingkiss"),
            Choice {
                move_id: String::from("drainingkiss"),
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    contact: true,
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.75),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("drainpunch"),
            Choice {
                move_id: String::from("drainpunch"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    heal: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dreameater"),
            Choice {
                move_id: String::from("dreameater"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("drillpeck"),
            Choice {
                move_id: String::from("drillpeck"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("drillrun"),
            Choice {
                move_id: String::from("drillrun"),
                accuracy: 95.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("drumbeating"),
            Choice {
                move_id: String::from("drumbeating"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
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
            String::from("dualchop"),
            Choice {
                move_id: String::from("dualchop"),
                accuracy: 90.0,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dualwingbeat"),
            Choice {
                move_id: String::from("dualwingbeat"),
                accuracy: 90.0,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dynamaxcannon"),
            Choice {
                move_id: String::from("dynamaxcannon"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("dynamicpunch"),
            Choice {
                move_id: String::from("dynamicpunch"),
                accuracy: 50.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("earthpower"),
            Choice {
                move_id: String::from("earthpower"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
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
            String::from("earthquake"),
            Choice {
                move_id: String::from("earthquake"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("echoedvoice"),
            Choice {
                move_id: String::from("echoedvoice"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("eerieimpulse"),
            Choice {
                move_id: String::from("eerieimpulse"),
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
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
            String::from("eeriespell"),
            Choice {
                move_id: String::from("eeriespell"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("eggbomb"),
            Choice {
                move_id: String::from("eggbomb"),
                accuracy: 75.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("electricterrain"),
            Choice {
                move_id: String::from("electricterrain"),
                move_type: PokemonType::Electric,
                flags: Flags {
                    nonsky: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("electrify"),
            Choice {
                move_id: String::from("electrify"),
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Electrify,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("electroball"),
            Choice {
                move_id: String::from("electroball"),
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("electrodrift"),
            Choice {
                move_id: String::from("electrodrift"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("electroweb"),
            Choice {
                move_id: String::from("electroweb"),
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
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
            String::from("embargo"),
            Choice {
                move_id: String::from("embargo"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Embargo,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ember"),
            Choice {
                move_id: String::from("ember"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("encore"),
            Choice {
                move_id: String::from("encore"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Encore,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("endeavor"),
            Choice {
                move_id: String::from("endeavor"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("endure"),
            Choice {
                move_id: String::from("endure"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Endure,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("energyball"),
            Choice {
                move_id: String::from("energyball"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    bullet: true,
                    mirror: true,
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
            String::from("entrainment"),
            Choice {
                move_id: String::from("entrainment"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("eruption"),
            Choice {
                move_id: String::from("eruption"),
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("esperwing"),
            Choice {
                move_id: String::from("esperwing"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
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
            String::from("eternabeam"),
            Choice {
                move_id: String::from("eternabeam"),
                accuracy: 90.0,
                base_power: 160.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("expandingforce"),
            Choice {
                move_id: String::from("expandingforce"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("explosion"),
            Choice {
                move_id: String::from("explosion"),
                base_power: 250.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -1.0,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("extrasensory"),
            Choice {
                move_id: String::from("extrasensory"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("extremespeed"),
            Choice {
                move_id: String::from("extremespeed"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                priority: 2,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("facade"),
            Choice {
                move_id: String::from("facade"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fairylock"),
            Choice {
                move_id: String::from("fairylock"),
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fairywind"),
            Choice {
                move_id: String::from("fairywind"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fakeout"),
            Choice {
                move_id: String::from("fakeout"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 3,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("faketears"),
            Choice {
                move_id: String::from("faketears"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
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
            String::from("falsesurrender"),
            Choice {
                move_id: String::from("falsesurrender"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("falseswipe"),
            Choice {
                move_id: String::from("falseswipe"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("featherdance"),
            Choice {
                move_id: String::from("featherdance"),
                move_type: PokemonType::Flying,
                flags: Flags {
                    dance: true,
                    mirror: true,
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
        moves.insert(
            String::from("feint"),
            Choice {
                move_id: String::from("feint"),
                base_power: 30.0,
                category: MoveCategory::Physical,
                priority: 2,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("feintattack"),
            Choice {
                move_id: String::from("feintattack"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fellstinger"),
            Choice {
                move_id: String::from("fellstinger"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fierydance"),
            Choice {
                move_id: String::from("fierydance"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    dance: true,
                    mirror: true,
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
            String::from("fierywrath"),
            Choice {
                move_id: String::from("fierywrath"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("filletaway"),
            Choice {
                move_id: String::from("filletaway"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("finalgambit"),
            Choice {
                move_id: String::from("finalgambit"),
                category: MoveCategory::Special,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -1.0,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fireblast"),
            Choice {
                move_id: String::from("fireblast"),
                accuracy: 85.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("firefang"),
            Choice {
                move_id: String::from("firefang"),
                accuracy: 95.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::Burn),
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                    },
                ]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("firelash"),
            Choice {
                move_id: String::from("firelash"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("firepledge"),
            Choice {
                move_id: String::from("firepledge"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("firepunch"),
            Choice {
                move_id: String::from("firepunch"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("firespin"),
            Choice {
                move_id: String::from("firespin"),
                accuracy: 85.0,
                base_power: 35.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("firstimpression"),
            Choice {
                move_id: String::from("firstimpression"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                priority: 2,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fishiousrend"),
            Choice {
                move_id: String::from("fishiousrend"),
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fissure"),
            Choice {
                move_id: String::from("fissure"),
                accuracy: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flail"),
            Choice {
                move_id: String::from("flail"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flameburst"),
            Choice {
                move_id: String::from("flameburst"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flamecharge"),
            Choice {
                move_id: String::from("flamecharge"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("flamethrower"),
            Choice {
                move_id: String::from("flamethrower"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flamewheel"),
            Choice {
                move_id: String::from("flamewheel"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flareblitz"),
            Choice {
                move_id: String::from("flareblitz"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flash"),
            Choice {
                move_id: String::from("flash"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("flashcannon"),
            Choice {
                move_id: String::from("flashcannon"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
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
            String::from("flatter"),
            Choice {
                move_id: String::from("flatter"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
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
                    volatile_status: PokemonVolatileStatus::Confusion,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fleurcannon"),
            Choice {
                move_id: String::from("fleurcannon"),
                accuracy: 90.0,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fling"),
            Choice {
                move_id: String::from("fling"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flipturn"),
            Choice {
                move_id: String::from("flipturn"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("floatyfall"),
            Choice {
                move_id: String::from("floatyfall"),
                accuracy: 95.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    gravity: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("floralhealing"),
            Choice {
                move_id: String::from("floralhealing"),
                move_type: PokemonType::Fairy,
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
            String::from("flowershield"),
            Choice {
                move_id: String::from("flowershield"),
                move_type: PokemonType::Fairy,
                flags: Flags {
                    distance: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flowertrick"),
            Choice {
                move_id: String::from("flowertrick"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fly"),
            Choice {
                move_id: String::from("fly"),
                accuracy: 95.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    charge: true,
                    contact: true,
                    distance: true,
                    gravity: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("flyingpress"),
            Choice {
                move_id: String::from("flyingpress"),
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    distance: true,
                    gravity: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("focusblast"),
            Choice {
                move_id: String::from("focusblast"),
                accuracy: 70.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    bullet: true,
                    mirror: true,
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
            String::from("focusenergy"),
            Choice {
                move_id: String::from("focusenergy"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Focusenergy,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("focuspunch"),
            Choice {
                move_id: String::from("focuspunch"),
                base_power: 150.0,
                category: MoveCategory::Physical,
                priority: -3,
                move_type: PokemonType::Fighting,
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
            String::from("followme"),
            Choice {
                move_id: String::from("followme"),
                priority: 2,
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::FollowMe,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("forcepalm"),
            Choice {
                move_id: String::from("forcepalm"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("foresight"),
            Choice {
                move_id: String::from("foresight"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Foresight,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("forestscurse"),
            Choice {
                move_id: String::from("forestscurse"),
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("foulplay"),
            Choice {
                move_id: String::from("foulplay"),
                base_power: 95.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("freezedry"),
            Choice {
                move_id: String::from("freezedry"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Freeze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("freezeshock"),
            Choice {
                move_id: String::from("freezeshock"),
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    charge: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("freezingglare"),
            Choice {
                move_id: String::from("freezingglare"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Freeze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("freezyfrost"),
            Choice {
                move_id: String::from("freezyfrost"),
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("frenzyplant"),
            Choice {
                move_id: String::from("frenzyplant"),
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("frostbreath"),
            Choice {
                move_id: String::from("frostbreath"),
                accuracy: 90.0,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("frustration"),
            Choice {
                move_id: String::from("frustration"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("furyattack"),
            Choice {
                move_id: String::from("furyattack"),
                accuracy: 85.0,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("furycutter"),
            Choice {
                move_id: String::from("furycutter"),
                accuracy: 95.0,
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("furyswipes"),
            Choice {
                move_id: String::from("furyswipes"),
                accuracy: 80.0,
                base_power: 18.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fusionbolt"),
            Choice {
                move_id: String::from("fusionbolt"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("fusionflare"),
            Choice {
                move_id: String::from("fusionflare"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("futuresight"),
            Choice {
                move_id: String::from("futuresight"),
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("gastroacid"),
            Choice {
                move_id: String::from("gastroacid"),
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::GastroAcid,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("geargrind"),
            Choice {
                move_id: String::from("geargrind"),
                accuracy: 85.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("gearup"),
            Choice {
                move_id: String::from("gearup"),
                target: MoveTarget::User,
                move_type: PokemonType::Steel,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("geomancy"),
            Choice {
                move_id: String::from("geomancy"),
                target: MoveTarget::User,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    charge: true,
                    nonsky: true,
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
        moves.insert(
            String::from("gigadrain"),
            Choice {
                move_id: String::from("gigadrain"),
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("gigaimpact"),
            Choice {
                move_id: String::from("gigaimpact"),
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("gigatonhammer"),
            Choice {
                move_id: String::from("gigatonhammer"),
                base_power: 160.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("glaciallance"),
            Choice {
                move_id: String::from("glaciallance"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("glaciate"),
            Choice {
                move_id: String::from("glaciate"),
                accuracy: 95.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
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
            String::from("glaiverush"),
            Choice {
                move_id: String::from("glaiverush"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("glare"),
            Choice {
                move_id: String::from("glare"),
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Paralyze,
                }),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("glitzyglow"),
            Choice {
                move_id: String::from("glitzyglow"),
                accuracy: 95.0,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("grassknot"),
            Choice {
                move_id: String::from("grassknot"),
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("grasspledge"),
            Choice {
                move_id: String::from("grasspledge"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("grasswhistle"),
            Choice {
                move_id: String::from("grasswhistle"),
                accuracy: 55.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Sleep,
                }),
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("grassyglide"),
            Choice {
                move_id: String::from("grassyglide"),
                base_power: 55.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("grassyterrain"),
            Choice {
                move_id: String::from("grassyterrain"),
                move_type: PokemonType::Grass,
                flags: Flags {
                    nonsky: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("gravapple"),
            Choice {
                move_id: String::from("gravapple"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
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
            String::from("gravity"),
            Choice {
                move_id: String::from("gravity"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    nonsky: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("growl"),
            Choice {
                move_id: String::from("growl"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
        moves.insert(
            String::from("growth"),
            Choice {
                move_id: String::from("growth"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
            String::from("grudge"),
            Choice {
                move_id: String::from("grudge"),
                target: MoveTarget::User,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Grudge,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("guardsplit"),
            Choice {
                move_id: String::from("guardsplit"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("guardswap"),
            Choice {
                move_id: String::from("guardswap"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("guillotine"),
            Choice {
                move_id: String::from("guillotine"),
                accuracy: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("gunkshot"),
            Choice {
                move_id: String::from("gunkshot"),
                accuracy: 80.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("gust"),
            Choice {
                move_id: String::from("gust"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("gyroball"),
            Choice {
                move_id: String::from("gyroball"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    bullet: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hail"),
            Choice {
                move_id: String::from("hail"),
                move_type: PokemonType::Ice,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hammerarm"),
            Choice {
                move_id: String::from("hammerarm"),
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("happyhour"),
            Choice {
                move_id: String::from("happyhour"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("harden"),
            Choice {
                move_id: String::from("harden"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
            String::from("haze"),
            Choice {
                move_id: String::from("haze"),
                move_type: PokemonType::Ice,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("headbutt"),
            Choice {
                move_id: String::from("headbutt"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("headcharge"),
            Choice {
                move_id: String::from("headcharge"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.25),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("headlongrush"),
            Choice {
                move_id: String::from("headlongrush"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("headsmash"),
            Choice {
                move_id: String::from("headsmash"),
                accuracy: 80.0,
                base_power: 150.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("healbell"),
            Choice {
                move_id: String::from("healbell"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    distance: true,
                    snatch: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("healblock"),
            Choice {
                move_id: String::from("healblock"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::HealBlock,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("healingwish"),
            Choice {
                move_id: String::from("healingwish"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("healorder"),
            Choice {
                move_id: String::from("healorder"),
                target: MoveTarget::User,
                move_type: PokemonType::Bug,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("healpulse"),
            Choice {
                move_id: String::from("healpulse"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    distance: true,
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
            String::from("heartstamp"),
            Choice {
                move_id: String::from("heartstamp"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("heartswap"),
            Choice {
                move_id: String::from("heartswap"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("heatcrash"),
            Choice {
                move_id: String::from("heatcrash"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("heatwave"),
            Choice {
                move_id: String::from("heatwave"),
                accuracy: 90.0,
                base_power: 95.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("heavyslam"),
            Choice {
                move_id: String::from("heavyslam"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("helpinghand"),
            Choice {
                move_id: String::from("helpinghand"),
                priority: 5,
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::HelpingHand,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hex"),
            Choice {
                move_id: String::from("hex"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                modify_move: Some(
                    |state: &State,
                     attacking_choice: &mut Choice,
                     _,
                     attacking_side_ref: &SideReference| {
                        if state
                            .get_side_immutable(&attacking_side_ref.get_other_side())
                            .get_active_immutable()
                            .status
                            != PokemonStatus::None
                        {
                            attacking_choice.base_power *= 2.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpower"),
            Choice {
                move_id: String::from("hiddenpower"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerbug60"),
            Choice {
                move_id: String::from("hiddenpowerbug60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerbug70"),
            Choice {
                move_id: String::from("hiddenpowerbug70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerdark60"),
            Choice {
                move_id: String::from("hiddenpowerdark60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerdark70"),
            Choice {
                move_id: String::from("hiddenpowerdark70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerdragon60"),
            Choice {
                move_id: String::from("hiddenpowerdragon60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerdragon70"),
            Choice {
                move_id: String::from("hiddenpowerdragon70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerelectric60"),
            Choice {
                move_id: String::from("hiddenpowerelectric60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerelectric70"),
            Choice {
                move_id: String::from("hiddenpowerelectric70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerfighting60"),
            Choice {
                move_id: String::from("hiddenpowerfighting60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerfighting70"),
            Choice {
                move_id: String::from("hiddenpowerfighting70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerfire60"),
            Choice {
                move_id: String::from("hiddenpowerfire60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerfire70"),
            Choice {
                move_id: String::from("hiddenpowerfire70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerflying60"),
            Choice {
                move_id: String::from("hiddenpowerflying60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerflying70"),
            Choice {
                move_id: String::from("hiddenpowerflying70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerghost60"),
            Choice {
                move_id: String::from("hiddenpowerghost60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerghost70"),
            Choice {
                move_id: String::from("hiddenpowerghost70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowergrass60"),
            Choice {
                move_id: String::from("hiddenpowergrass60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowergrass70"),
            Choice {
                move_id: String::from("hiddenpowergrass70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerground60"),
            Choice {
                move_id: String::from("hiddenpowerground60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerground70"),
            Choice {
                move_id: String::from("hiddenpowerground70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerice60"),
            Choice {
                move_id: String::from("hiddenpowerice60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerice70"),
            Choice {
                move_id: String::from("hiddenpowerice70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerpoison60"),
            Choice {
                move_id: String::from("hiddenpowerpoison60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerpoison70"),
            Choice {
                move_id: String::from("hiddenpowerpoison70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerpsychic60"),
            Choice {
                move_id: String::from("hiddenpowerpsychic60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerpsychic70"),
            Choice {
                move_id: String::from("hiddenpowerpsychic70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerrock60"),
            Choice {
                move_id: String::from("hiddenpowerrock60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerrock70"),
            Choice {
                move_id: String::from("hiddenpowerrock70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowersteel60"),
            Choice {
                move_id: String::from("hiddenpowersteel60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowersteel70"),
            Choice {
                move_id: String::from("hiddenpowersteel70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerwater60"),
            Choice {
                move_id: String::from("hiddenpowerwater60"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hiddenpowerwater70"),
            Choice {
                move_id: String::from("hiddenpowerwater70"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("highhorsepower"),
            Choice {
                move_id: String::from("highhorsepower"),
                accuracy: 95.0,
                base_power: 95.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("highjumpkick"),
            Choice {
                move_id: String::from("highjumpkick"),
                accuracy: 90.0,
                base_power: 130.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    gravity: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                crash: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("holdback"),
            Choice {
                move_id: String::from("holdback"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("holdhands"),
            Choice {
                move_id: String::from("holdhands"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("honeclaws"),
            Choice {
                move_id: String::from("honeclaws"),
                target: MoveTarget::User,
                move_type: PokemonType::Dark,
                flags: Flags {
                    snatch: true,
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
            String::from("hornattack"),
            Choice {
                move_id: String::from("hornattack"),
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("horndrill"),
            Choice {
                move_id: String::from("horndrill"),
                accuracy: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hornleech"),
            Choice {
                move_id: String::from("hornleech"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("howl"),
            Choice {
                move_id: String::from("howl"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
        moves.insert(
            String::from("hurricane"),
            Choice {
                move_id: String::from("hurricane"),
                accuracy: 70.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hydrocannon"),
            Choice {
                move_id: String::from("hydrocannon"),
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hydropump"),
            Choice {
                move_id: String::from("hydropump"),
                accuracy: 80.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hydrosteam"),
            Choice {
                move_id: String::from("hydrosteam"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hyperbeam"),
            Choice {
                move_id: String::from("hyperbeam"),
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hyperdrill"),
            Choice {
                move_id: String::from("hyperdrill"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hyperfang"),
            Choice {
                move_id: String::from("hyperfang"),
                accuracy: 90.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hyperspacefury"),
            Choice {
                move_id: String::from("hyperspacefury"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hyperspacehole"),
            Choice {
                move_id: String::from("hyperspacehole"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hypervoice"),
            Choice {
                move_id: String::from("hypervoice"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("hypnosis"),
            Choice {
                move_id: String::from("hypnosis"),
                accuracy: 60.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Sleep,
                }),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("iceball"),
            Choice {
                move_id: String::from("iceball"),
                accuracy: 90.0,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    bullet: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("icebeam"),
            Choice {
                move_id: String::from("icebeam"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Freeze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("iceburn"),
            Choice {
                move_id: String::from("iceburn"),
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    charge: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("icefang"),
            Choice {
                move_id: String::from("icefang"),
                accuracy: 95.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::Freeze),
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                    },
                ]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("icehammer"),
            Choice {
                move_id: String::from("icehammer"),
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("icepunch"),
            Choice {
                move_id: String::from("icepunch"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Freeze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("iceshard"),
            Choice {
                move_id: String::from("iceshard"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("icespinner"),
            Choice {
                move_id: String::from("icespinner"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("iciclecrash"),
            Choice {
                move_id: String::from("iciclecrash"),
                accuracy: 90.0,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("iciclespear"),
            Choice {
                move_id: String::from("iciclespear"),
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("icywind"),
            Choice {
                move_id: String::from("icywind"),
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
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
            String::from("imprison"),
            Choice {
                move_id: String::from("imprison"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Imprison,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("incinerate"),
            Choice {
                move_id: String::from("incinerate"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("infernalparade"),
            Choice {
                move_id: String::from("infernalparade"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("inferno"),
            Choice {
                move_id: String::from("inferno"),
                accuracy: 50.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("infestation"),
            Choice {
                move_id: String::from("infestation"),
                base_power: 20.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ingrain"),
            Choice {
                move_id: String::from("ingrain"),
                target: MoveTarget::User,
                move_type: PokemonType::Grass,
                flags: Flags {
                    nonsky: true,
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Ingrain,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("instruct"),
            Choice {
                move_id: String::from("instruct"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("iondeluge"),
            Choice {
                move_id: String::from("iondeluge"),
                priority: 1,
                move_type: PokemonType::Electric,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("irondefense"),
            Choice {
                move_id: String::from("irondefense"),
                target: MoveTarget::User,
                move_type: PokemonType::Steel,
                flags: Flags {
                    snatch: true,
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
            String::from("ironhead"),
            Choice {
                move_id: String::from("ironhead"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("irontail"),
            Choice {
                move_id: String::from("irontail"),
                accuracy: 75.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("ivycudgel"),
            Choice {
                move_id: String::from("ivycudgel"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("jawlock"),
            Choice {
                move_id: String::from("jawlock"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("jetpunch"),
            Choice {
                move_id: String::from("jetpunch"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("judgment"),
            Choice {
                move_id: String::from("judgment"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("jumpkick"),
            Choice {
                move_id: String::from("jumpkick"),
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    gravity: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                crash: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("junglehealing"),
            Choice {
                move_id: String::from("junglehealing"),
                target: MoveTarget::User,
                move_type: PokemonType::Grass,
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
            String::from("karatechop"),
            Choice {
                move_id: String::from("karatechop"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("kinesis"),
            Choice {
                move_id: String::from("kinesis"),
                accuracy: 80.0,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
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
            String::from("kingsshield"),
            Choice {
                move_id: String::from("kingsshield"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Steel,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::KingsShield,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("knockoff"),
            Choice {
                move_id: String::from("knockoff"),
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                after_damage_hit: Some(
                    |state: &State, _: &Choice, attacking_side_reference: &SideReference| {
                        let defending_side =
                            state.get_side_immutable(&attacking_side_reference.get_other_side());
                        let defender_active = defending_side.get_active_immutable();
                        if defender_active.item_can_be_removed() {
                            return vec![Instruction::ChangeItem(ChangeItemInstruction {
                                side_ref: attacking_side_reference.get_other_side(),
                                current_item: defender_active.item.clone(),
                                new_item: String::from(""),
                            })];
                        }
                        return vec![];
                    },
                ),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("kowtowcleave"),
            Choice {
                move_id: String::from("kowtowcleave"),
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("landswrath"),
            Choice {
                move_id: String::from("landswrath"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("laserfocus"),
            Choice {
                move_id: String::from("laserfocus"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::LaserFocus,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lashout"),
            Choice {
                move_id: String::from("lashout"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lastresort"),
            Choice {
                move_id: String::from("lastresort"),
                base_power: 140.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lastrespects"),
            Choice {
                move_id: String::from("lastrespects"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lavaplume"),
            Choice {
                move_id: String::from("lavaplume"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("leafage"),
            Choice {
                move_id: String::from("leafage"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("leafblade"),
            Choice {
                move_id: String::from("leafblade"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("leafstorm"),
            Choice {
                move_id: String::from("leafstorm"),
                accuracy: 90.0,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("leaftornado"),
            Choice {
                move_id: String::from("leaftornado"),
                accuracy: 90.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
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
            String::from("leechlife"),
            Choice {
                move_id: String::from("leechlife"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("leechseed"),
            Choice {
                move_id: String::from("leechseed"),
                accuracy: 90.0,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::LeechSeed,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("leer"),
            Choice {
                move_id: String::from("leer"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("lick"),
            Choice {
                move_id: String::from("lick"),
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lifedew"),
            Choice {
                move_id: String::from("lifedew"),
                target: MoveTarget::User,
                move_type: PokemonType::Water,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("lightofruin"),
            Choice {
                move_id: String::from("lightofruin"),
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lightscreen"),
            Choice {
                move_id: String::from("lightscreen"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::LightScreen,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("liquidation"),
            Choice {
                move_id: String::from("liquidation"),
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("lockon"),
            Choice {
                move_id: String::from("lockon"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lovelykiss"),
            Choice {
                move_id: String::from("lovelykiss"),
                accuracy: 75.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Sleep,
                }),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lowkick"),
            Choice {
                move_id: String::from("lowkick"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lowsweep"),
            Choice {
                move_id: String::from("lowsweep"),
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("luckychant"),
            Choice {
                move_id: String::from("luckychant"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
            String::from("luminacrash"),
            Choice {
                move_id: String::from("luminacrash"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
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
            String::from("lunarblessing"),
            Choice {
                move_id: String::from("lunarblessing"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("lunardance"),
            Choice {
                move_id: String::from("lunardance"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    dance: true,
                    heal: true,
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("lunge"),
            Choice {
                move_id: String::from("lunge"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("lusterpurge"),
            Choice {
                move_id: String::from("lusterpurge"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
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
        moves.insert(
            String::from("machpunch"),
            Choice {
                move_id: String::from("machpunch"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magicalleaf"),
            Choice {
                move_id: String::from("magicalleaf"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magicaltorque"),
            Choice {
                move_id: String::from("magicaltorque"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magiccoat"),
            Choice {
                move_id: String::from("magiccoat"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::MagicCoat,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magicpowder"),
            Choice {
                move_id: String::from("magicpowder"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magicroom"),
            Choice {
                move_id: String::from("magicroom"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magmastorm"),
            Choice {
                move_id: String::from("magmastorm"),
                accuracy: 75.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magnetbomb"),
            Choice {
                move_id: String::from("magnetbomb"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magneticflux"),
            Choice {
                move_id: String::from("magneticflux"),
                target: MoveTarget::User,
                move_type: PokemonType::Electric,
                flags: Flags {
                    distance: true,
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magnetrise"),
            Choice {
                move_id: String::from("magnetrise"),
                target: MoveTarget::User,
                move_type: PokemonType::Electric,
                flags: Flags {
                    gravity: true,
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::MagnetRise,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("magnitude"),
            Choice {
                move_id: String::from("magnitude"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("makeitrain"),
            Choice {
                move_id: String::from("makeitrain"),
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("matblock"),
            Choice {
                move_id: String::from("matblock"),
                target: MoveTarget::User,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    nonsky: true,
                    snatch: true,
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
            String::from("matchagotcha"),
            Choice {
                move_id: String::from("matchagotcha"),
                accuracy: 90.0,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("meanlook"),
            Choice {
                move_id: String::from("meanlook"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("meditate"),
            Choice {
                move_id: String::from("meditate"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
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
            String::from("mefirst"),
            Choice {
                move_id: String::from("mefirst"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("megadrain"),
            Choice {
                move_id: String::from("megadrain"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("megahorn"),
            Choice {
                move_id: String::from("megahorn"),
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("megakick"),
            Choice {
                move_id: String::from("megakick"),
                accuracy: 75.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("megapunch"),
            Choice {
                move_id: String::from("megapunch"),
                accuracy: 85.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("memento"),
            Choice {
                move_id: String::from("memento"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
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
            String::from("metalburst"),
            Choice {
                move_id: String::from("metalburst"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("metalclaw"),
            Choice {
                move_id: String::from("metalclaw"),
                accuracy: 95.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("metalsound"),
            Choice {
                move_id: String::from("metalsound"),
                accuracy: 85.0,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
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
            String::from("meteorassault"),
            Choice {
                move_id: String::from("meteorassault"),
                base_power: 150.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("meteorbeam"),
            Choice {
                move_id: String::from("meteorbeam"),
                accuracy: 90.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Rock,
                flags: Flags {
                    charge: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("meteormash"),
            Choice {
                move_id: String::from("meteormash"),
                accuracy: 90.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
        moves.insert(
            String::from("metronome"),
            Choice {
                move_id: String::from("metronome"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("milkdrink"),
            Choice {
                move_id: String::from("milkdrink"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("mimic"),
            Choice {
                move_id: String::from("mimic"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mindblown"),
            Choice {
                move_id: String::from("mindblown"),
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
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
            String::from("mindreader"),
            Choice {
                move_id: String::from("mindreader"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("minimize"),
            Choice {
                move_id: String::from("minimize"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
                    volatile_status: PokemonVolatileStatus::Minimize,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("miracleeye"),
            Choice {
                move_id: String::from("miracleeye"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::MiracleEye,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mirrorcoat"),
            Choice {
                move_id: String::from("mirrorcoat"),
                category: MoveCategory::Special,
                priority: -5,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mirrormove"),
            Choice {
                move_id: String::from("mirrormove"),
                move_type: PokemonType::Flying,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mirrorshot"),
            Choice {
                move_id: String::from("mirrorshot"),
                accuracy: 85.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
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
            String::from("mist"),
            Choice {
                move_id: String::from("mist"),
                target: MoveTarget::User,
                move_type: PokemonType::Ice,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Mist,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mistball"),
            Choice {
                move_id: String::from("mistball"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    bullet: true,
                    mirror: true,
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
        moves.insert(
            String::from("mistyexplosion"),
            Choice {
                move_id: String::from("mistyexplosion"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -1.0,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mistyterrain"),
            Choice {
                move_id: String::from("mistyterrain"),
                move_type: PokemonType::Fairy,
                flags: Flags {
                    nonsky: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("moonblast"),
            Choice {
                move_id: String::from("moonblast"),
                base_power: 95.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
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
            String::from("moongeistbeam"),
            Choice {
                move_id: String::from("moongeistbeam"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("moonlight"),
            Choice {
                move_id: String::from("moonlight"),
                target: MoveTarget::User,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("morningsun"),
            Choice {
                move_id: String::from("morningsun"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("mortalspin"),
            Choice {
                move_id: String::from("mortalspin"),
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mountaingale"),
            Choice {
                move_id: String::from("mountaingale"),
                accuracy: 85.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mudbomb"),
            Choice {
                move_id: String::from("mudbomb"),
                accuracy: 85.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ground,
                flags: Flags {
                    bullet: true,
                    mirror: true,
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
            String::from("muddywater"),
            Choice {
                move_id: String::from("muddywater"),
                accuracy: 85.0,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
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
            String::from("mudshot"),
            Choice {
                move_id: String::from("mudshot"),
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
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
            String::from("mudslap"),
            Choice {
                move_id: String::from("mudslap"),
                base_power: 20.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
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
            String::from("mudsport"),
            Choice {
                move_id: String::from("mudsport"),
                move_type: PokemonType::Ground,
                flags: Flags {
                    nonsky: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("multiattack"),
            Choice {
                move_id: String::from("multiattack"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("mysticalfire"),
            Choice {
                move_id: String::from("mysticalfire"),
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
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
            String::from("mysticalpower"),
            Choice {
                move_id: String::from("mysticalpower"),
                accuracy: 90.0,
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
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
            String::from("nastyplot"),
            Choice {
                move_id: String::from("nastyplot"),
                target: MoveTarget::User,
                move_type: PokemonType::Dark,
                flags: Flags {
                    snatch: true,
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
            String::from("naturalgift"),
            Choice {
                move_id: String::from("naturalgift"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("naturepower"),
            Choice {
                move_id: String::from("naturepower"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("naturesmadness"),
            Choice {
                move_id: String::from("naturesmadness"),
                accuracy: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("needlearm"),
            Choice {
                move_id: String::from("needlearm"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("nightdaze"),
            Choice {
                move_id: String::from("nightdaze"),
                accuracy: 95.0,
                base_power: 85.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
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
            String::from("nightmare"),
            Choice {
                move_id: String::from("nightmare"),
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Nightmare,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("nightshade"),
            Choice {
                move_id: String::from("nightshade"),
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("nightslash"),
            Choice {
                move_id: String::from("nightslash"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("nobleroar"),
            Choice {
                move_id: String::from("nobleroar"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("noretreat"),
            Choice {
                move_id: String::from("noretreat"),
                target: MoveTarget::User,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    snatch: true,
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
                    volatile_status: PokemonVolatileStatus::NoRetreat,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("nothing"),
            Choice {
                move_id: String::from("nothing"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    gravity: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("noxioustorque"),
            Choice {
                move_id: String::from("noxioustorque"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("nuzzle"),
            Choice {
                move_id: String::from("nuzzle"),
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("oblivionwing"),
            Choice {
                move_id: String::from("oblivionwing"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Flying,
                flags: Flags {
                    distance: true,
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.75),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("obstruct"),
            Choice {
                move_id: String::from("obstruct"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Dark,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Protect,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("octazooka"),
            Choice {
                move_id: String::from("octazooka"),
                accuracy: 85.0,
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    bullet: true,
                    mirror: true,
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
            String::from("octolock"),
            Choice {
                move_id: String::from("octolock"),
                move_type: PokemonType::Fighting,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Octolock,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("odorsleuth"),
            Choice {
                move_id: String::from("odorsleuth"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Foresight,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ominouswind"),
            Choice {
                move_id: String::from("ominouswind"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
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
            String::from("orderup"),
            Choice {
                move_id: String::from("orderup"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("originpulse"),
            Choice {
                move_id: String::from("originpulse"),
                accuracy: 85.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("outrage"),
            Choice {
                move_id: String::from("outrage"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("overdrive"),
            Choice {
                move_id: String::from("overdrive"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("overheat"),
            Choice {
                move_id: String::from("overheat"),
                accuracy: 90.0,
                base_power: 130.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("painsplit"),
            Choice {
                move_id: String::from("painsplit"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("paleowave"),
            Choice {
                move_id: String::from("paleowave"),
                base_power: 85.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
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
        moves.insert(
            String::from("paraboliccharge"),
            Choice {
                move_id: String::from("paraboliccharge"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    heal: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                drain: Some(0.5),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("partingshot"),
            Choice {
                move_id: String::from("partingshot"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
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
            String::from("payback"),
            Choice {
                move_id: String::from("payback"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("payday"),
            Choice {
                move_id: String::from("payday"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("peck"),
            Choice {
                move_id: String::from("peck"),
                base_power: 35.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("perishsong"),
            Choice {
                move_id: String::from("perishsong"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    distance: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("petalblizzard"),
            Choice {
                move_id: String::from("petalblizzard"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("petaldance"),
            Choice {
                move_id: String::from("petaldance"),
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    dance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("phantomforce"),
            Choice {
                move_id: String::from("phantomforce"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    charge: true,
                    contact: true,
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("photongeyser"),
            Choice {
                move_id: String::from("photongeyser"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("pikapapow"),
            Choice {
                move_id: String::from("pikapapow"),
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("pinmissile"),
            Choice {
                move_id: String::from("pinmissile"),
                accuracy: 95.0,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("plasmafists"),
            Choice {
                move_id: String::from("plasmafists"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("playnice"),
            Choice {
                move_id: String::from("playnice"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("playrough"),
            Choice {
                move_id: String::from("playrough"),
                accuracy: 90.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("pluck"),
            Choice {
                move_id: String::from("pluck"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("poisonfang"),
            Choice {
                move_id: String::from("poisonfang"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Toxic),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("poisongas"),
            Choice {
                move_id: String::from("poisongas"),
                accuracy: 90.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Poison,
                }),
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("poisonjab"),
            Choice {
                move_id: String::from("poisonjab"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("poisonpowder"),
            Choice {
                move_id: String::from("poisonpowder"),
                accuracy: 75.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Poison,
                }),
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("poisonsting"),
            Choice {
                move_id: String::from("poisonsting"),
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("poisontail"),
            Choice {
                move_id: String::from("poisontail"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Poison,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("pollenpuff"),
            Choice {
                move_id: String::from("pollenpuff"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Bug,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("poltergeist"),
            Choice {
                move_id: String::from("poltergeist"),
                accuracy: 90.0,
                base_power: 110.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("populationbomb"),
            Choice {
                move_id: String::from("populationbomb"),
                accuracy: 90.0,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("pounce"),
            Choice {
                move_id: String::from("pounce"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("pound"),
            Choice {
                move_id: String::from("pound"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("powder"),
            Choice {
                move_id: String::from("powder"),
                priority: 1,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Powder,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("powdersnow"),
            Choice {
                move_id: String::from("powdersnow"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Freeze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("powergem"),
            Choice {
                move_id: String::from("powergem"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("powershift"),
            Choice {
                move_id: String::from("powershift"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::PowerShift,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("powersplit"),
            Choice {
                move_id: String::from("powersplit"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("powerswap"),
            Choice {
                move_id: String::from("powerswap"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("powertrick"),
            Choice {
                move_id: String::from("powertrick"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::PowerTrick,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("powertrip"),
            Choice {
                move_id: String::from("powertrip"),
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("poweruppunch"),
            Choice {
                move_id: String::from("poweruppunch"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("powerwhip"),
            Choice {
                move_id: String::from("powerwhip"),
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("precipiceblades"),
            Choice {
                move_id: String::from("precipiceblades"),
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("present"),
            Choice {
                move_id: String::from("present"),
                accuracy: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("prismaticlaser"),
            Choice {
                move_id: String::from("prismaticlaser"),
                base_power: 160.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("protect"),
            Choice {
                move_id: String::from("protect"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Protect,
                }),
                modify_move: Some(
                    |state: &State,
                     attacking_choice: &mut Choice,
                     _,
                     attacking_side_ref: &SideReference| {
                        if state
                            .get_side_immutable(&attacking_side_ref)
                            .side_conditions
                            .protect
                            > 0
                        {
                            // for now, the engine doesn't support consecutive protects
                            // 2nd protect will always fail
                            attacking_choice.volatile_status = None;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psybeam"),
            Choice {
                move_id: String::from("psybeam"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psyblade"),
            Choice {
                move_id: String::from("psyblade"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psychic"),
            Choice {
                move_id: String::from("psychic"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
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
            String::from("psychicfangs"),
            Choice {
                move_id: String::from("psychicfangs"),
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psychicterrain"),
            Choice {
                move_id: String::from("psychicterrain"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    nonsky: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psychoboost"),
            Choice {
                move_id: String::from("psychoboost"),
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psychocut"),
            Choice {
                move_id: String::from("psychocut"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psychoshift"),
            Choice {
                move_id: String::from("psychoshift"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psychup"),
            Choice {
                move_id: String::from("psychup"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psyshieldbash"),
            Choice {
                move_id: String::from("psyshieldbash"),
                accuracy: 90.0,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("psyshock"),
            Choice {
                move_id: String::from("psyshock"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psystrike"),
            Choice {
                move_id: String::from("psystrike"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("psywave"),
            Choice {
                move_id: String::from("psywave"),
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("punishment"),
            Choice {
                move_id: String::from("punishment"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("purify"),
            Choice {
                move_id: String::from("purify"),
                move_type: PokemonType::Poison,
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
            String::from("pursuit"),
            Choice {
                move_id: String::from("pursuit"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("pyroball"),
            Choice {
                move_id: String::from("pyroball"),
                accuracy: 90.0,
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    bullet: true,
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("quash"),
            Choice {
                move_id: String::from("quash"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("quickattack"),
            Choice {
                move_id: String::from("quickattack"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("quickguard"),
            Choice {
                move_id: String::from("quickguard"),
                priority: 3,
                target: MoveTarget::User,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    snatch: true,
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
            String::from("quiverdance"),
            Choice {
                move_id: String::from("quiverdance"),
                target: MoveTarget::User,
                move_type: PokemonType::Bug,
                flags: Flags {
                    dance: true,
                    snatch: true,
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
            String::from("rage"),
            Choice {
                move_id: String::from("rage"),
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ragefist"),
            Choice {
                move_id: String::from("ragefist"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ragepowder"),
            Choice {
                move_id: String::from("ragepowder"),
                priority: 2,
                target: MoveTarget::User,
                move_type: PokemonType::Bug,
                flags: Flags {
                    powder: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::RagePowder,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ragingbull"),
            Choice {
                move_id: String::from("ragingbull"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ragingfury"),
            Choice {
                move_id: String::from("ragingfury"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("raindance"),
            Choice {
                move_id: String::from("raindance"),
                move_type: PokemonType::Water,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rapidspin"),
            Choice {
                move_id: String::from("rapidspin"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
                hazard_clear: Some(|state: &State, side_ref: &SideReference| {
                    let attacking_side = state.get_side_immutable(side_ref);
                    let mut instructions = vec![];
                    if attacking_side.side_conditions.stealth_rock > 0 {
                        instructions.push(Instruction::ChangeSideCondition(
                            ChangeSideConditionInstruction {
                                side_ref: *side_ref,
                                side_condition: PokemonSideCondition::Stealthrock,
                                amount: -1 * attacking_side.side_conditions.stealth_rock,
                            },
                        ))
                    }
                    if attacking_side.side_conditions.spikes > 0 {
                        instructions.push(Instruction::ChangeSideCondition(
                            ChangeSideConditionInstruction {
                                side_ref: *side_ref,
                                side_condition: PokemonSideCondition::Spikes,
                                amount: -1 * attacking_side.side_conditions.spikes,
                            },
                        ))
                    }
                    if attacking_side.side_conditions.toxic_spikes > 0 {
                        instructions.push(Instruction::ChangeSideCondition(
                            ChangeSideConditionInstruction {
                                side_ref: *side_ref,
                                side_condition: PokemonSideCondition::ToxicSpikes,
                                amount: -1 * attacking_side.side_conditions.toxic_spikes,
                            },
                        ))
                    }
                    if attacking_side.side_conditions.sticky_web > 0 {
                        instructions.push(Instruction::ChangeSideCondition(
                            ChangeSideConditionInstruction {
                                side_ref: *side_ref,
                                side_condition: PokemonSideCondition::StickyWeb,
                                amount: -1 * attacking_side.side_conditions.sticky_web,
                            },
                        ))
                    }
                    return instructions;
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("razorleaf"),
            Choice {
                move_id: String::from("razorleaf"),
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("razorshell"),
            Choice {
                move_id: String::from("razorshell"),
                accuracy: 95.0,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("razorwind"),
            Choice {
                move_id: String::from("razorwind"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    charge: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("recharge"),
            Choice {
                move_id: String::from("recharge"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("recover"),
            Choice {
                move_id: String::from("recover"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("recycle"),
            Choice {
                move_id: String::from("recycle"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("reflect"),
            Choice {
                move_id: String::from("reflect"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                side_condition: Some(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Reflect,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("reflecttype"),
            Choice {
                move_id: String::from("reflecttype"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("refresh"),
            Choice {
                move_id: String::from("refresh"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("relicsong"),
            Choice {
                move_id: String::from("relicsong"),
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Sleep),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rest"),
            Choice {
                move_id: String::from("rest"),
                status: Some(Status {
                    target: MoveTarget::User,
                    status: PokemonStatus::Sleep,
                }),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    heal: true,
                    snatch: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 1.0,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("retaliate"),
            Choice {
                move_id: String::from("retaliate"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("return"),
            Choice {
                move_id: String::from("return"),
                base_power: 102.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("return102"),
            Choice {
                move_id: String::from("return102"),
                base_power: 102.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("revelationdance"),
            Choice {
                move_id: String::from("revelationdance"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    dance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("revenge"),
            Choice {
                move_id: String::from("revenge"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                priority: -4,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("reversal"),
            Choice {
                move_id: String::from("reversal"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("revivalblessing"),
            Choice {
                move_id: String::from("revivalblessing"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("risingvoltage"),
            Choice {
                move_id: String::from("risingvoltage"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("roar"),
            Choice {
                move_id: String::from("roar"),
                priority: -6,
                move_type: PokemonType::Normal,
                flags: Flags {
                    drag: true,
                    mirror: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("roaroftime"),
            Choice {
                move_id: String::from("roaroftime"),
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rockblast"),
            Choice {
                move_id: String::from("rockblast"),
                accuracy: 90.0,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rockclimb"),
            Choice {
                move_id: String::from("rockclimb"),
                accuracy: 85.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rockpolish"),
            Choice {
                move_id: String::from("rockpolish"),
                target: MoveTarget::User,
                move_type: PokemonType::Rock,
                flags: Flags {
                    snatch: true,
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
            String::from("rockslide"),
            Choice {
                move_id: String::from("rockslide"),
                accuracy: 90.0,
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rocksmash"),
            Choice {
                move_id: String::from("rocksmash"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("rockthrow"),
            Choice {
                move_id: String::from("rockthrow"),
                accuracy: 90.0,
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rocktomb"),
            Choice {
                move_id: String::from("rocktomb"),
                accuracy: 95.0,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
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
            String::from("rockwrecker"),
            Choice {
                move_id: String::from("rockwrecker"),
                accuracy: 90.0,
                base_power: 150.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    recharge: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("roleplay"),
            Choice {
                move_id: String::from("roleplay"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rollingkick"),
            Choice {
                move_id: String::from("rollingkick"),
                accuracy: 85.0,
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rollout"),
            Choice {
                move_id: String::from("rollout"),
                accuracy: 90.0,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("roost"),
            Choice {
                move_id: String::from("roost"),
                target: MoveTarget::User,
                move_type: PokemonType::Flying,
                flags: Flags {
                    heal: true,
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Roost,
                }),
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("rototiller"),
            Choice {
                move_id: String::from("rototiller"),
                move_type: PokemonType::Ground,
                flags: Flags {
                    distance: true,
                    nonsky: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("round"),
            Choice {
                move_id: String::from("round"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("ruination"),
            Choice {
                move_id: String::from("ruination"),
                accuracy: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sacredfire"),
            Choice {
                move_id: String::from("sacredfire"),
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 50.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sacredsword"),
            Choice {
                move_id: String::from("sacredsword"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("safeguard"),
            Choice {
                move_id: String::from("safeguard"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
            String::from("saltcure"),
            Choice {
                move_id: String::from("saltcure"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::SaltCure,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sandattack"),
            Choice {
                move_id: String::from("sandattack"),
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
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
            String::from("sandsearstorm"),
            Choice {
                move_id: String::from("sandsearstorm"),
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sandstorm"),
            Choice {
                move_id: String::from("sandstorm"),
                move_type: PokemonType::Rock,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sandtomb"),
            Choice {
                move_id: String::from("sandtomb"),
                accuracy: 85.0,
                base_power: 35.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sappyseed"),
            Choice {
                move_id: String::from("sappyseed"),
                accuracy: 90.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("scald"),
            Choice {
                move_id: String::from("scald"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("scaleshot"),
            Choice {
                move_id: String::from("scaleshot"),
                accuracy: 90.0,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("scaryface"),
            Choice {
                move_id: String::from("scaryface"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("scorchingsands"),
            Choice {
                move_id: String::from("scorchingsands"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ground,
                flags: Flags {
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("scratch"),
            Choice {
                move_id: String::from("scratch"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("screech"),
            Choice {
                move_id: String::from("screech"),
                accuracy: 85.0,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("searingshot"),
            Choice {
                move_id: String::from("searingshot"),
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("secretpower"),
            Choice {
                move_id: String::from("secretpower"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("secretsword"),
            Choice {
                move_id: String::from("secretsword"),
                base_power: 85.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("seedbomb"),
            Choice {
                move_id: String::from("seedbomb"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("seedflare"),
            Choice {
                move_id: String::from("seedflare"),
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
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
            String::from("seismictoss"),
            Choice {
                move_id: String::from("seismictoss"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("selfdestruct"),
            Choice {
                move_id: String::from("selfdestruct"),
                base_power: 200.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                heal: Some(Heal {
                    target: MoveTarget::User,
                    amount: -1.0,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shadowball"),
            Choice {
                move_id: String::from("shadowball"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    bullet: true,
                    mirror: true,
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
            String::from("shadowbone"),
            Choice {
                move_id: String::from("shadowbone"),
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
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
            String::from("shadowclaw"),
            Choice {
                move_id: String::from("shadowclaw"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shadowforce"),
            Choice {
                move_id: String::from("shadowforce"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    charge: true,
                    contact: true,
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shadowpunch"),
            Choice {
                move_id: String::from("shadowpunch"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shadowsneak"),
            Choice {
                move_id: String::from("shadowsneak"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shadowstrike"),
            Choice {
                move_id: String::from("shadowstrike"),
                accuracy: 95.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("sharpen"),
            Choice {
                move_id: String::from("sharpen"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
            String::from("shedtail"),
            Choice {
                move_id: String::from("shedtail"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sheercold"),
            Choice {
                move_id: String::from("sheercold"),
                accuracy: 30.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Ice,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shellsidearm"),
            Choice {
                move_id: String::from("shellsidearm"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shellsmash"),
            Choice {
                move_id: String::from("shellsmash"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
            String::from("shelltrap"),
            Choice {
                move_id: String::from("shelltrap"),
                base_power: 150.0,
                category: MoveCategory::Special,
                priority: -3,
                move_type: PokemonType::Fire,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shelter"),
            Choice {
                move_id: String::from("shelter"),
                target: MoveTarget::User,
                move_type: PokemonType::Steel,
                flags: Flags {
                    snatch: true,
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
            String::from("shiftgear"),
            Choice {
                move_id: String::from("shiftgear"),
                target: MoveTarget::User,
                move_type: PokemonType::Steel,
                flags: Flags {
                    snatch: true,
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
            String::from("shockwave"),
            Choice {
                move_id: String::from("shockwave"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("shoreup"),
            Choice {
                move_id: String::from("shoreup"),
                target: MoveTarget::User,
                move_type: PokemonType::Ground,
                flags: Flags {
                    heal: true,
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("signalbeam"),
            Choice {
                move_id: String::from("signalbeam"),
                base_power: 75.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("silktrap"),
            Choice {
                move_id: String::from("silktrap"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Bug,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SilkTrap,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("silverwind"),
            Choice {
                move_id: String::from("silverwind"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
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
            String::from("simplebeam"),
            Choice {
                move_id: String::from("simplebeam"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sing"),
            Choice {
                move_id: String::from("sing"),
                accuracy: 55.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Sleep,
                }),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sizzlyslide"),
            Choice {
                move_id: String::from("sizzlyslide"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sketch"),
            Choice {
                move_id: String::from("sketch"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("skillswap"),
            Choice {
                move_id: String::from("skillswap"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("skittersmack"),
            Choice {
                move_id: String::from("skittersmack"),
                accuracy: 90.0,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("skullbash"),
            Choice {
                move_id: String::from("skullbash"),
                base_power: 130.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    charge: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("skyattack"),
            Choice {
                move_id: String::from("skyattack"),
                accuracy: 90.0,
                base_power: 140.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    charge: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("skydrop"),
            Choice {
                move_id: String::from("skydrop"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    charge: true,
                    contact: true,
                    distance: true,
                    gravity: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("skyuppercut"),
            Choice {
                move_id: String::from("skyuppercut"),
                accuracy: 90.0,
                base_power: 85.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("slackoff"),
            Choice {
                move_id: String::from("slackoff"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("slam"),
            Choice {
                move_id: String::from("slam"),
                accuracy: 75.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("slash"),
            Choice {
                move_id: String::from("slash"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sleeppowder"),
            Choice {
                move_id: String::from("sleeppowder"),
                accuracy: 75.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Sleep,
                }),
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sleeptalk"),
            Choice {
                move_id: String::from("sleeptalk"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sludge"),
            Choice {
                move_id: String::from("sludge"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sludgebomb"),
            Choice {
                move_id: String::from("sludgebomb"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sludgewave"),
            Choice {
                move_id: String::from("sludgewave"),
                base_power: 95.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("smackdown"),
            Choice {
                move_id: String::from("smackdown"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::SmackDown,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("smartstrike"),
            Choice {
                move_id: String::from("smartstrike"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("smellingsalts"),
            Choice {
                move_id: String::from("smellingsalts"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("smog"),
            Choice {
                move_id: String::from("smog"),
                accuracy: 70.0,
                base_power: 30.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 40.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("smokescreen"),
            Choice {
                move_id: String::from("smokescreen"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("snaptrap"),
            Choice {
                move_id: String::from("snaptrap"),
                base_power: 35.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("snarl"),
            Choice {
                move_id: String::from("snarl"),
                accuracy: 95.0,
                base_power: 55.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
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
            String::from("snatch"),
            Choice {
                move_id: String::from("snatch"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Dark,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Snatch,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("snipeshot"),
            Choice {
                move_id: String::from("snipeshot"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("snore"),
            Choice {
                move_id: String::from("snore"),
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("snowscape"),
            Choice {
                move_id: String::from("snowscape"),
                move_type: PokemonType::Ice,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("soak"),
            Choice {
                move_id: String::from("soak"),
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("softboiled"),
            Choice {
                move_id: String::from("softboiled"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("solarbeam"),
            Choice {
                move_id: String::from("solarbeam"),
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    charge: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("solarblade"),
            Choice {
                move_id: String::from("solarblade"),
                base_power: 125.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    charge: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sonicboom"),
            Choice {
                move_id: String::from("sonicboom"),
                accuracy: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spacialrend"),
            Choice {
                move_id: String::from("spacialrend"),
                accuracy: 95.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spark"),
            Choice {
                move_id: String::from("spark"),
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sparklingaria"),
            Choice {
                move_id: String::from("sparklingaria"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::SparklingAria),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sparklyswirl"),
            Choice {
                move_id: String::from("sparklyswirl"),
                accuracy: 85.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spectralthief"),
            Choice {
                move_id: String::from("spectralthief"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("speedswap"),
            Choice {
                move_id: String::from("speedswap"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spicyextract"),
            Choice {
                move_id: String::from("spicyextract"),
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
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
            String::from("spiderweb"),
            Choice {
                move_id: String::from("spiderweb"),
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spikecannon"),
            Choice {
                move_id: String::from("spikecannon"),
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spikes"),
            Choice {
                move_id: String::from("spikes"),
                move_type: PokemonType::Ground,
                flags: Flags {
                    nonsky: true,
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
            String::from("spikyshield"),
            Choice {
                move_id: String::from("spikyshield"),
                priority: 4,
                target: MoveTarget::User,
                move_type: PokemonType::Grass,
                flags: Flags {
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SpikyShield,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spinout"),
            Choice {
                move_id: String::from("spinout"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spiritbreak"),
            Choice {
                move_id: String::from("spiritbreak"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("spiritshackle"),
            Choice {
                move_id: String::from("spiritshackle"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spite"),
            Choice {
                move_id: String::from("spite"),
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spitup"),
            Choice {
                move_id: String::from("spitup"),
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("splash"),
            Choice {
                move_id: String::from("splash"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    gravity: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("splishysplash"),
            Choice {
                move_id: String::from("splishysplash"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spore"),
            Choice {
                move_id: String::from("spore"),
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Sleep,
                }),
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("spotlight"),
            Choice {
                move_id: String::from("spotlight"),
                priority: 3,
                move_type: PokemonType::Normal,
                flags: Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Spotlight,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("springtidestorm"),
            Choice {
                move_id: String::from("springtidestorm"),
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
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
            String::from("stealthrock"),
            Choice {
                move_id: String::from("stealthrock"),
                move_type: PokemonType::Rock,
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
            String::from("steameruption"),
            Choice {
                move_id: String::from("steameruption"),
                accuracy: 95.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    defrost: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("steamroller"),
            Choice {
                move_id: String::from("steamroller"),
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("steelbeam"),
            Choice {
                move_id: String::from("steelbeam"),
                accuracy: 95.0,
                base_power: 140.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Steel,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("steelroller"),
            Choice {
                move_id: String::from("steelroller"),
                base_power: 130.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("steelwing"),
            Choice {
                move_id: String::from("steelwing"),
                accuracy: 90.0,
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("stickyweb"),
            Choice {
                move_id: String::from("stickyweb"),
                move_type: PokemonType::Bug,
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
            String::from("stockpile"),
            Choice {
                move_id: String::from("stockpile"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::StockPile,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("stomp"),
            Choice {
                move_id: String::from("stomp"),
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("stompingtantrum"),
            Choice {
                move_id: String::from("stompingtantrum"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("stoneaxe"),
            Choice {
                move_id: String::from("stoneaxe"),
                accuracy: 90.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
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
            String::from("stoneedge"),
            Choice {
                move_id: String::from("stoneedge"),
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("storedpower"),
            Choice {
                move_id: String::from("storedpower"),
                base_power: 20.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("stormthrow"),
            Choice {
                move_id: String::from("stormthrow"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("strangesteam"),
            Choice {
                move_id: String::from("strangesteam"),
                accuracy: 95.0,
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("strength"),
            Choice {
                move_id: String::from("strength"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("strengthsap"),
            Choice {
                move_id: String::from("strengthsap"),
                move_type: PokemonType::Grass,
                flags: Flags {
                    heal: true,
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("stringshot"),
            Choice {
                move_id: String::from("stringshot"),
                accuracy: 95.0,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
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
            String::from("struggle"),
            Choice {
                move_id: String::from("struggle"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("strugglebug"),
            Choice {
                move_id: String::from("strugglebug"),
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
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
            String::from("stuffcheeks"),
            Choice {
                move_id: String::from("stuffcheeks"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("stunspore"),
            Choice {
                move_id: String::from("stunspore"),
                accuracy: 75.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Paralyze,
                }),
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("submission"),
            Choice {
                move_id: String::from("submission"),
                accuracy: 80.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.25),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("substitute"),
            Choice {
                move_id: String::from("substitute"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    nonsky: true,
                    snatch: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::Substitute,
                }),
                move_special_effect: Some(|state: &State, side_ref: &SideReference| {
                    let active_pkmn = state.get_side_immutable(side_ref).get_active_immutable();
                    let sub_target_health = active_pkmn.maxhp / 4;
                    if active_pkmn.hp > sub_target_health {
                        return vec![
                            Instruction::Damage(DamageInstruction {
                                side_ref: side_ref.clone(),
                                damage_amount: sub_target_health,
                            }),
                            Instruction::SetSubstituteHealth(SetSubstituteHealthInstruction {
                                side_ref: side_ref.clone(),
                                new_health: sub_target_health,
                                old_health: active_pkmn.substitute_health,
                            }),
                            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                                side_ref: side_ref.clone(),
                                volatile_status: PokemonVolatileStatus::Substitute
                            })
                        ];
                    }

                    return vec![];
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("suckerpunch"),
            Choice {
                move_id: String::from("suckerpunch"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                priority: 1,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sunnyday"),
            Choice {
                move_id: String::from("sunnyday"),
                move_type: PokemonType::Fire,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sunsteelstrike"),
            Choice {
                move_id: String::from("sunsteelstrike"),
                base_power: 100.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Steel,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("superfang"),
            Choice {
                move_id: String::from("superfang"),
                accuracy: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("superpower"),
            Choice {
                move_id: String::from("superpower"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("supersonic"),
            Choice {
                move_id: String::from("supersonic"),
                accuracy: 55.0,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Confusion,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("surf"),
            Choice {
                move_id: String::from("surf"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("surgingstrikes"),
            Choice {
                move_id: String::from("surgingstrikes"),
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("swagger"),
            Choice {
                move_id: String::from("swagger"),
                accuracy: 85.0,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
                    volatile_status: PokemonVolatileStatus::Confusion,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("swallow"),
            Choice {
                move_id: String::from("swallow"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    heal: true,
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sweetkiss"),
            Choice {
                move_id: String::from("sweetkiss"),
                accuracy: 75.0,
                move_type: PokemonType::Fairy,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Confusion,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("sweetscent"),
            Choice {
                move_id: String::from("sweetscent"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("swift"),
            Choice {
                move_id: String::from("swift"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("switcheroo"),
            Choice {
                move_id: String::from("switcheroo"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("swordsdance"),
            Choice {
                move_id: String::from("swordsdance"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    dance: true,
                    snatch: true,
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
        moves.insert(
            String::from("synchronoise"),
            Choice {
                move_id: String::from("synchronoise"),
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("synthesis"),
            Choice {
                move_id: String::from("synthesis"),
                target: MoveTarget::User,
                move_type: PokemonType::Grass,
                flags: Flags {
                    heal: true,
                    snatch: true,
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
            String::from("syrupbomb"),
            Choice {
                move_id: String::from("syrupbomb"),
                accuracy: 85.0,
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Grass,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::SyrupBomb),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tackle"),
            Choice {
                move_id: String::from("tackle"),
                base_power: 40.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tailglow"),
            Choice {
                move_id: String::from("tailglow"),
                target: MoveTarget::User,
                move_type: PokemonType::Bug,
                flags: Flags {
                    snatch: true,
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
        moves.insert(
            String::from("tailslap"),
            Choice {
                move_id: String::from("tailslap"),
                accuracy: 85.0,
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tailwhip"),
            Choice {
                move_id: String::from("tailwhip"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("tailwind"),
            Choice {
                move_id: String::from("tailwind"),
                target: MoveTarget::User,
                move_type: PokemonType::Flying,
                flags: Flags {
                    snatch: true,
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
            String::from("takedown"),
            Choice {
                move_id: String::from("takedown"),
                accuracy: 85.0,
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("takeheart"),
            Choice {
                move_id: String::from("takeheart"),
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tarshot"),
            Choice {
                move_id: String::from("tarshot"),
                move_type: PokemonType::Rock,
                flags: Flags {
                    mirror: true,
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
                    volatile_status: PokemonVolatileStatus::TarShot,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("taunt"),
            Choice {
                move_id: String::from("taunt"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Taunt,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tearfullook"),
            Choice {
                move_id: String::from("tearfullook"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("teatime"),
            Choice {
                move_id: String::from("teatime"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("technoblast"),
            Choice {
                move_id: String::from("technoblast"),
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("teeterdance"),
            Choice {
                move_id: String::from("teeterdance"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    dance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Confusion,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("telekinesis"),
            Choice {
                move_id: String::from("telekinesis"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    gravity: true,
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Telekinesis,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("teleport"),
            Choice {
                move_id: String::from("teleport"),
                priority: -6,
                target: MoveTarget::User,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("terablast"),
            Choice {
                move_id: String::from("terablast"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("terrainpulse"),
            Choice {
                move_id: String::from("terrainpulse"),
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thief"),
            Choice {
                move_id: String::from("thief"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thousandarrows"),
            Choice {
                move_id: String::from("thousandarrows"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::SmackDown,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thousandwaves"),
            Choice {
                move_id: String::from("thousandwaves"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ground,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thrash"),
            Choice {
                move_id: String::from("thrash"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("throatchop"),
            Choice {
                move_id: String::from("throatchop"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::ThroatChop,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thunder"),
            Choice {
                move_id: String::from("thunder"),
                accuracy: 70.0,
                base_power: 110.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thunderbolt"),
            Choice {
                move_id: String::from("thunderbolt"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thundercage"),
            Choice {
                move_id: String::from("thundercage"),
                accuracy: 90.0,
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thunderfang"),
            Choice {
                move_id: String::from("thunderfang"),
                accuracy: 95.0,
                base_power: 65.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    bite: true,
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::Status(PokemonStatus::Paralyze),
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::Opponent,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                    },
                ]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thunderouskick"),
            Choice {
                move_id: String::from("thunderouskick"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("thunderpunch"),
            Choice {
                move_id: String::from("thunderpunch"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thundershock"),
            Choice {
                move_id: String::from("thundershock"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("thunderwave"),
            Choice {
                move_id: String::from("thunderwave"),
                accuracy: 90.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Paralyze,
                }),
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tickle"),
            Choice {
                move_id: String::from("tickle"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
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
            String::from("tidyup"),
            Choice {
                move_id: String::from("tidyup"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
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
            String::from("topsyturvy"),
            Choice {
                move_id: String::from("topsyturvy"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("torchsong"),
            Choice {
                move_id: String::from("torchsong"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
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
            String::from("torment"),
            Choice {
                move_id: String::from("torment"),
                move_type: PokemonType::Dark,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Torment,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("toxic"),
            Choice {
                move_id: String::from("toxic"),
                accuracy: 90.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Toxic,
                }),
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("toxicspikes"),
            Choice {
                move_id: String::from("toxicspikes"),
                move_type: PokemonType::Poison,
                flags: Flags {
                    nonsky: true,
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
            String::from("toxicthread"),
            Choice {
                move_id: String::from("toxicthread"),
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Poison,
                }),
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
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
            String::from("trailblaze"),
            Choice {
                move_id: String::from("trailblaze"),
                base_power: 50.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("transform"),
            Choice {
                move_id: String::from("transform"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("triattack"),
            Choice {
                move_id: String::from("triattack"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("trick"),
            Choice {
                move_id: String::from("trick"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("trickortreat"),
            Choice {
                move_id: String::from("trickortreat"),
                move_type: PokemonType::Ghost,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("trickroom"),
            Choice {
                move_id: String::from("trickroom"),
                priority: -7,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("triplearrows"),
            Choice {
                move_id: String::from("triplearrows"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tripleaxel"),
            Choice {
                move_id: String::from("tripleaxel"),
                accuracy: 90.0,
                base_power: 20.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Ice,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tripledive"),
            Choice {
                move_id: String::from("tripledive"),
                accuracy: 95.0,
                base_power: 30.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("triplekick"),
            Choice {
                move_id: String::from("triplekick"),
                accuracy: 90.0,
                base_power: 10.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("tropkick"),
            Choice {
                move_id: String::from("tropkick"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
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
            String::from("trumpcard"),
            Choice {
                move_id: String::from("trumpcard"),
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("twinbeam"),
            Choice {
                move_id: String::from("twinbeam"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("twineedle"),
            Choice {
                move_id: String::from("twineedle"),
                base_power: 25.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Poison),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("twister"),
            Choice {
                move_id: String::from("twister"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Dragon,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("uproar"),
            Choice {
                move_id: String::from("uproar"),
                base_power: 90.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    sound: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("uturn"),
            Choice {
                move_id: String::from("uturn"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("vacuumwave"),
            Choice {
                move_id: String::from("vacuumwave"),
                base_power: 40.0,
                category: MoveCategory::Special,
                priority: 1,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("vcreate"),
            Choice {
                move_id: String::from("vcreate"),
                accuracy: 95.0,
                base_power: 180.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fire,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("veeveevolley"),
            Choice {
                move_id: String::from("veeveevolley"),
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("venomdrench"),
            Choice {
                move_id: String::from("venomdrench"),
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("venoshock"),
            Choice {
                move_id: String::from("venoshock"),
                base_power: 65.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Poison,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("victorydance"),
            Choice {
                move_id: String::from("victorydance"),
                target: MoveTarget::User,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    dance: true,
                    snatch: true,
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
        moves.insert(
            String::from("vinewhip"),
            Choice {
                move_id: String::from("vinewhip"),
                base_power: 45.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("visegrip"),
            Choice {
                move_id: String::from("visegrip"),
                base_power: 55.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("vitalthrow"),
            Choice {
                move_id: String::from("vitalthrow"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                priority: -1,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("voltswitch"),
            Choice {
                move_id: String::from("voltswitch"),
                base_power: 70.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("volttackle"),
            Choice {
                move_id: String::from("volttackle"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wakeupslap"),
            Choice {
                move_id: String::from("wakeupslap"),
                base_power: 70.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Fighting,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("waterfall"),
            Choice {
                move_id: String::from("waterfall"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("watergun"),
            Choice {
                move_id: String::from("watergun"),
                base_power: 40.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("waterpledge"),
            Choice {
                move_id: String::from("waterpledge"),
                base_power: 80.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    nonsky: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("waterpulse"),
            Choice {
                move_id: String::from("waterpulse"),
                base_power: 60.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    distance: true,
                    mirror: true,
                    protect: true,
                    pulse: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Confusion),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("watershuriken"),
            Choice {
                move_id: String::from("watershuriken"),
                base_power: 15.0,
                category: MoveCategory::Special,
                priority: 1,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("watersport"),
            Choice {
                move_id: String::from("watersport"),
                move_type: PokemonType::Water,
                flags: Flags {
                    nonsky: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("waterspout"),
            Choice {
                move_id: String::from("waterspout"),
                base_power: 150.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wavecrash"),
            Choice {
                move_id: String::from("wavecrash"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Water,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("weatherball"),
            Choice {
                move_id: String::from("weatherball"),
                base_power: 50.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("whirlpool"),
            Choice {
                move_id: String::from("whirlpool"),
                accuracy: 85.0,
                base_power: 35.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Water,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("whirlwind"),
            Choice {
                move_id: String::from("whirlwind"),
                priority: -6,
                move_type: PokemonType::Normal,
                flags: Flags {
                    drag: true,
                    mirror: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wickedblow"),
            Choice {
                move_id: String::from("wickedblow"),
                base_power: 75.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wickedtorque"),
            Choice {
                move_id: String::from("wickedtorque"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Dark,
                flags: Flags {
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 10.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Sleep),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wideguard"),
            Choice {
                move_id: String::from("wideguard"),
                priority: 3,
                target: MoveTarget::User,
                move_type: PokemonType::Rock,
                flags: Flags {
                    snatch: true,
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
            String::from("wildboltstorm"),
            Choice {
                move_id: String::from("wildboltstorm"),
                accuracy: 80.0,
                base_power: 100.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wildcharge"),
            Choice {
                move_id: String::from("wildcharge"),
                base_power: 90.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.25),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("willowisp"),
            Choice {
                move_id: String::from("willowisp"),
                accuracy: 85.0,
                status: Some(Status {
                    target: MoveTarget::Opponent,
                    status: PokemonStatus::Burn,
                }),
                move_type: PokemonType::Fire,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wingattack"),
            Choice {
                move_id: String::from("wingattack"),
                base_power: 60.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Flying,
                flags: Flags {
                    contact: true,
                    distance: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wish"),
            Choice {
                move_id: String::from("wish"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    heal: true,
                    snatch: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("withdraw"),
            Choice {
                move_id: String::from("withdraw"),
                target: MoveTarget::User,
                move_type: PokemonType::Water,
                flags: Flags {
                    snatch: true,
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
            String::from("wonderroom"),
            Choice {
                move_id: String::from("wonderroom"),
                move_type: PokemonType::Psychic,
                flags: Flags {
                    mirror: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("woodhammer"),
            Choice {
                move_id: String::from("woodhammer"),
                base_power: 120.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Grass,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                recoil: Some(0.33),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("workup"),
            Choice {
                move_id: String::from("workup"),
                target: MoveTarget::User,
                move_type: PokemonType::Normal,
                flags: Flags {
                    snatch: true,
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
            String::from("worryseed"),
            Choice {
                move_id: String::from("worryseed"),
                move_type: PokemonType::Grass,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wrap"),
            Choice {
                move_id: String::from("wrap"),
                accuracy: 90.0,
                base_power: 15.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::PartiallyTrapped,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("wringout"),
            Choice {
                move_id: String::from("wringout"),
                category: MoveCategory::Special,
                move_type: PokemonType::Normal,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("xscissor"),
            Choice {
                move_id: String::from("xscissor"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Bug,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        moves.insert(
            String::from("yawn"),
            Choice {
                move_id: String::from("yawn"),
                move_type: PokemonType::Normal,
                flags: Flags {
                    mirror: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                },
                volatile_status: Some(VolatileStatus {
                    target: MoveTarget::Opponent,
                    volatile_status: PokemonVolatileStatus::Yawn,
                }),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("zapcannon"),
            Choice {
                move_id: String::from("zapcannon"),
                accuracy: 50.0,
                base_power: 120.0,
                category: MoveCategory::Special,
                move_type: PokemonType::Electric,
                flags: Flags {
                    bullet: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 100.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Paralyze),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("zenheadbutt"),
            Choice {
                move_id: String::from("zenheadbutt"),
                accuracy: 90.0,
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Psychic,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 20.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("zingzap"),
            Choice {
                move_id: String::from("zingzap"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
                    protect: true,
                    ..Default::default()
                },
                secondaries: Some(vec![Secondary {
                    chance: 30.0,
                    target: MoveTarget::Opponent,
                    effect: Effect::VolatileStatus(PokemonVolatileStatus::Flinch),
                }]),
                ..Default::default()
            },
        );
        moves.insert(
            String::from("zippyzap"),
            Choice {
                move_id: String::from("zippyzap"),
                base_power: 80.0,
                category: MoveCategory::Physical,
                priority: 2,
                move_type: PokemonType::Electric,
                flags: Flags {
                    contact: true,
                    mirror: true,
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

        moves
    };
}

#[derive(Debug, PartialEq, Clone)]
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

impl StatBoosts {
    pub fn get_as_pokemon_boostable(&self) -> [(PokemonBoostableStat, i8); 6] {
        return [
            (PokemonBoostableStat::Attack, self.attack),
            (PokemonBoostableStat::Defense, self.defense),
            (PokemonBoostableStat::SpecialAttack, self.special_attack),
            (PokemonBoostableStat::SpecialDefense, self.special_defense),
            (PokemonBoostableStat::Speed, self.speed),
            (PokemonBoostableStat::Accuracy, self.accuracy),
        ];
    }
}

#[derive(Debug)]
pub struct Myself {
    pub volatile_status: Option<VolatileStatus>,
    pub boosts: StatBoosts,
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub authentic: bool,
    pub bite: bool,
    pub bullet: bool,
    pub charge: bool,
    pub contact: bool,
    pub dance: bool,
    pub defrost: bool,
    pub distance: bool,
    pub drag: bool,
    pub gravity: bool,
    pub heal: bool,
    pub mirror: bool,
    pub mystery: bool,
    pub nonsky: bool,
    pub powder: bool,
    pub protect: bool,
    pub pulse: bool,
    pub punch: bool,
    pub recharge: bool,
    pub reflectable: bool,
    pub snatch: bool,
    pub sound: bool,
}

impl Default for Flags {
    fn default() -> Flags {
        return Flags {
            authentic: false,
            bite: false,
            bullet: false,
            charge: false,
            contact: false,
            dance: false,
            defrost: false,
            distance: true,
            drag: false,
            gravity: false,
            heal: false,
            mirror: false,
            mystery: false,
            nonsky: true,
            powder: false,
            protect: false,
            pulse: false,
            punch: false,
            recharge: false,
            reflectable: false,
            snatch: false,
            sound: false,
        };
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
}

#[derive(Clone)]
pub struct Choice {
    // Basic move information
    pub move_id: String, // in the case of category::Switch, this is not used
    pub switch_id: usize,
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

    // Might not be needed since everything has it's own `target`
    pub target: MoveTarget,

    pub first_move: bool,

    pub modify_move: Option<ModifyChoiceFn>,
    pub after_damage_hit: Option<AfterDamageHitFn>,
    pub hazard_clear: Option<HazardClearFn>,
    pub move_special_effect: Option<MoveSpecialEffectFn>,
}

impl Choice {
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
        self.accuracy = 100.0;
        self.flags.drag = false;
        self.heal = None;
        self.drain = None;
        self.recoil = None;
        self.boost = None;
        self.status = None;
        self.volatile_status = None;
        self.side_condition = None;
        self.secondaries = None;
    }
}

impl Default for Choice {
    fn default() -> Choice {
        return Choice {
            move_id: "splash".to_string(),
            switch_id: 0,
            move_type: PokemonType::Normal,
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
            modify_move: None,
            after_damage_hit: None,
            hazard_clear: None,
            move_special_effect: None,
        };
    }
}
