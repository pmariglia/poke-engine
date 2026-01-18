use rand::Rng;

#[derive(Clone, Debug)]
pub struct StateHash {
    hash: u64,

    // Precomputed random numbers for each side/slot combination
    switch_numbers: [[u64; 6]; 2],
}

impl Default for StateHash {
    fn default() -> Self {
        StateHash {
            hash: 0,
            switch_numbers: [[0u64; 6], [0u64; 6]],
        }
    }
}

impl StateHash {
    pub fn get_hash(&self) -> u64 {
        self.hash
    }
    pub fn new_random() -> Self {
        let mut rng = rand::rng();

        StateHash {
            hash: rng.random(),
            switch_numbers: [
                std::array::from_fn(|_| rng.random()),
                std::array::from_fn(|_| rng.random()),
            ],
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
}
