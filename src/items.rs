use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::choices::{Choice, MoveCategory};
use crate::instruction::Instruction;
use crate::state::PokemonType;
use crate::state::SideReference;
use crate::state::State;

type ModifyAttackBeingUsed = fn(&State, &mut Choice, &SideReference);
type ModifyAttackAgainst = fn(&State, &mut Choice, &SideReference);
type ItemOnSwitchInFn = fn(&State, &SideReference) -> Vec<Instruction>;

lazy_static! {
    pub static ref ITEMS: HashMap<String, Item> = {
        let mut items: HashMap<String, Item> = HashMap::new();

        items.insert(
            "choiceband".to_string(),
            Item {
                modify_attack_being_used: Some(
                    |_state, attacking_choice: &mut Choice, _side_ref| {
                        if attacking_choice.category == MoveCategory::Physical {
                            attacking_choice.base_power *= 1.3;
                        }
                    },
                ),
                ..Default::default()
            },
        );
        // items.insert(
        //     "rockyhelmet".to_string(),
        //     Item {
        //         modify_attack_against: Some(
        //             |_state, attacking_choice: &mut Choice, _side_ref| {
        //                 if attacking_choice.flags.contact {
        //                     if let Some(secondaries) = attacking_choice.secondaries {
        //
        //                     }
        //                 }
        //             },
        //         ),
        //         ..Default::default()
        //     },
        // );
        items.insert(
            "airballoon".to_string(),
            Item {
                modify_attack_against: Some(|_state, attacking_choice: &mut Choice, _side_ref| {
                    if attacking_choice.move_type == PokemonType::Ground
                        && attacking_choice.move_id != "thousandarrows"
                    {
                        attacking_choice.base_power = 0.0;
                    }
                }),
                ..Default::default()
            },
        );

        items
    };
}

pub struct Item {
    pub modify_attack_being_used: Option<ModifyAttackBeingUsed>,
    pub modify_attack_against: Option<ModifyAttackAgainst>,
    pub on_switch_in: Option<ItemOnSwitchInFn>,
}

impl Default for Item {
    fn default() -> Item {
        return Item {
            modify_attack_being_used: None,
            modify_attack_against: None,
            on_switch_in: None,
        };
    }
}
