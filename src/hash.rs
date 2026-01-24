use rand::Rng;
use std::array::from_fn;

#[derive(Clone, Debug)]
pub struct StateHash {
    hash: u64,

    // Precomputed random numbers for each attribute that may change
    health_numbers: [[[u64; 715]; 6]; 2], // hp values from 0 to 714 for 6 pokemon on 2 sides
    switch_numbers: [[u64; 6]; 2],
    terastallized_numbers: [u64; 2],
    volatile_status_numbers: [[u64; 256]; 2],
    status_numbers: [[[u64; 7]; 6]; 2],
    boost_numbers: [[[u64; 13]; 7]; 2],
    side_condition_numbers: [[[u64; 17]; 19]; 2],
    volatile_duration_numbers: [[[u64; 17]; 6]; 2],
    weather_numbers: [[u64; 9]; 8],
    terrain_numbers: [[u64; 9]; 5],
    type_change_numbers: [[[u64; 20]; 20]; 2],
    ability_change_numbers: [[u64; 316]; 2],
    item_change_numbers: [[u64; 207]; 2],
    attack_change_numbers: [[u64; 650]; 2],
    defense_change_numbers: [[u64; 650]; 2],
    special_attack_change_numbers: [[u64; 650]; 2],
    special_defense_change_numbers: [[u64; 650]; 2],
    speed_change_numbers: [[u64; 650]; 2],
    move_disabled_numbers: [[[u64; 2]; 4]; 2],
    change_wish_counter_numbers: [[u64; 3]; 2],
    change_wish_health_numbers: [[u64; 357]; 2],
    future_sight_counter_numbers: [[u64; 4]; 2],
    future_sight_index_numbers: [[u64; 6]; 2],
}

impl Default for StateHash {
    fn default() -> Self {
        StateHash {
            hash: 0,
            health_numbers: [[[0u64; 715]; 6]; 2],
            switch_numbers: [[0u64; 6], [0u64; 6]],
            terastallized_numbers: [0u64; 2],
            volatile_status_numbers: [[0u64; 256]; 2],
            status_numbers: [[[0u64; 7]; 6]; 2],
            boost_numbers: [[[0u64; 13]; 7]; 2],
            side_condition_numbers: [[[0u64; 17]; 19]; 2],
            volatile_duration_numbers: [[[0u64; 17]; 6]; 2],
            weather_numbers: [[0u64; 9]; 8],
            terrain_numbers: [[0u64; 9]; 5],
            type_change_numbers: [[[0u64; 20]; 20]; 2],
            ability_change_numbers: [[0u64; 316]; 2],
            item_change_numbers: [[0u64; 207]; 2],
            attack_change_numbers: [[0u64; 650]; 2],
            defense_change_numbers: [[0u64; 650]; 2],
            special_attack_change_numbers: [[0u64; 650]; 2],
            special_defense_change_numbers: [[0u64; 650]; 2],
            speed_change_numbers: [[0u64; 650]; 2],
            move_disabled_numbers: [[[0u64; 2]; 4]; 2],
            change_wish_counter_numbers: [[0u64; 3]; 2],
            change_wish_health_numbers: [[0u64; 357]; 2],
            future_sight_counter_numbers: [[0u64; 4]; 2],
            future_sight_index_numbers: [[0u64; 6]; 2],
        }
    }
}

impl StateHash {
    pub fn get_hash(&self) -> u64 {
        self.hash
    }
    pub fn set_hash(&mut self, hash: u64) {
        self.hash = hash;
    }
    pub fn new_random() -> Self {
        let mut rng = rand::rng();

        StateHash {
            hash: rng.random(),
            health_numbers: from_fn(|_side| from_fn(|_slot| from_fn(|_hp| rng.random()))),
            switch_numbers: [from_fn(|_side| rng.random()), from_fn(|_slot| rng.random())],
            terastallized_numbers: from_fn(|_| rng.random()),
            volatile_status_numbers: from_fn(|_side| from_fn(|_volatile| rng.random())),
            status_numbers: from_fn(|_side| from_fn(|_slot| from_fn(|_status| rng.random()))),
            boost_numbers: from_fn(|_side| from_fn(|_boost| from_fn(|_amount| rng.random()))),
            side_condition_numbers: from_fn(|_side| {
                from_fn(|_condition| from_fn(|_amount| rng.random()))
            }),
            volatile_duration_numbers: from_fn(|_side| {
                from_fn(|_volatile| from_fn(|_duration| rng.random()))
            }),
            weather_numbers: from_fn(|_weather| from_fn(|_turns_remaining| rng.random())),
            terrain_numbers: from_fn(|_terrain| from_fn(|_turns_remaining| rng.random())),
            type_change_numbers: from_fn(|_side| {
                from_fn(|_type_1| from_fn(|_type_2| rng.random()))
            }),
            ability_change_numbers: from_fn(|_side| from_fn(|_ability| rng.random())),
            item_change_numbers: from_fn(|_side| from_fn(|_item| rng.random())),
            attack_change_numbers: from_fn(|_side| from_fn(|_change| rng.random())),
            defense_change_numbers: from_fn(|_side| from_fn(|_change| rng.random())),
            special_attack_change_numbers: from_fn(|_side| from_fn(|_change| rng.random())),
            special_defense_change_numbers: from_fn(|_side| from_fn(|_change| rng.random())),
            speed_change_numbers: from_fn(|_side| from_fn(|_change| rng.random())),
            move_disabled_numbers: from_fn(|_side| from_fn(|_move| from_fn(|_state| rng.random()))),
            change_wish_counter_numbers: from_fn(|_side| from_fn(|_counter| rng.random())),
            change_wish_health_numbers: from_fn(|_side| from_fn(|_hp| rng.random())),
            future_sight_counter_numbers: from_fn(|_side| from_fn(|_turns| rng.random())),
            future_sight_index_numbers: from_fn(|_side| from_fn(|_index| rng.random())),
        }
    }
    pub fn xor(&mut self, value: u64) {
        self.hash ^= value;
    }

    pub fn update_hash_switch(&mut self, side: usize, active_index: usize) {
        self.xor(self.switch_numbers[side][active_index]);
    }

    pub fn update_hash_terastallize(&mut self, side: usize) {
        self.xor(self.terastallized_numbers[side]);
    }

    pub fn update_hash_health(&mut self, side: usize, slot: usize, hp: usize) {
        self.xor(self.health_numbers[side][slot][hp]);
    }

    pub fn update_hash_volatile_status(&mut self, side: usize, status_index: usize) {
        self.xor(self.volatile_status_numbers[side][status_index]);
    }

    pub fn update_hash_status(&mut self, side: usize, slot: usize, status_index: usize) {
        self.xor(self.status_numbers[side][slot][status_index]);
    }

    pub fn update_hash_boost(&mut self, side: usize, boost_index: usize, amount_index: usize) {
        self.xor(self.boost_numbers[side][boost_index][amount_index]);
    }

    pub fn update_hash_side_condition(
        &mut self,
        side: usize,
        condition_index: usize,
        amount: usize,
    ) {
        self.xor(self.side_condition_numbers[side][condition_index][amount]);
    }

    pub fn update_hash_volatile_duration(
        &mut self,
        side: usize,
        volatile_index: usize,
        duration: usize,
    ) {
        self.xor(self.volatile_duration_numbers[side][volatile_index][duration]);
    }

    pub fn update_hash_weather(&mut self, weather_index: usize, turns_remaining_index: usize) {
        self.xor(self.weather_numbers[weather_index][turns_remaining_index]);
    }

    pub fn update_hash_terrain(&mut self, terrain_index: usize, turns_remaining_index: usize) {
        self.xor(self.terrain_numbers[terrain_index][turns_remaining_index]);
    }

    pub fn update_hash_type_change(
        &mut self,
        side: usize,
        type_1_index: usize,
        type_2_index: usize,
    ) {
        self.xor(self.type_change_numbers[side][type_1_index][type_2_index]);
    }

    pub fn update_hash_ability_change(&mut self, side: usize, ability_index: usize) {
        self.xor(self.ability_change_numbers[side][ability_index]);
    }

    pub fn update_hash_item_change(&mut self, side: usize, item_index: usize) {
        self.xor(self.item_change_numbers[side][item_index]);
    }

    pub fn update_hash_attack_change(&mut self, side: usize, change_index: usize) {
        self.xor(self.attack_change_numbers[side][change_index]);
    }

    pub fn update_hash_defense_change(&mut self, side: usize, change_index: usize) {
        self.xor(self.defense_change_numbers[side][change_index]);
    }

    pub fn update_hash_special_attack_change(&mut self, side: usize, change_index: usize) {
        self.xor(self.special_attack_change_numbers[side][change_index]);
    }

    pub fn update_hash_special_defense_change(&mut self, side: usize, change_index: usize) {
        self.xor(self.special_defense_change_numbers[side][change_index]);
    }

    pub fn update_hash_speed_change(&mut self, side: usize, change_index: usize) {
        self.xor(self.speed_change_numbers[side][change_index]);
    }

    pub fn update_hash_move_disabled(
        &mut self,
        side: usize,
        move_index: usize,
        state_index: usize,
    ) {
        self.xor(self.move_disabled_numbers[side][move_index][state_index]);
    }

    pub fn update_hash_change_wish_counter(&mut self, side: usize, counter_index: usize) {
        self.xor(self.change_wish_counter_numbers[side][counter_index]);
    }

    pub fn update_hash_change_wish_health(&mut self, side: usize, hp_index: usize) {
        self.xor(self.change_wish_health_numbers[side][hp_index]);
    }

    pub fn update_hash_change_future_sight_counter(&mut self, side: usize, turns_index: usize) {
        self.xor(self.future_sight_counter_numbers[side][turns_index]);
    }

    pub fn update_hash_future_sight_pkmn_index(&mut self, side: usize, index: usize) {
        self.xor(self.future_sight_index_numbers[side][index]);
    }
}
