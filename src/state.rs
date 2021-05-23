use std::collections::HashMap;
use std::collections::HashSet;

use super::data::moves::SideCondition;
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
    pub status: Status,
    pub nature: PokemonNatures,
    pub volatile_statuses: HashSet<VolatileStatus>,
    pub moves: Vec<String>,
}

#[derive(Debug)]
pub struct Side {
    pub active_index: usize,
    pub reserve: [Pokemon; 6],
    pub side_conditions: HashMap<SideCondition, i8>,
    pub wish: (i8, i16),
}

impl Side {
    pub fn get_active_immutable(&self) -> &Pokemon {
        &self.reserve[self.active_index]
    }

    pub fn switch(&mut self, new_active_index: usize) {
        self.active_index = new_active_index;
    }

    pub fn apply_volatile_status(&mut self, volatile_status: VolatileStatus) {
        self.reserve[self.active_index]
            .volatile_statuses
            .insert(volatile_status);
    }

    pub fn remove_volatile_status(&mut self, volatile_status: VolatileStatus) {
        self.reserve[self.active_index]
            .volatile_statuses
            .remove(&volatile_status);
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

    pub fn set_status(&mut self, status: Status) {
        self.reserve[self.active_index].status = status;
    }

    pub fn increment_side_condition(&mut self, side_condition: SideCondition, amount: i8) {
        *self.side_conditions.entry(side_condition).or_insert(0) += amount;
    }

    pub fn decrement_side_condition(&mut self, side_condition: SideCondition, amount: i8) {
        Side::increment_side_condition(self, side_condition, -1 * amount)
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

mod test {
    use super::super::helpers::create_dummy_state;
    use super::State;

    use super::super::data::moves::SideCondition;

    #[test]
    fn test_increment_side_condition() {
        let mut state: State = create_dummy_state();

        state
            .side_one
            .increment_side_condition(SideCondition::Tailwind, 5);

        let tw: i8 = *state
            .side_one
            .side_conditions
            .get(&SideCondition::Tailwind)
            .unwrap();

        assert_eq!(tw, 5);
    }

    #[test]
    fn test_increment_side_condition_when_value_already_exists() {
        let mut state: State = create_dummy_state();

        state
            .side_one
            .side_conditions
            .insert(SideCondition::Tailwind, 1);
        state
            .side_one
            .increment_side_condition(SideCondition::Tailwind, 3);

        let tw: i8 = *state
            .side_one
            .side_conditions
            .get(&SideCondition::Tailwind)
            .unwrap();

        assert_eq!(tw, 4);
    }
}
