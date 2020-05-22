use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use serde_json::Value as JsonValue;

use crate::state::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct Boosts {
    attack: i8,
    defense: i8,
    special_attack: i8,
    special_defense: i8,
    speed: i8,
    accuracy: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Myself {
    pub volatile_status: Option<String>,
    pub boosts: Boosts,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flags {
    pub authentic: bool,
    pub bite: bool,
    pub bullet: bool,
    pub charge: bool,
    pub contact: bool,
    pub dance: bool,
    pub defrost: bool,
    pub distance: bool,
    pub gravity: bool,
    pub heal: bool,
    pub mirror: bool,
    pub mystery: bool,
    pub nonsky: bool,
    pub powder: bool,
    pub protect: bool,
    pub pulse: bool,
    pub punch: bool,
    pub recharge: bool,
    pub reflectable: bool,
    pub snatch: bool,
    pub sound: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Secondary {
    pub chance: i8,
    pub volatile_status: Option<String>,
    pub boosts: Boosts,
    pub myself: Myself,
    pub status: Option<Status>,
}

#[derive(Serialize, Deserialize)]
pub struct Move {
    pub accuracy: f32,
    pub base_power: f32,
    pub boosts: Boosts,
    pub category: String,
    pub flags: Flags,
    pub name: String,
    pub status: Option<Status>,
    pub priority: i8,
    pub secondary: Secondary,
    pub myself: Myself,
    pub target: String,
    pub move_type: String,
    pub pp: i8,
    pub volatile_status: Option<String>,
    pub side_condition: Option<String>,
    pub heal: Option<f32>,
    pub crash: Option<f32>,
    pub drain: Option<f32>,
    pub recoil: Option<f32>,
}

fn create_flags(info: &JsonValue) -> Flags {
    let new_flags = Flags {
        authentic: info["flags"]["authentic"].as_i64().unwrap_or_else(|| 0) != 0,
        bite: info["flags"]["bite"].as_i64().unwrap_or_else(|| 0) != 0,
        bullet: info["flags"]["bullet"].as_i64().unwrap_or_else(|| 0) != 0,
        charge: info["flags"]["charge"].as_i64().unwrap_or_else(|| 0) != 0,
        contact: info["flags"]["contact"].as_i64().unwrap_or_else(|| 0) != 0,
        dance: info["flags"]["dance"].as_i64().unwrap_or_else(|| 0) != 0,
        defrost: info["flags"]["defrost"].as_i64().unwrap_or_else(|| 0) != 0,
        distance: info["flags"]["distance"].as_i64().unwrap_or_else(|| 0) != 0,
        gravity: info["flags"]["gravity"].as_i64().unwrap_or_else(|| 0) != 0,
        heal: info["flags"]["heal"].as_i64().unwrap_or_else(|| 0) != 0,
        mirror: info["flags"]["mirror"].as_i64().unwrap_or_else(|| 0) != 0,
        mystery: info["flags"]["mystery"].as_i64().unwrap_or_else(|| 0) != 0,
        nonsky: info["flags"]["nonsky"].as_i64().unwrap_or_else(|| 0) != 0,
        powder: info["flags"]["powder"].as_i64().unwrap_or_else(|| 0) != 0,
        protect: info["flags"]["protect"].as_i64().unwrap_or_else(|| 0) != 0,
        pulse: info["flags"]["pulse"].as_i64().unwrap_or_else(|| 0) != 0,
        punch: info["flags"]["punch"].as_i64().unwrap_or_else(|| 0) != 0,
        recharge: info["flags"]["recharge"].as_i64().unwrap_or_else(|| 0) != 0,
        reflectable: info["flags"]["reflectable"].as_i64().unwrap_or_else(|| 0) != 0,
        snatch: info["flags"]["snatch"].as_i64().unwrap_or_else(|| 0) != 0,
        sound: info["flags"]["sound"].as_i64().unwrap_or_else(|| 0) != 0,
    };
    return new_flags;
}

fn get_volatile_status(object: JsonValue) -> Option<String> {
    let volatile_status_string = object["volatileStatus"].as_str().unwrap_or_else(|| "");
    let volatile_status = match volatile_status_string {
        "" => None,
        _ => Some(volatile_status_string.to_string()),
    };
    return volatile_status;
}

fn get_side_conditions(object: JsonValue) -> Option<String> {
    let side_condition_string = object["side_conditions"].as_str().unwrap_or_else(|| "");
    let volatile_status = match side_condition_string {
        "" => None,
        _ => Some(side_condition_string.to_string()),
    };
    return volatile_status;
}

fn get_boosts(info: JsonValue) -> Boosts {
    let boosts = Boosts {
        attack: info["boosts"]["attack"].as_i64().unwrap_or_else(|| 0) as i8,
        defense: info["boosts"]["defense"].as_i64().unwrap_or_else(|| 0) as i8,
        special_attack: info["boosts"]["special-attack"]
            .as_i64()
            .unwrap_or_else(|| 0) as i8,
        special_defense: info["boosts"]["special-defense"]
            .as_i64()
            .unwrap_or_else(|| 0) as i8,
        speed: info["boosts"]["speed"].as_i64().unwrap_or_else(|| 0) as i8,
        accuracy: info["boosts"]["accuracy"].as_i64().unwrap_or_else(|| 0) as i8,
    };
    return boosts;
}

fn get_status(object: JsonValue) -> Option<Status> {
    let status_string = object["status"].as_str().unwrap_or_else(|| "");
    let status = match status_string {
        "brn" => Some(Status::Burn),
        "frz" => Some(Status::Freeze),
        "par" => Some(Status::Paralyze),
        "slp" => Some(Status::Sleep),
        "psn" => Some(Status::Poison),
        "tox" => Some(Status::Toxic),
        "" => None,
        _ => panic!("Invalid status found: {}", status_string),
    };
    return status;
}

fn create_secondary(info: &JsonValue) -> Secondary {
    let new_secondary = Secondary {
        chance: info["secondary"]["chance"].as_i64().unwrap_or_else(|| 0) as i8,
        volatile_status: get_volatile_status(info["secondary"].clone()),
        boosts: get_boosts(info["secondary"].clone()),
        myself: Myself {
            boosts: get_boosts(info["secondary"]["self"].clone()),
            volatile_status: get_volatile_status(info["secondary"]["self"].clone()),
        },
        status: get_status(info["secondary"].clone()),
    };

    return new_secondary;
}

fn create_self(info: &JsonValue) -> Myself {
    let new_self = Myself {
        volatile_status: get_volatile_status(info["self"].clone()),
        boosts: get_boosts(info["self"].clone()),
    };
    return new_self;
}

pub fn get_f32_option_from_f64(info: JsonValue) -> Option<f32> {
    let value: Option<f64> = info.as_f64();
    if value.is_some() {
        return Some(value.unwrap() as f32);
    }
    return None;
}

pub fn create_moves(file_path: &str) -> HashMap<String, Move> {
    let reader = BufReader::new(File::open(file_path).unwrap());
    let loaded_json: JsonValue = serde_json::from_reader(reader).unwrap();

    let mut moves: HashMap<String, Move> = HashMap::new();
    for (name, info) in loaded_json.as_object().unwrap() {
        // if accuracy cannot be extracted as a float, set to a negative number to signify the move does not check accuracy
        let accuracy: f32 = info["accuracy"].as_f64().unwrap_or_else(|| -1.0) as f32;

        moves.insert(
            name.to_string(),
            Move {
                name: name.to_string(),
                accuracy: accuracy,
                base_power: info["basePower"]
                    .as_f64()
                    .unwrap_or_else(|| panic!("Could not unwrap basePower for {}", name))
                    as f32,
                category: info["category"]
                    .as_str()
                    .unwrap_or_else(|| panic!("Could not unwrap category for {}", name))
                    .to_owned(),
                status: get_status(info.to_owned()),
                target: info["target"]
                    .as_str()
                    .unwrap_or_else(|| panic!("Could not unwrap target for {}", name))
                    .to_owned(),
                move_type: info["target"]
                    .as_str()
                    .unwrap_or_else(|| panic!("Could not unwrap move_type for {}", name))
                    .to_owned(),
                pp: info["pp"]
                    .as_i64()
                    .unwrap_or_else(|| panic!("Could not unwrap pp for {}", name))
                    .to_owned() as i8,
                flags: create_flags(info),
                priority: info["priority"]
                    .as_i64()
                    .unwrap_or_else(|| panic!("Coult not unwrap priority for {}", name))
                    .to_owned() as i8,
                myself: create_self(info),
                secondary: create_secondary(info),
                boosts: get_boosts(info.to_owned()),
                volatile_status: get_volatile_status(info.to_owned()),
                side_condition: get_side_conditions(info.to_owned()),
                heal: get_f32_option_from_f64(info["heal"].to_owned()),
                crash: get_f32_option_from_f64(info["crash"].to_owned()),
                drain: get_f32_option_from_f64(info["drain"].to_owned()),
                recoil: get_f32_option_from_f64(info["recoil"].to_owned()),
            },
        );
    }

    return moves;
}
// heal_target
