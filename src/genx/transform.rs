//! Reversible Transform/Imposter state, including copied PP and switch-out restoration.
use super::state::PokemonVolatileStatus;
use crate::choices::{Choices, MOVES};
use crate::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, Instruction, RemoveVolatileStatusInstruction,
    StateInstructions, TransformInstruction,
};
use crate::state::{PokemonBoostableStat, PokemonMoveIndex, SideReference, State};
use std::str::FromStr;
use std::sync::Arc;

pub fn transform(state: &mut State, side_ref: SideReference, instructions: &mut StateInstructions) {
    let own = state.get_side_immutable(&side_ref);
    let foe = state.get_side_immutable(&side_ref.get_other_side());
    let actor = own.get_active_immutable();
    let target = foe.get_active_immutable();
    if actor.hp <= 0
        || target.hp <= 0
        || actor.transform_original.is_some()
        || target.transform_original.is_some()
        || foe
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SUBSTITUTE)
    {
        return;
    }
    let previous = Arc::new(actor.clone());
    let mut next = actor.clone();
    next.copy_transform_attributes(target);
    next.transform_original = Some(previous.clone());
    for index in [
        PokemonMoveIndex::M0,
        PokemonMoveIndex::M1,
        PokemonMoveIndex::M2,
        PokemonMoveIndex::M3,
    ] {
        let mv = &mut next.moves[&index];
        if mv.id.to_string().starts_with("HIDDENPOWER") {
            let id = Choices::from_str(&format!(
                "HIDDENPOWER{}{}",
                actor.hidden_power_type,
                if cfg!(any(feature = "gen4", feature = "gen5")) {
                    70
                } else {
                    60
                }
            ))
            .unwrap();
            mv.id = id;
            mv.choice = MOVES.get(&id).unwrap().clone();
        }
        mv.pp = match mv.id {
            Choices::NONE => 0,
            Choices::SKETCH => 1,
            _ => 5,
        };
        mv.disabled = mv.id == Choices::NONE;
    }
    let changes = [
        (
            PokemonBoostableStat::Attack,
            foe.attack_boost - own.attack_boost,
        ),
        (
            PokemonBoostableStat::Defense,
            foe.defense_boost - own.defense_boost,
        ),
        (
            PokemonBoostableStat::SpecialAttack,
            foe.special_attack_boost - own.special_attack_boost,
        ),
        (
            PokemonBoostableStat::SpecialDefense,
            foe.special_defense_boost - own.special_defense_boost,
        ),
        (
            PokemonBoostableStat::Speed,
            foe.speed_boost - own.speed_boost,
        ),
        (
            PokemonBoostableStat::Accuracy,
            foe.accuracy_boost - own.accuracy_boost,
        ),
        (
            PokemonBoostableStat::Evasion,
            foe.evasion_boost - own.evasion_boost,
        ),
    ];
    let critical_flags = [
        PokemonVolatileStatus::FOCUSENERGY,
        PokemonVolatileStatus::LASERFOCUS,
    ]
    .map(|v| {
        (
            v,
            own.volatile_statuses.contains(&v),
            foe.volatile_statuses.contains(&v),
        )
    });
    let instruction = Instruction::Transform(Arc::new(TransformInstruction {
        side_ref,
        pokemon_index: own.active_index,
        previous,
        next: Arc::new(next),
    }));
    state.apply_one_instruction(&instruction);
    instructions.instruction_list.push(instruction);
    for (stat, amount) in changes {
        if amount != 0 {
            let instruction = Instruction::Boost(BoostInstruction {
                side_ref,
                stat,
                amount,
            });
            state.apply_one_instruction(&instruction);
            instructions.instruction_list.push(instruction);
        }
    }
    for (volatile_status, own_has, foe_has) in critical_flags {
        if own_has == foe_has {
            continue;
        }
        let instruction = if foe_has {
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref,
                volatile_status,
            })
        } else {
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref,
                volatile_status,
            })
        };
        state.apply_one_instruction(&instruction);
        instructions.instruction_list.push(instruction);
    }
}

pub fn restore(state: &mut State, side_ref: SideReference, instructions: &mut StateInstructions) {
    let side = state.get_side_immutable(&side_ref);
    let actor = side.get_active_immutable();
    if let Some(original) = &actor.transform_original {
        let instruction = Instruction::Transform(Arc::new(TransformInstruction {
            side_ref,
            pokemon_index: side.active_index,
            previous: Arc::new(actor.clone()),
            next: original.clone(),
        }));
        state.apply_one_instruction(&instruction);
        instructions.instruction_list.push(instruction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::abilities::Abilities;
    use crate::engine::items::Items;
    use crate::pokemon::PokemonName;
    use crate::state::{PokemonStatus, PokemonType};

    #[test]
    fn restoration_keeps_later_hp_status_and_item_changes() {
        let mut state = State::default();
        state.side_one.get_active().id = PokemonName::DITTO;
        state.side_one.get_active().ability = Abilities::IMPOSTER;
        state
            .side_one
            .get_active()
            .replace_move(PokemonMoveIndex::M0, Choices::TRANSFORM);
        state.side_two.get_active().id = PokemonName::JUMPLUFF;
        state.side_two.get_active().types = (PokemonType::GRASS, PokemonType::FLYING);
        state
            .side_two
            .get_active()
            .replace_move(PokemonMoveIndex::M0, Choices::SYNTHESIS);
        let mut instructions = StateInstructions::default();
        transform(&mut state, SideReference::SideOne, &mut instructions);
        state.side_one.get_active().hp = 17;
        state.side_one.get_active().status = PokemonStatus::BURN;
        state.side_one.get_active().item = Items::NONE;
        let before_restore = state.serialize();
        let mut restoration = StateInstructions::default();
        restore(&mut state, SideReference::SideOne, &mut restoration);
        let ditto = state.side_one.get_active_immutable();
        assert_eq!(ditto.id, PokemonName::DITTO);
        assert_eq!(ditto.moves[&PokemonMoveIndex::M0].id, Choices::TRANSFORM);
        assert_eq!(ditto.hp, 17);
        assert_eq!(ditto.status, PokemonStatus::BURN);
        assert_eq!(ditto.item, Items::NONE);
        state.reverse_instructions(&restoration.instruction_list);
        assert_eq!(state.serialize(), before_restore);
    }

    #[test]
    fn fainted_target_cannot_be_copied_and_generates_no_instruction() {
        let mut state = State::default();
        state.side_two.get_active().hp = 0;
        let before = state.serialize();
        let mut instructions = StateInstructions::default();
        transform(&mut state, SideReference::SideOne, &mut instructions);
        assert!(instructions.instruction_list.is_empty());
        assert_eq!(state.serialize(), before);
    }
}
