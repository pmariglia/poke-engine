use super::state::SideReference;

pub struct SwitchInstruction {
    pub is_side_one: bool,
    pub previous_index: usize,
    pub next_index: usize,
}
