use poke_engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::instruction::{BoostInstruction, DamageInstruction, Instruction, StateInstructions};
use poke_engine::state::{PokemonBoostableStat, SideReference, State};

#[test]
fn test_basic_move_pair_instruction_generation() {
    let mut state = State::default();
    state.side_one.get_active().speed = 100;
    state.side_two.get_active().speed = 50;
    let side_one_move = String::from("tackle");
    let side_two_move = String::from("tackle");

    let vec_of_instructions =
        generate_instructions_from_move_pair(&mut state, side_one_move, side_two_move);

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
        ],
    }];

    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_move_pair_instruction_generation_where_first_move_branches() {
    let mut state = State::default();
    state.side_one.get_active().speed = 100;
    state.side_two.get_active().speed = 50;
    let side_one_move = String::from("playrough");
    let side_two_move = String::from("tackle");

    let vec_of_instructions =
        generate_instructions_from_move_pair(&mut state, side_one_move, side_two_move);

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 9.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 71,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Attack,
                    amount: -1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    // playrough lowered attack means this does less dmg than other branches
                    damage_amount: 33,
                }),
            ],
        },
        StateInstructions {
            percentage: 81.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 71,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
            ],
        },
    ];

    assert_eq!(expected_instructions, vec_of_instructions)
}

#[test]
fn test_move_pair_instruction_generation_where_second_move_branches() {
    let mut state = State::default();
    state.side_one.get_active().speed = 50;
    state.side_two.get_active().speed = 100;
    let side_one_move = String::from("playrough");
    let side_two_move = String::from("tackle");

    let vec_of_instructions =
        generate_instructions_from_move_pair(&mut state, side_one_move, side_two_move);

    let expected_instructions = vec![
        StateInstructions {
            percentage: 10.000002,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            })],
        },
        StateInstructions {
            percentage: 9.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 71,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Attack,
                    amount: -1,
                }),
            ],
        },
        StateInstructions {
            percentage: 81.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 48,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 71,
                }),
            ],
        },
    ];

    assert_eq!(expected_instructions, vec_of_instructions)
}