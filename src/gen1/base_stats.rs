use crate::pokemon::PokemonName;

impl PokemonName {
    pub fn base_speed(&self) -> i16 {
        match self {
            PokemonName::NONE => 0,
            PokemonName::BULBASAUR => 45,
            PokemonName::IVYSAUR => 60,
            PokemonName::VENUSAUR => 80,
            PokemonName::CHARMANDER => 65,
            PokemonName::CHARMELEON => 80,
            PokemonName::CHARIZARD => 100,
            PokemonName::SQUIRTLE => 43,
            PokemonName::WARTORTLE => 58,
            PokemonName::BLASTOISE => 78,
            PokemonName::CATERPIE => 45,
            PokemonName::METAPOD => 30,
            PokemonName::BUTTERFREE => 70,
            PokemonName::WEEDLE => 50,
            PokemonName::KAKUNA => 35,
            PokemonName::BEEDRILL => 75,
            PokemonName::PIDGEY => 56,
            PokemonName::PIDGEOTTO => 71,
            PokemonName::PIDGEOT => 91,
            PokemonName::RATTATA => 72,
            PokemonName::RATICATE => 97,
            PokemonName::SPEAROW => 70,
            PokemonName::FEAROW => 100,
            PokemonName::EKANS => 55,
            PokemonName::ARBOK => 80,
            PokemonName::PIKACHU => 90,
            PokemonName::RAICHU => 100,
            PokemonName::SANDSHREW => 40,
            PokemonName::SANDSLASH => 65,
            PokemonName::NIDORANF => 41,
            PokemonName::NIDORINA => 56,
            PokemonName::NIDOQUEEN => 76,
            PokemonName::NIDORANM => 50,
            PokemonName::NIDORINO => 65,
            PokemonName::NIDOKING => 85,
            PokemonName::CLEFAIRY => 35,
            PokemonName::CLEFABLE => 60,
            PokemonName::VULPIX => 65,
            PokemonName::NINETALES => 100,
            PokemonName::JIGGLYPUFF => 20,
            PokemonName::WIGGLYTUFF => 45,
            PokemonName::ZUBAT => 55,
            PokemonName::GOLBAT => 90,
            PokemonName::ODDISH => 30,
            PokemonName::GLOOM => 40,
            PokemonName::VILEPLUME => 50,
            PokemonName::PARAS => 25,
            PokemonName::PARASECT => 30,
            PokemonName::VENONAT => 45,
            PokemonName::VENOMOTH => 90,
            PokemonName::DIGLETT => 95,
            PokemonName::DUGTRIO => 120,
            PokemonName::MEOWTH => 90,
            PokemonName::PERSIAN => 115,
            PokemonName::PSYDUCK => 55,
            PokemonName::GOLDUCK => 85,
            PokemonName::MANKEY => 70,
            PokemonName::PRIMEAPE => 95,
            PokemonName::GROWLITHE => 60,
            PokemonName::ARCANINE => 95,
            PokemonName::POLIWAG => 90,
            PokemonName::POLIWHIRL => 90,
            PokemonName::POLIWRATH => 70,
            PokemonName::ABRA => 90,
            PokemonName::KADABRA => 105,
            PokemonName::ALAKAZAM => 120,
            PokemonName::MACHOP => 35,
            PokemonName::MACHOKE => 45,
            PokemonName::MACHAMP => 55,
            PokemonName::BELLSPROUT => 40,
            PokemonName::WEEPINBELL => 55,
            PokemonName::VICTREEBEL => 70,
            PokemonName::TENTACOOL => 70,
            PokemonName::TENTACRUEL => 100,
            PokemonName::GEODUDE => 20,
            PokemonName::GRAVELER => 35,
            PokemonName::GOLEM => 45,
            PokemonName::PONYTA => 90,
            PokemonName::RAPIDASH => 105,
            PokemonName::SLOWPOKE => 15,
            PokemonName::SLOWBRO => 30,
            PokemonName::MAGNEMITE => 45,
            PokemonName::MAGNETON => 70,
            PokemonName::FARFETCHD => 52,
            PokemonName::DODUO => 75,
            PokemonName::DODRIO => 100,
            PokemonName::SEEL => 45,
            PokemonName::DEWGONG => 70,
            PokemonName::GRIMER => 25,
            PokemonName::MUK => 50,
            PokemonName::SHELLDER => 40,
            PokemonName::CLOYSTER => 70,
            PokemonName::GASTLY => 80,
            PokemonName::HAUNTER => 95,
            PokemonName::GENGAR => 110,
            PokemonName::ONIX => 70,
            PokemonName::DROWZEE => 42,
            PokemonName::HYPNO => 67,
            PokemonName::KRABBY => 50,
            PokemonName::KINGLER => 75,
            PokemonName::VOLTORB => 100,
            PokemonName::ELECTRODE => 140,
            PokemonName::EXEGGCUTE => 40,
            PokemonName::EXEGGUTOR => 55,
            PokemonName::CUBONE => 35,
            PokemonName::MAROWAK => 45,
            PokemonName::HITMONLEE => 87,
            PokemonName::HITMONCHAN => 76,
            PokemonName::LICKITUNG => 30,
            PokemonName::KOFFING => 35,
            PokemonName::WEEZING => 60,
            PokemonName::RHYHORN => 25,
            PokemonName::RHYDON => 40,
            PokemonName::CHANSEY => 50,
            PokemonName::TANGELA => 60,
            PokemonName::KANGASKHAN => 90,
            PokemonName::HORSEA => 60,
            PokemonName::SEADRA => 85,
            PokemonName::GOLDEEN => 63,
            PokemonName::SEAKING => 68,
            PokemonName::STARYU => 85,
            PokemonName::STARMIE => 115,
            PokemonName::MRMIME => 90,
            PokemonName::SCYTHER => 105,
            PokemonName::JYNX => 95,
            PokemonName::ELECTABUZZ => 105,
            PokemonName::MAGMAR => 93,
            PokemonName::PINSIR => 85,
            PokemonName::TAUROS => 110,
            PokemonName::MAGIKARP => 80,
            PokemonName::GYARADOS => 81,
            PokemonName::LAPRAS => 60,
            PokemonName::DITTO => 48,
            PokemonName::EEVEE => 55,
            PokemonName::VAPOREON => 65,
            PokemonName::JOLTEON => 130,
            PokemonName::FLAREON => 65,
            PokemonName::PORYGON => 40,
            PokemonName::OMANYTE => 35,
            PokemonName::OMASTAR => 55,
            PokemonName::KABUTO => 55,
            PokemonName::KABUTOPS => 80,
            PokemonName::AERODACTYL => 130,
            PokemonName::SNORLAX => 30,
            PokemonName::ARTICUNO => 85,
            PokemonName::ZAPDOS => 100,
            PokemonName::MOLTRES => 90,
            PokemonName::DRATINI => 50,
            PokemonName::DRAGONAIR => 70,
            PokemonName::DRAGONITE => 80,
            PokemonName::MEWTWO => 130,
            PokemonName::MEW => 100,
            _ => panic!("Invalid PokemonName for Gen1: {:?}", self),
        }
    }
}
