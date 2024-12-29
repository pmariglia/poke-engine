use crate::pokemon::PokemonName;

impl PokemonName {
    /*
    Base Stats are only required to re-calculate stats when a pokemon changes forme
    so not every pokemon will be here
    */
    pub fn base_stats(&self) -> (i16, i16, i16, i16, i16, i16) {
        match self {
            PokemonName::MINIOR => (60, 100, 60, 100, 60, 120),
            PokemonName::MINIORMETEOR => (60, 60, 100, 60, 100, 60),
            _ => panic!("Base stats not implemented for {}", self),
        }
    }
}
