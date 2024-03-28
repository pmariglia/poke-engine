use std::io;
use std::io::Write;
use crate::choices::Choices;
use crate::generate_instructions::generate_instructions_from_move_pair;
use crate::instruction::{Instruction, StateInstructions};
use crate::search::expectiminimax_search;
use crate::state::{Move, MoveChoice, Pokemon, State};

struct IOData {
    state: State,
    instruction_list: Vec<Vec<Instruction>>,
    last_instructions_generated: Vec<StateInstructions>
}

impl Default for IOData {
    fn default() -> Self {
        IOData {
            state: State::default(),
            instruction_list: Vec::new(),
            last_instructions_generated: Vec::new()
        }
    }
}

impl Pokemon {
    fn io_print(&self) -> String {
        let moves: Vec<Choices> = self.moves.into_iter().map(|m| m.id).collect();
        return format!("Name: {}\nHP: {}/{}\nStatus: {:?}\nBoosts: {:?}\nMoves: {:?}",
            self.id, self.hp, self.maxhp, self.status, self.get_pkmn_boost_enum_pairs(), moves);
    }
}

pub fn command_loop() {
    let mut io_data = IOData::default();
    loop {
        print!("> ");
        io::stdout().flush();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(error) => {
                println!("Error reading input: {}", error);
                continue;
            }
        }
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let mut args = parts;

        match command {
            "state" | "s" => {
                let state_string;
                match args.next() {
                    Some(s) => {
                        state_string = s;
                        let state = State::deserialize(state_string);
                        io_data.state = state;
                        println!("state initialized");
                    },
                    None => {
                        println!("Expected state string");
                    }
                }
                println!("{:?}", io_data.state);

            }
            "matchup" | "m" => {
                let p1_active = io_data.state.side_one.get_active_immutable();
                let p2_active = io_data.state.side_two.get_active_immutable();
                let (side_one_options, side_two_options) = io_data.state.get_all_options();
                println!("{}\nAvailable Choices: {:?}\n\nvs\n\n{}\nAvailable Choices: {:?}\n", p1_active.io_print(), side_one_options, p2_active.io_print(), side_two_options);
            }
            "generate-instructions" | "g" => {
                let (s1_move, s2_move);
                match args.next() {
                    Some(s) => {
                        s1_move = MoveChoice::deserialize(s);
                    }
                    None => {
                        println!("Usage: generate-instructions <side-1 move> <side-2 move>");
                        continue;
                    }
                }
                match args.next() {
                    Some(s) => {
                        s2_move = MoveChoice::deserialize(s);
                    }
                    None => {
                        println!("Usage: generate-instructions <side-1 move> <side-2 move>");
                        continue;
                    }
                }
                let instructions = generate_instructions_from_move_pair(&mut io_data.state, &s1_move, &s2_move);
                println!("{:?}", instructions);
                io_data.last_instructions_generated = instructions;
            }
            "instructions" | "i" => {
                println!("{:?}", io_data.last_instructions_generated);
            }
            "apply" | "a" => {
                match args.next() {
                    Some(s) => {
                        let index = s.parse::<usize>().unwrap();
                        let instructions = io_data.last_instructions_generated.remove(index);
                        io_data.state.apply_instructions(&instructions.instruction_list);
                        io_data.instruction_list.push(instructions.instruction_list);
                        io_data.last_instructions_generated = Vec::new();
                    }
                    None => {
                        println!("Usage: apply <instruction index>");
                        continue;
                    }
                }
            }
            "pop" | "p" => {
                if io_data.instruction_list.is_empty() {
                    println!("No instructions to pop");
                    continue;
                }
                let instructions = io_data.instruction_list.pop().unwrap();
                io_data.state.reverse_instructions(&instructions);
            }
            "pop-all" | "pa" => {
                for i in io_data.instruction_list.iter().rev() {
                    io_data.state.reverse_instructions(i);
                }
                io_data.instruction_list.clear();
            }
            "expectiminimax" | "e" => {
                match args.next() {
                    Some(s) => {
                        let depth = s.parse::<i8>().unwrap();
                        let (side_one_options, side_two_options) = io_data.state.get_all_options();
                        let start_time = std::time::Instant::now();
                        let _result = expectiminimax_search(&mut io_data.state, depth, side_one_options, side_two_options, false);
                        let elapsed = start_time.elapsed();
                        println!("Took: {:?}", elapsed);
                    }
                    None => {
                        println!("Usage: expectiminimax <depth>");
                        continue;
                    }
                }
            }
            "" => {
                continue;
            }
            "exit" | "quit" | "q" => {
                break;
            }
            command => {
                println!("Unknown command: {}", command);
            }
        }
    }
}