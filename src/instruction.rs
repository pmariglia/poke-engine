use crate::choices::Choices;
use crate::items::Items;
use crate::state::PokemonVolatileStatus;
use crate::state::SideReference;
use crate::state::Terrain;
use crate::state::Weather;
use crate::state::{PokemonBoostableStat, PokemonType};
use crate::state::{PokemonIndex, PokemonSideCondition};
use crate::state::{PokemonMoveIndex, PokemonStatus};
use std::fmt;

#[derive(PartialEq, Clone)]
pub struct StateInstructions {
    pub percentage: f32,
    pub instruction_list: Vec<Instruction>,
}

impl Default for StateInstructions {
    fn default() -> StateInstructions {
        return StateInstructions {
            percentage: 100.0,
            instruction_list: Vec::with_capacity(16),
        };
    }
}

impl fmt::Debug for StateInstructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut final_string = format!("\n\tPercentage: {}", self.percentage);
        for i in self.instruction_list.iter() {
            final_string.push_str(format!("\n\t\t{:?}", i).as_str());
        }
        write!(f, "{}\n", final_string)
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
    IncrementWish(IncrementWishInstruction),
    DecrementWish(DecrementWishInstruction),
    DamageSubstitute(DamageInstruction),
    SetSubstituteHealth(SetSubstituteHealthInstruction),
    SetSideOneMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction),
    SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction),
    ToggleSideOneSwitchOutMove,
    ToggleSideTwoSwitchOutMove,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetSecondMoveSwitchOutMoveInstruction {
    pub new_choice: Choices,
    pub previous_choice: Choices,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IncrementWishInstruction {
    pub side_ref: SideReference,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecrementWishInstruction {
    pub side_ref: SideReference,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnableMoveInstruction {
    pub side_ref: SideReference,
    pub move_index: PokemonMoveIndex,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DisableMoveInstruction {
    pub side_ref: SideReference,
    pub move_index: PokemonMoveIndex,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeItemInstruction {
    pub side_ref: SideReference,
    pub current_item: Items,
    pub new_item: Items,
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
pub struct SetSubstituteHealthInstruction {
    pub side_ref: SideReference,
    pub new_health: i16,
    pub old_health: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchInstruction {
    pub side_ref: SideReference,
    pub previous_index: PokemonIndex,
    pub next_index: PokemonIndex,
}

// pokemon_index is present because even reserve pokemon can have their status
// changed (i.e. healbell)
#[derive(Debug, PartialEq, Clone)]
pub struct ChangeStatusInstruction {
    pub side_ref: SideReference,
    pub pokemon_index: PokemonIndex,
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
