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
            PokemonName::WISHIWASHI => (45, 20, 20, 25, 25, 40),
            PokemonName::WISHIWASHISCHOOL => (45, 140, 130, 140, 135, 30),
            PokemonName::PALAFIN => (100, 70, 72, 53, 62, 100),
            PokemonName::PALAFINHERO => (100, 160, 97, 106, 87, 100),
            PokemonName::EISCUE => (75, 80, 110, 65, 90, 50),
            PokemonName::EISCUENOICE => (75, 80, 70, 65, 50, 130),
            _ => panic!("Base stats not implemented for {}", self),
        }
    }
}
