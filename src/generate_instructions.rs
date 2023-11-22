use crate::{
    data::moves::{Choice, MoveCategory},
    instruction::{DamageInstruction, Instruction, StateInstruction, SwitchInstruction},
    state::{SideReference, State}, damage_calc::{calculate_damage, DamageRolls},
};
use std::cmp;

fn generate_instructions_from_switch(
    state: &mut State,
    new_pokemon_index: usize,
    switching_side: SideReference,
    incoming_instructions: StateInstruction,
) -> Vec<StateInstruction> {
    let mut incoming_instructions = incoming_instructions;
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

    return vec![incoming_instructions];
}

// TODO: This isn't ready to integrate yet, and it might not be complete
// Come back to this
fn generate_instructions_from_damage(
    state: &mut State,
    choice: Choice,
    calculated_damage: i16,
    attacking_side_ref: &SideReference,
    incoming_instructions: StateInstruction,
) -> Vec<StateInstruction> {
    let mut return_instructions: Vec<StateInstruction> = vec![];

    state.apply_instructions(&incoming_instructions.instruction_list);

    let (attacking_side, defending_side) = state.get_both_sides(attacking_side_ref);
    let attacking_pokemon = attacking_side.get_active_immutable();
    let defending_pokemon = defending_side.get_active_immutable();

    let percent_hit = choice.accuracy / 100.0;
    // Move hits some of the time
    if percent_hit > 0.0 {
        let mut move_hit_instructions = incoming_instructions.clone();

        let damage_dealt = cmp::min(calculated_damage, defending_pokemon.hp);
        move_hit_instructions
            .instruction_list
            .push(Instruction::Damage(DamageInstruction {
                side_ref: attacking_side_ref.get_other_side(),
                damage_amount: damage_dealt,
            }));

        move_hit_instructions.update_percengate(percent_hit);

        return_instructions.push(move_hit_instructions);
    }

    // Move misses some of the time
    if percent_hit < 1.0 {
        let mut move_missed_instruction = incoming_instructions.clone();

        move_missed_instruction.update_percengate(1.0 - percent_hit);

        return_instructions.push(move_missed_instruction);
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);

    return return_instructions;
}

// Interpreting the function arguments/return-value:
//
// This function takes in a mutable StateInstruction,
// and returns a Vector of StateInstructions, which
// represent all the possible branches that can be taken
// given that move being used
pub fn generate_instructions_from_move(
    state: &mut State,
    choice: Choice,
    attacking_side: SideReference,
    incoming_instructions: StateInstruction,
) -> Vec<StateInstruction> {
    /*
    Note: end-of-turn instructions are not included here - this is only the instructions from a move

    Order of Operations:
    (*) indicates it can branch, (-) indicates it does not

    - check for if the user is switching - do so & exit early
    - check for short-curcuit situations that would exit before doing anything
        - using drag move but you moved 2nd (possible if say both users use dragontail)
        - attacking pokemon is dead (possible if you got KO-ed this turn)
        - attacking pokemon is taunted & chose a non-damaging move
        - attacker was flinched
            not a branching event because the previous turn would've decided whether a flinch happened or not
        - protect (or it's variants) nullifying a move - this may generate a custom instruction 
        - move has no effect
            i.e. electric-type status move used against a ground type, powder move used against grass / overcoat
            normally, the move doing 0 damage would trigger this, but for non-damaging moves there needs to be another
            spot where this is checked. This may be better done elsewhere
    - update choice struct based on special effects
        - move special effect
        - ability special effect (both sides)
        - item special effect (both sides)

    * attacker is fully-paralyzed, asleep, frozen (the first thing that can branch from the old engine)
    - move special effects
        hail, trick, futuresight, trickroom, etc. Anything that cannot be succinctly expressed in a Choice
    * calculate damage amount(s) and do the damage
    - after-move effects
        * move special effect (both sides)
            - static/flamebody means this needs to possibly branch
        - ability (both sides)
        - item (both sides)
    - side_conditions: spikes, wish, veil. Anything using the `side_condition` section of the Choice
        potentially could be an `after_move`
    - hazard clearing: defog, rapidspin, courtchange, etc.
        potentially could be an `after_move`
    * volatile_statuses: Anything using the `volatile_status` section of the Choice
    - status effects: Anything using the `status` section of the Choice
    - boosts: Anything using the `boosts` section of the Choice
    - boost reset (clearsmog & haze)
        potentially could be an `after_move` for clearsmog, and a move special effect for haze
    - heal Anything using the `heal` section of the Choice
    * flinching move
        potentially could be collapsed into secondaries?
    - drag moves
        potentially could be a move special effect, or even a short-circuit since nothing else could happen?
    - secondaries, one of the following:
        PokemonVolatileStatus
        PokemonSideCondition
        StatBoosts
        Heal
        PokemonStatus

        These secondaries have their own separate chance & target,
        whereas their equivalents above are assumed to be 100% if the
        move hit
        They only are attempted if the move did not miss , so some
        flag will be needed to signify that the move hit/missed

    - switch-out move
        Will have to come back to this since it breaks a bunch of patterns and stops the turn mid-way through

    */

    if choice.category == MoveCategory::Switch {
        return generate_instructions_from_switch(
            state,
            choice.switch_id,
            attacking_side,
            incoming_instructions,
        );
    }


    // This was just here to make sure it works - unsure where it will end up
    let damages_dealt = calculate_damage(state, attacking_side, &choice, DamageRolls::Average);

    println!("{:?}", damages_dealt);
    println!("{:?}", choice);

    panic!("Not implemented yet");
}

//fn update_move

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

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(vec![expected_instructions], incoming_instructions);
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

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            incoming_instructions,
        );

        assert_eq!(vec![expected_instructions], incoming_instructions);
    }

    macro_rules! damage_instructions_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (
                        move_id,
                        move_category,
                        move_base_power,
                        move_accuracy,
                        incoming_instructions,
                        expected_instructions
                    ) = $value;

                    let mut state: State = get_dummy_state();
                    let mut choice = Choice {
                        ..Default::default()
                    };

                    choice.move_id = move_id;
                    choice.category = move_category;
                    choice.base_power = move_base_power;
                    choice.accuracy = move_accuracy;

                    let new_instructions = generate_instructions_from_damage(
                        &mut state,
                        choice,
                        35,
                        &SideReference::SideOne,
                        incoming_instructions,
                        );

                    assert_eq!(expected_instructions, new_instructions);
                }
             )*
        }
    }

    damage_instructions_tests! {
        test_basic_move_with_100_accuracy: (
            "tackle".to_string(),
            MoveCategory::Physical,
            40.0,
            100.0,
            get_dummy_instruction(),
            vec![StateInstruction {
                percentage: 100.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 35,
                })],
            }]
        ),
        test_basic_move_with_90_accuracy: (
            "tackle".to_string(),
            MoveCategory::Physical,
            40.0,
            90.0,
            get_dummy_instruction(),
            vec![
                StateInstruction {
                    percentage: 90.0,
                    instruction_list: vec![Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 35,
                })],
            },
                StateInstruction {
                    percentage: 10.000002,
                    instruction_list: vec![],
                },
            ]
        ),
    }
}
