use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::data::conditions::PokemonSideCondition;
use crate::data::conditions::PokemonStatus;
use crate::data::conditions::PokemonVolatileStatus;
use crate::instruction::Instruction;

#[derive(Debug)]
pub enum SideReference {
    SideOne,
    SideTwo,
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
pub struct SideConditions {
    pub aurora_veil: i8,
    pub crafty_shield: i8,
    pub healing_wish: i8,
    pub light_screen: i8,
    pub lucky_chant: i8,
    pub lunar_dance: i8,
    pub mat_block: i8,
    pub mist: i8,
    pub quick_guard: i8,
    pub reflect: i8,
    pub safeguard: i8,
    pub spikes: i8,
    pub stealth_rock: i8,
    pub sticky_web: i8,
    pub tailwind: i8,
    pub toxic_spikes: i8,
    pub wide_guard: i8,
    pub wish: i8,
}

impl Default for SideConditions {
    fn default() -> SideConditions {
        SideConditions {
            aurora_veil: 0,
            crafty_shield: 0,
            healing_wish: 0,
            light_screen: 0,
            lucky_chant: 0,
            lunar_dance: 0,
            mat_block: 0,
            mist: 0,
            quick_guard: 0,
            reflect: 0,
            safeguard: 0,
            spikes: 0,
            stealth_rock: 0,
            sticky_web: 0,
            tailwind: 0,
            toxic_spikes: 0,
            wide_guard: 0,
            wish: 0,
        }
    }
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
    pub side_conditions: SideConditions,
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
    pub fn get_side(&mut self, side_ref: &SideReference) -> &mut Side {
        match side_ref {
            SideReference::SideOne => return &mut self.side_one,
            SideReference::SideTwo => return &mut self.side_two,
        }
    }

    pub fn damage(&mut self, side_ref: &SideReference, amount: i16) {
        let active = self.get_side(&side_ref).get_active();

        active.hp -= amount;
    }

    pub fn heal(&mut self, side_ref: &SideReference, amount: i16) {
        let active = self.get_side(&side_ref).get_active();

        active.hp += amount;
    }

    pub fn switch(&mut self, side_ref: &SideReference, next_active_index: usize, _: usize) {
        let side = self.get_side(&side_ref);
        side.active_index = next_active_index;
    }

    pub fn reverse_switch(
        &mut self,
        side_ref: &SideReference,
        _: usize,
        previous_active_index: usize,
    ) {
        let side = self.get_side(&side_ref);
        side.active_index = previous_active_index;
    }

    pub fn apply_volatile_status(
        &mut self,
        side_ref: &SideReference,
        volatile_status: PokemonVolatileStatus,
    ) {
        let active = self.get_side(&side_ref).get_active();
        active.volatile_statuses.insert(volatile_status);
    }

    pub fn remove_volatile_status(
        &mut self,
        side_ref: &SideReference,
        volatile_status: PokemonVolatileStatus,
    ) {
        let active = self.get_side(&side_ref).get_active();
        active.volatile_statuses.remove(&volatile_status);
    }

    pub fn change_status(
        &mut self,
        side_ref: &SideReference,
        pokemon_index: usize,
        new_status: PokemonStatus,
    ) {
        let mut pkmn = &mut self.get_side(&side_ref).reserve[pokemon_index];
        pkmn.status = new_status;
    }

    pub fn apply_boost(
        &mut self,
        side_ref: &SideReference,
        stat: &PokemonBoostableStat,
        amount: i8,
    ) {
        let active = self.get_side(&side_ref).get_active();
        match stat {
            PokemonBoostableStat::Attack => active.attack_boost += amount,
            PokemonBoostableStat::Defense => active.defense_boost += amount,
            PokemonBoostableStat::SpecialAttack => active.special_attack_boost += amount,
            PokemonBoostableStat::SpecialDefense => active.special_defense_boost += amount,
            PokemonBoostableStat::Speed => active.speed_boost += amount,
            PokemonBoostableStat::Evasion => active.evasion_boost += amount,
            PokemonBoostableStat::Accuracy => active.accuracy_boost += amount,
        }
    }

    pub fn increment_side_condition(
        &mut self,
        side_ref: &SideReference,
        side_condition: &PokemonSideCondition,
        amount: i8,
    ) {
        let side = self.get_side(&side_ref);

        match side_condition {
            PokemonSideCondition::AuroraVeil => side.side_conditions.aurora_veil += amount,
            PokemonSideCondition::CraftyShield => side.side_conditions.crafty_shield += amount,
            PokemonSideCondition::HealingWish => side.side_conditions.healing_wish += amount,
            PokemonSideCondition::LightScreen => side.side_conditions.light_screen += amount,
            PokemonSideCondition::LuckyChant => side.side_conditions.lucky_chant += amount,
            PokemonSideCondition::LunarDance => side.side_conditions.lunar_dance += amount,
            PokemonSideCondition::MatBlock => side.side_conditions.mat_block += amount,
            PokemonSideCondition::Mist => side.side_conditions.mist += amount,
            PokemonSideCondition::QuickGuard => side.side_conditions.quick_guard += amount,
            PokemonSideCondition::Reflect => side.side_conditions.reflect += amount,
            PokemonSideCondition::Safeguard => side.side_conditions.safeguard += amount,
            PokemonSideCondition::Spikes => side.side_conditions.spikes += amount,
            PokemonSideCondition::Stealthrock => side.side_conditions.stealth_rock += amount,
            PokemonSideCondition::StickyWeb => side.side_conditions.sticky_web += amount,
            PokemonSideCondition::Tailwind => side.side_conditions.tailwind += amount,
            PokemonSideCondition::ToxicSpikes => side.side_conditions.toxic_spikes += amount,
            PokemonSideCondition::WideGuard => side.side_conditions.wide_guard += amount,
            PokemonSideCondition::Wish => side.side_conditions.wish += amount,
        }
    }

    pub fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Damage(instruction) => {
                self.damage(&instruction.side_ref, instruction.damage_amount)
            }
            Instruction::Switch(instruction) => self.switch(
                &instruction.side_ref,
                instruction.next_index,
                instruction.previous_index,
            ),
            Instruction::VolatileStatus(instruction) => {
                self.apply_volatile_status(&instruction.side_ref, instruction.volatile_status)
            }
            Instruction::ChangeStatus(instruction) => self.change_status(
                &instruction.side_ref,
                instruction.pokemon_index,
                instruction.new_status,
            ),
            Instruction::Boost(instruction) => {
                self.apply_boost(&instruction.side_ref, &instruction.stat, instruction.amount)
            }
            Instruction::ChangeSideCondition(instruction) => self.increment_side_condition(
                &instruction.side_ref,
                &instruction.side_condition,
                instruction.amount,
            ),
            _ => {
                panic!("Not implemented yet")
            }
        }
    }

    pub fn reverse_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Damage(instruction) => {
                self.heal(&instruction.side_ref, instruction.damage_amount)
            }
            Instruction::Switch(instruction) => self.reverse_switch(
                &instruction.side_ref,
                instruction.next_index,
                instruction.previous_index,
            ),
            Instruction::VolatileStatus(instruction) => {
                self.remove_volatile_status(&instruction.side_ref, instruction.volatile_status)
            }
            Instruction::ChangeStatus(instruction) => self.change_status(
                &instruction.side_ref,
                instruction.pokemon_index,
                instruction.old_status,
            ),
            Instruction::Boost(instruction) => self.apply_boost(
                &instruction.side_ref,
                &instruction.stat,
                -1 * instruction.amount,
            ),
            Instruction::ChangeSideCondition(instruction) => self.increment_side_condition(
                &instruction.side_ref,
                &instruction.side_condition,
                -1 * instruction.amount,
            ),
            _ => {
                panic!("Not implemented yet")
            }
        }
    }
}
