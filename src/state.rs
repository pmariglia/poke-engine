use crate::choices::{Choice, Choices, MoveCategory, MOVES};
use crate::define_enum_with_from_str;
use crate::engine::abilities::Abilities;
use crate::engine::items::Items;
use crate::engine::state::{PokemonVolatileStatus, Terrain, Weather};
use crate::instruction::{BoostInstruction, EnableMoveInstruction, Instruction};
use crate::pokemon::PokemonName;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SideReference {
    SideOne,
    SideTwo,
}
impl SideReference {
    pub fn get_other_side(&self) -> SideReference {
        match self {
            SideReference::SideOne => SideReference::SideTwo,
            SideReference::SideTwo => SideReference::SideOne,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum PokemonSideCondition {
    AuroraVeil,
    CraftyShield,
    HealingWish,
    LightScreen,
    LuckyChant,
    LunarDance,
    MatBlock,
    Mist,
    Protect,
    QuickGuard,
    Reflect,
    Safeguard,
    Spikes,
    Stealthrock,
    StickyWeb,
    Tailwind,
    ToxicCount,
    ToxicSpikes,
    WideGuard,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LastUsedMove {
    Move(PokemonMoveIndex),
    Switch(PokemonIndex),
    None,
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

#[derive(Debug, Copy, PartialEq, Clone, Eq, Hash)]
pub enum PokemonMoveIndex {
    M0,
    M1,
    M2,
    M3,
}
impl PokemonMoveIndex {
    pub fn serialize(&self) -> String {
        match self {
            PokemonMoveIndex::M0 => "0".to_string(),
            PokemonMoveIndex::M1 => "1".to_string(),
            PokemonMoveIndex::M2 => "2".to_string(),
            PokemonMoveIndex::M3 => "3".to_string(),
        }
    }
    pub fn deserialize(serialized: &str) -> PokemonMoveIndex {
        match serialized {
            "0" => PokemonMoveIndex::M0,
            "1" => PokemonMoveIndex::M1,
            "2" => PokemonMoveIndex::M2,
            "3" => PokemonMoveIndex::M3,
            _ => panic!("Invalid PokemonMoveIndex: {}", serialized),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PokemonBoostableStat {
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
    Evasion,
    Accuracy,
}

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    PokemonStatus {
        NONE,
        BURN,
        SLEEP,
        FREEZE,
        PARALYZE,
        POISON,
        TOXIC,
    }
}

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    SideMovesFirst {
        SideOne,
        SideTwo,
        SpeedTie
    }
}

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone)]
    PokemonNature {
        HARDY,
        LONELY,
        ADAMANT,
        NAUGHTY,
        BRAVE,
        BOLD,
        DOCILE,
        IMPISH,
        LAX,
        RELAXED,
        MODEST,
        MILD,
        BASHFUL,
        RASH,
        QUIET,
        CALM,
        GENTLE,
        CAREFUL,
        QUIRKY,
        SASSY,
        TIMID,
        HASTY,
        JOLLY,
        NAIVE,
        SERIOUS
    }
}

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    PokemonType {
        NORMAL,
        FIRE,
        WATER,
        ELECTRIC,
        GRASS,
        ICE,
        FIGHTING,
        POISON,
        GROUND,
        FLYING,
        PSYCHIC,
        BUG,
        ROCK,
        GHOST,
        DRAGON,
        DARK,
        STEEL,
        FAIRY,
        STELLAR,
        TYPELESS,
    },
    default = TYPELESS
}

impl Index<&PokemonMoveIndex> for PokemonMoves {
    type Output = Move;

    fn index(&self, index: &PokemonMoveIndex) -> &Self::Output {
        match index {
            PokemonMoveIndex::M0 => &self.m0,
            PokemonMoveIndex::M1 => &self.m1,
            PokemonMoveIndex::M2 => &self.m2,
            PokemonMoveIndex::M3 => &self.m3,
        }
    }
}

impl IndexMut<&PokemonMoveIndex> for PokemonMoves {
    fn index_mut(&mut self, index: &PokemonMoveIndex) -> &mut Self::Output {
        match index {
            PokemonMoveIndex::M0 => &mut self.m0,
            PokemonMoveIndex::M1 => &mut self.m1,
            PokemonMoveIndex::M2 => &mut self.m2,
            PokemonMoveIndex::M3 => &mut self.m3,
        }
    }
}

pub struct PokemonMoveIterator<'a> {
    pub pokemon_move: &'a PokemonMoves,
    pub pokemon_move_index: PokemonMoveIndex,
    pub index: usize,
}

impl<'a> Iterator for PokemonMoveIterator<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                self.pokemon_move_index = PokemonMoveIndex::M0;
                Some(&self.pokemon_move.m0)
            }
            1 => {
                self.index += 1;
                self.pokemon_move_index = PokemonMoveIndex::M1;
                Some(&self.pokemon_move.m1)
            }
            2 => {
                self.index += 1;
                self.pokemon_move_index = PokemonMoveIndex::M2;
                Some(&self.pokemon_move.m2)
            }
            3 => {
                self.index += 1;
                self.pokemon_move_index = PokemonMoveIndex::M3;
                Some(&self.pokemon_move.m3)
            }
            _ => None,
        }
    }
}

impl<'a> IntoIterator for &'a PokemonMoves {
    type Item = &'a Move;
    type IntoIter = PokemonMoveIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PokemonMoveIterator {
            pokemon_move: &self,
            pokemon_move_index: PokemonMoveIndex::M0,
            index: 0,
        }
    }
}

#[derive(Debug, Copy, PartialEq, Clone, Eq, Hash)]
#[repr(u8)]
pub enum PokemonIndex {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
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

pub fn pokemon_index_iter() -> PokemonIndexIterator {
    PokemonIndexIterator { index: 0 }
}

pub struct PokemonIndexIterator {
    index: usize,
}

impl Iterator for PokemonIndexIterator {
    type Item = PokemonIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some(PokemonIndex::P0)
            }
            1 => {
                self.index += 1;
                Some(PokemonIndex::P1)
            }
            2 => {
                self.index += 1;
                Some(PokemonIndex::P2)
            }
            3 => {
                self.index += 1;
                Some(PokemonIndex::P3)
            }
            4 => {
                self.index += 1;
                Some(PokemonIndex::P4)
            }
            5 => {
                self.index += 1;
                Some(PokemonIndex::P5)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SidePokemon {
    pub pkmn: [Pokemon; 6],
}

impl<'a> IntoIterator for &'a SidePokemon {
    type Item = &'a Pokemon;
    type IntoIter = SidePokemonIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SidePokemonIterator {
            side_pokemon: &self,
            pokemon_index: PokemonIndex::P0,
            index: 0,
        }
    }
}

pub struct SidePokemonIterator<'a> {
    pub side_pokemon: &'a SidePokemon,
    pub pokemon_index: PokemonIndex,
    pub index: usize,
}

impl<'a> Iterator for SidePokemonIterator<'a> {
    type Item = &'a Pokemon;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P0;
                Some(&self.side_pokemon.pkmn[0])
            }
            1 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P1;
                Some(&self.side_pokemon.pkmn[1])
            }
            2 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P2;
                Some(&self.side_pokemon.pkmn[2])
            }
            3 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P3;
                Some(&self.side_pokemon.pkmn[3])
            }
            4 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P4;
                Some(&self.side_pokemon.pkmn[4])
            }
            5 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P5;
                Some(&self.side_pokemon.pkmn[5])
            }
            _ => None,
        }
    }
}

impl Index<PokemonIndex> for SidePokemon {
    type Output = Pokemon;

    fn index(&self, index: PokemonIndex) -> &Self::Output {
        &self.pkmn[index as usize]
    }
}

impl Index<&PokemonIndex> for SidePokemon {
    type Output = Pokemon;

    fn index(&self, index: &PokemonIndex) -> &Self::Output {
        &self.pkmn[*index as usize]
    }
}

impl IndexMut<PokemonIndex> for SidePokemon {
    fn index_mut(&mut self, index: PokemonIndex) -> &mut Self::Output {
        &mut self.pkmn[index as usize]
    }
}

impl Default for Side {
    fn default() -> Side {
        Side {
            active_index: PokemonIndex::P0,
            baton_passing: false,
            shed_tailing: false,
            pokemon: SidePokemon {
                pkmn: [
                    Pokemon::default(),
                    Pokemon::default(),
                    Pokemon::default(),
                    Pokemon::default(),
                    Pokemon::default(),
                    Pokemon::default(),
                ],
            },
            substitute_health: 0,
            attack_boost: 0,
            defense_boost: 0,
            special_attack_boost: 0,
            special_defense_boost: 0,
            speed_boost: 0,
            accuracy_boost: 0,
            side_conditions: SideConditions {
                ..Default::default()
            },
            volatile_status_durations: VolatileStatusDurations::default(),
            volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
            wish: (0, 0),
            future_sight: (0, PokemonIndex::P0),
            force_switch: false,
            slow_uturn_move: false,
            force_trapped: false,
            last_used_move: LastUsedMove::None,
            damage_dealt: DamageDealt::default(),
            switch_out_move_second_saved_move: Choices::NONE,
            evasion_boost: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub id: Choices,
    pub disabled: bool,
    pub pp: i8,
    pub choice: Choice,
}
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
impl Default for Move {
    fn default() -> Move {
        Move {
            id: Choices::NONE,
            disabled: false,
            pp: 32,
            choice: Choice::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DamageDealt {
    pub damage: i16,
    pub move_category: MoveCategory,
    pub hit_substitute: bool,
}

impl Default for DamageDealt {
    fn default() -> DamageDealt {
        DamageDealt {
            damage: 0,
            move_category: MoveCategory::Physical,
            hit_substitute: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PokemonMoves {
    pub m0: Move,
    pub m1: Move,
    pub m2: Move,
    pub m3: Move,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SideConditions {
    pub aurora_veil: i8,
    pub crafty_shield: i8,
    pub healing_wish: i8,
    pub light_screen: i8,
    pub lucky_chant: i8,
    pub lunar_dance: i8,
    pub mat_block: i8,
    pub mist: i8,
    pub protect: i8,
    pub quick_guard: i8,
    pub reflect: i8,
    pub safeguard: i8,
    pub spikes: i8,
    pub stealth_rock: i8,
    pub sticky_web: i8,
    pub tailwind: i8,
    pub toxic_count: i8,
    pub toxic_spikes: i8,
    pub wide_guard: i8,
}
impl SideConditions {
    pub fn pprint(&self) -> String {
        let conditions = [
            ("aurora_veil", self.aurora_veil),
            ("crafty_shield", self.crafty_shield),
            ("healing_wish", self.healing_wish),
            ("light_screen", self.light_screen),
            ("lucky_chant", self.lucky_chant),
            ("lunar_dance", self.lunar_dance),
            ("mat_block", self.mat_block),
            ("mist", self.mist),
            ("protect", self.protect),
            ("quick_guard", self.quick_guard),
            ("reflect", self.reflect),
            ("safeguard", self.safeguard),
            ("spikes", self.spikes),
            ("stealth_rock", self.stealth_rock),
            ("sticky_web", self.sticky_web),
            ("tailwind", self.tailwind),
            ("toxic_count", self.toxic_count),
            ("toxic_spikes", self.toxic_spikes),
            ("wide_guard", self.wide_guard),
        ];

        let mut output = String::new();
        for (name, value) in conditions {
            if value != 0 {
                output.push_str(&format!("\n  {}: {}", name, value));
            }
        }
        if output.is_empty() {
            return "none".to_string();
        }
        output
    }
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
impl Default for SideConditions {
    fn default() -> SideConditions {
        SideConditions {
            aurora_veil: 0,
            crafty_shield: 0,
            healing_wish: 0,
            light_screen: 0,
            lucky_chant: 0,
            lunar_dance: 0,
            mat_block: 0,
            mist: 0,
            protect: 0,
            quick_guard: 0,
            reflect: 0,
            safeguard: 0,
            spikes: 0,
            stealth_rock: 0,
            sticky_web: 0,
            tailwind: 0,
            toxic_count: 0,
            toxic_spikes: 0,
            wide_guard: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateWeather {
    pub weather_type: Weather,
    pub turns_remaining: i8,
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

#[derive(Debug, PartialEq, Clone)]
pub struct StateTerrain {
    pub terrain_type: Terrain,
    pub turns_remaining: i8,
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

#[derive(Debug, PartialEq, Clone)]
pub struct StateTrickRoom {
    pub active: bool,
    pub turns_remaining: i8,
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

#[derive(Debug, Clone)]
pub struct VolatileStatusDurations {
    pub confusion: i8,
    pub encore: i8,
    pub lockedmove: i8,
    pub slowstart: i8,
    pub taunt: i8,
    pub yawn: i8,
}

impl Default for VolatileStatusDurations {
    fn default() -> VolatileStatusDurations {
        VolatileStatusDurations {
            confusion: 0,
            encore: 0,
            lockedmove: 0,
            slowstart: 0,
            taunt: 0,
            yawn: 0,
        }
    }
}

impl VolatileStatusDurations {
    pub fn pprint(&self) -> String {
        let durations = [
            ("confusion", self.confusion),
            ("encore", self.encore),
            ("lockedmove", self.lockedmove),
            ("slowstart", self.slowstart),
            ("taunt", self.taunt),
            ("yawn", self.yawn),
        ];

        let mut output = String::new();
        for (name, value) in durations {
            if value != 0 {
                output.push_str(&format!("\n  {}: {}", name, value));
            }
        }
        if output.is_empty() {
            return "none".to_string();
        }
        output
    }

    pub fn serialize(&self) -> String {
        format!(
            "{};{};{};{};{};{}",
            self.confusion, self.encore, self.lockedmove, self.slowstart, self.taunt, self.yawn
        )
    }
    pub fn deserialize(serialized: &str) -> VolatileStatusDurations {
        let split: Vec<&str> = serialized.split(";").collect();
        VolatileStatusDurations {
            confusion: split[0].parse::<i8>().unwrap(),
            encore: split[1].parse::<i8>().unwrap(),
            lockedmove: split[2].parse::<i8>().unwrap(),
            slowstart: split[3].parse::<i8>().unwrap(),
            taunt: split[4].parse::<i8>().unwrap(),
            yawn: split[5].parse::<i8>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub id: PokemonName,
    pub level: i8,
    pub types: (PokemonType, PokemonType),
    pub base_types: (PokemonType, PokemonType),
    pub hp: i16,
    pub maxhp: i16,
    pub ability: Abilities,
    pub base_ability: Abilities,
    pub item: Items,
    pub nature: PokemonNature,
    pub evs: (u8, u8, u8, u8, u8, u8),
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
    pub status: PokemonStatus,
    pub rest_turns: i8,
    pub sleep_turns: i8,
    pub weight_kg: f32,
    pub terastallized: bool,
    pub tera_type: PokemonType,
    pub moves: PokemonMoves,
}

impl Default for Pokemon {
    fn default() -> Pokemon {
        Pokemon {
            id: PokemonName::NONE,
            level: 100,
            types: (PokemonType::NORMAL, PokemonType::TYPELESS),
            base_types: (PokemonType::NORMAL, PokemonType::TYPELESS),
            hp: 100,
            maxhp: 100,
            ability: Abilities::NONE,
            base_ability: Abilities::NONE,
            item: Items::NONE,
            nature: PokemonNature::SERIOUS,
            evs: (85, 85, 85, 85, 85, 85),
            attack: 100,
            defense: 100,
            special_attack: 100,
            special_defense: 100,
            speed: 100,
            status: PokemonStatus::NONE,
            rest_turns: 0,
            sleep_turns: 0,
            weight_kg: 1.0,
            terastallized: false,
            tera_type: PokemonType::NORMAL,
            moves: PokemonMoves {
                m0: Default::default(),
                m1: Default::default(),
                m2: Default::default(),
                m3: Default::default(),
            },
        }
    }
}

impl Pokemon {
    pub fn replace_move(&mut self, move_index: PokemonMoveIndex, new_move_name: Choices) {
        self.moves[&move_index].choice = MOVES.get(&new_move_name).unwrap().to_owned();
        self.moves[&move_index].id = new_move_name;
    }
    pub fn get_sleep_talk_choices(&self) -> Vec<Choice> {
        let mut vec = Vec::with_capacity(4);
        for p in self.moves.into_iter() {
            if p.id != Choices::SLEEPTALK && p.id != Choices::NONE {
                vec.push(p.choice.clone());
            }
        }
        vec
    }

    fn pprint_stats(&self) -> String {
        format!(
            "atk:{} def:{} spa:{} spd:{} spe:{}",
            self.attack, self.defense, self.special_attack, self.special_defense, self.speed
        )
    }
    pub fn pprint_concise(&self) -> String {
        format!("{}:{}/{}", self.id, self.hp, self.maxhp)
    }
    pub fn pprint_verbose(&self) -> String {
        let moves: Vec<String> = self
            .moves
            .into_iter()
            .map(|m| format!("{:?}", m.id).to_lowercase())
            .filter(|x| x != "none")
            .collect();
        format!(
            "\n  Name: {}\n  HP: {}/{}\n  Status: {:?}\n  Ability: {:?}\n  Item: {:?}\n  Stats: {}\n  Moves: {}",
            self.id,
            self.hp,
            self.maxhp,
            self.status,
            self.ability,
            self.item,
            self.pprint_stats(),
            moves.join(", ")
        )
    }
}

impl Pokemon {
    pub fn serialize(&self) -> String {
        let evs_str = format!(
            "{};{};{};{};{};{}",
            self.evs.0, self.evs.1, self.evs.2, self.evs.3, self.evs.4, self.evs.5
        );
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.id,
            self.level,
            self.types.0.to_string(),
            self.types.1.to_string(),
            self.base_types.0.to_string(),
            self.base_types.1.to_string(),
            self.hp,
            self.maxhp,
            self.ability.to_string(),
            self.base_ability.to_string(),
            self.item.to_string(),
            self.nature.to_string(),
            evs_str,
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
            self.terastallized,
            self.tera_type.to_string(),
        )
    }

    pub fn deserialize(serialized: &str) -> Pokemon {
        let split: Vec<&str> = serialized.split(",").collect();
        let evs = if split[12] != "" {
            let mut ev_iter = split[12].split(";");
            (
                ev_iter.next().unwrap().parse::<u8>().unwrap(),
                ev_iter.next().unwrap().parse::<u8>().unwrap(),
                ev_iter.next().unwrap().parse::<u8>().unwrap(),
                ev_iter.next().unwrap().parse::<u8>().unwrap(),
                ev_iter.next().unwrap().parse::<u8>().unwrap(),
                ev_iter.next().unwrap().parse::<u8>().unwrap(),
            )
        } else {
            (85, 85, 85, 85, 85, 85)
        };
        Pokemon {
            id: PokemonName::from_str(split[0]).unwrap(),
            level: split[1].parse::<i8>().unwrap(),
            types: (
                PokemonType::from_str(split[2]).unwrap(),
                PokemonType::from_str(split[3]).unwrap(),
            ),
            base_types: (
                PokemonType::from_str(split[4]).unwrap(),
                PokemonType::from_str(split[5]).unwrap(),
            ),
            hp: split[6].parse::<i16>().unwrap(),
            maxhp: split[7].parse::<i16>().unwrap(),
            ability: Abilities::from_str(split[8]).unwrap(),
            base_ability: Abilities::from_str(split[9]).unwrap(),
            item: Items::from_str(split[10]).unwrap(),
            nature: PokemonNature::from_str(split[11]).unwrap(),
            evs,
            attack: split[13].parse::<i16>().unwrap(),
            defense: split[14].parse::<i16>().unwrap(),
            special_attack: split[15].parse::<i16>().unwrap(),
            special_defense: split[16].parse::<i16>().unwrap(),
            speed: split[17].parse::<i16>().unwrap(),
            status: PokemonStatus::from_str(split[18]).unwrap(),
            rest_turns: split[19].parse::<i8>().unwrap(),
            sleep_turns: split[20].parse::<i8>().unwrap(),
            weight_kg: split[21].parse::<f32>().unwrap(),
            moves: PokemonMoves {
                m0: Move::deserialize(split[22]),
                m1: Move::deserialize(split[23]),
                m2: Move::deserialize(split[24]),
                m3: Move::deserialize(split[25]),
            },
            terastallized: split[26].parse::<bool>().unwrap(),
            tera_type: PokemonType::from_str(split[27]).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Side {
    pub active_index: PokemonIndex,
    pub baton_passing: bool,
    pub shed_tailing: bool,
    pub pokemon: SidePokemon,
    pub side_conditions: SideConditions,
    pub volatile_status_durations: VolatileStatusDurations,
    pub wish: (i8, i16),
    pub future_sight: (i8, PokemonIndex),
    pub force_switch: bool,
    pub force_trapped: bool,
    pub slow_uturn_move: bool,
    pub volatile_statuses: HashSet<PokemonVolatileStatus>,
    pub substitute_health: i16,
    pub attack_boost: i8,
    pub defense_boost: i8,
    pub special_attack_boost: i8,
    pub special_defense_boost: i8,
    pub speed_boost: i8,
    pub accuracy_boost: i8,
    pub evasion_boost: i8,
    pub last_used_move: LastUsedMove,
    pub damage_dealt: DamageDealt,
    pub switch_out_move_second_saved_move: Choices,
}
impl Side {
    fn io_conditional_print(&self) -> String {
        let mut output = String::new();
        if self.baton_passing {
            output.push_str("\n  baton_passing: true");
        }
        if self.wish.0 != 0 {
            output.push_str(&format!("\n  wish: ({}, {})", self.wish.0, self.wish.1));
        }
        if self.future_sight.0 != 0 {
            output.push_str(&format!(
                "\n  future_sight: ({}, {:?})",
                self.future_sight.0, self.pokemon[self.future_sight.1].id
            ));
        }
        if self
            .volatile_statuses
            .contains(&PokemonVolatileStatus::SUBSTITUTE)
        {
            output.push_str(&format!(
                "\n  substitute_health: {}",
                self.substitute_health
            ));
        }

        if !output.is_empty() {
            output.insert_str(0, "Extras:");
            output.push_str("\n");
        }

        output
    }
    pub fn pprint(&self, available_choices: Vec<String>) -> String {
        let reserve = self
            .pokemon
            .into_iter()
            .map(|p| p.pprint_concise())
            .collect::<Vec<String>>();
        format!(
            "\nPokemon: {}\n\
            Active:{}\n\
            Boosts: {}\n\
            Last Used Move: {}\n\
            Volatiles: {:?}\n\
            VolatileDurations: {}\n\
            Side Conditions: {}\n\
            {}\
            Available Choices: {}",
            reserve.join(", "),
            self.get_active_immutable().pprint_verbose(),
            format!(
                "Attack:{}, Defense:{}, SpecialAttack:{}, SpecialDefense:{}, Speed:{}",
                self.attack_boost,
                self.defense_boost,
                self.special_attack_boost,
                self.special_defense_boost,
                self.speed_boost
            ),
            self.last_used_move.serialize(),
            self.volatile_statuses,
            self.volatile_status_durations.pprint(),
            self.side_conditions.pprint(),
            self.io_conditional_print(),
            available_choices.join(", ")
        )
    }
    pub fn serialize(&self) -> String {
        let mut vs_string = String::new();
        for vs in &self.volatile_statuses {
            vs_string.push_str(&vs.to_string());
            vs_string.push_str(":");
        }
        format!(
            "{}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}={}",
            self.pokemon.pkmn[0].serialize(),
            self.pokemon.pkmn[1].serialize(),
            self.pokemon.pkmn[2].serialize(),
            self.pokemon.pkmn[3].serialize(),
            self.pokemon.pkmn[4].serialize(),
            self.pokemon.pkmn[5].serialize(),
            self.active_index.serialize(),
            self.side_conditions.serialize(),
            vs_string,
            self.volatile_status_durations.serialize(),
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
            self.shed_tailing,
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
                pkmn: [
                    Pokemon::deserialize(split[0]),
                    Pokemon::deserialize(split[1]),
                    Pokemon::deserialize(split[2]),
                    Pokemon::deserialize(split[3]),
                    Pokemon::deserialize(split[4]),
                    Pokemon::deserialize(split[5]),
                ],
            },
            active_index: PokemonIndex::deserialize(split[6]),
            side_conditions: SideConditions::deserialize(split[7]),
            volatile_statuses: vs_hashset,
            volatile_status_durations: VolatileStatusDurations::deserialize(split[9]),
            substitute_health: split[10].parse::<i16>().unwrap(),
            attack_boost: split[11].parse::<i8>().unwrap(),
            defense_boost: split[12].parse::<i8>().unwrap(),
            special_attack_boost: split[13].parse::<i8>().unwrap(),
            special_defense_boost: split[14].parse::<i8>().unwrap(),
            speed_boost: split[15].parse::<i8>().unwrap(),
            accuracy_boost: split[16].parse::<i8>().unwrap(),
            evasion_boost: split[17].parse::<i8>().unwrap(),
            wish: (
                split[18].parse::<i8>().unwrap(),
                split[19].parse::<i16>().unwrap(),
            ),
            future_sight: (
                split[20].parse::<i8>().unwrap(),
                PokemonIndex::deserialize(split[21]),
            ),
            force_switch: split[22].parse::<bool>().unwrap(),
            switch_out_move_second_saved_move: Choices::from_str(split[23]).unwrap(),
            baton_passing: split[24].parse::<bool>().unwrap(),
            shed_tailing: split[25].parse::<bool>().unwrap(),
            force_trapped: split[26].parse::<bool>().unwrap(),
            last_used_move: LastUsedMove::deserialize(split[27]),
            damage_dealt: DamageDealt::default(),
            slow_uturn_move: split[28].parse::<bool>().unwrap(),
        }
    }
}
impl Side {
    pub fn visible_alive_pkmn(&self) -> i8 {
        let mut count = 0;
        for p in self.pokemon.into_iter() {
            if p.hp > 0 {
                count += 1;
            }
        }
        count
    }
    pub fn get_active(&mut self) -> &mut Pokemon {
        &mut self.pokemon[self.active_index]
    }
    pub fn get_active_immutable(&self) -> &Pokemon {
        &self.pokemon[self.active_index]
    }
    fn toggle_force_switch(&mut self) {
        self.force_switch = !self.force_switch;
    }
    pub fn get_side_condition(&self, side_condition: PokemonSideCondition) -> i8 {
        match side_condition {
            PokemonSideCondition::AuroraVeil => self.side_conditions.aurora_veil,
            PokemonSideCondition::CraftyShield => self.side_conditions.crafty_shield,
            PokemonSideCondition::HealingWish => self.side_conditions.healing_wish,
            PokemonSideCondition::LightScreen => self.side_conditions.light_screen,
            PokemonSideCondition::LuckyChant => self.side_conditions.lucky_chant,
            PokemonSideCondition::LunarDance => self.side_conditions.lunar_dance,
            PokemonSideCondition::MatBlock => self.side_conditions.mat_block,
            PokemonSideCondition::Mist => self.side_conditions.mist,
            PokemonSideCondition::Protect => self.side_conditions.protect,
            PokemonSideCondition::QuickGuard => self.side_conditions.quick_guard,
            PokemonSideCondition::Reflect => self.side_conditions.reflect,
            PokemonSideCondition::Safeguard => self.side_conditions.safeguard,
            PokemonSideCondition::Spikes => self.side_conditions.spikes,
            PokemonSideCondition::Stealthrock => self.side_conditions.stealth_rock,
            PokemonSideCondition::StickyWeb => self.side_conditions.sticky_web,
            PokemonSideCondition::Tailwind => self.side_conditions.tailwind,
            PokemonSideCondition::ToxicCount => self.side_conditions.toxic_count,
            PokemonSideCondition::ToxicSpikes => self.side_conditions.toxic_spikes,
            PokemonSideCondition::WideGuard => self.side_conditions.wide_guard,
        }
    }
    pub fn update_side_condition(&mut self, side_condition: PokemonSideCondition, amount: i8) {
        match side_condition {
            PokemonSideCondition::AuroraVeil => self.side_conditions.aurora_veil += amount,
            PokemonSideCondition::CraftyShield => self.side_conditions.crafty_shield += amount,
            PokemonSideCondition::HealingWish => self.side_conditions.healing_wish += amount,
            PokemonSideCondition::LightScreen => self.side_conditions.light_screen += amount,
            PokemonSideCondition::LuckyChant => self.side_conditions.lucky_chant += amount,
            PokemonSideCondition::LunarDance => self.side_conditions.lunar_dance += amount,
            PokemonSideCondition::MatBlock => self.side_conditions.mat_block += amount,
            PokemonSideCondition::Mist => self.side_conditions.mist += amount,
            PokemonSideCondition::Protect => self.side_conditions.protect += amount,
            PokemonSideCondition::QuickGuard => self.side_conditions.quick_guard += amount,
            PokemonSideCondition::Reflect => self.side_conditions.reflect += amount,
            PokemonSideCondition::Safeguard => self.side_conditions.safeguard += amount,
            PokemonSideCondition::Spikes => self.side_conditions.spikes += amount,
            PokemonSideCondition::Stealthrock => self.side_conditions.stealth_rock += amount,
            PokemonSideCondition::StickyWeb => self.side_conditions.sticky_web += amount,
            PokemonSideCondition::Tailwind => self.side_conditions.tailwind += amount,
            PokemonSideCondition::ToxicCount => self.side_conditions.toxic_count += amount,
            PokemonSideCondition::ToxicSpikes => self.side_conditions.toxic_spikes += amount,
            PokemonSideCondition::WideGuard => self.side_conditions.wide_guard += amount,
        }
    }
    pub fn get_alive_pkmn_indices(&self) -> Vec<PokemonIndex> {
        let mut vec = Vec::with_capacity(6);
        let mut iter = self.pokemon.into_iter();

        while let Some(p) = iter.next() {
            if p.hp > 0 && iter.pokemon_index != self.active_index {
                vec.push(iter.pokemon_index.clone());
            }
        }

        vec
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub side_one: Side,
    pub side_two: Side,
    pub weather: StateWeather,
    pub terrain: StateTerrain,
    pub trick_room: StateTrickRoom,
    pub team_preview: bool,
    pub use_last_used_move: bool,
    pub use_damage_dealt: bool,
}
impl Default for State {
    fn default() -> State {
        let mut s = State {
            side_one: Side::default(),
            side_two: Side::default(),
            weather: StateWeather {
                weather_type: Weather::NONE,
                turns_remaining: -1,
            },
            terrain: StateTerrain {
                terrain_type: Terrain::NONE,
                turns_remaining: 0,
            },
            trick_room: StateTrickRoom {
                active: false,
                turns_remaining: 0,
            },
            team_preview: false,
            use_damage_dealt: false,
            use_last_used_move: false,
        };

        // many tests rely on the speed of side 2's active pokemon being greater than side_one's
        s.side_two.get_active().speed += 1;
        s
    }
}
impl State {
    pub fn battle_is_over(&self) -> f32 {
        //  0 if battle is not over
        //  1 if side one has won
        // -1 if side two has won
        if self.side_one.pokemon.into_iter().all(|p| p.hp <= 0) {
            return -1.0;
        }
        if self.side_two.pokemon.into_iter().all(|p| p.hp <= 0) {
            return 1.0;
        }
        0.0
    }

    pub fn get_side(&mut self, side_ref: &SideReference) -> &mut Side {
        match side_ref {
            SideReference::SideOne => &mut self.side_one,
            SideReference::SideTwo => &mut self.side_two,
        }
    }

    pub fn get_side_immutable(&self, side_ref: &SideReference) -> &Side {
        match side_ref {
            SideReference::SideOne => &self.side_one,
            SideReference::SideTwo => &self.side_two,
        }
    }

    pub fn get_both_sides(&mut self, side_ref: &SideReference) -> (&mut Side, &mut Side) {
        match side_ref {
            SideReference::SideOne => (&mut self.side_one, &mut self.side_two),
            SideReference::SideTwo => (&mut self.side_two, &mut self.side_one),
        }
    }

    pub fn get_both_sides_immutable(&self, side_ref: &SideReference) -> (&Side, &Side) {
        match side_ref {
            SideReference::SideOne => (&self.side_one, &self.side_two),
            SideReference::SideTwo => (&self.side_two, &self.side_one),
        }
    }

    pub fn reset_boosts(&mut self, side_ref: &SideReference, vec_to_add_to: &mut Vec<Instruction>) {
        let side = self.get_side(side_ref);

        if side.attack_boost != 0 {
            vec_to_add_to.push(Instruction::Boost(BoostInstruction {
                side_ref: *side_ref,
                stat: PokemonBoostableStat::Attack,
                amount: -1 * side.attack_boost,
            }));
            side.attack_boost = 0;
        }

        if side.defense_boost != 0 {
            vec_to_add_to.push(Instruction::Boost(BoostInstruction {
                side_ref: *side_ref,
                stat: PokemonBoostableStat::Defense,
                amount: -1 * side.defense_boost,
            }));
            side.defense_boost = 0;
        }

        if side.special_attack_boost != 0 {
            vec_to_add_to.push(Instruction::Boost(BoostInstruction {
                side_ref: *side_ref,
                stat: PokemonBoostableStat::SpecialAttack,
                amount: -1 * side.special_attack_boost,
            }));
            side.special_attack_boost = 0;
        }

        if side.special_defense_boost != 0 {
            vec_to_add_to.push(Instruction::Boost(BoostInstruction {
                side_ref: *side_ref,
                stat: PokemonBoostableStat::SpecialDefense,
                amount: -1 * side.special_defense_boost,
            }));
            side.special_defense_boost = 0;
        }

        if side.speed_boost != 0 {
            vec_to_add_to.push(Instruction::Boost(BoostInstruction {
                side_ref: *side_ref,
                stat: PokemonBoostableStat::Speed,
                amount: -1 * side.speed_boost,
            }));
            side.speed_boost = 0;
        }

        if side.evasion_boost != 0 {
            vec_to_add_to.push(Instruction::Boost(BoostInstruction {
                side_ref: *side_ref,
                stat: PokemonBoostableStat::Evasion,
                amount: -1 * side.evasion_boost,
            }));
            side.evasion_boost = 0;
        }

        if side.accuracy_boost != 0 {
            vec_to_add_to.push(Instruction::Boost(BoostInstruction {
                side_ref: *side_ref,
                stat: PokemonBoostableStat::Accuracy,
                amount: -1 * side.accuracy_boost,
            }));
            side.accuracy_boost = 0;
        }
    }

    pub fn re_enable_disabled_moves(
        &mut self,
        side_ref: &SideReference,
        vec_to_add_to: &mut Vec<Instruction>,
    ) {
        let active = self.get_side(side_ref).get_active();
        if active.moves.m0.disabled {
            active.moves.m0.disabled = false;
            vec_to_add_to.push(Instruction::EnableMove(EnableMoveInstruction {
                side_ref: *side_ref,
                move_index: PokemonMoveIndex::M0,
            }));
        }
        if active.moves.m1.disabled {
            active.moves.m1.disabled = false;
            vec_to_add_to.push(Instruction::EnableMove(EnableMoveInstruction {
                side_ref: *side_ref,
                move_index: PokemonMoveIndex::M1,
            }));
        }
        if active.moves.m2.disabled {
            active.moves.m2.disabled = false;
            vec_to_add_to.push(Instruction::EnableMove(EnableMoveInstruction {
                side_ref: *side_ref,
                move_index: PokemonMoveIndex::M2,
            }));
        }
        if active.moves.m3.disabled {
            active.moves.m3.disabled = false;
            vec_to_add_to.push(Instruction::EnableMove(EnableMoveInstruction {
                side_ref: *side_ref,
                move_index: PokemonMoveIndex::M3,
            }));
        }
    }

    fn damage(&mut self, side_ref: &SideReference, amount: i16) {
        let active = self.get_side(&side_ref).get_active();

        active.hp -= amount;
    }

    fn heal(&mut self, side_ref: &SideReference, amount: i16) {
        let active = self.get_side(&side_ref).get_active();

        active.hp += amount;
    }

    fn switch(
        &mut self,
        side_ref: &SideReference,
        next_active_index: PokemonIndex,
        _: PokemonIndex,
    ) {
        let side = self.get_side(&side_ref);
        side.active_index = next_active_index;
    }

    fn reverse_switch(
        &mut self,
        side_ref: &SideReference,
        _: PokemonIndex,
        previous_active_index: PokemonIndex,
    ) {
        let side = self.get_side(&side_ref);
        side.active_index = previous_active_index;
    }

    fn apply_volatile_status(
        &mut self,
        side_ref: &SideReference,
        volatile_status: PokemonVolatileStatus,
    ) {
        self.get_side(&side_ref)
            .volatile_statuses
            .insert(volatile_status);
    }

    fn remove_volatile_status(
        &mut self,
        side_ref: &SideReference,
        volatile_status: PokemonVolatileStatus,
    ) {
        self.get_side(&side_ref)
            .volatile_statuses
            .remove(&volatile_status);
    }

    fn change_status(
        &mut self,
        side_ref: &SideReference,
        pokemon_index: PokemonIndex,
        new_status: PokemonStatus,
    ) {
        let pkmn = &mut self.get_side(&side_ref).pokemon[pokemon_index];
        pkmn.status = new_status;
    }

    fn apply_boost(&mut self, side_ref: &SideReference, stat: &PokemonBoostableStat, amount: i8) {
        let side = self.get_side(&side_ref);
        match stat {
            PokemonBoostableStat::Attack => side.attack_boost += amount,
            PokemonBoostableStat::Defense => side.defense_boost += amount,
            PokemonBoostableStat::SpecialAttack => side.special_attack_boost += amount,
            PokemonBoostableStat::SpecialDefense => side.special_defense_boost += amount,
            PokemonBoostableStat::Speed => side.speed_boost += amount,
            PokemonBoostableStat::Evasion => side.evasion_boost += amount,
            PokemonBoostableStat::Accuracy => side.accuracy_boost += amount,
        }
    }

    fn increment_side_condition(
        &mut self,
        side_ref: &SideReference,
        side_condition: &PokemonSideCondition,
        amount: i8,
    ) {
        let side = self.get_side(&side_ref);

        match side_condition {
            PokemonSideCondition::AuroraVeil => side.side_conditions.aurora_veil += amount,
            PokemonSideCondition::CraftyShield => side.side_conditions.crafty_shield += amount,
            PokemonSideCondition::HealingWish => side.side_conditions.healing_wish += amount,
            PokemonSideCondition::LightScreen => side.side_conditions.light_screen += amount,
            PokemonSideCondition::LuckyChant => side.side_conditions.lucky_chant += amount,
            PokemonSideCondition::LunarDance => side.side_conditions.lunar_dance += amount,
            PokemonSideCondition::MatBlock => side.side_conditions.mat_block += amount,
            PokemonSideCondition::Mist => side.side_conditions.mist += amount,
            PokemonSideCondition::Protect => side.side_conditions.protect += amount,
            PokemonSideCondition::QuickGuard => side.side_conditions.quick_guard += amount,
            PokemonSideCondition::Reflect => side.side_conditions.reflect += amount,
            PokemonSideCondition::Safeguard => side.side_conditions.safeguard += amount,
            PokemonSideCondition::Spikes => side.side_conditions.spikes += amount,
            PokemonSideCondition::Stealthrock => side.side_conditions.stealth_rock += amount,
            PokemonSideCondition::StickyWeb => side.side_conditions.sticky_web += amount,
            PokemonSideCondition::Tailwind => side.side_conditions.tailwind += amount,
            PokemonSideCondition::ToxicCount => side.side_conditions.toxic_count += amount,
            PokemonSideCondition::ToxicSpikes => side.side_conditions.toxic_spikes += amount,
            PokemonSideCondition::WideGuard => side.side_conditions.wide_guard += amount,
        }
    }

    fn increment_volatile_status_duration(
        &mut self,
        side_ref: &SideReference,
        volatile_status: &PokemonVolatileStatus,
        amount: i8,
    ) {
        let side = self.get_side(&side_ref);
        match volatile_status {
            PokemonVolatileStatus::CONFUSION => {
                side.volatile_status_durations.confusion += amount;
            }
            PokemonVolatileStatus::LOCKEDMOVE => {
                side.volatile_status_durations.lockedmove += amount;
            }
            PokemonVolatileStatus::ENCORE => {
                side.volatile_status_durations.encore += amount;
            }
            PokemonVolatileStatus::SLOWSTART => {
                side.volatile_status_durations.slowstart += amount;
            }
            PokemonVolatileStatus::TAUNT => {
                side.volatile_status_durations.taunt += amount;
            }
            PokemonVolatileStatus::YAWN => {
                side.volatile_status_durations.yawn += amount;
            }
            _ => panic!(
                "Invalid volatile status for increment_volatile_status_duration: {:?}",
                volatile_status
            ),
        }
    }

    fn change_types(
        &mut self,
        side_reference: &SideReference,
        new_types: (PokemonType, PokemonType),
    ) {
        self.get_side(side_reference).get_active().types = new_types;
    }

    fn change_item(&mut self, side_reference: &SideReference, new_item: Items) {
        self.get_side(side_reference).get_active().item = new_item;
    }

    fn change_weather(&mut self, weather_type: Weather, turns_remaining: i8) {
        self.weather.weather_type = weather_type;
        self.weather.turns_remaining = turns_remaining;
    }

    fn change_terrain(&mut self, terrain_type: Terrain, turns_remaining: i8) {
        self.terrain.terrain_type = terrain_type;
        self.terrain.turns_remaining = turns_remaining;
    }

    fn enable_move(&mut self, side_reference: &SideReference, move_index: &PokemonMoveIndex) {
        self.get_side(side_reference).get_active().moves[move_index].disabled = false;
    }

    fn disable_move(&mut self, side_reference: &SideReference, move_index: &PokemonMoveIndex) {
        self.get_side(side_reference).get_active().moves[move_index].disabled = true;
    }

    fn set_wish(&mut self, side_reference: &SideReference, wish_amount_change: i16) {
        self.get_side(side_reference).wish.0 = 2;
        self.get_side(side_reference).wish.1 += wish_amount_change;
    }

    fn unset_wish(&mut self, side_reference: &SideReference, wish_amount_change: i16) {
        self.get_side(side_reference).wish.0 = 0;
        self.get_side(side_reference).wish.1 -= wish_amount_change;
    }

    fn increment_wish(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).wish.0 += 1;
    }

    fn decrement_wish(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).wish.0 -= 1;
    }

    fn set_future_sight(&mut self, side_reference: &SideReference, pokemon_index: PokemonIndex) {
        let side = self.get_side(side_reference);
        side.future_sight.0 = 3;
        side.future_sight.1 = pokemon_index;
    }

    fn unset_future_sight(
        &mut self,
        side_reference: &SideReference,
        previous_pokemon_index: PokemonIndex,
    ) {
        let side = self.get_side(side_reference);
        side.future_sight.0 = 0;
        side.future_sight.1 = previous_pokemon_index;
    }

    fn increment_future_sight(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).future_sight.0 += 1;
    }

    fn decrement_future_sight(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).future_sight.0 -= 1;
    }

    fn damage_substitute(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).substitute_health -= amount;
    }

    fn heal_substitute(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).substitute_health += amount;
    }

    fn set_substitute_health(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).substitute_health += amount;
    }

    fn decrement_rest_turn(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).get_active().rest_turns -= 1;
    }

    fn increment_rest_turn(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).get_active().rest_turns += 1;
    }

    fn set_rest_turn(
        &mut self,
        side_reference: &SideReference,
        pokemon_index: PokemonIndex,
        amount: i8,
    ) {
        self.get_side(side_reference).pokemon[pokemon_index].rest_turns = amount;
    }

    fn set_sleep_turn(
        &mut self,
        side_reference: &SideReference,
        pokemon_index: PokemonIndex,
        amount: i8,
    ) {
        self.get_side(side_reference).pokemon[pokemon_index].sleep_turns = amount;
    }

    fn toggle_trickroom(&mut self, new_turns_remaining: i8) {
        self.trick_room.active = !self.trick_room.active;
        self.trick_room.turns_remaining = new_turns_remaining;
    }

    fn set_last_used_move(&mut self, side_reference: &SideReference, last_used_move: LastUsedMove) {
        match side_reference {
            SideReference::SideOne => self.side_one.last_used_move = last_used_move,
            SideReference::SideTwo => self.side_two.last_used_move = last_used_move,
        }
    }

    fn decrement_pp(
        &mut self,
        side_reference: &SideReference,
        move_index: &PokemonMoveIndex,
        amount: &i8,
    ) {
        match side_reference {
            SideReference::SideOne => self.side_one.get_active().moves[move_index].pp -= amount,
            SideReference::SideTwo => self.side_two.get_active().moves[move_index].pp -= amount,
        }
    }

    fn increment_pp(
        &mut self,
        side_reference: &SideReference,
        move_index: &PokemonMoveIndex,
        amount: &i8,
    ) {
        match side_reference {
            SideReference::SideOne => self.side_one.get_active().moves[move_index].pp += amount,
            SideReference::SideTwo => self.side_two.get_active().moves[move_index].pp += amount,
        }
    }

    pub fn apply_instructions(&mut self, instructions: &Vec<Instruction>) {
        for i in instructions {
            self.apply_one_instruction(i)
        }
    }

    pub fn apply_one_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Damage(instruction) => {
                self.damage(&instruction.side_ref, instruction.damage_amount)
            }
            Instruction::Switch(instruction) => self.switch(
                &instruction.side_ref,
                instruction.next_index,
                instruction.previous_index,
            ),
            Instruction::ApplyVolatileStatus(instruction) => {
                self.apply_volatile_status(&instruction.side_ref, instruction.volatile_status)
            }
            Instruction::RemoveVolatileStatus(instruction) => {
                self.remove_volatile_status(&instruction.side_ref, instruction.volatile_status)
            }
            Instruction::ChangeStatus(instruction) => self.change_status(
                &instruction.side_ref,
                instruction.pokemon_index,
                instruction.new_status,
            ),
            Instruction::Boost(instruction) => {
                self.apply_boost(&instruction.side_ref, &instruction.stat, instruction.amount)
            }
            Instruction::ChangeSideCondition(instruction) => self.increment_side_condition(
                &instruction.side_ref,
                &instruction.side_condition,
                instruction.amount,
            ),
            Instruction::ChangeVolatileStatusDuration(instruction) => self
                .increment_volatile_status_duration(
                    &instruction.side_ref,
                    &instruction.volatile_status,
                    instruction.amount,
                ),
            Instruction::ChangeWeather(instruction) => self.change_weather(
                instruction.new_weather,
                instruction.new_weather_turns_remaining,
            ),
            Instruction::DecrementWeatherTurnsRemaining => {
                self.weather.turns_remaining -= 1;
            }
            Instruction::ChangeTerrain(instruction) => self.change_terrain(
                instruction.new_terrain,
                instruction.new_terrain_turns_remaining,
            ),
            Instruction::DecrementTerrainTurnsRemaining => {
                self.terrain.turns_remaining -= 1;
            }
            Instruction::ChangeType(instruction) => {
                self.change_types(&instruction.side_ref, instruction.new_types)
            }
            Instruction::ChangeAbility(instruction) => {
                let active = self.get_side(&instruction.side_ref).get_active();
                active.ability =
                    Abilities::from(active.ability as i16 + instruction.ability_change);
            }
            Instruction::Heal(instruction) => {
                self.heal(&instruction.side_ref, instruction.heal_amount)
            }
            Instruction::ChangeItem(instruction) => {
                self.change_item(&instruction.side_ref, instruction.new_item)
            }
            Instruction::ChangeAttack(instruction) => {
                self.get_side(&instruction.side_ref).get_active().attack += instruction.amount;
            }
            Instruction::ChangeDefense(instruction) => {
                self.get_side(&instruction.side_ref).get_active().defense += instruction.amount;
            }
            Instruction::ChangeSpecialAttack(instruction) => {
                self.get_side(&instruction.side_ref)
                    .get_active()
                    .special_attack += instruction.amount;
            }
            Instruction::ChangeSpecialDefense(instruction) => {
                self.get_side(&instruction.side_ref)
                    .get_active()
                    .special_defense += instruction.amount;
            }
            Instruction::ChangeSpeed(instruction) => {
                self.get_side(&instruction.side_ref).get_active().speed += instruction.amount;
            }
            Instruction::EnableMove(instruction) => {
                self.enable_move(&instruction.side_ref, &instruction.move_index)
            }
            Instruction::DisableMove(instruction) => {
                self.disable_move(&instruction.side_ref, &instruction.move_index)
            }
            Instruction::ChangeWish(instruction) => {
                self.set_wish(&instruction.side_ref, instruction.wish_amount_change);
            }
            Instruction::DecrementWish(instruction) => {
                self.decrement_wish(&instruction.side_ref);
            }
            Instruction::SetFutureSight(instruction) => {
                self.set_future_sight(&instruction.side_ref, instruction.pokemon_index);
            }
            Instruction::DecrementFutureSight(instruction) => {
                self.decrement_future_sight(&instruction.side_ref);
            }
            Instruction::DamageSubstitute(instruction) => {
                self.damage_substitute(&instruction.side_ref, instruction.damage_amount);
            }
            Instruction::ChangeSubstituteHealth(instruction) => {
                self.set_substitute_health(&instruction.side_ref, instruction.health_change);
            }
            Instruction::SetRestTurns(instruction) => {
                self.set_rest_turn(
                    &instruction.side_ref,
                    instruction.pokemon_index,
                    instruction.new_turns,
                );
            }
            Instruction::SetSleepTurns(instruction) => {
                self.set_sleep_turn(
                    &instruction.side_ref,
                    instruction.pokemon_index,
                    instruction.new_turns,
                );
            }
            Instruction::DecrementRestTurns(instruction) => {
                self.decrement_rest_turn(&instruction.side_ref);
            }
            Instruction::ToggleTrickRoom(instruction) => {
                self.toggle_trickroom(instruction.new_trickroom_turns_remaining)
            }
            Instruction::DecrementTrickRoomTurnsRemaining => {
                self.trick_room.turns_remaining -= 1;
            }
            Instruction::ToggleSideOneForceSwitch => self.side_one.toggle_force_switch(),
            Instruction::ToggleSideTwoForceSwitch => self.side_two.toggle_force_switch(),
            Instruction::SetSideOneMoveSecondSwitchOutMove(instruction) => {
                self.side_one.switch_out_move_second_saved_move = instruction.new_choice;
            }
            Instruction::SetSideTwoMoveSecondSwitchOutMove(instruction) => {
                self.side_two.switch_out_move_second_saved_move = instruction.new_choice;
            }
            Instruction::ToggleBatonPassing(instruction) => match instruction.side_ref {
                SideReference::SideOne => {
                    self.side_one.baton_passing = !self.side_one.baton_passing
                }
                SideReference::SideTwo => {
                    self.side_two.baton_passing = !self.side_two.baton_passing
                }
            },
            Instruction::ToggleShedTailing(instruction) => match instruction.side_ref {
                SideReference::SideOne => self.side_one.shed_tailing = !self.side_one.shed_tailing,
                SideReference::SideTwo => self.side_two.shed_tailing = !self.side_two.shed_tailing,
            },
            Instruction::ToggleTerastallized(instruction) => match instruction.side_ref {
                SideReference::SideOne => self.side_one.get_active().terastallized ^= true,
                SideReference::SideTwo => self.side_two.get_active().terastallized ^= true,
            },
            Instruction::SetLastUsedMove(instruction) => {
                self.set_last_used_move(&instruction.side_ref, instruction.last_used_move)
            }
            Instruction::ChangeDamageDealtDamage(instruction) => {
                self.get_side(&instruction.side_ref).damage_dealt.damage +=
                    instruction.damage_change
            }
            Instruction::ChangeDamageDealtMoveCatagory(instruction) => {
                self.get_side(&instruction.side_ref)
                    .damage_dealt
                    .move_category = instruction.move_category
            }
            Instruction::ToggleDamageDealtHitSubstitute(instruction) => {
                let side = self.get_side(&instruction.side_ref);
                side.damage_dealt.hit_substitute = !side.damage_dealt.hit_substitute;
            }
            Instruction::DecrementPP(instruction) => self.decrement_pp(
                &instruction.side_ref,
                &instruction.move_index,
                &instruction.amount,
            ),
            Instruction::FormeChange(instruction) => {
                let active = self.get_side(&instruction.side_ref).get_active();
                active.id = PokemonName::from(active.id as i16 + instruction.name_change);
            }
        }
    }

    pub fn reverse_instructions(&mut self, instructions: &Vec<Instruction>) {
        for i in instructions.iter().rev() {
            self.reverse_one_instruction(i);
        }
    }

    pub fn reverse_one_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Damage(instruction) => {
                self.heal(&instruction.side_ref, instruction.damage_amount)
            }
            Instruction::Switch(instruction) => self.reverse_switch(
                &instruction.side_ref,
                instruction.next_index,
                instruction.previous_index,
            ),
            Instruction::ApplyVolatileStatus(instruction) => {
                self.remove_volatile_status(&instruction.side_ref, instruction.volatile_status)
            }
            Instruction::RemoveVolatileStatus(instruction) => {
                self.apply_volatile_status(&instruction.side_ref, instruction.volatile_status)
            }
            Instruction::ChangeStatus(instruction) => self.change_status(
                &instruction.side_ref,
                instruction.pokemon_index,
                instruction.old_status,
            ),
            Instruction::Boost(instruction) => self.apply_boost(
                &instruction.side_ref,
                &instruction.stat,
                -1 * instruction.amount,
            ),
            Instruction::ChangeSideCondition(instruction) => self.increment_side_condition(
                &instruction.side_ref,
                &instruction.side_condition,
                -1 * instruction.amount,
            ),
            Instruction::ChangeVolatileStatusDuration(instruction) => self
                .increment_volatile_status_duration(
                    &instruction.side_ref,
                    &instruction.volatile_status,
                    -1 * instruction.amount,
                ),
            Instruction::ChangeWeather(instruction) => self.change_weather(
                instruction.previous_weather,
                instruction.previous_weather_turns_remaining,
            ),
            Instruction::DecrementWeatherTurnsRemaining => {
                self.weather.turns_remaining += 1;
            }
            Instruction::ChangeTerrain(instruction) => self.change_terrain(
                instruction.previous_terrain,
                instruction.previous_terrain_turns_remaining,
            ),
            Instruction::DecrementTerrainTurnsRemaining => {
                self.terrain.turns_remaining += 1;
            }
            Instruction::ChangeType(instruction) => {
                self.change_types(&instruction.side_ref, instruction.old_types)
            }
            Instruction::ChangeAbility(instruction) => {
                let active = self.get_side(&instruction.side_ref).get_active();
                active.ability =
                    Abilities::from(active.ability as i16 - instruction.ability_change);
            }
            Instruction::EnableMove(instruction) => {
                self.disable_move(&instruction.side_ref, &instruction.move_index)
            }
            Instruction::DisableMove(instruction) => {
                self.enable_move(&instruction.side_ref, &instruction.move_index)
            }
            Instruction::Heal(instruction) => {
                self.damage(&instruction.side_ref, instruction.heal_amount)
            }
            Instruction::ChangeItem(instruction) => {
                self.change_item(&instruction.side_ref, instruction.current_item)
            }
            Instruction::ChangeAttack(instruction) => {
                self.get_side(&instruction.side_ref).get_active().attack -= instruction.amount;
            }
            Instruction::ChangeDefense(instruction) => {
                self.get_side(&instruction.side_ref).get_active().defense -= instruction.amount;
            }
            Instruction::ChangeSpecialAttack(instruction) => {
                self.get_side(&instruction.side_ref)
                    .get_active()
                    .special_attack -= instruction.amount;
            }
            Instruction::ChangeSpecialDefense(instruction) => {
                self.get_side(&instruction.side_ref)
                    .get_active()
                    .special_defense -= instruction.amount;
            }
            Instruction::ChangeSpeed(instruction) => {
                self.get_side(&instruction.side_ref).get_active().speed -= instruction.amount;
            }
            Instruction::ChangeWish(instruction) => {
                self.unset_wish(&instruction.side_ref, instruction.wish_amount_change)
            }
            Instruction::DecrementWish(instruction) => self.increment_wish(&instruction.side_ref),
            Instruction::SetFutureSight(instruction) => {
                self.unset_future_sight(&instruction.side_ref, instruction.previous_pokemon_index)
            }
            Instruction::DecrementFutureSight(instruction) => {
                self.increment_future_sight(&instruction.side_ref)
            }
            Instruction::DamageSubstitute(instruction) => {
                self.heal_substitute(&instruction.side_ref, instruction.damage_amount);
            }
            Instruction::ChangeSubstituteHealth(instruction) => {
                self.set_substitute_health(&instruction.side_ref, -1 * instruction.health_change);
            }
            Instruction::SetRestTurns(instruction) => {
                self.set_rest_turn(
                    &instruction.side_ref,
                    instruction.pokemon_index,
                    instruction.previous_turns,
                );
            }
            Instruction::SetSleepTurns(instruction) => {
                self.set_sleep_turn(
                    &instruction.side_ref,
                    instruction.pokemon_index,
                    instruction.previous_turns,
                );
            }
            Instruction::DecrementRestTurns(instruction) => {
                self.increment_rest_turn(&instruction.side_ref);
            }
            Instruction::ToggleTrickRoom(instruction) => {
                self.toggle_trickroom(instruction.previous_trickroom_turns_remaining)
            }
            Instruction::DecrementTrickRoomTurnsRemaining => {
                self.trick_room.turns_remaining += 1;
            }
            Instruction::ToggleSideOneForceSwitch => self.side_one.toggle_force_switch(),
            Instruction::ToggleSideTwoForceSwitch => self.side_two.toggle_force_switch(),
            Instruction::SetSideOneMoveSecondSwitchOutMove(instruction) => {
                self.side_one.switch_out_move_second_saved_move = instruction.previous_choice;
            }
            Instruction::SetSideTwoMoveSecondSwitchOutMove(instruction) => {
                self.side_two.switch_out_move_second_saved_move = instruction.previous_choice;
            }
            Instruction::ToggleBatonPassing(instruction) => match instruction.side_ref {
                SideReference::SideOne => {
                    self.side_one.baton_passing = !self.side_one.baton_passing
                }
                SideReference::SideTwo => {
                    self.side_two.baton_passing = !self.side_two.baton_passing
                }
            },
            Instruction::ToggleShedTailing(instruction) => match instruction.side_ref {
                SideReference::SideOne => self.side_one.shed_tailing = !self.side_one.shed_tailing,
                SideReference::SideTwo => self.side_two.shed_tailing = !self.side_two.shed_tailing,
            },
            Instruction::ToggleTerastallized(instruction) => match instruction.side_ref {
                SideReference::SideOne => self.side_one.get_active().terastallized ^= true,
                SideReference::SideTwo => self.side_two.get_active().terastallized ^= true,
            },
            Instruction::SetLastUsedMove(instruction) => {
                self.set_last_used_move(&instruction.side_ref, instruction.previous_last_used_move)
            }
            Instruction::ChangeDamageDealtDamage(instruction) => {
                self.get_side(&instruction.side_ref).damage_dealt.damage -=
                    instruction.damage_change
            }
            Instruction::ChangeDamageDealtMoveCatagory(instruction) => {
                self.get_side(&instruction.side_ref)
                    .damage_dealt
                    .move_category = instruction.previous_move_category
            }
            Instruction::ToggleDamageDealtHitSubstitute(instruction) => {
                let side = self.get_side(&instruction.side_ref);
                side.damage_dealt.hit_substitute = !side.damage_dealt.hit_substitute;
            }
            Instruction::DecrementPP(instruction) => self.increment_pp(
                &instruction.side_ref,
                &instruction.move_index,
                &instruction.amount,
            ),
            Instruction::FormeChange(instruction) => {
                let active = self.get_side(&instruction.side_ref).get_active();
                active.id = PokemonName::from(active.id as i16 - instruction.name_change);
            }
        }
    }
}
impl State {
    pub fn pprint(&self) -> String {
        let (side_one_options, side_two_options) = self.root_get_all_options();

        let mut side_one_choices = vec![];
        for option in side_one_options {
            side_one_choices.push(format!("{}", option.to_string(&self.side_one)).to_lowercase());
        }
        let mut side_two_choices = vec![];
        for option in side_two_options {
            side_two_choices.push(format!("{}", option.to_string(&self.side_two)).to_lowercase());
        }
        format!(
            "SideOne {}\n\nvs\n\nSideTwo {}\n\nState:\n  Weather: {:?},{}\n  Terrain: {:?},{}\n  TrickRoom: {},{}\n  UseLastUsedMove: {}\n  UseDamageDealt: {}",
            self.side_one.pprint(side_one_choices),
            self.side_two.pprint(side_two_choices),
            self.weather.weather_type,
            self.weather.turns_remaining,
            self.terrain.terrain_type,
            self.terrain.turns_remaining,
            self.trick_room.active,
            self.trick_room.turns_remaining,
            self.use_last_used_move,
            self.use_damage_dealt,
        )
    }

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
    ///     p0=p1=p2=p3=p4=p5=active_index=side_conditions=wish0=wish1=force_switch=switch_out_move_second_saved_move=baton_passing=shed_tailing=force_trapped=last_used_move=slow_uturn_move
    ///
    /// And the format for a pokemon is:
    ///    id,level,type1,type2,hp,maxhp,ability,item,attack,defense,special_attack,special_defense,speed,attack_boost,defense_boost,special_attack_boost,special_defense_boost,speed_boost,accuracy_boost,evasion_boost,status,substitute_health,rest_turns,weight_kg,volatile_statuses,m0,m1,m2,m3
    ///
    /// There's more to it, follow the code below to see a full example of a serialized state.
    /// */
    ///
    /// if cfg!(feature = "gen2") {
    ///    return;
    /// }
    ///
    /// use poke_engine::engine::abilities::Abilities;
    /// use poke_engine::engine::items::Items;
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
    /// // base_types 1 and 2. These are needed to revert to the correct type when switching out after being typechanged
    /// "Psychic,",
    /// "Typeless,",
    ///
    /// // hp
    /// "251,",
    ///
    /// // maxhp
    /// "251,",
    ///
    /// // ability
    /// "NONE,",
    ///
    /// // base ability. This is needed to revert to the correct ability when switching out after having the ability changed
    /// "NONE,",
    ///
    /// // item
    /// "LIFEORB,",
    ///
    /// // nature
    /// "SERIOUS,",
    ///
    /// // EVs split by `;`. Leave blank for default EVs (85 in all)
    /// "252;0;252;0;4;0,",
    /// // ",", left blank for default EVs
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
    /// // moves 1 through 4 (move_id;disabled;pp)
    /// "PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,",
    ///
    /// // terastallized
    /// "false,",
    ///
    /// // tera_type
    /// "Normal=",
    ///
    /// // all remaining Pokémon shown in 1 line for brevity
    /// "skarmory,100,Steel,Flying,Steel,Flying,271,271,STURDY,STURDY,CUSTAPBERRY,SERIOUS,,259,316,104,177,262,None,0,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,false,Normal=",
    /// "tyranitar,100,Rock,Dark,Rock,Dark,404,404,SANDSTREAM,SANDSTREAM,CHOPLEBERRY,SERIOUS,,305,256,203,327,159,None,0,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,false,Normal=",
    /// "mamoswine,100,Ice,Ground,Ice,Ground,362,362,THICKFAT,THICKFAT,NEVERMELTICE,SERIOUS,,392,196,158,176,241,None,0,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,false,Normal=",
    /// "jellicent,100,Water,Ghost,Water,Ghost,404,404,WATERABSORB,WATERABSORB,AIRBALLOON,SERIOUS,,140,237,206,246,180,None,0,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,false,Normal=",
    /// "excadrill,100,Ground,Steel,Ground,Steel,362,362,SANDFORCE,SANDFORCE,CHOICESCARF,SERIOUS,,367,156,122,168,302,None,0,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,false,Normal=",
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
    /// // some volatile statuses have durations associated with them, delimited by ;
    /// "0;0;0;0;0;0=",
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
    /// // a boolean representing if the side is baton passing
    /// "false=",
    ///
    /// // a boolean representing if the side is shed tailing
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
    /// "terrakion,100,Rock,Fighting,Rock,Fighting,323,323,NONE,NONE,FOCUSSASH,SERIOUS,,357,216,163,217,346,None,0,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,false,Normal=lucario,100,Fighting,Steel,Fighting,Steel,281,281,NONE,NONE,LIFEORB,SERIOUS,,350,176,241,177,279,None,0,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,false,Normal=breloom,100,Grass,Fighting,Grass,Fighting,262,262,TECHNICIAN,TECHNICIAN,LIFEORB,SERIOUS,,394,196,141,156,239,None,0,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,false,Normal=keldeo,100,Water,Fighting,Water,Fighting,323,323,NONE,NONE,LEFTOVERS,SERIOUS,,163,216,357,217,346,None,0,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,false,Normal=conkeldurr,100,Fighting,Typeless,Fighting,Typeless,414,414,GUTS,GUTS,LEFTOVERS,SERIOUS,,416,226,132,167,126,None,0,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,false,Normal=toxicroak,100,Poison,Fighting,Poison,Fighting,307,307,DRYSKIN,DRYSKIN,LIFEORB,SERIOUS,,311,166,189,167,295,None,0,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,false,Normal=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0;0;0;0;0;0=0=0=0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=false=switch:0=false/",
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
    ///
    /// // the same state, but all in one line
    /// let serialized_state = "alakazam,100,Psychic,Typeless,Psychic,Typeless,251,251,NONE,NONE,LIFEORB,SERIOUS,252;0;252;0;4;0,121,148,353,206,365,None,0,0,25.5,PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,false,Normal=skarmory,100,Steel,Flying,Steel,Flying,271,271,STURDY,STURDY,CUSTAPBERRY,SERIOUS,,259,316,104,177,262,None,0,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,false,Normal=tyranitar,100,Rock,Dark,Rock,Dark,404,404,SANDSTREAM,SANDSTREAM,CHOPLEBERRY,SERIOUS,,305,256,203,327,159,None,0,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,false,Normal=mamoswine,100,Ice,Ground,Ice,Ground,362,362,THICKFAT,THICKFAT,NEVERMELTICE,SERIOUS,,392,196,158,176,241,None,0,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,false,Normal=jellicent,100,Water,Ghost,Water,Ghost,404,404,WATERABSORB,WATERABSORB,AIRBALLOON,SERIOUS,,140,237,206,246,180,None,0,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,false,Normal=excadrill,100,Ground,Steel,Ground,Steel,362,362,SANDFORCE,SANDFORCE,CHOICESCARF,SERIOUS,,367,156,122,168,302,None,0,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,false,Normal=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0;0;0;0;0;0=0=0=0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=false=switch:0=false/terrakion,100,Rock,Fighting,Rock,Fighting,323,323,NONE,NONE,FOCUSSASH,SERIOUS,,357,216,163,217,346,None,0,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,false,Normal=lucario,100,Fighting,Steel,Fighting,Steel,281,281,NONE,NONE,LIFEORB,SERIOUS,,350,176,241,177,279,None,0,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,false,Normal=breloom,100,Grass,Fighting,Grass,Fighting,262,262,TECHNICIAN,TECHNICIAN,LIFEORB,SERIOUS,,394,196,141,156,239,None,0,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,false,Normal=keldeo,100,Water,Fighting,Water,Fighting,323,323,NONE,NONE,LEFTOVERS,SERIOUS,,163,216,357,217,346,None,0,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,false,Normal=conkeldurr,100,Fighting,Typeless,Fighting,Typeless,414,414,GUTS,GUTS,LEFTOVERS,SERIOUS,,416,226,132,167,126,None,0,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,false,Normal=toxicroak,100,Poison,Fighting,Poison,Fighting,307,307,DRYSKIN,DRYSKIN,LIFEORB,SERIOUS,,311,166,189,167,295,None,0,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,false,Normal=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0;0;0;0;0;0=0=0=0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=false=switch:0=false/none;5/none;5/false;5/false";
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
