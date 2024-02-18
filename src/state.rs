use crate::choices::Choice;
use core::panic;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::instruction::Instruction;

lazy_static! {
    pub static ref BOOST_MULTIPLIER: HashMap<i8, f32> = {
        let mut boost_multiplier: HashMap<i8, f32> = HashMap::new();

        boost_multiplier.insert(-6, 2.0 / 8.0);
        boost_multiplier.insert(-5, 2.0 / 7.0);
        boost_multiplier.insert(-4, 2.0 / 6.0);
        boost_multiplier.insert(-3, 2.0 / 5.0);
        boost_multiplier.insert(-2, 2.0 / 4.0);
        boost_multiplier.insert(-1, 2.0 / 3.0);
        boost_multiplier.insert(0, 2.0 / 2.0);
        boost_multiplier.insert(1, 3.0 / 2.0);
        boost_multiplier.insert(2, 4.0 / 8.0);
        boost_multiplier.insert(3, 5.0 / 8.0);
        boost_multiplier.insert(4, 6.0 / 8.0);
        boost_multiplier.insert(5, 7.0 / 8.0);
        boost_multiplier.insert(6, 8.0 / 8.0);

        boost_multiplier
    };
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
    GastroAcid,
    GlaiveRush,
    Grudge,
    HealBlock,
    HelpingHand,
    Imprison,
    Ingrain,
    KingsShield,
    LaserFocus,
    LeechSeed,
    LockedMove,
    MagicCoat,
    MagnetRise,
    MaxGuard,
    Minimize,
    MiracleEye,
    MustRecharge,
    Nightmare,
    NoRetreat,
    Octolock,
    PartiallyTrapped,
    PhantomForce,
    Powder,
    PowerShift,
    PowerTrick,
    Protect,
    Rage,
    RagePowder,
    Roost,
    SaltCure,
    ShadowForce,
    SilkTrap,
    SmackDown,
    Snatch,
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub enum PokemonNatures {
    Lonely,
    Adamant,
    Naughty,
    Brave,
    Bold,
    Impish,
    Lax,
    Relaxed,
    Modest,
    Mild,
    Rash,
    Quiet,
    Calm,
    Gentle,
    Careful,
    Sassy,
    Timid,
    Hasty,
    Jolly,
    Naive,

    // Neutral Natures
    Hardy,
    Docile,
    Bashful,
    Quirky,
    Serious,
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Move {
    pub id: String,
    pub disabled: bool,
    pub pp: i8,
}

#[derive(Debug)]
pub struct Pokemon {
    pub id: String,
    pub level: i8,
    pub types: (PokemonType, PokemonType),
    pub hp: i16,
    pub maxhp: i16,
    pub ability: String,
    pub item: String,
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
    pub attack_boost: i8,
    pub defense_boost: i8,
    pub special_attack_boost: i8,
    pub special_defense_boost: i8,
    pub speed_boost: i8,
    pub accuracy_boost: i8,
    pub evasion_boost: i8,
    pub status: PokemonStatus,
    pub substitute_health: i16,
    pub nature: PokemonNatures,
    pub volatile_statuses: HashSet<PokemonVolatileStatus>,
    pub moves: Vec<Move>,
}

impl Pokemon {
    pub fn get_pkmn_boost_enum_pairs(&self) -> [(PokemonBoostableStat, i8); 7] {
        return [
            (PokemonBoostableStat::Attack, self.attack_boost),
            (PokemonBoostableStat::Defense, self.defense_boost),
            (
                PokemonBoostableStat::SpecialAttack,
                self.special_attack_boost,
            ),
            (
                PokemonBoostableStat::SpecialDefense,
                self.special_defense_boost,
            ),
            (PokemonBoostableStat::Speed, self.speed_boost),
            (PokemonBoostableStat::Evasion, self.evasion_boost),
            (PokemonBoostableStat::Accuracy, self.accuracy_boost),
        ];
    }

    pub fn get_boost_from_boost_enum(&self, boost_enum: &PokemonBoostableStat) -> i8 {
        return match boost_enum {
            PokemonBoostableStat::Attack => self.attack_boost,
            PokemonBoostableStat::Defense => self.defense_boost,
            PokemonBoostableStat::SpecialAttack => self.special_attack_boost,
            PokemonBoostableStat::SpecialDefense => self.special_defense_boost,
            PokemonBoostableStat::Speed => self.speed_boost,
            PokemonBoostableStat::Evasion => self.evasion_boost,
            PokemonBoostableStat::Accuracy => self.accuracy_boost,
        };
    }
    pub fn has_type(&self, pkmn_type: &PokemonType) -> bool {
        return pkmn_type == &self.types.0 || pkmn_type == &self.types.1;
    }

    pub fn item_can_be_removed(&self) -> bool {
        return true;
    }

    pub fn clear_volatile_statuses(&mut self) {
        self.volatile_statuses.clear();
    }

    pub fn is_grounded(&self) -> bool {
        if self.has_type(&PokemonType::Flying)
            || self.ability == "levitate"
            || self.item == "airballoon"
        {
            return false;
        }
        return true;
    }

    pub fn calculate_boosted_stat(&self, stat: PokemonBoostableStat) -> i16 {
        match stat {
            PokemonBoostableStat::Attack => {
                (BOOST_MULTIPLIER.get(&self.attack_boost).unwrap() * self.attack as f32) as i16
            }
            PokemonBoostableStat::Defense => {
                (BOOST_MULTIPLIER.get(&self.defense_boost).unwrap() * self.defense as f32) as i16
            }
            PokemonBoostableStat::SpecialAttack => {
                (BOOST_MULTIPLIER.get(&self.special_attack_boost).unwrap()
                    * self.special_attack as f32) as i16
            }
            PokemonBoostableStat::SpecialDefense => {
                (BOOST_MULTIPLIER.get(&self.special_defense_boost).unwrap()
                    * self.special_defense as f32) as i16
            }
            PokemonBoostableStat::Speed => {
                (BOOST_MULTIPLIER.get(&self.speed_boost).unwrap() * self.speed as f32) as i16
            }
            _ => {
                panic!("Not implemented")
            }
        }
    }

    pub fn calculate_highest_stat(&self) -> PokemonBoostableStat {
        return PokemonBoostableStat::Attack;
    }

    pub fn volatile_status_can_be_applied(
        &self,
        volatile_status: &PokemonVolatileStatus,
        first_move: bool,
    ) -> bool {
        if self.volatile_statuses.contains(volatile_status) {
            return false;
        }
        match volatile_status {
            PokemonVolatileStatus::Substitute => return self.hp > self.maxhp / 4,
            PokemonVolatileStatus::Flinch => {
                if !first_move || ["innerfocus"].contains(&self.ability.as_str()) {
                    return false;
                }
                return true;
            }
            PokemonVolatileStatus::Protect => return first_move,
            PokemonVolatileStatus::Taunt
            | PokemonVolatileStatus::Torment
            | PokemonVolatileStatus::Encore
            | PokemonVolatileStatus::Disable
            | PokemonVolatileStatus::HealBlock
            | PokemonVolatileStatus::Attract => return self.ability.as_str() != "aromaveil",
            | PokemonVolatileStatus::Yawn => return self.ability.as_str() != "insomnia",
            _ => return true,
        }
    }

    pub fn immune_to_stats_lowered_by_opponent(&self, stat: &PokemonBoostableStat) -> bool {
        if ["clearbody", "whitesmoke", "fullmetalbody"].contains(&self.ability.as_str())
            || (["clearamulet"].contains(&self.item.as_str()))
        {
            return true;
        }

        if stat == &PokemonBoostableStat::Attack && self.ability.as_str() == "hypercutter" {
            return true;
        }

        return false;
    }
}

impl Default for Pokemon {
    fn default() -> Pokemon {
        return Pokemon {
            id: "rattata".to_string(),
            level: 100,
            types: (PokemonType::Normal, PokemonType::Typeless),
            hp: 100,
            maxhp: 100,
            ability: "none".to_string(),
            item: "none".to_string(),
            attack: 100,
            defense: 100,
            special_attack: 100,
            special_defense: 100,
            speed: 100,
            attack_boost: 0,
            defense_boost: 0,
            special_attack_boost: 0,
            special_defense_boost: 0,
            speed_boost: 0,
            accuracy_boost: 0,
            evasion_boost: 0,
            status: PokemonStatus::None,
            substitute_health: 0,
            nature: PokemonNatures::Serious,
            volatile_statuses: HashSet::<PokemonVolatileStatus>::new(),
            moves: vec![
                Move {
                    id: "tackle".to_string(),
                    disabled: false,
                    pp: 35,
                },
                Move {
                    id: "growl".to_string(),
                    disabled: false,
                    pp: 40,
                },
                Move {
                    id: "quickattack".to_string(),
                    disabled: false,
                    pp: 30,
                },
                Move {
                    id: "tailwhip".to_string(),
                    disabled: false,
                    pp: 30,
                },
            ],
        };
    }
}

#[derive(Debug)]
pub struct Side {
    pub active_index: usize,
    pub pokemon: [Pokemon; 6],
    pub side_conditions: SideConditions,
    pub wish: (i8, i16),
}

impl Side {
    pub fn get_active(&mut self) -> &mut Pokemon {
        &mut self.pokemon[self.active_index]
    }

    pub fn get_active_immutable(&self) -> &Pokemon {
        &self.pokemon[self.active_index]
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
}

impl Default for Side {
    fn default() -> Side {
        Side {
            active_index: 0,
            pokemon: [
                Pokemon {
                    ..Pokemon::default()
                },
                Pokemon {
                    ..Pokemon::default()
                },
                Pokemon {
                    ..Pokemon::default()
                },
                Pokemon {
                    ..Pokemon::default()
                },
                Pokemon {
                    ..Pokemon::default()
                },
                Pokemon {
                    ..Pokemon::default()
                },
            ],
            side_conditions: SideConditions {
                ..Default::default()
            },
            wish: (0, 0),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub side_one: Side,
    pub side_two: Side,
    pub weather: StateWeather,
    pub terrain: StateTerrain,
    pub trick_room: bool,
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
        }
    }
}

impl State {
    pub fn get_side(&mut self, side_ref: &SideReference) -> &mut Side {
        match side_ref {
            SideReference::SideOne => return &mut self.side_one,
            SideReference::SideTwo => return &mut self.side_two,
        }
    }

    pub fn get_side_immutable(&self, side_ref: &SideReference) -> &Side {
        match side_ref {
            SideReference::SideOne => return &self.side_one,
            SideReference::SideTwo => return &self.side_two,
        }
    }

    pub fn get_both_sides(&mut self, side_ref: &SideReference) -> (&mut Side, &mut Side) {
        match side_ref {
            SideReference::SideOne => return (&mut self.side_one, &mut self.side_two),
            SideReference::SideTwo => return (&mut self.side_two, &mut self.side_one),
        }
    }

    pub fn get_both_sides_immutable(&self, side_ref: &SideReference) -> (&Side, &Side) {
        match side_ref {
            SideReference::SideOne => return (&self.side_one, &self.side_two),
            SideReference::SideTwo => return (&self.side_two, &self.side_one),
        }
    }

    pub fn move_makes_contact(&self, choice: &Choice, attacking_side_ref: &SideReference) -> bool {
        if choice.flags.contact {
            if self
                .get_side_immutable(attacking_side_ref)
                .get_active_immutable()
                .item
                .as_str()
                == "protectivepads"
            {
                return false;
            }
            return true;
        }
        return false;
    }

    pub fn terrain_is_active(&self, terrain: &Terrain) -> bool {
        return &self.terrain.terrain_type == terrain && self.terrain.turns_remaining > 0;
    }

    pub fn weather_is_active(&self, weather: &Weather) -> bool {
        return &self.weather.weather_type == weather && self.weather.turns_remaining > 0;
    }

    fn damage(&mut self, side_ref: &SideReference, amount: i16) {
        let active = self.get_side(&side_ref).get_active();

        active.hp -= amount;
    }

    fn heal(&mut self, side_ref: &SideReference, amount: i16) {
        let active = self.get_side(&side_ref).get_active();

        active.hp += amount;
    }

    fn switch(&mut self, side_ref: &SideReference, next_active_index: usize, _: usize) {
        let side = self.get_side(&side_ref);
        side.active_index = next_active_index;
    }

    fn reverse_switch(&mut self, side_ref: &SideReference, _: usize, previous_active_index: usize) {
        let side = self.get_side(&side_ref);
        side.active_index = previous_active_index;
    }

    fn apply_volatile_status(
        &mut self,
        side_ref: &SideReference,
        volatile_status: PokemonVolatileStatus,
    ) {
        let active = self.get_side(&side_ref).get_active();
        active.volatile_statuses.insert(volatile_status);
    }

    fn remove_volatile_status(
        &mut self,
        side_ref: &SideReference,
        volatile_status: PokemonVolatileStatus,
    ) {
        let active = self.get_side(&side_ref).get_active();
        active.volatile_statuses.remove(&volatile_status);
    }

    fn change_status(
        &mut self,
        side_ref: &SideReference,
        pokemon_index: usize,
        new_status: PokemonStatus,
    ) {
        let pkmn = &mut self.get_side(&side_ref).pokemon[pokemon_index];
        pkmn.status = new_status;
    }

    fn apply_boost(&mut self, side_ref: &SideReference, stat: &PokemonBoostableStat, amount: i8) {
        let active = self.get_side(&side_ref).get_active();
        match stat {
            PokemonBoostableStat::Attack => active.attack_boost += amount,
            PokemonBoostableStat::Defense => active.defense_boost += amount,
            PokemonBoostableStat::SpecialAttack => active.special_attack_boost += amount,
            PokemonBoostableStat::SpecialDefense => active.special_defense_boost += amount,
            PokemonBoostableStat::Speed => active.speed_boost += amount,
            PokemonBoostableStat::Evasion => active.evasion_boost += amount,
            PokemonBoostableStat::Accuracy => active.accuracy_boost += amount,
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

    fn change_item(&mut self, side_reference: &SideReference, new_item: String) {
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

    fn enable_move(&mut self, side_reference: &SideReference, move_index: usize) {
        self.get_side(side_reference).get_active().moves[move_index].disabled = false;
    }

    fn disable_move(&mut self, side_reference: &SideReference, move_index: usize) {
        self.get_side(side_reference).get_active().moves[move_index].disabled = false;
    }

    fn increment_wish(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).wish.0 += 1;
    }

    fn decrement_wish(&mut self, side_reference: &SideReference) {
        self.get_side(side_reference).wish.0 -= 1;
    }

    fn damage_substitute(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).get_active().substitute_health -= amount;
    }

    fn heal_substitute(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).get_active().substitute_health += amount;
    }

    fn set_substitute_health(&mut self, side_reference: &SideReference, amount: i16) {
        self.get_side(side_reference).get_active().substitute_health = amount;
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
                self.change_item(&instruction.side_ref, instruction.new_item.clone())
            }
            Instruction::EnableMove(instruction) => {
                self.enable_move(&instruction.side_ref, instruction.move_index)
            }
            Instruction::DisableMove(instruction) => {
                self.disable_move(&instruction.side_ref, instruction.move_index)
            }
            Instruction::IncrementWish(instruction) => {
                self.increment_wish(&instruction.side_ref);
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
                self.change_item(&instruction.side_ref, instruction.current_item.clone())
            }
            Instruction::IncrementWish(instruction) => self.decrement_wish(&instruction.side_ref),
            Instruction::DecrementWish(instruction) => self.increment_wish(&instruction.side_ref),
            Instruction::DamageSubstitute(instruction) => {
                self.heal_substitute(&instruction.side_ref, instruction.damage_amount);
            }
            Instruction::SetSubstituteHealth(instruction) => {
                self.set_substitute_health(&instruction.side_ref, instruction.old_health);
            }
        }
    }
}
