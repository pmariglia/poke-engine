use crate::data::conditions::PokemonStatus;
use crate::data::conditions::PokemonVolatileStatus;
use crate::state::PokemonTypes;
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::conditions::PokemonSideCondition;

// type ModifyPriorityFn = fn(&State) -> i8;

lazy_static! {
    static ref MOVES: HashMap<String, Choice> = {
        let mut moves: HashMap<String, Choice> = HashMap::new();

        moves.insert(
            "tackle".to_string(),
            Choice {
                id: "tackle".to_string(),
                accuracy: 100 as f32,
                base_power: 40 as f32,
                category: MoveCategory::Status,
                status: None,
                priority: 0,
                target: MoveTarget::Opponent,
                move_type: PokemonTypes::Ground,
                flags: Flags {
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
                },
                secondaries: Some(vec![Secondary {
                    chance: 0,
                    target: MoveTarget::Opponent,
                    effect: Effect::Status(PokemonStatus::Burn),
                }]),
                boost: None,
                volatile_status: None,
                side_condition: None,
                heal: None,
            },
        );

        moves
    };
}

#[derive(Debug, PartialEq)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
    Switch,
}

#[derive(Debug)]
pub enum MoveTarget {
    User,
    Opponent,
}

#[derive(Debug)]
pub struct VolatileStatus {
    target: MoveTarget,
    volatile_status: PokemonVolatileStatus,
}

#[derive(Debug)]
pub struct SideCondition {
    target: MoveTarget,
    condition: PokemonSideCondition,
}

#[derive(Debug)]
pub struct Boost {
    target: MoveTarget,
    boosts: StatBoosts,
}

#[derive(Debug)]
pub struct Heal {
    target: MoveTarget,
    amount_percentage: f32,
}

#[derive(Debug)]
pub struct Status {
    target: MoveTarget,
    status: PokemonStatus,
}

#[derive(Debug)]
pub struct StatBoosts {
    attack: i8,
    defense: i8,
    special_attack: i8,
    special_defense: i8,
    speed: i8,
    accuracy: i8,
}

#[derive(Debug)]
pub struct Myself {
    pub volatile_status: Option<VolatileStatus>,
    pub boosts: StatBoosts,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Secondary {
    pub chance: i8,
    pub target: MoveTarget,
    pub effect: Effect,
}

#[derive(Debug)]
pub enum Effect {
    VolatileStatus(PokemonVolatileStatus),
    SideCondition(PokemonSideCondition),
    Boost(StatBoosts),
    Heal(Heal),
    Status(PokemonStatus),
}

#[derive(Debug)]
pub struct Choice {
    // Basic move information
    pub id: String, // in the case of category::Switch, this is the name of the pokemon being switched into
    pub move_type: PokemonTypes,
    pub accuracy: f32,
    pub category: MoveCategory,
    pub base_power: f32,
    pub boost: Option<Boost>,
    pub priority: i8,
    pub flags: Flags,
    pub heal: Option<Heal>,
    pub status: Option<Status>,
    pub volatile_status: Option<VolatileStatus>,
    pub side_condition: Option<SideCondition>,
    pub secondaries: Option<Vec<Secondary>>,

    // Might not be needed since everything has it's own `target`
    pub target: MoveTarget, // Things that

                            // pub myself: Myself,
                            // pub target: MoveTarget,
                            // pub move_type: PokemonTypes,
                            // pub pp: i8,
                            // pub crash: Option<f32>,
                            // pub drain: Option<f32>,
                            // pub recoil: Option<f32>,
                            // pub modify_priority: Option<ModifyPriorityFn>,
}
