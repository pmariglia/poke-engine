use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
pub struct Abilities {
    pub first: String,
    pub second: String,
    pub hidden: String,
}

#[derive(Serialize, Deserialize)]
pub struct BaseStats {
    pub hp: i16,
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
}

#[derive(Serialize, Deserialize)]
pub struct JsonPokemon {
    pub species: String,
    pub weight: f32,
    pub types: (String, String),
    pub base_stats: BaseStats,
    pub abilities: Abilities,
}

pub fn create_pokedex(file_path: &str) -> HashMap<String, JsonPokemon> {
    let reader = BufReader::new(File::open(file_path).unwrap());
    let loaded_json: JsonValue = serde_json::from_reader(reader).unwrap();

    let mut pokedex: HashMap<String, JsonPokemon> = HashMap::new();
    for (name, info) in loaded_json.as_object().unwrap() {
        pokedex.insert(
            name.to_string(),
            JsonPokemon {
                species: name.to_string(),
                base_stats: BaseStats {
                    hp: info["baseStats"]["hp"].as_i64().unwrap() as i16,
                    attack: info["baseStats"]["attack"].as_i64().unwrap() as i16,
                    defense: info["baseStats"]["defense"].as_i64().unwrap() as i16,
                    special_attack: info["baseStats"]["special-attack"].as_i64().unwrap() as i16,
                    special_defense: info["baseStats"]["special-attack"].as_i64().unwrap() as i16,
                    speed: info["baseStats"]["speed"].as_i64().unwrap() as i16,
                },
                abilities: Abilities {
                    first: info["abilities"]["0"].as_str().unwrap().to_owned(),
                    second: info["abilities"]["1"]
                        .as_str()
                        .unwrap_or_else(|| "none")
                        .to_owned(),
                    hidden: info["abilities"]["H"]
                        .as_str()
                        .unwrap_or_else(|| "none")
                        .to_owned(),
                },
                types: (
                    info["types"][0].as_str().unwrap().to_owned(),
                    info["types"][1]
                        .as_str()
                        .unwrap_or_else(|| "typeless")
                        .to_owned(),
                ),
                weight: info["weight"]
                    .as_f64()
                    .unwrap_or_else(|| panic!("Could not unwrap weight for {}", name))
                    as f32,
            },
        );
    }
    return pokedex;
}
