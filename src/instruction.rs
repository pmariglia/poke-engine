use serde::{Deserialize, Serialize};

use super::data::moves::VolatileStatus;

// https://stackoverflow.com/questions/50686411/whats-the-usual-way-to-create-a-vector-of-different-structs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Instruction {
    Switch(SwitchInstruction),
    RemoveVolatileStatus(RemoveVolatileStatusInstruction),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SwitchInstruction {
    pub is_side_one: bool,
    pub previous_index: usize,
    pub next_index: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RemoveVolatileStatusInstruction {
    pub is_side_one: bool,
    pub volatile_status: VolatileStatus,
}
