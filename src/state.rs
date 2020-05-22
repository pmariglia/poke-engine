use std::collections::HashSet;


#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Burn,
    Sleep,
    Freeze,
    Paralyze,
    Poison,
    Toxic
}

pub enum PokemonTypes {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
    Typeless,
}

pub struct Pokemon {
    id: String,
    level: i8,
    types: HashSet<PokemonTypes>,
    hp: i16,
    maxhp: i16,
    ability: String,
    item: String,
    attack: i16,
    defense: i16,
    special_attack: i16,
    special_defense: i16,
    speed: i16,
    attack_boost: i8,
    defense_boost: i8,
    special_attack_boost: i8,
    special_defense_boost: i8,
    speed_boost: i8,
    accuracy_boost: i8,
    evasion_boost: i8,
    status: Status,
}
