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
        }
    }
    pub fn xor(&mut self, value: u64) {
        self.hash ^= value;
    }

    // SWITCH
    fn get_zobrist_switch(&self, side: usize, active_index: usize) -> u64 {
        self.switch_numbers[side][active_index]
    }
    pub fn update_hash_switch(&mut self, side: usize, active_index: usize) {
        self.xor(self.get_zobrist_switch(side, active_index));
    }

    // TERASTALLIZE
    fn get_zobrist_terastallize(&self, side: usize) -> u64 {
        self.terastallized_numbers[side]
    }
    pub fn update_hash_terastallize(&mut self, side: usize) {
        self.xor(self.get_zobrist_terastallize(side));
    }

    // HEALTH
    fn get_zobrist_health(&self, side: usize, slot: usize, hp: usize) -> u64 {
        self.health_numbers[side][slot][hp]
    }
    pub fn update_hash_health(&mut self, side: usize, slot: usize, hp: usize) {
        self.xor(self.get_zobrist_health(side, slot, hp));
    }

    // VOLATILE STATUSES
    fn get_zobrist_volatile_status(&self, side: usize, status_index: usize) -> u64 {
        self.volatile_status_numbers[side][status_index]
    }
    pub fn update_hash_volatile_status(&mut self, side: usize, status_index: usize) {
        self.xor(self.get_zobrist_volatile_status(side, status_index));
    }

    // STATUS
    fn get_zobrist_status(&self, side: usize, slot: usize, status_index: usize) -> u64 {
        self.status_numbers[side][slot][status_index]
    }
    pub fn update_hash_status(&mut self, side: usize, slot: usize, status_index: usize) {
        self.xor(self.get_zobrist_status(side, slot, status_index));
    }

    // BOOSTS
    fn get_zobrist_boost(&self, side: usize, boost_index: usize, amount_index: usize) -> u64 {
        self.boost_numbers[side][boost_index][amount_index]
    }
    pub fn update_hash_boost(&mut self, side: usize, boost_index: usize, amount_index: usize) {
        self.xor(self.get_zobrist_boost(side, boost_index, amount_index));
    }
}
