extern crate lazy_static;

mod moves;
mod pokedex;
// mod state;

fn print_pkmn(pkmn: &pokedex::PokedexPokemon) {
    println!(
        "{}\n    Types: {}, {}\n    Weight: {}\n    Abilities: {}, {}, {}\n    BaseStats\n    \tHP: {}\n    \tAtk: {}\n    \tDef: {}\n    \tSpa: {}\n    \tSpd: {}\n    \tSpe: {}\n    ",
        pkmn.species,
        pkmn.types.0,
        pkmn.types.1,
        pkmn.weight,
        pkmn.abilities.first,
        pkmn.abilities.second,
        pkmn.abilities.hidden,
        pkmn.base_stats.hp,
        pkmn.base_stats.attack,
        pkmn.base_stats.defense,
        pkmn.base_stats.special_attack,
        pkmn.base_stats.special_defense,
        pkmn.base_stats.speed,
    )
}

fn print_move(mv: &moves::Move) {
    println!(
        "{:?}",
        mv
    )
}

fn main() {
    let pkmn: String = "calyrexshadow".to_string();
    let json_pkmn: &pokedex::PokedexPokemon = pokedex::get_pkmn(pkmn);
    print_pkmn(json_pkmn);
    
    let m: String = "meteormash".to_string();
    let move_obj: &moves::Move = moves::get_move(m);
    print_move(move_obj);

}
