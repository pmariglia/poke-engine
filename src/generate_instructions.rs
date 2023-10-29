use crate::{
    data::moves::{Choice, MoveCategory},
    instruction::{Instruction, StateInstruction, SwitchInstruction},
    state::{SideReference, State},
};

fn generate_instructions_from_switch(
    state: &mut State,
    new_pokemon_index: usize,
    switching_side: SideReference,
    incoming_instructions: &mut StateInstruction,
) {
    state.apply_instructions(&incoming_instructions.instruction_list);

    println!(
        "Before switch side 1 active name: {:?}",
        state.side_one.get_active().id
    );

    let switch_instruction = Instruction::Switch(SwitchInstruction {
        side_ref: switching_side,
        previous_index: state.get_side(&switching_side).active_index,
        next_index: new_pokemon_index,
    });
    state.apply_one_instruction(&switch_instruction);
    incoming_instructions
        .instruction_list
        .push(switch_instruction);

    println!(
        "After switch side 1 active name: {:?}",
        state.side_one.get_active().id
    );

    state.reverse_instructions(&incoming_instructions.instruction_list);
}

//fn generate_instructions_from_damage(
//    state: &mut State,
//    choice: Choice,
//    incoming_instructions: &mut StateInstruction
//) {}

// Interpreting the function arguments/return-value:
//
// This function takes in a mutable StateInstruction,
// and returns a Vector of StateInstructions, which
// represent all the possible branches that can be taken
// given that move being used
pub fn generate_instructions_from_move<'a>(
    state: &mut State,
    choice: Choice,
    attacking_side: SideReference,
    incoming_instructions: &'a mut StateInstruction,
) -> Vec<&'a mut StateInstruction> {
    if choice.category == MoveCategory::Switch {
        generate_instructions_from_switch(
            state,
            choice.switch_id,
            attacking_side,
            incoming_instructions,
        );
        return vec![incoming_instructions];
    }

    panic!("Not implemented yet");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::data::conditions::{PokemonStatus, PokemonVolatileStatus};
    use crate::instruction::{DamageInstruction, SwitchInstruction};
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

    fn get_dummy_instruction() -> StateInstruction {
        return StateInstruction {
            percentage: 100.0,
            instruction_list: vec![],
        };
    }

    #[test]
    fn test_basic_switch_functionality_with_no_prior_instructions() {
        let mut state: State = get_dummy_state();
        let mut incoming_instructions = get_dummy_instruction();
        let mut choice = Choice {
            ..Default::default()
        };

        choice.switch_id = 1;

        let expected_instructions: StateInstruction = StateInstruction {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
        };

        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_functionality_with_a_prior_instruction() {
        let mut state: State = get_dummy_state();
        let mut incoming_instructions = get_dummy_instruction();
        let mut choice = Choice {
            ..Default::default()
        };

        choice.switch_id = 1;
        incoming_instructions
            .instruction_list
            .push(Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }));

        let expected_instructions: StateInstruction = StateInstruction {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: 0,
                    next_index: 1,
                }),
            ],
        };

        generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }
}
