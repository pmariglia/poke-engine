use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::choices::{Choice, MoveTarget};
use crate::instruction::{ChangeType, Instruction};
use crate::state::PokemonType;
use crate::state::SideReference;
use crate::state::State;

type ModifyAttackBeingUsed = fn(&State, &mut Choice, &Choice, &SideReference);
type ModifyAttackAgainst = fn(&State, &mut Choice, &Choice, &SideReference);
type AbilityBeforeMove = fn(&State, &Choice, &SideReference) -> Vec<Instruction>;

lazy_static! {
    pub static ref ABILITIES: HashMap<String, Ability> = {
        let mut abilities: HashMap<String, Ability> = HashMap::new();

        abilities.insert(
            "ripen".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "tangledfeet".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "dragonsmaw".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "clearbody".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "galvanize".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "vitalspirit".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "aerilate".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "defiant".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "cutecharm".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "neuroforce".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "soundproof".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "rkssystem".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "poisonpoint".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "stakeout".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "unnerve".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "rockhead".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "aurabreak".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "mimicry".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "bulletproof".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "powerofalchemy".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "technician".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "multiscale".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "arenatrap".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "battlebond".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "disguise".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "earlybird".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "lightningrod".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "magician".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "refrigerate".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "friendguard".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "noability".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "gulpmissile".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "powerconstruct".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "forecast".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "prankster".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "protean".to_string(),
            Ability {
                before_move: Some(|state: &State, choice: &Choice, side_ref: &SideReference| {
                    let active_pkmn = state.get_side_immutable(side_ref).get_active_immutable();
                    if !active_pkmn.has_type(&choice.move_type) {
                        return vec![Instruction::ChangeType(ChangeType {
                            side_ref: *side_ref,
                            new_types: (choice.move_type, PokemonType::Typeless),
                            old_types: active_pkmn.types,
                        })];
                    }
                    return vec![];
                }),
                ..Default::default()
            },
        );

        abilities.insert(
            "asoneglastrier".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "shadowtag".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "skilllink".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "intrepidsword".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "soulheart".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "swiftswim".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "superluck".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "insomnia".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "dancer".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "steamengine".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "angerpoint".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "contrary".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "magmaarmor".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "hungerswitch".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "receiver".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "zenmode".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "emergencyexit".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "illusion".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "weakarmor".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "drought".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "innardsout".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "shieldsdown".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "adaptability".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "corrosion".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "longreach".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "purepower".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "tintedlens".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "queenlymajesty".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "desolateland".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "moxie".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sapsipper".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "slushrush".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "bigpecks".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "stall".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "whitesmoke".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "flareboost".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "shadowshield".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "liquidvoice".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "mistysurge".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "multitype".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "noguard".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "torrent".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "deltastream".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "klutz".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "libero".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "serenegrace".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "cursedbody".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "unaware".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "lightmetal".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "marvelscale".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "telepathy".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "quickdraw".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "hypercutter".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "symbiosis".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "plus".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "mirrorarmor".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "pastelveil".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "toughclaws".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "effectspore".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "mummy".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "baddreams".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "magicguard".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sandstream".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "powerspot".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "flamebody".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "reckless".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "pressure".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "gooey".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "immunity".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "leafguard".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "hugepower".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "solarpower".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "schooling".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "motordrive".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "anticipation".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "merciless".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "trace".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "naturalcure".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "harvest".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "suctioncups".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "iceface".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "roughskin".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "wonderguard".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "waterveil".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "fairyaura".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sandspit".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "intimidate".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "dauntlessshield".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "aromaveil".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "airlock".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "normalize".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "darkaura".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "victorystar".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "grassysurge".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sturdy".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "pickpocket".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "electricsurge".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "runaway".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "oblivious".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "surgesurfer".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "levitate".to_string(),
            Ability {
                modify_attack_against: Some(
                    |_state, attacker_choice: &mut Choice, _defender_choice, _attacking_side| {
                        if attacker_choice.move_type == PokemonType::Ground
                            && attacker_choice.target == MoveTarget::Opponent
                            && attacker_choice.move_id != "thousandarrows"
                        {
                            attacker_choice.base_power = 0.0;
                        }
                    },
                ),
                ..Default::default()
            },
        );

        abilities.insert(
            "asonespectrier".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "pickup".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "icebody".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "curiousmedicine".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "flowerveil".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "static".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "wonderskin".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "overgrow".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "propellertail".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "thickfat".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "gluttony".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "keeneye".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "mountaineer".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "flashfire".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "compoundeyes".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "steelworker".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "comatose".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "ballfetch".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "dazzling".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "download".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "transistor".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "moldbreaker".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "liquidooze".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "poisonheal".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "prismarmor".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sniper".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "stench".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "competitive".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "swarm".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "stalwart".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "illuminate".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "turboblaze".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "gorillatactics".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "speedboost".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "heatproof".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "snowcloak".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "teravolt".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "chillingneigh".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "shielddust".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "rivalry".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "primordialsea".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "screencleaner".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "magnetpull".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "honeygather".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "cottondown".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "grasspelt".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "battlearmor".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "beastboost".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "berserk".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "minus".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "raindish".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "synchronize".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "filter".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "truant".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "furcoat".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "fullmetalbody".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "regenerator".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "forewarn".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "ironbarbs".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "stamina".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sandrush".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "colorchange".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "blaze".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "analytic".to_string(),
            Ability {
                modify_attack_being_used: Some(|_, attacking_choice, _, _| {
                    if !attacking_choice.first_move {
                        attacking_choice.base_power *= 1.3;
                    }
                }),
                ..Default::default()
            },
        );

        abilities.insert(
            "tanglinghair".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "cloudnine".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "steelyspirit".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "quickfeet".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "magicbounce".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "megalauncher".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "heavymetal".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "stormdrain".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "pixilate".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "watercompaction".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "justified".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "slowstart".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "snowwarning".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "flowergift".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "shedskin".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "wimpout".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "icescales".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "infiltrator".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "limber".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "psychicsurge".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "defeatist".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "waterabsorb".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "imposter".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "dryskin".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "fluffy".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "unburden".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "cheekpouch".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "stancechange".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "moody".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "punkrock".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sandveil".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "parentalbond".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "strongjaw".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "battery".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "healer".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "steadfast".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "damp".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "perishbody".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "triage".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sheerforce".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "owntempo".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "frisk".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "voltabsorb".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "galewings".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "aftermath".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "stickyhold".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "grimneigh".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "ironfist".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "rebound".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "unseenfist".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "solidrock".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "hustle".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "hydration".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "scrappy".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "overcoat".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "neutralizinggas".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sweetveil".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "drizzle".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "innerfocus".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "poisontouch".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "wanderingspirit".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "guts".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "shellarmor".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "rattled".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "waterbubble".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "sandforce".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "toxicboost".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "persistent".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "chlorophyll".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities.insert(
            "simple".to_string(),
            Ability {
                ..Default::default()
            },
        );

        abilities
    };
}

pub struct Ability {
    pub modify_attack_being_used: Option<ModifyAttackBeingUsed>,
    pub modify_attack_against: Option<ModifyAttackAgainst>,
    pub before_move: Option<AbilityBeforeMove>,
}

impl Default for Ability {
    fn default() -> Ability {
        return Ability {
            modify_attack_being_used: None,
            modify_attack_against: None,
            before_move: None,
        };
    }
}
