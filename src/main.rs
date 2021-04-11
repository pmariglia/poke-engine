use std::collections::HashMap;
extern crate lazy_static;

mod moves;
mod pokedex;
mod state;
mod abilities;
mod items;
mod find_instructions;

fn main() {

    let mut pikachu: state::Pokemon = state::create_basic_pokemon("pikachu".to_string(), 100);
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

    pikachu.moves.push("volttackle".to_string());
    pikachu.moves.push("voltswitch".to_string());
    pikachu.moves.push("irontail".to_string());
    pikachu.moves.push("surf".to_string());
    

    let my_side: state::Side = state::Side {
        active: pikachu,
        reserve: vec![charizard, blastoise, espeon, snorlax, venusaur],
        side_conditions: HashMap::<String, i8>::new(),
        wish: (0, 0)
    };

    let your_side: state::Side = state::Side {
        active: landorustherian,
        reserve: vec![tapulele, rillaboom, rhyperior, gengar, melmetal],
        side_conditions: HashMap::<String, i8>::new(),
        wish: (0, 0)
    };

    let mut state: state::State = state::State {
        side_one: my_side,
        side_two: your_side,
        weather: state::Weather::None,
        terrain: state::Terrain::None,
        trick_room: false
    };

    state.side_one.active.speed = 5;
    state.side_two.active.speed = 10;
    // state.trick_room = true;
    // state.side_two.active.ability = "prankster".to_string();

    let s1mf = find_instructions::side_one_moves_first(&state, "tackle", "thunderwave");

    println!("Side one moves first: {}", s1mf);

}
