use std::collections::HashMap;
use std::collections::HashSet;

use super::data::moves::VolatileStatus;
use crate::data::conditions::Status;

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
pub enum Terrain {
    None,
    ElectricTerrain,
    PsychicTerrain,
    MistyTerrain,
    GrassyTerrain,
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
    pub status: Status,
    pub nature: PokemonNatures,
    pub volatile_statuses: HashSet<VolatileStatus>,
    pub moves: Vec<String>,
}

#[derive(Debug)]
pub struct Side {
    pub active_index: usize,
    pub reserve: [Pokemon; 6],
    pub side_conditions: HashMap<String, i8>,
    pub wish: (i8, i16),
}

impl Side {
    pub fn get_active_immutable(&self) -> &Pokemon {
        &self.reserve[self.active_index]
    }

    pub fn switch(&mut self, new_active_index: usize) {
        self.active_index = new_active_index;
    }

}

#[derive(Debug)]
pub struct State {
    pub side_one: Side,
    pub side_two: Side,
    pub weather: Weather,
    pub terrain: Terrain,
    pub trick_room: bool,
}
