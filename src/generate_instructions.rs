use crate::{
    data::moves::{Choice, MoveCategory},
    instruction::{Instruction, SwitchInstruction},
    state::{SideReference, State},
};

fn generate_instructions_from_switch(
    state: &mut State,
    new_pokemon_index: usize,
    switching_side: SideReference,
    incoming_instructions: &mut Vec<Instruction>,
) {
    state.apply_instructions(&incoming_instructions);

    println!("{:?}", state.side_one.get_active());

    // How do I get side from sidereference?
    let switch_instruction = Instruction::Switch(SwitchInstruction {
        side_ref: switching_side,
        previous_index: state.get_side(&switching_side).active_index,
        next_index: new_pokemon_index,
    });
    state.apply_one_instruction(&switch_instruction);
    incoming_instructions.push(switch_instruction);

    state.reverse_instructions(&incoming_instructions);

    //return incoming_instructions;
}

// Vec<Instruction> is wrong.
// There needs to be a struct that wraps the
// Instruction and includes the percentage (likelihood)
// of it happening. One of the elements of that struct would
// be Vec<Instruction>
//
// Then.. This function needs to take in ThatNewStruct 
// and return Vec<ThatNewStruct>
//
// generate_instructions_from_switch needs to change to
pub fn generate_instructions_from_move(
    state: &mut State,
    choice: Choice,
    attacking_side: SideReference,
    incoming_instructions: &mut Vec<Instruction>,
) {
    if choice.category == MoveCategory::Switch {
        let t = generate_instructions_from_switch(
            state,
            choice.switch_id,
            attacking_side,
            incoming_instructions,
        );
        generate_instructions_from_switch(
            state,
            choice.switch_id,
            attacking_side,
            incoming_instructions,
        );
        return; //incoming_instructions;
    }

    panic!("Not implemented yet");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::data::conditions::{PokemonStatus, PokemonVolatileStatus};
    use crate::instruction::SwitchInstruction;
    use crate::state::{
        Pokemon, PokemonNatures, PokemonTypes, Side, SideConditions, SideReference, State,
        StateTerrain, StateWeather, Terrain, Weather,
    };

    fn get_dummy_state() -> State {
        return State {
            side_one: Side {
                active_index: 0,
                pokemon: [
                    Pokemon {
                        id: "squirtle".to_string(),
                        level: 100,
                        types: (PokemonTypes::Water, PokemonTypes::Typeless),
                        hp: 100,
                        maxhp: 100,
                        ability: "torrent".to_string(),
                        item: "none".to_string(),
                        attack: 100,
                        defense: 100,
                        special_attack: 100,
                        special_defense: 100,
                        speed: 100,
                        attack_boost: 0,
                        defense_boost: 0,
                        special_attack_boost: 0,
                        special_defense_boost: 0,
                        speed_boost: 0,
                        accuracy_boost: 0,
                        evasion_boost: 0,
                        status: PokemonStatus::None,
                        nature: PokemonNatures::Serious,
                        volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
                        moves: vec![],
                    },
                    Pokemon {
                        ..Pokemon::default()
                    },
                ],
                side_conditions: SideConditions {
                    ..Default::default()
                },
                wish: (0, 0),
            },
            side_two: Side {
                active_index: 0,
                pokemon: [
                    Pokemon {
                        id: "charmander".to_string(),
                        level: 100,
                        types: (PokemonTypes::Fire, PokemonTypes::Typeless),
                        hp: 100,
                        maxhp: 100,
                        ability: "blaze".to_string(),
                        item: "none".to_string(),
                        attack: 100,
                        defense: 100,
                        special_attack: 100,
                        special_defense: 100,
                        speed: 100,
                        attack_boost: 0,
                        defense_boost: 0,
                        special_attack_boost: 0,
                        special_defense_boost: 0,
                        speed_boost: 0,
                        accuracy_boost: 0,
                        evasion_boost: 0,
                        status: PokemonStatus::None,
                        nature: PokemonNatures::Serious,
                        volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
                        moves: vec![],
                    },
                    Pokemon {
                        ..Pokemon::default()
                    },
                ],
                side_conditions: SideConditions {
                    ..Default::default()
                },
                wish: (0, 0),
            },
            weather: StateWeather {
                weather_type: Weather::None,
                turns_remaining: 0,
            },
            terrain: StateTerrain {
                terrain_type: Terrain::None,
                turns_remaining: 0,
            },
            trick_room: false,
        };
    }

    #[test]
    fn test_basic_switch_functionality() {
        let mut state: State = get_dummy_state();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = 1;
        let mut incoming_instructions: Vec<Instruction> = vec![];

        let expected_instructions = vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: 0,
            next_index: 1,
        })];

        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }
}
