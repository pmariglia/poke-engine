use crate::{
    damage_calc::{calculate_damage, DamageRolls},
    data::{
        conditions::PokemonVolatileStatus,
        moves::{Choice, MoveCategory},
    },
    instruction::{DamageInstruction, Instruction, StateInstruction, SwitchInstruction},
    state::{Pokemon, SideReference, State},
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

fn cannot_use_move(state: &State, choice: &Choice, attacking_side_ref: &SideReference) -> bool {
    let attacking_pkmn: &Pokemon = state
        .get_side_immutable(attacking_side_ref)
        .get_active_immutable();

    // If you were taunted, you can't use a Physical/Special move
    if attacking_pkmn
        .volatile_statuses
        .contains(&PokemonVolatileStatus::Taunt)
        && matches!(
            choice.category,
            MoveCategory::Physical | MoveCategory::Special
        )
    {
        return true;
    } else if attacking_pkmn.hp == 0 {
        return true;
    } else if attacking_pkmn
        .volatile_statuses
        .contains(&PokemonVolatileStatus::Flinch)
    {
        return true;
    }

    return false;
}

// Updates the attacker's Choice based on some special effects
fn update_choice(
    state: &State,
    choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side: &SideReference,
) {
    match choice.modify_move {
        Some(modify_move_fn) => {
            modify_move_fn(state, choice, defender_choice, attacking_side);
        }
        None => {}
    }
}

fn move_special_effects(state: &State, choice: &mut Choice) {}

// Interpreting the function arguments/return-value:
//
// This function takes in a mutable StateInstruction,
// and returns a Vector of StateInstructions, which
// represent all the possible branches that can be taken
// given that move being used
pub fn generate_instructions_from_move(
    state: &mut State,
    mut choice: Choice,
    defender_choice: &Choice,
    attacking_side: SideReference,
    incoming_instructions: StateInstruction,
) -> Vec<StateInstruction> {
    /*
    The functions that are called by this function will each take a StateInstruction struct that
    signifies what has already happened. If the function can cause a branch, it will return a
    vector of StateInstructions, otherwise it will return a StateInstruction. In both cases,
    the functions will take ownership of the value, and return a new value.

    Note: end-of-turn instructions are not included here - this is only the instructions from a move

    Order of Operations:
    (*) indicates it can branch, (-) indicates it does not

    - check for if the user is switching - do so & exit early
    - check for short-curcuit situations that would exit before doing anything
        - DONE using drag move but you moved 2nd (possible if say both users use dragontail)
        - DONE attacking pokemon is dead (possible if you got KO-ed this turn)
        - DONE attacking pokemon is taunted & chose a non-damaging move
        - DONE attacker was flinched
            - not a branching event because the previous turn would've decided whether a flinch happened or not
    - update choice struct based on special effects
        - protect (or it's variants) nullifying a move
            - this may generate a custom instruction because some protect variants do things (spikyshield, banefulbunker, etc)
        - charging move that sets some charge flags and exits
        - move has no effect
            i.e. electric-type status move used against a ground type, powder move used against grass / overcoat
            normally, the move doing 0 damage would trigger this, but for non-damaging moves there needs to be another
            spot where this is checked. This may be better done elsewhere
        - move special effect
        - ability special effect (both sides)
        - item special effect (both sides)

    BEGIN THINGS THAT HAPPEN AFTER FIRST POSSIBLE BRANCH
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

    if !choice.first_move && choice.flags.drag {
        return vec![incoming_instructions];
    }

    state.apply_instructions(&incoming_instructions.instruction_list);

    if cannot_use_move(state, &choice, &attacking_side) {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        return vec![incoming_instructions];
    }

    // NEXT STEP:
    //
    // Need to make 2 functions:
    //  1st for updating the Choice {}
    //  2nd for generating custom instructions before the rest of the move
    //      This is where the callback will start: i.e. `ability_before_move`

    update_choice(state, &mut choice, defender_choice, &attacking_side);

    // This was just here to make sure it works - unsure where it will end up
    let damages_dealt = calculate_damage(state, attacking_side, &choice, DamageRolls::Average);

    println!("{:?}", damages_dealt);

    panic!("Not implemented yet");
}

pub fn generate_instructions_from_move_pair(//state: &mut State,
    //side_one_move: &String,
    //side_two_move: &String,
) -> Vec<Instruction> {
    panic!("Not implemented yet");
    /*
    - get Choice structs from moves
    - determine who moves first
    - initialize empty instructions
    - run move 1
    - run move 2
    - run end of turn instructions

    NOTE: End of turn instructions will need to generate the removing of certain volatile statuses, like flinched.
          This was done elsewhere in the other bot, but it should be here instead
    */

    // return vec![];
}

//fn update_move

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::moves::MOVES;
    use crate::instruction::{DamageInstruction, SwitchInstruction, VolatileStatusInstruction};
    use crate::state::{SideReference, State};

    fn get_empty_state_instruction() -> StateInstruction {
        return StateInstruction {
            percentage: 100.0,
            instruction_list: vec![],
        };
    }

    #[test]
    fn test_drag_move_as_second_move_exits_early() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("dragontail").unwrap().to_owned();
        choice.first_move = false;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            get_empty_state_instruction(),
        );
        assert_eq!(instructions, vec![get_empty_state_instruction()])
    }

    #[test]
    fn test_flinched_pokemon_cannot_move() {
        let mut state: State = State::default();
        let choice = MOVES.get("tackle").unwrap().to_owned();
        state
            .side_one
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::Flinch);

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            get_empty_state_instruction(),
        );
        assert_eq!(instructions, vec![get_empty_state_instruction()])
    }

    #[test]
    fn test_taunted_pokemon_cannot_use_status_move() {
        let mut state: State = State::default();
        let choice = MOVES.get("tackle").unwrap().to_owned();
        state
            .side_one
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::Taunt);

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            get_empty_state_instruction(),
        );
        assert_eq!(instructions, vec![get_empty_state_instruction()])
    }

    #[test]
    fn test_pokemon_taunted_on_first_turn_cannot_use_status_move() {
        let mut state: State = State::default();
        state
            .side_one
            .get_active()
            .volatile_statuses
            .insert(PokemonVolatileStatus::Taunt);

        let mut choice = MOVES.get("tackle").unwrap().to_owned();
        choice.first_move = false;

        let mut incoming_instructions = get_empty_state_instruction();
        incoming_instructions
            .instruction_list
            .push(Instruction::VolatileStatus(VolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::Taunt,
            }));

        let original_incoming_instructions = incoming_instructions.clone();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            incoming_instructions,
        );
        assert_eq!(instructions, vec![original_incoming_instructions])
    }

    #[test]
    fn test_dead_pokemon_moving_second_does_nothing() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("tackle").unwrap().to_owned();
        choice.first_move = false;
        state.side_one.get_active().hp = 0;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            get_empty_state_instruction(),
        );
        assert_eq!(instructions, vec![get_empty_state_instruction()])
    }

    #[test]
    fn test_basic_switch_functionality_with_no_prior_instructions() {
        let mut state: State = State::default();
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
            get_empty_state_instruction(),
        );

        assert_eq!(vec![expected_instructions], incoming_instructions);
    }

    #[test]
    fn test_basic_switch_functionality_with_a_prior_instruction() {
        let mut state: State = State::default();
        let mut incoming_instructions = get_empty_state_instruction();
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

                    let mut state: State = State::default();
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
            get_empty_state_instruction(),
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
            get_empty_state_instruction(),
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
