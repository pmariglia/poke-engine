#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod moves;
mod pokedex;
mod state;

fn print_pokedex() {
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

fn print_moves() {
    let file_path: &str = "data/moves.json";
    let my_moves = moves::create_moves(file_path);
    for (key, value) in my_moves {
        // match value.secondary.status {
        //     Some(status) => println!("{} has secondary status {:?} that happens {}% of the time", key, status, value.secondary.chance),
        //     None => ()
        // }
        println!(
            "{}\nAccuracy: {}\nBasePower: {}\nCategory: {}\nStatus: {:?}\nPriority: {}\nTarget: {}\nType: {}\nPP: {}\nFlags: {:?}\nSecondary: {:?}\nMyself: {:?}\nBoosts: {:?}\nVolatileStatus: {:?}\nSideCondition: {:?}\nHeal: {:?}\nCrash: {:?}\nDrain: {:?}\nRecoil: {:?}\n",
            key,
            value.accuracy,
            value.base_power,
            value.category,
            value.status,
            value.priority,
            value.target,
            value.move_type,
            value.pp,
            value.flags,
            value.secondary,
            value.myself,
            value.boosts,
            value.volatile_status,
            value.side_condition,
            value.heal,
            value.crash,
            value.drain,
            value.recoil
        );
    }
}

fn main() {
    print_pokedex();
    print_moves();
}
