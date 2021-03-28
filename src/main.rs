extern crate lazy_static;

mod moves;
mod pokedex;
mod state;

fn print_pkmn(pkmn: &pokedex::PokedexPokemon) {
    println!(
        "{}\n    Types: {:?}, {:?}\n    Weight: {}\n    Abilities: {}, {}, {}\n    BaseStats\n    \tHP: {}\n    \tAtk: {}\n    \tDef: {}\n    \tSpa: {}\n    \tSpd: {}\n    \tSpe: {}\n    ",
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
    let pkmn: &str = "pikachu";
    let json_pkmn: &pokedex::PokedexPokemon = pokedex::get_pkmn(pkmn);
    print_pkmn(json_pkmn);
    
    let m: String = "meteormash".to_string();
    let move_obj: &moves::Move = moves::get_move(m);
    print_move(move_obj);

    let pikachu: state::Pokemon = state::create_basic_pokemon("pikachu".to_string(), 100);
    let charizard: state::Pokemon = state::create_basic_pokemon("charizard".to_string(), 100);
    let blastoise: state::Pokemon = state::create_basic_pokemon("blastoise".to_string(), 100);
    let espeon: state::Pokemon = state::create_basic_pokemon("espeon".to_string(), 100);
    let snorlax: state::Pokemon = state::create_basic_pokemon("snorlax".to_string(), 100);
    let venusaur: state::Pokemon = state::create_basic_pokemon("venusaur".to_string(), 100);

    let landorustherian: state::Pokemon = state::create_basic_pokemon("landorustherian".to_string(), 100);
    let tapulele: state::Pokemon = state::create_basic_pokemon("tapulele".to_string(), 100);
    let rillaboom: state::Pokemon = state::create_basic_pokemon("rillaboom".to_string(), 100);
    let rhyperior: state::Pokemon = state::create_basic_pokemon("rhyperior".to_string(), 100);
    let gengar: state::Pokemon = state::create_basic_pokemon("gengar".to_string(), 100);
    let melmetal: state::Pokemon = state::create_basic_pokemon("melmetal".to_string(), 100);
    
    let my_side: state::Side = state::Side {
        active: pikachu,
        reserve: vec![charizard, blastoise, espeon, snorlax, venusaur],
        side_conditions: Vec::<String>::new(),
        wish: (0, 0)
    };

    let your_side: state::Side = state::Side {
        active: landorustherian,
        reserve: vec![tapulele, rillaboom, rhyperior, gengar, melmetal],
        side_conditions: Vec::<String>::new(),
        wish: (0, 0)
    };

    let state: state::State = state::State {
        side_one: my_side,
        side_two: your_side,
        weather: state::Weather::None,
        terrain: state::Terrain::None,
        trick_room: false
    };

    println!("{:?}", state)

}
