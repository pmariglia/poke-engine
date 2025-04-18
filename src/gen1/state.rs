use crate::choices::Choices;
use crate::define_enum_with_from_str;
use crate::instruction::{
    ChangeSideConditionInstruction, ChangeStatusInstruction, Instruction,
    RemoveVolatileStatusInstruction,
};
use crate::state::{
    LastUsedMove, Pokemon, PokemonBoostableStat, PokemonIndex, PokemonMoveIndex,
    PokemonSideCondition, PokemonStatus, PokemonType, Side, SideReference, State,
};
use core::panic;
use std::cmp;
use std::collections::HashSet;

fn multiply_boost(boost_num: i8, stat_value: i16) -> i16 {
    match boost_num {
        -6 => stat_value * 25 / 100,
        -5 => stat_value * 28 / 100,
        -4 => stat_value * 33 / 100,
        -3 => stat_value * 40 / 100,
        -2 => stat_value * 50 / 100,
        -1 => stat_value * 66 / 100,
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
                format!("{}", side.get_active_immutable().moves[&index].id).to_lowercase()
            }
            MoveChoice::Switch(index) => format!("{}", side.pokemon[*index].id).to_lowercase(),
            MoveChoice::None => "No Move".to_string(),
        }
    }
    pub fn from_string(s: &str, side: &Side) -> Option<MoveChoice> {
        let s = s.to_lowercase();
        if s == "none" {
            return Some(MoveChoice::None);
        }

        let mut pkmn_iter = side.pokemon.into_iter();
        while let Some(pkmn) = pkmn_iter.next() {
            if pkmn.id.to_string().to_lowercase() == s
                && pkmn_iter.pokemon_index != side.active_index
            {
                return Some(MoveChoice::Switch(pkmn_iter.pokemon_index));
            }
        }

        let mut move_iter = side.get_active_immutable().moves.into_iter();
        let move_name = s;
        while let Some(mv) = move_iter.next() {
            if format!("{:?}", mv.id).to_lowercase() == move_name {
                return Some(MoveChoice::Move(move_iter.pokemon_move_index));
            }
        }

        None
    }
}

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
    PokemonVolatileStatus {
        NONE,
        AQUARING,
        ATTRACT,
        AUTOTOMIZE,
        BANEFULBUNKER,
        BIDE,
        BOUNCE,
        BURNINGBULWARK,
        CHARGE,
        CONFUSION,
        CURSE,
        DEFENSECURL,
        DESTINYBOND,
        DIG,
        DISABLE,
        DIVE,
        ELECTRIFY,
        ELECTROSHOT,
        EMBARGO,
        ENCORE,
        ENDURE,
        FLASHFIRE,
        FLINCH,
        FLY,
        FOCUSENERGY,
        FOLLOWME,
        FORESIGHT,
        FREEZESHOCK,
        GASTROACID,
        GEOMANCY,
        GLAIVERUSH,
        GRUDGE,
        HEALBLOCK,
        HELPINGHAND,
        ICEBURN,
        IMPRISON,
        INGRAIN,
        KINGSSHIELD,
        LASERFOCUS,
        LEECHSEED,
        LIGHTSCREEN,
        LOCKEDMOVE,
        MAGICCOAT,
        MAGNETRISE,
        MAXGUARD,
        METEORBEAM,
        MINIMIZE,
        MIRACLEEYE,
        MUSTRECHARGE,
        NIGHTMARE,
        NORETREAT,
        OCTOLOCK,
        PARTIALLYTRAPPED,
        PERISH4,
        PERISH3,
        PERISH2,
        PERISH1,
        PHANTOMFORCE,
        POWDER,
        POWERSHIFT,
        POWERTRICK,
        PROTECT,
        PROTOSYNTHESISATK,
        PROTOSYNTHESISDEF,
        PROTOSYNTHESISSPA,
        PROTOSYNTHESISSPD,
        PROTOSYNTHESISSPE,
        QUARKDRIVEATK,
        QUARKDRIVEDEF,
        QUARKDRIVESPA,
        QUARKDRIVESPD,
        QUARKDRIVESPE,
        RAGE,
        RAGEPOWDER,
        RAZORWIND,
        REFLECT,
        ROOST,
        SALTCURE,
        SHADOWFORCE,
        SKULLBASH,
        SKYATTACK,
        SKYDROP,
        SILKTRAP,
        SLOWSTART,
        SMACKDOWN,
        SNATCH,
        SOLARBEAM,
        SOLARBLADE,
        SPARKLINGARIA,
        SPIKYSHIELD,
        SPOTLIGHT,
        STOCKPILE,
        SUBSTITUTE,
        SYRUPBOMB,
        TARSHOT,
        TAUNT,
        TELEKINESIS,
        THROATCHOP,
        TORMENT,
        UNBURDEN,
        UPROAR,
        YAWN,

        GEN1BURNNULLIFY,
        GEN1PARALYSISNULLIFY,
    },
    default = NONE
}

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    Weather {
        NONE,
        SUN,
        RAIN,
        SAND,
        HAIL,
        SNOW,
        HARSHSUN,
        HEAVYRAIN,
    }
}

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    Terrain {
        NONE,
        ELECTRICTERRAIN,
        PSYCHICTERRAIN,
        MISTYTERRAIN,
        GRASSYTERRAIN,
    }
}

impl Pokemon {
    pub fn crit_rate(&self, move_id: &Choices) -> f32 {
        let crit_multiplier = match move_id {
            Choices::SLASH => 8,
            Choices::CRABHAMMER => 8,
            Choices::RAZORLEAF => 8,
            Choices::KARATECHOP => 8,
            _ => 1,
        };
        let crit_rate = crit_multiplier * self.id.base_speed() / 2;
        let crit_rate = cmp::min(crit_rate, 255);
        crit_rate as f32 / 255.0
    }
    pub fn add_available_moves(
        &self,
        vec: &mut Vec<MoveChoice>,
        _last_used_move: &LastUsedMove,
        _encored: bool,
    ) {
        let mut iter = self.moves.into_iter();
        while let Some(p) = iter.next() {
            if !p.disabled && p.pp > 0 {
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

    pub fn item_is_permanent(&self) -> bool {
        false
    }

    pub fn item_can_be_removed(&self) -> bool {
        !self.item_is_permanent()
    }

    pub fn is_grounded(&self) -> bool {
        if self.has_type(&PokemonType::FLYING) {
            return false;
        }
        true
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
            PokemonVolatileStatus::LEECHSEED | PokemonVolatileStatus::CONFUSION => {
                if active_volatiles.contains(&PokemonVolatileStatus::SUBSTITUTE) {
                    return false;
                }
                true
            }
            PokemonVolatileStatus::SUBSTITUTE => self.hp > self.maxhp / 4,
            PokemonVolatileStatus::FLINCH => {
                if !first_move {
                    return false;
                }
                true
            }
            _ => true,
        }
    }

    pub fn immune_to_stats_lowered_by_opponent(
        &self,
        _stat: &PokemonBoostableStat,
        volatiles: &HashSet<PokemonVolatileStatus>,
    ) -> bool {
        if volatiles.contains(&PokemonVolatileStatus::SUBSTITUTE) {
            return true;
        }
        false
    }
}

impl Side {
    pub fn get_boost_from_boost_enum(&self, boost_enum: &PokemonBoostableStat) -> i8 {
        match boost_enum {
            PokemonBoostableStat::Attack => self.attack_boost,
            PokemonBoostableStat::Defense => self.defense_boost,
            PokemonBoostableStat::SpecialAttack => self.special_attack_boost,
            PokemonBoostableStat::SpecialDefense => {
                panic!("Cannot Boost spd in gen1. spa is used for spc")
            }
            PokemonBoostableStat::Speed => self.speed_boost,
            PokemonBoostableStat::Evasion => self.evasion_boost,
            PokemonBoostableStat::Accuracy => self.accuracy_boost,
        }
    }

    pub fn calculate_boosted_stat(&self, stat: PokemonBoostableStat) -> i16 {
        let active = self.get_active_immutable();
        match stat {
            PokemonBoostableStat::Attack => {
                let boost = self.attack_boost;
                multiply_boost(boost, active.attack)
            }
            PokemonBoostableStat::Defense => {
                let boost = self.defense_boost;
                multiply_boost(boost, active.defense)
            }
            PokemonBoostableStat::SpecialAttack => {
                let boost = self.special_attack_boost;
                multiply_boost(boost, active.special_attack)
            }
            PokemonBoostableStat::SpecialDefense => {
                panic!("Cannot Boost spd in gen1. spa is used for spc")
            }
            PokemonBoostableStat::Speed => {
                let boost = self.speed_boost;
                multiply_boost(boost, active.speed)
            }
            _ => {
                panic!("Not implemented")
            }
        }
    }

    pub fn has_alive_non_rested_sleeping_pkmn(&self) -> bool {
        for p in self.pokemon.into_iter() {
            if p.status == PokemonStatus::SLEEP && p.hp > 0 && p.rest_turns == 0 {
                return true;
            }
        }
        false
    }

    pub fn has_alive_frozen_pokemon(&self) -> bool {
        for p in self.pokemon.into_iter() {
            if p.status == PokemonStatus::FREEZE && p.hp > 0 {
                return true;
            }
        }
        false
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

    pub fn trapped(&self, _opponent_active: &Pokemon) -> bool {
        if self
            .volatile_statuses
            .contains(&PokemonVolatileStatus::LOCKEDMOVE)
        {
            return true;
        } else if self
            .volatile_statuses
            .contains(&PokemonVolatileStatus::PARTIALLYTRAPPED)
        {
            return true;
        }
        false
    }
}

impl State {
    pub fn root_get_all_options(&self) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        if self.team_preview {
            let mut s1_options = Vec::with_capacity(6);
            let mut s2_options = Vec::with_capacity(6);

            let mut pkmn_iter = self.side_one.pokemon.into_iter();
            while let Some(_) = pkmn_iter.next() {
                if self.side_one.pokemon[pkmn_iter.pokemon_index].hp > 0 {
                    s1_options.push(MoveChoice::Switch(pkmn_iter.pokemon_index));
                }
            }
            let mut pkmn_iter = self.side_two.pokemon.into_iter();
            while let Some(_) = pkmn_iter.next() {
                if self.side_two.pokemon[pkmn_iter.pokemon_index].hp > 0 {
                    s2_options.push(MoveChoice::Switch(pkmn_iter.pokemon_index));
                }
            }
            return (s1_options, s2_options);
        }

        let (mut s1_options, mut s2_options) = self.get_all_options();

        if self.side_one.force_trapped {
            s1_options.retain(|x| match x {
                MoveChoice::Move(_) => true,
                MoveChoice::Switch(_) => false,
                MoveChoice::None => true,
            });
        }
        if self.side_one.slow_uturn_move {
            s1_options.clear();
            let encored = self
                .side_one
                .volatile_statuses
                .contains(&PokemonVolatileStatus::ENCORE);
            self.side_one.get_active_immutable().add_available_moves(
                &mut s1_options,
                &self.side_one.last_used_move,
                encored,
            );
        }

        if self.side_two.force_trapped {
            s2_options.retain(|x| match x {
                MoveChoice::Move(_) => true,
                MoveChoice::Switch(_) => false,
                MoveChoice::None => true,
            });
        }
        if self.side_two.slow_uturn_move {
            s2_options.clear();
            let encored = self
                .side_two
                .volatile_statuses
                .contains(&PokemonVolatileStatus::ENCORE);
            self.side_two.get_active_immutable().add_available_moves(
                &mut s2_options,
                &self.side_two.last_used_move,
                encored,
            );
        }

        if s1_options.len() == 0 {
            s1_options.push(MoveChoice::None);
        }
        if s2_options.len() == 0 {
            s2_options.push(MoveChoice::None);
        }

        (s1_options, s2_options)
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

        if self
            .side_one
            .volatile_statuses
            .contains(&PokemonVolatileStatus::MUSTRECHARGE)
        {
            side_one_options.push(MoveChoice::None);
        } else {
            self.side_one.get_active_immutable().add_available_moves(
                &mut side_one_options,
                &self.side_one.last_used_move,
                false,
            );
            if !self.side_one.trapped(side_two_active) {
                self.side_one.add_switches(&mut side_one_options);
            }
        }

        if self
            .side_two
            .volatile_statuses
            .contains(&PokemonVolatileStatus::MUSTRECHARGE)
        {
            side_two_options.push(MoveChoice::None);
        } else {
            self.side_two.get_active_immutable().add_available_moves(
                &mut side_two_options,
                &self.side_two.last_used_move,
                false,
            );
            if !self.side_two.trapped(side_one_active) {
                self.side_two.add_switches(&mut side_two_options);
            }
        }

        if side_one_options.len() == 0 {
            side_one_options.push(MoveChoice::None);
        }
        if side_two_options.len() == 0 {
            side_two_options.push(MoveChoice::None);
        }

        (side_one_options, side_two_options)
    }

    pub fn reset_toxic(&mut self, side_ref: &SideReference, vec_to_add_to: &mut Vec<Instruction>) {
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
        let active = side.get_active();
        if active.status == PokemonStatus::TOXIC {
            active.status = PokemonStatus::POISON;
            vec_to_add_to.push(Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: *side_ref,
                pokemon_index: side.active_index,
                old_status: PokemonStatus::TOXIC,
                new_status: PokemonStatus::POISON,
            }));
        }
    }

    pub fn remove_volatile_statuses_on_switch(
        &mut self,
        side_ref: &SideReference,
        vec_to_add_to: &mut Vec<Instruction>,
    ) {
        let side = self.get_side(side_ref);
        for pkmn_volatile_status in &side.volatile_statuses {
            vec_to_add_to.push(Instruction::RemoveVolatileStatus(
                RemoveVolatileStatusInstruction {
                    side_ref: *side_ref,
                    volatile_status: *pkmn_volatile_status,
                },
            ));
        }
        side.volatile_statuses.drain();
    }

    pub fn terrain_is_active(&self, terrain: &Terrain) -> bool {
        &self.terrain.terrain_type == terrain && self.terrain.turns_remaining > 0
    }

    pub fn weather_is_active(&self, weather: &Weather) -> bool {
        &self.weather.weather_type == weather
    }

    fn _state_contains_any_move(&self, moves: &[Choices]) -> bool {
        for s in [&self.side_one, &self.side_two] {
            for pkmn in s.pokemon.into_iter() {
                for mv in pkmn.moves.into_iter() {
                    if moves.contains(&mv.id) {
                        return true;
                    }
                }
            }
        }

        false
    }

    // TODO: Change this so that only the turn in consideration sets these flags
    // TODO: Check opponent's move is counter
    pub fn set_conditional_mechanics(&mut self) {
        if self._state_contains_any_move(&[Choices::COUNTER]) {
            self.use_damage_dealt = true;
        }
    }
}
