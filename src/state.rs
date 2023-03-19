use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::collections::HashSet;

use super::data::moves::SideCondition;
use super::data::moves::VolatileStatus;
use crate::data::conditions::PokemonSideCondition;
use crate::data::conditions::PokemonVolatileStatus;
use crate::data::conditions::PokemonStatus;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Weather {
    None,
    Sun,
    Rain,
    Sand,
    Hail,
    HarshSun,
    HeavyRain,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StateWeather {
    pub weather_type: Weather,
    pub turns_remaining: i8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Terrain {
    None,
    ElectricTerrain,
    PsychicTerrain,
    MistyTerrain,
    GrassyTerrain,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StateTerrain {
    pub terrain_type: Terrain,
    pub turns_remaining: i8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SideReference {
    SideOne,
    SideTwo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

    // heals by amount, but doesn't overheal
    // returns the amount healed
    // a fainted pokemon (hp=0) cannot be healed
    pub fn heal(&mut self, mut amount: i16) -> i16 {
        if self.hp == 0 {
            return 0;
        }
        self.hp += amount;
        if self.hp > self.maxhp {
            amount -= self.hp - self.maxhp;
            self.hp = self.maxhp;
        }
        return amount;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Side {
    pub active_index: usize,
    pub reserve: [Pokemon; 6],
    pub side_conditions: HashMap<PokemonSideCondition, i8>,
    pub wish: (i8, i16),
}

impl Side {
    pub fn get_active_immutable(&self) -> &Pokemon {
        &self.reserve[self.active_index]
    }

    pub fn _switch(&mut self, new_active_index: usize) {
        self.active_index = new_active_index;
    }

    pub fn switch_to_name(&mut self, new_active_name: &String) {
        let new_index = self
            .reserve
            .iter()
            .position(|r| &r.id == new_active_name)
            .unwrap();
        self._switch(new_index);
    }

    pub fn apply_volatile_status(&mut self, volatile_status: VolatileStatus) {
        self.reserve[self.active_index]
            .volatile_statuses
            .insert(volatile_status);
    }

    pub fn remove_volatile_status(&mut self, volatile_status: &VolatileStatus) {
        self.reserve[self.active_index]
            .volatile_statuses
            .remove(volatile_status);
    }
    pub fn remove_all_volatile_statuses(&mut self) {
        self.reserve[self.active_index].volatile_statuses.clear();
    }

    pub fn damage(&mut self, damage_amount: i16) {
        self.reserve[self.active_index].hp -= damage_amount;
    }

    pub fn heal(&mut self, heal_amount: i16) {
        self.reserve[self.active_index].hp += heal_amount;
    }

    pub fn boost(&mut self, stat: PokemonBoostableStat, amount: i8) {
        match stat {
            PokemonBoostableStat::Attack => {
                self.reserve[self.active_index].attack_boost += amount;
            }
            PokemonBoostableStat::Defense => {
                self.reserve[self.active_index].defense_boost += amount;
            }
            PokemonBoostableStat::SpecialAttack => {
                self.reserve[self.active_index].special_attack_boost += amount;
            }
            PokemonBoostableStat::SpecialDefense => {
                self.reserve[self.active_index].special_defense_boost += amount;
            }
            PokemonBoostableStat::Speed => {
                self.reserve[self.active_index].speed_boost += amount;
            }
            PokemonBoostableStat::Accuracy => {
                self.reserve[self.active_index].accuracy_boost += amount;
            }
            PokemonBoostableStat::Evasion => {
                self.reserve[self.active_index].evasion_boost += amount;
            }
        }
    }

    pub fn unboost(&mut self, stat: PokemonBoostableStat, amount: i8) {
        Side::boost(self, stat, -1 * amount);
    }

    pub fn set_status(&mut self, status: PokemonStatus) {
        self.reserve[self.active_index].status = status;
    }

    pub fn increment_side_condition(&mut self, side_condition: SideCondition, amount: i8) {
        *self.side_conditions.entry(side_condition).or_insert(0) += amount;
    }

    pub fn decrement_side_condition(&mut self, side_condition: SideCondition, amount: i8) {
        Side::increment_side_condition(self, side_condition, -1 * amount)
    }

    pub fn start_wish(&mut self, health: i16) {
        self.wish = (2, health);
    }

    pub fn decrement_wish(&mut self) {
        self.wish = (self.wish.0 - 1, self.wish.1);
    }

    pub fn change_types(&mut self, new_types: (PokemonTypes, PokemonTypes)) {
        self.reserve[self.active_index].types = new_types;
    }

    pub fn change_item(&mut self, new_item: String) {
        self.reserve[self.active_index].item = new_item;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub side_one: Side,
    pub side_two: Side,
    pub weather: StateWeather,
    pub terrain: StateTerrain,
    pub trick_room: bool,
}

impl State {
    pub fn change_weather(&mut self, weather_type: Weather, turns_active: i8) {
        self.weather.weather_type = weather_type;
        self.weather.turns_remaining = turns_active;
    }

    pub fn change_terrain(&mut self, terrain_type: Terrain, turns_active: i8) {
        self.terrain.terrain_type = terrain_type;
        self.terrain.turns_remaining = turns_active
    }

    pub fn toggle_trickroom(&mut self) {
        self.trick_room ^= true;
    }
}
