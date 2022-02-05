use serde::{Serialize, Deserialize};

// https://stackoverflow.com/questions/50686411/whats-the-usual-way-to-create-a-vector-of-different-structs
#[derive(Serialize, Deserialize)]
pub enum Instruction {
    SwitchInstruction(SwitchInstruction)
}

#[derive(Serialize, Deserialize)]
pub struct SwitchInstruction {
    pub is_side_one: bool,
    pub previous_index: usize,
    pub next_index: usize,
}
