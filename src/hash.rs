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
}
