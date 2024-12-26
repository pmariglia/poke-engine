use crate::choices::{Choices, MoveCategory};
use crate::items::Items;
use crate::state::Terrain;
use crate::state::Weather;
use crate::state::{FormeChange, SideReference};
use crate::state::{LastUsedMove, PokemonVolatileStatus};
use crate::state::{PokemonBoostableStat, PokemonType};
use crate::state::{PokemonIndex, PokemonSideCondition};
use crate::state::{PokemonMoveIndex, PokemonStatus};
use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Clone)]
pub struct StateInstructions {
    pub percentage: f32,
    pub instruction_list: Vec<Instruction>,
}

impl Default for StateInstructions {
    fn default() -> StateInstructions {
        StateInstructions {
            percentage: 100.0,
            instruction_list: Vec::with_capacity(16),
        }
    }
}

impl fmt::Debug for StateInstructions {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut final_string = format!("\n\tPercentage: {}\n\tInstructions:", self.percentage);
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
#[derive(PartialEq, Clone)]
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
    DecrementWeatherTurnsRemaining,
    ChangeTerrain(ChangeTerrain),
    DecrementTerrainTurnsRemaining,
    ChangeType(ChangeType),
    ChangeItem(ChangeItemInstruction),
    DisableMove(DisableMoveInstruction),
    EnableMove(EnableMoveInstruction),
    ChangeWish(ChangeWishInstruction),
    DecrementWish(DecrementWishInstruction),
    SetFutureSight(SetFutureSightInstruction),
    DecrementFutureSight(DecrementFutureSightInstruction),
    DamageSubstitute(DamageInstruction),
    DecrementRestTurns(DecrementRestTurnsInstruction),
    SetRestTurns(SetSleepTurnsInstruction),
    SetSleepTurns(SetSleepTurnsInstruction),
    ChangeSubstituteHealth(ChangeSubsituteHealthInstruction),
    FormeChange(FormeChangeInstruction),
    SetSideOneMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction),
    SetSideTwoMoveSecondSwitchOutMove(SetSecondMoveSwitchOutMoveInstruction),
    ToggleBatonPassing(ToggleBatonPassingInstruction),
    SetLastUsedMove(SetLastUsedMoveInstruction),
    SetDamageDealtSideOne(SetDamageDealtSideOneInstruction),
    SetDamageDealtSideTwo(SetDamageDealtSideTwoInstruction),
    DecrementPP(DecrementPPInstruction),
    ToggleTrickRoom(ToggleTrickRoomInstruction),
    DecrementTrickRoomTurnsRemaining,
    ToggleSideOneForceSwitch,
    ToggleSideTwoForceSwitch,
    ToggleTerastallized(ToggleTerastallizedInstruction),
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Instruction::Switch(s) => {
                write!(
                    f,
                    "Switch {:?}: {:?} -> {:?}",
                    s.side_ref, s.previous_index, s.next_index
                )
            }
            Instruction::ApplyVolatileStatus(a) => {
                write!(
                    f,
                    "ApplyVolatileStatus {:?}: {:?}",
                    a.side_ref, a.volatile_status
                )
            }
            Instruction::RemoveVolatileStatus(r) => {
                write!(
                    f,
                    "RemoveVolatileStatus {:?}: {:?}",
                    r.side_ref, r.volatile_status
                )
            }
            Instruction::ChangeStatus(c) => {
                write!(
                    f,
                    "ChangeStatus {:?}-{:?}: {:?} -> {:?}",
                    c.side_ref, c.pokemon_index, c.old_status, c.new_status
                )
            }
            Instruction::Heal(h) => {
                write!(f, "Heal {:?}: {:?}", h.side_ref, h.heal_amount)
            }
            Instruction::Damage(d) => {
                write!(f, "Damage {:?}: {}", d.side_ref, d.damage_amount)
            }
            Instruction::Boost(b) => {
                write!(f, "Boost {:?} {:?}: {:?}", b.side_ref, b.stat, b.amount)
            }
            Instruction::ChangeSideCondition(c) => {
                write!(
                    f,
                    "ChangeSideCondition {:?} {:?}: {:?}",
                    c.side_ref, c.side_condition, c.amount
                )
            }
            Instruction::ChangeWeather(c) => {
                write!(
                    f,
                    "ChangeWeather: {:?},{:?} -> {:?},{:?}",
                    c.previous_weather,
                    c.previous_weather_turns_remaining,
                    c.new_weather,
                    c.new_weather_turns_remaining
                )
            }
            Instruction::DecrementWeatherTurnsRemaining => {
                write!(f, "DecrementWeatherTurnsRemaining",)
            }
            Instruction::ChangeTerrain(c) => {
                write!(
                    f,
                    "ChangeTerrain: {:?},{:?} -> {:?},{:?}",
                    c.previous_terrain,
                    c.previous_terrain_turns_remaining,
                    c.new_terrain,
                    c.new_terrain_turns_remaining
                )
            }
            Instruction::DecrementTerrainTurnsRemaining => {
                write!(f, "DecrementTerrainTurnsRemaining",)
            }
            Instruction::ChangeType(c) => {
                write!(
                    f,
                    "ChangeType {:?}: {:?} -> {:?}",
                    c.side_ref, c.old_types, c.new_types
                )
            }
            Instruction::ChangeItem(c) => {
                write!(
                    f,
                    "ChangeItem {:?}: {:?} -> {:?}",
                    c.side_ref, c.current_item, c.new_item
                )
            }
            Instruction::DisableMove(d) => {
                write!(f, "DisableMove {:?}: {:?}", d.side_ref, d.move_index)
            }
            Instruction::EnableMove(e) => {
                write!(f, "EnableMove {:?}: {:?}", e.side_ref, e.move_index)
            }
            Instruction::ChangeWish(s) => {
                write!(f, "SetWish {:?}: {:?}", s.side_ref, s.wish_amount_change)
            }
            Instruction::DecrementWish(d) => {
                write!(f, "DecrementWish {:?}", d.side_ref)
            }
            Instruction::SetFutureSight(s) => {
                write!(
                    f,
                    "SetFutureSight {:?}: {:?} -> {:?}",
                    s.side_ref, s.previous_pokemon_index, s.pokemon_index
                )
            }
            Instruction::DecrementFutureSight(d) => {
                write!(f, "DecrementFutureSight {:?}", d.side_ref)
            }
            Instruction::DamageSubstitute(d) => {
                write!(
                    f,
                    "DamageSubstitute {:?}: {:?}",
                    d.side_ref, d.damage_amount
                )
            }
            Instruction::DecrementRestTurns(d) => {
                write!(f, "DecrementRestTurns {:?}", d.side_ref)
            }
            Instruction::SetRestTurns(s) => {
                write!(
                    f,
                    "SetRestTurns {:?}-{:?}: {:?} -> {:?}",
                    s.side_ref, s.pokemon_index, s.previous_turns, s.new_turns
                )
            }
            Instruction::SetSleepTurns(s) => {
                write!(
                    f,
                    "SetSleepTurns {:?}-{:?}: {:?} -> {:?}",
                    s.side_ref, s.pokemon_index, s.previous_turns, s.new_turns
                )
            }
            Instruction::ChangeSubstituteHealth(s) => {
                write!(
                    f,
                    "SetSubstituteHealth {:?}: {:?}",
                    s.side_ref, s.health_change,
                )
            }
            Instruction::FormeChange(s) => {
                write!(f, "FormeChange {:?}: {:?}", s.side_ref, s.forme_change)
            }
            Instruction::SetSideOneMoveSecondSwitchOutMove(s) => {
                write!(
                    f,
                    "SideOneMoveSecondSwitchOutMove: {:?} -> {:?}",
                    s.previous_choice, s.new_choice
                )
            }
            Instruction::SetSideTwoMoveSecondSwitchOutMove(s) => {
                write!(
                    f,
                    "SideTwoMoveSecondSwitchOutMove: {:?} -> {:?}",
                    s.previous_choice, s.new_choice
                )
            }
            Instruction::ToggleBatonPassing(s) => {
                write!(f, "ToggleBatonPassing {:?}", s.side_ref)
            }
            Instruction::ToggleTerastallized(s) => {
                write!(f, "ToggleTerastallized {:?}", s.side_ref)
            }
            Instruction::SetLastUsedMove(s) => {
                write!(
                    f,
                    "SetLastUsedMove {:?}: {:?} -> {:?}",
                    s.side_ref, s.previous_last_used_move, s.last_used_move
                )
            }
            Instruction::SetDamageDealtSideOne(s) => {
                write!(
                    f,
                    "SetDamageDealt SideOne: ({:?} -> {:?}) Damage Change: {:?} HitSub Change: {:?}",
                    s.previous_move_category,
                    s.move_category,
                    s.damage_change,
                    s.toggle_hit_substitute
                )
            }
            Instruction::SetDamageDealtSideTwo(s) => {
                write!(
                    f,
                    "SetDamageDealt SideTwo: ({:?} -> {:?}) Damage Change: {:?} HitSub Change: {:?}",
                    s.previous_move_category,
                    s.move_category,
                    s.damage_change,
                    s.toggle_hit_substitute
                )
            }
            Instruction::DecrementPP(s) => {
                write!(
                    f,
                    "DecrementPP {:?}: {:?} {}",
                    s.side_ref, s.move_index, s.amount
                )
            }
            Instruction::ToggleTrickRoom(i) => {
                write!(
                    f,
                    "ToggleTrickRoom: {:?},{:?} -> {:?},{:?}",
                    i.currently_active,
                    i.previous_trickroom_turns_remaining,
                    !i.currently_active,
                    i.new_trickroom_turns_remaining,
                )
            }
            Instruction::DecrementTrickRoomTurnsRemaining => {
                write!(f, "DecrementTrickRoomTurnsRemaining")
            }
            Instruction::ToggleSideOneForceSwitch => {
                write!(f, "ToggleSideOneForceSwitch")
            }
            Instruction::ToggleSideTwoForceSwitch => {
                write!(f, "ToggleSideTwoForceSwitch")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetDamageDealtSideOneInstruction {
    pub damage_change: i16,
    pub move_category: MoveCategory,
    pub previous_move_category: MoveCategory,
    pub toggle_hit_substitute: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetDamageDealtSideTwoInstruction {
    pub damage_change: i16,
    pub move_category: MoveCategory,
    pub previous_move_category: MoveCategory,
    pub toggle_hit_substitute: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecrementPPInstruction {
    pub side_ref: SideReference,
    pub move_index: PokemonMoveIndex,
    pub amount: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetLastUsedMoveInstruction {
    pub side_ref: SideReference,
    pub last_used_move: LastUsedMove,
    pub previous_last_used_move: LastUsedMove,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ToggleBatonPassingInstruction {
    pub side_ref: SideReference,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecrementRestTurnsInstruction {
    pub side_ref: SideReference,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetSleepTurnsInstruction {
    pub side_ref: SideReference,
    pub pokemon_index: PokemonIndex,
    pub new_turns: i8,
    pub previous_turns: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetSecondMoveSwitchOutMoveInstruction {
    pub new_choice: Choices,
    pub previous_choice: Choices,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeWishInstruction {
    pub side_ref: SideReference,
    pub wish_amount_change: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecrementWishInstruction {
    pub side_ref: SideReference,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetFutureSightInstruction {
    pub side_ref: SideReference,
    pub pokemon_index: PokemonIndex,
    pub previous_pokemon_index: PokemonIndex,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecrementFutureSightInstruction {
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
pub struct ChangeSubsituteHealthInstruction {
    pub side_ref: SideReference,
    pub health_change: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FormeChangeInstruction {
    pub side_ref: SideReference,
    pub forme_change: FormeChange,
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
pub struct ToggleTrickRoomInstruction {
    pub currently_active: bool,
    pub new_trickroom_turns_remaining: i8,
    pub previous_trickroom_turns_remaining: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ToggleTerastallizedInstruction {
    pub side_ref: SideReference,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeType {
    pub side_ref: SideReference,
    pub new_types: (PokemonType, PokemonType),
    pub old_types: (PokemonType, PokemonType),
}

#[cfg(test)]
mod test {
    use super::Instruction;

    // Make sure that the size of the Instruction enum doesn't change
    #[test]
    fn test_instruction_size() {
        assert_eq!(size_of::<Instruction>(), 8);
        assert_eq!(align_of::<Instruction>(), 2);
    }
}
