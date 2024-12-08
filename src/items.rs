#![allow(unused_variables)]
use crate::abilities::Abilities;
use crate::choices::{Choice, Choices, Effect, MoveCategory, MoveTarget, Secondary, StatBoosts};
use crate::damage_calc::type_effectiveness_modifier;
use crate::define_enum_with_from_str;
use crate::generate_instructions::{get_boost_instruction, immune_to_status};
use crate::instruction::{
    ChangeItemInstruction, ChangeStatusInstruction, DamageInstruction, DisableMoveInstruction,
    HealInstruction, Instruction, StateInstructions,
};
use crate::pokemon::PokemonName;
use crate::state::{Pokemon, PokemonType, Side};
use crate::state::{PokemonBoostableStat, State, Terrain};
use crate::state::{PokemonStatus, SideReference};
use std::cmp;

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    Items {
        NONE,
        UNKNOWNITEM,
        ABSORBBULB,
        ADRENALINEORB,
        ADAMANTORB,
        ADAMANTCRYSTAL,
        AIRBALLOON,
        ASSAULTVEST,
        BABIRIBERRY,
        BLACKBELT,
        BLACKSLUDGE,
        BLACKGLASSES,
        BLANKPLATE,
        BOOSTERENERGY,
        CELLBATTERY,
        CHARCOAL,
        CHARTIBERRY,
        CHILANBERRY,
        CHOICEBAND,
        CHOICESPECS,
        CHOICESCARF,
        CHOPLEBERRY,
        COBABERRY,
        COLBURBERRY,
        CUSTAPBERRY,
        DRAGONFANG,
        DRAGONSCALE,
        DREADPLATE,
        EARTHPLATE,
        ELECTRICSEED,
        EXPERTBELT,
        EVIOLITE,
        FAIRYFEATHER,
        FISTPLATE,
        FLAMEORB,
        GRASSYSEED,
        HABANBERRY,
        KASIBBERRY,
        KEBIABERRY,
        LEFTOVERS,
        LIFEORB,
        LUSTROUSORB,
        LUSTROUSGLOBE,
        METALCOAT,
        MISTYSEED,
        MUSCLEBAND,
        MYSTICWATER,
        NEVERMELTICE,
        PINKBOW,
        POLKADOTBOW,
        OCCABERRY,
        ODDINCENSE,
        PASSHOBERRY,
        PAYAPABERRY,
        POISONBARB,
        POWERHERB,
        PSYCHICSEED,
        PUNCHINGGLOVE,
        RINDOBERRY,
        ROSELIBERRY,
        ROCKYHELMET,
        SEAINCENSE,
        SHARPBEAK,
        SPELLTAG,
        MIRACLESEED,
        SHELLBELL,
        SHUCABERRY,
        SILKSCARF,
        SILVERPOWDER,
        SKYPLATE,
        SOFTSAND,
        SOULDEW,
        GRISEOUSORB,
        GRISEOUSCORE,
        TANGABERRY,
        THROATSPRAY,
        THICKCLUB,
        TOXICORB,
        TOXICPLATE,
        TWISTEDSPOON,
        HARDSTONE,
        METALPOWDER,
        WACANBERRY,
        WAVEINCENSE,
        MAGNET,
        WEAKNESSPOLICY,
        WISEGLASSES,
        BLUNDERPOLICY,
        HEAVYDUTYBOOTS,
        CLEARAMULET,
        PROTECTIVEPADS,
        SHEDSHELL,
        YACHEBERRY,
        STONEPLATE,
        INSECTPLATE,
        SPOOKYPLATE,
        IRONBALL,
        IRONPLATE,
        FLAMEPLATE,
        SPLASHPLATE,
        MEADOWPLATE,
        ZAPPLATE,
        MINDPLATE,
        ICICLEPLATE,
        DRACOPLATE,
        PIXIEPLATE,
        LIGHTBALL,
        FOCUSSASH,
        LUMBERRY,
        SITRUSBERRY,
        PETAYABERRY,
        SALACBERRY,
        LIECHIBERRY,
        NORMALGEM,
        BUGGEM,
        ELECTRICGEM,
        FIGHTINGGEM,
        GHOSTGEM,
        PSYCHICGEM,
        FLYINGGEM,
        STEELGEM,
        ICEGEM,
        POISONGEM,
        FIREGEM,
        DRAGONGEM,
        GROUNDGEM,
        WATERGEM,
        DARKGEM,
        ROCKGEM,
        GRASSGEM,
        FAIRYGEM,
        BUGMEMORY,
        FIGHTINGMEMORY,
        GHOSTMEMORY,
        PSYCHICMEMORY,
        FLYINGMEMORY,
        STEELMEMORY,
        ICEMEMORY,
        POISONMEMORY,
        FIREMEMORY,
        DRAGONMEMORY,
        GROUNDMEMORY,
        WATERMEMORY,
        DARKMEMORY,
        ROCKMEMORY,
        GRASSMEMORY,
        FAIRYMEMORY,
        ELECTRICMEMORY,
        WELLSPRINGMASK,
        HEARTHFLAMEMASK,
        CORNERSTONEMASK,
        WIDELENS,
    },
    default = UNKNOWNITEM
}

pub fn get_choice_move_disable_instructions(
    pkmn: &Pokemon,
    side_ref: &SideReference,
    move_name: &Choices,
) -> Vec<Instruction> {
    let mut moves_to_disable = vec![];
    let mut iter = pkmn.moves.into_iter();
    while let Some(p) = iter.next() {
        if &p.id != move_name && p.disabled == false {
            moves_to_disable.push(Instruction::DisableMove(DisableMoveInstruction {
                side_ref: *side_ref,
                move_index: iter.pokemon_move_index,
            }));
        }
    }
    moves_to_disable
}

fn damage_reduction_berry(
    defending_pkmn: &mut Pokemon,
    attacking_side_ref: &SideReference,
    choice: &mut Choice,
    berry: Items,
    pkmn_type: &PokemonType,
    instructions: &mut StateInstructions,
) {
    if &choice.move_type == pkmn_type
        && type_effectiveness_modifier(pkmn_type, &defending_pkmn) > 1.0
    {
        instructions
            .instruction_list
            .push(Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: attacking_side_ref.get_other_side(),
                current_item: berry,
                new_item: Items::NONE,
            }));
        defending_pkmn.item = Items::NONE;
        choice.base_power /= 2.0;
    }
}

/*
NormalGem, FlyingGem, etc.
*/
fn power_up_gem(
    attacking_side_ref: &SideReference,
    attacking_pkmn: &mut Pokemon,
    choice: &mut Choice,
    gem_type: PokemonType,
    instructions: &mut StateInstructions,
) {
    if &choice.move_type == &gem_type {
        #[cfg(feature = "gen5")]
        {
            choice.base_power *= 1.5;
        }
        #[cfg(not(feature = "gen5"))]
        {
            choice.base_power *= 1.3;
        }

        instructions
            .instruction_list
            .push(Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: *attacking_side_ref,
                current_item: attacking_pkmn.item,
                new_item: Items::NONE,
            }));
        attacking_pkmn.item = Items::NONE;
    }
}

/*
Regarding berries:
    most berries (lum, sitrus, etc) activate right away when applicable, but there isn't
    logic in this engine to implement that. Attempting to activate these berries before the user's
    move AND at the end-of-turn should be accurate enough for a simulation. The item is
    removed after this is triggered so only one will take effect
*/
fn lum_berry(
    side_ref: &SideReference,
    attacking_side: &mut Side,
    instructions: &mut StateInstructions,
) {
    let active_index = attacking_side.active_index;
    let active_pkmn = attacking_side.get_active();
    instructions
        .instruction_list
        .push(Instruction::ChangeStatus(ChangeStatusInstruction {
            side_ref: *side_ref,
            pokemon_index: active_index,
            new_status: PokemonStatus::NONE,
            old_status: active_pkmn.status,
        }));
    active_pkmn.status = PokemonStatus::NONE;
    instructions
        .instruction_list
        .push(Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: *side_ref,
            current_item: Items::LUMBERRY,
            new_item: Items::NONE,
        }));
    active_pkmn.item = Items::NONE;
}

fn sitrus_berry(
    side_ref: &SideReference,
    attacking_side: &mut Side,
    instructions: &mut StateInstructions,
) {
    let active_pkmn = attacking_side.get_active();
    let heal_amount = cmp::min(active_pkmn.maxhp / 4, active_pkmn.maxhp - active_pkmn.hp);
    instructions
        .instruction_list
        .push(Instruction::Heal(HealInstruction {
            side_ref: *side_ref,
            heal_amount: heal_amount,
        }));
    active_pkmn.hp += heal_amount;
    instructions
        .instruction_list
        .push(Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: *side_ref,
            current_item: Items::SITRUSBERRY,
            new_item: Items::NONE,
        }));
    active_pkmn.item = Items::NONE;
}

fn boost_berry(
    side_ref: &SideReference,
    attacking_side: &mut Side,
    stat: PokemonBoostableStat,
    instructions: &mut StateInstructions,
) {
    if let Some(ins) = get_boost_instruction(&attacking_side, &stat, &1, side_ref, side_ref) {
        match stat {
            PokemonBoostableStat::Attack => attacking_side.attack_boost += 1,
            PokemonBoostableStat::Defense => attacking_side.defense_boost += 1,
            PokemonBoostableStat::SpecialAttack => attacking_side.special_attack_boost += 1,
            PokemonBoostableStat::SpecialDefense => attacking_side.special_defense_boost += 1,
            PokemonBoostableStat::Speed => attacking_side.speed_boost += 1,
            PokemonBoostableStat::Accuracy => attacking_side.accuracy_boost += 1,
            PokemonBoostableStat::Evasion => attacking_side.evasion_boost += 1,
        }
        instructions.instruction_list.push(ins);
    }
    let attacker = attacking_side.get_active();
    instructions
        .instruction_list
        .push(Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: *side_ref,
            current_item: attacker.item,
            new_item: Items::NONE,
        }));
    attacking_side.get_active().item = Items::NONE;
}

pub fn item_before_move(
    state: &mut State,
    choice: &mut Choice,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let (attacking_side, defending_side) = state.get_both_sides(side_ref);
    let active_pkmn = attacking_side.get_active();
    let defending_pkmn = defending_side.get_active();
    match defending_pkmn.item {
        Items::CHOPLEBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::CHOPLEBERRY,
            &PokemonType::FIGHTING,
            instructions,
        ),
        Items::BABIRIBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::BABIRIBERRY,
            &PokemonType::STEEL,
            instructions,
        ),
        Items::CHARTIBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::CHARTIBERRY,
            &PokemonType::ROCK,
            instructions,
        ),
        Items::CHILANBERRY => {
            // no type effectiveness check for chilan
            if &choice.move_type == &PokemonType::NORMAL {
                instructions.instruction_list.push(Instruction::ChangeItem(
                    ChangeItemInstruction {
                        side_ref: side_ref.get_other_side(),
                        current_item: Items::CHILANBERRY,
                        new_item: Items::NONE,
                    },
                ));
                defending_pkmn.item = Items::NONE;
                choice.base_power /= 2.0;
            }
        }
        Items::COBABERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::COBABERRY,
            &PokemonType::FLYING,
            instructions,
        ),
        Items::COLBURBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::COLBURBERRY,
            &PokemonType::DARK,
            instructions,
        ),
        Items::HABANBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::HABANBERRY,
            &PokemonType::DRAGON,
            instructions,
        ),
        Items::KASIBBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::KASIBBERRY,
            &PokemonType::GHOST,
            instructions,
        ),
        Items::KEBIABERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::KEBIABERRY,
            &PokemonType::POISON,
            instructions,
        ),
        Items::OCCABERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::OCCABERRY,
            &PokemonType::FIRE,
            instructions,
        ),
        Items::PASSHOBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::PASSHOBERRY,
            &PokemonType::WATER,
            instructions,
        ),
        Items::PAYAPABERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::PAYAPABERRY,
            &PokemonType::PSYCHIC,
            instructions,
        ),
        Items::RINDOBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::RINDOBERRY,
            &PokemonType::GRASS,
            instructions,
        ),
        Items::ROSELIBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::ROSELIBERRY,
            &PokemonType::FAIRY,
            instructions,
        ),
        Items::SHUCABERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::SHUCABERRY,
            &PokemonType::GROUND,
            instructions,
        ),
        Items::TANGABERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::TANGABERRY,
            &PokemonType::BUG,
            instructions,
        ),
        Items::WACANBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::WACANBERRY,
            &PokemonType::ELECTRIC,
            instructions,
        ),
        Items::YACHEBERRY => damage_reduction_berry(
            defending_pkmn,
            side_ref,
            choice,
            Items::YACHEBERRY,
            &PokemonType::ICE,
            instructions,
        ),
        _ => {}
    }
    match active_pkmn.item {
        Items::NORMALGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::NORMAL,
            instructions,
        ),
        Items::BUGGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::BUG,
            instructions,
        ),
        Items::ELECTRICGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::ELECTRIC,
            instructions,
        ),
        Items::FIGHTINGGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::FIGHTING,
            instructions,
        ),
        Items::GHOSTGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::GHOST,
            instructions,
        ),
        Items::PSYCHICGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::PSYCHIC,
            instructions,
        ),
        Items::FLYINGGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::FLYING,
            instructions,
        ),
        Items::STEELGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::STEEL,
            instructions,
        ),
        Items::ICEGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::ICE,
            instructions,
        ),
        Items::POISONGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::POISON,
            instructions,
        ),
        Items::FIREGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::FIRE,
            instructions,
        ),
        Items::DRAGONGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::DRAGON,
            instructions,
        ),
        Items::GROUNDGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::GROUND,
            instructions,
        ),
        Items::WATERGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::WATER,
            instructions,
        ),
        Items::DARKGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::DARK,
            instructions,
        ),
        Items::ROCKGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::ROCK,
            instructions,
        ),
        Items::GRASSGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::GRASS,
            instructions,
        ),
        Items::FAIRYGEM => power_up_gem(
            side_ref,
            active_pkmn,
            choice,
            PokemonType::FAIRY,
            instructions,
        ),
        Items::LUMBERRY if active_pkmn.status != PokemonStatus::NONE => {
            lum_berry(side_ref, attacking_side, instructions)
        }
        Items::SITRUSBERRY
            if active_pkmn.ability == Abilities::GLUTTONY
                && active_pkmn.hp < active_pkmn.maxhp / 2 =>
        {
            sitrus_berry(side_ref, attacking_side, instructions)
        }
        Items::SITRUSBERRY if active_pkmn.hp < active_pkmn.maxhp / 4 => {
            sitrus_berry(side_ref, attacking_side, instructions)
        }
        Items::PETAYABERRY if active_pkmn.hp < active_pkmn.maxhp / 4 => boost_berry(
            side_ref,
            attacking_side,
            PokemonBoostableStat::SpecialAttack,
            instructions,
        ),
        Items::LIECHIBERRY if active_pkmn.hp < active_pkmn.maxhp / 4 => boost_berry(
            side_ref,
            attacking_side,
            PokemonBoostableStat::Attack,
            instructions,
        ),
        Items::SALACBERRY if active_pkmn.hp < active_pkmn.maxhp / 4 => boost_berry(
            side_ref,
            attacking_side,
            PokemonBoostableStat::Speed,
            instructions,
        ),
        Items::CUSTAPBERRY => {
            if active_pkmn.hp <= active_pkmn.maxhp / 4 {
                active_pkmn.item = Items::NONE;
                instructions.instruction_list.push(Instruction::ChangeItem(
                    ChangeItemInstruction {
                        side_ref: *side_ref,
                        current_item: Items::CUSTAPBERRY,
                        new_item: Items::NONE,
                    },
                ));
            }
        }
        Items::CHOICESPECS | Items::CHOICEBAND | Items::CHOICESCARF => {
            let ins = get_choice_move_disable_instructions(active_pkmn, side_ref, &choice.move_id);
            for i in ins {
                state.apply_one_instruction(&i);
                instructions.instruction_list.push(i);
            }
        }
        Items::PROTECTIVEPADS => {
            choice.flags.contact = false;
        }
        _ => {}
    }
}

pub fn item_on_switch_in(
    state: &mut State,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let switching_in_side = state.get_side_immutable(side_ref);
    let switching_in_pkmn = switching_in_side.get_active_immutable();
    match switching_in_pkmn.item {
        Items::ELECTRICSEED => {
            if state.terrain_is_active(&Terrain::ELECTRICTERRAIN) {
                if let Some(boost_instruction) = get_boost_instruction(
                    &switching_in_side,
                    &PokemonBoostableStat::Defense,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                    state.get_side(side_ref).get_active().item = Items::NONE;
                    instructions.instruction_list.push(Instruction::ChangeItem(
                        ChangeItemInstruction {
                            side_ref: side_ref.clone(),
                            current_item: Items::ELECTRICSEED,
                            new_item: Items::NONE,
                        },
                    ));
                }
            }
        }
        Items::GRASSYSEED => {
            if state.terrain_is_active(&Terrain::GRASSYTERRAIN) {
                if let Some(boost_instruction) = get_boost_instruction(
                    &switching_in_side,
                    &PokemonBoostableStat::Defense,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                    state.get_side(side_ref).get_active().item = Items::NONE;
                    instructions.instruction_list.push(Instruction::ChangeItem(
                        ChangeItemInstruction {
                            side_ref: side_ref.clone(),
                            current_item: Items::GRASSYSEED,
                            new_item: Items::NONE,
                        },
                    ));
                }
            }
        }
        Items::MISTYSEED => {
            if state.terrain_is_active(&Terrain::MISTYTERRAIN) {
                if let Some(boost_instruction) = get_boost_instruction(
                    &switching_in_side,
                    &PokemonBoostableStat::SpecialDefense,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                    state.get_side(side_ref).get_active().item = Items::NONE;
                    instructions.instruction_list.push(Instruction::ChangeItem(
                        ChangeItemInstruction {
                            side_ref: side_ref.clone(),
                            current_item: Items::MISTYSEED,
                            new_item: Items::NONE,
                        },
                    ));
                }
            }
        }
        Items::PSYCHICSEED => {
            if state.terrain_is_active(&Terrain::PSYCHICTERRAIN) {
                if let Some(boost_instruction) = get_boost_instruction(
                    &switching_in_side,
                    &PokemonBoostableStat::SpecialDefense,
                    &1,
                    side_ref,
                    side_ref,
                ) {
                    state.apply_one_instruction(&boost_instruction);
                    instructions.instruction_list.push(boost_instruction);
                    state.get_side(side_ref).get_active().item = Items::NONE;
                    instructions.instruction_list.push(Instruction::ChangeItem(
                        ChangeItemInstruction {
                            side_ref: side_ref.clone(),
                            current_item: Items::PSYCHICSEED,
                            new_item: Items::NONE,
                        },
                    ));
                }
            }
        }
        _ => {}
    }
}

pub fn item_end_of_turn(
    state: &mut State,
    side_ref: &SideReference,
    instructions: &mut StateInstructions,
) {
    let attacking_side = state.get_side(side_ref);
    let active_pkmn = attacking_side.get_active();
    match active_pkmn.item {
        Items::LUMBERRY if active_pkmn.status != PokemonStatus::NONE => {
            lum_berry(side_ref, attacking_side, instructions)
        }
        Items::SITRUSBERRY if active_pkmn.hp < active_pkmn.maxhp / 2 => {
            sitrus_berry(side_ref, attacking_side, instructions)
        }
        Items::BLACKSLUDGE => {
            if active_pkmn.has_type(&PokemonType::POISON) {
                if active_pkmn.hp < active_pkmn.maxhp {
                    let heal_amount =
                        cmp::min(active_pkmn.maxhp / 16, active_pkmn.maxhp - active_pkmn.hp);
                    let ins = Instruction::Heal(HealInstruction {
                        side_ref: side_ref.clone(),
                        heal_amount: heal_amount,
                    });
                    active_pkmn.hp += heal_amount;
                    instructions.instruction_list.push(ins);
                }
            } else {
                let damage_amount =
                    cmp::min(active_pkmn.maxhp / 16, active_pkmn.maxhp - active_pkmn.hp);
                let ins = Instruction::Damage(DamageInstruction {
                    side_ref: side_ref.clone(),
                    damage_amount: damage_amount,
                });
                active_pkmn.hp -= damage_amount;
                instructions.instruction_list.push(ins);
            }
        }
        Items::FLAMEORB => {
            if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::BURN) {
                let side = state.get_side(side_ref);
                let ins = Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: side_ref.clone(),
                    pokemon_index: side.active_index,
                    new_status: PokemonStatus::BURN,
                    old_status: PokemonStatus::NONE,
                });
                side.get_active().status = PokemonStatus::BURN;
                instructions.instruction_list.push(ins);
            }
        }
        Items::LEFTOVERS => {
            let attacker = state.get_side(side_ref).get_active();
            if attacker.hp < attacker.maxhp {
                let heal_amount = cmp::min(attacker.maxhp / 16, attacker.maxhp - attacker.hp);
                let ins = Instruction::Heal(HealInstruction {
                    side_ref: side_ref.clone(),
                    heal_amount: heal_amount,
                });
                attacker.hp += heal_amount;
                instructions.instruction_list.push(ins);
            }
        }
        Items::TOXICORB => {
            if !immune_to_status(state, &MoveTarget::User, side_ref, &PokemonStatus::TOXIC) {
                let side = state.get_side(side_ref);
                let ins = Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: side_ref.clone(),
                    pokemon_index: side.active_index,
                    new_status: PokemonStatus::TOXIC,
                    old_status: PokemonStatus::NONE,
                });
                side.get_active().status = PokemonStatus::TOXIC;
                instructions.instruction_list.push(ins);
            }
        }
        _ => {}
    }
}

pub fn item_modify_attack_against(
    state: &State,
    attacking_choice: &mut Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    match defending_side.get_active_immutable().item {
        Items::ABSORBBULB => {
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                    target: MoveTarget::Opponent,
                });
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::Opponent,
                });
            }
        }
        Items::AIRBALLOON => {
            if attacking_choice.move_type == PokemonType::GROUND
                && attacking_choice.move_id != Choices::THOUSANDARROWS
            {
                attacking_choice.base_power = 0.0;
            } else {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::Opponent,
                });
            }
        }
        Items::ASSAULTVEST => {
            if attacking_choice.category == MoveCategory::Special {
                attacking_choice.base_power /= 1.5;
            }
        }
        Items::METALPOWDER if attacking_side.get_active_immutable().id == PokemonName::DITTO => {
            attacking_choice.base_power /= 1.5;
        }
        Items::CELLBATTERY => {
            if attacking_choice.move_type == PokemonType::ELECTRIC {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Boost(StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                    target: MoveTarget::Opponent,
                });
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::Opponent,
                });
            }
        }
        Items::EVIOLITE => {
            attacking_choice.base_power /= 1.5;
        }
        Items::ROCKYHELMET => {
            if attacking_choice.flags.contact {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Heal(-0.166),
                    target: MoveTarget::User,
                })
            }
        }
        Items::WEAKNESSPOLICY => {
            if attacking_choice.category != MoveCategory::Status
                && type_effectiveness_modifier(
                    &attacking_choice.move_type,
                    &defending_side.get_active_immutable(),
                ) > 1.0
            {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Boost(StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                    target: MoveTarget::Opponent,
                });
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::Opponent,
                });
            }
        }
        Items::SOULDEW => {
            if defending_side.get_active_immutable().id == PokemonName::LATIOS
                || defending_side.get_active_immutable().id == PokemonName::LATIAS
            {
                #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5", feature = "gen6"))]
                if attacking_choice.category == MoveCategory::Special {
                    attacking_choice.base_power /= 1.5;
                }
            }
        }
        _ => {}
    }
}

pub fn item_modify_attack_being_used(
    state: &State,
    attacking_choice: &mut Choice,
    attacking_side_ref: &SideReference,
) {
    let (attacking_side, defending_side) = state.get_both_sides_immutable(attacking_side_ref);
    match attacking_side.get_active_immutable().item {
        Items::WELLSPRINGMASK => match attacking_side.get_active_immutable().id {
            PokemonName::OGERPONWELLSPRING | PokemonName::OGERPONWELLSPRINGTERA => {
                attacking_choice.base_power *= 1.2;
            }
            _ => {}
        },
        Items::HEARTHFLAMEMASK => match attacking_side.get_active_immutable().id {
            PokemonName::OGERPONHEARTHFLAME | PokemonName::OGERPONHEARTHFLAMETERA => {
                attacking_choice.base_power *= 1.2;
            }
            _ => {}
        },
        Items::CORNERSTONEMASK => match attacking_side.get_active_immutable().id {
            PokemonName::OGERPONCORNERSTONE | PokemonName::OGERPONCORNERSTONETERA => {
                attacking_choice.base_power *= 1.2;
            }
            _ => {}
        },
        #[cfg(feature = "gen3")]
        Items::BLACKBELT => {
            if attacking_choice.move_type == PokemonType::FIGHTING {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::BLACKBELT => {
            if attacking_choice.move_type == PokemonType::FIGHTING {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::BLACKGLASSES => {
            if attacking_choice.move_type == PokemonType::DARK {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::BLACKGLASSES => {
            if attacking_choice.move_type == PokemonType::DARK {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::CHARCOAL => {
            if attacking_choice.move_type == PokemonType::FIRE {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::CHARCOAL => {
            if attacking_choice.move_type == PokemonType::FIRE {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::CHOICEBAND => {
            if attacking_choice.category == MoveCategory::Physical {
                attacking_choice.base_power *= 1.5;
            }
        }
        Items::CHOICESPECS => {
            if attacking_choice.category == MoveCategory::Special {
                attacking_choice.base_power *= 1.5;
            }
        }
        #[cfg(feature = "gen3")]
        Items::DRAGONFANG | Items::DRAGONSCALE => {
            if attacking_choice.move_type == PokemonType::DRAGON {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::DRAGONFANG | Items::DRAGONSCALE => {
            if attacking_choice.move_type == PokemonType::DRAGON {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::EXPERTBELT => {
            if type_effectiveness_modifier(
                &attacking_choice.move_type,
                &defending_side.get_active_immutable(),
            ) > 1.0
            {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::FAIRYFEATHER => {
            if attacking_choice.move_type == PokemonType::FAIRY {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::LIFEORB => {
            if attacking_choice.category != MoveCategory::Status {
                attacking_choice.base_power *= 1.3;
                if attacking_side.get_active_immutable().ability != Abilities::MAGICGUARD {
                    attacking_choice.add_or_create_secondaries(Secondary {
                        chance: 100.0,
                        effect: Effect::Heal(-0.1),
                        target: MoveTarget::User,
                    });
                }
            }
        }
        #[cfg(feature = "gen3")]
        Items::METALCOAT => {
            if attacking_choice.move_type == PokemonType::STEEL {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::METALCOAT => {
            if attacking_choice.move_type == PokemonType::STEEL {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::MUSCLEBAND => {
            if attacking_choice.category == MoveCategory::Physical {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(feature = "gen3")]
        Items::MYSTICWATER => {
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::MYSTICWATER => {
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::NEVERMELTICE => {
            if attacking_choice.move_type == PokemonType::ICE {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::NEVERMELTICE => {
            if attacking_choice.move_type == PokemonType::ICE {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::PINKBOW | Items::POLKADOTBOW => {
            if attacking_choice.move_type == PokemonType::NORMAL {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::ODDINCENSE => {
            if attacking_choice.move_type == PokemonType::PSYCHIC {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::POISONBARB => {
            if attacking_choice.move_type == PokemonType::POISON {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::POISONBARB => {
            if attacking_choice.move_type == PokemonType::POISON {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::PUNCHINGGLOVE => {
            if attacking_choice.flags.punch {
                attacking_choice.base_power *= 1.1;
                attacking_choice.flags.contact = false
            }
        }
        Items::SEAINCENSE => {
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::SHARPBEAK => {
            if attacking_choice.move_type == PokemonType::FLYING {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::SHARPBEAK => {
            if attacking_choice.move_type == PokemonType::FLYING {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SHELLBELL => {
            attacking_choice.drain = Some(0.125);
        }
        Items::SILKSCARF => {
            if attacking_choice.move_type == PokemonType::NORMAL {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::SILVERPOWDER => {
            if attacking_choice.move_type == PokemonType::BUG {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::SILVERPOWDER => {
            if attacking_choice.move_type == PokemonType::BUG {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::SOFTSAND => {
            if attacking_choice.move_type == PokemonType::GROUND {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::SOFTSAND => {
            if attacking_choice.move_type == PokemonType::GROUND {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SOULDEW => {
            if attacking_side.get_active_immutable().id == PokemonName::LATIOS
                || attacking_side.get_active_immutable().id == PokemonName::LATIAS
            {
                #[cfg(any(feature = "gen3", feature = "gen4", feature = "gen5", feature = "gen6"))]
                if attacking_choice.category == MoveCategory::Special {
                    attacking_choice.base_power *= 1.5;
                }

                #[cfg(not(any(
                    feature = "gen3",
                    feature = "gen4",
                    feature = "gen5",
                    feature = "gen6"
                )))]
                if attacking_choice.move_type == PokemonType::DRAGON
                    || attacking_choice.move_type == PokemonType::PSYCHIC
                {
                    attacking_choice.base_power *= 1.2;
                }
            }
        }
        Items::GRISEOUSORB | Items::GRISEOUSCORE => {
            if [PokemonName::GIRATINAORIGIN, PokemonName::GIRATINA]
                .contains(&attacking_side.get_active_immutable().id)
            {
                if attacking_choice.move_type == PokemonType::DRAGON
                    || attacking_choice.move_type == PokemonType::GHOST
                {
                    attacking_choice.base_power *= 1.2;
                }
            }
        }
        Items::LUSTROUSORB | Items::LUSTROUSGLOBE => {
            if [PokemonName::PALKIAORIGIN, PokemonName::PALKIA]
                .contains(&attacking_side.get_active_immutable().id)
            {
                if attacking_choice.move_type == PokemonType::DRAGON
                    || attacking_choice.move_type == PokemonType::WATER
                {
                    attacking_choice.base_power *= 1.2;
                }
            }
        }
        Items::ADAMANTORB | Items::ADAMANTCRYSTAL => {
            if [PokemonName::DIALGAORIGIN, PokemonName::DIALGA]
                .contains(&attacking_side.get_active_immutable().id)
            {
                if attacking_choice.move_type == PokemonType::DRAGON
                    || attacking_choice.move_type == PokemonType::STEEL
                {
                    attacking_choice.base_power *= 1.2;
                }
            }
        }
        Items::THROATSPRAY => {
            if attacking_choice.flags.sound {
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::Boost(StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    }),
                    target: MoveTarget::User,
                });
                attacking_choice.add_or_create_secondaries(Secondary {
                    chance: 100.0,
                    effect: Effect::RemoveItem,
                    target: MoveTarget::User,
                });
            }
        }
        Items::THICKCLUB => match attacking_side.get_active_immutable().id {
            PokemonName::CUBONE
            | PokemonName::MAROWAK
            | PokemonName::MAROWAKALOLA
            | PokemonName::MAROWAKALOLATOTEM => {
                attacking_choice.base_power *= 2.0;
            }
            _ => {}
        },
        #[cfg(feature = "gen3")]
        Items::TWISTEDSPOON => {
            if attacking_choice.move_type == PokemonType::PSYCHIC {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::TWISTEDSPOON => {
            if attacking_choice.move_type == PokemonType::PSYCHIC {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::HARDSTONE => {
            if attacking_choice.move_type == PokemonType::ROCK {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::HARDSTONE => {
            if attacking_choice.move_type == PokemonType::ROCK {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::WAVEINCENSE => {
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.base_power *= 1.2;
            }
        }
        #[cfg(feature = "gen3")]
        Items::MAGNET => {
            if attacking_choice.move_type == PokemonType::ELECTRIC {
                attacking_choice.base_power *= 1.1;
            }
        }
        #[cfg(any(
            feature = "gen4",
            feature = "gen5",
            feature = "gen6",
            feature = "gen7",
            feature = "gen8",
            feature = "gen9"
        ))]
        Items::MAGNET => {
            if attacking_choice.move_type == PokemonType::ELECTRIC {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::WISEGLASSES => {
            if attacking_choice.category == MoveCategory::Special {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::FISTPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::FIGHTING;
            }
            if attacking_choice.move_type == PokemonType::FIGHTING {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SKYPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::FLYING;
            }
            if attacking_choice.move_type == PokemonType::FLYING {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::TOXICPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::POISON;
            }
            if attacking_choice.move_type == PokemonType::POISON {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::EARTHPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::GROUND;
            }
            if attacking_choice.move_type == PokemonType::GROUND {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::STONEPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::ROCK;
            }
            if attacking_choice.move_type == PokemonType::ROCK {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::INSECTPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::BUG;
            }
            if attacking_choice.move_type == PokemonType::BUG {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SPOOKYPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::GHOST;
            }
            if attacking_choice.move_type == PokemonType::GHOST {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::IRONPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::STEEL;
            }
            if attacking_choice.move_type == PokemonType::STEEL {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::FLAMEPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::FIRE;
            }
            if attacking_choice.move_type == PokemonType::FIRE {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::SPLASHPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::WATER;
            }
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::MEADOWPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::GRASS;
            }
            if attacking_choice.move_type == PokemonType::GRASS {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::ZAPPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::ELECTRIC;
            }
            if attacking_choice.move_type == PokemonType::ELECTRIC {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::MINDPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::PSYCHIC;
            }
            if attacking_choice.move_type == PokemonType::PSYCHIC {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::ICICLEPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::ICE;
            }
            if attacking_choice.move_type == PokemonType::ICE {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::DRACOPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::DRAGON;
            }
            if attacking_choice.move_type == PokemonType::DRAGON {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::DREADPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::DARK;
            }
            if attacking_choice.move_type == PokemonType::DARK {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::PIXIEPLATE => {
            if attacking_choice.move_id == Choices::JUDGMENT {
                attacking_choice.move_type = PokemonType::FAIRY;
            }
            if attacking_choice.move_type == PokemonType::FAIRY {
                attacking_choice.base_power *= 1.2;
            }
        }
        Items::LIGHTBALL => {
            if attacking_side
                .get_active_immutable()
                .id
                .is_pikachu_variant()
            {
                attacking_choice.base_power *= 2.0;
            }
        }
        _ => {}
    }
}
