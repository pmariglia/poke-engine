use crate::state::PokemonSideCondition;
use crate::state::PokemonStatus;
use crate::state::PokemonVolatileStatus;
use crate::state::SideReference;
use crate::state::Terrain;
use crate::state::Weather;
use crate::state::{PokemonBoostableStat, PokemonType};

#[derive(Debug, PartialEq, Clone)]
pub struct StateInstructions {
    pub percentage: f32,
    pub instruction_list: Vec<Instruction>,
}

impl Default for StateInstructions {
    fn default() -> StateInstructions {
        return StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };
    }
}

impl StateInstructions {
    pub fn update_percentage(&mut self, modifier: f32) {
        self.percentage *= modifier;
    }
}

// https://stackoverflow.com/questions/50686411/whats-the-usual-way-to-create-a-vector-of-different-structs
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Switch(SwitchInstruction),
    ApplyVolatileStatus(ApplyVolatileStatusInstruction),
    RemoveVolatileStatus(RemoveVolatileStatusInstruction),
    ChangeStatus(ChangeStatusInstruction),
    Heal(HealInstruction),
    Damage(DamageInstruction),
    Boost(BoostInstruction),
    ChangeSideCondition(ChangeSideConditionInstruction),
    ChangeWeather(ChangeWeather),
    ChangeTerrain(ChangeTerrain),
    ChangeType(ChangeType),
    ChangeItem(ChangeItemInstruction),
    DisableMove(DisableMoveInstruction),
    EnableMove(EnableMoveInstruction),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnableMoveInstruction {
    pub side_ref: SideReference,
    pub move_index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DisableMoveInstruction {
    pub side_ref: SideReference,
    pub move_index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeItemInstruction {
    pub side_ref: SideReference,
    pub current_item: String,
    pub new_item: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HealInstruction {
    pub side_ref: SideReference,
    pub heal_amount: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DamageInstruction {
    pub side_ref: SideReference,
    pub damage_amount: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchInstruction {
    pub side_ref: SideReference,
    pub previous_index: usize,
    pub next_index: usize,
}

// pokemon_index is present because even reserve pokemon can have their status
// changed (i.e. healbell)
#[derive(Debug, PartialEq, Clone)]
pub struct ChangeStatusInstruction {
    pub side_ref: SideReference,
    pub pokemon_index: usize,
    pub old_status: PokemonStatus,
    pub new_status: PokemonStatus,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ApplyVolatileStatusInstruction {
    pub side_ref: SideReference,
    pub volatile_status: PokemonVolatileStatus,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RemoveVolatileStatusInstruction {
    pub side_ref: SideReference,
    pub volatile_status: PokemonVolatileStatus,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BoostInstruction {
    pub side_ref: SideReference,
    pub stat: PokemonBoostableStat,
    pub amount: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeSideConditionInstruction {
    pub side_ref: SideReference,
    pub side_condition: PokemonSideCondition,
    pub amount: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeWeather {
    pub new_weather: Weather,
    pub new_weather_turns_remaining: i8,
    pub previous_weather: Weather,
    pub previous_weather_turns_remaining: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeTerrain {
    pub new_terrain: Terrain,
    pub new_terrain_turns_remaining: i8,
    pub previous_terrain: Terrain,
    pub previous_terrain_turns_remaining: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeType {
    pub side_ref: SideReference,
    pub new_types: (PokemonType, PokemonType),
    pub old_types: (PokemonType, PokemonType),
}
