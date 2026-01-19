use rand::Rng;
use std::array::from_fn;

#[derive(Clone, Debug)]
pub struct StateHash {
    hash: u64,

    // Precomputed random numbers for each side/slot combination
    health_numbers: [[[u64; 715]; 6]; 2], // hp values from 0 to 714 for 6 pokemon on 2 sides
    switch_numbers: [[u64; 6]; 2],
    terastallized_numbers: [u64; 2],
}

impl Default for StateHash {
    fn default() -> Self {
        StateHash {
            hash: 0,
            health_numbers: [[[0u64; 715]; 6]; 2],
            switch_numbers: [[0u64; 6], [0u64; 6]],
            terastallized_numbers: [0u64; 2],
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
}
