#![allow(unused_variables)]
use super::generate_instructions::{add_remove_status_instructions, get_boost_instruction};
use crate::choices::{Choice, Choices, MoveCategory};
use crate::define_enum_with_from_str;
use crate::instruction::{
    ChangeItemInstruction, ChangeStatusInstruction, DisableMoveInstruction, HealInstruction,
    Instruction, StateInstructions,
};
use crate::pokemon::PokemonName;
use crate::state::{
    Pokemon, PokemonBoostableStat, PokemonStatus, PokemonType, Side, SideReference, State,
};
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
        CHESTOBERRY,
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
        LOADEDDICE,
        RUSTEDSWORD,
        RUSTEDSHIELD,
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

fn chesto_berry(
    side_ref: &SideReference,
    attacking_side: &mut Side,
    instructions: &mut StateInstructions,
) {
    let active_index = attacking_side.active_index;
    let active_pkmn = attacking_side.get_active();
    instructions
        .instruction_list
        .push(Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: *side_ref,
            current_item: Items::CHESTOBERRY,
            new_item: Items::NONE,
        }));
    active_pkmn.item = Items::NONE;
    add_remove_status_instructions(instructions, active_index, *side_ref, attacking_side);
}

fn boost_berry(
    side_ref: &SideReference,
    state: &mut State,
    stat: PokemonBoostableStat,
    instructions: &mut StateInstructions,
) {
    if let Some(ins) = get_boost_instruction(
        &state.get_side_immutable(side_ref),
        &stat,
        &1,
        side_ref,
        side_ref,
    ) {
        state.apply_one_instruction(&ins);
        instructions.instruction_list.push(ins);
    }
    let attacker = state.get_side(side_ref).get_active();
    instructions
        .instruction_list
        .push(Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: *side_ref,
            current_item: attacker.item,
            new_item: Items::NONE,
        }));
    attacker.item = Items::NONE;
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
    match active_pkmn.item {
        Items::CHOICEBAND => {
            let ins = get_choice_move_disable_instructions(active_pkmn, side_ref, &choice.move_id);
            for i in ins {
                state.apply_one_instruction(&i);
                instructions.instruction_list.push(i);
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
        Items::PETAYABERRY if active_pkmn.hp <= active_pkmn.maxhp / 4 => boost_berry(
            side_ref,
            state,
            PokemonBoostableStat::SpecialAttack,
            instructions,
        ),
        Items::LIECHIBERRY if active_pkmn.hp <= active_pkmn.maxhp / 4 => {
            boost_berry(side_ref, state, PokemonBoostableStat::Attack, instructions)
        }
        Items::SALACBERRY if active_pkmn.hp <= active_pkmn.maxhp / 4 => {
            boost_berry(side_ref, state, PokemonBoostableStat::Speed, instructions)
        }
        Items::LUMBERRY if active_pkmn.status != PokemonStatus::NONE => {
            lum_berry(side_ref, attacking_side, instructions)
        }
        Items::SITRUSBERRY if active_pkmn.hp <= active_pkmn.maxhp / 2 => {
            sitrus_berry(side_ref, attacking_side, instructions)
        }
        Items::CHESTOBERRY if active_pkmn.status == PokemonStatus::SLEEP => {
            chesto_berry(side_ref, attacking_side, instructions)
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
        Items::METALPOWDER if attacking_side.get_active_immutable().id == PokemonName::DITTO => {
            attacking_choice.base_power /= 1.5;
        }
        Items::SOULDEW => {
            if defending_side.get_active_immutable().id == PokemonName::LATIOS
                || defending_side.get_active_immutable().id == PokemonName::LATIAS
            {
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
        Items::BLACKBELT => {
            if attacking_choice.move_type == PokemonType::FIGHTING {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::BLACKGLASSES => {
            if attacking_choice.move_type == PokemonType::DARK {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::CHARCOAL => {
            if attacking_choice.move_type == PokemonType::FIRE {
                attacking_choice.base_power *= 1.1;
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
        Items::DRAGONFANG | Items::DRAGONSCALE => {
            if attacking_choice.move_type == PokemonType::DRAGON {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::METALCOAT => {
            if attacking_choice.move_type == PokemonType::STEEL {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::MYSTICWATER => {
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::NEVERMELTICE => {
            if attacking_choice.move_type == PokemonType::ICE {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::POISONBARB => {
            if attacking_choice.move_type == PokemonType::POISON {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SEAINCENSE => {
            if attacking_choice.move_type == PokemonType::WATER {
                attacking_choice.base_power *= 1.05;
            }
        }
        Items::SHARPBEAK => {
            if attacking_choice.move_type == PokemonType::FLYING {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SHELLBELL => {
            attacking_choice.drain = Some(0.125);
        }
        Items::SILKSCARF => {
            if attacking_choice.move_type == PokemonType::NORMAL {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SILVERPOWDER => {
            if attacking_choice.move_type == PokemonType::BUG {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SOFTSAND => {
            if attacking_choice.move_type == PokemonType::GROUND {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SPELLTAG => {
            if attacking_choice.move_type == PokemonType::GHOST {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::MIRACLESEED => {
            if attacking_choice.move_type == PokemonType::GRASS {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::SOULDEW => {
            if attacking_side.get_active_immutable().id == PokemonName::LATIOS
                || attacking_side.get_active_immutable().id == PokemonName::LATIAS
            {
                if attacking_choice.category == MoveCategory::Special {
                    attacking_choice.base_power *= 1.5;
                }
            }
        }
        Items::THICKCLUB => match attacking_side.get_active_immutable().id {
            PokemonName::CUBONE | PokemonName::MAROWAK => {
                attacking_choice.base_power *= 2.0;
            }
            _ => {}
        },
        Items::TWISTEDSPOON => {
            if attacking_choice.move_type == PokemonType::PSYCHIC {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::HARDSTONE => {
            if attacking_choice.move_type == PokemonType::ROCK {
                attacking_choice.base_power *= 1.1;
            }
        }
        Items::MAGNET => {
            if attacking_choice.move_type == PokemonType::ELECTRIC {
                attacking_choice.base_power *= 1.1;
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
