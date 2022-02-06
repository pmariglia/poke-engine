use serde::{Deserialize, Serialize};

// https://stackoverflow.com/questions/50686411/whats-the-usual-way-to-create-a-vector-of-different-structs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Instruction {
    SwitchInstruction(SwitchInstruction),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SwitchInstruction {
    pub is_side_one: bool,
    pub previous_index: usize,
    pub next_index: usize,
}
