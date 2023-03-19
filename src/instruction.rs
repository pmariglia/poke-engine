use serde::{Deserialize, Serialize};

use super::data::conditions::PokemonStatus;
use super::data::conditions::PokemonVolatileStatus;

// https://stackoverflow.com/questions/50686411/whats-the-usual-way-to-create-a-vector-of-different-structs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Instruction {
    Switch(SwitchInstruction),
    RemoveVolatileStatus(RemoveVolatileStatusInstruction),
    ChangeStatus(ChangeStatusInstruction),
    Heal(HealInstruction),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HealInstruction {
    pub is_side_one: bool,
    pub heal_amount: i16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SwitchInstruction {
    pub is_side_one: bool,
    pub previous_index: usize,
    pub next_index: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChangeStatusInstruction {
    pub is_side_one: bool,
    pub pokemon_index: usize,
    pub old_status: PokemonStatus,
    pub new_status: PokemonStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RemoveVolatileStatusInstruction {
    pub is_side_one: bool,
    pub volatile_status: PokemonVolatileStatus,
}
