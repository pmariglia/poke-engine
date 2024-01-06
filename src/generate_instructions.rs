use crate::choices::MoveTarget;
use crate::instruction::{
    BoostInstruction, ChangeItemInstruction, ChangeSideConditionInstruction, HealInstruction,
};
use crate::state::{PokemonBoostableStat, PokemonSideCondition, PokemonType};
use crate::{
    abilities::ABILITIES,
    choices::{Choice, MoveCategory},
    damage_calc::{calculate_damage, DamageRolls},
    instruction::{
        ChangeStatusInstruction, DamageInstruction, Instruction, StateInstructions,
        SwitchInstruction,
    },
    items::ITEMS,
    state::{Pokemon, PokemonStatus, PokemonVolatileStatus, SideReference, State, Weather},
};
use std::cmp;

type InstructionGenerationFn = fn(
    &mut State,
    &Choice,
    &SideReference,
    StateInstructions,
    &mut Vec<StateInstructions>,
) -> Vec<StateInstructions>;

fn generate_instructions_from_switch(
    state: &mut State,
    new_pokemon_index: usize,
    switching_side: SideReference,
    incoming_instructions: StateInstructions,
) -> Vec<StateInstructions> {
    let mut incoming_instructions = incoming_instructions;
    state.apply_instructions(&incoming_instructions.instruction_list);

    let switch_instruction = Instruction::Switch(SwitchInstruction {
        side_ref: switching_side,
        previous_index: state.get_side(&switching_side).active_index,
        next_index: new_pokemon_index,
    });
    state.apply_one_instruction(&switch_instruction);
    incoming_instructions
        .instruction_list
        .push(switch_instruction);

    state.reverse_instructions(&incoming_instructions.instruction_list);

    return vec![incoming_instructions];
}

fn generate_instructions_from_side_conditions(
    state: &mut State,
    choice: &Choice,
    attacking_side_reference: &SideReference,
    mut incoming_instructions: StateInstructions,
    _: &mut Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    if let Some(side_condition) = &choice.side_condition {
        state.apply_instructions(&incoming_instructions.instruction_list);

        let affected_side_ref;
        match side_condition.target {
            MoveTarget::Opponent => affected_side_ref = attacking_side_reference.get_other_side(),
            MoveTarget::User => affected_side_ref = *attacking_side_reference,
        }

        let affected_side = state.get_side_immutable(&affected_side_ref);

        let max_layers;
        match side_condition.condition {
            PokemonSideCondition::Spikes => max_layers = 3,
            PokemonSideCondition::ToxicSpikes => max_layers = 3,
            PokemonSideCondition::AuroraVeil => {
                max_layers = if state.weather.weather_type == Weather::Hail {
                    1
                } else {
                    0
                }
            }
            _ => max_layers = 1,
        }

        let mut additional_instructions = vec![];
        if affected_side.get_side_condition(side_condition.condition) < max_layers {
            additional_instructions.push(Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: affected_side_ref,
                    side_condition: side_condition.condition,
                    amount: 1,
                },
            ));
        }

        state.reverse_instructions(&incoming_instructions.instruction_list);

        for i in additional_instructions {
            incoming_instructions.instruction_list.push(i)
        }

        return vec![incoming_instructions];
    }

    return vec![incoming_instructions];
}

fn generate_instructions_from_ability_after_move(
    state: &mut State,
    choice: &Choice,
    side_reference: &SideReference,
    incoming_instructions: StateInstructions,
    frozen_instructions: &mut Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    return vec![incoming_instructions];
}

fn generate_instructions_from_move_special_effect(
    state: &mut State,
    choice: &Choice,
    side_reference: &SideReference,
    incoming_instructions: StateInstructions,
    frozen_instructions: &mut Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    return vec![incoming_instructions];
}

fn run_instruction_generation_fn(
    instruction_generation_fn: InstructionGenerationFn,
    state: &mut State,
    choice: &Choice,
    side_reference: &SideReference,
    incoming_instructions: Vec<StateInstructions>,
    frozen_instructions: &mut Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    let mut continuing_instructions: Vec<StateInstructions> = vec![];
    for instruction in incoming_instructions {
        continuing_instructions.extend(instruction_generation_fn(
            state,
            choice,
            side_reference,
            instruction,
            frozen_instructions,
        ));
    }
    return continuing_instructions;
}

fn generate_instructions_from_damage(
    state: &mut State,
    choice: &Choice,
    calculated_damage: i16,
    attacking_side_ref: &SideReference,
    incoming_instructions: StateInstructions,
    frozen_instructions: &mut Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    /*
    - Something for crash moves??
        - after_move_miss
        - after move miss??? <-- probably this

    - substitute consideration
        - requires a state change. VolatileStatus for sub & value for sub-health
        - do last tbh

    - DONE in-line because sturdy is a one-off. If it has more I can come back to it
        - before apply damage callback
            - for sturdy to reduce damage by 1 if necessary
            - tricky because this is not an instruction technically
                I guess you could add a heal instruction but that is kinda dumb

    - after apply damage callback (AFTER MOVE HIT) DONE FRAMEWORK
        - DONE for beastboost to generate an instruction if KO
        - DONE for drain moves???
        - DONE for recoil moves???
        - DONE after_damage_hit instructions
            - eg. for knockoff removing an item

    - after move miss callback (AFTER MOVE MISS)
        - DONE here is where crash would be ???
        - DONE blunderpolicy (in-line because blunderpolicy is a one-off. Cam come back if more fit the pattern)

    - arbitrary other after_move as well from the old engine (triggers on hit OR miss)
        - dig/dive/bounce/fly volatilestatus


    */

    let mut return_instructions: Vec<StateInstructions> = vec![];

    state.apply_instructions(&incoming_instructions.instruction_list);

    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    let attacking_pokemon = attacking_side.get_active_immutable();
    let defending_pokemon = defending_side.get_active_immutable();

    let percent_hit = choice.accuracy / 100.0;
    // Move hit
    if percent_hit > 0.0 {
        let mut move_hit_instructions = incoming_instructions.clone();

        let mut damage_dealt = cmp::min(calculated_damage, defending_pokemon.hp);

        if defending_pokemon.ability.as_str() == "sturdy"
            && defending_pokemon.maxhp == defending_pokemon.hp
        {
            damage_dealt -= 1;
        }

        move_hit_instructions
            .instruction_list
            .push(Instruction::Damage(DamageInstruction {
                side_ref: attacking_side_ref.get_other_side(),
                damage_amount: damage_dealt,
            }));

        move_hit_instructions.update_percentage(percent_hit);

        if let Some(ability) = ABILITIES.get(&attacking_pokemon.ability) {
            if let Some(after_damage_hit_fn) = ability.after_damage_hit {
                move_hit_instructions
                    .instruction_list
                    .extend(after_damage_hit_fn(
                        state,
                        choice,
                        attacking_side_ref,
                        damage_dealt,
                    ));
            };
        }

        /*
        Generating these instructions does not need to mutate the state, so use
        `attacking_pokemon_health` to keep track of the attacking pokemon's health separately
        */
        let mut attacking_pokemon_health = attacking_pokemon.hp;
        if let Some(drain_fraction) = choice.drain {
            let drain_amount = (damage_dealt as f32 * drain_fraction) as i16;
            let heal_amount = cmp::min(
                drain_amount,
                attacking_pokemon.maxhp - attacking_pokemon_health,
            );
            let drain_instruction = Instruction::Heal(HealInstruction {
                side_ref: *attacking_side_ref,
                heal_amount: heal_amount,
            });
            move_hit_instructions
                .instruction_list
                .push(drain_instruction);
            attacking_pokemon_health += heal_amount;
        }

        if let Some(recoil_fraction) = choice.recoil {
            let recoil_amount = (damage_dealt as f32 * recoil_fraction) as i16;
            let recoil_instruction = Instruction::Damage(DamageInstruction {
                side_ref: *attacking_side_ref,
                damage_amount: cmp::min(recoil_amount, attacking_pokemon_health),
            });
            move_hit_instructions
                .instruction_list
                .push(recoil_instruction);
        }

        if let Some(after_damage_hit_fn) = choice.after_damage_hit {
            move_hit_instructions
                .instruction_list
                .extend(after_damage_hit_fn(&state, &choice, attacking_side_ref));
        }

        return_instructions.push(move_hit_instructions);
    }

    // Move miss
    if percent_hit < 1.0 {
        let mut move_missed_instruction = incoming_instructions.clone();
        move_missed_instruction.update_percentage(1.0 - percent_hit);
        if let Some(crash_fraction) = choice.crash {
            let crash_amount = (attacking_pokemon.maxhp as f32 * crash_fraction) as i16;
            let crash_instruction = Instruction::Damage(DamageInstruction {
                side_ref: *attacking_side_ref,
                damage_amount: cmp::min(crash_amount, attacking_pokemon.hp),
            });

            move_missed_instruction
                .instruction_list
                .push(crash_instruction);
        }

        if attacking_pokemon.item.as_str() == "blunderpolicy"
            && attacking_pokemon.item_can_be_removed()
        {
            move_missed_instruction.instruction_list.extend(vec![
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: *attacking_side_ref,
                    current_item: String::from("blunderpolicy"),
                    new_item: "".to_string(),
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: *attacking_side_ref,
                    stat: PokemonBoostableStat::Speed,
                    amount: 2,
                }),
            ]);
        }

        frozen_instructions.push(move_missed_instruction);
    }

    state.reverse_instructions(&incoming_instructions.instruction_list);

    return return_instructions;
}

fn cannot_use_move(state: &State, choice: &Choice, attacking_side_ref: &SideReference) -> bool {
    /*
        Checks for any situation where a move cannot be used.
        Some examples:
            - electric type move versus a ground type
            - you are taunted and are trying to use a non-damaging move
            - you were flinched
            - etc.
    */
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
    } else if attacking_pkmn
        .volatile_statuses
        .contains(&PokemonVolatileStatus::Flinch)
    {
        return true;
    } else if choice.move_type == PokemonType::Electric
        && state
            .get_side_immutable(&attacking_side_ref.get_other_side())
            .get_active_immutable()
            .has_type(&PokemonType::Ground)
    {
        return true;
    } else if choice.flags.powder
        && state
            .get_side_immutable(&attacking_side_ref.get_other_side())
            .get_active_immutable()
            .has_type(&PokemonType::Grass)
    {
        return true;
    }

    return false;
}

fn before_move(state: &State, choice: &Choice, attacking_side: &SideReference) -> Vec<Instruction> {
    let mut new_instructions = vec![];
    let attacking_pokemon = state
        .get_side_immutable(attacking_side)
        .get_active_immutable();

    if let Some(ability) = ABILITIES.get(&attacking_pokemon.ability) {
        if let Some(before_move_fn) = ability.before_move {
            new_instructions.append(&mut before_move_fn(state, choice, attacking_side));
        };
    }

    return new_instructions;
}

// Updates the attacker's Choice based on some special effects
fn update_choice(
    state: &State,
    attacker_choice: &mut Choice,
    defender_choice: &Choice,
    attacking_side: &SideReference,
) {
    let (attacker_side, defender_side) = state.get_both_sides_immutable(attacking_side);
    let attacking_pokemon = attacker_side.get_active_immutable();
    let defending_pokemon = defender_side.get_active_immutable();

    match attacker_choice.modify_move {
        Some(modify_move_fn) => {
            modify_move_fn(state, attacker_choice, defender_choice, attacking_side);
        }
        None => {}
    }

    if let Some(ability) = ABILITIES.get(&attacking_pokemon.ability) {
        if let Some(modify_move_fn) = ability.modify_attack_being_used {
            modify_move_fn(state, attacker_choice, defender_choice, attacking_side)
        };
    }

    if let Some(ability) = ABILITIES.get(&defending_pokemon.ability) {
        if let Some(modify_move_fn) = ability.modify_attack_against {
            modify_move_fn(state, attacker_choice, defender_choice, attacking_side)
        };
    }

    if let Some(item) = ITEMS.get(&attacking_pokemon.item) {
        if let Some(modify_move_fn) = item.modify_attack_being_used {
            modify_move_fn(state, attacker_choice, attacking_side)
        };
    }

    if let Some(item) = ITEMS.get(&defending_pokemon.item) {
        if let Some(modify_move_fn) = item.modify_attack_against {
            modify_move_fn(state, attacker_choice, attacking_side)
        };
    }
}

fn generate_instructions_from_existing_status_conditions(
    state: &mut State,
    attacking_side_ref: &SideReference,
    mut incoming_instructions: StateInstructions,
    mut frozen_instructions: &mut Vec<StateInstructions>,
) -> Vec<StateInstructions> {
    // Frozen, Sleep, and Paralysis may cause a Pokemon to not move

    let apply_reverse_instruction_list = incoming_instructions.instruction_list.clone();
    state.apply_instructions(&apply_reverse_instruction_list);

    let (attacking_side, _defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    let attacker_active = attacking_side.get_active_immutable();

    let mut instructions_that_will_proceed = Vec::<StateInstructions>::new();
    match attacker_active.status {
        PokemonStatus::Paralyze => {
            // Fully-Paralyzed Branch
            let mut fully_paralyzed_instruction = incoming_instructions.clone();
            fully_paralyzed_instruction.update_percentage(0.25);
            frozen_instructions.push(fully_paralyzed_instruction);

            // Non-Paralyzed Branch
            incoming_instructions.update_percentage(0.75);
            instructions_that_will_proceed.push(incoming_instructions);
        }
        PokemonStatus::Freeze => {
            // Thawing is a 20% event, and changes a pokemons status
            let mut thaw_instruction = incoming_instructions.clone();
            thaw_instruction.update_percentage(0.20);
            thaw_instruction
                .instruction_list
                .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: attacking_side_ref.clone(),
                    pokemon_index: attacking_side.active_index,
                    old_status: attacker_active.status,
                    new_status: PokemonStatus::None,
                }));
            instructions_that_will_proceed.push(thaw_instruction);

            // staying frozen
            incoming_instructions.update_percentage(0.80);
            frozen_instructions.push(incoming_instructions);
        }
        PokemonStatus::Sleep => {
            // Waking up is a 33% event, and changes the status
            let mut awake_instruction = incoming_instructions.clone();
            awake_instruction.update_percentage(0.33);
            awake_instruction
                .instruction_list
                .push(Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: attacking_side_ref.clone(),
                    pokemon_index: attacking_side.active_index,
                    old_status: attacker_active.status,
                    new_status: PokemonStatus::None,
                }));
            instructions_that_will_proceed.push(awake_instruction);

            // staying asleep
            incoming_instructions.update_percentage(0.67);
            frozen_instructions.push(incoming_instructions);
        }
        _ => {
            instructions_that_will_proceed.push(incoming_instructions);
        }
    }

    state.reverse_instructions(&apply_reverse_instruction_list);
    return instructions_that_will_proceed;
}

// fn move_special_effects(state: &State, choice: &mut Choice) {}

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
    mut incoming_instructions: StateInstructions,
) -> Vec<StateInstructions> {
    /*
    The functions that are called by this function will each take a StateInstruction struct that
    signifies what has already happened. If the function can cause a branch, it will return a
    vector of StateInstructions, otherwise it will return a StateInstruction. In both cases,
    the functions will take ownership of the value, and return a new value.

    Note: end-of-turn instructions are not included here - this is only the instructions from a move

    Order of Operations:
    (*) indicates it can branch, (-) indicates it does not

    - check for if the user is switching - do so & exit early
    - check for short-circuit situations that would exit before doing anything
        - DONE using drag move but you moved 2nd (possible if say both users use dragontail)
        - DONE attacking pokemon is dead (possible if you got KO-ed this turn)
        - DONE attacking pokemon is taunted & chose a non-damaging move
        - DONE attacker was flinched
            - not a branching event because the previous turn would've decided whether a flinch happened or not
    - update choice struct based on special effects
        - protect (or it's variants) nullifying a move
            - this may generate a custom instruction because some protect variants do things (spikyshield, banefulbunker, etc)
            - rather than updating the choice struct, this should be a check that immediately adds the instruction list
              to `final_instructions` after applying the custom instructions from something like spikyshield ofc.
        - charging move that sets some charge flags and exits
            - again.. rather than exit, add the instructions to final instructions
        - DONE move special effect
        - DONE ability special effect (both sides)
        - DONE item special effect (both sides)

    BEGIN THINGS THAT HAPPEN AFTER FIRST POSSIBLE BRANCH
    * DONE attacker is fully-paralyzed, asleep, frozen (the first thing that can branch from the old engine)
    - DONE move has no effect (maybe something like check_if_move_can_hit)
        - i.e. electric-type status move used against a ground type, powder move used against grass / overcoat
        - Normally, the move doing 0 damage would trigger this, but for non-damaging moves there needs to be another
        spot where this is checked. This may be better done elsewhere
        - This HAS to be done after the frozen/sleep/paralyzed check, which is the first possible branch
    - move special effects
        hail, trick, futuresight, trickroom, etc. Anything that cannot be succinctly expressed in a Choice
        these will generate instructions (sometimes conditionally), but should not branch
    - MULTI HIT MOVES?!
    * DONE GOOD ENOUGH - WILL COME BACK TO AFTER ENGINE COMPLETE calculate damage amount(s) and do the damage
    - after-move effects
        * move special effect (both sides)
            - static/flamebody means this needs to possibly branch
            - protect and it's variants, which can generate some custom instructions
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

    if state
        .get_side_immutable(&attacking_side)
        .get_active_immutable()
        .hp
        == 0
    {
        state.reverse_instructions(&incoming_instructions.instruction_list);
        return vec![incoming_instructions];
    }

    // if cannot_use_move(state, &choice, &attacking_side) {
    //     state.reverse_instructions(&incoming_instructions.instruction_list);
    //     return vec![incoming_instructions];
    // }

    // Before-Move callbacks to update the choice
    update_choice(state, &mut choice, defender_choice, &attacking_side);

    // Before-Move callbacks to generate new instructions
    let before_move_instructions = before_move(state, &choice, &attacking_side);
    state.apply_instructions(&before_move_instructions);
    incoming_instructions
        .instruction_list
        .extend(before_move_instructions);

    let damage = calculate_damage(state, attacking_side, &choice, DamageRolls::Average);

    state.reverse_instructions(&incoming_instructions.instruction_list);

    // The final return-value of this function
    let mut final_instructions: Vec<StateInstructions> = vec![];
    let mut list_of_instructions = generate_instructions_from_existing_status_conditions(
        state,
        &attacking_side,
        incoming_instructions,
        &mut final_instructions,
    );

    let mut next_instructions = vec![];
    for instruction in list_of_instructions {
        state.apply_instructions(&instruction.instruction_list);
        if cannot_use_move(state, &choice, &attacking_side) {
            state.reverse_instructions(&instruction.instruction_list);
            final_instructions.push(instruction);
        } else {
            state.reverse_instructions(&instruction.instruction_list);
            next_instructions.push(instruction);
        }
    }
    next_instructions = run_instruction_generation_fn(
        generate_instructions_from_move_special_effect,
        state,
        &choice,
        &attacking_side,
        next_instructions,
        &mut final_instructions,
    );

    // Damage generation gets its own block because it has some special logic
    if let Some(damages_dealt) = damage {
        let mut temp_instructions: Vec<StateInstructions> = vec![];
        for instruction in next_instructions {
            let num_damage_amounts = damages_dealt.len() as f32;
            for dmg in &damages_dealt {
                let mut this_instruction = instruction.clone();
                this_instruction.update_percentage(1.0 / num_damage_amounts);
                println!("Instruction: {:?}, Run dmg: {:?}", this_instruction, dmg);
                temp_instructions.extend(generate_instructions_from_damage(
                    state,
                    &choice,
                    *dmg,
                    &attacking_side,
                    this_instruction,
                    &mut final_instructions,
                ));
            }
        }
        next_instructions = temp_instructions;
    }

    // Ability-After-Move (flamebody, static) should be done IN `generate_instructions_from_damage`
    // ... or not ... come back to that
    let instruction_generation_functions = [
        // generate_instructions_from_ability_after_move,
        generate_instructions_from_side_conditions,
        // get_instructions_from_hazard_clearing_moves,
        // get_instructions_from_volatile_statuses,
        // get_instructions_from_status_effects,
        // get_instructions_from_boosts,
        // get_instructions_from_boost_reset_moves,
        // get_instructions_from_attacker_recovery,
        // get_instructions_from_flinching_moves,
        // get_instructions_from_drag,

        // get_instructions_from_switch, // (u-turn and friends... probably omit this for now)
    ];

    for function in instruction_generation_functions {
        next_instructions = run_instruction_generation_fn(
            function,
            state,
            &choice,
            &attacking_side,
            next_instructions,
            &mut final_instructions,
        )
    }

    for instruction in next_instructions {
        final_instructions.push(instruction);
    }

    return final_instructions;
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
    use crate::choices::MOVES;
    use crate::instruction::{
        BoostInstruction, ChangeItemInstruction, ChangeStatusInstruction, DamageInstruction,
        SwitchInstruction, VolatileStatusInstruction,
    };
    use crate::state::{PokemonBoostableStat, SideReference, State};

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
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_electric_move_does_nothing_versus_ground_type() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("thunderbolt").unwrap().to_owned();
        state.side_two.get_active().types = (PokemonType::Ground, PokemonType::Typeless);
        choice.first_move = false;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_grass_type_cannot_have_powder_move_used_against_it() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("spore").unwrap().to_owned(); // Spore is a powder move
        state.side_two.get_active().types = (PokemonType::Grass, PokemonType::Typeless);
        choice.first_move = false;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_spikes_sets_first_layer() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("spikes").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: 1,
                },
            )],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_spikes_layers_cannot_exceed_3() {
        let mut state: State = State::default();
        state.side_two.side_conditions.spikes = 3;
        let mut choice = MOVES.get("spikes").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_auroa_veil_works_in_hail() {
        let mut state: State = State::default();
        state.weather.weather_type = Weather::Hail;
        let mut choice = MOVES.get("auroraveil").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::AuroraVeil,
                    amount: 1,
                },
            )],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_auroa_veil_fails_outside_of_hail() {
        let mut state: State = State::default();
        state.weather.weather_type = Weather::None;
        let mut choice = MOVES.get("auroraveil").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_stealthrock_cannot_exceed_1_layer() {
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        let mut choice = MOVES.get("stealthrock").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_stoneaxe_damage_and_stealthrock_setting() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("stoneaxe").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                        side_ref: SideReference::SideTwo,
                        side_condition: PokemonSideCondition::Stealthrock,
                        amount: 1,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_stoneaxe_does_not_set_stealthrock_if_already_set() {
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        let mut choice = MOVES.get("stoneaxe").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
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
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
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
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
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

        let mut incoming_instructions = StateInstructions::default();
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
            StateInstructions::default(),
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_cannot_ohko_versus_study() {
        let mut state: State = State::default();
        let choice = MOVES.get("earthquake").unwrap().to_owned();
        state.side_two.get_active().ability = String::from("sturdy");
        state.side_two.get_active().hp = 50;
        state.side_two.get_active().maxhp = 50;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 49,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_sturdy_does_not_affect_non_ohko_move() {
        let mut state: State = State::default();
        let choice = MOVES.get("earthquake").unwrap().to_owned();
        state.side_two.get_active().ability = String::from("sturdy");
        state.side_two.get_active().hp = 45;
        state.side_two.get_active().maxhp = 50;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 45,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_boosts_on_kill() {
        let mut state: State = State::default();
        let choice = MOVES.get("tackle").unwrap().to_owned();
        state.side_one.get_active().ability = String::from("beastboost");
        state.side_one.get_active().attack = 500; // highest stat
        state.side_two.get_active().hp = 1;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: 1,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_does_not_boost_without_kill() {
        let mut state: State = State::default();
        let choice = MOVES.get("tackle").unwrap().to_owned();
        state.side_one.get_active().ability = String::from("beastboost");
        state.side_one.get_active().attack = 150; // highest stat
        state.side_two.get_active().hp = 100;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 72,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_move_heals() {
        let mut state: State = State::default();
        let choice = MOVES.get("absorb").unwrap().to_owned();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().maxhp = 200;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 8,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_move_does_not_overheal() {
        let mut state: State = State::default();
        let choice = MOVES.get("absorb").unwrap().to_owned();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().maxhp = 105;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_recoil_damage() {
        let mut state: State = State::default();
        let choice = MOVES.get("bravebird").unwrap().to_owned();
        state.side_one.get_active().hp = 105;
        state.side_one.get_active().maxhp = 105;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 94,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 31,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_recoil_cannot_overkill() {
        let mut state: State = State::default();
        let choice = MOVES.get("bravebird").unwrap().to_owned();
        state.side_one.get_active().hp = 5;
        state.side_one.get_active().maxhp = 105;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 94,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_and_recoil_together() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("absorb").unwrap().to_owned();
        choice.recoil = Some(0.33);
        state.side_one.get_active().hp = 1;
        state.side_one.get_active().maxhp = 105;

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 8,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_crash_move_missing() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("jumpkick").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 5.000001,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 50,
                })],
            },
            StateInstructions {
                percentage: 95.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_crash_move_missing_cannot_overkill() {
        let mut state: State = State::default();
        state.get_side(&SideReference::SideOne).get_active().hp = 5;
        let mut choice = MOVES.get("jumpkick").unwrap().to_owned();

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 5.000001,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                })],
            },
            StateInstructions {
                percentage: 95.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_knockoff_removing_item() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("knockoff").unwrap().to_owned();
        state.get_side(&SideReference::SideTwo).get_active().item = String::from("item");

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                }),
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: SideReference::SideTwo,
                    current_item: "item".to_string(),
                    new_item: "".to_string(),
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_blunderpolicy_boost() {
        let mut state: State = State::default();
        let mut choice = MOVES.get("crosschop").unwrap().to_owned();
        state.get_side(&SideReference::SideOne).get_active().item = String::from("blunderpolicy");

        let instructions = generate_instructions_from_move(
            &mut state,
            choice,
            MOVES.get("tackle").unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 19.999998,
                instruction_list: vec![
                    Instruction::ChangeItem(ChangeItemInstruction {
                        side_ref: SideReference::SideOne,
                        current_item: "blunderpolicy".to_string(),
                        new_item: "".to_string(),
                    }),
                    Instruction::Boost(BoostInstruction {
                        side_ref: SideReference::SideOne,
                        stat: PokemonBoostableStat::Speed,
                        amount: 2,
                    }),
                ],
            },
            StateInstructions {
                percentage: 80.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn test_basic_switch_functionality_with_no_prior_instructions() {
        let mut state: State = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        choice.switch_id = 1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: 0,
                next_index: 1,
            })],
            ..Default::default()
        };

        let incoming_instructions = generate_instructions_from_switch(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            StateInstructions::default(),
        );

        assert_eq!(vec![expected_instructions], incoming_instructions);
    }

    #[test]
    fn test_basic_switch_functionality_with_a_prior_instruction() {
        let mut state: State = State::default();
        let mut incoming_instructions = StateInstructions::default();
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

        let expected_instructions: StateInstructions = StateInstructions {
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
            ..Default::default()
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
    fn test_healthy_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        let incoming_instructions = StateInstructions::default();

        let expected_instructions = vec![StateInstructions::default()];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            &mut vec![],
        );

        assert_eq!(expected_instructions, actual_instructions);
    }

    #[test]
    fn test_paralyzed_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Paralyze;
        let incoming_instructions = StateInstructions::default();

        let expected_instructions = vec![StateInstructions {
            percentage: 75.0,
            instruction_list: vec![],
        }];

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 25.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, actual_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_frozen_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Freeze;
        let incoming_instructions = StateInstructions::default();

        let expected_instructions = vec![StateInstructions {
            percentage: 20.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: PokemonStatus::Freeze,
                new_status: PokemonStatus::None,
            })],
        }];

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 80.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, actual_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Sleep;
        let incoming_instructions = StateInstructions::default();

        let expected_instructions = vec![StateInstructions {
            percentage: 33.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: PokemonStatus::Sleep,
                new_status: PokemonStatus::None,
            })],
        }];

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 67.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, actual_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_paralyzed_pokemon_preserves_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Paralyze;
        let mut incoming_instructions = StateInstructions::default();
        incoming_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })];

        let expected_instructions = vec![StateInstructions {
            percentage: 75.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        }];

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 25.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        }];

        let frozen_instructions = &mut vec![];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, actual_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_previous_instruction_removing_paralysis_stops_the_branch() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::Paralyze; // pokemon is paralyzed

        let mut incoming_instructions = StateInstructions::default();
        // there is an incoming instruction to remove the paralysis
        incoming_instructions.instruction_list =
            vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: state.side_one.get_active_immutable().status,
                new_status: PokemonStatus::None,
            })];

        // expected instructions are the incoming instructions, since the paralysis check should
        // fail
        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: state.side_one.get_active_immutable().status,
                new_status: PokemonStatus::None,
            })],
            ..Default::default()
        }];

        let actual_instructions = generate_instructions_from_existing_status_conditions(
            &mut state,
            &SideReference::SideOne,
            incoming_instructions,
            &mut vec![],
        );

        assert_eq!(expected_instructions, actual_instructions);
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
                        expected_instructions,
                        mut frozen_instructions_start,
                        expected_frozen_instructions_end
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
                        &choice,
                        35,
                        &SideReference::SideOne,
                        incoming_instructions,
                        &mut frozen_instructions_start
                    );

                    assert_eq!(expected_instructions, new_instructions);
                    assert_eq!(frozen_instructions_start, expected_frozen_instructions_end);
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
            StateInstructions::default(),
            vec![StateInstructions {
                percentage: 100.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 35,
                })],
            ..Default::default()
            }],
            vec![],
            vec![]
        ),
        test_basic_move_with_90_accuracy: (
            "tackle".to_string(),
            MoveCategory::Physical,
            40.0,
            90.0,
            StateInstructions::default(),
            vec![
                StateInstructions {
                    percentage: 90.0,
                    instruction_list: vec![Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 35,
                    })],
                    ..Default::default()
                },
            ],
            vec![],
            vec![
                StateInstructions {
                    percentage: 10.000002,
                    instruction_list: vec![],
                    ..Default::default()
                },
            ]
        ),
    }
}
