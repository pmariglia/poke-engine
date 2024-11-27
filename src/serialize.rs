use crate::abilities::Abilities;
use crate::choices::{Choices, MOVES};
use crate::items::Items;
use crate::pokemon::PokemonName;
use crate::state::{
    DamageDealt, LastUsedMove, Move, Pokemon, PokemonIndex, PokemonMoveIndex, PokemonMoves,
    PokemonStatus, PokemonType, PokemonVolatileStatus, Side, SideConditions, SidePokemon, State,
    StateTerrain, StateTrickRoom, StateWeather, Terrain, Weather,
};
use std::collections::HashSet;
use std::str::FromStr;

impl Move {
    pub fn serialize(&self) -> String {
        format!("{:?};{};{}", self.id, self.disabled, self.pp)
    }
    pub fn deserialize(serialized: &str) -> Move {
        let split: Vec<&str> = serialized.split(";").collect();
        Move {
            id: Choices::from_str(split[0]).unwrap(),
            disabled: split[1].parse::<bool>().unwrap(),
            pp: split[2].parse::<i8>().unwrap(),
            choice: MOVES
                .get(&Choices::from_str(split[0]).unwrap())
                .unwrap()
                .to_owned(),
        }
    }
}

impl Pokemon {
    pub fn serialize(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.id,
            self.level,
            self.types.0.to_string(),
            self.types.1.to_string(),
            self.hp,
            self.maxhp,
            self.ability.to_string(),
            self.item.to_string(),
            self.attack,
            self.defense,
            self.special_attack,
            self.special_defense,
            self.speed,
            self.status.to_string(),
            self.rest_turns,
            self.sleep_turns,
            self.weight_kg,
            self.moves.m0.serialize(),
            self.moves.m1.serialize(),
            self.moves.m2.serialize(),
            self.moves.m3.serialize(),
            self.moves.m4.serialize(),
            self.moves.m5.serialize(),
            self.terastallized,
            self.tera_type.to_string(),
        )
    }

    pub fn deserialize(serialized: &str) -> Pokemon {
        let split: Vec<&str> = serialized.split(",").collect();
        Pokemon {
            id: PokemonName::from_str(split[0]).unwrap(),
            level: split[1].parse::<i8>().unwrap(),
            types: (
                PokemonType::from_str(split[2]).unwrap(),
                PokemonType::from_str(split[3]).unwrap(),
            ),
            hp: split[4].parse::<i16>().unwrap(),
            maxhp: split[5].parse::<i16>().unwrap(),
            ability: Abilities::from_str(split[6]).unwrap(),
            item: Items::from_str(split[7]).unwrap(),
            attack: split[8].parse::<i16>().unwrap(),
            defense: split[9].parse::<i16>().unwrap(),
            special_attack: split[10].parse::<i16>().unwrap(),
            special_defense: split[11].parse::<i16>().unwrap(),
            speed: split[12].parse::<i16>().unwrap(),
            status: PokemonStatus::from_str(split[13]).unwrap(),
            rest_turns: split[14].parse::<i8>().unwrap(),
            sleep_turns: split[15].parse::<i8>().unwrap(),
            weight_kg: split[16].parse::<f32>().unwrap(),
            moves: PokemonMoves {
                m0: Move::deserialize(split[17]),
                m1: Move::deserialize(split[18]),
                m2: Move::deserialize(split[19]),
                m3: Move::deserialize(split[20]),
                m4: Move::deserialize(split[21]),
                m5: Move::deserialize(split[22]),
            },
            terastallized: split[23].parse::<bool>().unwrap(),
            tera_type: PokemonType::from_str(split[24]).unwrap(),
        }
    }
}

impl LastUsedMove {
    pub fn serialize(&self) -> String {
        match self {
            LastUsedMove::Move(move_index) => format!("move:{}", move_index.serialize()),
            LastUsedMove::Switch(pkmn_index) => format!("switch:{}", pkmn_index.serialize()),
            LastUsedMove::None => "move:none".to_string(),
        }
    }
    pub fn deserialize(serialized: &str) -> LastUsedMove {
        let split: Vec<&str> = serialized.split(":").collect();
        match split[0] {
            "move" => {
                if split[1] == "none" {
                    LastUsedMove::None
                } else {
                    LastUsedMove::Move(PokemonMoveIndex::deserialize(split[1]))
                }
            }
            "switch" => LastUsedMove::Switch(PokemonIndex::deserialize(split[1])),
            _ => panic!("Invalid LastUsedMove: {}", serialized),
        }
    }
}

impl PokemonIndex {
    pub fn serialize(&self) -> String {
        match self {
            PokemonIndex::P0 => "0".to_string(),
            PokemonIndex::P1 => "1".to_string(),
            PokemonIndex::P2 => "2".to_string(),
            PokemonIndex::P3 => "3".to_string(),
            PokemonIndex::P4 => "4".to_string(),
            PokemonIndex::P5 => "5".to_string(),
        }
    }

    pub fn deserialize(serialized: &str) -> PokemonIndex {
        match serialized {
            "0" => PokemonIndex::P0,
            "1" => PokemonIndex::P1,
            "2" => PokemonIndex::P2,
            "3" => PokemonIndex::P3,
            "4" => PokemonIndex::P4,
            "5" => PokemonIndex::P5,
            _ => panic!("Invalid PokemonIndex: {}", serialized),
        }
    }
}

impl PokemonMoveIndex {
    pub fn serialize(&self) -> String {
        match self {
            PokemonMoveIndex::M0 => "0".to_string(),
            PokemonMoveIndex::M1 => "1".to_string(),
            PokemonMoveIndex::M2 => "2".to_string(),
            PokemonMoveIndex::M3 => "3".to_string(),
            PokemonMoveIndex::M4 => "4".to_string(),
            PokemonMoveIndex::M5 => "5".to_string(),
        }
    }

    pub fn deserialize(serialized: &str) -> PokemonMoveIndex {
        match serialized {
            "0" => PokemonMoveIndex::M0,
            "1" => PokemonMoveIndex::M1,
            "2" => PokemonMoveIndex::M2,
            "3" => PokemonMoveIndex::M3,
            "4" => PokemonMoveIndex::M4,
            "5" => PokemonMoveIndex::M5,
            _ => panic!("Invalid PokemonMoveIndex: {}", serialized),
        }
    }
}

impl SideConditions {
    pub fn serialize(&self) -> String {
        format!(
            "{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
            self.aurora_veil,
            self.crafty_shield,
            self.healing_wish,
            self.light_screen,
            self.lucky_chant,
            self.lunar_dance,
            self.mat_block,
            self.mist,
            self.protect,
            self.quick_guard,
            self.reflect,
            self.safeguard,
            self.spikes,
            self.stealth_rock,
            self.sticky_web,
            self.tailwind,
            self.toxic_count,
            self.toxic_spikes,
            self.wide_guard,
        )
    }
    pub fn deserialize(serialized: &str) -> SideConditions {
        let split: Vec<&str> = serialized.split(";").collect();
        SideConditions {
            aurora_veil: split[0].parse::<i8>().unwrap(),
            crafty_shield: split[1].parse::<i8>().unwrap(),
            healing_wish: split[2].parse::<i8>().unwrap(),
            light_screen: split[3].parse::<i8>().unwrap(),
            lucky_chant: split[4].parse::<i8>().unwrap(),
            lunar_dance: split[5].parse::<i8>().unwrap(),
            mat_block: split[6].parse::<i8>().unwrap(),
            mist: split[7].parse::<i8>().unwrap(),
            protect: split[8].parse::<i8>().unwrap(),
            quick_guard: split[9].parse::<i8>().unwrap(),
            reflect: split[10].parse::<i8>().unwrap(),
            safeguard: split[11].parse::<i8>().unwrap(),
            spikes: split[12].parse::<i8>().unwrap(),
            stealth_rock: split[13].parse::<i8>().unwrap(),
            sticky_web: split[14].parse::<i8>().unwrap(),
            tailwind: split[15].parse::<i8>().unwrap(),
            toxic_count: split[16].parse::<i8>().unwrap(),
            toxic_spikes: split[17].parse::<i8>().unwrap(),
            wide_guard: split[18].parse::<i8>().unwrap(),
        }
    }
}

impl Side {
    pub fn serialize(&self) -> String {
        let mut vs_string = String::new();
        for vs in &self.volatile_statuses {
            vs_string.push_str(&vs.to_string());
            vs_string.push_str(":");
        }
        format!(
            "{}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}",
            self.pokemon.p0.serialize(),
            self.pokemon.p1.serialize(),
            self.pokemon.p2.serialize(),
            self.pokemon.p3.serialize(),
            self.pokemon.p4.serialize(),
            self.pokemon.p5.serialize(),
            self.active_index.serialize(),
            self.side_conditions.serialize(),
            vs_string,
            self.substitute_health,
            self.attack_boost,
            self.defense_boost,
            self.special_attack_boost,
            self.special_defense_boost,
            self.speed_boost,
            self.accuracy_boost,
            self.evasion_boost,
            self.wish.0,
            self.wish.1,
            self.future_sight.0,
            self.future_sight.1.serialize(),
            self.force_switch,
            self.switch_out_move_second_saved_move.to_string(),
            self.baton_passing,
            self.force_trapped,
            self.last_used_move.serialize(),
            self.slow_uturn_move,
        )
    }
    pub fn deserialize(serialized: &str) -> Side {
        let split: Vec<&str> = serialized.split("=").collect();

        let mut vs_hashset = HashSet::new();
        if split[8] != "" {
            for item in split[8].split(":") {
                vs_hashset.insert(PokemonVolatileStatus::from_str(item).unwrap());
            }
        }
        Side {
            pokemon: SidePokemon {
                p0: Pokemon::deserialize(split[0]),
                p1: Pokemon::deserialize(split[1]),
                p2: Pokemon::deserialize(split[2]),
                p3: Pokemon::deserialize(split[3]),
                p4: Pokemon::deserialize(split[4]),
                p5: Pokemon::deserialize(split[5]),
            },
            active_index: PokemonIndex::deserialize(split[6]),
            side_conditions: SideConditions::deserialize(split[7]),
            volatile_statuses: vs_hashset,
            substitute_health: split[9].parse::<i16>().unwrap(),
            attack_boost: split[10].parse::<i8>().unwrap(),
            defense_boost: split[11].parse::<i8>().unwrap(),
            special_attack_boost: split[12].parse::<i8>().unwrap(),
            special_defense_boost: split[13].parse::<i8>().unwrap(),
            speed_boost: split[14].parse::<i8>().unwrap(),
            accuracy_boost: split[15].parse::<i8>().unwrap(),
            evasion_boost: split[16].parse::<i8>().unwrap(),
            wish: (
                split[17].parse::<i8>().unwrap(),
                split[18].parse::<i16>().unwrap(),
            ),
            future_sight: (
                split[19].parse::<i8>().unwrap(),
                PokemonIndex::deserialize(split[20]),
            ),
            force_switch: split[21].parse::<bool>().unwrap(),
            switch_out_move_second_saved_move: Choices::from_str(split[22]).unwrap(),
            baton_passing: split[23].parse::<bool>().unwrap(),
            force_trapped: split[24].parse::<bool>().unwrap(),
            last_used_move: LastUsedMove::deserialize(split[25]),
            damage_dealt: DamageDealt::default(),
            slow_uturn_move: split[26].parse::<bool>().unwrap(),
        }
    }
}

impl StateWeather {
    pub fn serialize(&self) -> String {
        format!("{:?};{}", self.weather_type, self.turns_remaining)
    }
    pub fn deserialize(serialized: &str) -> StateWeather {
        let split: Vec<&str> = serialized.split(";").collect();
        StateWeather {
            weather_type: Weather::from_str(split[0]).unwrap(),
            turns_remaining: split[1].parse::<i8>().unwrap(),
        }
    }
}

impl StateTerrain {
    pub fn serialize(&self) -> String {
        format!("{:?};{}", self.terrain_type, self.turns_remaining)
    }
    pub fn deserialize(serialized: &str) -> StateTerrain {
        let split: Vec<&str> = serialized.split(";").collect();
        StateTerrain {
            terrain_type: Terrain::from_str(split[0]).unwrap(),
            turns_remaining: split[1].parse::<i8>().unwrap(),
        }
    }
}

impl StateTrickRoom {
    pub fn serialize(&self) -> String {
        format!("{};{}", self.active, self.turns_remaining)
    }
    pub fn deserialize(serialized: &str) -> StateTrickRoom {
        let split: Vec<&str> = serialized.split(";").collect();
        StateTrickRoom {
            active: split[0].parse::<bool>().unwrap(),
            turns_remaining: split[1].parse::<i8>().unwrap(),
        }
    }
}

impl State {
    pub fn serialize(&self) -> String {
        format!(
            "{}/{}/{}/{}/{}/{}",
            self.side_one.serialize(),
            self.side_two.serialize(),
            self.weather.serialize(),
            self.terrain.serialize(),
            self.trick_room.serialize(),
            self.team_preview
        )
    }

    /// ```
    ///
    /// /*
    /// This doctest does its best to show the format of the serialized state.
    ///
    /// Roughly, the format for a state is:
    ///     side1/side2/weather/terrain/trick_room/team_preview
    ///
    /// Where the format for a side is:
    ///     p0=p1=p2=p3=p4=p5=active_index=side_conditions=wish0=wish1=force_switch=switch_out_move_second_saved_move=baton_passing=force_trapped=last_used_move=slow_uturn_move
    ///
    /// And the format for a pokemon is:
    ///    id,level,type1,type2,hp,maxhp,ability,item,attack,defense,special_attack,special_defense,speed,attack_boost,defense_boost,special_attack_boost,special_defense_boost,speed_boost,accuracy_boost,evasion_boost,status,substitute_health,rest_turns,weight_kg,volatile_statuses,m0,m1,m2,m3,m4,m5    ///
    ///
    /// There's more to it, follow the code below to see a full example of a serialized state.
    /// */
    ///
    /// if cfg!(feature = "gen2") {
    ///    return;
    /// }
    ///
    /// use poke_engine::abilities::Abilities;
    /// use poke_engine::items::Items;
    /// use poke_engine::pokemon::PokemonName;
    /// use poke_engine::state::State;
    ///
    /// let serialized_state = concat!(
    ///
    /// // SIDE 1
    ///
    /// // POKEMON 1
    ///
    /// // name
    /// "alakazam,",
    ///
    /// // level
    /// "100,",
    ///
    /// // type1
    /// "Psychic,",
    ///
    /// // type2
    /// "Typeless,",
    ///
    /// // hp
    /// "251,",
    ///
    /// // maxhp
    /// "251,",
    ///
    /// // ability
    /// "MAGICGUARD,",
    ///
    /// // item
    /// "LIFEORB,",
    ///
    /// // attack,defense,special attack,special defense,speed
    /// // note these are final stats, not base stats
    /// "121,148,353,206,365,",
    ///
    /// // status
    /// "None,",
    ///
    /// // rest_turns
    /// "0,",
    ///
    /// // sleep_turns
    /// "0,",
    ///
    /// // weight_kg
    /// "25.5,",
    ///
    /// // moves 1 through 6 (move_id;disabled;pp)
    /// "PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,NONE;true;32,NONE;true;32,",
    ///
    /// // terastallized
    /// "false,",
    ///
    /// // tera_type
    /// "Normal=",
    ///
    /// // all remaining Pokémon shown in 1 line for brevity
    /// "skarmory,100,Steel,Flying,271,271,STURDY,CUSTAPBERRY,259,316,104,177,262,None,0,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,NONE;true;32,NONE;true;32,false,Normal=",
    /// "tyranitar,100,Rock,Dark,404,404,SANDSTREAM,CHOPLEBERRY,305,256,203,327,159,None,0,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,NONE;true;32,NONE;true;32,false,Normal=",
    /// "mamoswine,100,Ice,Ground,362,362,THICKFAT,NEVERMELTICE,392,196,158,176,241,None,0,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,NONE;true;32,NONE;true;32,false,Normal=",
    /// "jellicent,100,Water,Ghost,404,404,WATERABSORB,AIRBALLOON,140,237,206,246,180,None,0,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,NONE;true;32,NONE;true;32,false,Normal=",
    /// "excadrill,100,Ground,Steel,362,362,SANDFORCE,CHOICESCARF,367,156,122,168,302,None,0,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,NONE;true;32,NONE;true;32,false,Normal=",
    ///
    /// // active-index. This is the index of the active Pokémon in the side's Pokémon array
    /// "0=",
    ///
    /// // side conditions are integers
    /// "0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;=",
    ///
    /// // volatile_statuses (delimited by ":")
    /// "=",
    ///
    /// // substitute_health
    /// "0=",
    ///
    /// // For the active pokemon:
    /// // attack_boost,defense_boost,special attack_boost,special defense_boost,speed_boost,accuracy_boost,evasion_boost
    /// "0=0=0=0=0=0=0=",
    ///
    /// // wish condition is represented by 2 integers, the first is how many wish turns remaining, the second is the amount of HP to heal
    /// "0=",
    /// "0=",
    ///
    /// // future sight is represented by the PokemonIndex of the pokemon that used futuresight, and the number of turns remaining until it hits
    /// "0=",
    /// "0=",
    ///
    /// // a boolean representing if the side is forced to switch
    /// "false=",
    ///
    /// // a 'saved moved' that a pokemon may be waiting to use after the opponent finished their uturn/volt switch/etc.
    /// "NONE=",
    ///
    /// // a b=oolean representing if the side is baton passing
    /// "false=",
    ///
    /// // a boolean representing if the side is force trapped. This is only ever externally provided and never changed by the engine
    /// "false=",
    ///
    /// // last used move is a string that can be either "move:move_name" or "switch:pokemon_index"
    /// "switch:0=",
    ///
    /// // a boolean representing if the side is slow uturning.
    /// // This is only ever set externally. It is used to know if the opposing side has a stored move to use after uturn.
    /// "false/",
    ///
    /// // SIDE 2, all in one line for brevity
    /// "terrakion,100,Rock,Fighting,323,323,JUSTIFIED,FOCUSSASH,357,216,163,217,346,None,0,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,XSCISSOR;false;24,QUICKATTACK;false;48,false,Normal=lucario,100,Fighting,Steel,281,281,JUSTIFIED,LIFEORB,350,176,241,177,279,None,0,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,ICEPUNCH;false;24,AURASPHERE;false;32,false,Normal=breloom,100,Grass,Fighting,262,262,TECHNICIAN,LIFEORB,394,196,141,156,239,None,0,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,DRAINPUNCH;false;16,PROTECT;false;16,false,Normal=keldeo,100,Water,Fighting,323,323,JUSTIFIED,LEFTOVERS,163,216,357,217,346,None,0,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,HIDDENPOWERICE70;false;24,CALMMIND;false;32,false,Normal=conkeldurr,100,Fighting,Typeless,414,414,GUTS,LEFTOVERS,416,226,132,167,126,None,0,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,BULKUP;false;32,PAYBACK;false;16,false,Normal=toxicroak,100,Poison,Fighting,307,307,DRYSKIN,LIFEORB,311,166,189,167,295,None,0,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,POISONJAB;false;32,SUBSTITUTE;false;16,false,Normal=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/",
    ///
    /// // weather is a string representing the weather type and the number of turns remaining
    /// "none;5/",
    ///
    /// // terrain is a string representing the terrain type and the number of turns remaining
    /// "none;5/",
    ///
    /// // trick room is a boolean representing if trick room is active and the number of turns remaining
    /// "false;5/",
    ///
    /// // team preview is a boolean representing if the team preview is active
    /// "false"
    ///
    /// );
    ///
    /// let state = State::deserialize(serialized_state);
    ///
    /// assert_eq!(state.side_one.get_active_immutable().id, PokemonName::ALAKAZAM);
    /// assert_eq!(state.side_one.get_active_immutable().weight_kg, 25.5);
    /// assert_eq!(state.side_one.substitute_health, 0);
    /// assert_eq!(state.side_two.get_active_immutable().id, PokemonName::TERRAKION);
    /// assert_eq!(state.trick_room.active, false);
    /// assert_eq!(state.team_preview, false);
    ///
    /// #[cfg(not(feature = "gen2"))]
    /// {
    ///     assert_eq!(state.side_two.get_active_immutable().item, Items::FOCUSSASH);
    ///     assert_eq!(state.side_two.get_active_immutable().ability, Abilities::JUSTIFIED);
    ///     assert_eq!(state.side_one.get_active_immutable().item, Items::LIFEORB);
    ///     assert_eq!(state.side_one.get_active_immutable().ability, Abilities::MAGICGUARD);
    /// }
    ///
    /// // the same state, but all in one line
    /// let serialized_state = "alakazam,100,Psychic,Typeless,251,251,MAGICGUARD,LIFEORB,121,148,353,206,365,None,0,0,25.5,PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,NONE;true;32,NONE;true;32,false,Normal=skarmory,100,Steel,Flying,271,271,STURDY,CUSTAPBERRY,259,316,104,177,262,None,0,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,NONE;true;32,NONE;true;32,false,Normal=tyranitar,100,Rock,Dark,404,404,SANDSTREAM,CHOPLEBERRY,305,256,203,327,159,None,0,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,NONE;true;32,NONE;true;32,false,Normal=mamoswine,100,Ice,Ground,362,362,THICKFAT,NEVERMELTICE,392,196,158,176,241,None,0,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,NONE;true;32,NONE;true;32,false,Normal=jellicent,100,Water,Ghost,404,404,WATERABSORB,AIRBALLOON,140,237,206,246,180,None,0,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,NONE;true;32,NONE;true;32,false,Normal=excadrill,100,Ground,Steel,362,362,SANDFORCE,CHOICESCARF,367,156,122,168,302,None,0,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,NONE;true;32,NONE;true;32,false,Normal=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/terrakion,100,Rock,Fighting,323,323,JUSTIFIED,FOCUSSASH,357,216,163,217,346,None,0,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,XSCISSOR;false;24,QUICKATTACK;false;48,false,Normal=lucario,100,Fighting,Steel,281,281,JUSTIFIED,LIFEORB,350,176,241,177,279,None,0,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,ICEPUNCH;false;24,AURASPHERE;false;32,false,Normal=breloom,100,Grass,Fighting,262,262,TECHNICIAN,LIFEORB,394,196,141,156,239,None,0,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,DRAINPUNCH;false;16,PROTECT;false;16,false,Normal=keldeo,100,Water,Fighting,323,323,JUSTIFIED,LEFTOVERS,163,216,357,217,346,None,0,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,HIDDENPOWERICE70;false;24,CALMMIND;false;32,false,Normal=conkeldurr,100,Fighting,Typeless,414,414,GUTS,LEFTOVERS,416,226,132,167,126,None,0,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,BULKUP;false;32,PAYBACK;false;16,false,Normal=toxicroak,100,Poison,Fighting,307,307,DRYSKIN,LIFEORB,311,166,189,167,295,None,0,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,POISONJAB;false;32,SUBSTITUTE;false;16,false,Normal=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/none;5/none;5/false;5/false";
    /// let state2 = State::deserialize(serialized_state);
    /// assert_eq!(state.serialize(), state2.serialize());
    ///
    /// ```
    pub fn deserialize(serialized: &str) -> State {
        let split: Vec<&str> = serialized.split("/").collect();
        let mut state = State {
            side_one: Side::deserialize(split[0]),
            side_two: Side::deserialize(split[1]),
            weather: StateWeather::deserialize(split[2]),
            terrain: StateTerrain::deserialize(split[3]),
            trick_room: StateTrickRoom::deserialize(split[4]),
            team_preview: split[5].parse::<bool>().unwrap(),
            use_damage_dealt: false,
            use_last_used_move: false,
        };
        state.set_conditional_mechanics();
        state
    }
}
