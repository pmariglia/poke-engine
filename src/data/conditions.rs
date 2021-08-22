use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    None,
    Burn,
    Sleep,
    Freeze,
    Paralyze,
    Poison,
    Toxic,
}
