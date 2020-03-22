#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod pokedex;

fn main() {
    let file_path: &str = "data/pokedex.json";
    let my_pokedex = pokedex::create_pokedex(file_path);
    for (key, value) in my_pokedex {
        println!(
            "{}\nTypes: {}, {}\nWeight: {}\nAbilities: {}, {}, {}\nBaseStats\n\tHP: {}\n\tAtk: {}\n\tDef: {}\n\tSpa: {}\n\tSpd: {}\n\tSpe: {}\n",
            key,
            value.types.0,
            value.types.1,
            value.weight,
            value.abilities.first,
            value.abilities.second,
            value.abilities.hidden,
            value.base_stats.hp,
            value.base_stats.attack,
            value.base_stats.defense,
            value.base_stats.special_attack,
            value.base_stats.special_defense,
            value.base_stats.speed,
        )
    }
}
