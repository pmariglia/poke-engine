use crate::abilities::Abilities;
use crate::choices::{Choice, Choices, MoveCategory, MOVES};
use core::panic;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

use crate::instruction::{
    BoostInstruction, ChangeSideConditionInstruction, EnableMoveInstruction, Instruction,
    RemoveVolatileStatusInstruction,
};
use crate::items::Items;

fn multiply_boost(boost_num: i8, stat_value: i16) -> i16 {
    match boost_num {
        -6 => stat_value * 2 / 8,
        -5 => stat_value * 2 / 7,
        -4 => stat_value * 2 / 6,
        -3 => stat_value * 2 / 5,
        -2 => stat_value * 2 / 4,
        -1 => stat_value * 2 / 3,
        0 => stat_value,
        1 => stat_value * 3 / 2,
        2 => stat_value * 4 / 2,
        3 => stat_value * 5 / 2,
        4 => stat_value * 6 / 2,
        5 => stat_value * 7 / 2,
        6 => stat_value * 8 / 2,
        _ => panic!("Invalid boost number"),
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LastUsedMove {
    Move(Choices),
    Switch(PokemonIndex),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum MoveChoice {
    Move(PokemonMoveIndex),
    Switch(PokemonIndex),
    None,
}

impl MoveChoice {
    pub fn to_string(&self, side: &Side) -> String {
        match self {
            MoveChoice::Move(index) => {
                format!("{}", side.get_active_immutable().moves[*index].id).to_lowercase()
            }
            MoveChoice::Switch(index) => {
                format!("switch {}", side.pokemon[*index].id).to_lowercase()
            }
            MoveChoice::None => "No Move".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub enum PokemonStatus {
    None,
    Burn,
    Sleep,
    Freeze,
    Paralyze,
    Poison,
    Toxic,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum PokemonVolatileStatus {
    None,
    AquaRing,
    Attract,
    Autotomize,
    BanefulBunker,
    Bide,
    Bounce,
    Charge,
    Confusion,
    Curse,
    DefenseCurl,
    DestinyBond,
    Dig,
    Disable,
    Dive,
    Electrify,
    Embargo,
    Encore,
    Endure,
    FlashFire,
    Flinch,
    Fly,
    Focusenergy,
    FollowMe,
    Foresight,
    Freezeshock,
    GastroAcid,
    Geomancy,
    GlaiveRush,
    Grudge,
    HealBlock,
    HelpingHand,
    IceBurn,
    Imprison,
    Ingrain,
    KingsShield,
    LaserFocus,
    LeechSeed,
    LockedMove,
    MagicCoat,
    MagnetRise,
    MaxGuard,
    MeteorBeam,
    Minimize,
    MiracleEye,
    MustRecharge,
    Nightmare,
    NoRetreat,
    Octolock,
    PartiallyTrapped,
    Perish4,
    Perish3,
    Perish2,
    Perish1,
    PhantomForce,
    Powder,
    PowerShift,
    PowerTrick,
    Protect,
    Rage,
    RagePowder,
    RazorWind,
    Roost,
    SaltCure,
    ShadowForce,
    SkullBash,
    SkyAttack,
    SkyDrop,
    SilkTrap,
    SlowStart,
    SmackDown,
    Snatch,
    SolarBeam,
    SolarBlade,
    SparklingAria,
    SpikyShield,
    Spotlight,
    StockPile,
    Substitute,
    SyrupBomb,
    TarShot,
    Taunt,
    Telekinesis,
    ThroatChop,
    Torment,
    Unburden,
    Uproar,
    Yawn,
    YawnSleepThisTurn,
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Weather {
    None,
    Sun,
    Rain,
    Sand,
    Hail,
    HarshSun,
    HeavyRain,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateWeather {
    pub weather_type: Weather,
    pub turns_remaining: i8,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Terrain {
    None,
    ElectricTerrain,
    PsychicTerrain,
    MistyTerrain,
    GrassyTerrain,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateTerrain {
    pub terrain_type: Terrain,
    pub turns_remaining: i8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
    Typeless,
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

#[derive(Debug, Copy, PartialEq, Clone, Eq, Hash)]
pub enum PokemonMoveIndex {
    M0,
    M1,
    M2,
    M3,
    M4,
    M5,
}

#[derive(Debug, Clone)]
pub struct PokemonMoves {
    pub m0: Move,
    pub m1: Move,
    pub m2: Move,
    pub m3: Move,
    pub m4: Move,
    pub m5: Move,
}

impl Index<PokemonMoveIndex> for PokemonMoves {
    type Output = Move;

    fn index(&self, index: PokemonMoveIndex) -> &Self::Output {
        match index {
            PokemonMoveIndex::M0 => &self.m0,
            PokemonMoveIndex::M1 => &self.m1,
            PokemonMoveIndex::M2 => &self.m2,
            PokemonMoveIndex::M3 => &self.m3,
            PokemonMoveIndex::M4 => &self.m4,
            PokemonMoveIndex::M5 => &self.m5,
        }
    }
}

impl IndexMut<PokemonMoveIndex> for PokemonMoves {
    fn index_mut(&mut self, index: PokemonMoveIndex) -> &mut Self::Output {
        match index {
            PokemonMoveIndex::M0 => &mut self.m0,
            PokemonMoveIndex::M1 => &mut self.m1,
            PokemonMoveIndex::M2 => &mut self.m2,
            PokemonMoveIndex::M3 => &mut self.m3,
            PokemonMoveIndex::M4 => &mut self.m4,
            PokemonMoveIndex::M5 => &mut self.m5,
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
            4 => {
                self.index += 1;
                self.pokemon_move_index = PokemonMoveIndex::M4;
                Some(&self.pokemon_move.m4)
            }
            5 => {
                self.index += 1;
                self.pokemon_move_index = PokemonMoveIndex::M5;
                Some(&self.pokemon_move.m5)
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

#[derive(Debug, Clone)]
pub struct Move {
    pub id: Choices,
    pub disabled: bool,
    pub pp: i8,
    pub choice: Choice,
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
pub struct Pokemon {
    pub id: String,
    pub level: i8,
    pub types: (PokemonType, PokemonType),
    pub hp: i16,
    pub maxhp: i16,
    pub ability: Abilities,
    pub item: Items,
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
    pub status: PokemonStatus,
    pub rest_turns: i8,
    pub weight_kg: f32,
    pub moves: PokemonMoves,
}

impl Pokemon {
    pub fn get_sleep_talk_choices(&self) -> Vec<Choice> {
        let mut vec = Vec::with_capacity(4);
        for p in self.moves.into_iter() {
            if p.id != Choices::SLEEPTALK && p.id != Choices::NONE {
                vec.push(p.choice.clone());
            }
        }
        vec
    }
    pub fn replace_move(&mut self, move_index: PokemonMoveIndex, new_move_name: Choices) {
        self.moves[move_index].choice = MOVES.get(&new_move_name).unwrap().to_owned();
        self.moves[move_index].id = new_move_name;
    }

    pub fn add_available_moves(
        &self,
        vec: &mut Vec<MoveChoice>,
        last_used_move: &LastUsedMove,
        encored: bool,
    ) {
        let mut iter = self.moves.into_iter();
        while let Some(p) = iter.next() {
            if !p.disabled {
                if (iter.pokemon_move_index == PokemonMoveIndex::M4
                    || iter.pokemon_move_index == PokemonMoveIndex::M5)
                    && p.id == Choices::NONE
                {
                    break;
                }

                if encored {
                    match last_used_move {
                        LastUsedMove::Move(last_used_move) => {
                            if last_used_move != &p.id {
                                continue;
                            }
                        }
                        LastUsedMove::Switch(_) => {
                            // there are some situations where you switched out and got encored into
                            // a move from a different pokemon, because you also have that move
                            // just assume nothing is locked in this case
                        }
                    }
                }
                vec.push(MoveChoice::Move(iter.pokemon_move_index));
            }
        }
    }

    pub fn add_move_from_choice(&self, vec: &mut Vec<MoveChoice>, choice: Choices) {
        let mut iter = self.moves.into_iter();
        while let Some(p) = iter.next() {
            if p.id == choice {
                vec.push(MoveChoice::Move(iter.pokemon_move_index));
            }
        }
    }

    pub fn has_type(&self, pkmn_type: &PokemonType) -> bool {
        pkmn_type == &self.types.0 || pkmn_type == &self.types.1
    }

    pub fn item_can_be_removed(&self) -> bool {
        if self.ability == Abilities::STICKYHOLD {
            return false;
        }
        match self.item {
            Items::SPLASHPLATE
            | Items::TOXICPLATE
            | Items::EARTHPLATE
            | Items::STONEPLATE
            | Items::INSECTPLATE
            | Items::SPOOKYPLATE
            | Items::IRONPLATE
            | Items::FLAMEPLATE
            | Items::MEADOWPLATE
            | Items::ZAPPLATE
            | Items::MINDPLATE
            | Items::ICICLEPLATE
            | Items::DRACOPLATE
            | Items::DREADPLATE
            | Items::FISTPLATE
            | Items::BLANKPLATE
            | Items::SKYPLATE
            | Items::PIXIEPLATE => !self.id.starts_with("arceus"),
            _ => true,
        }
    }

    pub fn is_grounded(&self) -> bool {
        if self.has_type(&PokemonType::Flying)
            || self.ability == Abilities::LEVITATE
            || self.item == Items::AIRBALLOON
        {
            return false;
        }
        true
    }

    pub fn calculate_highest_stat(&self) -> PokemonBoostableStat {
        PokemonBoostableStat::Attack
    }

    pub fn volatile_status_can_be_applied(
        &self,
        volatile_status: &PokemonVolatileStatus,
        active_volatiles: &HashSet<PokemonVolatileStatus>,
        first_move: bool,
    ) -> bool {
        if active_volatiles.contains(volatile_status) || self.hp == 0 {
            return false;
        }
        match volatile_status {
            // grass immunity to leechseed covered by `powder`
            PokemonVolatileStatus::LeechSeed => {
                if active_volatiles.contains(&PokemonVolatileStatus::Substitute) {
                    return false;
                }
                true
            }
            PokemonVolatileStatus::Substitute => self.hp > self.maxhp / 4,
            PokemonVolatileStatus::Flinch => {
                if !first_move || [Abilities::INNERFOCUS].contains(&self.ability) {
                    return false;
                }
                true
            }
            PokemonVolatileStatus::Protect => first_move,
            PokemonVolatileStatus::Taunt
            | PokemonVolatileStatus::Torment
            | PokemonVolatileStatus::Encore
            | PokemonVolatileStatus::Disable
            | PokemonVolatileStatus::HealBlock
            | PokemonVolatileStatus::Attract => self.ability != Abilities::AROMAVEIL,
            PokemonVolatileStatus::Yawn => {
                // immunity to yawn via sleep immunity is handled in `get_instructions_from_volatile_statuses`
                !active_volatiles.contains(&PokemonVolatileStatus::YawnSleepThisTurn)
            }
            _ => true,
        }
    }

    pub fn immune_to_stats_lowered_by_opponent(
        &self,
        stat: &PokemonBoostableStat,
        volatiles: &HashSet<PokemonVolatileStatus>,
    ) -> bool {
        if [
            Abilities::CLEARBODY,
            Abilities::WHITESMOKE,
            Abilities::FULLMETALBODY,
        ]
        .contains(&self.ability)
            || ([Items::CLEARAMULET].contains(&self.item))
        {
            return true;
        }

        if volatiles.contains(&PokemonVolatileStatus::Substitute) {
            return true;
        }

        if stat == &PokemonBoostableStat::Attack && self.ability == Abilities::HYPERCUTTER {
            return true;
        } else if stat == &PokemonBoostableStat::Accuracy && self.ability == Abilities::KEENEYE {
            return true;
        }

        false
    }
}

impl Default for Pokemon {
    fn default() -> Pokemon {
        Pokemon {
            id: "rattata".to_string(),
            level: 100,
            types: (PokemonType::Normal, PokemonType::Typeless),
            hp: 100,
            maxhp: 100,
            ability: Abilities::NONE,
            item: Items::NONE,
            attack: 100,
            defense: 100,
            special_attack: 100,
            special_defense: 100,
            speed: 100,
            status: PokemonStatus::None,
            rest_turns: 0,
            weight_kg: 1.0,
            moves: PokemonMoves {
                m0: Default::default(),
                m1: Default::default(),
                m2: Default::default(),
                m3: Default::default(),
                m4: Default::default(),
                m5: Default::default(),
            },
        }
    }
}

#[derive(Debug, Copy, PartialEq, Clone, Eq, Hash)]
pub enum PokemonIndex {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
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
    pub p0: Pokemon,
    pub p1: Pokemon,
    pub p2: Pokemon,
    pub p3: Pokemon,
    pub p4: Pokemon,
    pub p5: Pokemon,
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
                Some(&self.side_pokemon.p0)
            }
            1 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P1;
                Some(&self.side_pokemon.p1)
            }
            2 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P2;
                Some(&self.side_pokemon.p2)
            }
            3 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P3;
                Some(&self.side_pokemon.p3)
            }
            4 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P4;
                Some(&self.side_pokemon.p4)
            }
            5 => {
                self.index += 1;
                self.pokemon_index = PokemonIndex::P5;
                Some(&self.side_pokemon.p5)
            }
            _ => None,
        }
    }
}

impl Index<PokemonIndex> for SidePokemon {
    type Output = Pokemon;

    fn index(&self, index: PokemonIndex) -> &Self::Output {
        match index {
            PokemonIndex::P0 => &self.p0,
            PokemonIndex::P1 => &self.p1,
            PokemonIndex::P2 => &self.p2,
            PokemonIndex::P3 => &self.p3,
            PokemonIndex::P4 => &self.p4,
            PokemonIndex::P5 => &self.p5,
        }
    }
}

impl IndexMut<PokemonIndex> for SidePokemon {
    fn index_mut(&mut self, index: PokemonIndex) -> &mut Self::Output {
        match index {
            PokemonIndex::P0 => &mut self.p0,
            PokemonIndex::P1 => &mut self.p1,
            PokemonIndex::P2 => &mut self.p2,
            PokemonIndex::P3 => &mut self.p3,
            PokemonIndex::P4 => &mut self.p4,
            PokemonIndex::P5 => &mut self.p5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Side {
    pub active_index: PokemonIndex,
    pub baton_passing: bool,
    pub pokemon: SidePokemon,
    pub side_conditions: SideConditions,
    pub wish: (i8, i16),
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
    pub fn get_boost_from_boost_enum(&self, boost_enum: &PokemonBoostableStat) -> i8 {
        match boost_enum {
            PokemonBoostableStat::Attack => self.attack_boost,
            PokemonBoostableStat::Defense => self.defense_boost,
            PokemonBoostableStat::SpecialAttack => self.special_attack_boost,
            PokemonBoostableStat::SpecialDefense => self.special_defense_boost,
            PokemonBoostableStat::Speed => self.speed_boost,
            PokemonBoostableStat::Evasion => self.evasion_boost,
            PokemonBoostableStat::Accuracy => self.accuracy_boost,
        }
    }

    pub fn calculate_boosted_stat(&self, stat: PokemonBoostableStat) -> i16 {
        /*
        In Gen4, simple doubles the effective boost, without it visually being doubled
        It will not boost beyond an effective value of 6 though.
        */
        let active = self.get_active_immutable();
        match stat {
            PokemonBoostableStat::Attack => {
                #[cfg(feature = "gen4")]
                let boost = if active.ability == Abilities::SIMPLE {
                    (self.attack_boost * 2).min(6).max(-6)
                } else {
                    self.attack_boost
                };

                #[cfg(not(feature = "gen4"))]
                let boost = self.attack_boost;

                multiply_boost(boost, active.attack)
            }
            PokemonBoostableStat::Defense => {
                #[cfg(feature = "gen4")]
                let boost = if active.ability == Abilities::SIMPLE {
                    (self.defense_boost * 2).min(6).max(-6)
                } else {
                    self.defense_boost
                };
                #[cfg(not(feature = "gen4"))]
                let boost = self.defense_boost;

                multiply_boost(boost, active.defense)
            }
            PokemonBoostableStat::SpecialAttack => {
                #[cfg(feature = "gen4")]
                let boost = if active.ability == Abilities::SIMPLE {
                    (self.special_attack_boost * 2).min(6).max(-6)
                } else {
                    self.special_attack_boost
                };
                #[cfg(not(feature = "gen4"))]
                let boost = self.special_attack_boost;

                multiply_boost(boost, active.special_attack)
            }
            PokemonBoostableStat::SpecialDefense => {
                #[cfg(feature = "gen4")]
                let boost = if active.ability == Abilities::SIMPLE {
                    (self.special_defense_boost * 2).min(6).max(-6)
                } else {
                    self.special_defense_boost
                };
                #[cfg(not(feature = "gen4"))]
                let boost = self.special_defense_boost;

                multiply_boost(boost, active.special_defense)
            }
            PokemonBoostableStat::Speed => {
                #[cfg(feature = "gen4")]
                let boost = if active.ability == Abilities::SIMPLE {
                    (self.speed_boost * 2).min(6).max(-6)
                } else {
                    self.speed_boost
                };
                #[cfg(not(feature = "gen4"))]
                let boost = self.speed_boost;

                multiply_boost(boost, active.speed)
            }
            _ => {
                panic!("Not implemented")
            }
        }
    }

    pub fn has_sleeping_pkmn(&self) -> bool {
        for p in self.pokemon.into_iter() {
            if p.status == PokemonStatus::Sleep {
                return true;
            }
        }
        false
    }

    fn toggle_force_switch(&mut self) {
        self.force_switch = !self.force_switch;
    }

    pub fn add_switches(&self, vec: &mut Vec<MoveChoice>) {
        let mut iter = self.pokemon.into_iter();
        while let Some(p) = iter.next() {
            if p.hp > 0 && iter.pokemon_index != self.active_index {
                vec.push(MoveChoice::Switch(iter.pokemon_index));
            }
        }
        if vec.len() == 0 {
            vec.push(MoveChoice::None);
        }
    }

    pub fn trapped(&self, opponent_active: &Pokemon) -> bool {
        let active_pkmn = self.get_active_immutable();
        if self
            .volatile_statuses
            .contains(&PokemonVolatileStatus::LockedMove)
        {
            return true;
        }
        if active_pkmn.item == Items::SHEDSHELL || active_pkmn.has_type(&PokemonType::Ghost) {
            return false;
        } else if self
            .volatile_statuses
            .contains(&PokemonVolatileStatus::PartiallyTrapped)
        {
            return true;
        } else if opponent_active.ability == Abilities::SHADOWTAG {
            return true;
        } else if opponent_active.ability == Abilities::ARENATRAP && active_pkmn.is_grounded() {
            return true;
        } else if opponent_active.ability == Abilities::MAGNETPULL
            && active_pkmn.has_type(&PokemonType::Steel)
        {
            return true;
        }
        false
    }

    pub fn get_active(&mut self) -> &mut Pokemon {
        &mut self.pokemon[self.active_index]
    }

    pub fn get_active_immutable(&self) -> &Pokemon {
        &self.pokemon[self.active_index]
    }

    pub fn num_alive_pkmn(&self) -> i8 {
        let mut count = 0;
        for p in self.pokemon.into_iter() {
            if p.hp > 0 {
                count += 1;
            }
        }
        count
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
}

impl Default for Side {
    fn default() -> Side {
        Side {
            active_index: PokemonIndex::P0,
            baton_passing: false,
            pokemon: SidePokemon {
                p0: Pokemon {
                    ..Pokemon::default()
                },
                p1: Pokemon {
                    ..Pokemon::default()
                },
                p2: Pokemon {
                    ..Pokemon::default()
                },
                p3: Pokemon {
                    ..Pokemon::default()
                },
                p4: Pokemon {
                    ..Pokemon::default()
                },
                p5: Pokemon {
                    ..Pokemon::default()
                },
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
            volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
            wish: (0, 0),
            force_switch: false,
            slow_uturn_move: false,
            force_trapped: false,
            last_used_move: LastUsedMove::Move(Choices::NONE),
            damage_dealt: DamageDealt::default(),
            switch_out_move_second_saved_move: Choices::NONE,
            evasion_boost: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub side_one: Side,
    pub side_two: Side,
    pub weather: StateWeather,
    pub terrain: StateTerrain,
    pub trick_room: bool,
    pub team_preview: bool,
}

impl Default for State {
    fn default() -> State {
        State {
            side_one: Side::default(),
            side_two: Side::default(),
            weather: StateWeather {
                weather_type: Weather::None,
                turns_remaining: 0,
            },
            terrain: StateTerrain {
                terrain_type: Terrain::None,
                turns_remaining: 0,
            },
            trick_room: false,
            team_preview: false,
        }
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
        // level == 1 represents an un-revealed pokemon
        if self
            .side_two
            .pokemon
            .into_iter()
            .all(|p| p.hp <= 0 && p.level != 1)
        {
            return 1.0;
        }
        0.0
    }

    pub fn get_all_options(&self) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        let mut side_one_options: Vec<MoveChoice> = Vec::with_capacity(9);
        let mut side_two_options: Vec<MoveChoice> = Vec::with_capacity(9);

        let side_one_active = self.side_one.get_active_immutable();
        let side_two_active = self.side_two.get_active_immutable();

        if self.side_one.force_switch {
            self.side_one.add_switches(&mut side_one_options);
            if self.side_two.switch_out_move_second_saved_move == Choices::NONE {
                side_two_options.push(MoveChoice::None);
            } else {
                self.side_two.get_active_immutable().add_move_from_choice(
                    &mut side_two_options,
                    self.side_two.switch_out_move_second_saved_move,
                );
            }
            return (side_one_options, side_two_options);
        }

        if self.side_two.force_switch {
            self.side_two.add_switches(&mut side_two_options);
            if self.side_one.switch_out_move_second_saved_move == Choices::NONE {
                side_one_options.push(MoveChoice::None);
            } else {
                self.side_one.get_active_immutable().add_move_from_choice(
                    &mut side_one_options,
                    self.side_one.switch_out_move_second_saved_move,
                );
            }
            return (side_one_options, side_two_options);
        }

        let side_one_force_switch = self.side_one.get_active_immutable().hp <= 0;
        let side_two_force_switch = self.side_two.get_active_immutable().hp <= 0;

        if side_one_force_switch && side_two_force_switch {
            self.side_one.add_switches(&mut side_one_options);
            self.side_two.add_switches(&mut side_two_options);
            return (side_one_options, side_two_options);
        }
        if side_one_force_switch {
            self.side_one.add_switches(&mut side_one_options);
            side_two_options.push(MoveChoice::None);
            return (side_one_options, side_two_options);
        }
        if side_two_force_switch {
            side_one_options.push(MoveChoice::None);
            self.side_two.add_switches(&mut side_two_options);
            return (side_one_options, side_two_options);
        }

        let encored = self
            .side_one
            .volatile_statuses
            .contains(&PokemonVolatileStatus::Encore);
        self.side_one.get_active_immutable().add_available_moves(
            &mut side_one_options,
            &self.side_one.last_used_move,
            encored,
        );

        if !self.side_one.trapped(side_two_active) {
            self.side_one.add_switches(&mut side_one_options);
        }

        let encored = self
            .side_two
            .volatile_statuses
            .contains(&PokemonVolatileStatus::Encore);
        self.side_two.get_active_immutable().add_available_moves(
            &mut side_two_options,
            &self.side_two.last_used_move,
            encored,
        );

        if !self.side_two.trapped(side_one_active) {
            self.side_two.add_switches(&mut side_two_options);
        }

        if side_one_options.len() == 0 {
            side_one_options.push(MoveChoice::None);
        }
        if side_two_options.len() == 0 {
            side_two_options.push(MoveChoice::None);
        }

        (side_one_options, side_two_options)
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

    pub fn reset_toxic_count(
        &mut self,
        side_ref: &SideReference,
        vec_to_add_to: &mut Vec<Instruction>,
    ) {
        let side = self.get_side(side_ref);
        if side.side_conditions.toxic_count > 0 {
            vec_to_add_to.push(Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: *side_ref,
                    side_condition: PokemonSideCondition::ToxicCount,
                    amount: -1 * side.side_conditions.toxic_count,
                },
            ));
            side.side_conditions.toxic_count = 0;
        }
    }

    pub fn remove_volatile_statuses(
        &mut self,
        side_ref: &SideReference,
        vec_to_add_to: &mut Vec<Instruction>,
        baton_passing: bool,
    ) {
        let side = self.get_side(side_ref);
        for pkmn_volatile_status in &side.volatile_statuses {
            // dont remove substitute or leechseed if batonpassing
            if baton_passing
                && (pkmn_volatile_status == &PokemonVolatileStatus::Substitute
                    || pkmn_volatile_status == &PokemonVolatileStatus::LeechSeed)
            {
                continue;
            }
            vec_to_add_to.push(Instruction::RemoveVolatileStatus(
                RemoveVolatileStatusInstruction {
                    side_ref: *side_ref,
                    volatile_status: *pkmn_volatile_status,
                },
            ));
        }
        side.volatile_statuses.drain();
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

    pub fn move_makes_contact(&self, choice: &Choice, attacking_side_ref: &SideReference) -> bool {
        if choice.flags.contact {
            if self
                .get_side_immutable(attacking_side_ref)
                .get_active_immutable()
                .item
                == Items::PROTECTIVEPADS
            {
                return false;
            }
            return true;
        }
        false
    }

    pub fn terrain_is_active(&self, terrain: &Terrain) -> bool {
        &self.terrain.terrain_type == terrain && self.terrain.turns_remaining > 0
    }

    pub fn weather_is_active(&self, weather: &Weather) -> bool {
        let s1_active = self.side_one.get_active_immutable();
        let s2_active = self.side_two.get_active_immutable();
        &self.weather.weather_type == weather
            && s1_active.ability != Abilities::AIRLOCK
            && s1_active.ability != Abilities::CLOUDNINE
            && s2_active.ability != Abilities::AIRLOCK
            && s2_active.ability != Abilities::CLOUDNINE
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

    fn enable_move(&mut self, side_reference: &SideReference, move_index: PokemonMoveIndex) {
        self.get_side(side_reference).get_active().moves[move_index].disabled = false;
    }

    fn disable_move(&mut self, side_reference: &SideReference, move_index: PokemonMoveIndex) {
        self.get_side(side_reference).get_active().moves[move_index].disabled = true;
    }

    fn set_wish(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).wish.0 = 2;
        self.get_side(side_reference).wish.1 = amount;
    }

    fn unset_wish(&mut self, side_reference: &SideReference, previous_wish_amount: i16) {
        self.get_side(side_reference).wish.0 = 0;
        self.get_side(side_reference).wish.1 = previous_wish_amount;
    }

    fn increment_wish(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).wish.0 += 1;
    }

    fn decrement_wish(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).wish.0 -= 1;
    }

    fn damage_substitute(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).substitute_health -= amount;
    }

    fn heal_substitute(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).substitute_health += amount;
    }

    fn set_substitute_health(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).substitute_health = amount;
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

    fn toggle_trickroom(&mut self) {
        self.trick_room = !self.trick_room;
    }

    fn set_last_used_move(&mut self, side_reference: &SideReference, last_used_move: LastUsedMove) {
        match side_reference {
            SideReference::SideOne => self.side_one.last_used_move = last_used_move,
            SideReference::SideTwo => self.side_two.last_used_move = last_used_move,
        }
    }

    fn set_damage_dealt(
        &mut self,
        side_reference: &SideReference,
        damage_amount: i16,
        move_category: MoveCategory,
        hit_substitute: bool,
    ) {
        let side = self.get_side(side_reference);
        side.damage_dealt.damage = damage_amount;
        side.damage_dealt.move_category = move_category;
        side.damage_dealt.hit_substitute = hit_substitute;
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
            Instruction::ChangeWeather(instruction) => self.change_weather(
                instruction.new_weather,
                instruction.new_weather_turns_remaining,
            ),
            Instruction::ChangeTerrain(instruction) => self.change_terrain(
                instruction.new_terrain,
                instruction.new_terrain_turns_remaining,
            ),
            Instruction::ChangeType(instruction) => {
                self.change_types(&instruction.side_ref, instruction.new_types)
            }
            Instruction::Heal(instruction) => {
                self.heal(&instruction.side_ref, instruction.heal_amount)
            }
            Instruction::ChangeItem(instruction) => {
                self.change_item(&instruction.side_ref, instruction.new_item)
            }
            Instruction::EnableMove(instruction) => {
                self.enable_move(&instruction.side_ref, instruction.move_index)
            }
            Instruction::DisableMove(instruction) => {
                self.disable_move(&instruction.side_ref, instruction.move_index)
            }
            Instruction::SetWish(instruction) => {
                self.set_wish(&instruction.side_ref, instruction.wish_amount);
            }
            Instruction::DecrementWish(instruction) => {
                self.decrement_wish(&instruction.side_ref);
            }
            Instruction::DamageSubstitute(instruction) => {
                self.damage_substitute(&instruction.side_ref, instruction.damage_amount);
            }
            Instruction::SetSubstituteHealth(instruction) => {
                self.set_substitute_health(&instruction.side_ref, instruction.new_health);
            }
            Instruction::SetRestTurns(instruction) => {
                self.set_rest_turn(
                    &instruction.side_ref,
                    instruction.pokemon_index,
                    instruction.new_turns,
                );
            }
            Instruction::DecrementRestTurns(instruction) => {
                self.decrement_rest_turn(&instruction.side_ref);
            }
            Instruction::ToggleTrickRoom => self.toggle_trickroom(),
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
            Instruction::SetLastUsedMove(instruction) => {
                self.set_last_used_move(&instruction.side_ref, instruction.last_used_move)
            }
            Instruction::SetDamageDealt(instruction) => self.set_damage_dealt(
                &instruction.side_ref,
                instruction.damage,
                instruction.move_category,
                instruction.hit_substitute,
            ),
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
            Instruction::ChangeWeather(instruction) => self.change_weather(
                instruction.previous_weather,
                instruction.previous_weather_turns_remaining,
            ),
            Instruction::ChangeTerrain(instruction) => self.change_terrain(
                instruction.previous_terrain,
                instruction.previous_terrain_turns_remaining,
            ),
            Instruction::ChangeType(instruction) => {
                self.change_types(&instruction.side_ref, instruction.old_types)
            }
            Instruction::EnableMove(instruction) => {
                self.disable_move(&instruction.side_ref, instruction.move_index)
            }
            Instruction::DisableMove(instruction) => {
                self.enable_move(&instruction.side_ref, instruction.move_index)
            }
            Instruction::Heal(instruction) => {
                self.damage(&instruction.side_ref, instruction.heal_amount)
            }
            Instruction::ChangeItem(instruction) => {
                self.change_item(&instruction.side_ref, instruction.current_item)
            }
            Instruction::SetWish(instruction) => {
                self.unset_wish(&instruction.side_ref, instruction.previous_wish_amount)
            }
            Instruction::DecrementWish(instruction) => self.increment_wish(&instruction.side_ref),
            Instruction::DamageSubstitute(instruction) => {
                self.heal_substitute(&instruction.side_ref, instruction.damage_amount);
            }
            Instruction::SetSubstituteHealth(instruction) => {
                self.set_substitute_health(&instruction.side_ref, instruction.old_health);
            }
            Instruction::SetRestTurns(instruction) => {
                self.set_rest_turn(
                    &instruction.side_ref,
                    instruction.pokemon_index,
                    instruction.previous_turns,
                );
            }
            Instruction::DecrementRestTurns(instruction) => {
                self.increment_rest_turn(&instruction.side_ref);
            }
            Instruction::ToggleTrickRoom => self.toggle_trickroom(),
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
            Instruction::SetLastUsedMove(instruction) => {
                self.set_last_used_move(&instruction.side_ref, instruction.previous_last_used_move)
            }
            Instruction::SetDamageDealt(instruction) => self.set_damage_dealt(
                &instruction.side_ref,
                instruction.previous_damage,
                instruction.previous_move_category,
                instruction.previous_hit_substitute,
            ),
        }
    }
}
