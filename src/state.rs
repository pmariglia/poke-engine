use serde::{Deserialize, Serialize};

use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::data::conditions::PokemonSideCondition;
use crate::data::conditions::PokemonVolatileStatus;
use crate::data::conditions::PokemonStatus;
use crate::instruction::Instruction;

#[derive(Debug)]
pub enum SideReference {
    SideOne,
    SideTwo
}

#[derive(Debug, PartialEq)]
pub enum Weather {
    None,
    Sun,
    Rain,
    Sand,
    Hail,
    HarshSun,
    HeavyRain,
}

#[derive(Debug, PartialEq)]
pub struct StateWeather {
    pub weather_type: Weather,
    pub turns_remaining: i8,
}

#[derive(Debug, PartialEq)]
pub enum Terrain {
    None,
    ElectricTerrain,
    PsychicTerrain,
    MistyTerrain,
    GrassyTerrain,
}

#[derive(Debug, PartialEq)]
pub struct StateTerrain {
    pub terrain_type: Terrain,
    pub turns_remaining: i8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PokemonTypes {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
    Typeless,
}

#[derive(Debug)]
pub enum PokemonNatures {
    Lonely,
    Adamant,
    Naughty,
    Brave,
    Bold,
    Impish,
    Lax,
    Relaxed,
    Modest,
    Mild,
    Rash,
    Quiet,
    Calm,
    Gentle,
    Careful,
    Sassy,
    Timid,
    Hasty,
    Jolly,
    Naive,

    // Neutral Natures
    Hardy,
    Docile,
    Bashful,
    Quirky,
    Serious,
}

#[derive(Debug)]
pub enum PokemonBoostableStat {
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
    Evasion,
    Accuracy,
}

#[derive(Debug)]
pub struct Pokemon {
    pub id: String,
    pub level: i8,
    pub types: (PokemonTypes, PokemonTypes),
    pub hp: i16,
    pub maxhp: i16,
    pub ability: String,
    pub item: String,
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
    pub attack_boost: i8,
    pub defense_boost: i8,
    pub special_attack_boost: i8,
    pub special_defense_boost: i8,
    pub speed_boost: i8,
    pub accuracy_boost: i8,
    pub evasion_boost: i8,
    pub status: PokemonStatus,
    pub nature: PokemonNatures,
    pub volatile_statuses: HashSet<PokemonVolatileStatus>,
    pub moves: Vec<String>,
}

impl Pokemon {
    pub fn clear_volatile_statuses(&mut self) {
        self.volatile_statuses.clear();
    }
}

#[derive(Debug)]
pub struct Side {
    pub active_index: usize,
    pub reserve: [Pokemon; 1],
    pub side_conditions: HashMap<PokemonSideCondition, i8>,
    pub wish: (i8, i16),
}

impl Side {
    pub fn get_active(&mut self) -> &mut Pokemon {
        &mut self.reserve[self.active_index]
    }

    pub fn get_active_immutable(&self) -> &Pokemon {
        &self.reserve[self.active_index]
    }
}

#[derive(Debug)]
pub struct State {
    pub side_one: Side,
    pub side_two: Side,
    pub weather: StateWeather,
    pub terrain: StateTerrain,
    pub trick_room: bool,
}

impl State {

    pub fn get_side(&mut self, side_ref: SideReference) -> &mut Side {
        match side_ref {
            SideReference::SideOne => {
                return &mut self.side_one
            }
            SideReference::SideTwo => {
                return &mut self.side_two
            }
        }
    }

    pub fn damage(&mut self, side_ref: SideReference, amount: i16) {
        let active = self
        .get_side(side_ref)
        .get_active();
        
        active.hp -= amount;
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Damage(instruction) => {
                self.damage(instruction.side_ref, instruction.damage_amount)
            }
            _ => {panic!("Not implemented yet")}
        }
    }

}
