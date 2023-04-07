use serde::{Deserialize, Serialize};

use crate::state::SideReference;

use super::data::conditions::PokemonStatus;
use super::data::conditions::PokemonVolatileStatus;

// https://stackoverflow.com/questions/50686411/whats-the-usual-way-to-create-a-vector-of-different-structs
#[derive(Debug)]
pub enum Instruction {
    Switch(SwitchInstruction),
    RemoveVolatileStatus(RemoveVolatileStatusInstruction),
    ChangeStatus(ChangeStatusInstruction),
    Heal(HealInstruction),
    Damage(DamageInstruction),
}

#[derive(Debug)]
pub struct HealInstruction {
    pub side_ref: SideReference,
    pub heal_amount: i16,
}

#[derive(Debug)]
pub struct DamageInstruction {
    pub side_ref: SideReference,
    pub damage_amount: i16,
}

#[derive(Debug)]
pub struct SwitchInstruction {
    pub side_ref: SideReference,
    pub previous_index: usize,
    pub next_index: usize,
}

#[derive(Debug)]
pub struct ChangeStatusInstruction {
    pub side_ref: SideReference,
    pub pokemon_index: usize,
    pub old_status: PokemonStatus,
    pub new_status: PokemonStatus,
}

#[derive(Debug)]
pub struct RemoveVolatileStatusInstruction {
    pub side_ref: SideReference,
    pub volatile_status: PokemonVolatileStatus,
}
