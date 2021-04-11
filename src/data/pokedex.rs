use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::state::PokemonTypes;

lazy_static! {
    static ref POKEDEX: HashMap<String, PokedexPokemon> = {        
        let mut pokedex: HashMap<String, PokedexPokemon> = HashMap::new();

        pokedex.insert(
            "bulbasaur".to_string(),
            PokedexPokemon {
                species: "bulbasaur".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 49,
                    defense: 49,
                    special_attack: 65,
                    special_defense: 65,
                    speed: 45
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "chlorophyll".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 6.9 as f32,
            }
        );
        
        pokedex.insert(
            "ivysaur".to_string(),
            PokedexPokemon {
                species: "ivysaur".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 62,
                    defense: 63,
                    special_attack: 80,
                    special_defense: 80,
                    speed: 60
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "chlorophyll".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 13 as f32,
            }
        );
        
        pokedex.insert(
            "venusaur".to_string(),
            PokedexPokemon {
                species: "venusaur".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 82,
                    defense: 83,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 80
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "chlorophyll".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 100 as f32,
            }
        );
        
        pokedex.insert(
            "venusaurmega".to_string(),
            PokedexPokemon {
                species: "venusaurmega".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 100,
                    defense: 123,
                    special_attack: 122,
                    special_defense: 120,
                    speed: 80
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 155.5 as f32,
            }
        );
        
        pokedex.insert(
            "charmander".to_string(),
            PokedexPokemon {
                species: "charmander".to_string(),
                base_stats: BaseStats {
                    hp: 39,
                    attack: 52,
                    defense: 43,
                    special_attack: 60,
                    special_defense: 50,
                    speed: 65
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "solarpower".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "charmeleon".to_string(),
            PokedexPokemon {
                species: "charmeleon".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 64,
                    defense: 58,
                    special_attack: 80,
                    special_defense: 65,
                    speed: 80
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "solarpower".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 19 as f32,
            }
        );
        
        pokedex.insert(
            "charizard".to_string(),
            PokedexPokemon {
                species: "charizard".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 84,
                    defense: 78,
                    special_attack: 109,
                    special_defense: 85,
                    speed: 100
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "solarpower".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Flying
                ),
                weight: 90.5 as f32,
            }
        );
        
        pokedex.insert(
            "charizardmegax".to_string(),
            PokedexPokemon {
                species: "charizardmegax".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 130,
                    defense: 111,
                    special_attack: 130,
                    special_defense: 85,
                    speed: 100
                },
                abilities: Abilities {
                    first: "toughclaws".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Dragon
                ),
                weight: 110.5 as f32,
            }
        );
        
        pokedex.insert(
            "charizardmegay".to_string(),
            PokedexPokemon {
                species: "charizardmegay".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 104,
                    defense: 78,
                    special_attack: 159,
                    special_defense: 115,
                    speed: 100
                },
                abilities: Abilities {
                    first: "drought".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Flying
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "squirtle".to_string(),
            PokedexPokemon {
                species: "squirtle".to_string(),
                base_stats: BaseStats {
                    hp: 44,
                    attack: 48,
                    defense: 65,
                    special_attack: 50,
                    special_defense: 64,
                    speed: 43
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 9 as f32,
            }
        );
        
        pokedex.insert(
            "wartortle".to_string(),
            PokedexPokemon {
                species: "wartortle".to_string(),
                base_stats: BaseStats {
                    hp: 59,
                    attack: 63,
                    defense: 80,
                    special_attack: 65,
                    special_defense: 80,
                    speed: 58
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 22.5 as f32,
            }
        );
        
        pokedex.insert(
            "blastoise".to_string(),
            PokedexPokemon {
                species: "blastoise".to_string(),
                base_stats: BaseStats {
                    hp: 79,
                    attack: 83,
                    defense: 100,
                    special_attack: 85,
                    special_defense: 105,
                    speed: 78
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 85.5 as f32,
            }
        );
        
        pokedex.insert(
            "blastoisemega".to_string(),
            PokedexPokemon {
                species: "blastoisemega".to_string(),
                base_stats: BaseStats {
                    hp: 79,
                    attack: 103,
                    defense: 120,
                    special_attack: 135,
                    special_defense: 115,
                    speed: 78
                },
                abilities: Abilities {
                    first: "megalauncher".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 101.1 as f32,
            }
        );
        
        pokedex.insert(
            "caterpie".to_string(),
            PokedexPokemon {
                species: "caterpie".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 30,
                    defense: 35,
                    special_attack: 20,
                    special_defense: 20,
                    speed: 45
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "none".to_string(),
                    hidden: "runaway".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 2.9 as f32,
            }
        );
        
        pokedex.insert(
            "metapod".to_string(),
            PokedexPokemon {
                species: "metapod".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 20,
                    defense: 55,
                    special_attack: 25,
                    special_defense: 25,
                    speed: 30
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 9.9 as f32,
            }
        );
        
        pokedex.insert(
            "butterfree".to_string(),
            PokedexPokemon {
                species: "butterfree".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 45,
                    defense: 50,
                    special_attack: 90,
                    special_defense: 80,
                    speed: 70
                },
                abilities: Abilities {
                    first: "compoundeyes".to_string(),
                    second: "none".to_string(),
                    hidden: "tintedlens".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 32 as f32,
            }
        );
        
        pokedex.insert(
            "weedle".to_string(),
            PokedexPokemon {
                species: "weedle".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 35,
                    defense: 30,
                    special_attack: 20,
                    special_defense: 20,
                    speed: 50
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "none".to_string(),
                    hidden: "runaway".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 3.2 as f32,
            }
        );
        
        pokedex.insert(
            "kakuna".to_string(),
            PokedexPokemon {
                species: "kakuna".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 25,
                    defense: 50,
                    special_attack: 25,
                    special_defense: 25,
                    speed: 35
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 10 as f32,
            }
        );
        
        pokedex.insert(
            "beedrill".to_string(),
            PokedexPokemon {
                species: "beedrill".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 90,
                    defense: 40,
                    special_attack: 45,
                    special_defense: 80,
                    speed: 75
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "none".to_string(),
                    hidden: "sniper".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 29.5 as f32,
            }
        );
        
        pokedex.insert(
            "beedrillmega".to_string(),
            PokedexPokemon {
                species: "beedrillmega".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 150,
                    defense: 40,
                    special_attack: 15,
                    special_defense: 80,
                    speed: 145
                },
                abilities: Abilities {
                    first: "adaptability".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 40.5 as f32,
            }
        );
        
        pokedex.insert(
            "pidgey".to_string(),
            PokedexPokemon {
                species: "pidgey".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 40,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 56
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "tangledfeet".to_string(),
                    hidden: "bigpecks".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 1.8 as f32,
            }
        );
        
        pokedex.insert(
            "pidgeotto".to_string(),
            PokedexPokemon {
                species: "pidgeotto".to_string(),
                base_stats: BaseStats {
                    hp: 63,
                    attack: 60,
                    defense: 55,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 71
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "tangledfeet".to_string(),
                    hidden: "bigpecks".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "pidgeot".to_string(),
            PokedexPokemon {
                species: "pidgeot".to_string(),
                base_stats: BaseStats {
                    hp: 83,
                    attack: 80,
                    defense: 75,
                    special_attack: 70,
                    special_defense: 70,
                    speed: 101
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "tangledfeet".to_string(),
                    hidden: "bigpecks".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 39.5 as f32,
            }
        );
        
        pokedex.insert(
            "pidgeotmega".to_string(),
            PokedexPokemon {
                species: "pidgeotmega".to_string(),
                base_stats: BaseStats {
                    hp: 83,
                    attack: 80,
                    defense: 80,
                    special_attack: 135,
                    special_defense: 80,
                    speed: 121
                },
                abilities: Abilities {
                    first: "noguard".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 50.5 as f32,
            }
        );
        
        pokedex.insert(
            "rattata".to_string(),
            PokedexPokemon {
                species: "rattata".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 56,
                    defense: 35,
                    special_attack: 25,
                    special_defense: 35,
                    speed: 72
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "guts".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 3.5 as f32,
            }
        );
        
        pokedex.insert(
            "rattataalola".to_string(),
            PokedexPokemon {
                species: "rattataalola".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 56,
                    defense: 35,
                    special_attack: 25,
                    special_defense: 35,
                    speed: 72
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "hustle".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Normal
                ),
                weight: 3.8 as f32,
            }
        );
        
        pokedex.insert(
            "raticate".to_string(),
            PokedexPokemon {
                species: "raticate".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 81,
                    defense: 60,
                    special_attack: 50,
                    special_defense: 70,
                    speed: 97
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "guts".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 18.5 as f32,
            }
        );
        
        pokedex.insert(
            "raticatealola".to_string(),
            PokedexPokemon {
                species: "raticatealola".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 71,
                    defense: 70,
                    special_attack: 40,
                    special_defense: 80,
                    speed: 77
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "hustle".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Normal
                ),
                weight: 25.5 as f32,
            }
        );
        
        pokedex.insert(
            "spearow".to_string(),
            PokedexPokemon {
                species: "spearow".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 60,
                    defense: 30,
                    special_attack: 31,
                    special_defense: 31,
                    speed: 70
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "none".to_string(),
                    hidden: "sniper".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "fearow".to_string(),
            PokedexPokemon {
                species: "fearow".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 90,
                    defense: 65,
                    special_attack: 61,
                    special_defense: 61,
                    speed: 100
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "none".to_string(),
                    hidden: "sniper".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 38 as f32,
            }
        );
        
        pokedex.insert(
            "ekans".to_string(),
            PokedexPokemon {
                species: "ekans".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 60,
                    defense: 44,
                    special_attack: 40,
                    special_defense: 54,
                    speed: 55
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "shedskin".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 6.9 as f32,
            }
        );
        
        pokedex.insert(
            "arbok".to_string(),
            PokedexPokemon {
                species: "arbok".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 95,
                    defense: 69,
                    special_attack: 65,
                    special_defense: 79,
                    speed: 80
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "shedskin".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 65 as f32,
            }
        );
        
        pokedex.insert(
            "pikachu".to_string(),
            PokedexPokemon {
                species: "pikachu".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "lightningrod".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachucosplay".to_string(),
            PokedexPokemon {
                species: "pikachucosplay".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachurockstar".to_string(),
            PokedexPokemon {
                species: "pikachurockstar".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachubelle".to_string(),
            PokedexPokemon {
                species: "pikachubelle".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachupopstar".to_string(),
            PokedexPokemon {
                species: "pikachupopstar".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachuphd".to_string(),
            PokedexPokemon {
                species: "pikachuphd".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachulibre".to_string(),
            PokedexPokemon {
                species: "pikachulibre".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "raichu".to_string(),
            PokedexPokemon {
                species: "raichu".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 90,
                    defense: 55,
                    special_attack: 90,
                    special_defense: 80,
                    speed: 110
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "lightningrod".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "raichualola".to_string(),
            PokedexPokemon {
                species: "raichualola".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 85,
                    defense: 50,
                    special_attack: 95,
                    special_defense: 85,
                    speed: 110
                },
                abilities: Abilities {
                    first: "surgesurfer".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Psychic
                ),
                weight: 21 as f32,
            }
        );
        
        pokedex.insert(
            "sandshrew".to_string(),
            PokedexPokemon {
                species: "sandshrew".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 75,
                    defense: 85,
                    special_attack: 20,
                    special_defense: 30,
                    speed: 40
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "none".to_string(),
                    hidden: "sandrush".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "sandshrewalola".to_string(),
            PokedexPokemon {
                species: "sandshrewalola".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 75,
                    defense: 90,
                    special_attack: 10,
                    special_defense: 35,
                    speed: 40
                },
                abilities: Abilities {
                    first: "snowcloak".to_string(),
                    second: "none".to_string(),
                    hidden: "slushrush".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Steel
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "sandslash".to_string(),
            PokedexPokemon {
                species: "sandslash".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 100,
                    defense: 110,
                    special_attack: 45,
                    special_defense: 55,
                    speed: 65
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "none".to_string(),
                    hidden: "sandrush".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 29.5 as f32,
            }
        );
        
        pokedex.insert(
            "sandslashalola".to_string(),
            PokedexPokemon {
                species: "sandslashalola".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 100,
                    defense: 120,
                    special_attack: 25,
                    special_defense: 65,
                    speed: 65
                },
                abilities: Abilities {
                    first: "snowcloak".to_string(),
                    second: "none".to_string(),
                    hidden: "slushrush".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Steel
                ),
                weight: 55 as f32,
            }
        );
        
        pokedex.insert(
            "nidoranf".to_string(),
            PokedexPokemon {
                species: "nidoranf".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 47,
                    defense: 52,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 41
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "rivalry".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 7 as f32,
            }
        );
        
        pokedex.insert(
            "nidorina".to_string(),
            PokedexPokemon {
                species: "nidorina".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 62,
                    defense: 67,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 56
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "rivalry".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 20 as f32,
            }
        );
        
        pokedex.insert(
            "nidoqueen".to_string(),
            PokedexPokemon {
                species: "nidoqueen".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 92,
                    defense: 87,
                    special_attack: 75,
                    special_defense: 85,
                    speed: 76
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "rivalry".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Ground
                ),
                weight: 60 as f32,
            }
        );
        
        pokedex.insert(
            "nidoranm".to_string(),
            PokedexPokemon {
                species: "nidoranm".to_string(),
                base_stats: BaseStats {
                    hp: 46,
                    attack: 57,
                    defense: 40,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 50
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "rivalry".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 9 as f32,
            }
        );
        
        pokedex.insert(
            "nidorino".to_string(),
            PokedexPokemon {
                species: "nidorino".to_string(),
                base_stats: BaseStats {
                    hp: 61,
                    attack: 72,
                    defense: 57,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 65
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "rivalry".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 19.5 as f32,
            }
        );
        
        pokedex.insert(
            "nidoking".to_string(),
            PokedexPokemon {
                species: "nidoking".to_string(),
                base_stats: BaseStats {
                    hp: 81,
                    attack: 102,
                    defense: 77,
                    special_attack: 85,
                    special_defense: 75,
                    speed: 85
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "rivalry".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Ground
                ),
                weight: 62 as f32,
            }
        );
        
        pokedex.insert(
            "clefairy".to_string(),
            PokedexPokemon {
                species: "clefairy".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 45,
                    defense: 48,
                    special_attack: 60,
                    special_defense: 65,
                    speed: 35
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "magicguard".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 7.5 as f32,
            }
        );
        
        pokedex.insert(
            "clefable".to_string(),
            PokedexPokemon {
                species: "clefable".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 70,
                    defense: 73,
                    special_attack: 95,
                    special_defense: 90,
                    speed: 60
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "magicguard".to_string(),
                    hidden: "unaware".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "vulpix".to_string(),
            PokedexPokemon {
                species: "vulpix".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 41,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 65,
                    speed: 65
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "none".to_string(),
                    hidden: "drought".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 9.9 as f32,
            }
        );
        
        pokedex.insert(
            "vulpixalola".to_string(),
            PokedexPokemon {
                species: "vulpixalola".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 41,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 65,
                    speed: 65
                },
                abilities: Abilities {
                    first: "snowcloak".to_string(),
                    second: "none".to_string(),
                    hidden: "snowwarning".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 9.9 as f32,
            }
        );
        
        pokedex.insert(
            "ninetales".to_string(),
            PokedexPokemon {
                species: "ninetales".to_string(),
                base_stats: BaseStats {
                    hp: 73,
                    attack: 76,
                    defense: 75,
                    special_attack: 81,
                    special_defense: 100,
                    speed: 100
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "none".to_string(),
                    hidden: "drought".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 19.9 as f32,
            }
        );
        
        pokedex.insert(
            "ninetalesalola".to_string(),
            PokedexPokemon {
                species: "ninetalesalola".to_string(),
                base_stats: BaseStats {
                    hp: 73,
                    attack: 67,
                    defense: 75,
                    special_attack: 81,
                    special_defense: 100,
                    speed: 109
                },
                abilities: Abilities {
                    first: "snowcloak".to_string(),
                    second: "none".to_string(),
                    hidden: "snowwarning".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Fairy
                ),
                weight: 19.9 as f32,
            }
        );
        
        pokedex.insert(
            "jigglypuff".to_string(),
            PokedexPokemon {
                species: "jigglypuff".to_string(),
                base_stats: BaseStats {
                    hp: 115,
                    attack: 45,
                    defense: 20,
                    special_attack: 45,
                    special_defense: 25,
                    speed: 20
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "competitive".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fairy
                ),
                weight: 5.5 as f32,
            }
        );
        
        pokedex.insert(
            "wigglytuff".to_string(),
            PokedexPokemon {
                species: "wigglytuff".to_string(),
                base_stats: BaseStats {
                    hp: 140,
                    attack: 70,
                    defense: 45,
                    special_attack: 85,
                    special_defense: 50,
                    speed: 45
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "competitive".to_string(),
                    hidden: "frisk".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fairy
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "zubat".to_string(),
            PokedexPokemon {
                species: "zubat".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 35,
                    special_attack: 30,
                    special_defense: 40,
                    speed: 55
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "none".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Flying
                ),
                weight: 7.5 as f32,
            }
        );
        
        pokedex.insert(
            "golbat".to_string(),
            PokedexPokemon {
                species: "golbat".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 80,
                    defense: 70,
                    special_attack: 65,
                    special_defense: 75,
                    speed: 90
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "none".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Flying
                ),
                weight: 55 as f32,
            }
        );
        
        pokedex.insert(
            "oddish".to_string(),
            PokedexPokemon {
                species: "oddish".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 50,
                    defense: 55,
                    special_attack: 75,
                    special_defense: 65,
                    speed: 30
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "runaway".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 5.4 as f32,
            }
        );
        
        pokedex.insert(
            "gloom".to_string(),
            PokedexPokemon {
                species: "gloom".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 65,
                    defense: 70,
                    special_attack: 85,
                    special_defense: 75,
                    speed: 40
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "stench".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 8.6 as f32,
            }
        );
        
        pokedex.insert(
            "vileplume".to_string(),
            PokedexPokemon {
                species: "vileplume".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 80,
                    defense: 85,
                    special_attack: 110,
                    special_defense: 90,
                    speed: 50
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "effectspore".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 18.6 as f32,
            }
        );
        
        pokedex.insert(
            "paras".to_string(),
            PokedexPokemon {
                species: "paras".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 70,
                    defense: 55,
                    special_attack: 45,
                    special_defense: 55,
                    speed: 25
                },
                abilities: Abilities {
                    first: "effectspore".to_string(),
                    second: "dryskin".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Grass
                ),
                weight: 5.4 as f32,
            }
        );
        
        pokedex.insert(
            "parasect".to_string(),
            PokedexPokemon {
                species: "parasect".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 95,
                    defense: 80,
                    special_attack: 60,
                    special_defense: 80,
                    speed: 30
                },
                abilities: Abilities {
                    first: "effectspore".to_string(),
                    second: "dryskin".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Grass
                ),
                weight: 29.5 as f32,
            }
        );
        
        pokedex.insert(
            "venonat".to_string(),
            PokedexPokemon {
                species: "venonat".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 55,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 55,
                    speed: 45
                },
                abilities: Abilities {
                    first: "compoundeyes".to_string(),
                    second: "tintedlens".to_string(),
                    hidden: "runaway".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "venomoth".to_string(),
            PokedexPokemon {
                species: "venomoth".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 65,
                    defense: 60,
                    special_attack: 90,
                    special_defense: 75,
                    speed: 90
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "tintedlens".to_string(),
                    hidden: "wonderskin".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 12.5 as f32,
            }
        );
        
        pokedex.insert(
            "diglett".to_string(),
            PokedexPokemon {
                species: "diglett".to_string(),
                base_stats: BaseStats {
                    hp: 10,
                    attack: 55,
                    defense: 25,
                    special_attack: 35,
                    special_defense: 45,
                    speed: 95
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "arenatrap".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 0.8 as f32,
            }
        );
        
        pokedex.insert(
            "diglettalola".to_string(),
            PokedexPokemon {
                species: "diglettalola".to_string(),
                base_stats: BaseStats {
                    hp: 10,
                    attack: 55,
                    defense: 30,
                    special_attack: 35,
                    special_defense: 45,
                    speed: 90
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "tanglinghair".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Steel
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "dugtrio".to_string(),
            PokedexPokemon {
                species: "dugtrio".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 100,
                    defense: 50,
                    special_attack: 50,
                    special_defense: 70,
                    speed: 120
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "arenatrap".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 33.3 as f32,
            }
        );
        
        pokedex.insert(
            "dugtrioalola".to_string(),
            PokedexPokemon {
                species: "dugtrioalola".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 100,
                    defense: 60,
                    special_attack: 50,
                    special_defense: 70,
                    speed: 110
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "tanglinghair".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Steel
                ),
                weight: 66.6 as f32,
            }
        );
        
        pokedex.insert(
            "meowth".to_string(),
            PokedexPokemon {
                species: "meowth".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 35,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 90
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "technician".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 4.2 as f32,
            }
        );
        
        pokedex.insert(
            "meowthalola".to_string(),
            PokedexPokemon {
                species: "meowthalola".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 35,
                    defense: 35,
                    special_attack: 50,
                    special_defense: 40,
                    speed: 90
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "technician".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 4.2 as f32,
            }
        );
        
        pokedex.insert(
            "persian".to_string(),
            PokedexPokemon {
                species: "persian".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 70,
                    defense: 60,
                    special_attack: 65,
                    special_defense: 65,
                    speed: 115
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "technician".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 32 as f32,
            }
        );
        
        pokedex.insert(
            "persianalola".to_string(),
            PokedexPokemon {
                species: "persianalola".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 60,
                    defense: 60,
                    special_attack: 75,
                    special_defense: 65,
                    speed: 115
                },
                abilities: Abilities {
                    first: "furcoat".to_string(),
                    second: "technician".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 33 as f32,
            }
        );
        
        pokedex.insert(
            "psyduck".to_string(),
            PokedexPokemon {
                species: "psyduck".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 52,
                    defense: 48,
                    special_attack: 65,
                    special_defense: 50,
                    speed: 55
                },
                abilities: Abilities {
                    first: "damp".to_string(),
                    second: "cloudnine".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 19.6 as f32,
            }
        );
        
        pokedex.insert(
            "golduck".to_string(),
            PokedexPokemon {
                species: "golduck".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 82,
                    defense: 78,
                    special_attack: 95,
                    special_defense: 80,
                    speed: 85
                },
                abilities: Abilities {
                    first: "damp".to_string(),
                    second: "cloudnine".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 76.6 as f32,
            }
        );
        
        pokedex.insert(
            "mankey".to_string(),
            PokedexPokemon {
                species: "mankey".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 80,
                    defense: 35,
                    special_attack: 35,
                    special_defense: 45,
                    speed: 70
                },
                abilities: Abilities {
                    first: "vitalspirit".to_string(),
                    second: "angerpoint".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "primeape".to_string(),
            PokedexPokemon {
                species: "primeape".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 105,
                    defense: 60,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 95
                },
                abilities: Abilities {
                    first: "vitalspirit".to_string(),
                    second: "angerpoint".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 32 as f32,
            }
        );
        
        pokedex.insert(
            "growlithe".to_string(),
            PokedexPokemon {
                species: "growlithe".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 70,
                    defense: 45,
                    special_attack: 70,
                    special_defense: 50,
                    speed: 60
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "flashfire".to_string(),
                    hidden: "justified".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 19 as f32,
            }
        );
        
        pokedex.insert(
            "arcanine".to_string(),
            PokedexPokemon {
                species: "arcanine".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 110,
                    defense: 80,
                    special_attack: 100,
                    special_defense: 80,
                    speed: 95
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "flashfire".to_string(),
                    hidden: "justified".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 155 as f32,
            }
        );
        
        pokedex.insert(
            "poliwag".to_string(),
            PokedexPokemon {
                species: "poliwag".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 50,
                    defense: 40,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 90
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "damp".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 12.4 as f32,
            }
        );
        
        pokedex.insert(
            "poliwhirl".to_string(),
            PokedexPokemon {
                species: "poliwhirl".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 65,
                    defense: 65,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "damp".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 20 as f32,
            }
        );
        
        pokedex.insert(
            "poliwrath".to_string(),
            PokedexPokemon {
                species: "poliwrath".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 95,
                    defense: 95,
                    special_attack: 70,
                    special_defense: 90,
                    speed: 70
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "damp".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fighting
                ),
                weight: 54 as f32,
            }
        );
        
        pokedex.insert(
            "abra".to_string(),
            PokedexPokemon {
                species: "abra".to_string(),
                base_stats: BaseStats {
                    hp: 25,
                    attack: 20,
                    defense: 15,
                    special_attack: 105,
                    special_defense: 55,
                    speed: 90
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "magicguard".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 19.5 as f32,
            }
        );
        
        pokedex.insert(
            "kadabra".to_string(),
            PokedexPokemon {
                species: "kadabra".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 35,
                    defense: 30,
                    special_attack: 120,
                    special_defense: 70,
                    speed: 105
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "magicguard".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 56.5 as f32,
            }
        );
        
        pokedex.insert(
            "alakazam".to_string(),
            PokedexPokemon {
                species: "alakazam".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 50,
                    defense: 45,
                    special_attack: 135,
                    special_defense: 95,
                    speed: 120
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "magicguard".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 48 as f32,
            }
        );
        
        pokedex.insert(
            "alakazammega".to_string(),
            PokedexPokemon {
                species: "alakazammega".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 50,
                    defense: 65,
                    special_attack: 175,
                    special_defense: 105,
                    speed: 150
                },
                abilities: Abilities {
                    first: "trace".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 48 as f32,
            }
        );
        
        pokedex.insert(
            "machop".to_string(),
            PokedexPokemon {
                species: "machop".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 80,
                    defense: 50,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 35
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "noguard".to_string(),
                    hidden: "steadfast".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 19.5 as f32,
            }
        );
        
        pokedex.insert(
            "machoke".to_string(),
            PokedexPokemon {
                species: "machoke".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 100,
                    defense: 70,
                    special_attack: 50,
                    special_defense: 60,
                    speed: 45
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "noguard".to_string(),
                    hidden: "steadfast".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 70.5 as f32,
            }
        );
        
        pokedex.insert(
            "machamp".to_string(),
            PokedexPokemon {
                species: "machamp".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 130,
                    defense: 80,
                    special_attack: 65,
                    special_defense: 85,
                    speed: 55
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "noguard".to_string(),
                    hidden: "steadfast".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 130 as f32,
            }
        );
        
        pokedex.insert(
            "bellsprout".to_string(),
            PokedexPokemon {
                species: "bellsprout".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 75,
                    defense: 35,
                    special_attack: 70,
                    special_defense: 30,
                    speed: 40
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "weepinbell".to_string(),
            PokedexPokemon {
                species: "weepinbell".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 90,
                    defense: 50,
                    special_attack: 85,
                    special_defense: 45,
                    speed: 55
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 6.4 as f32,
            }
        );
        
        pokedex.insert(
            "victreebel".to_string(),
            PokedexPokemon {
                species: "victreebel".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 105,
                    defense: 65,
                    special_attack: 100,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 15.5 as f32,
            }
        );
        
        pokedex.insert(
            "tentacool".to_string(),
            PokedexPokemon {
                species: "tentacool".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 40,
                    defense: 35,
                    special_attack: 50,
                    special_defense: 100,
                    speed: 70
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "liquidooze".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Poison
                ),
                weight: 45.5 as f32,
            }
        );
        
        pokedex.insert(
            "tentacruel".to_string(),
            PokedexPokemon {
                species: "tentacruel".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 70,
                    defense: 65,
                    special_attack: 80,
                    special_defense: 120,
                    speed: 100
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "liquidooze".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Poison
                ),
                weight: 55 as f32,
            }
        );
        
        pokedex.insert(
            "geodude".to_string(),
            PokedexPokemon {
                species: "geodude".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 80,
                    defense: 100,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 20
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Ground
                ),
                weight: 20 as f32,
            }
        );
        
        pokedex.insert(
            "geodudealola".to_string(),
            PokedexPokemon {
                species: "geodudealola".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 80,
                    defense: 100,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 20
                },
                abilities: Abilities {
                    first: "magnetpull".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "galvanize".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Electric
                ),
                weight: 20.3 as f32,
            }
        );
        
        pokedex.insert(
            "graveler".to_string(),
            PokedexPokemon {
                species: "graveler".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 95,
                    defense: 115,
                    special_attack: 45,
                    special_defense: 45,
                    speed: 35
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Ground
                ),
                weight: 105 as f32,
            }
        );
        
        pokedex.insert(
            "graveleralola".to_string(),
            PokedexPokemon {
                species: "graveleralola".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 95,
                    defense: 115,
                    special_attack: 45,
                    special_defense: 45,
                    speed: 35
                },
                abilities: Abilities {
                    first: "magnetpull".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "galvanize".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Electric
                ),
                weight: 110 as f32,
            }
        );
        
        pokedex.insert(
            "golem".to_string(),
            PokedexPokemon {
                species: "golem".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 120,
                    defense: 130,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 45
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Ground
                ),
                weight: 300 as f32,
            }
        );
        
        pokedex.insert(
            "golemalola".to_string(),
            PokedexPokemon {
                species: "golemalola".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 120,
                    defense: 130,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 45
                },
                abilities: Abilities {
                    first: "magnetpull".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "galvanize".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Electric
                ),
                weight: 316 as f32,
            }
        );
        
        pokedex.insert(
            "ponyta".to_string(),
            PokedexPokemon {
                species: "ponyta".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 85,
                    defense: 55,
                    special_attack: 65,
                    special_defense: 65,
                    speed: 90
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "flashfire".to_string(),
                    hidden: "flamebody".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "rapidash".to_string(),
            PokedexPokemon {
                species: "rapidash".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 100,
                    defense: 70,
                    special_attack: 80,
                    special_defense: 80,
                    speed: 105
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "flashfire".to_string(),
                    hidden: "flamebody".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 95 as f32,
            }
        );
        
        pokedex.insert(
            "slowpoke".to_string(),
            PokedexPokemon {
                species: "slowpoke".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 65,
                    defense: 65,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 15
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Psychic
                ),
                weight: 36 as f32,
            }
        );
        
        pokedex.insert(
            "slowbro".to_string(),
            PokedexPokemon {
                species: "slowbro".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 75,
                    defense: 110,
                    special_attack: 100,
                    special_defense: 80,
                    speed: 30
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Psychic
                ),
                weight: 78.5 as f32,
            }
        );
        
        pokedex.insert(
            "slowbromega".to_string(),
            PokedexPokemon {
                species: "slowbromega".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 75,
                    defense: 180,
                    special_attack: 130,
                    special_defense: 80,
                    speed: 30
                },
                abilities: Abilities {
                    first: "shellarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Psychic
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "magnemite".to_string(),
            PokedexPokemon {
                species: "magnemite".to_string(),
                base_stats: BaseStats {
                    hp: 25,
                    attack: 35,
                    defense: 70,
                    special_attack: 95,
                    special_defense: 55,
                    speed: 45
                },
                abilities: Abilities {
                    first: "magnetpull".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Steel
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "magneton".to_string(),
            PokedexPokemon {
                species: "magneton".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 60,
                    defense: 95,
                    special_attack: 120,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "magnetpull".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Steel
                ),
                weight: 60 as f32,
            }
        );
        
        pokedex.insert(
            "farfetchd".to_string(),
            PokedexPokemon {
                species: "farfetchd".to_string(),
                base_stats: BaseStats {
                    hp: 52,
                    attack: 90,
                    defense: 55,
                    special_attack: 58,
                    special_defense: 62,
                    speed: 60
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "doduo".to_string(),
            PokedexPokemon {
                species: "doduo".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 85,
                    defense: 45,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 75
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "tangledfeet".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 39.2 as f32,
            }
        );
        
        pokedex.insert(
            "dodrio".to_string(),
            PokedexPokemon {
                species: "dodrio".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 110,
                    defense: 70,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 110
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "tangledfeet".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 85.2 as f32,
            }
        );
        
        pokedex.insert(
            "seel".to_string(),
            PokedexPokemon {
                species: "seel".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 45,
                    defense: 55,
                    special_attack: 45,
                    special_defense: 70,
                    speed: 45
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "hydration".to_string(),
                    hidden: "icebody".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 90 as f32,
            }
        );
        
        pokedex.insert(
            "dewgong".to_string(),
            PokedexPokemon {
                species: "dewgong".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 70,
                    defense: 80,
                    special_attack: 70,
                    special_defense: 95,
                    speed: 70
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "hydration".to_string(),
                    hidden: "icebody".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ice
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "grimer".to_string(),
            PokedexPokemon {
                species: "grimer".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 80,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 25
                },
                abilities: Abilities {
                    first: "stench".to_string(),
                    second: "stickyhold".to_string(),
                    hidden: "poisontouch".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "grimeralola".to_string(),
            PokedexPokemon {
                species: "grimeralola".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 80,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 25
                },
                abilities: Abilities {
                    first: "poisontouch".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "powerofalchemy".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Dark
                ),
                weight: 42 as f32,
            }
        );
        
        pokedex.insert(
            "muk".to_string(),
            PokedexPokemon {
                species: "muk".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 105,
                    defense: 75,
                    special_attack: 65,
                    special_defense: 100,
                    speed: 50
                },
                abilities: Abilities {
                    first: "stench".to_string(),
                    second: "stickyhold".to_string(),
                    hidden: "poisontouch".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "mukalola".to_string(),
            PokedexPokemon {
                species: "mukalola".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 105,
                    defense: 75,
                    special_attack: 65,
                    special_defense: 100,
                    speed: 50
                },
                abilities: Abilities {
                    first: "poisontouch".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "powerofalchemy".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Dark
                ),
                weight: 52 as f32,
            }
        );
        
        pokedex.insert(
            "shellder".to_string(),
            PokedexPokemon {
                species: "shellder".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 65,
                    defense: 100,
                    special_attack: 45,
                    special_defense: 25,
                    speed: 40
                },
                abilities: Abilities {
                    first: "shellarmor".to_string(),
                    second: "skilllink".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "cloyster".to_string(),
            PokedexPokemon {
                species: "cloyster".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 95,
                    defense: 180,
                    special_attack: 85,
                    special_defense: 45,
                    speed: 70
                },
                abilities: Abilities {
                    first: "shellarmor".to_string(),
                    second: "skilllink".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ice
                ),
                weight: 132.5 as f32,
            }
        );
        
        pokedex.insert(
            "gastly".to_string(),
            PokedexPokemon {
                species: "gastly".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 35,
                    defense: 30,
                    special_attack: 100,
                    special_defense: 35,
                    speed: 80
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Poison
                ),
                weight: 0.1 as f32,
            }
        );
        
        pokedex.insert(
            "haunter".to_string(),
            PokedexPokemon {
                species: "haunter".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 50,
                    defense: 45,
                    special_attack: 115,
                    special_defense: 55,
                    speed: 95
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Poison
                ),
                weight: 0.1 as f32,
            }
        );
        
        pokedex.insert(
            "gengar".to_string(),
            PokedexPokemon {
                species: "gengar".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 65,
                    defense: 60,
                    special_attack: 130,
                    special_defense: 75,
                    speed: 110
                },
                abilities: Abilities {
                    first: "cursedbody".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Poison
                ),
                weight: 40.5 as f32,
            }
        );
        
        pokedex.insert(
            "gengarmega".to_string(),
            PokedexPokemon {
                species: "gengarmega".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 65,
                    defense: 80,
                    special_attack: 170,
                    special_defense: 95,
                    speed: 130
                },
                abilities: Abilities {
                    first: "shadowtag".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Poison
                ),
                weight: 40.5 as f32,
            }
        );
        
        pokedex.insert(
            "onix".to_string(),
            PokedexPokemon {
                species: "onix".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 45,
                    defense: 160,
                    special_attack: 30,
                    special_defense: 45,
                    speed: 70
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Ground
                ),
                weight: 210 as f32,
            }
        );
        
        pokedex.insert(
            "drowzee".to_string(),
            PokedexPokemon {
                species: "drowzee".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 48,
                    defense: 45,
                    special_attack: 43,
                    special_defense: 90,
                    speed: 42
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "forewarn".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 32.4 as f32,
            }
        );
        
        pokedex.insert(
            "hypno".to_string(),
            PokedexPokemon {
                species: "hypno".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 73,
                    defense: 70,
                    special_attack: 73,
                    special_defense: 115,
                    speed: 67
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "forewarn".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 75.6 as f32,
            }
        );
        
        pokedex.insert(
            "krabby".to_string(),
            PokedexPokemon {
                species: "krabby".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 105,
                    defense: 90,
                    special_attack: 25,
                    special_defense: 25,
                    speed: 50
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "kingler".to_string(),
            PokedexPokemon {
                species: "kingler".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 130,
                    defense: 115,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 75
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 60 as f32,
            }
        );
        
        pokedex.insert(
            "voltorb".to_string(),
            PokedexPokemon {
                species: "voltorb".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 30,
                    defense: 50,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 100
                },
                abilities: Abilities {
                    first: "soundproof".to_string(),
                    second: "static".to_string(),
                    hidden: "aftermath".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 10.4 as f32,
            }
        );
        
        pokedex.insert(
            "electrode".to_string(),
            PokedexPokemon {
                species: "electrode".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 50,
                    defense: 70,
                    special_attack: 80,
                    special_defense: 80,
                    speed: 150
                },
                abilities: Abilities {
                    first: "soundproof".to_string(),
                    second: "static".to_string(),
                    hidden: "aftermath".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 66.6 as f32,
            }
        );
        
        pokedex.insert(
            "exeggcute".to_string(),
            PokedexPokemon {
                species: "exeggcute".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 40,
                    defense: 80,
                    special_attack: 60,
                    special_defense: 45,
                    speed: 40
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "harvest".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Psychic
                ),
                weight: 2.5 as f32,
            }
        );
        
        pokedex.insert(
            "exeggutor".to_string(),
            PokedexPokemon {
                species: "exeggutor".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 85,
                    special_attack: 125,
                    special_defense: 75,
                    speed: 55
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "harvest".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Psychic
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "exeggutoralola".to_string(),
            PokedexPokemon {
                species: "exeggutoralola".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 105,
                    defense: 85,
                    special_attack: 125,
                    special_defense: 75,
                    speed: 45
                },
                abilities: Abilities {
                    first: "frisk".to_string(),
                    second: "none".to_string(),
                    hidden: "harvest".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dragon
                ),
                weight: 415.6 as f32,
            }
        );
        
        pokedex.insert(
            "cubone".to_string(),
            PokedexPokemon {
                species: "cubone".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 50,
                    defense: 95,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 35
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "lightningrod".to_string(),
                    hidden: "battlearmor".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "marowak".to_string(),
            PokedexPokemon {
                species: "marowak".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 80,
                    defense: 110,
                    special_attack: 50,
                    special_defense: 80,
                    speed: 45
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "lightningrod".to_string(),
                    hidden: "battlearmor".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 45 as f32,
            }
        );
        
        pokedex.insert(
            "marowakalola".to_string(),
            PokedexPokemon {
                species: "marowakalola".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 80,
                    defense: 110,
                    special_attack: 50,
                    special_defense: 80,
                    speed: 45
                },
                abilities: Abilities {
                    first: "cursedbody".to_string(),
                    second: "lightningrod".to_string(),
                    hidden: "rockhead".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Ghost
                ),
                weight: 34 as f32,
            }
        );
        
        pokedex.insert(
            "hitmonlee".to_string(),
            PokedexPokemon {
                species: "hitmonlee".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 120,
                    defense: 53,
                    special_attack: 35,
                    special_defense: 110,
                    speed: 87
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "reckless".to_string(),
                    hidden: "unburden".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 49.8 as f32,
            }
        );
        
        pokedex.insert(
            "hitmonchan".to_string(),
            PokedexPokemon {
                species: "hitmonchan".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 105,
                    defense: 79,
                    special_attack: 35,
                    special_defense: 110,
                    speed: 76
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "ironfist".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 50.2 as f32,
            }
        );
        
        pokedex.insert(
            "lickitung".to_string(),
            PokedexPokemon {
                species: "lickitung".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 55,
                    defense: 75,
                    special_attack: 60,
                    special_defense: 75,
                    speed: 30
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "oblivious".to_string(),
                    hidden: "cloudnine".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 65.5 as f32,
            }
        );
        
        pokedex.insert(
            "koffing".to_string(),
            PokedexPokemon {
                species: "koffing".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 65,
                    defense: 95,
                    special_attack: 60,
                    special_defense: 45,
                    speed: 35
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "weezing".to_string(),
            PokedexPokemon {
                species: "weezing".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 90,
                    defense: 120,
                    special_attack: 85,
                    special_defense: 70,
                    speed: 60
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 9.5 as f32,
            }
        );
        
        pokedex.insert(
            "rhyhorn".to_string(),
            PokedexPokemon {
                species: "rhyhorn".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 85,
                    defense: 95,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 25
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "rockhead".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Rock
                ),
                weight: 115 as f32,
            }
        );
        
        pokedex.insert(
            "rhydon".to_string(),
            PokedexPokemon {
                species: "rhydon".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 130,
                    defense: 120,
                    special_attack: 45,
                    special_defense: 45,
                    speed: 40
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "rockhead".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Rock
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "chansey".to_string(),
            PokedexPokemon {
                species: "chansey".to_string(),
                base_stats: BaseStats {
                    hp: 250,
                    attack: 5,
                    defense: 5,
                    special_attack: 35,
                    special_defense: 105,
                    speed: 50
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "serenegrace".to_string(),
                    hidden: "healer".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 34.6 as f32,
            }
        );
        
        pokedex.insert(
            "tangela".to_string(),
            PokedexPokemon {
                species: "tangela".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 55,
                    defense: 115,
                    special_attack: 100,
                    special_defense: 40,
                    speed: 60
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "leafguard".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 35 as f32,
            }
        );
        
        pokedex.insert(
            "kangaskhan".to_string(),
            PokedexPokemon {
                species: "kangaskhan".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 95,
                    defense: 80,
                    special_attack: 40,
                    special_defense: 80,
                    speed: 90
                },
                abilities: Abilities {
                    first: "earlybird".to_string(),
                    second: "scrappy".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 80 as f32,
            }
        );
        
        pokedex.insert(
            "kangaskhanmega".to_string(),
            PokedexPokemon {
                species: "kangaskhanmega".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 125,
                    defense: 100,
                    special_attack: 60,
                    special_defense: 100,
                    speed: 100
                },
                abilities: Abilities {
                    first: "parentalbond".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 100 as f32,
            }
        );
        
        pokedex.insert(
            "horsea".to_string(),
            PokedexPokemon {
                species: "horsea".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 40,
                    defense: 70,
                    special_attack: 70,
                    special_defense: 25,
                    speed: 60
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "sniper".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 8 as f32,
            }
        );
        
        pokedex.insert(
            "seadra".to_string(),
            PokedexPokemon {
                species: "seadra".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 65,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 45,
                    speed: 85
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "sniper".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 25 as f32,
            }
        );
        
        pokedex.insert(
            "goldeen".to_string(),
            PokedexPokemon {
                species: "goldeen".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 67,
                    defense: 60,
                    special_attack: 35,
                    special_defense: 50,
                    speed: 63
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "waterveil".to_string(),
                    hidden: "lightningrod".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "seaking".to_string(),
            PokedexPokemon {
                species: "seaking".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 92,
                    defense: 65,
                    special_attack: 65,
                    special_defense: 80,
                    speed: 68
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "waterveil".to_string(),
                    hidden: "lightningrod".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 39 as f32,
            }
        );
        
        pokedex.insert(
            "staryu".to_string(),
            PokedexPokemon {
                species: "staryu".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 45,
                    defense: 55,
                    special_attack: 70,
                    special_defense: 55,
                    speed: 85
                },
                abilities: Abilities {
                    first: "illuminate".to_string(),
                    second: "naturalcure".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 34.5 as f32,
            }
        );
        
        pokedex.insert(
            "starmie".to_string(),
            PokedexPokemon {
                species: "starmie".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 75,
                    defense: 85,
                    special_attack: 100,
                    special_defense: 85,
                    speed: 115
                },
                abilities: Abilities {
                    first: "illuminate".to_string(),
                    second: "naturalcure".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Psychic
                ),
                weight: 80 as f32,
            }
        );
        
        pokedex.insert(
            "mrmime".to_string(),
            PokedexPokemon {
                species: "mrmime".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 65,
                    special_attack: 100,
                    special_defense: 120,
                    speed: 90
                },
                abilities: Abilities {
                    first: "soundproof".to_string(),
                    second: "filter".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 54.5 as f32,
            }
        );
        
        pokedex.insert(
            "scyther".to_string(),
            PokedexPokemon {
                species: "scyther".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 110,
                    defense: 80,
                    special_attack: 55,
                    special_defense: 80,
                    speed: 105
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "technician".to_string(),
                    hidden: "steadfast".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 56 as f32,
            }
        );
        
        pokedex.insert(
            "jynx".to_string(),
            PokedexPokemon {
                species: "jynx".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 50,
                    defense: 35,
                    special_attack: 115,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "forewarn".to_string(),
                    hidden: "dryskin".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Psychic
                ),
                weight: 40.6 as f32,
            }
        );
        
        pokedex.insert(
            "electabuzz".to_string(),
            PokedexPokemon {
                species: "electabuzz".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 83,
                    defense: 57,
                    special_attack: 95,
                    special_defense: 85,
                    speed: 105
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "vitalspirit".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "magmar".to_string(),
            PokedexPokemon {
                species: "magmar".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 95,
                    defense: 57,
                    special_attack: 100,
                    special_defense: 85,
                    speed: 93
                },
                abilities: Abilities {
                    first: "flamebody".to_string(),
                    second: "none".to_string(),
                    hidden: "vitalspirit".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 44.5 as f32,
            }
        );
        
        pokedex.insert(
            "pinsir".to_string(),
            PokedexPokemon {
                species: "pinsir".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 125,
                    defense: 100,
                    special_attack: 55,
                    special_defense: 70,
                    speed: 85
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "moldbreaker".to_string(),
                    hidden: "moxie".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 55 as f32,
            }
        );
        
        pokedex.insert(
            "pinsirmega".to_string(),
            PokedexPokemon {
                species: "pinsirmega".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 155,
                    defense: 120,
                    special_attack: 65,
                    special_defense: 90,
                    speed: 105
                },
                abilities: Abilities {
                    first: "aerilate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 59 as f32,
            }
        );
        
        pokedex.insert(
            "tauros".to_string(),
            PokedexPokemon {
                species: "tauros".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 100,
                    defense: 95,
                    special_attack: 40,
                    special_defense: 70,
                    speed: 110
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "angerpoint".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 88.4 as f32,
            }
        );
        
        pokedex.insert(
            "magikarp".to_string(),
            PokedexPokemon {
                species: "magikarp".to_string(),
                base_stats: BaseStats {
                    hp: 20,
                    attack: 10,
                    defense: 55,
                    special_attack: 15,
                    special_defense: 20,
                    speed: 80
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 10 as f32,
            }
        );
        
        pokedex.insert(
            "gyarados".to_string(),
            PokedexPokemon {
                species: "gyarados".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 125,
                    defense: 79,
                    special_attack: 60,
                    special_defense: 100,
                    speed: 81
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "none".to_string(),
                    hidden: "moxie".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Flying
                ),
                weight: 235 as f32,
            }
        );
        
        pokedex.insert(
            "gyaradosmega".to_string(),
            PokedexPokemon {
                species: "gyaradosmega".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 155,
                    defense: 109,
                    special_attack: 70,
                    special_defense: 130,
                    speed: 81
                },
                abilities: Abilities {
                    first: "moldbreaker".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dark
                ),
                weight: 305 as f32,
            }
        );
        
        pokedex.insert(
            "lapras".to_string(),
            PokedexPokemon {
                species: "lapras".to_string(),
                base_stats: BaseStats {
                    hp: 130,
                    attack: 85,
                    defense: 80,
                    special_attack: 85,
                    special_defense: 95,
                    speed: 60
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ice
                ),
                weight: 220 as f32,
            }
        );
        
        pokedex.insert(
            "ditto".to_string(),
            PokedexPokemon {
                species: "ditto".to_string(),
                base_stats: BaseStats {
                    hp: 48,
                    attack: 48,
                    defense: 48,
                    special_attack: 48,
                    special_defense: 48,
                    speed: 48
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "none".to_string(),
                    hidden: "imposter".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "eevee".to_string(),
            PokedexPokemon {
                species: "eevee".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 55,
                    defense: 50,
                    special_attack: 45,
                    special_defense: 65,
                    speed: 55
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "adaptability".to_string(),
                    hidden: "anticipation".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "vaporeon".to_string(),
            PokedexPokemon {
                species: "vaporeon".to_string(),
                base_stats: BaseStats {
                    hp: 130,
                    attack: 65,
                    defense: 60,
                    special_attack: 110,
                    special_defense: 95,
                    speed: 65
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "none".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 29 as f32,
            }
        );
        
        pokedex.insert(
            "jolteon".to_string(),
            PokedexPokemon {
                species: "jolteon".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 65,
                    defense: 60,
                    special_attack: 110,
                    special_defense: 95,
                    speed: 130
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "none".to_string(),
                    hidden: "quickfeet".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 24.5 as f32,
            }
        );
        
        pokedex.insert(
            "flareon".to_string(),
            PokedexPokemon {
                species: "flareon".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 130,
                    defense: 60,
                    special_attack: 95,
                    special_defense: 110,
                    speed: 65
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "none".to_string(),
                    hidden: "guts".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 25 as f32,
            }
        );
        
        pokedex.insert(
            "porygon".to_string(),
            PokedexPokemon {
                species: "porygon".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 60,
                    defense: 70,
                    special_attack: 85,
                    special_defense: 75,
                    speed: 40
                },
                abilities: Abilities {
                    first: "trace".to_string(),
                    second: "download".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 36.5 as f32,
            }
        );
        
        pokedex.insert(
            "omanyte".to_string(),
            PokedexPokemon {
                species: "omanyte".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 40,
                    defense: 100,
                    special_attack: 90,
                    special_defense: 55,
                    speed: 35
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Water
                ),
                weight: 7.5 as f32,
            }
        );
        
        pokedex.insert(
            "omastar".to_string(),
            PokedexPokemon {
                species: "omastar".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 60,
                    defense: 125,
                    special_attack: 115,
                    special_defense: 70,
                    speed: 55
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Water
                ),
                weight: 35 as f32,
            }
        );
        
        pokedex.insert(
            "kabuto".to_string(),
            PokedexPokemon {
                species: "kabuto".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 80,
                    defense: 90,
                    special_attack: 55,
                    special_defense: 45,
                    speed: 55
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "battlearmor".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Water
                ),
                weight: 11.5 as f32,
            }
        );
        
        pokedex.insert(
            "kabutops".to_string(),
            PokedexPokemon {
                species: "kabutops".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 115,
                    defense: 105,
                    special_attack: 65,
                    special_defense: 70,
                    speed: 80
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "battlearmor".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Water
                ),
                weight: 40.5 as f32,
            }
        );
        
        pokedex.insert(
            "aerodactyl".to_string(),
            PokedexPokemon {
                species: "aerodactyl".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 105,
                    defense: 65,
                    special_attack: 60,
                    special_defense: 75,
                    speed: 130
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "pressure".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Flying
                ),
                weight: 59 as f32,
            }
        );
        
        pokedex.insert(
            "aerodactylmega".to_string(),
            PokedexPokemon {
                species: "aerodactylmega".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 135,
                    defense: 85,
                    special_attack: 70,
                    special_defense: 95,
                    speed: 150
                },
                abilities: Abilities {
                    first: "toughclaws".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Flying
                ),
                weight: 79 as f32,
            }
        );
        
        pokedex.insert(
            "snorlax".to_string(),
            PokedexPokemon {
                species: "snorlax".to_string(),
                base_stats: BaseStats {
                    hp: 160,
                    attack: 110,
                    defense: 65,
                    special_attack: 65,
                    special_defense: 110,
                    speed: 30
                },
                abilities: Abilities {
                    first: "immunity".to_string(),
                    second: "thickfat".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 460 as f32,
            }
        );
        
        pokedex.insert(
            "articuno".to_string(),
            PokedexPokemon {
                species: "articuno".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 85,
                    defense: 100,
                    special_attack: 95,
                    special_defense: 125,
                    speed: 85
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "snowcloak".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Flying
                ),
                weight: 55.4 as f32,
            }
        );
        
        pokedex.insert(
            "articunogalar".to_string(),
            PokedexPokemon {
                species: "articunogalar".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 85,
                    defense: 85,
                    special_attack: 125,
                    special_defense: 100,
                    speed: 95
                },
                abilities: Abilities {
                    first: "competitive".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Flying
                ),
                weight: 50.9 as f32,
            }
        );
        
        pokedex.insert(
            "zapdos".to_string(),
            PokedexPokemon {
                species: "zapdos".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 90,
                    defense: 85,
                    special_attack: 125,
                    special_defense: 90,
                    speed: 100
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "static".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Flying
                ),
                weight: 52.6 as f32,
            }
        );
        
        pokedex.insert(
            "zapdosgalar".to_string(),
            PokedexPokemon {
                species: "zapdosgalar".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 125,
                    defense: 90,
                    special_attack: 85,
                    special_defense: 90,
                    speed: 100
                },
                abilities: Abilities {
                    first: "defiant".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Flying
                ),
                weight: 58.2 as f32,
            }
        );
        
        pokedex.insert(
            "moltres".to_string(),
            PokedexPokemon {
                species: "moltres".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 100,
                    defense: 90,
                    special_attack: 125,
                    special_defense: 85,
                    speed: 90
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "flamebody".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Flying
                ),
                weight: 60 as f32,
            }
        );
        
        pokedex.insert(
            "moltresgalar".to_string(),
            PokedexPokemon {
                species: "moltresgalar".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 85,
                    defense: 90,
                    special_attack: 100,
                    special_defense: 125,
                    speed: 90
                },
                abilities: Abilities {
                    first: "berserk".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Flying
                ),
                weight: 66 as f32,
            }
        );
        
        pokedex.insert(
            "dratini".to_string(),
            PokedexPokemon {
                species: "dratini".to_string(),
                base_stats: BaseStats {
                    hp: 41,
                    attack: 64,
                    defense: 45,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 50
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "marvelscale".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 3.3 as f32,
            }
        );
        
        pokedex.insert(
            "dragonair".to_string(),
            PokedexPokemon {
                species: "dragonair".to_string(),
                base_stats: BaseStats {
                    hp: 61,
                    attack: 84,
                    defense: 65,
                    special_attack: 70,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "marvelscale".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 16.5 as f32,
            }
        );
        
        pokedex.insert(
            "dragonite".to_string(),
            PokedexPokemon {
                species: "dragonite".to_string(),
                base_stats: BaseStats {
                    hp: 91,
                    attack: 134,
                    defense: 95,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 80
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "none".to_string(),
                    hidden: "multiscale".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Flying
                ),
                weight: 210 as f32,
            }
        );
        
        pokedex.insert(
            "mewtwo".to_string(),
            PokedexPokemon {
                species: "mewtwo".to_string(),
                base_stats: BaseStats {
                    hp: 106,
                    attack: 110,
                    defense: 90,
                    special_attack: 154,
                    special_defense: 90,
                    speed: 130
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 122 as f32,
            }
        );
        
        pokedex.insert(
            "mewtwomegax".to_string(),
            PokedexPokemon {
                species: "mewtwomegax".to_string(),
                base_stats: BaseStats {
                    hp: 106,
                    attack: 190,
                    defense: 100,
                    special_attack: 154,
                    special_defense: 100,
                    speed: 130
                },
                abilities: Abilities {
                    first: "steadfast".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fighting
                ),
                weight: 127 as f32,
            }
        );
        
        pokedex.insert(
            "mewtwomegay".to_string(),
            PokedexPokemon {
                species: "mewtwomegay".to_string(),
                base_stats: BaseStats {
                    hp: 106,
                    attack: 150,
                    defense: 70,
                    special_attack: 194,
                    special_defense: 120,
                    speed: 140
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 33 as f32,
            }
        );
        
        pokedex.insert(
            "mew".to_string(),
            PokedexPokemon {
                species: "mew".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 100,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 100
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "chikorita".to_string(),
            PokedexPokemon {
                species: "chikorita".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 49,
                    defense: 65,
                    special_attack: 49,
                    special_defense: 65,
                    speed: 45
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "leafguard".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 6.4 as f32,
            }
        );
        
        pokedex.insert(
            "bayleef".to_string(),
            PokedexPokemon {
                species: "bayleef".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 62,
                    defense: 80,
                    special_attack: 63,
                    special_defense: 80,
                    speed: 60
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "leafguard".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 15.8 as f32,
            }
        );
        
        pokedex.insert(
            "meganium".to_string(),
            PokedexPokemon {
                species: "meganium".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 82,
                    defense: 100,
                    special_attack: 83,
                    special_defense: 100,
                    speed: 80
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "leafguard".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "cyndaquil".to_string(),
            PokedexPokemon {
                species: "cyndaquil".to_string(),
                base_stats: BaseStats {
                    hp: 39,
                    attack: 52,
                    defense: 43,
                    special_attack: 60,
                    special_defense: 50,
                    speed: 65
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "flashfire".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 7.9 as f32,
            }
        );
        
        pokedex.insert(
            "quilava".to_string(),
            PokedexPokemon {
                species: "quilava".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 64,
                    defense: 58,
                    special_attack: 80,
                    special_defense: 65,
                    speed: 80
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "flashfire".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 19 as f32,
            }
        );
        
        pokedex.insert(
            "typhlosion".to_string(),
            PokedexPokemon {
                species: "typhlosion".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 84,
                    defense: 78,
                    special_attack: 109,
                    special_defense: 85,
                    speed: 100
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "flashfire".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 79.5 as f32,
            }
        );
        
        pokedex.insert(
            "totodile".to_string(),
            PokedexPokemon {
                species: "totodile".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 64,
                    special_attack: 44,
                    special_defense: 48,
                    speed: 43
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 9.5 as f32,
            }
        );
        
        pokedex.insert(
            "croconaw".to_string(),
            PokedexPokemon {
                species: "croconaw".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 80,
                    defense: 80,
                    special_attack: 59,
                    special_defense: 63,
                    speed: 58
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 25 as f32,
            }
        );
        
        pokedex.insert(
            "feraligatr".to_string(),
            PokedexPokemon {
                species: "feraligatr".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 105,
                    defense: 100,
                    special_attack: 79,
                    special_defense: 83,
                    speed: 78
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 88.8 as f32,
            }
        );
        
        pokedex.insert(
            "sentret".to_string(),
            PokedexPokemon {
                species: "sentret".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 46,
                    defense: 34,
                    special_attack: 35,
                    special_defense: 45,
                    speed: 20
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "keeneye".to_string(),
                    hidden: "frisk".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "furret".to_string(),
            PokedexPokemon {
                species: "furret".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 76,
                    defense: 64,
                    special_attack: 45,
                    special_defense: 55,
                    speed: 90
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "keeneye".to_string(),
                    hidden: "frisk".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 32.5 as f32,
            }
        );
        
        pokedex.insert(
            "hoothoot".to_string(),
            PokedexPokemon {
                species: "hoothoot".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 30,
                    defense: 30,
                    special_attack: 36,
                    special_defense: 56,
                    speed: 50
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "keeneye".to_string(),
                    hidden: "tintedlens".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 21.2 as f32,
            }
        );
        
        pokedex.insert(
            "noctowl".to_string(),
            PokedexPokemon {
                species: "noctowl".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 50,
                    defense: 50,
                    special_attack: 86,
                    special_defense: 96,
                    speed: 70
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "keeneye".to_string(),
                    hidden: "tintedlens".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 40.8 as f32,
            }
        );
        
        pokedex.insert(
            "ledyba".to_string(),
            PokedexPokemon {
                species: "ledyba".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 20,
                    defense: 30,
                    special_attack: 40,
                    special_defense: 80,
                    speed: 55
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 10.8 as f32,
            }
        );
        
        pokedex.insert(
            "ledian".to_string(),
            PokedexPokemon {
                species: "ledian".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 35,
                    defense: 50,
                    special_attack: 55,
                    special_defense: 110,
                    speed: 85
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "ironfist".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 35.6 as f32,
            }
        );
        
        pokedex.insert(
            "spinarak".to_string(),
            PokedexPokemon {
                species: "spinarak".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 60,
                    defense: 40,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 30
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "insomnia".to_string(),
                    hidden: "sniper".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "ariados".to_string(),
            PokedexPokemon {
                species: "ariados".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 90,
                    defense: 70,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 40
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "insomnia".to_string(),
                    hidden: "sniper".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 33.5 as f32,
            }
        );
        
        pokedex.insert(
            "crobat".to_string(),
            PokedexPokemon {
                species: "crobat".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 90,
                    defense: 80,
                    special_attack: 70,
                    special_defense: 80,
                    speed: 130
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "none".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Flying
                ),
                weight: 75 as f32,
            }
        );
        
        pokedex.insert(
            "chinchou".to_string(),
            PokedexPokemon {
                species: "chinchou".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 38,
                    defense: 38,
                    special_attack: 56,
                    special_defense: 56,
                    speed: 67
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "illuminate".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Electric
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "lanturn".to_string(),
            PokedexPokemon {
                species: "lanturn".to_string(),
                base_stats: BaseStats {
                    hp: 125,
                    attack: 58,
                    defense: 58,
                    special_attack: 76,
                    special_defense: 76,
                    speed: 67
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "illuminate".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Electric
                ),
                weight: 22.5 as f32,
            }
        );
        
        pokedex.insert(
            "pichu".to_string(),
            PokedexPokemon {
                species: "pichu".to_string(),
                base_stats: BaseStats {
                    hp: 20,
                    attack: 40,
                    defense: 15,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 60
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "lightningrod".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "pichuspikyeared".to_string(),
            PokedexPokemon {
                species: "pichuspikyeared".to_string(),
                base_stats: BaseStats {
                    hp: 20,
                    attack: 40,
                    defense: 15,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 60
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "cleffa".to_string(),
            PokedexPokemon {
                species: "cleffa".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 25,
                    defense: 28,
                    special_attack: 45,
                    special_defense: 55,
                    speed: 15
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "magicguard".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 3 as f32,
            }
        );
        
        pokedex.insert(
            "igglybuff".to_string(),
            PokedexPokemon {
                species: "igglybuff".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 30,
                    defense: 15,
                    special_attack: 40,
                    special_defense: 20,
                    speed: 15
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "competitive".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fairy
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "togepi".to_string(),
            PokedexPokemon {
                species: "togepi".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 20,
                    defense: 65,
                    special_attack: 40,
                    special_defense: 65,
                    speed: 20
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "serenegrace".to_string(),
                    hidden: "superluck".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 1.5 as f32,
            }
        );
        
        pokedex.insert(
            "togetic".to_string(),
            PokedexPokemon {
                species: "togetic".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 40,
                    defense: 85,
                    special_attack: 80,
                    special_defense: 105,
                    speed: 40
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "serenegrace".to_string(),
                    hidden: "superluck".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Flying
                ),
                weight: 3.2 as f32,
            }
        );
        
        pokedex.insert(
            "natu".to_string(),
            PokedexPokemon {
                species: "natu".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 50,
                    defense: 45,
                    special_attack: 70,
                    special_defense: 45,
                    speed: 70
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "magicbounce".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Flying
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "xatu".to_string(),
            PokedexPokemon {
                species: "xatu".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 75,
                    defense: 70,
                    special_attack: 95,
                    special_defense: 70,
                    speed: 95
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "magicbounce".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Flying
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "mareep".to_string(),
            PokedexPokemon {
                species: "mareep".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 40,
                    defense: 40,
                    special_attack: 65,
                    special_defense: 45,
                    speed: 35
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "plus".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 7.8 as f32,
            }
        );
        
        pokedex.insert(
            "flaaffy".to_string(),
            PokedexPokemon {
                species: "flaaffy".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 55,
                    defense: 55,
                    special_attack: 80,
                    special_defense: 60,
                    speed: 45
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "plus".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 13.3 as f32,
            }
        );
        
        pokedex.insert(
            "ampharos".to_string(),
            PokedexPokemon {
                species: "ampharos".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 75,
                    defense: 85,
                    special_attack: 115,
                    special_defense: 90,
                    speed: 55
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "plus".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 61.5 as f32,
            }
        );
        
        pokedex.insert(
            "ampharosmega".to_string(),
            PokedexPokemon {
                species: "ampharosmega".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 95,
                    defense: 105,
                    special_attack: 165,
                    special_defense: 110,
                    speed: 45
                },
                abilities: Abilities {
                    first: "moldbreaker".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Dragon
                ),
                weight: 61.5 as f32,
            }
        );
        
        pokedex.insert(
            "bellossom".to_string(),
            PokedexPokemon {
                species: "bellossom".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 80,
                    defense: 95,
                    special_attack: 90,
                    special_defense: 100,
                    speed: 50
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "healer".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 5.8 as f32,
            }
        );
        
        pokedex.insert(
            "marill".to_string(),
            PokedexPokemon {
                species: "marill".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 20,
                    defense: 50,
                    special_attack: 20,
                    special_defense: 50,
                    speed: 40
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "hugepower".to_string(),
                    hidden: "sapsipper".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fairy
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "azumarill".to_string(),
            PokedexPokemon {
                species: "azumarill".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 50,
                    defense: 80,
                    special_attack: 60,
                    special_defense: 80,
                    speed: 50
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "hugepower".to_string(),
                    hidden: "sapsipper".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fairy
                ),
                weight: 28.5 as f32,
            }
        );
        
        pokedex.insert(
            "sudowoodo".to_string(),
            PokedexPokemon {
                species: "sudowoodo".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 100,
                    defense: 115,
                    special_attack: 30,
                    special_defense: 65,
                    speed: 30
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "rockhead".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 38 as f32,
            }
        );
        
        pokedex.insert(
            "politoed".to_string(),
            PokedexPokemon {
                species: "politoed".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 75,
                    defense: 75,
                    special_attack: 90,
                    special_defense: 100,
                    speed: 70
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "damp".to_string(),
                    hidden: "drizzle".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 33.9 as f32,
            }
        );
        
        pokedex.insert(
            "hoppip".to_string(),
            PokedexPokemon {
                species: "hoppip".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 35,
                    defense: 40,
                    special_attack: 35,
                    special_defense: 55,
                    speed: 50
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "leafguard".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Flying
                ),
                weight: 0.5 as f32,
            }
        );
        
        pokedex.insert(
            "skiploom".to_string(),
            PokedexPokemon {
                species: "skiploom".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 45,
                    defense: 50,
                    special_attack: 45,
                    special_defense: 65,
                    speed: 80
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "leafguard".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Flying
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "jumpluff".to_string(),
            PokedexPokemon {
                species: "jumpluff".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 55,
                    defense: 70,
                    special_attack: 55,
                    special_defense: 95,
                    speed: 110
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "leafguard".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Flying
                ),
                weight: 3 as f32,
            }
        );
        
        pokedex.insert(
            "aipom".to_string(),
            PokedexPokemon {
                species: "aipom".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 70,
                    defense: 55,
                    special_attack: 40,
                    special_defense: 55,
                    speed: 85
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "pickup".to_string(),
                    hidden: "skilllink".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 11.5 as f32,
            }
        );
        
        pokedex.insert(
            "sunkern".to_string(),
            PokedexPokemon {
                species: "sunkern".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 30,
                    defense: 30,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 30
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "solarpower".to_string(),
                    hidden: "earlybird".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 1.8 as f32,
            }
        );
        
        pokedex.insert(
            "sunflora".to_string(),
            PokedexPokemon {
                species: "sunflora".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 75,
                    defense: 55,
                    special_attack: 105,
                    special_defense: 85,
                    speed: 30
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "solarpower".to_string(),
                    hidden: "earlybird".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "yanma".to_string(),
            PokedexPokemon {
                species: "yanma".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 65,
                    defense: 45,
                    special_attack: 75,
                    special_defense: 45,
                    speed: 95
                },
                abilities: Abilities {
                    first: "speedboost".to_string(),
                    second: "compoundeyes".to_string(),
                    hidden: "frisk".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 38 as f32,
            }
        );
        
        pokedex.insert(
            "wooper".to_string(),
            PokedexPokemon {
                species: "wooper".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 45,
                    defense: 45,
                    special_attack: 25,
                    special_defense: 25,
                    speed: 15
                },
                abilities: Abilities {
                    first: "damp".to_string(),
                    second: "waterabsorb".to_string(),
                    hidden: "unaware".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "quagsire".to_string(),
            PokedexPokemon {
                species: "quagsire".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 85,
                    defense: 85,
                    special_attack: 65,
                    special_defense: 65,
                    speed: 35
                },
                abilities: Abilities {
                    first: "damp".to_string(),
                    second: "waterabsorb".to_string(),
                    hidden: "unaware".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 75 as f32,
            }
        );
        
        pokedex.insert(
            "espeon".to_string(),
            PokedexPokemon {
                species: "espeon".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 65,
                    defense: 60,
                    special_attack: 130,
                    special_defense: 95,
                    speed: 110
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "none".to_string(),
                    hidden: "magicbounce".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 26.5 as f32,
            }
        );
        
        pokedex.insert(
            "umbreon".to_string(),
            PokedexPokemon {
                species: "umbreon".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 65,
                    defense: 110,
                    special_attack: 60,
                    special_defense: 130,
                    speed: 65
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "none".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 27 as f32,
            }
        );
        
        pokedex.insert(
            "murkrow".to_string(),
            PokedexPokemon {
                species: "murkrow".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 85,
                    defense: 42,
                    special_attack: 85,
                    special_defense: 42,
                    speed: 91
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "superluck".to_string(),
                    hidden: "prankster".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Flying
                ),
                weight: 2.1 as f32,
            }
        );
        
        pokedex.insert(
            "slowking".to_string(),
            PokedexPokemon {
                species: "slowking".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 75,
                    defense: 80,
                    special_attack: 100,
                    special_defense: 110,
                    speed: 30
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Psychic
                ),
                weight: 79.5 as f32,
            }
        );
        
        pokedex.insert(
            "slowkinggalar".to_string(),
            PokedexPokemon {
                species: "slowkinggalar".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 65,
                    defense: 80,
                    special_attack: 110,
                    special_defense: 110,
                    speed: 30
                },
                abilities: Abilities {
                    first: "curiousmedicine".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Psychic
                ),
                weight: 79.5 as f32,
            }
        );
        
        pokedex.insert(
            "misdreavus".to_string(),
            PokedexPokemon {
                species: "misdreavus".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 60,
                    special_attack: 85,
                    special_defense: 85,
                    speed: 85
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "unown".to_string(),
            PokedexPokemon {
                species: "unown".to_string(),
                base_stats: BaseStats {
                    hp: 48,
                    attack: 72,
                    defense: 48,
                    special_attack: 72,
                    special_defense: 48,
                    speed: 48
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "wobbuffet".to_string(),
            PokedexPokemon {
                species: "wobbuffet".to_string(),
                base_stats: BaseStats {
                    hp: 190,
                    attack: 33,
                    defense: 58,
                    special_attack: 33,
                    special_defense: 58,
                    speed: 33
                },
                abilities: Abilities {
                    first: "shadowtag".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 28.5 as f32,
            }
        );
        
        pokedex.insert(
            "girafarig".to_string(),
            PokedexPokemon {
                species: "girafarig".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 80,
                    defense: 65,
                    special_attack: 90,
                    special_defense: 65,
                    speed: 85
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "sapsipper".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Psychic
                ),
                weight: 41.5 as f32,
            }
        );
        
        pokedex.insert(
            "pineco".to_string(),
            PokedexPokemon {
                species: "pineco".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 90,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 15
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "none".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 7.2 as f32,
            }
        );
        
        pokedex.insert(
            "forretress".to_string(),
            PokedexPokemon {
                species: "forretress".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 90,
                    defense: 140,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 40
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "none".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 125.8 as f32,
            }
        );
        
        pokedex.insert(
            "dunsparce".to_string(),
            PokedexPokemon {
                species: "dunsparce".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 70,
                    defense: 70,
                    special_attack: 65,
                    special_defense: 65,
                    speed: 45
                },
                abilities: Abilities {
                    first: "serenegrace".to_string(),
                    second: "runaway".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 14 as f32,
            }
        );
        
        pokedex.insert(
            "gligar".to_string(),
            PokedexPokemon {
                species: "gligar".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 75,
                    defense: 105,
                    special_attack: 35,
                    special_defense: 65,
                    speed: 85
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "sandveil".to_string(),
                    hidden: "immunity".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Flying
                ),
                weight: 64.8 as f32,
            }
        );
        
        pokedex.insert(
            "steelix".to_string(),
            PokedexPokemon {
                species: "steelix".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 85,
                    defense: 200,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 30
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Ground
                ),
                weight: 400 as f32,
            }
        );
        
        pokedex.insert(
            "steelixmega".to_string(),
            PokedexPokemon {
                species: "steelixmega".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 125,
                    defense: 230,
                    special_attack: 55,
                    special_defense: 95,
                    speed: 30
                },
                abilities: Abilities {
                    first: "sandforce".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Ground
                ),
                weight: 740 as f32,
            }
        );
        
        pokedex.insert(
            "snubbull".to_string(),
            PokedexPokemon {
                species: "snubbull".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 80,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 30
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "runaway".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 7.8 as f32,
            }
        );
        
        pokedex.insert(
            "granbull".to_string(),
            PokedexPokemon {
                species: "granbull".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 120,
                    defense: 75,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 45
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "quickfeet".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 48.7 as f32,
            }
        );
        
        pokedex.insert(
            "qwilfish".to_string(),
            PokedexPokemon {
                species: "qwilfish".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 95,
                    defense: 85,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 85
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "swiftswim".to_string(),
                    hidden: "intimidate".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Poison
                ),
                weight: 3.9 as f32,
            }
        );
        
        pokedex.insert(
            "scizor".to_string(),
            PokedexPokemon {
                species: "scizor".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 130,
                    defense: 100,
                    special_attack: 55,
                    special_defense: 80,
                    speed: 65
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "technician".to_string(),
                    hidden: "lightmetal".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 118 as f32,
            }
        );
        
        pokedex.insert(
            "scizormega".to_string(),
            PokedexPokemon {
                species: "scizormega".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 150,
                    defense: 140,
                    special_attack: 65,
                    special_defense: 100,
                    speed: 75
                },
                abilities: Abilities {
                    first: "technician".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 125 as f32,
            }
        );
        
        pokedex.insert(
            "shuckle".to_string(),
            PokedexPokemon {
                species: "shuckle".to_string(),
                base_stats: BaseStats {
                    hp: 20,
                    attack: 10,
                    defense: 230,
                    special_attack: 10,
                    special_defense: 230,
                    speed: 5
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "contrary".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Rock
                ),
                weight: 20.5 as f32,
            }
        );
        
        pokedex.insert(
            "heracross".to_string(),
            PokedexPokemon {
                species: "heracross".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 125,
                    defense: 75,
                    special_attack: 40,
                    special_defense: 95,
                    speed: 85
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "guts".to_string(),
                    hidden: "moxie".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fighting
                ),
                weight: 54 as f32,
            }
        );
        
        pokedex.insert(
            "heracrossmega".to_string(),
            PokedexPokemon {
                species: "heracrossmega".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 185,
                    defense: 115,
                    special_attack: 40,
                    special_defense: 105,
                    speed: 75
                },
                abilities: Abilities {
                    first: "skilllink".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fighting
                ),
                weight: 62.5 as f32,
            }
        );
        
        pokedex.insert(
            "sneasel".to_string(),
            PokedexPokemon {
                species: "sneasel".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 95,
                    defense: 55,
                    special_attack: 35,
                    special_defense: 75,
                    speed: 115
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "keeneye".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Ice
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "teddiursa".to_string(),
            PokedexPokemon {
                species: "teddiursa".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 80,
                    defense: 50,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 40
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "quickfeet".to_string(),
                    hidden: "honeygather".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 8.8 as f32,
            }
        );
        
        pokedex.insert(
            "ursaring".to_string(),
            PokedexPokemon {
                species: "ursaring".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 130,
                    defense: 75,
                    special_attack: 75,
                    special_defense: 75,
                    speed: 55
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "quickfeet".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 125.8 as f32,
            }
        );
        
        pokedex.insert(
            "slugma".to_string(),
            PokedexPokemon {
                species: "slugma".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 40,
                    defense: 40,
                    special_attack: 70,
                    special_defense: 40,
                    speed: 20
                },
                abilities: Abilities {
                    first: "magmaarmor".to_string(),
                    second: "flamebody".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 35 as f32,
            }
        );
        
        pokedex.insert(
            "magcargo".to_string(),
            PokedexPokemon {
                species: "magcargo".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 50,
                    defense: 120,
                    special_attack: 90,
                    special_defense: 80,
                    speed: 30
                },
                abilities: Abilities {
                    first: "magmaarmor".to_string(),
                    second: "flamebody".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Rock
                ),
                weight: 55 as f32,
            }
        );
        
        pokedex.insert(
            "swinub".to_string(),
            PokedexPokemon {
                species: "swinub".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 50,
                    defense: 40,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 50
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "snowcloak".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Ground
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "piloswine".to_string(),
            PokedexPokemon {
                species: "piloswine".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 80,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 50
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "snowcloak".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Ground
                ),
                weight: 55.8 as f32,
            }
        );
        
        pokedex.insert(
            "corsola".to_string(),
            PokedexPokemon {
                species: "corsola".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 55,
                    defense: 95,
                    special_attack: 65,
                    special_defense: 95,
                    speed: 35
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "naturalcure".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Rock
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "remoraid".to_string(),
            PokedexPokemon {
                species: "remoraid".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 65,
                    defense: 35,
                    special_attack: 65,
                    special_defense: 35,
                    speed: 65
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "sniper".to_string(),
                    hidden: "moody".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "octillery".to_string(),
            PokedexPokemon {
                species: "octillery".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 105,
                    defense: 75,
                    special_attack: 105,
                    special_defense: 75,
                    speed: 45
                },
                abilities: Abilities {
                    first: "suctioncups".to_string(),
                    second: "sniper".to_string(),
                    hidden: "moody".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 28.5 as f32,
            }
        );
        
        pokedex.insert(
            "delibird".to_string(),
            PokedexPokemon {
                species: "delibird".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 55,
                    defense: 45,
                    special_attack: 65,
                    special_defense: 45,
                    speed: 75
                },
                abilities: Abilities {
                    first: "vitalspirit".to_string(),
                    second: "hustle".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Flying
                ),
                weight: 16 as f32,
            }
        );
        
        pokedex.insert(
            "mantine".to_string(),
            PokedexPokemon {
                species: "mantine".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 40,
                    defense: 70,
                    special_attack: 80,
                    special_defense: 140,
                    speed: 70
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "waterabsorb".to_string(),
                    hidden: "waterveil".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Flying
                ),
                weight: 220 as f32,
            }
        );
        
        pokedex.insert(
            "skarmory".to_string(),
            PokedexPokemon {
                species: "skarmory".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 80,
                    defense: 140,
                    special_attack: 40,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Flying
                ),
                weight: 50.5 as f32,
            }
        );
        
        pokedex.insert(
            "houndour".to_string(),
            PokedexPokemon {
                species: "houndour".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 60,
                    defense: 30,
                    special_attack: 80,
                    special_defense: 50,
                    speed: 65
                },
                abilities: Abilities {
                    first: "earlybird".to_string(),
                    second: "flashfire".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fire
                ),
                weight: 10.8 as f32,
            }
        );
        
        pokedex.insert(
            "houndoom".to_string(),
            PokedexPokemon {
                species: "houndoom".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 90,
                    defense: 50,
                    special_attack: 110,
                    special_defense: 80,
                    speed: 95
                },
                abilities: Abilities {
                    first: "earlybird".to_string(),
                    second: "flashfire".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fire
                ),
                weight: 35 as f32,
            }
        );
        
        pokedex.insert(
            "houndoommega".to_string(),
            PokedexPokemon {
                species: "houndoommega".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 90,
                    defense: 90,
                    special_attack: 140,
                    special_defense: 90,
                    speed: 115
                },
                abilities: Abilities {
                    first: "solarpower".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fire
                ),
                weight: 49.5 as f32,
            }
        );
        
        pokedex.insert(
            "kingdra".to_string(),
            PokedexPokemon {
                species: "kingdra".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 85
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "sniper".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dragon
                ),
                weight: 152 as f32,
            }
        );
        
        pokedex.insert(
            "phanpy".to_string(),
            PokedexPokemon {
                species: "phanpy".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 60,
                    defense: 60,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 40
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "none".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 33.5 as f32,
            }
        );
        
        pokedex.insert(
            "donphan".to_string(),
            PokedexPokemon {
                species: "donphan".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 120,
                    defense: 120,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 50
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "none".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "porygon2".to_string(),
            PokedexPokemon {
                species: "porygon2".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 80,
                    defense: 90,
                    special_attack: 105,
                    special_defense: 95,
                    speed: 60
                },
                abilities: Abilities {
                    first: "trace".to_string(),
                    second: "download".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 32.5 as f32,
            }
        );
        
        pokedex.insert(
            "stantler".to_string(),
            PokedexPokemon {
                species: "stantler".to_string(),
                base_stats: BaseStats {
                    hp: 73,
                    attack: 95,
                    defense: 62,
                    special_attack: 85,
                    special_defense: 65,
                    speed: 85
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "frisk".to_string(),
                    hidden: "sapsipper".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 71.2 as f32,
            }
        );
        
        pokedex.insert(
            "smeargle".to_string(),
            PokedexPokemon {
                species: "smeargle".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 20,
                    defense: 35,
                    special_attack: 20,
                    special_defense: 45,
                    speed: 75
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "technician".to_string(),
                    hidden: "moody".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 58 as f32,
            }
        );
        
        pokedex.insert(
            "tyrogue".to_string(),
            PokedexPokemon {
                species: "tyrogue".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 35,
                    defense: 35,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 35
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "steadfast".to_string(),
                    hidden: "vitalspirit".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 21 as f32,
            }
        );
        
        pokedex.insert(
            "hitmontop".to_string(),
            PokedexPokemon {
                species: "hitmontop".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 95,
                    defense: 95,
                    special_attack: 35,
                    special_defense: 110,
                    speed: 70
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "technician".to_string(),
                    hidden: "steadfast".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 48 as f32,
            }
        );
        
        pokedex.insert(
            "smoochum".to_string(),
            PokedexPokemon {
                species: "smoochum".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 30,
                    defense: 15,
                    special_attack: 85,
                    special_defense: 65,
                    speed: 65
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "forewarn".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Psychic
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "elekid".to_string(),
            PokedexPokemon {
                species: "elekid".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 63,
                    defense: 37,
                    special_attack: 65,
                    special_defense: 55,
                    speed: 95
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "vitalspirit".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 23.5 as f32,
            }
        );
        
        pokedex.insert(
            "magby".to_string(),
            PokedexPokemon {
                species: "magby".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 75,
                    defense: 37,
                    special_attack: 70,
                    special_defense: 55,
                    speed: 83
                },
                abilities: Abilities {
                    first: "flamebody".to_string(),
                    second: "none".to_string(),
                    hidden: "vitalspirit".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 21.4 as f32,
            }
        );
        
        pokedex.insert(
            "miltank".to_string(),
            PokedexPokemon {
                species: "miltank".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 80,
                    defense: 105,
                    special_attack: 40,
                    special_defense: 70,
                    speed: 100
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "scrappy".to_string(),
                    hidden: "sapsipper".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 75.5 as f32,
            }
        );
        
        pokedex.insert(
            "blissey".to_string(),
            PokedexPokemon {
                species: "blissey".to_string(),
                base_stats: BaseStats {
                    hp: 255,
                    attack: 10,
                    defense: 10,
                    special_attack: 75,
                    special_defense: 135,
                    speed: 55
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "serenegrace".to_string(),
                    hidden: "healer".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 46.8 as f32,
            }
        );
        
        pokedex.insert(
            "raikou".to_string(),
            PokedexPokemon {
                species: "raikou".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 85,
                    defense: 75,
                    special_attack: 115,
                    special_defense: 100,
                    speed: 115
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 178 as f32,
            }
        );
        
        pokedex.insert(
            "entei".to_string(),
            PokedexPokemon {
                species: "entei".to_string(),
                base_stats: BaseStats {
                    hp: 115,
                    attack: 115,
                    defense: 85,
                    special_attack: 90,
                    special_defense: 75,
                    speed: 100
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 198 as f32,
            }
        );
        
        pokedex.insert(
            "suicune".to_string(),
            PokedexPokemon {
                species: "suicune".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 75,
                    defense: 115,
                    special_attack: 90,
                    special_defense: 115,
                    speed: 85
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 187 as f32,
            }
        );
        
        pokedex.insert(
            "larvitar".to_string(),
            PokedexPokemon {
                species: "larvitar".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 64,
                    defense: 50,
                    special_attack: 45,
                    special_defense: 50,
                    speed: 41
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "none".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Ground
                ),
                weight: 72 as f32,
            }
        );
        
        pokedex.insert(
            "pupitar".to_string(),
            PokedexPokemon {
                species: "pupitar".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 84,
                    defense: 70,
                    special_attack: 65,
                    special_defense: 70,
                    speed: 51
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Ground
                ),
                weight: 152 as f32,
            }
        );
        
        pokedex.insert(
            "tyranitar".to_string(),
            PokedexPokemon {
                species: "tyranitar".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 134,
                    defense: 110,
                    special_attack: 95,
                    special_defense: 100,
                    speed: 61
                },
                abilities: Abilities {
                    first: "sandstream".to_string(),
                    second: "none".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Dark
                ),
                weight: 202 as f32,
            }
        );
        
        pokedex.insert(
            "tyranitarmega".to_string(),
            PokedexPokemon {
                species: "tyranitarmega".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 164,
                    defense: 150,
                    special_attack: 95,
                    special_defense: 120,
                    speed: 71
                },
                abilities: Abilities {
                    first: "sandstream".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Dark
                ),
                weight: 255 as f32,
            }
        );
        
        pokedex.insert(
            "lugia".to_string(),
            PokedexPokemon {
                species: "lugia".to_string(),
                base_stats: BaseStats {
                    hp: 106,
                    attack: 90,
                    defense: 130,
                    special_attack: 90,
                    special_defense: 154,
                    speed: 110
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "multiscale".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Flying
                ),
                weight: 216 as f32,
            }
        );
        
        pokedex.insert(
            "hooh".to_string(),
            PokedexPokemon {
                species: "hooh".to_string(),
                base_stats: BaseStats {
                    hp: 106,
                    attack: 130,
                    defense: 90,
                    special_attack: 110,
                    special_defense: 154,
                    speed: 90
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Flying
                ),
                weight: 199 as f32,
            }
        );
        
        pokedex.insert(
            "celebi".to_string(),
            PokedexPokemon {
                species: "celebi".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 100,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 100
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Grass
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "treecko".to_string(),
            PokedexPokemon {
                species: "treecko".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 35,
                    special_attack: 65,
                    special_defense: 55,
                    speed: 70
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "unburden".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "grovyle".to_string(),
            PokedexPokemon {
                species: "grovyle".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 45,
                    special_attack: 85,
                    special_defense: 65,
                    speed: 95
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "unburden".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 21.6 as f32,
            }
        );
        
        pokedex.insert(
            "sceptile".to_string(),
            PokedexPokemon {
                species: "sceptile".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 65,
                    special_attack: 105,
                    special_defense: 85,
                    speed: 120
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "unburden".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 52.2 as f32,
            }
        );
        
        pokedex.insert(
            "sceptilemega".to_string(),
            PokedexPokemon {
                species: "sceptilemega".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 110,
                    defense: 75,
                    special_attack: 145,
                    special_defense: 85,
                    speed: 145
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dragon
                ),
                weight: 55.2 as f32,
            }
        );
        
        pokedex.insert(
            "torchic".to_string(),
            PokedexPokemon {
                species: "torchic".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 60,
                    defense: 40,
                    special_attack: 70,
                    special_defense: 50,
                    speed: 45
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "speedboost".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 2.5 as f32,
            }
        );
        
        pokedex.insert(
            "combusken".to_string(),
            PokedexPokemon {
                species: "combusken".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 85,
                    defense: 60,
                    special_attack: 85,
                    special_defense: 60,
                    speed: 55
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "speedboost".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Fighting
                ),
                weight: 19.5 as f32,
            }
        );
        
        pokedex.insert(
            "blaziken".to_string(),
            PokedexPokemon {
                species: "blaziken".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 120,
                    defense: 70,
                    special_attack: 110,
                    special_defense: 70,
                    speed: 80
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "speedboost".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Fighting
                ),
                weight: 52 as f32,
            }
        );
        
        pokedex.insert(
            "blazikenmega".to_string(),
            PokedexPokemon {
                species: "blazikenmega".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 160,
                    defense: 80,
                    special_attack: 130,
                    special_defense: 80,
                    speed: 100
                },
                abilities: Abilities {
                    first: "speedboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Fighting
                ),
                weight: 52 as f32,
            }
        );
        
        pokedex.insert(
            "mudkip".to_string(),
            PokedexPokemon {
                species: "mudkip".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 70,
                    defense: 50,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 40
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 7.6 as f32,
            }
        );
        
        pokedex.insert(
            "marshtomp".to_string(),
            PokedexPokemon {
                species: "marshtomp".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 70,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 50
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "swampert".to_string(),
            PokedexPokemon {
                species: "swampert".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 110,
                    defense: 90,
                    special_attack: 85,
                    special_defense: 90,
                    speed: 60
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 81.9 as f32,
            }
        );
        
        pokedex.insert(
            "swampertmega".to_string(),
            PokedexPokemon {
                species: "swampertmega".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 150,
                    defense: 110,
                    special_attack: 95,
                    special_defense: 110,
                    speed: 70
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 102 as f32,
            }
        );
        
        pokedex.insert(
            "poochyena".to_string(),
            PokedexPokemon {
                species: "poochyena".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 35,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 35
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "quickfeet".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 13.6 as f32,
            }
        );
        
        pokedex.insert(
            "mightyena".to_string(),
            PokedexPokemon {
                species: "mightyena".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 90,
                    defense: 70,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 70
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "quickfeet".to_string(),
                    hidden: "moxie".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 37 as f32,
            }
        );
        
        pokedex.insert(
            "zigzagoon".to_string(),
            PokedexPokemon {
                species: "zigzagoon".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 30,
                    defense: 41,
                    special_attack: 30,
                    special_defense: 41,
                    speed: 60
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "quickfeet".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 17.5 as f32,
            }
        );
        
        pokedex.insert(
            "linoone".to_string(),
            PokedexPokemon {
                species: "linoone".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 70,
                    defense: 61,
                    special_attack: 50,
                    special_defense: 61,
                    speed: 100
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "quickfeet".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 32.5 as f32,
            }
        );
        
        pokedex.insert(
            "wurmple".to_string(),
            PokedexPokemon {
                species: "wurmple".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 45,
                    defense: 35,
                    special_attack: 20,
                    special_defense: 30,
                    speed: 20
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "none".to_string(),
                    hidden: "runaway".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 3.6 as f32,
            }
        );
        
        pokedex.insert(
            "silcoon".to_string(),
            PokedexPokemon {
                species: "silcoon".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 35,
                    defense: 55,
                    special_attack: 25,
                    special_defense: 25,
                    speed: 15
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 10 as f32,
            }
        );
        
        pokedex.insert(
            "beautifly".to_string(),
            PokedexPokemon {
                species: "beautifly".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 70,
                    defense: 50,
                    special_attack: 100,
                    special_defense: 50,
                    speed: 65
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "none".to_string(),
                    hidden: "rivalry".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 28.4 as f32,
            }
        );
        
        pokedex.insert(
            "cascoon".to_string(),
            PokedexPokemon {
                species: "cascoon".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 35,
                    defense: 55,
                    special_attack: 25,
                    special_defense: 25,
                    speed: 15
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 11.5 as f32,
            }
        );
        
        pokedex.insert(
            "dustox".to_string(),
            PokedexPokemon {
                species: "dustox".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 50,
                    defense: 70,
                    special_attack: 50,
                    special_defense: 90,
                    speed: 65
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "none".to_string(),
                    hidden: "compoundeyes".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 31.6 as f32,
            }
        );
        
        pokedex.insert(
            "lotad".to_string(),
            PokedexPokemon {
                species: "lotad".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 30,
                    defense: 30,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 30
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "raindish".to_string(),
                    hidden: "owntempo".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Grass
                ),
                weight: 2.6 as f32,
            }
        );
        
        pokedex.insert(
            "lombre".to_string(),
            PokedexPokemon {
                species: "lombre".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 50,
                    defense: 50,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 50
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "raindish".to_string(),
                    hidden: "owntempo".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Grass
                ),
                weight: 32.5 as f32,
            }
        );
        
        pokedex.insert(
            "ludicolo".to_string(),
            PokedexPokemon {
                species: "ludicolo".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 70,
                    defense: 70,
                    special_attack: 90,
                    special_defense: 100,
                    speed: 70
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "raindish".to_string(),
                    hidden: "owntempo".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Grass
                ),
                weight: 55 as f32,
            }
        );
        
        pokedex.insert(
            "seedot".to_string(),
            PokedexPokemon {
                species: "seedot".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 40,
                    defense: 50,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 30
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "nuzleaf".to_string(),
            PokedexPokemon {
                species: "nuzleaf".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 70,
                    defense: 40,
                    special_attack: 60,
                    special_defense: 40,
                    speed: 60
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dark
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "shiftry".to_string(),
            PokedexPokemon {
                species: "shiftry".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 100,
                    defense: 60,
                    special_attack: 90,
                    special_defense: 60,
                    speed: 80
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "earlybird".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dark
                ),
                weight: 59.6 as f32,
            }
        );
        
        pokedex.insert(
            "taillow".to_string(),
            PokedexPokemon {
                species: "taillow".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 55,
                    defense: 30,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 85
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "none".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 2.3 as f32,
            }
        );
        
        pokedex.insert(
            "swellow".to_string(),
            PokedexPokemon {
                species: "swellow".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 85,
                    defense: 60,
                    special_attack: 75,
                    special_defense: 50,
                    speed: 125
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "none".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 19.8 as f32,
            }
        );
        
        pokedex.insert(
            "wingull".to_string(),
            PokedexPokemon {
                species: "wingull".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 30,
                    defense: 30,
                    special_attack: 55,
                    special_defense: 30,
                    speed: 85
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "hydration".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Flying
                ),
                weight: 9.5 as f32,
            }
        );
        
        pokedex.insert(
            "pelipper".to_string(),
            PokedexPokemon {
                species: "pelipper".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 50,
                    defense: 100,
                    special_attack: 95,
                    special_defense: 70,
                    speed: 65
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "drizzle".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Flying
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "ralts".to_string(),
            PokedexPokemon {
                species: "ralts".to_string(),
                base_stats: BaseStats {
                    hp: 28,
                    attack: 25,
                    defense: 25,
                    special_attack: 45,
                    special_defense: 35,
                    speed: 40
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "trace".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 6.6 as f32,
            }
        );
        
        pokedex.insert(
            "kirlia".to_string(),
            PokedexPokemon {
                species: "kirlia".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 35,
                    defense: 35,
                    special_attack: 65,
                    special_defense: 55,
                    speed: 50
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "trace".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 20.2 as f32,
            }
        );
        
        pokedex.insert(
            "gardevoir".to_string(),
            PokedexPokemon {
                species: "gardevoir".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 65,
                    defense: 65,
                    special_attack: 125,
                    special_defense: 115,
                    speed: 80
                },
                abilities: Abilities {
                    first: "synchronize".to_string(),
                    second: "trace".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 48.4 as f32,
            }
        );
        
        pokedex.insert(
            "gardevoirmega".to_string(),
            PokedexPokemon {
                species: "gardevoirmega".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 85,
                    defense: 65,
                    special_attack: 165,
                    special_defense: 135,
                    speed: 100
                },
                abilities: Abilities {
                    first: "pixilate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 48.4 as f32,
            }
        );
        
        pokedex.insert(
            "surskit".to_string(),
            PokedexPokemon {
                species: "surskit".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 30,
                    defense: 32,
                    special_attack: 50,
                    special_defense: 52,
                    speed: 65
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Water
                ),
                weight: 1.7 as f32,
            }
        );
        
        pokedex.insert(
            "masquerain".to_string(),
            PokedexPokemon {
                species: "masquerain".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 60,
                    defense: 62,
                    special_attack: 100,
                    special_defense: 82,
                    speed: 80
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "none".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 3.6 as f32,
            }
        );
        
        pokedex.insert(
            "shroomish".to_string(),
            PokedexPokemon {
                species: "shroomish".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 40,
                    defense: 60,
                    special_attack: 40,
                    special_defense: 60,
                    speed: 35
                },
                abilities: Abilities {
                    first: "effectspore".to_string(),
                    second: "poisonheal".to_string(),
                    hidden: "quickfeet".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 4.5 as f32,
            }
        );
        
        pokedex.insert(
            "breloom".to_string(),
            PokedexPokemon {
                species: "breloom".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 130,
                    defense: 80,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 70
                },
                abilities: Abilities {
                    first: "effectspore".to_string(),
                    second: "poisonheal".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Fighting
                ),
                weight: 39.2 as f32,
            }
        );
        
        pokedex.insert(
            "slakoth".to_string(),
            PokedexPokemon {
                species: "slakoth".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 60,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 30
                },
                abilities: Abilities {
                    first: "truant".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 24 as f32,
            }
        );
        
        pokedex.insert(
            "vigoroth".to_string(),
            PokedexPokemon {
                species: "vigoroth".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 80,
                    defense: 80,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 90
                },
                abilities: Abilities {
                    first: "vitalspirit".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 46.5 as f32,
            }
        );
        
        pokedex.insert(
            "slaking".to_string(),
            PokedexPokemon {
                species: "slaking".to_string(),
                base_stats: BaseStats {
                    hp: 150,
                    attack: 160,
                    defense: 100,
                    special_attack: 95,
                    special_defense: 65,
                    speed: 100
                },
                abilities: Abilities {
                    first: "truant".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 130.5 as f32,
            }
        );
        
        pokedex.insert(
            "nincada".to_string(),
            PokedexPokemon {
                species: "nincada".to_string(),
                base_stats: BaseStats {
                    hp: 31,
                    attack: 45,
                    defense: 90,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 40
                },
                abilities: Abilities {
                    first: "compoundeyes".to_string(),
                    second: "none".to_string(),
                    hidden: "runaway".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Ground
                ),
                weight: 5.5 as f32,
            }
        );
        
        pokedex.insert(
            "ninjask".to_string(),
            PokedexPokemon {
                species: "ninjask".to_string(),
                base_stats: BaseStats {
                    hp: 61,
                    attack: 90,
                    defense: 45,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 160
                },
                abilities: Abilities {
                    first: "speedboost".to_string(),
                    second: "none".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "shedinja".to_string(),
            PokedexPokemon {
                species: "shedinja".to_string(),
                base_stats: BaseStats {
                    hp: 1,
                    attack: 90,
                    defense: 45,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 40
                },
                abilities: Abilities {
                    first: "wonderguard".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Ghost
                ),
                weight: 1.2 as f32,
            }
        );
        
        pokedex.insert(
            "whismur".to_string(),
            PokedexPokemon {
                species: "whismur".to_string(),
                base_stats: BaseStats {
                    hp: 64,
                    attack: 51,
                    defense: 23,
                    special_attack: 51,
                    special_defense: 23,
                    speed: 28
                },
                abilities: Abilities {
                    first: "soundproof".to_string(),
                    second: "none".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 16.3 as f32,
            }
        );
        
        pokedex.insert(
            "loudred".to_string(),
            PokedexPokemon {
                species: "loudred".to_string(),
                base_stats: BaseStats {
                    hp: 84,
                    attack: 71,
                    defense: 43,
                    special_attack: 71,
                    special_defense: 43,
                    speed: 48
                },
                abilities: Abilities {
                    first: "soundproof".to_string(),
                    second: "none".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 40.5 as f32,
            }
        );
        
        pokedex.insert(
            "exploud".to_string(),
            PokedexPokemon {
                species: "exploud".to_string(),
                base_stats: BaseStats {
                    hp: 104,
                    attack: 91,
                    defense: 63,
                    special_attack: 91,
                    special_defense: 73,
                    speed: 68
                },
                abilities: Abilities {
                    first: "soundproof".to_string(),
                    second: "none".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 84 as f32,
            }
        );
        
        pokedex.insert(
            "makuhita".to_string(),
            PokedexPokemon {
                species: "makuhita".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 60,
                    defense: 30,
                    special_attack: 20,
                    special_defense: 30,
                    speed: 25
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "guts".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 86.4 as f32,
            }
        );
        
        pokedex.insert(
            "hariyama".to_string(),
            PokedexPokemon {
                species: "hariyama".to_string(),
                base_stats: BaseStats {
                    hp: 144,
                    attack: 120,
                    defense: 60,
                    special_attack: 40,
                    special_defense: 60,
                    speed: 50
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "guts".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 253.8 as f32,
            }
        );
        
        pokedex.insert(
            "azurill".to_string(),
            PokedexPokemon {
                species: "azurill".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 20,
                    defense: 40,
                    special_attack: 20,
                    special_defense: 40,
                    speed: 20
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "hugepower".to_string(),
                    hidden: "sapsipper".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fairy
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "nosepass".to_string(),
            PokedexPokemon {
                species: "nosepass".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 45,
                    defense: 135,
                    special_attack: 45,
                    special_defense: 90,
                    speed: 30
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "magnetpull".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 97 as f32,
            }
        );
        
        pokedex.insert(
            "skitty".to_string(),
            PokedexPokemon {
                species: "skitty".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 45,
                    defense: 45,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 50
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "normalize".to_string(),
                    hidden: "wonderskin".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 11 as f32,
            }
        );
        
        pokedex.insert(
            "delcatty".to_string(),
            PokedexPokemon {
                species: "delcatty".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 65,
                    defense: 65,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 90
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "normalize".to_string(),
                    hidden: "wonderskin".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 32.6 as f32,
            }
        );
        
        pokedex.insert(
            "sableye".to_string(),
            PokedexPokemon {
                species: "sableye".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 75,
                    defense: 75,
                    special_attack: 65,
                    special_defense: 65,
                    speed: 50
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "stall".to_string(),
                    hidden: "prankster".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Ghost
                ),
                weight: 11 as f32,
            }
        );
        
        pokedex.insert(
            "sableyemega".to_string(),
            PokedexPokemon {
                species: "sableyemega".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 85,
                    defense: 125,
                    special_attack: 85,
                    special_defense: 115,
                    speed: 20
                },
                abilities: Abilities {
                    first: "magicbounce".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Ghost
                ),
                weight: 161 as f32,
            }
        );
        
        pokedex.insert(
            "mawile".to_string(),
            PokedexPokemon {
                species: "mawile".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 85,
                    defense: 85,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 50
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "intimidate".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Fairy
                ),
                weight: 11.5 as f32,
            }
        );
        
        pokedex.insert(
            "mawilemega".to_string(),
            PokedexPokemon {
                species: "mawilemega".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 105,
                    defense: 125,
                    special_attack: 55,
                    special_defense: 95,
                    speed: 50
                },
                abilities: Abilities {
                    first: "hugepower".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Fairy
                ),
                weight: 23.5 as f32,
            }
        );
        
        pokedex.insert(
            "aron".to_string(),
            PokedexPokemon {
                species: "aron".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 70,
                    defense: 100,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 30
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "rockhead".to_string(),
                    hidden: "heavymetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Rock
                ),
                weight: 60 as f32,
            }
        );
        
        pokedex.insert(
            "lairon".to_string(),
            PokedexPokemon {
                species: "lairon".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 90,
                    defense: 140,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 40
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "rockhead".to_string(),
                    hidden: "heavymetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Rock
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "aggron".to_string(),
            PokedexPokemon {
                species: "aggron".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 110,
                    defense: 180,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 50
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "rockhead".to_string(),
                    hidden: "heavymetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Rock
                ),
                weight: 360 as f32,
            }
        );
        
        pokedex.insert(
            "aggronmega".to_string(),
            PokedexPokemon {
                species: "aggronmega".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 140,
                    defense: 230,
                    special_attack: 60,
                    special_defense: 80,
                    speed: 50
                },
                abilities: Abilities {
                    first: "filter".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 395 as f32,
            }
        );
        
        pokedex.insert(
            "meditite".to_string(),
            PokedexPokemon {
                species: "meditite".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 40,
                    defense: 55,
                    special_attack: 40,
                    special_defense: 55,
                    speed: 60
                },
                abilities: Abilities {
                    first: "purepower".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Psychic
                ),
                weight: 11.2 as f32,
            }
        );
        
        pokedex.insert(
            "medicham".to_string(),
            PokedexPokemon {
                species: "medicham".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 75,
                    special_attack: 60,
                    special_defense: 75,
                    speed: 80
                },
                abilities: Abilities {
                    first: "purepower".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Psychic
                ),
                weight: 31.5 as f32,
            }
        );
        
        pokedex.insert(
            "medichammega".to_string(),
            PokedexPokemon {
                species: "medichammega".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 100,
                    defense: 85,
                    special_attack: 80,
                    special_defense: 85,
                    speed: 100
                },
                abilities: Abilities {
                    first: "purepower".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Psychic
                ),
                weight: 31.5 as f32,
            }
        );
        
        pokedex.insert(
            "electrike".to_string(),
            PokedexPokemon {
                species: "electrike".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 40,
                    special_attack: 65,
                    special_defense: 40,
                    speed: 65
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "lightningrod".to_string(),
                    hidden: "minus".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 15.2 as f32,
            }
        );
        
        pokedex.insert(
            "manectric".to_string(),
            PokedexPokemon {
                species: "manectric".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 75,
                    defense: 60,
                    special_attack: 105,
                    special_defense: 60,
                    speed: 105
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "lightningrod".to_string(),
                    hidden: "minus".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 40.2 as f32,
            }
        );
        
        pokedex.insert(
            "manectricmega".to_string(),
            PokedexPokemon {
                species: "manectricmega".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 75,
                    defense: 80,
                    special_attack: 135,
                    special_defense: 80,
                    speed: 135
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 44 as f32,
            }
        );
        
        pokedex.insert(
            "plusle".to_string(),
            PokedexPokemon {
                species: "plusle".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 50,
                    defense: 40,
                    special_attack: 85,
                    special_defense: 75,
                    speed: 95
                },
                abilities: Abilities {
                    first: "plus".to_string(),
                    second: "none".to_string(),
                    hidden: "lightningrod".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 4.2 as f32,
            }
        );
        
        pokedex.insert(
            "minun".to_string(),
            PokedexPokemon {
                species: "minun".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 40,
                    defense: 50,
                    special_attack: 75,
                    special_defense: 85,
                    speed: 95
                },
                abilities: Abilities {
                    first: "minus".to_string(),
                    second: "none".to_string(),
                    hidden: "voltabsorb".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 4.2 as f32,
            }
        );
        
        pokedex.insert(
            "volbeat".to_string(),
            PokedexPokemon {
                species: "volbeat".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 73,
                    defense: 75,
                    special_attack: 47,
                    special_defense: 85,
                    speed: 85
                },
                abilities: Abilities {
                    first: "illuminate".to_string(),
                    second: "swarm".to_string(),
                    hidden: "prankster".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 17.7 as f32,
            }
        );
        
        pokedex.insert(
            "illumise".to_string(),
            PokedexPokemon {
                species: "illumise".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 47,
                    defense: 75,
                    special_attack: 73,
                    special_defense: 85,
                    speed: 85
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "tintedlens".to_string(),
                    hidden: "prankster".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 17.7 as f32,
            }
        );
        
        pokedex.insert(
            "roselia".to_string(),
            PokedexPokemon {
                species: "roselia".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 60,
                    defense: 45,
                    special_attack: 100,
                    special_defense: 80,
                    speed: 65
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "poisonpoint".to_string(),
                    hidden: "leafguard".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "gulpin".to_string(),
            PokedexPokemon {
                species: "gulpin".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 43,
                    defense: 53,
                    special_attack: 43,
                    special_defense: 53,
                    speed: 40
                },
                abilities: Abilities {
                    first: "liquidooze".to_string(),
                    second: "stickyhold".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 10.3 as f32,
            }
        );
        
        pokedex.insert(
            "swalot".to_string(),
            PokedexPokemon {
                species: "swalot".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 73,
                    defense: 83,
                    special_attack: 73,
                    special_defense: 83,
                    speed: 55
                },
                abilities: Abilities {
                    first: "liquidooze".to_string(),
                    second: "stickyhold".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 80 as f32,
            }
        );
        
        pokedex.insert(
            "carvanha".to_string(),
            PokedexPokemon {
                species: "carvanha".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 90,
                    defense: 20,
                    special_attack: 65,
                    special_defense: 20,
                    speed: 65
                },
                abilities: Abilities {
                    first: "roughskin".to_string(),
                    second: "none".to_string(),
                    hidden: "speedboost".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dark
                ),
                weight: 20.8 as f32,
            }
        );
        
        pokedex.insert(
            "sharpedo".to_string(),
            PokedexPokemon {
                species: "sharpedo".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 120,
                    defense: 40,
                    special_attack: 95,
                    special_defense: 40,
                    speed: 95
                },
                abilities: Abilities {
                    first: "roughskin".to_string(),
                    second: "none".to_string(),
                    hidden: "speedboost".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dark
                ),
                weight: 88.8 as f32,
            }
        );
        
        pokedex.insert(
            "sharpedomega".to_string(),
            PokedexPokemon {
                species: "sharpedomega".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 140,
                    defense: 70,
                    special_attack: 110,
                    special_defense: 65,
                    speed: 105
                },
                abilities: Abilities {
                    first: "strongjaw".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dark
                ),
                weight: 130.3 as f32,
            }
        );
        
        pokedex.insert(
            "wailmer".to_string(),
            PokedexPokemon {
                species: "wailmer".to_string(),
                base_stats: BaseStats {
                    hp: 130,
                    attack: 70,
                    defense: 35,
                    special_attack: 70,
                    special_defense: 35,
                    speed: 60
                },
                abilities: Abilities {
                    first: "waterveil".to_string(),
                    second: "oblivious".to_string(),
                    hidden: "pressure".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 130 as f32,
            }
        );
        
        pokedex.insert(
            "wailord".to_string(),
            PokedexPokemon {
                species: "wailord".to_string(),
                base_stats: BaseStats {
                    hp: 170,
                    attack: 90,
                    defense: 45,
                    special_attack: 90,
                    special_defense: 45,
                    speed: 60
                },
                abilities: Abilities {
                    first: "waterveil".to_string(),
                    second: "oblivious".to_string(),
                    hidden: "pressure".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 398 as f32,
            }
        );
        
        pokedex.insert(
            "numel".to_string(),
            PokedexPokemon {
                species: "numel".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 40,
                    special_attack: 65,
                    special_defense: 45,
                    speed: 35
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "simple".to_string(),
                    hidden: "owntempo".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Ground
                ),
                weight: 24 as f32,
            }
        );
        
        pokedex.insert(
            "camerupt".to_string(),
            PokedexPokemon {
                species: "camerupt".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 100,
                    defense: 70,
                    special_attack: 105,
                    special_defense: 75,
                    speed: 40
                },
                abilities: Abilities {
                    first: "magmaarmor".to_string(),
                    second: "solidrock".to_string(),
                    hidden: "angerpoint".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Ground
                ),
                weight: 220 as f32,
            }
        );
        
        pokedex.insert(
            "cameruptmega".to_string(),
            PokedexPokemon {
                species: "cameruptmega".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 120,
                    defense: 100,
                    special_attack: 145,
                    special_defense: 105,
                    speed: 20
                },
                abilities: Abilities {
                    first: "sheerforce".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Ground
                ),
                weight: 320.5 as f32,
            }
        );
        
        pokedex.insert(
            "torkoal".to_string(),
            PokedexPokemon {
                species: "torkoal".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 140,
                    special_attack: 85,
                    special_defense: 70,
                    speed: 20
                },
                abilities: Abilities {
                    first: "whitesmoke".to_string(),
                    second: "drought".to_string(),
                    hidden: "shellarmor".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 80.4 as f32,
            }
        );
        
        pokedex.insert(
            "spoink".to_string(),
            PokedexPokemon {
                species: "spoink".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 25,
                    defense: 35,
                    special_attack: 70,
                    special_defense: 80,
                    speed: 60
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 30.6 as f32,
            }
        );
        
        pokedex.insert(
            "grumpig".to_string(),
            PokedexPokemon {
                species: "grumpig".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 45,
                    defense: 65,
                    special_attack: 90,
                    special_defense: 110,
                    speed: 80
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 71.5 as f32,
            }
        );
        
        pokedex.insert(
            "spinda".to_string(),
            PokedexPokemon {
                species: "spinda".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 60,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 60
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "tangledfeet".to_string(),
                    hidden: "contrary".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "trapinch".to_string(),
            PokedexPokemon {
                species: "trapinch".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 100,
                    defense: 45,
                    special_attack: 45,
                    special_defense: 45,
                    speed: 10
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "arenatrap".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "vibrava".to_string(),
            PokedexPokemon {
                species: "vibrava".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 70,
                    defense: 50,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 70
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Dragon
                ),
                weight: 15.3 as f32,
            }
        );
        
        pokedex.insert(
            "flygon".to_string(),
            PokedexPokemon {
                species: "flygon".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 100,
                    defense: 80,
                    special_attack: 80,
                    special_defense: 80,
                    speed: 100
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Dragon
                ),
                weight: 82 as f32,
            }
        );
        
        pokedex.insert(
            "cacnea".to_string(),
            PokedexPokemon {
                species: "cacnea".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 85,
                    defense: 40,
                    special_attack: 85,
                    special_defense: 40,
                    speed: 35
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "none".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 51.3 as f32,
            }
        );
        
        pokedex.insert(
            "cacturne".to_string(),
            PokedexPokemon {
                species: "cacturne".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 115,
                    defense: 60,
                    special_attack: 115,
                    special_defense: 60,
                    speed: 55
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "none".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dark
                ),
                weight: 77.4 as f32,
            }
        );
        
        pokedex.insert(
            "swablu".to_string(),
            PokedexPokemon {
                species: "swablu".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 40,
                    defense: 60,
                    special_attack: 40,
                    special_defense: 75,
                    speed: 50
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "none".to_string(),
                    hidden: "cloudnine".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 1.2 as f32,
            }
        );
        
        pokedex.insert(
            "altaria".to_string(),
            PokedexPokemon {
                species: "altaria".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 70,
                    defense: 90,
                    special_attack: 70,
                    special_defense: 105,
                    speed: 80
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "none".to_string(),
                    hidden: "cloudnine".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Flying
                ),
                weight: 20.6 as f32,
            }
        );
        
        pokedex.insert(
            "altariamega".to_string(),
            PokedexPokemon {
                species: "altariamega".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 110,
                    defense: 110,
                    special_attack: 110,
                    special_defense: 105,
                    speed: 80
                },
                abilities: Abilities {
                    first: "pixilate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Fairy
                ),
                weight: 20.6 as f32,
            }
        );
        
        pokedex.insert(
            "zangoose".to_string(),
            PokedexPokemon {
                species: "zangoose".to_string(),
                base_stats: BaseStats {
                    hp: 73,
                    attack: 115,
                    defense: 60,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 90
                },
                abilities: Abilities {
                    first: "immunity".to_string(),
                    second: "none".to_string(),
                    hidden: "toxicboost".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 40.3 as f32,
            }
        );
        
        pokedex.insert(
            "seviper".to_string(),
            PokedexPokemon {
                species: "seviper".to_string(),
                base_stats: BaseStats {
                    hp: 73,
                    attack: 100,
                    defense: 60,
                    special_attack: 100,
                    special_defense: 60,
                    speed: 65
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 52.5 as f32,
            }
        );
        
        pokedex.insert(
            "lunatone".to_string(),
            PokedexPokemon {
                species: "lunatone".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 55,
                    defense: 65,
                    special_attack: 95,
                    special_defense: 85,
                    speed: 70
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Psychic
                ),
                weight: 168 as f32,
            }
        );
        
        pokedex.insert(
            "solrock".to_string(),
            PokedexPokemon {
                species: "solrock".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 95,
                    defense: 85,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 70
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Psychic
                ),
                weight: 154 as f32,
            }
        );
        
        pokedex.insert(
            "barboach".to_string(),
            PokedexPokemon {
                species: "barboach".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 48,
                    defense: 43,
                    special_attack: 46,
                    special_defense: 41,
                    speed: 60
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "anticipation".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 1.9 as f32,
            }
        );
        
        pokedex.insert(
            "whiscash".to_string(),
            PokedexPokemon {
                species: "whiscash".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 78,
                    defense: 73,
                    special_attack: 76,
                    special_defense: 71,
                    speed: 60
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "anticipation".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 23.6 as f32,
            }
        );
        
        pokedex.insert(
            "corphish".to_string(),
            PokedexPokemon {
                species: "corphish".to_string(),
                base_stats: BaseStats {
                    hp: 43,
                    attack: 80,
                    defense: 65,
                    special_attack: 50,
                    special_defense: 35,
                    speed: 35
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "adaptability".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 11.5 as f32,
            }
        );
        
        pokedex.insert(
            "crawdaunt".to_string(),
            PokedexPokemon {
                species: "crawdaunt".to_string(),
                base_stats: BaseStats {
                    hp: 63,
                    attack: 120,
                    defense: 85,
                    special_attack: 90,
                    special_defense: 55,
                    speed: 55
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "adaptability".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dark
                ),
                weight: 32.8 as f32,
            }
        );
        
        pokedex.insert(
            "baltoy".to_string(),
            PokedexPokemon {
                species: "baltoy".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 40,
                    defense: 55,
                    special_attack: 40,
                    special_defense: 70,
                    speed: 55
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Psychic
                ),
                weight: 21.5 as f32,
            }
        );
        
        pokedex.insert(
            "claydol".to_string(),
            PokedexPokemon {
                species: "claydol".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 70,
                    defense: 105,
                    special_attack: 70,
                    special_defense: 120,
                    speed: 75
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Psychic
                ),
                weight: 108 as f32,
            }
        );
        
        pokedex.insert(
            "lileep".to_string(),
            PokedexPokemon {
                species: "lileep".to_string(),
                base_stats: BaseStats {
                    hp: 66,
                    attack: 41,
                    defense: 77,
                    special_attack: 61,
                    special_defense: 87,
                    speed: 23
                },
                abilities: Abilities {
                    first: "suctioncups".to_string(),
                    second: "none".to_string(),
                    hidden: "stormdrain".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Grass
                ),
                weight: 23.8 as f32,
            }
        );
        
        pokedex.insert(
            "cradily".to_string(),
            PokedexPokemon {
                species: "cradily".to_string(),
                base_stats: BaseStats {
                    hp: 86,
                    attack: 81,
                    defense: 97,
                    special_attack: 81,
                    special_defense: 107,
                    speed: 43
                },
                abilities: Abilities {
                    first: "suctioncups".to_string(),
                    second: "none".to_string(),
                    hidden: "stormdrain".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Grass
                ),
                weight: 60.4 as f32,
            }
        );
        
        pokedex.insert(
            "anorith".to_string(),
            PokedexPokemon {
                species: "anorith".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 95,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 75
                },
                abilities: Abilities {
                    first: "battlearmor".to_string(),
                    second: "none".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Bug
                ),
                weight: 12.5 as f32,
            }
        );
        
        pokedex.insert(
            "armaldo".to_string(),
            PokedexPokemon {
                species: "armaldo".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 125,
                    defense: 100,
                    special_attack: 70,
                    special_defense: 80,
                    speed: 45
                },
                abilities: Abilities {
                    first: "battlearmor".to_string(),
                    second: "none".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Bug
                ),
                weight: 68.2 as f32,
            }
        );
        
        pokedex.insert(
            "feebas".to_string(),
            PokedexPokemon {
                species: "feebas".to_string(),
                base_stats: BaseStats {
                    hp: 20,
                    attack: 15,
                    defense: 20,
                    special_attack: 10,
                    special_defense: 55,
                    speed: 80
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "oblivious".to_string(),
                    hidden: "adaptability".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 7.4 as f32,
            }
        );
        
        pokedex.insert(
            "milotic".to_string(),
            PokedexPokemon {
                species: "milotic".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 60,
                    defense: 79,
                    special_attack: 100,
                    special_defense: 125,
                    speed: 81
                },
                abilities: Abilities {
                    first: "marvelscale".to_string(),
                    second: "competitive".to_string(),
                    hidden: "cutecharm".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 162 as f32,
            }
        );
        
        pokedex.insert(
            "castform".to_string(),
            PokedexPokemon {
                species: "castform".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 70,
                    defense: 70,
                    special_attack: 70,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "forecast".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 0.8 as f32,
            }
        );
        
        pokedex.insert(
            "castformsunny".to_string(),
            PokedexPokemon {
                species: "castformsunny".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 70,
                    defense: 70,
                    special_attack: 70,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "forecast".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 0.8 as f32,
            }
        );
        
        pokedex.insert(
            "castformrainy".to_string(),
            PokedexPokemon {
                species: "castformrainy".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 70,
                    defense: 70,
                    special_attack: 70,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "forecast".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 0.8 as f32,
            }
        );
        
        pokedex.insert(
            "castformsnowy".to_string(),
            PokedexPokemon {
                species: "castformsnowy".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 70,
                    defense: 70,
                    special_attack: 70,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "forecast".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 0.8 as f32,
            }
        );
        
        pokedex.insert(
            "kecleon".to_string(),
            PokedexPokemon {
                species: "kecleon".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 90,
                    defense: 70,
                    special_attack: 60,
                    special_defense: 120,
                    speed: 40
                },
                abilities: Abilities {
                    first: "colorchange".to_string(),
                    second: "none".to_string(),
                    hidden: "protean".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 22 as f32,
            }
        );
        
        pokedex.insert(
            "shuppet".to_string(),
            PokedexPokemon {
                species: "shuppet".to_string(),
                base_stats: BaseStats {
                    hp: 44,
                    attack: 75,
                    defense: 35,
                    special_attack: 63,
                    special_defense: 33,
                    speed: 45
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "frisk".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 2.3 as f32,
            }
        );
        
        pokedex.insert(
            "banette".to_string(),
            PokedexPokemon {
                species: "banette".to_string(),
                base_stats: BaseStats {
                    hp: 64,
                    attack: 115,
                    defense: 65,
                    special_attack: 83,
                    special_defense: 63,
                    speed: 65
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "frisk".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 12.5 as f32,
            }
        );
        
        pokedex.insert(
            "banettemega".to_string(),
            PokedexPokemon {
                species: "banettemega".to_string(),
                base_stats: BaseStats {
                    hp: 64,
                    attack: 165,
                    defense: 75,
                    special_attack: 93,
                    special_defense: 83,
                    speed: 75
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 13 as f32,
            }
        );
        
        pokedex.insert(
            "duskull".to_string(),
            PokedexPokemon {
                species: "duskull".to_string(),
                base_stats: BaseStats {
                    hp: 20,
                    attack: 40,
                    defense: 90,
                    special_attack: 30,
                    special_defense: 90,
                    speed: 25
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "frisk".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "dusclops".to_string(),
            PokedexPokemon {
                species: "dusclops".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 70,
                    defense: 130,
                    special_attack: 60,
                    special_defense: 130,
                    speed: 25
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "frisk".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 30.6 as f32,
            }
        );
        
        pokedex.insert(
            "tropius".to_string(),
            PokedexPokemon {
                species: "tropius".to_string(),
                base_stats: BaseStats {
                    hp: 99,
                    attack: 68,
                    defense: 83,
                    special_attack: 72,
                    special_defense: 87,
                    speed: 51
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "solarpower".to_string(),
                    hidden: "harvest".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Flying
                ),
                weight: 100 as f32,
            }
        );
        
        pokedex.insert(
            "chimecho".to_string(),
            PokedexPokemon {
                species: "chimecho".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 50,
                    defense: 80,
                    special_attack: 95,
                    special_defense: 90,
                    speed: 65
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "absol".to_string(),
            PokedexPokemon {
                species: "absol".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 130,
                    defense: 60,
                    special_attack: 75,
                    special_defense: 60,
                    speed: 75
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "superluck".to_string(),
                    hidden: "justified".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 47 as f32,
            }
        );
        
        pokedex.insert(
            "absolmega".to_string(),
            PokedexPokemon {
                species: "absolmega".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 150,
                    defense: 60,
                    special_attack: 115,
                    special_defense: 60,
                    speed: 115
                },
                abilities: Abilities {
                    first: "magicbounce".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 49 as f32,
            }
        );
        
        pokedex.insert(
            "wynaut".to_string(),
            PokedexPokemon {
                species: "wynaut".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 23,
                    defense: 48,
                    special_attack: 23,
                    special_defense: 48,
                    speed: 23
                },
                abilities: Abilities {
                    first: "shadowtag".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 14 as f32,
            }
        );
        
        pokedex.insert(
            "snorunt".to_string(),
            PokedexPokemon {
                species: "snorunt".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 50,
                    defense: 50,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 50
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "icebody".to_string(),
                    hidden: "moody".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 16.8 as f32,
            }
        );
        
        pokedex.insert(
            "glalie".to_string(),
            PokedexPokemon {
                species: "glalie".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 80,
                    defense: 80,
                    special_attack: 80,
                    special_defense: 80,
                    speed: 80
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "icebody".to_string(),
                    hidden: "moody".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 256.5 as f32,
            }
        );
        
        pokedex.insert(
            "glaliemega".to_string(),
            PokedexPokemon {
                species: "glaliemega".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 120,
                    defense: 80,
                    special_attack: 120,
                    special_defense: 80,
                    speed: 100
                },
                abilities: Abilities {
                    first: "refrigerate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 350.2 as f32,
            }
        );
        
        pokedex.insert(
            "spheal".to_string(),
            PokedexPokemon {
                species: "spheal".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 40,
                    defense: 50,
                    special_attack: 55,
                    special_defense: 50,
                    speed: 25
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "icebody".to_string(),
                    hidden: "oblivious".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Water
                ),
                weight: 39.5 as f32,
            }
        );
        
        pokedex.insert(
            "sealeo".to_string(),
            PokedexPokemon {
                species: "sealeo".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 60,
                    defense: 70,
                    special_attack: 75,
                    special_defense: 70,
                    speed: 45
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "icebody".to_string(),
                    hidden: "oblivious".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Water
                ),
                weight: 87.6 as f32,
            }
        );
        
        pokedex.insert(
            "walrein".to_string(),
            PokedexPokemon {
                species: "walrein".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 80,
                    defense: 90,
                    special_attack: 95,
                    special_defense: 90,
                    speed: 65
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "icebody".to_string(),
                    hidden: "oblivious".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Water
                ),
                weight: 150.6 as f32,
            }
        );
        
        pokedex.insert(
            "clamperl".to_string(),
            PokedexPokemon {
                species: "clamperl".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 64,
                    defense: 85,
                    special_attack: 74,
                    special_defense: 55,
                    speed: 32
                },
                abilities: Abilities {
                    first: "shellarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 52.5 as f32,
            }
        );
        
        pokedex.insert(
            "huntail".to_string(),
            PokedexPokemon {
                species: "huntail".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 104,
                    defense: 105,
                    special_attack: 94,
                    special_defense: 75,
                    speed: 52
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "waterveil".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 27 as f32,
            }
        );
        
        pokedex.insert(
            "gorebyss".to_string(),
            PokedexPokemon {
                species: "gorebyss".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 84,
                    defense: 105,
                    special_attack: 114,
                    special_defense: 75,
                    speed: 52
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 22.6 as f32,
            }
        );
        
        pokedex.insert(
            "relicanth".to_string(),
            PokedexPokemon {
                species: "relicanth".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 90,
                    defense: 130,
                    special_attack: 45,
                    special_defense: 65,
                    speed: 55
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "rockhead".to_string(),
                    hidden: "sturdy".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Rock
                ),
                weight: 23.4 as f32,
            }
        );
        
        pokedex.insert(
            "luvdisc".to_string(),
            PokedexPokemon {
                species: "luvdisc".to_string(),
                base_stats: BaseStats {
                    hp: 43,
                    attack: 30,
                    defense: 55,
                    special_attack: 40,
                    special_defense: 65,
                    speed: 97
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 8.7 as f32,
            }
        );
        
        pokedex.insert(
            "bagon".to_string(),
            PokedexPokemon {
                species: "bagon".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 75,
                    defense: 60,
                    special_attack: 40,
                    special_defense: 30,
                    speed: 50
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "none".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 42.1 as f32,
            }
        );
        
        pokedex.insert(
            "shelgon".to_string(),
            PokedexPokemon {
                species: "shelgon".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 95,
                    defense: 100,
                    special_attack: 60,
                    special_defense: 50,
                    speed: 50
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "none".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 110.5 as f32,
            }
        );
        
        pokedex.insert(
            "salamence".to_string(),
            PokedexPokemon {
                species: "salamence".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 135,
                    defense: 80,
                    special_attack: 110,
                    special_defense: 80,
                    speed: 100
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "none".to_string(),
                    hidden: "moxie".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Flying
                ),
                weight: 102.6 as f32,
            }
        );
        
        pokedex.insert(
            "salamencemega".to_string(),
            PokedexPokemon {
                species: "salamencemega".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 145,
                    defense: 130,
                    special_attack: 120,
                    special_defense: 90,
                    speed: 120
                },
                abilities: Abilities {
                    first: "aerilate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Flying
                ),
                weight: 112.6 as f32,
            }
        );
        
        pokedex.insert(
            "beldum".to_string(),
            PokedexPokemon {
                species: "beldum".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 55,
                    defense: 80,
                    special_attack: 35,
                    special_defense: 60,
                    speed: 30
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "none".to_string(),
                    hidden: "lightmetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Psychic
                ),
                weight: 95.2 as f32,
            }
        );
        
        pokedex.insert(
            "metang".to_string(),
            PokedexPokemon {
                species: "metang".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 75,
                    defense: 100,
                    special_attack: 55,
                    special_defense: 80,
                    speed: 50
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "none".to_string(),
                    hidden: "lightmetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Psychic
                ),
                weight: 202.5 as f32,
            }
        );
        
        pokedex.insert(
            "metagross".to_string(),
            PokedexPokemon {
                species: "metagross".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 135,
                    defense: 130,
                    special_attack: 95,
                    special_defense: 90,
                    speed: 70
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "none".to_string(),
                    hidden: "lightmetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Psychic
                ),
                weight: 550 as f32,
            }
        );
        
        pokedex.insert(
            "metagrossmega".to_string(),
            PokedexPokemon {
                species: "metagrossmega".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 145,
                    defense: 150,
                    special_attack: 105,
                    special_defense: 110,
                    speed: 110
                },
                abilities: Abilities {
                    first: "toughclaws".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Psychic
                ),
                weight: 942.9 as f32,
            }
        );
        
        pokedex.insert(
            "regirock".to_string(),
            PokedexPokemon {
                species: "regirock".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 100,
                    defense: 200,
                    special_attack: 50,
                    special_defense: 100,
                    speed: 50
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "none".to_string(),
                    hidden: "sturdy".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 230 as f32,
            }
        );
        
        pokedex.insert(
            "regice".to_string(),
            PokedexPokemon {
                species: "regice".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 50,
                    defense: 100,
                    special_attack: 100,
                    special_defense: 200,
                    speed: 50
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "none".to_string(),
                    hidden: "icebody".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 175 as f32,
            }
        );
        
        pokedex.insert(
            "registeel".to_string(),
            PokedexPokemon {
                species: "registeel".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 75,
                    defense: 150,
                    special_attack: 75,
                    special_defense: 150,
                    speed: 50
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "none".to_string(),
                    hidden: "lightmetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 205 as f32,
            }
        );
        
        pokedex.insert(
            "latias".to_string(),
            PokedexPokemon {
                species: "latias".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 80,
                    defense: 90,
                    special_attack: 110,
                    special_defense: 130,
                    speed: 110
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Psychic
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "latiasmega".to_string(),
            PokedexPokemon {
                species: "latiasmega".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 100,
                    defense: 120,
                    special_attack: 140,
                    special_defense: 150,
                    speed: 110
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Psychic
                ),
                weight: 52 as f32,
            }
        );
        
        pokedex.insert(
            "latios".to_string(),
            PokedexPokemon {
                species: "latios".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 90,
                    defense: 80,
                    special_attack: 130,
                    special_defense: 110,
                    speed: 110
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Psychic
                ),
                weight: 60 as f32,
            }
        );
        
        pokedex.insert(
            "latiosmega".to_string(),
            PokedexPokemon {
                species: "latiosmega".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 130,
                    defense: 100,
                    special_attack: 160,
                    special_defense: 120,
                    speed: 110
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Psychic
                ),
                weight: 70 as f32,
            }
        );
        
        pokedex.insert(
            "kyogre".to_string(),
            PokedexPokemon {
                species: "kyogre".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 90,
                    special_attack: 150,
                    special_defense: 140,
                    speed: 90
                },
                abilities: Abilities {
                    first: "drizzle".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 352 as f32,
            }
        );
        
        pokedex.insert(
            "kyogreprimal".to_string(),
            PokedexPokemon {
                species: "kyogreprimal".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 150,
                    defense: 90,
                    special_attack: 180,
                    special_defense: 160,
                    speed: 90
                },
                abilities: Abilities {
                    first: "primordialsea".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 430 as f32,
            }
        );
        
        pokedex.insert(
            "groudon".to_string(),
            PokedexPokemon {
                species: "groudon".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 150,
                    defense: 140,
                    special_attack: 100,
                    special_defense: 90,
                    speed: 90
                },
                abilities: Abilities {
                    first: "drought".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 950 as f32,
            }
        );
        
        pokedex.insert(
            "groudonprimal".to_string(),
            PokedexPokemon {
                species: "groudonprimal".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 180,
                    defense: 160,
                    special_attack: 150,
                    special_defense: 90,
                    speed: 90
                },
                abilities: Abilities {
                    first: "desolateland".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Fire
                ),
                weight: 999.7 as f32,
            }
        );
        
        pokedex.insert(
            "rayquaza".to_string(),
            PokedexPokemon {
                species: "rayquaza".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 150,
                    defense: 90,
                    special_attack: 150,
                    special_defense: 90,
                    speed: 95
                },
                abilities: Abilities {
                    first: "airlock".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Flying
                ),
                weight: 206.5 as f32,
            }
        );
        
        pokedex.insert(
            "rayquazamega".to_string(),
            PokedexPokemon {
                species: "rayquazamega".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 180,
                    defense: 100,
                    special_attack: 180,
                    special_defense: 100,
                    speed: 115
                },
                abilities: Abilities {
                    first: "deltastream".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Flying
                ),
                weight: 392 as f32,
            }
        );
        
        pokedex.insert(
            "jirachi".to_string(),
            PokedexPokemon {
                species: "jirachi".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 100,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 100
                },
                abilities: Abilities {
                    first: "serenegrace".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Psychic
                ),
                weight: 1.1 as f32,
            }
        );
        
        pokedex.insert(
            "deoxys".to_string(),
            PokedexPokemon {
                species: "deoxys".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 150,
                    defense: 50,
                    special_attack: 150,
                    special_defense: 50,
                    speed: 150
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 60.8 as f32,
            }
        );
        
        pokedex.insert(
            "deoxysattack".to_string(),
            PokedexPokemon {
                species: "deoxysattack".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 180,
                    defense: 20,
                    special_attack: 180,
                    special_defense: 20,
                    speed: 150
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 60.8 as f32,
            }
        );
        
        pokedex.insert(
            "deoxysdefense".to_string(),
            PokedexPokemon {
                species: "deoxysdefense".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 70,
                    defense: 160,
                    special_attack: 70,
                    special_defense: 160,
                    speed: 90
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 60.8 as f32,
            }
        );
        
        pokedex.insert(
            "deoxysspeed".to_string(),
            PokedexPokemon {
                species: "deoxysspeed".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 95,
                    defense: 90,
                    special_attack: 95,
                    special_defense: 90,
                    speed: 180
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 60.8 as f32,
            }
        );
        
        pokedex.insert(
            "turtwig".to_string(),
            PokedexPokemon {
                species: "turtwig".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 68,
                    defense: 64,
                    special_attack: 45,
                    special_defense: 55,
                    speed: 31
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "shellarmor".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 10.2 as f32,
            }
        );
        
        pokedex.insert(
            "grotle".to_string(),
            PokedexPokemon {
                species: "grotle".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 89,
                    defense: 85,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 36
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "shellarmor".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 97 as f32,
            }
        );
        
        pokedex.insert(
            "torterra".to_string(),
            PokedexPokemon {
                species: "torterra".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 109,
                    defense: 105,
                    special_attack: 75,
                    special_defense: 85,
                    speed: 56
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "shellarmor".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Ground
                ),
                weight: 310 as f32,
            }
        );
        
        pokedex.insert(
            "chimchar".to_string(),
            PokedexPokemon {
                species: "chimchar".to_string(),
                base_stats: BaseStats {
                    hp: 44,
                    attack: 58,
                    defense: 44,
                    special_attack: 58,
                    special_defense: 44,
                    speed: 61
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "ironfist".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 6.2 as f32,
            }
        );
        
        pokedex.insert(
            "monferno".to_string(),
            PokedexPokemon {
                species: "monferno".to_string(),
                base_stats: BaseStats {
                    hp: 64,
                    attack: 78,
                    defense: 52,
                    special_attack: 78,
                    special_defense: 52,
                    speed: 81
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "ironfist".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Fighting
                ),
                weight: 22 as f32,
            }
        );
        
        pokedex.insert(
            "infernape".to_string(),
            PokedexPokemon {
                species: "infernape".to_string(),
                base_stats: BaseStats {
                    hp: 76,
                    attack: 104,
                    defense: 71,
                    special_attack: 104,
                    special_defense: 71,
                    speed: 108
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "ironfist".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Fighting
                ),
                weight: 55 as f32,
            }
        );
        
        pokedex.insert(
            "piplup".to_string(),
            PokedexPokemon {
                species: "piplup".to_string(),
                base_stats: BaseStats {
                    hp: 53,
                    attack: 51,
                    defense: 53,
                    special_attack: 61,
                    special_defense: 56,
                    speed: 40
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 5.2 as f32,
            }
        );
        
        pokedex.insert(
            "prinplup".to_string(),
            PokedexPokemon {
                species: "prinplup".to_string(),
                base_stats: BaseStats {
                    hp: 64,
                    attack: 66,
                    defense: 68,
                    special_attack: 81,
                    special_defense: 76,
                    speed: 50
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 23 as f32,
            }
        );
        
        pokedex.insert(
            "empoleon".to_string(),
            PokedexPokemon {
                species: "empoleon".to_string(),
                base_stats: BaseStats {
                    hp: 84,
                    attack: 86,
                    defense: 88,
                    special_attack: 111,
                    special_defense: 101,
                    speed: 60
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Steel
                ),
                weight: 84.5 as f32,
            }
        );
        
        pokedex.insert(
            "starly".to_string(),
            PokedexPokemon {
                species: "starly".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 55,
                    defense: 30,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 60
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "none".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "staravia".to_string(),
            PokedexPokemon {
                species: "staravia".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 75,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 80
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "none".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 15.5 as f32,
            }
        );
        
        pokedex.insert(
            "staraptor".to_string(),
            PokedexPokemon {
                species: "staraptor".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 120,
                    defense: 70,
                    special_attack: 50,
                    special_defense: 60,
                    speed: 100
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "none".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 24.9 as f32,
            }
        );
        
        pokedex.insert(
            "bidoof".to_string(),
            PokedexPokemon {
                species: "bidoof".to_string(),
                base_stats: BaseStats {
                    hp: 59,
                    attack: 45,
                    defense: 40,
                    special_attack: 35,
                    special_defense: 40,
                    speed: 31
                },
                abilities: Abilities {
                    first: "simple".to_string(),
                    second: "unaware".to_string(),
                    hidden: "moody".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 20 as f32,
            }
        );
        
        pokedex.insert(
            "bibarel".to_string(),
            PokedexPokemon {
                species: "bibarel".to_string(),
                base_stats: BaseStats {
                    hp: 79,
                    attack: 85,
                    defense: 60,
                    special_attack: 55,
                    special_defense: 60,
                    speed: 71
                },
                abilities: Abilities {
                    first: "simple".to_string(),
                    second: "unaware".to_string(),
                    hidden: "moody".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Water
                ),
                weight: 31.5 as f32,
            }
        );
        
        pokedex.insert(
            "kricketot".to_string(),
            PokedexPokemon {
                species: "kricketot".to_string(),
                base_stats: BaseStats {
                    hp: 37,
                    attack: 25,
                    defense: 41,
                    special_attack: 25,
                    special_defense: 41,
                    speed: 25
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "runaway".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 2.2 as f32,
            }
        );
        
        pokedex.insert(
            "kricketune".to_string(),
            PokedexPokemon {
                species: "kricketune".to_string(),
                base_stats: BaseStats {
                    hp: 77,
                    attack: 85,
                    defense: 51,
                    special_attack: 55,
                    special_defense: 51,
                    speed: 65
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "none".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 25.5 as f32,
            }
        );
        
        pokedex.insert(
            "shinx".to_string(),
            PokedexPokemon {
                species: "shinx".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 65,
                    defense: 34,
                    special_attack: 40,
                    special_defense: 34,
                    speed: 45
                },
                abilities: Abilities {
                    first: "rivalry".to_string(),
                    second: "intimidate".to_string(),
                    hidden: "guts".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 9.5 as f32,
            }
        );
        
        pokedex.insert(
            "luxio".to_string(),
            PokedexPokemon {
                species: "luxio".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 85,
                    defense: 49,
                    special_attack: 60,
                    special_defense: 49,
                    speed: 60
                },
                abilities: Abilities {
                    first: "rivalry".to_string(),
                    second: "intimidate".to_string(),
                    hidden: "guts".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 30.5 as f32,
            }
        );
        
        pokedex.insert(
            "luxray".to_string(),
            PokedexPokemon {
                species: "luxray".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 120,
                    defense: 79,
                    special_attack: 95,
                    special_defense: 79,
                    speed: 70
                },
                abilities: Abilities {
                    first: "rivalry".to_string(),
                    second: "intimidate".to_string(),
                    hidden: "guts".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 42 as f32,
            }
        );
        
        pokedex.insert(
            "budew".to_string(),
            PokedexPokemon {
                species: "budew".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 30,
                    defense: 35,
                    special_attack: 50,
                    special_defense: 70,
                    speed: 55
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "poisonpoint".to_string(),
                    hidden: "leafguard".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 1.2 as f32,
            }
        );
        
        pokedex.insert(
            "roserade".to_string(),
            PokedexPokemon {
                species: "roserade".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 70,
                    defense: 65,
                    special_attack: 125,
                    special_defense: 105,
                    speed: 90
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "poisonpoint".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 14.5 as f32,
            }
        );
        
        pokedex.insert(
            "cranidos".to_string(),
            PokedexPokemon {
                species: "cranidos".to_string(),
                base_stats: BaseStats {
                    hp: 67,
                    attack: 125,
                    defense: 40,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 58
                },
                abilities: Abilities {
                    first: "moldbreaker".to_string(),
                    second: "none".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 31.5 as f32,
            }
        );
        
        pokedex.insert(
            "rampardos".to_string(),
            PokedexPokemon {
                species: "rampardos".to_string(),
                base_stats: BaseStats {
                    hp: 97,
                    attack: 165,
                    defense: 60,
                    special_attack: 65,
                    special_defense: 50,
                    speed: 58
                },
                abilities: Abilities {
                    first: "moldbreaker".to_string(),
                    second: "none".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 102.5 as f32,
            }
        );
        
        pokedex.insert(
            "shieldon".to_string(),
            PokedexPokemon {
                species: "shieldon".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 42,
                    defense: 118,
                    special_attack: 42,
                    special_defense: 88,
                    speed: 30
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "none".to_string(),
                    hidden: "soundproof".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Steel
                ),
                weight: 57 as f32,
            }
        );
        
        pokedex.insert(
            "bastiodon".to_string(),
            PokedexPokemon {
                species: "bastiodon".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 52,
                    defense: 168,
                    special_attack: 47,
                    special_defense: 138,
                    speed: 30
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "none".to_string(),
                    hidden: "soundproof".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Steel
                ),
                weight: 149.5 as f32,
            }
        );
        
        pokedex.insert(
            "burmy".to_string(),
            PokedexPokemon {
                species: "burmy".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 29,
                    defense: 45,
                    special_attack: 29,
                    special_defense: 45,
                    speed: 36
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 3.4 as f32,
            }
        );
        
        pokedex.insert(
            "wormadam".to_string(),
            PokedexPokemon {
                species: "wormadam".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 59,
                    defense: 85,
                    special_attack: 79,
                    special_defense: 105,
                    speed: 36
                },
                abilities: Abilities {
                    first: "anticipation".to_string(),
                    second: "none".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Grass
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "wormadamsandy".to_string(),
            PokedexPokemon {
                species: "wormadamsandy".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 79,
                    defense: 105,
                    special_attack: 59,
                    special_defense: 85,
                    speed: 36
                },
                abilities: Abilities {
                    first: "anticipation".to_string(),
                    second: "none".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Ground
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "wormadamtrash".to_string(),
            PokedexPokemon {
                species: "wormadamtrash".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 69,
                    defense: 95,
                    special_attack: 69,
                    special_defense: 95,
                    speed: 36
                },
                abilities: Abilities {
                    first: "anticipation".to_string(),
                    second: "none".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "mothim".to_string(),
            PokedexPokemon {
                species: "mothim".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 94,
                    defense: 50,
                    special_attack: 94,
                    special_defense: 50,
                    speed: 66
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "none".to_string(),
                    hidden: "tintedlens".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 23.3 as f32,
            }
        );
        
        pokedex.insert(
            "combee".to_string(),
            PokedexPokemon {
                species: "combee".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 30,
                    defense: 42,
                    special_attack: 30,
                    special_defense: 42,
                    speed: 70
                },
                abilities: Abilities {
                    first: "honeygather".to_string(),
                    second: "none".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 5.5 as f32,
            }
        );
        
        pokedex.insert(
            "vespiquen".to_string(),
            PokedexPokemon {
                species: "vespiquen".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 80,
                    defense: 102,
                    special_attack: 80,
                    special_defense: 102,
                    speed: 40
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 38.5 as f32,
            }
        );
        
        pokedex.insert(
            "pachirisu".to_string(),
            PokedexPokemon {
                species: "pachirisu".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 45,
                    defense: 70,
                    special_attack: 45,
                    special_defense: 90,
                    speed: 95
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "pickup".to_string(),
                    hidden: "voltabsorb".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 3.9 as f32,
            }
        );
        
        pokedex.insert(
            "buizel".to_string(),
            PokedexPokemon {
                species: "buizel".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 65,
                    defense: 35,
                    special_attack: 60,
                    special_defense: 30,
                    speed: 85
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "waterveil".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 29.5 as f32,
            }
        );
        
        pokedex.insert(
            "floatzel".to_string(),
            PokedexPokemon {
                species: "floatzel".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 105,
                    defense: 55,
                    special_attack: 85,
                    special_defense: 50,
                    speed: 115
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "waterveil".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 33.5 as f32,
            }
        );
        
        pokedex.insert(
            "cherubi".to_string(),
            PokedexPokemon {
                species: "cherubi".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 35,
                    defense: 45,
                    special_attack: 62,
                    special_defense: 53,
                    speed: 35
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 3.3 as f32,
            }
        );
        
        pokedex.insert(
            "cherrim".to_string(),
            PokedexPokemon {
                species: "cherrim".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 60,
                    defense: 70,
                    special_attack: 87,
                    special_defense: 78,
                    speed: 85
                },
                abilities: Abilities {
                    first: "flowergift".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 9.3 as f32,
            }
        );
        
        pokedex.insert(
            "cherrimsunshine".to_string(),
            PokedexPokemon {
                species: "cherrimsunshine".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 60,
                    defense: 70,
                    special_attack: 87,
                    special_defense: 78,
                    speed: 85
                },
                abilities: Abilities {
                    first: "flowergift".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 9.3 as f32,
            }
        );
        
        pokedex.insert(
            "shellos".to_string(),
            PokedexPokemon {
                species: "shellos".to_string(),
                base_stats: BaseStats {
                    hp: 76,
                    attack: 48,
                    defense: 48,
                    special_attack: 57,
                    special_defense: 62,
                    speed: 34
                },
                abilities: Abilities {
                    first: "stickyhold".to_string(),
                    second: "stormdrain".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 6.3 as f32,
            }
        );
        
        pokedex.insert(
            "gastrodon".to_string(),
            PokedexPokemon {
                species: "gastrodon".to_string(),
                base_stats: BaseStats {
                    hp: 111,
                    attack: 83,
                    defense: 68,
                    special_attack: 92,
                    special_defense: 82,
                    speed: 39
                },
                abilities: Abilities {
                    first: "stickyhold".to_string(),
                    second: "stormdrain".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 29.9 as f32,
            }
        );
        
        pokedex.insert(
            "ambipom".to_string(),
            PokedexPokemon {
                species: "ambipom".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 100,
                    defense: 66,
                    special_attack: 60,
                    special_defense: 66,
                    speed: 115
                },
                abilities: Abilities {
                    first: "technician".to_string(),
                    second: "pickup".to_string(),
                    hidden: "skilllink".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 20.3 as f32,
            }
        );
        
        pokedex.insert(
            "drifloon".to_string(),
            PokedexPokemon {
                species: "drifloon".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 50,
                    defense: 34,
                    special_attack: 60,
                    special_defense: 44,
                    speed: 70
                },
                abilities: Abilities {
                    first: "aftermath".to_string(),
                    second: "unburden".to_string(),
                    hidden: "flareboost".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Flying
                ),
                weight: 1.2 as f32,
            }
        );
        
        pokedex.insert(
            "drifblim".to_string(),
            PokedexPokemon {
                species: "drifblim".to_string(),
                base_stats: BaseStats {
                    hp: 150,
                    attack: 80,
                    defense: 44,
                    special_attack: 90,
                    special_defense: 54,
                    speed: 80
                },
                abilities: Abilities {
                    first: "aftermath".to_string(),
                    second: "unburden".to_string(),
                    hidden: "flareboost".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Flying
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "buneary".to_string(),
            PokedexPokemon {
                species: "buneary".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 66,
                    defense: 44,
                    special_attack: 44,
                    special_defense: 56,
                    speed: 85
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "klutz".to_string(),
                    hidden: "limber".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 5.5 as f32,
            }
        );
        
        pokedex.insert(
            "lopunny".to_string(),
            PokedexPokemon {
                species: "lopunny".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 76,
                    defense: 84,
                    special_attack: 54,
                    special_defense: 96,
                    speed: 105
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "klutz".to_string(),
                    hidden: "limber".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 33.3 as f32,
            }
        );
        
        pokedex.insert(
            "lopunnymega".to_string(),
            PokedexPokemon {
                species: "lopunnymega".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 136,
                    defense: 94,
                    special_attack: 54,
                    special_defense: 96,
                    speed: 135
                },
                abilities: Abilities {
                    first: "scrappy".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fighting
                ),
                weight: 28.3 as f32,
            }
        );
        
        pokedex.insert(
            "mismagius".to_string(),
            PokedexPokemon {
                species: "mismagius".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 60,
                    special_attack: 105,
                    special_defense: 105,
                    speed: 105
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 4.4 as f32,
            }
        );
        
        pokedex.insert(
            "honchkrow".to_string(),
            PokedexPokemon {
                species: "honchkrow".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 125,
                    defense: 52,
                    special_attack: 105,
                    special_defense: 52,
                    speed: 71
                },
                abilities: Abilities {
                    first: "insomnia".to_string(),
                    second: "superluck".to_string(),
                    hidden: "moxie".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Flying
                ),
                weight: 27.3 as f32,
            }
        );
        
        pokedex.insert(
            "glameow".to_string(),
            PokedexPokemon {
                species: "glameow".to_string(),
                base_stats: BaseStats {
                    hp: 49,
                    attack: 55,
                    defense: 42,
                    special_attack: 42,
                    special_defense: 37,
                    speed: 85
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "keeneye".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 3.9 as f32,
            }
        );
        
        pokedex.insert(
            "purugly".to_string(),
            PokedexPokemon {
                species: "purugly".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 82,
                    defense: 64,
                    special_attack: 64,
                    special_defense: 59,
                    speed: 112
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 43.8 as f32,
            }
        );
        
        pokedex.insert(
            "chingling".to_string(),
            PokedexPokemon {
                species: "chingling".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 30,
                    defense: 50,
                    special_attack: 65,
                    special_defense: 50,
                    speed: 45
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 0.6 as f32,
            }
        );
        
        pokedex.insert(
            "stunky".to_string(),
            PokedexPokemon {
                species: "stunky".to_string(),
                base_stats: BaseStats {
                    hp: 63,
                    attack: 63,
                    defense: 47,
                    special_attack: 41,
                    special_defense: 41,
                    speed: 74
                },
                abilities: Abilities {
                    first: "stench".to_string(),
                    second: "aftermath".to_string(),
                    hidden: "keeneye".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Dark
                ),
                weight: 19.2 as f32,
            }
        );
        
        pokedex.insert(
            "skuntank".to_string(),
            PokedexPokemon {
                species: "skuntank".to_string(),
                base_stats: BaseStats {
                    hp: 103,
                    attack: 93,
                    defense: 67,
                    special_attack: 71,
                    special_defense: 61,
                    speed: 84
                },
                abilities: Abilities {
                    first: "stench".to_string(),
                    second: "aftermath".to_string(),
                    hidden: "keeneye".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Dark
                ),
                weight: 38 as f32,
            }
        );
        
        pokedex.insert(
            "bronzor".to_string(),
            PokedexPokemon {
                species: "bronzor".to_string(),
                base_stats: BaseStats {
                    hp: 57,
                    attack: 24,
                    defense: 86,
                    special_attack: 24,
                    special_defense: 86,
                    speed: 23
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "heatproof".to_string(),
                    hidden: "heavymetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Psychic
                ),
                weight: 60.5 as f32,
            }
        );
        
        pokedex.insert(
            "bronzong".to_string(),
            PokedexPokemon {
                species: "bronzong".to_string(),
                base_stats: BaseStats {
                    hp: 67,
                    attack: 89,
                    defense: 116,
                    special_attack: 79,
                    special_defense: 116,
                    speed: 33
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "heatproof".to_string(),
                    hidden: "heavymetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Psychic
                ),
                weight: 187 as f32,
            }
        );
        
        pokedex.insert(
            "bonsly".to_string(),
            PokedexPokemon {
                species: "bonsly".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 80,
                    defense: 95,
                    special_attack: 10,
                    special_defense: 45,
                    speed: 10
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "rockhead".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "mimejr".to_string(),
            PokedexPokemon {
                species: "mimejr".to_string(),
                base_stats: BaseStats {
                    hp: 20,
                    attack: 25,
                    defense: 45,
                    special_attack: 70,
                    special_defense: 90,
                    speed: 60
                },
                abilities: Abilities {
                    first: "soundproof".to_string(),
                    second: "filter".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 13 as f32,
            }
        );
        
        pokedex.insert(
            "happiny".to_string(),
            PokedexPokemon {
                species: "happiny".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 5,
                    defense: 5,
                    special_attack: 15,
                    special_defense: 65,
                    speed: 30
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "serenegrace".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 24.4 as f32,
            }
        );
        
        pokedex.insert(
            "chatot".to_string(),
            PokedexPokemon {
                species: "chatot".to_string(),
                base_stats: BaseStats {
                    hp: 76,
                    attack: 65,
                    defense: 45,
                    special_attack: 92,
                    special_defense: 42,
                    speed: 91
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "tangledfeet".to_string(),
                    hidden: "bigpecks".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 1.9 as f32,
            }
        );
        
        pokedex.insert(
            "spiritomb".to_string(),
            PokedexPokemon {
                species: "spiritomb".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 92,
                    defense: 108,
                    special_attack: 92,
                    special_defense: 108,
                    speed: 35
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Dark
                ),
                weight: 108 as f32,
            }
        );
        
        pokedex.insert(
            "gible".to_string(),
            PokedexPokemon {
                species: "gible".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 70,
                    defense: 45,
                    special_attack: 40,
                    special_defense: 45,
                    speed: 42
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "none".to_string(),
                    hidden: "roughskin".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ground
                ),
                weight: 20.5 as f32,
            }
        );
        
        pokedex.insert(
            "gabite".to_string(),
            PokedexPokemon {
                species: "gabite".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 90,
                    defense: 65,
                    special_attack: 50,
                    special_defense: 55,
                    speed: 82
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "none".to_string(),
                    hidden: "roughskin".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ground
                ),
                weight: 56 as f32,
            }
        );
        
        pokedex.insert(
            "garchomp".to_string(),
            PokedexPokemon {
                species: "garchomp".to_string(),
                base_stats: BaseStats {
                    hp: 108,
                    attack: 130,
                    defense: 95,
                    special_attack: 80,
                    special_defense: 85,
                    speed: 102
                },
                abilities: Abilities {
                    first: "sandveil".to_string(),
                    second: "none".to_string(),
                    hidden: "roughskin".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ground
                ),
                weight: 95 as f32,
            }
        );
        
        pokedex.insert(
            "garchompmega".to_string(),
            PokedexPokemon {
                species: "garchompmega".to_string(),
                base_stats: BaseStats {
                    hp: 108,
                    attack: 170,
                    defense: 115,
                    special_attack: 120,
                    special_defense: 95,
                    speed: 92
                },
                abilities: Abilities {
                    first: "sandforce".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ground
                ),
                weight: 95 as f32,
            }
        );
        
        pokedex.insert(
            "munchlax".to_string(),
            PokedexPokemon {
                species: "munchlax".to_string(),
                base_stats: BaseStats {
                    hp: 135,
                    attack: 85,
                    defense: 40,
                    special_attack: 40,
                    special_defense: 85,
                    speed: 5
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "thickfat".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 105 as f32,
            }
        );
        
        pokedex.insert(
            "riolu".to_string(),
            PokedexPokemon {
                species: "riolu".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 70,
                    defense: 40,
                    special_attack: 35,
                    special_defense: 40,
                    speed: 60
                },
                abilities: Abilities {
                    first: "steadfast".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "prankster".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 20.2 as f32,
            }
        );
        
        pokedex.insert(
            "lucario".to_string(),
            PokedexPokemon {
                species: "lucario".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 110,
                    defense: 70,
                    special_attack: 115,
                    special_defense: 70,
                    speed: 90
                },
                abilities: Abilities {
                    first: "steadfast".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "justified".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Steel
                ),
                weight: 54 as f32,
            }
        );
        
        pokedex.insert(
            "lucariomega".to_string(),
            PokedexPokemon {
                species: "lucariomega".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 145,
                    defense: 88,
                    special_attack: 140,
                    special_defense: 70,
                    speed: 112
                },
                abilities: Abilities {
                    first: "adaptability".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Steel
                ),
                weight: 57.5 as f32,
            }
        );
        
        pokedex.insert(
            "hippopotas".to_string(),
            PokedexPokemon {
                species: "hippopotas".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 72,
                    defense: 78,
                    special_attack: 38,
                    special_defense: 42,
                    speed: 32
                },
                abilities: Abilities {
                    first: "sandstream".to_string(),
                    second: "none".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 49.5 as f32,
            }
        );
        
        pokedex.insert(
            "hippowdon".to_string(),
            PokedexPokemon {
                species: "hippowdon".to_string(),
                base_stats: BaseStats {
                    hp: 108,
                    attack: 112,
                    defense: 118,
                    special_attack: 68,
                    special_defense: 72,
                    speed: 47
                },
                abilities: Abilities {
                    first: "sandstream".to_string(),
                    second: "none".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 300 as f32,
            }
        );
        
        pokedex.insert(
            "skorupi".to_string(),
            PokedexPokemon {
                species: "skorupi".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 50,
                    defense: 90,
                    special_attack: 30,
                    special_defense: 55,
                    speed: 65
                },
                abilities: Abilities {
                    first: "battlearmor".to_string(),
                    second: "sniper".to_string(),
                    hidden: "keeneye".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Bug
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "drapion".to_string(),
            PokedexPokemon {
                species: "drapion".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 90,
                    defense: 110,
                    special_attack: 60,
                    special_defense: 75,
                    speed: 95
                },
                abilities: Abilities {
                    first: "battlearmor".to_string(),
                    second: "sniper".to_string(),
                    hidden: "keeneye".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Dark
                ),
                weight: 61.5 as f32,
            }
        );
        
        pokedex.insert(
            "croagunk".to_string(),
            PokedexPokemon {
                species: "croagunk".to_string(),
                base_stats: BaseStats {
                    hp: 48,
                    attack: 61,
                    defense: 40,
                    special_attack: 61,
                    special_defense: 40,
                    speed: 50
                },
                abilities: Abilities {
                    first: "anticipation".to_string(),
                    second: "dryskin".to_string(),
                    hidden: "poisontouch".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Fighting
                ),
                weight: 23 as f32,
            }
        );
        
        pokedex.insert(
            "toxicroak".to_string(),
            PokedexPokemon {
                species: "toxicroak".to_string(),
                base_stats: BaseStats {
                    hp: 83,
                    attack: 106,
                    defense: 65,
                    special_attack: 86,
                    special_defense: 65,
                    speed: 85
                },
                abilities: Abilities {
                    first: "anticipation".to_string(),
                    second: "dryskin".to_string(),
                    hidden: "poisontouch".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Fighting
                ),
                weight: 44.4 as f32,
            }
        );
        
        pokedex.insert(
            "carnivine".to_string(),
            PokedexPokemon {
                species: "carnivine".to_string(),
                base_stats: BaseStats {
                    hp: 74,
                    attack: 100,
                    defense: 72,
                    special_attack: 90,
                    special_defense: 72,
                    speed: 46
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 27 as f32,
            }
        );
        
        pokedex.insert(
            "finneon".to_string(),
            PokedexPokemon {
                species: "finneon".to_string(),
                base_stats: BaseStats {
                    hp: 49,
                    attack: 49,
                    defense: 56,
                    special_attack: 49,
                    special_defense: 61,
                    speed: 66
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "stormdrain".to_string(),
                    hidden: "waterveil".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 7 as f32,
            }
        );
        
        pokedex.insert(
            "lumineon".to_string(),
            PokedexPokemon {
                species: "lumineon".to_string(),
                base_stats: BaseStats {
                    hp: 69,
                    attack: 69,
                    defense: 76,
                    special_attack: 69,
                    special_defense: 86,
                    speed: 91
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "stormdrain".to_string(),
                    hidden: "waterveil".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 24 as f32,
            }
        );
        
        pokedex.insert(
            "mantyke".to_string(),
            PokedexPokemon {
                species: "mantyke".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 20,
                    defense: 50,
                    special_attack: 60,
                    special_defense: 120,
                    speed: 50
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "waterabsorb".to_string(),
                    hidden: "waterveil".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Flying
                ),
                weight: 65 as f32,
            }
        );
        
        pokedex.insert(
            "snover".to_string(),
            PokedexPokemon {
                species: "snover".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 62,
                    defense: 50,
                    special_attack: 62,
                    special_defense: 60,
                    speed: 40
                },
                abilities: Abilities {
                    first: "snowwarning".to_string(),
                    second: "none".to_string(),
                    hidden: "soundproof".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Ice
                ),
                weight: 50.5 as f32,
            }
        );
        
        pokedex.insert(
            "abomasnow".to_string(),
            PokedexPokemon {
                species: "abomasnow".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 92,
                    defense: 75,
                    special_attack: 92,
                    special_defense: 85,
                    speed: 60
                },
                abilities: Abilities {
                    first: "snowwarning".to_string(),
                    second: "none".to_string(),
                    hidden: "soundproof".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Ice
                ),
                weight: 135.5 as f32,
            }
        );
        
        pokedex.insert(
            "abomasnowmega".to_string(),
            PokedexPokemon {
                species: "abomasnowmega".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 132,
                    defense: 105,
                    special_attack: 132,
                    special_defense: 105,
                    speed: 30
                },
                abilities: Abilities {
                    first: "snowwarning".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Ice
                ),
                weight: 185 as f32,
            }
        );
        
        pokedex.insert(
            "weavile".to_string(),
            PokedexPokemon {
                species: "weavile".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 120,
                    defense: 65,
                    special_attack: 45,
                    special_defense: 85,
                    speed: 125
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Ice
                ),
                weight: 34 as f32,
            }
        );
        
        pokedex.insert(
            "magnezone".to_string(),
            PokedexPokemon {
                species: "magnezone".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 70,
                    defense: 115,
                    special_attack: 130,
                    special_defense: 90,
                    speed: 60
                },
                abilities: Abilities {
                    first: "magnetpull".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Steel
                ),
                weight: 180 as f32,
            }
        );
        
        pokedex.insert(
            "lickilicky".to_string(),
            PokedexPokemon {
                species: "lickilicky".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 85,
                    defense: 95,
                    special_attack: 80,
                    special_defense: 95,
                    speed: 50
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "oblivious".to_string(),
                    hidden: "cloudnine".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 140 as f32,
            }
        );
        
        pokedex.insert(
            "rhyperior".to_string(),
            PokedexPokemon {
                species: "rhyperior".to_string(),
                base_stats: BaseStats {
                    hp: 115,
                    attack: 140,
                    defense: 130,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 40
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "solidrock".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Rock
                ),
                weight: 282.8 as f32,
            }
        );
        
        pokedex.insert(
            "tangrowth".to_string(),
            PokedexPokemon {
                species: "tangrowth".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 125,
                    special_attack: 110,
                    special_defense: 50,
                    speed: 50
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "leafguard".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 128.6 as f32,
            }
        );
        
        pokedex.insert(
            "electivire".to_string(),
            PokedexPokemon {
                species: "electivire".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 123,
                    defense: 67,
                    special_attack: 95,
                    special_defense: 85,
                    speed: 95
                },
                abilities: Abilities {
                    first: "motordrive".to_string(),
                    second: "none".to_string(),
                    hidden: "vitalspirit".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 138.6 as f32,
            }
        );
        
        pokedex.insert(
            "magmortar".to_string(),
            PokedexPokemon {
                species: "magmortar".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 95,
                    defense: 67,
                    special_attack: 125,
                    special_defense: 95,
                    speed: 83
                },
                abilities: Abilities {
                    first: "flamebody".to_string(),
                    second: "none".to_string(),
                    hidden: "vitalspirit".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 68 as f32,
            }
        );
        
        pokedex.insert(
            "togekiss".to_string(),
            PokedexPokemon {
                species: "togekiss".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 50,
                    defense: 95,
                    special_attack: 120,
                    special_defense: 115,
                    speed: 80
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "serenegrace".to_string(),
                    hidden: "superluck".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Flying
                ),
                weight: 38 as f32,
            }
        );
        
        pokedex.insert(
            "yanmega".to_string(),
            PokedexPokemon {
                species: "yanmega".to_string(),
                base_stats: BaseStats {
                    hp: 86,
                    attack: 76,
                    defense: 86,
                    special_attack: 116,
                    special_defense: 56,
                    speed: 95
                },
                abilities: Abilities {
                    first: "speedboost".to_string(),
                    second: "tintedlens".to_string(),
                    hidden: "frisk".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 51.5 as f32,
            }
        );
        
        pokedex.insert(
            "leafeon".to_string(),
            PokedexPokemon {
                species: "leafeon".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 110,
                    defense: 130,
                    special_attack: 60,
                    special_defense: 65,
                    speed: 95
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "none".to_string(),
                    hidden: "chlorophyll".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 25.5 as f32,
            }
        );
        
        pokedex.insert(
            "glaceon".to_string(),
            PokedexPokemon {
                species: "glaceon".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 60,
                    defense: 110,
                    special_attack: 130,
                    special_defense: 95,
                    speed: 65
                },
                abilities: Abilities {
                    first: "snowcloak".to_string(),
                    second: "none".to_string(),
                    hidden: "icebody".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 25.9 as f32,
            }
        );
        
        pokedex.insert(
            "gliscor".to_string(),
            PokedexPokemon {
                species: "gliscor".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 95,
                    defense: 125,
                    special_attack: 45,
                    special_defense: 75,
                    speed: 95
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "sandveil".to_string(),
                    hidden: "poisonheal".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Flying
                ),
                weight: 42.5 as f32,
            }
        );
        
        pokedex.insert(
            "mamoswine".to_string(),
            PokedexPokemon {
                species: "mamoswine".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 130,
                    defense: 80,
                    special_attack: 70,
                    special_defense: 60,
                    speed: 80
                },
                abilities: Abilities {
                    first: "oblivious".to_string(),
                    second: "snowcloak".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Ground
                ),
                weight: 291 as f32,
            }
        );
        
        pokedex.insert(
            "porygonz".to_string(),
            PokedexPokemon {
                species: "porygonz".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 80,
                    defense: 70,
                    special_attack: 135,
                    special_defense: 75,
                    speed: 90
                },
                abilities: Abilities {
                    first: "adaptability".to_string(),
                    second: "download".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 34 as f32,
            }
        );
        
        pokedex.insert(
            "gallade".to_string(),
            PokedexPokemon {
                species: "gallade".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 125,
                    defense: 65,
                    special_attack: 65,
                    special_defense: 115,
                    speed: 80
                },
                abilities: Abilities {
                    first: "steadfast".to_string(),
                    second: "none".to_string(),
                    hidden: "justified".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fighting
                ),
                weight: 52 as f32,
            }
        );
        
        pokedex.insert(
            "gallademega".to_string(),
            PokedexPokemon {
                species: "gallademega".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 165,
                    defense: 95,
                    special_attack: 65,
                    special_defense: 115,
                    speed: 110
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fighting
                ),
                weight: 56.4 as f32,
            }
        );
        
        pokedex.insert(
            "probopass".to_string(),
            PokedexPokemon {
                species: "probopass".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 55,
                    defense: 145,
                    special_attack: 75,
                    special_defense: 150,
                    speed: 40
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "magnetpull".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Steel
                ),
                weight: 340 as f32,
            }
        );
        
        pokedex.insert(
            "dusknoir".to_string(),
            PokedexPokemon {
                species: "dusknoir".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 100,
                    defense: 135,
                    special_attack: 65,
                    special_defense: 135,
                    speed: 45
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "frisk".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 106.6 as f32,
            }
        );
        
        pokedex.insert(
            "froslass".to_string(),
            PokedexPokemon {
                species: "froslass".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 80,
                    defense: 70,
                    special_attack: 80,
                    special_defense: 70,
                    speed: 110
                },
                abilities: Abilities {
                    first: "snowcloak".to_string(),
                    second: "none".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Ghost
                ),
                weight: 26.6 as f32,
            }
        );
        
        pokedex.insert(
            "rotom".to_string(),
            PokedexPokemon {
                species: "rotom".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 50,
                    defense: 77,
                    special_attack: 95,
                    special_defense: 77,
                    speed: 91
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Ghost
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "rotomheat".to_string(),
            PokedexPokemon {
                species: "rotomheat".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 107,
                    special_attack: 105,
                    special_defense: 107,
                    speed: 86
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Fire
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "rotomwash".to_string(),
            PokedexPokemon {
                species: "rotomwash".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 107,
                    special_attack: 105,
                    special_defense: 107,
                    speed: 86
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Water
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "rotomfrost".to_string(),
            PokedexPokemon {
                species: "rotomfrost".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 107,
                    special_attack: 105,
                    special_defense: 107,
                    speed: 86
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Ice
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "rotomfan".to_string(),
            PokedexPokemon {
                species: "rotomfan".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 107,
                    special_attack: 105,
                    special_defense: 107,
                    speed: 86
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Flying
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "rotommow".to_string(),
            PokedexPokemon {
                species: "rotommow".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 107,
                    special_attack: 105,
                    special_defense: 107,
                    speed: 86
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Grass
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "uxie".to_string(),
            PokedexPokemon {
                species: "uxie".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 75,
                    defense: 130,
                    special_attack: 75,
                    special_defense: 130,
                    speed: 95
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "mesprit".to_string(),
            PokedexPokemon {
                species: "mesprit".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 105,
                    defense: 105,
                    special_attack: 105,
                    special_defense: 105,
                    speed: 80
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "azelf".to_string(),
            PokedexPokemon {
                species: "azelf".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 125,
                    defense: 70,
                    special_attack: 125,
                    special_defense: 70,
                    speed: 115
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "dialga".to_string(),
            PokedexPokemon {
                species: "dialga".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 120,
                    defense: 120,
                    special_attack: 150,
                    special_defense: 100,
                    speed: 90
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Dragon
                ),
                weight: 683 as f32,
            }
        );
        
        pokedex.insert(
            "palkia".to_string(),
            PokedexPokemon {
                species: "palkia".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 120,
                    defense: 100,
                    special_attack: 150,
                    special_defense: 120,
                    speed: 100
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dragon
                ),
                weight: 336 as f32,
            }
        );
        
        pokedex.insert(
            "heatran".to_string(),
            PokedexPokemon {
                species: "heatran".to_string(),
                base_stats: BaseStats {
                    hp: 91,
                    attack: 90,
                    defense: 106,
                    special_attack: 130,
                    special_defense: 106,
                    speed: 77
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "none".to_string(),
                    hidden: "flamebody".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Steel
                ),
                weight: 430 as f32,
            }
        );
        
        pokedex.insert(
            "regigigas".to_string(),
            PokedexPokemon {
                species: "regigigas".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 160,
                    defense: 110,
                    special_attack: 80,
                    special_defense: 110,
                    speed: 100
                },
                abilities: Abilities {
                    first: "slowstart".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 420 as f32,
            }
        );
        
        pokedex.insert(
            "giratina".to_string(),
            PokedexPokemon {
                species: "giratina".to_string(),
                base_stats: BaseStats {
                    hp: 150,
                    attack: 100,
                    defense: 120,
                    special_attack: 100,
                    special_defense: 120,
                    speed: 90
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Dragon
                ),
                weight: 750 as f32,
            }
        );
        
        pokedex.insert(
            "giratinaorigin".to_string(),
            PokedexPokemon {
                species: "giratinaorigin".to_string(),
                base_stats: BaseStats {
                    hp: 150,
                    attack: 120,
                    defense: 100,
                    special_attack: 120,
                    special_defense: 100,
                    speed: 90
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Dragon
                ),
                weight: 650 as f32,
            }
        );
        
        pokedex.insert(
            "cresselia".to_string(),
            PokedexPokemon {
                species: "cresselia".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 70,
                    defense: 120,
                    special_attack: 75,
                    special_defense: 130,
                    speed: 85
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 85.6 as f32,
            }
        );
        
        pokedex.insert(
            "phione".to_string(),
            PokedexPokemon {
                species: "phione".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 80,
                    defense: 80,
                    special_attack: 80,
                    special_defense: 80,
                    speed: 80
                },
                abilities: Abilities {
                    first: "hydration".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 3.1 as f32,
            }
        );
        
        pokedex.insert(
            "manaphy".to_string(),
            PokedexPokemon {
                species: "manaphy".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 100,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 100
                },
                abilities: Abilities {
                    first: "hydration".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 1.4 as f32,
            }
        );
        
        pokedex.insert(
            "darkrai".to_string(),
            PokedexPokemon {
                species: "darkrai".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 90,
                    defense: 90,
                    special_attack: 135,
                    special_defense: 90,
                    speed: 125
                },
                abilities: Abilities {
                    first: "baddreams".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 50.5 as f32,
            }
        );
        
        pokedex.insert(
            "shaymin".to_string(),
            PokedexPokemon {
                species: "shaymin".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 100,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 100
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 2.1 as f32,
            }
        );
        
        pokedex.insert(
            "shayminsky".to_string(),
            PokedexPokemon {
                species: "shayminsky".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 103,
                    defense: 75,
                    special_attack: 120,
                    special_defense: 75,
                    speed: 127
                },
                abilities: Abilities {
                    first: "serenegrace".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Flying
                ),
                weight: 5.2 as f32,
            }
        );
        
        pokedex.insert(
            "arceus".to_string(),
            PokedexPokemon {
                species: "arceus".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusbug".to_string(),
            PokedexPokemon {
                species: "arceusbug".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusdark".to_string(),
            PokedexPokemon {
                species: "arceusdark".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusdragon".to_string(),
            PokedexPokemon {
                species: "arceusdragon".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceuselectric".to_string(),
            PokedexPokemon {
                species: "arceuselectric".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusfairy".to_string(),
            PokedexPokemon {
                species: "arceusfairy".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusfighting".to_string(),
            PokedexPokemon {
                species: "arceusfighting".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusfire".to_string(),
            PokedexPokemon {
                species: "arceusfire".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusflying".to_string(),
            PokedexPokemon {
                species: "arceusflying".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusghost".to_string(),
            PokedexPokemon {
                species: "arceusghost".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusgrass".to_string(),
            PokedexPokemon {
                species: "arceusgrass".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusground".to_string(),
            PokedexPokemon {
                species: "arceusground".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusice".to_string(),
            PokedexPokemon {
                species: "arceusice".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceuspoison".to_string(),
            PokedexPokemon {
                species: "arceuspoison".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceuspsychic".to_string(),
            PokedexPokemon {
                species: "arceuspsychic".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceusrock".to_string(),
            PokedexPokemon {
                species: "arceusrock".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceussteel".to_string(),
            PokedexPokemon {
                species: "arceussteel".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "arceuswater".to_string(),
            PokedexPokemon {
                species: "arceuswater".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 120,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 120,
                    speed: 120
                },
                abilities: Abilities {
                    first: "multitype".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 320 as f32,
            }
        );
        
        pokedex.insert(
            "victini".to_string(),
            PokedexPokemon {
                species: "victini".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 100,
                    defense: 100,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 100
                },
                abilities: Abilities {
                    first: "victorystar".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fire
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "snivy".to_string(),
            PokedexPokemon {
                species: "snivy".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 45,
                    defense: 55,
                    special_attack: 45,
                    special_defense: 55,
                    speed: 63
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "contrary".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 8.1 as f32,
            }
        );
        
        pokedex.insert(
            "servine".to_string(),
            PokedexPokemon {
                species: "servine".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 75,
                    special_attack: 60,
                    special_defense: 75,
                    speed: 83
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "contrary".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 16 as f32,
            }
        );
        
        pokedex.insert(
            "serperior".to_string(),
            PokedexPokemon {
                species: "serperior".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 75,
                    defense: 95,
                    special_attack: 75,
                    special_defense: 95,
                    speed: 113
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "contrary".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 63 as f32,
            }
        );
        
        pokedex.insert(
            "tepig".to_string(),
            PokedexPokemon {
                species: "tepig".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 63,
                    defense: 45,
                    special_attack: 45,
                    special_defense: 45,
                    speed: 45
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 9.9 as f32,
            }
        );
        
        pokedex.insert(
            "pignite".to_string(),
            PokedexPokemon {
                species: "pignite".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 93,
                    defense: 55,
                    special_attack: 70,
                    special_defense: 55,
                    speed: 55
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Fighting
                ),
                weight: 55.5 as f32,
            }
        );
        
        pokedex.insert(
            "emboar".to_string(),
            PokedexPokemon {
                species: "emboar".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 123,
                    defense: 65,
                    special_attack: 100,
                    special_defense: 65,
                    speed: 65
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Fighting
                ),
                weight: 150 as f32,
            }
        );
        
        pokedex.insert(
            "oshawott".to_string(),
            PokedexPokemon {
                species: "oshawott".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 55,
                    defense: 45,
                    special_attack: 63,
                    special_defense: 45,
                    speed: 45
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "shellarmor".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 5.9 as f32,
            }
        );
        
        pokedex.insert(
            "dewott".to_string(),
            PokedexPokemon {
                species: "dewott".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 75,
                    defense: 60,
                    special_attack: 83,
                    special_defense: 60,
                    speed: 60
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "shellarmor".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 24.5 as f32,
            }
        );
        
        pokedex.insert(
            "samurott".to_string(),
            PokedexPokemon {
                species: "samurott".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 100,
                    defense: 85,
                    special_attack: 108,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "shellarmor".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 94.6 as f32,
            }
        );
        
        pokedex.insert(
            "patrat".to_string(),
            PokedexPokemon {
                species: "patrat".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 55,
                    defense: 39,
                    special_attack: 35,
                    special_defense: 39,
                    speed: 42
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "keeneye".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 11.6 as f32,
            }
        );
        
        pokedex.insert(
            "watchog".to_string(),
            PokedexPokemon {
                species: "watchog".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 85,
                    defense: 69,
                    special_attack: 60,
                    special_defense: 69,
                    speed: 77
                },
                abilities: Abilities {
                    first: "illuminate".to_string(),
                    second: "keeneye".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 27 as f32,
            }
        );
        
        pokedex.insert(
            "lillipup".to_string(),
            PokedexPokemon {
                species: "lillipup".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 60,
                    defense: 45,
                    special_attack: 25,
                    special_defense: 45,
                    speed: 55
                },
                abilities: Abilities {
                    first: "vitalspirit".to_string(),
                    second: "pickup".to_string(),
                    hidden: "runaway".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 4.1 as f32,
            }
        );
        
        pokedex.insert(
            "herdier".to_string(),
            PokedexPokemon {
                species: "herdier".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 80,
                    defense: 65,
                    special_attack: 35,
                    special_defense: 65,
                    speed: 60
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "sandrush".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 14.7 as f32,
            }
        );
        
        pokedex.insert(
            "stoutland".to_string(),
            PokedexPokemon {
                species: "stoutland".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 110,
                    defense: 90,
                    special_attack: 45,
                    special_defense: 90,
                    speed: 80
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "sandrush".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 61 as f32,
            }
        );
        
        pokedex.insert(
            "purrloin".to_string(),
            PokedexPokemon {
                species: "purrloin".to_string(),
                base_stats: BaseStats {
                    hp: 41,
                    attack: 50,
                    defense: 37,
                    special_attack: 50,
                    special_defense: 37,
                    speed: 66
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "unburden".to_string(),
                    hidden: "prankster".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 10.1 as f32,
            }
        );
        
        pokedex.insert(
            "liepard".to_string(),
            PokedexPokemon {
                species: "liepard".to_string(),
                base_stats: BaseStats {
                    hp: 64,
                    attack: 88,
                    defense: 50,
                    special_attack: 88,
                    special_defense: 50,
                    speed: 106
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "unburden".to_string(),
                    hidden: "prankster".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 37.5 as f32,
            }
        );
        
        pokedex.insert(
            "pansage".to_string(),
            PokedexPokemon {
                species: "pansage".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 53,
                    defense: 48,
                    special_attack: 53,
                    special_defense: 48,
                    speed: 64
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "none".to_string(),
                    hidden: "overgrow".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 10.5 as f32,
            }
        );
        
        pokedex.insert(
            "simisage".to_string(),
            PokedexPokemon {
                species: "simisage".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 98,
                    defense: 63,
                    special_attack: 98,
                    special_defense: 63,
                    speed: 101
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "none".to_string(),
                    hidden: "overgrow".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 30.5 as f32,
            }
        );
        
        pokedex.insert(
            "pansear".to_string(),
            PokedexPokemon {
                species: "pansear".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 53,
                    defense: 48,
                    special_attack: 53,
                    special_defense: 48,
                    speed: 64
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "none".to_string(),
                    hidden: "blaze".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 11 as f32,
            }
        );
        
        pokedex.insert(
            "simisear".to_string(),
            PokedexPokemon {
                species: "simisear".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 98,
                    defense: 63,
                    special_attack: 98,
                    special_defense: 63,
                    speed: 101
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "none".to_string(),
                    hidden: "blaze".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "panpour".to_string(),
            PokedexPokemon {
                species: "panpour".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 53,
                    defense: 48,
                    special_attack: 53,
                    special_defense: 48,
                    speed: 64
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "none".to_string(),
                    hidden: "torrent".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 13.5 as f32,
            }
        );
        
        pokedex.insert(
            "simipour".to_string(),
            PokedexPokemon {
                species: "simipour".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 98,
                    defense: 63,
                    special_attack: 98,
                    special_defense: 63,
                    speed: 101
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "none".to_string(),
                    hidden: "torrent".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 29 as f32,
            }
        );
        
        pokedex.insert(
            "munna".to_string(),
            PokedexPokemon {
                species: "munna".to_string(),
                base_stats: BaseStats {
                    hp: 76,
                    attack: 25,
                    defense: 45,
                    special_attack: 67,
                    special_defense: 55,
                    speed: 24
                },
                abilities: Abilities {
                    first: "forewarn".to_string(),
                    second: "synchronize".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 23.3 as f32,
            }
        );
        
        pokedex.insert(
            "musharna".to_string(),
            PokedexPokemon {
                species: "musharna".to_string(),
                base_stats: BaseStats {
                    hp: 116,
                    attack: 55,
                    defense: 85,
                    special_attack: 107,
                    special_defense: 95,
                    speed: 29
                },
                abilities: Abilities {
                    first: "forewarn".to_string(),
                    second: "synchronize".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 60.5 as f32,
            }
        );
        
        pokedex.insert(
            "pidove".to_string(),
            PokedexPokemon {
                species: "pidove".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 55,
                    defense: 50,
                    special_attack: 36,
                    special_defense: 30,
                    speed: 43
                },
                abilities: Abilities {
                    first: "bigpecks".to_string(),
                    second: "superluck".to_string(),
                    hidden: "rivalry".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 2.1 as f32,
            }
        );
        
        pokedex.insert(
            "tranquill".to_string(),
            PokedexPokemon {
                species: "tranquill".to_string(),
                base_stats: BaseStats {
                    hp: 62,
                    attack: 77,
                    defense: 62,
                    special_attack: 50,
                    special_defense: 42,
                    speed: 65
                },
                abilities: Abilities {
                    first: "bigpecks".to_string(),
                    second: "superluck".to_string(),
                    hidden: "rivalry".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "unfezant".to_string(),
            PokedexPokemon {
                species: "unfezant".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 115,
                    defense: 80,
                    special_attack: 65,
                    special_defense: 55,
                    speed: 93
                },
                abilities: Abilities {
                    first: "bigpecks".to_string(),
                    second: "superluck".to_string(),
                    hidden: "rivalry".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 29 as f32,
            }
        );
        
        pokedex.insert(
            "blitzle".to_string(),
            PokedexPokemon {
                species: "blitzle".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 60,
                    defense: 32,
                    special_attack: 50,
                    special_defense: 32,
                    speed: 76
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "motordrive".to_string(),
                    hidden: "sapsipper".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 29.8 as f32,
            }
        );
        
        pokedex.insert(
            "zebstrika".to_string(),
            PokedexPokemon {
                species: "zebstrika".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 100,
                    defense: 63,
                    special_attack: 80,
                    special_defense: 63,
                    speed: 116
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "motordrive".to_string(),
                    hidden: "sapsipper".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 79.5 as f32,
            }
        );
        
        pokedex.insert(
            "roggenrola".to_string(),
            PokedexPokemon {
                species: "roggenrola".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 75,
                    defense: 85,
                    special_attack: 25,
                    special_defense: 25,
                    speed: 15
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "weakarmor".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "boldore".to_string(),
            PokedexPokemon {
                species: "boldore".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 105,
                    defense: 105,
                    special_attack: 50,
                    special_defense: 40,
                    speed: 20
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "weakarmor".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 102 as f32,
            }
        );
        
        pokedex.insert(
            "gigalith".to_string(),
            PokedexPokemon {
                species: "gigalith".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 135,
                    defense: 130,
                    special_attack: 60,
                    special_defense: 80,
                    speed: 25
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "sandstream".to_string(),
                    hidden: "sandforce".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 260 as f32,
            }
        );
        
        pokedex.insert(
            "woobat".to_string(),
            PokedexPokemon {
                species: "woobat".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 45,
                    defense: 43,
                    special_attack: 55,
                    special_defense: 43,
                    speed: 72
                },
                abilities: Abilities {
                    first: "unaware".to_string(),
                    second: "klutz".to_string(),
                    hidden: "simple".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Flying
                ),
                weight: 2.1 as f32,
            }
        );
        
        pokedex.insert(
            "swoobat".to_string(),
            PokedexPokemon {
                species: "swoobat".to_string(),
                base_stats: BaseStats {
                    hp: 67,
                    attack: 57,
                    defense: 55,
                    special_attack: 77,
                    special_defense: 55,
                    speed: 114
                },
                abilities: Abilities {
                    first: "unaware".to_string(),
                    second: "klutz".to_string(),
                    hidden: "simple".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Flying
                ),
                weight: 10.5 as f32,
            }
        );
        
        pokedex.insert(
            "drilbur".to_string(),
            PokedexPokemon {
                species: "drilbur".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 85,
                    defense: 40,
                    special_attack: 30,
                    special_defense: 45,
                    speed: 68
                },
                abilities: Abilities {
                    first: "sandrush".to_string(),
                    second: "sandforce".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "excadrill".to_string(),
            PokedexPokemon {
                species: "excadrill".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 135,
                    defense: 60,
                    special_attack: 50,
                    special_defense: 65,
                    speed: 88
                },
                abilities: Abilities {
                    first: "sandrush".to_string(),
                    second: "sandforce".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Steel
                ),
                weight: 40.4 as f32,
            }
        );
        
        pokedex.insert(
            "audino".to_string(),
            PokedexPokemon {
                species: "audino".to_string(),
                base_stats: BaseStats {
                    hp: 103,
                    attack: 60,
                    defense: 86,
                    special_attack: 60,
                    special_defense: 86,
                    speed: 50
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "regenerator".to_string(),
                    hidden: "klutz".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 31 as f32,
            }
        );
        
        pokedex.insert(
            "audinomega".to_string(),
            PokedexPokemon {
                species: "audinomega".to_string(),
                base_stats: BaseStats {
                    hp: 103,
                    attack: 60,
                    defense: 126,
                    special_attack: 80,
                    special_defense: 126,
                    speed: 50
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fairy
                ),
                weight: 32 as f32,
            }
        );
        
        pokedex.insert(
            "timburr".to_string(),
            PokedexPokemon {
                species: "timburr".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 80,
                    defense: 55,
                    special_attack: 25,
                    special_defense: 35,
                    speed: 35
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "sheerforce".to_string(),
                    hidden: "ironfist".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 12.5 as f32,
            }
        );
        
        pokedex.insert(
            "gurdurr".to_string(),
            PokedexPokemon {
                species: "gurdurr".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 105,
                    defense: 85,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 40
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "sheerforce".to_string(),
                    hidden: "ironfist".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "conkeldurr".to_string(),
            PokedexPokemon {
                species: "conkeldurr".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 140,
                    defense: 95,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 45
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "sheerforce".to_string(),
                    hidden: "ironfist".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 87 as f32,
            }
        );
        
        pokedex.insert(
            "tympole".to_string(),
            PokedexPokemon {
                species: "tympole".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 50,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 40,
                    speed: 64
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "hydration".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 4.5 as f32,
            }
        );
        
        pokedex.insert(
            "palpitoad".to_string(),
            PokedexPokemon {
                species: "palpitoad".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 65,
                    defense: 55,
                    special_attack: 65,
                    special_defense: 55,
                    speed: 69
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "hydration".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 17 as f32,
            }
        );
        
        pokedex.insert(
            "seismitoad".to_string(),
            PokedexPokemon {
                species: "seismitoad".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 95,
                    defense: 75,
                    special_attack: 85,
                    special_defense: 75,
                    speed: 74
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "poisontouch".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ground
                ),
                weight: 62 as f32,
            }
        );
        
        pokedex.insert(
            "throh".to_string(),
            PokedexPokemon {
                species: "throh".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 100,
                    defense: 85,
                    special_attack: 30,
                    special_defense: 85,
                    speed: 45
                },
                abilities: Abilities {
                    first: "guts".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 55.5 as f32,
            }
        );
        
        pokedex.insert(
            "sawk".to_string(),
            PokedexPokemon {
                species: "sawk".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 125,
                    defense: 75,
                    special_attack: 30,
                    special_defense: 75,
                    speed: 85
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 51 as f32,
            }
        );
        
        pokedex.insert(
            "sewaddle".to_string(),
            PokedexPokemon {
                species: "sewaddle".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 53,
                    defense: 70,
                    special_attack: 40,
                    special_defense: 60,
                    speed: 42
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "chlorophyll".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Grass
                ),
                weight: 2.5 as f32,
            }
        );
        
        pokedex.insert(
            "swadloon".to_string(),
            PokedexPokemon {
                species: "swadloon".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 63,
                    defense: 90,
                    special_attack: 50,
                    special_defense: 80,
                    speed: 42
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "chlorophyll".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Grass
                ),
                weight: 7.3 as f32,
            }
        );
        
        pokedex.insert(
            "leavanny".to_string(),
            PokedexPokemon {
                species: "leavanny".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 103,
                    defense: 80,
                    special_attack: 70,
                    special_defense: 80,
                    speed: 92
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "chlorophyll".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Grass
                ),
                weight: 20.5 as f32,
            }
        );
        
        pokedex.insert(
            "venipede".to_string(),
            PokedexPokemon {
                species: "venipede".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 45,
                    defense: 59,
                    special_attack: 30,
                    special_defense: 39,
                    speed: 57
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "swarm".to_string(),
                    hidden: "speedboost".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 5.3 as f32,
            }
        );
        
        pokedex.insert(
            "whirlipede".to_string(),
            PokedexPokemon {
                species: "whirlipede".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 55,
                    defense: 99,
                    special_attack: 40,
                    special_defense: 79,
                    speed: 47
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "swarm".to_string(),
                    hidden: "speedboost".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 58.5 as f32,
            }
        );
        
        pokedex.insert(
            "scolipede".to_string(),
            PokedexPokemon {
                species: "scolipede".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 100,
                    defense: 89,
                    special_attack: 55,
                    special_defense: 69,
                    speed: 112
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "swarm".to_string(),
                    hidden: "speedboost".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Poison
                ),
                weight: 200.5 as f32,
            }
        );
        
        pokedex.insert(
            "cottonee".to_string(),
            PokedexPokemon {
                species: "cottonee".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 27,
                    defense: 60,
                    special_attack: 37,
                    special_defense: 50,
                    speed: 66
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "chlorophyll".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Fairy
                ),
                weight: 0.6 as f32,
            }
        );
        
        pokedex.insert(
            "whimsicott".to_string(),
            PokedexPokemon {
                species: "whimsicott".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 67,
                    defense: 85,
                    special_attack: 77,
                    special_defense: 75,
                    speed: 116
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "chlorophyll".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Fairy
                ),
                weight: 6.6 as f32,
            }
        );
        
        pokedex.insert(
            "petilil".to_string(),
            PokedexPokemon {
                species: "petilil".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 35,
                    defense: 50,
                    special_attack: 70,
                    special_defense: 50,
                    speed: 30
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "leafguard".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 6.6 as f32,
            }
        );
        
        pokedex.insert(
            "lilligant".to_string(),
            PokedexPokemon {
                species: "lilligant".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 60,
                    defense: 75,
                    special_attack: 110,
                    special_defense: 75,
                    speed: 90
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "leafguard".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 16.3 as f32,
            }
        );
        
        pokedex.insert(
            "basculin".to_string(),
            PokedexPokemon {
                species: "basculin".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 92,
                    defense: 65,
                    special_attack: 80,
                    special_defense: 55,
                    speed: 98
                },
                abilities: Abilities {
                    first: "reckless".to_string(),
                    second: "adaptability".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "basculinbluestriped".to_string(),
            PokedexPokemon {
                species: "basculinbluestriped".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 92,
                    defense: 65,
                    special_attack: 80,
                    special_defense: 55,
                    speed: 98
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "adaptability".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "sandile".to_string(),
            PokedexPokemon {
                species: "sandile".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 72,
                    defense: 35,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 65
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "moxie".to_string(),
                    hidden: "angerpoint".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Dark
                ),
                weight: 15.2 as f32,
            }
        );
        
        pokedex.insert(
            "krokorok".to_string(),
            PokedexPokemon {
                species: "krokorok".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 82,
                    defense: 45,
                    special_attack: 45,
                    special_defense: 45,
                    speed: 74
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "moxie".to_string(),
                    hidden: "angerpoint".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Dark
                ),
                weight: 33.4 as f32,
            }
        );
        
        pokedex.insert(
            "krookodile".to_string(),
            PokedexPokemon {
                species: "krookodile".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 117,
                    defense: 80,
                    special_attack: 65,
                    special_defense: 70,
                    speed: 92
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "moxie".to_string(),
                    hidden: "angerpoint".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Dark
                ),
                weight: 96.3 as f32,
            }
        );
        
        pokedex.insert(
            "darumaka".to_string(),
            PokedexPokemon {
                species: "darumaka".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 90,
                    defense: 45,
                    special_attack: 15,
                    special_defense: 45,
                    speed: 50
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "none".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 37.5 as f32,
            }
        );
        
        pokedex.insert(
            "darmanitan".to_string(),
            PokedexPokemon {
                species: "darmanitan".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 140,
                    defense: 55,
                    special_attack: 30,
                    special_defense: 55,
                    speed: 95
                },
                abilities: Abilities {
                    first: "sheerforce".to_string(),
                    second: "none".to_string(),
                    hidden: "zenmode".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 92.9 as f32,
            }
        );
        
        pokedex.insert(
            "darmanitanzen".to_string(),
            PokedexPokemon {
                species: "darmanitanzen".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 30,
                    defense: 105,
                    special_attack: 140,
                    special_defense: 105,
                    speed: 55
                },
                abilities: Abilities {
                    first: "zenmode".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Psychic
                ),
                weight: 92.9 as f32,
            }
        );
        
        pokedex.insert(
            "maractus".to_string(),
            PokedexPokemon {
                species: "maractus".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 86,
                    defense: 67,
                    special_attack: 106,
                    special_defense: 67,
                    speed: 60
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "chlorophyll".to_string(),
                    hidden: "stormdrain".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "dwebble".to_string(),
            PokedexPokemon {
                species: "dwebble".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 85,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 55
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Rock
                ),
                weight: 14.5 as f32,
            }
        );
        
        pokedex.insert(
            "crustle".to_string(),
            PokedexPokemon {
                species: "crustle".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 105,
                    defense: 125,
                    special_attack: 65,
                    special_defense: 75,
                    speed: 45
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Rock
                ),
                weight: 200 as f32,
            }
        );
        
        pokedex.insert(
            "scraggy".to_string(),
            PokedexPokemon {
                species: "scraggy".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 75,
                    defense: 70,
                    special_attack: 35,
                    special_defense: 70,
                    speed: 48
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "moxie".to_string(),
                    hidden: "intimidate".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fighting
                ),
                weight: 11.8 as f32,
            }
        );
        
        pokedex.insert(
            "scrafty".to_string(),
            PokedexPokemon {
                species: "scrafty".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 90,
                    defense: 115,
                    special_attack: 45,
                    special_defense: 115,
                    speed: 58
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "moxie".to_string(),
                    hidden: "intimidate".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fighting
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "sigilyph".to_string(),
            PokedexPokemon {
                species: "sigilyph".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 58,
                    defense: 80,
                    special_attack: 103,
                    special_defense: 80,
                    speed: 97
                },
                abilities: Abilities {
                    first: "wonderskin".to_string(),
                    second: "magicguard".to_string(),
                    hidden: "tintedlens".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Flying
                ),
                weight: 14 as f32,
            }
        );
        
        pokedex.insert(
            "yamask".to_string(),
            PokedexPokemon {
                species: "yamask".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 30,
                    defense: 85,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 30
                },
                abilities: Abilities {
                    first: "mummy".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 1.5 as f32,
            }
        );
        
        pokedex.insert(
            "cofagrigus".to_string(),
            PokedexPokemon {
                species: "cofagrigus".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 50,
                    defense: 145,
                    special_attack: 95,
                    special_defense: 105,
                    speed: 30
                },
                abilities: Abilities {
                    first: "mummy".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 76.5 as f32,
            }
        );
        
        pokedex.insert(
            "tirtouga".to_string(),
            PokedexPokemon {
                species: "tirtouga".to_string(),
                base_stats: BaseStats {
                    hp: 54,
                    attack: 78,
                    defense: 103,
                    special_attack: 53,
                    special_defense: 45,
                    speed: 22
                },
                abilities: Abilities {
                    first: "solidrock".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Rock
                ),
                weight: 16.5 as f32,
            }
        );
        
        pokedex.insert(
            "carracosta".to_string(),
            PokedexPokemon {
                species: "carracosta".to_string(),
                base_stats: BaseStats {
                    hp: 74,
                    attack: 108,
                    defense: 133,
                    special_attack: 83,
                    special_defense: 65,
                    speed: 32
                },
                abilities: Abilities {
                    first: "solidrock".to_string(),
                    second: "sturdy".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Rock
                ),
                weight: 81 as f32,
            }
        );
        
        pokedex.insert(
            "archen".to_string(),
            PokedexPokemon {
                species: "archen".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 112,
                    defense: 45,
                    special_attack: 74,
                    special_defense: 45,
                    speed: 70
                },
                abilities: Abilities {
                    first: "defeatist".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Flying
                ),
                weight: 9.5 as f32,
            }
        );
        
        pokedex.insert(
            "archeops".to_string(),
            PokedexPokemon {
                species: "archeops".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 140,
                    defense: 65,
                    special_attack: 112,
                    special_defense: 65,
                    speed: 110
                },
                abilities: Abilities {
                    first: "defeatist".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Flying
                ),
                weight: 32 as f32,
            }
        );
        
        pokedex.insert(
            "trubbish".to_string(),
            PokedexPokemon {
                species: "trubbish".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 50,
                    defense: 62,
                    special_attack: 40,
                    special_defense: 62,
                    speed: 65
                },
                abilities: Abilities {
                    first: "stench".to_string(),
                    second: "stickyhold".to_string(),
                    hidden: "aftermath".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 31 as f32,
            }
        );
        
        pokedex.insert(
            "garbodor".to_string(),
            PokedexPokemon {
                species: "garbodor".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 95,
                    defense: 82,
                    special_attack: 60,
                    special_defense: 82,
                    speed: 75
                },
                abilities: Abilities {
                    first: "stench".to_string(),
                    second: "weakarmor".to_string(),
                    hidden: "aftermath".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 107.3 as f32,
            }
        );
        
        pokedex.insert(
            "zorua".to_string(),
            PokedexPokemon {
                species: "zorua".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 65,
                    defense: 40,
                    special_attack: 80,
                    special_defense: 40,
                    speed: 65
                },
                abilities: Abilities {
                    first: "illusion".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 12.5 as f32,
            }
        );
        
        pokedex.insert(
            "zoroark".to_string(),
            PokedexPokemon {
                species: "zoroark".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 105,
                    defense: 60,
                    special_attack: 120,
                    special_defense: 60,
                    speed: 105
                },
                abilities: Abilities {
                    first: "illusion".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 81.1 as f32,
            }
        );
        
        pokedex.insert(
            "minccino".to_string(),
            PokedexPokemon {
                species: "minccino".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 50,
                    defense: 40,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 75
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "technician".to_string(),
                    hidden: "skilllink".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 5.8 as f32,
            }
        );
        
        pokedex.insert(
            "cinccino".to_string(),
            PokedexPokemon {
                species: "cinccino".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 95,
                    defense: 60,
                    special_attack: 65,
                    special_defense: 60,
                    speed: 115
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "technician".to_string(),
                    hidden: "skilllink".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 7.5 as f32,
            }
        );
        
        pokedex.insert(
            "gothita".to_string(),
            PokedexPokemon {
                species: "gothita".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 30,
                    defense: 50,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 45
                },
                abilities: Abilities {
                    first: "frisk".to_string(),
                    second: "competitive".to_string(),
                    hidden: "shadowtag".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 5.8 as f32,
            }
        );
        
        pokedex.insert(
            "gothorita".to_string(),
            PokedexPokemon {
                species: "gothorita".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 45,
                    defense: 70,
                    special_attack: 75,
                    special_defense: 85,
                    speed: 55
                },
                abilities: Abilities {
                    first: "frisk".to_string(),
                    second: "competitive".to_string(),
                    hidden: "shadowtag".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "gothitelle".to_string(),
            PokedexPokemon {
                species: "gothitelle".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 55,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 110,
                    speed: 65
                },
                abilities: Abilities {
                    first: "frisk".to_string(),
                    second: "competitive".to_string(),
                    hidden: "shadowtag".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 44 as f32,
            }
        );
        
        pokedex.insert(
            "solosis".to_string(),
            PokedexPokemon {
                species: "solosis".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 30,
                    defense: 40,
                    special_attack: 105,
                    special_defense: 50,
                    speed: 20
                },
                abilities: Abilities {
                    first: "overcoat".to_string(),
                    second: "magicguard".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "duosion".to_string(),
            PokedexPokemon {
                species: "duosion".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 40,
                    defense: 50,
                    special_attack: 125,
                    special_defense: 60,
                    speed: 30
                },
                abilities: Abilities {
                    first: "overcoat".to_string(),
                    second: "magicguard".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 8 as f32,
            }
        );
        
        pokedex.insert(
            "reuniclus".to_string(),
            PokedexPokemon {
                species: "reuniclus".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 65,
                    defense: 75,
                    special_attack: 125,
                    special_defense: 85,
                    speed: 30
                },
                abilities: Abilities {
                    first: "overcoat".to_string(),
                    second: "magicguard".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 20.1 as f32,
            }
        );
        
        pokedex.insert(
            "ducklett".to_string(),
            PokedexPokemon {
                species: "ducklett".to_string(),
                base_stats: BaseStats {
                    hp: 62,
                    attack: 44,
                    defense: 50,
                    special_attack: 44,
                    special_defense: 50,
                    speed: 55
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "bigpecks".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Flying
                ),
                weight: 5.5 as f32,
            }
        );
        
        pokedex.insert(
            "swanna".to_string(),
            PokedexPokemon {
                species: "swanna".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 87,
                    defense: 63,
                    special_attack: 87,
                    special_defense: 63,
                    speed: 98
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "bigpecks".to_string(),
                    hidden: "hydration".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Flying
                ),
                weight: 24.2 as f32,
            }
        );
        
        pokedex.insert(
            "vanillite".to_string(),
            PokedexPokemon {
                species: "vanillite".to_string(),
                base_stats: BaseStats {
                    hp: 36,
                    attack: 50,
                    defense: 50,
                    special_attack: 65,
                    special_defense: 60,
                    speed: 44
                },
                abilities: Abilities {
                    first: "icebody".to_string(),
                    second: "snowcloak".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 5.7 as f32,
            }
        );
        
        pokedex.insert(
            "vanillish".to_string(),
            PokedexPokemon {
                species: "vanillish".to_string(),
                base_stats: BaseStats {
                    hp: 51,
                    attack: 65,
                    defense: 65,
                    special_attack: 80,
                    special_defense: 75,
                    speed: 59
                },
                abilities: Abilities {
                    first: "icebody".to_string(),
                    second: "snowcloak".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 41 as f32,
            }
        );
        
        pokedex.insert(
            "vanilluxe".to_string(),
            PokedexPokemon {
                species: "vanilluxe".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 95,
                    defense: 85,
                    special_attack: 110,
                    special_defense: 95,
                    speed: 79
                },
                abilities: Abilities {
                    first: "icebody".to_string(),
                    second: "snowwarning".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 57.5 as f32,
            }
        );
        
        pokedex.insert(
            "deerling".to_string(),
            PokedexPokemon {
                species: "deerling".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 75
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "sapsipper".to_string(),
                    hidden: "serenegrace".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Grass
                ),
                weight: 19.5 as f32,
            }
        );
        
        pokedex.insert(
            "sawsbuck".to_string(),
            PokedexPokemon {
                species: "sawsbuck".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 100,
                    defense: 70,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 95
                },
                abilities: Abilities {
                    first: "chlorophyll".to_string(),
                    second: "sapsipper".to_string(),
                    hidden: "serenegrace".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Grass
                ),
                weight: 92.5 as f32,
            }
        );
        
        pokedex.insert(
            "emolga".to_string(),
            PokedexPokemon {
                species: "emolga".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 75,
                    defense: 60,
                    special_attack: 75,
                    special_defense: 60,
                    speed: 103
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "motordrive".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Flying
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "karrablast".to_string(),
            PokedexPokemon {
                species: "karrablast".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 75,
                    defense: 45,
                    special_attack: 40,
                    special_defense: 45,
                    speed: 60
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "shedskin".to_string(),
                    hidden: "noguard".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 5.9 as f32,
            }
        );
        
        pokedex.insert(
            "escavalier".to_string(),
            PokedexPokemon {
                species: "escavalier".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 135,
                    defense: 105,
                    special_attack: 60,
                    special_defense: 105,
                    speed: 20
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 33 as f32,
            }
        );
        
        pokedex.insert(
            "foongus".to_string(),
            PokedexPokemon {
                species: "foongus".to_string(),
                base_stats: BaseStats {
                    hp: 69,
                    attack: 55,
                    defense: 45,
                    special_attack: 55,
                    special_defense: 55,
                    speed: 15
                },
                abilities: Abilities {
                    first: "effectspore".to_string(),
                    second: "none".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "amoonguss".to_string(),
            PokedexPokemon {
                species: "amoonguss".to_string(),
                base_stats: BaseStats {
                    hp: 114,
                    attack: 85,
                    defense: 70,
                    special_attack: 85,
                    special_defense: 80,
                    speed: 30
                },
                abilities: Abilities {
                    first: "effectspore".to_string(),
                    second: "none".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Poison
                ),
                weight: 10.5 as f32,
            }
        );
        
        pokedex.insert(
            "frillish".to_string(),
            PokedexPokemon {
                species: "frillish".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 40,
                    defense: 50,
                    special_attack: 65,
                    special_defense: 85,
                    speed: 40
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "cursedbody".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ghost
                ),
                weight: 33 as f32,
            }
        );
        
        pokedex.insert(
            "jellicent".to_string(),
            PokedexPokemon {
                species: "jellicent".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 60,
                    defense: 70,
                    special_attack: 85,
                    special_defense: 105,
                    speed: 60
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "cursedbody".to_string(),
                    hidden: "damp".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ghost
                ),
                weight: 135 as f32,
            }
        );
        
        pokedex.insert(
            "alomomola".to_string(),
            PokedexPokemon {
                species: "alomomola".to_string(),
                base_stats: BaseStats {
                    hp: 165,
                    attack: 75,
                    defense: 80,
                    special_attack: 40,
                    special_defense: 45,
                    speed: 65
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "hydration".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 31.6 as f32,
            }
        );
        
        pokedex.insert(
            "joltik".to_string(),
            PokedexPokemon {
                species: "joltik".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 47,
                    defense: 50,
                    special_attack: 57,
                    special_defense: 50,
                    speed: 65
                },
                abilities: Abilities {
                    first: "compoundeyes".to_string(),
                    second: "unnerve".to_string(),
                    hidden: "swarm".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Electric
                ),
                weight: 0.6 as f32,
            }
        );
        
        pokedex.insert(
            "galvantula".to_string(),
            PokedexPokemon {
                species: "galvantula".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 77,
                    defense: 60,
                    special_attack: 97,
                    special_defense: 60,
                    speed: 108
                },
                abilities: Abilities {
                    first: "compoundeyes".to_string(),
                    second: "unnerve".to_string(),
                    hidden: "swarm".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Electric
                ),
                weight: 14.3 as f32,
            }
        );
        
        pokedex.insert(
            "ferroseed".to_string(),
            PokedexPokemon {
                species: "ferroseed".to_string(),
                base_stats: BaseStats {
                    hp: 44,
                    attack: 50,
                    defense: 91,
                    special_attack: 24,
                    special_defense: 86,
                    speed: 10
                },
                abilities: Abilities {
                    first: "ironbarbs".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Steel
                ),
                weight: 18.8 as f32,
            }
        );
        
        pokedex.insert(
            "ferrothorn".to_string(),
            PokedexPokemon {
                species: "ferrothorn".to_string(),
                base_stats: BaseStats {
                    hp: 74,
                    attack: 94,
                    defense: 131,
                    special_attack: 54,
                    special_defense: 116,
                    speed: 20
                },
                abilities: Abilities {
                    first: "ironbarbs".to_string(),
                    second: "none".to_string(),
                    hidden: "anticipation".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Steel
                ),
                weight: 110 as f32,
            }
        );
        
        pokedex.insert(
            "klink".to_string(),
            PokedexPokemon {
                species: "klink".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 55,
                    defense: 70,
                    special_attack: 45,
                    special_defense: 60,
                    speed: 30
                },
                abilities: Abilities {
                    first: "plus".to_string(),
                    second: "minus".to_string(),
                    hidden: "clearbody".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 21 as f32,
            }
        );
        
        pokedex.insert(
            "klang".to_string(),
            PokedexPokemon {
                species: "klang".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 80,
                    defense: 95,
                    special_attack: 70,
                    special_defense: 85,
                    speed: 50
                },
                abilities: Abilities {
                    first: "plus".to_string(),
                    second: "minus".to_string(),
                    hidden: "clearbody".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 51 as f32,
            }
        );
        
        pokedex.insert(
            "klinklang".to_string(),
            PokedexPokemon {
                species: "klinklang".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 100,
                    defense: 115,
                    special_attack: 70,
                    special_defense: 85,
                    speed: 90
                },
                abilities: Abilities {
                    first: "plus".to_string(),
                    second: "minus".to_string(),
                    hidden: "clearbody".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 81 as f32,
            }
        );
        
        pokedex.insert(
            "tynamo".to_string(),
            PokedexPokemon {
                species: "tynamo".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 45,
                    special_defense: 40,
                    speed: 60
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "eelektrik".to_string(),
            PokedexPokemon {
                species: "eelektrik".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 85,
                    defense: 70,
                    special_attack: 75,
                    special_defense: 70,
                    speed: 40
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 22 as f32,
            }
        );
        
        pokedex.insert(
            "eelektross".to_string(),
            PokedexPokemon {
                species: "eelektross".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 115,
                    defense: 80,
                    special_attack: 105,
                    special_defense: 80,
                    speed: 50
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 80.5 as f32,
            }
        );
        
        pokedex.insert(
            "elgyem".to_string(),
            PokedexPokemon {
                species: "elgyem".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 55,
                    defense: 55,
                    special_attack: 85,
                    special_defense: 55,
                    speed: 30
                },
                abilities: Abilities {
                    first: "telepathy".to_string(),
                    second: "synchronize".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 9 as f32,
            }
        );
        
        pokedex.insert(
            "beheeyem".to_string(),
            PokedexPokemon {
                species: "beheeyem".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 75,
                    defense: 75,
                    special_attack: 125,
                    special_defense: 95,
                    speed: 40
                },
                abilities: Abilities {
                    first: "telepathy".to_string(),
                    second: "synchronize".to_string(),
                    hidden: "analytic".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 34.5 as f32,
            }
        );
        
        pokedex.insert(
            "litwick".to_string(),
            PokedexPokemon {
                species: "litwick".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 30,
                    defense: 55,
                    special_attack: 65,
                    special_defense: 55,
                    speed: 20
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "flamebody".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Fire
                ),
                weight: 3.1 as f32,
            }
        );
        
        pokedex.insert(
            "lampent".to_string(),
            PokedexPokemon {
                species: "lampent".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 40,
                    defense: 60,
                    special_attack: 95,
                    special_defense: 60,
                    speed: 55
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "flamebody".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Fire
                ),
                weight: 13 as f32,
            }
        );
        
        pokedex.insert(
            "chandelure".to_string(),
            PokedexPokemon {
                species: "chandelure".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 55,
                    defense: 90,
                    special_attack: 145,
                    special_defense: 90,
                    speed: 80
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "flamebody".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Fire
                ),
                weight: 34.3 as f32,
            }
        );
        
        pokedex.insert(
            "axew".to_string(),
            PokedexPokemon {
                species: "axew".to_string(),
                base_stats: BaseStats {
                    hp: 46,
                    attack: 87,
                    defense: 60,
                    special_attack: 30,
                    special_defense: 40,
                    speed: 57
                },
                abilities: Abilities {
                    first: "rivalry".to_string(),
                    second: "moldbreaker".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "fraxure".to_string(),
            PokedexPokemon {
                species: "fraxure".to_string(),
                base_stats: BaseStats {
                    hp: 66,
                    attack: 117,
                    defense: 70,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 67
                },
                abilities: Abilities {
                    first: "rivalry".to_string(),
                    second: "moldbreaker".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 36 as f32,
            }
        );
        
        pokedex.insert(
            "haxorus".to_string(),
            PokedexPokemon {
                species: "haxorus".to_string(),
                base_stats: BaseStats {
                    hp: 76,
                    attack: 147,
                    defense: 90,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 97
                },
                abilities: Abilities {
                    first: "rivalry".to_string(),
                    second: "moldbreaker".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 105.5 as f32,
            }
        );
        
        pokedex.insert(
            "cubchoo".to_string(),
            PokedexPokemon {
                species: "cubchoo".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 70,
                    defense: 40,
                    special_attack: 60,
                    special_defense: 40,
                    speed: 40
                },
                abilities: Abilities {
                    first: "snowcloak".to_string(),
                    second: "slushrush".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "beartic".to_string(),
            PokedexPokemon {
                species: "beartic".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 130,
                    defense: 80,
                    special_attack: 70,
                    special_defense: 80,
                    speed: 50
                },
                abilities: Abilities {
                    first: "snowcloak".to_string(),
                    second: "slushrush".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 260 as f32,
            }
        );
        
        pokedex.insert(
            "cryogonal".to_string(),
            PokedexPokemon {
                species: "cryogonal".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 50,
                    defense: 50,
                    special_attack: 95,
                    special_defense: 135,
                    speed: 105
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 148 as f32,
            }
        );
        
        pokedex.insert(
            "shelmet".to_string(),
            PokedexPokemon {
                species: "shelmet".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 40,
                    defense: 85,
                    special_attack: 40,
                    special_defense: 65,
                    speed: 25
                },
                abilities: Abilities {
                    first: "hydration".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 7.7 as f32,
            }
        );
        
        pokedex.insert(
            "accelgor".to_string(),
            PokedexPokemon {
                species: "accelgor".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 70,
                    defense: 40,
                    special_attack: 100,
                    special_defense: 60,
                    speed: 145
                },
                abilities: Abilities {
                    first: "hydration".to_string(),
                    second: "stickyhold".to_string(),
                    hidden: "unburden".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 25.3 as f32,
            }
        );
        
        pokedex.insert(
            "stunfisk".to_string(),
            PokedexPokemon {
                species: "stunfisk".to_string(),
                base_stats: BaseStats {
                    hp: 109,
                    attack: 66,
                    defense: 84,
                    special_attack: 81,
                    special_defense: 99,
                    speed: 32
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "limber".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Electric
                ),
                weight: 11 as f32,
            }
        );
        
        pokedex.insert(
            "mienfoo".to_string(),
            PokedexPokemon {
                species: "mienfoo".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 85,
                    defense: 50,
                    special_attack: 55,
                    special_defense: 50,
                    speed: 65
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "regenerator".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 20 as f32,
            }
        );
        
        pokedex.insert(
            "mienshao".to_string(),
            PokedexPokemon {
                species: "mienshao".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 125,
                    defense: 60,
                    special_attack: 95,
                    special_defense: 60,
                    speed: 105
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "regenerator".to_string(),
                    hidden: "reckless".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 35.5 as f32,
            }
        );
        
        pokedex.insert(
            "druddigon".to_string(),
            PokedexPokemon {
                species: "druddigon".to_string(),
                base_stats: BaseStats {
                    hp: 77,
                    attack: 120,
                    defense: 90,
                    special_attack: 60,
                    special_defense: 90,
                    speed: 48
                },
                abilities: Abilities {
                    first: "roughskin".to_string(),
                    second: "sheerforce".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 139 as f32,
            }
        );
        
        pokedex.insert(
            "golett".to_string(),
            PokedexPokemon {
                species: "golett".to_string(),
                base_stats: BaseStats {
                    hp: 59,
                    attack: 74,
                    defense: 50,
                    special_attack: 35,
                    special_defense: 50,
                    speed: 35
                },
                abilities: Abilities {
                    first: "ironfist".to_string(),
                    second: "klutz".to_string(),
                    hidden: "noguard".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Ghost
                ),
                weight: 92 as f32,
            }
        );
        
        pokedex.insert(
            "golurk".to_string(),
            PokedexPokemon {
                species: "golurk".to_string(),
                base_stats: BaseStats {
                    hp: 89,
                    attack: 124,
                    defense: 80,
                    special_attack: 55,
                    special_defense: 80,
                    speed: 55
                },
                abilities: Abilities {
                    first: "ironfist".to_string(),
                    second: "klutz".to_string(),
                    hidden: "noguard".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Ghost
                ),
                weight: 330 as f32,
            }
        );
        
        pokedex.insert(
            "pawniard".to_string(),
            PokedexPokemon {
                species: "pawniard".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 85,
                    defense: 70,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 60
                },
                abilities: Abilities {
                    first: "defiant".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "pressure".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Steel
                ),
                weight: 10.2 as f32,
            }
        );
        
        pokedex.insert(
            "bisharp".to_string(),
            PokedexPokemon {
                species: "bisharp".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 125,
                    defense: 100,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 70
                },
                abilities: Abilities {
                    first: "defiant".to_string(),
                    second: "innerfocus".to_string(),
                    hidden: "pressure".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Steel
                ),
                weight: 70 as f32,
            }
        );
        
        pokedex.insert(
            "bouffalant".to_string(),
            PokedexPokemon {
                species: "bouffalant".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 110,
                    defense: 95,
                    special_attack: 40,
                    special_defense: 95,
                    speed: 55
                },
                abilities: Abilities {
                    first: "reckless".to_string(),
                    second: "sapsipper".to_string(),
                    hidden: "soundproof".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 94.6 as f32,
            }
        );
        
        pokedex.insert(
            "rufflet".to_string(),
            PokedexPokemon {
                species: "rufflet".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 83,
                    defense: 50,
                    special_attack: 37,
                    special_defense: 50,
                    speed: 60
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "sheerforce".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 10.5 as f32,
            }
        );
        
        pokedex.insert(
            "braviary".to_string(),
            PokedexPokemon {
                species: "braviary".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 123,
                    defense: 75,
                    special_attack: 57,
                    special_defense: 75,
                    speed: 80
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "sheerforce".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 41 as f32,
            }
        );
        
        pokedex.insert(
            "vullaby".to_string(),
            PokedexPokemon {
                species: "vullaby".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 55,
                    defense: 75,
                    special_attack: 45,
                    special_defense: 65,
                    speed: 60
                },
                abilities: Abilities {
                    first: "bigpecks".to_string(),
                    second: "overcoat".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Flying
                ),
                weight: 9 as f32,
            }
        );
        
        pokedex.insert(
            "mandibuzz".to_string(),
            PokedexPokemon {
                species: "mandibuzz".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 65,
                    defense: 105,
                    special_attack: 55,
                    special_defense: 95,
                    speed: 80
                },
                abilities: Abilities {
                    first: "bigpecks".to_string(),
                    second: "overcoat".to_string(),
                    hidden: "weakarmor".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Flying
                ),
                weight: 39.5 as f32,
            }
        );
        
        pokedex.insert(
            "heatmor".to_string(),
            PokedexPokemon {
                species: "heatmor".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 97,
                    defense: 66,
                    special_attack: 105,
                    special_defense: 66,
                    speed: 65
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "flashfire".to_string(),
                    hidden: "whitesmoke".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 58 as f32,
            }
        );
        
        pokedex.insert(
            "durant".to_string(),
            PokedexPokemon {
                species: "durant".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 109,
                    defense: 112,
                    special_attack: 48,
                    special_defense: 48,
                    speed: 109
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "hustle".to_string(),
                    hidden: "truant".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 33 as f32,
            }
        );
        
        pokedex.insert(
            "deino".to_string(),
            PokedexPokemon {
                species: "deino".to_string(),
                base_stats: BaseStats {
                    hp: 52,
                    attack: 65,
                    defense: 50,
                    special_attack: 45,
                    special_defense: 50,
                    speed: 38
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Dragon
                ),
                weight: 17.3 as f32,
            }
        );
        
        pokedex.insert(
            "zweilous".to_string(),
            PokedexPokemon {
                species: "zweilous".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 85,
                    defense: 70,
                    special_attack: 65,
                    special_defense: 70,
                    speed: 58
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Dragon
                ),
                weight: 50 as f32,
            }
        );
        
        pokedex.insert(
            "hydreigon".to_string(),
            PokedexPokemon {
                species: "hydreigon".to_string(),
                base_stats: BaseStats {
                    hp: 92,
                    attack: 105,
                    defense: 90,
                    special_attack: 125,
                    special_defense: 90,
                    speed: 98
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Dragon
                ),
                weight: 160 as f32,
            }
        );
        
        pokedex.insert(
            "larvesta".to_string(),
            PokedexPokemon {
                species: "larvesta".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 85,
                    defense: 55,
                    special_attack: 50,
                    special_defense: 55,
                    speed: 60
                },
                abilities: Abilities {
                    first: "flamebody".to_string(),
                    second: "none".to_string(),
                    hidden: "swarm".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fire
                ),
                weight: 28.8 as f32,
            }
        );
        
        pokedex.insert(
            "volcarona".to_string(),
            PokedexPokemon {
                species: "volcarona".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 60,
                    defense: 65,
                    special_attack: 135,
                    special_defense: 105,
                    speed: 100
                },
                abilities: Abilities {
                    first: "flamebody".to_string(),
                    second: "none".to_string(),
                    hidden: "swarm".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fire
                ),
                weight: 46 as f32,
            }
        );
        
        pokedex.insert(
            "cobalion".to_string(),
            PokedexPokemon {
                species: "cobalion".to_string(),
                base_stats: BaseStats {
                    hp: 91,
                    attack: 90,
                    defense: 129,
                    special_attack: 90,
                    special_defense: 72,
                    speed: 108
                },
                abilities: Abilities {
                    first: "justified".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Fighting
                ),
                weight: 250 as f32,
            }
        );
        
        pokedex.insert(
            "terrakion".to_string(),
            PokedexPokemon {
                species: "terrakion".to_string(),
                base_stats: BaseStats {
                    hp: 91,
                    attack: 129,
                    defense: 90,
                    special_attack: 72,
                    special_defense: 90,
                    speed: 108
                },
                abilities: Abilities {
                    first: "justified".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Fighting
                ),
                weight: 260 as f32,
            }
        );
        
        pokedex.insert(
            "virizion".to_string(),
            PokedexPokemon {
                species: "virizion".to_string(),
                base_stats: BaseStats {
                    hp: 91,
                    attack: 90,
                    defense: 72,
                    special_attack: 90,
                    special_defense: 129,
                    speed: 108
                },
                abilities: Abilities {
                    first: "justified".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Fighting
                ),
                weight: 200 as f32,
            }
        );
        
        pokedex.insert(
            "tornadus".to_string(),
            PokedexPokemon {
                species: "tornadus".to_string(),
                base_stats: BaseStats {
                    hp: 79,
                    attack: 115,
                    defense: 70,
                    special_attack: 125,
                    special_defense: 80,
                    speed: 111
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "none".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Typeless
                ),
                weight: 63 as f32,
            }
        );
        
        pokedex.insert(
            "tornadustherian".to_string(),
            PokedexPokemon {
                species: "tornadustherian".to_string(),
                base_stats: BaseStats {
                    hp: 79,
                    attack: 100,
                    defense: 80,
                    special_attack: 110,
                    special_defense: 90,
                    speed: 121
                },
                abilities: Abilities {
                    first: "regenerator".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Typeless
                ),
                weight: 63 as f32,
            }
        );
        
        pokedex.insert(
            "thundurus".to_string(),
            PokedexPokemon {
                species: "thundurus".to_string(),
                base_stats: BaseStats {
                    hp: 79,
                    attack: 115,
                    defense: 70,
                    special_attack: 125,
                    special_defense: 80,
                    speed: 111
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "none".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Flying
                ),
                weight: 61 as f32,
            }
        );
        
        pokedex.insert(
            "thundurustherian".to_string(),
            PokedexPokemon {
                species: "thundurustherian".to_string(),
                base_stats: BaseStats {
                    hp: 79,
                    attack: 105,
                    defense: 70,
                    special_attack: 145,
                    special_defense: 80,
                    speed: 101
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Flying
                ),
                weight: 61 as f32,
            }
        );
        
        pokedex.insert(
            "reshiram".to_string(),
            PokedexPokemon {
                species: "reshiram".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 120,
                    defense: 100,
                    special_attack: 150,
                    special_defense: 120,
                    speed: 90
                },
                abilities: Abilities {
                    first: "turboblaze".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Fire
                ),
                weight: 330 as f32,
            }
        );
        
        pokedex.insert(
            "zekrom".to_string(),
            PokedexPokemon {
                species: "zekrom".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 150,
                    defense: 120,
                    special_attack: 120,
                    special_defense: 100,
                    speed: 90
                },
                abilities: Abilities {
                    first: "teravolt".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Electric
                ),
                weight: 345 as f32,
            }
        );
        
        pokedex.insert(
            "landorus".to_string(),
            PokedexPokemon {
                species: "landorus".to_string(),
                base_stats: BaseStats {
                    hp: 89,
                    attack: 125,
                    defense: 90,
                    special_attack: 115,
                    special_defense: 80,
                    speed: 101
                },
                abilities: Abilities {
                    first: "sandforce".to_string(),
                    second: "none".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Flying
                ),
                weight: 68 as f32,
            }
        );
        
        pokedex.insert(
            "landorustherian".to_string(),
            PokedexPokemon {
                species: "landorustherian".to_string(),
                base_stats: BaseStats {
                    hp: 89,
                    attack: 145,
                    defense: 90,
                    special_attack: 105,
                    special_defense: 80,
                    speed: 91
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Flying
                ),
                weight: 68 as f32,
            }
        );
        
        pokedex.insert(
            "kyurem".to_string(),
            PokedexPokemon {
                species: "kyurem".to_string(),
                base_stats: BaseStats {
                    hp: 125,
                    attack: 130,
                    defense: 90,
                    special_attack: 130,
                    special_defense: 90,
                    speed: 95
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ice
                ),
                weight: 325 as f32,
            }
        );
        
        pokedex.insert(
            "kyuremblack".to_string(),
            PokedexPokemon {
                species: "kyuremblack".to_string(),
                base_stats: BaseStats {
                    hp: 125,
                    attack: 170,
                    defense: 100,
                    special_attack: 120,
                    special_defense: 90,
                    speed: 95
                },
                abilities: Abilities {
                    first: "teravolt".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ice
                ),
                weight: 325 as f32,
            }
        );
        
        pokedex.insert(
            "kyuremwhite".to_string(),
            PokedexPokemon {
                species: "kyuremwhite".to_string(),
                base_stats: BaseStats {
                    hp: 125,
                    attack: 120,
                    defense: 90,
                    special_attack: 170,
                    special_defense: 100,
                    speed: 95
                },
                abilities: Abilities {
                    first: "turboblaze".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ice
                ),
                weight: 325 as f32,
            }
        );
        
        pokedex.insert(
            "keldeo".to_string(),
            PokedexPokemon {
                species: "keldeo".to_string(),
                base_stats: BaseStats {
                    hp: 91,
                    attack: 72,
                    defense: 90,
                    special_attack: 129,
                    special_defense: 90,
                    speed: 108
                },
                abilities: Abilities {
                    first: "justified".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fighting
                ),
                weight: 48.5 as f32,
            }
        );
        
        pokedex.insert(
            "keldeoresolute".to_string(),
            PokedexPokemon {
                species: "keldeoresolute".to_string(),
                base_stats: BaseStats {
                    hp: 91,
                    attack: 72,
                    defense: 90,
                    special_attack: 129,
                    special_defense: 90,
                    speed: 108
                },
                abilities: Abilities {
                    first: "justified".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fighting
                ),
                weight: 48.5 as f32,
            }
        );
        
        pokedex.insert(
            "meloetta".to_string(),
            PokedexPokemon {
                species: "meloetta".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 77,
                    defense: 77,
                    special_attack: 128,
                    special_defense: 128,
                    speed: 90
                },
                abilities: Abilities {
                    first: "serenegrace".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Psychic
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "meloettapirouette".to_string(),
            PokedexPokemon {
                species: "meloettapirouette".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 128,
                    defense: 90,
                    special_attack: 77,
                    special_defense: 77,
                    speed: 128
                },
                abilities: Abilities {
                    first: "serenegrace".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fighting
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "genesect".to_string(),
            PokedexPokemon {
                species: "genesect".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 120,
                    defense: 95,
                    special_attack: 120,
                    special_defense: 95,
                    speed: 99
                },
                abilities: Abilities {
                    first: "download".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 82.5 as f32,
            }
        );
        
        pokedex.insert(
            "genesectdouse".to_string(),
            PokedexPokemon {
                species: "genesectdouse".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 120,
                    defense: 95,
                    special_attack: 120,
                    special_defense: 95,
                    speed: 99
                },
                abilities: Abilities {
                    first: "download".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 82.5 as f32,
            }
        );
        
        pokedex.insert(
            "genesectshock".to_string(),
            PokedexPokemon {
                species: "genesectshock".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 120,
                    defense: 95,
                    special_attack: 120,
                    special_defense: 95,
                    speed: 99
                },
                abilities: Abilities {
                    first: "download".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 82.5 as f32,
            }
        );
        
        pokedex.insert(
            "genesectburn".to_string(),
            PokedexPokemon {
                species: "genesectburn".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 120,
                    defense: 95,
                    special_attack: 120,
                    special_defense: 95,
                    speed: 99
                },
                abilities: Abilities {
                    first: "download".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 82.5 as f32,
            }
        );
        
        pokedex.insert(
            "genesectchill".to_string(),
            PokedexPokemon {
                species: "genesectchill".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 120,
                    defense: 95,
                    special_attack: 120,
                    special_defense: 95,
                    speed: 99
                },
                abilities: Abilities {
                    first: "download".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Steel
                ),
                weight: 82.5 as f32,
            }
        );
        
        pokedex.insert(
            "chespin".to_string(),
            PokedexPokemon {
                species: "chespin".to_string(),
                base_stats: BaseStats {
                    hp: 56,
                    attack: 61,
                    defense: 65,
                    special_attack: 48,
                    special_defense: 45,
                    speed: 38
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "bulletproof".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 9 as f32,
            }
        );
        
        pokedex.insert(
            "quilladin".to_string(),
            PokedexPokemon {
                species: "quilladin".to_string(),
                base_stats: BaseStats {
                    hp: 61,
                    attack: 78,
                    defense: 95,
                    special_attack: 56,
                    special_defense: 58,
                    speed: 57
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "bulletproof".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 29 as f32,
            }
        );
        
        pokedex.insert(
            "chesnaught".to_string(),
            PokedexPokemon {
                species: "chesnaught".to_string(),
                base_stats: BaseStats {
                    hp: 88,
                    attack: 107,
                    defense: 122,
                    special_attack: 74,
                    special_defense: 75,
                    speed: 64
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "bulletproof".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Fighting
                ),
                weight: 90 as f32,
            }
        );
        
        pokedex.insert(
            "fennekin".to_string(),
            PokedexPokemon {
                species: "fennekin".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 40,
                    special_attack: 62,
                    special_defense: 60,
                    speed: 60
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "magician".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 9.4 as f32,
            }
        );
        
        pokedex.insert(
            "braixen".to_string(),
            PokedexPokemon {
                species: "braixen".to_string(),
                base_stats: BaseStats {
                    hp: 59,
                    attack: 59,
                    defense: 58,
                    special_attack: 90,
                    special_defense: 70,
                    speed: 73
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "magician".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 14.5 as f32,
            }
        );
        
        pokedex.insert(
            "delphox".to_string(),
            PokedexPokemon {
                species: "delphox".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 69,
                    defense: 72,
                    special_attack: 114,
                    special_defense: 100,
                    speed: 104
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "magician".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Psychic
                ),
                weight: 39 as f32,
            }
        );
        
        pokedex.insert(
            "froakie".to_string(),
            PokedexPokemon {
                species: "froakie".to_string(),
                base_stats: BaseStats {
                    hp: 41,
                    attack: 56,
                    defense: 40,
                    special_attack: 62,
                    special_defense: 44,
                    speed: 71
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "protean".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 7 as f32,
            }
        );
        
        pokedex.insert(
            "frogadier".to_string(),
            PokedexPokemon {
                species: "frogadier".to_string(),
                base_stats: BaseStats {
                    hp: 54,
                    attack: 63,
                    defense: 52,
                    special_attack: 83,
                    special_defense: 56,
                    speed: 97
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "protean".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 10.9 as f32,
            }
        );
        
        pokedex.insert(
            "greninja".to_string(),
            PokedexPokemon {
                species: "greninja".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 95,
                    defense: 67,
                    special_attack: 103,
                    special_defense: 71,
                    speed: 122
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "battlebond".to_string(),
                    hidden: "protean".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dark
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "greninjaash".to_string(),
            PokedexPokemon {
                species: "greninjaash".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 145,
                    defense: 67,
                    special_attack: 153,
                    special_defense: 71,
                    speed: 132
                },
                abilities: Abilities {
                    first: "battlebond".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dark
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "bunnelby".to_string(),
            PokedexPokemon {
                species: "bunnelby".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 36,
                    defense: 38,
                    special_attack: 32,
                    special_defense: 36,
                    speed: 57
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "cheekpouch".to_string(),
                    hidden: "hugepower".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "diggersby".to_string(),
            PokedexPokemon {
                species: "diggersby".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 56,
                    defense: 77,
                    special_attack: 50,
                    special_defense: 77,
                    speed: 78
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "cheekpouch".to_string(),
                    hidden: "hugepower".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Ground
                ),
                weight: 42.4 as f32,
            }
        );
        
        pokedex.insert(
            "fletchling".to_string(),
            PokedexPokemon {
                species: "fletchling".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 50,
                    defense: 43,
                    special_attack: 40,
                    special_defense: 38,
                    speed: 62
                },
                abilities: Abilities {
                    first: "bigpecks".to_string(),
                    second: "none".to_string(),
                    hidden: "galewings".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 1.7 as f32,
            }
        );
        
        pokedex.insert(
            "fletchinder".to_string(),
            PokedexPokemon {
                species: "fletchinder".to_string(),
                base_stats: BaseStats {
                    hp: 62,
                    attack: 73,
                    defense: 55,
                    special_attack: 56,
                    special_defense: 52,
                    speed: 84
                },
                abilities: Abilities {
                    first: "flamebody".to_string(),
                    second: "none".to_string(),
                    hidden: "galewings".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Flying
                ),
                weight: 16 as f32,
            }
        );
        
        pokedex.insert(
            "talonflame".to_string(),
            PokedexPokemon {
                species: "talonflame".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 81,
                    defense: 71,
                    special_attack: 74,
                    special_defense: 69,
                    speed: 126
                },
                abilities: Abilities {
                    first: "flamebody".to_string(),
                    second: "none".to_string(),
                    hidden: "galewings".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Flying
                ),
                weight: 24.5 as f32,
            }
        );
        
        pokedex.insert(
            "scatterbug".to_string(),
            PokedexPokemon {
                species: "scatterbug".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 35,
                    defense: 40,
                    special_attack: 27,
                    special_defense: 25,
                    speed: 35
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "compoundeyes".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 2.5 as f32,
            }
        );
        
        pokedex.insert(
            "spewpa".to_string(),
            PokedexPokemon {
                species: "spewpa".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 22,
                    defense: 60,
                    special_attack: 27,
                    special_defense: 30,
                    speed: 29
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "none".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 8.4 as f32,
            }
        );
        
        pokedex.insert(
            "vivillon".to_string(),
            PokedexPokemon {
                species: "vivillon".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 52,
                    defense: 50,
                    special_attack: 90,
                    special_defense: 50,
                    speed: 89
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "compoundeyes".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 17 as f32,
            }
        );
        
        pokedex.insert(
            "vivillonfancy".to_string(),
            PokedexPokemon {
                species: "vivillonfancy".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 52,
                    defense: 50,
                    special_attack: 90,
                    special_defense: 50,
                    speed: 89
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "compoundeyes".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 17 as f32,
            }
        );
        
        pokedex.insert(
            "vivillonpokeball".to_string(),
            PokedexPokemon {
                species: "vivillonpokeball".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 52,
                    defense: 50,
                    special_attack: 90,
                    special_defense: 50,
                    speed: 89
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "compoundeyes".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Flying
                ),
                weight: 17 as f32,
            }
        );
        
        pokedex.insert(
            "litleo".to_string(),
            PokedexPokemon {
                species: "litleo".to_string(),
                base_stats: BaseStats {
                    hp: 62,
                    attack: 50,
                    defense: 58,
                    special_attack: 73,
                    special_defense: 54,
                    speed: 72
                },
                abilities: Abilities {
                    first: "rivalry".to_string(),
                    second: "unnerve".to_string(),
                    hidden: "moxie".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Normal
                ),
                weight: 13.5 as f32,
            }
        );
        
        pokedex.insert(
            "pyroar".to_string(),
            PokedexPokemon {
                species: "pyroar".to_string(),
                base_stats: BaseStats {
                    hp: 86,
                    attack: 68,
                    defense: 72,
                    special_attack: 109,
                    special_defense: 66,
                    speed: 106
                },
                abilities: Abilities {
                    first: "rivalry".to_string(),
                    second: "unnerve".to_string(),
                    hidden: "moxie".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Normal
                ),
                weight: 81.5 as f32,
            }
        );
        
        pokedex.insert(
            "flabebe".to_string(),
            PokedexPokemon {
                species: "flabebe".to_string(),
                base_stats: BaseStats {
                    hp: 44,
                    attack: 38,
                    defense: 39,
                    special_attack: 61,
                    special_defense: 79,
                    speed: 42
                },
                abilities: Abilities {
                    first: "flowerveil".to_string(),
                    second: "none".to_string(),
                    hidden: "symbiosis".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 0.1 as f32,
            }
        );
        
        pokedex.insert(
            "floette".to_string(),
            PokedexPokemon {
                species: "floette".to_string(),
                base_stats: BaseStats {
                    hp: 54,
                    attack: 45,
                    defense: 47,
                    special_attack: 75,
                    special_defense: 98,
                    speed: 52
                },
                abilities: Abilities {
                    first: "flowerveil".to_string(),
                    second: "none".to_string(),
                    hidden: "symbiosis".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 0.9 as f32,
            }
        );
        
        pokedex.insert(
            "floetteeternal".to_string(),
            PokedexPokemon {
                species: "floetteeternal".to_string(),
                base_stats: BaseStats {
                    hp: 74,
                    attack: 65,
                    defense: 67,
                    special_attack: 125,
                    special_defense: 128,
                    speed: 92
                },
                abilities: Abilities {
                    first: "flowerveil".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 0.9 as f32,
            }
        );
        
        pokedex.insert(
            "florges".to_string(),
            PokedexPokemon {
                species: "florges".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 65,
                    defense: 68,
                    special_attack: 112,
                    special_defense: 154,
                    speed: 75
                },
                abilities: Abilities {
                    first: "flowerveil".to_string(),
                    second: "none".to_string(),
                    hidden: "symbiosis".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 10 as f32,
            }
        );
        
        pokedex.insert(
            "skiddo".to_string(),
            PokedexPokemon {
                species: "skiddo".to_string(),
                base_stats: BaseStats {
                    hp: 66,
                    attack: 65,
                    defense: 48,
                    special_attack: 62,
                    special_defense: 57,
                    speed: 52
                },
                abilities: Abilities {
                    first: "sapsipper".to_string(),
                    second: "none".to_string(),
                    hidden: "grasspelt".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 31 as f32,
            }
        );
        
        pokedex.insert(
            "gogoat".to_string(),
            PokedexPokemon {
                species: "gogoat".to_string(),
                base_stats: BaseStats {
                    hp: 123,
                    attack: 100,
                    defense: 62,
                    special_attack: 97,
                    special_defense: 81,
                    speed: 68
                },
                abilities: Abilities {
                    first: "sapsipper".to_string(),
                    second: "none".to_string(),
                    hidden: "grasspelt".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 91 as f32,
            }
        );
        
        pokedex.insert(
            "pancham".to_string(),
            PokedexPokemon {
                species: "pancham".to_string(),
                base_stats: BaseStats {
                    hp: 67,
                    attack: 82,
                    defense: 62,
                    special_attack: 46,
                    special_defense: 48,
                    speed: 43
                },
                abilities: Abilities {
                    first: "ironfist".to_string(),
                    second: "moldbreaker".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 8 as f32,
            }
        );
        
        pokedex.insert(
            "pangoro".to_string(),
            PokedexPokemon {
                species: "pangoro".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 124,
                    defense: 78,
                    special_attack: 69,
                    special_defense: 71,
                    speed: 58
                },
                abilities: Abilities {
                    first: "ironfist".to_string(),
                    second: "moldbreaker".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Dark
                ),
                weight: 136 as f32,
            }
        );
        
        pokedex.insert(
            "furfrou".to_string(),
            PokedexPokemon {
                species: "furfrou".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 80,
                    defense: 60,
                    special_attack: 65,
                    special_defense: 90,
                    speed: 102
                },
                abilities: Abilities {
                    first: "furcoat".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "espurr".to_string(),
            PokedexPokemon {
                species: "espurr".to_string(),
                base_stats: BaseStats {
                    hp: 62,
                    attack: 48,
                    defense: 54,
                    special_attack: 63,
                    special_defense: 60,
                    speed: 68
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "owntempo".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 3.5 as f32,
            }
        );
        
        pokedex.insert(
            "meowstic".to_string(),
            PokedexPokemon {
                species: "meowstic".to_string(),
                base_stats: BaseStats {
                    hp: 74,
                    attack: 48,
                    defense: 76,
                    special_attack: 83,
                    special_defense: 81,
                    speed: 104
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "prankster".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "meowsticf".to_string(),
            PokedexPokemon {
                species: "meowsticf".to_string(),
                base_stats: BaseStats {
                    hp: 74,
                    attack: 48,
                    defense: 76,
                    special_attack: 83,
                    special_defense: 81,
                    speed: 104
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "competitive".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "honedge".to_string(),
            PokedexPokemon {
                species: "honedge".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 80,
                    defense: 100,
                    special_attack: 35,
                    special_defense: 37,
                    speed: 28
                },
                abilities: Abilities {
                    first: "noguard".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Ghost
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "doublade".to_string(),
            PokedexPokemon {
                species: "doublade".to_string(),
                base_stats: BaseStats {
                    hp: 59,
                    attack: 110,
                    defense: 150,
                    special_attack: 45,
                    special_defense: 49,
                    speed: 35
                },
                abilities: Abilities {
                    first: "noguard".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Ghost
                ),
                weight: 4.5 as f32,
            }
        );
        
        pokedex.insert(
            "aegislash".to_string(),
            PokedexPokemon {
                species: "aegislash".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 50,
                    defense: 140,
                    special_attack: 50,
                    special_defense: 140,
                    speed: 60
                },
                abilities: Abilities {
                    first: "stancechange".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Ghost
                ),
                weight: 53 as f32,
            }
        );
        
        pokedex.insert(
            "aegislashblade".to_string(),
            PokedexPokemon {
                species: "aegislashblade".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 140,
                    defense: 50,
                    special_attack: 140,
                    special_defense: 50,
                    speed: 60
                },
                abilities: Abilities {
                    first: "stancechange".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Ghost
                ),
                weight: 53 as f32,
            }
        );
        
        pokedex.insert(
            "spritzee".to_string(),
            PokedexPokemon {
                species: "spritzee".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 52,
                    defense: 60,
                    special_attack: 63,
                    special_defense: 65,
                    speed: 23
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "none".to_string(),
                    hidden: "aromaveil".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 0.5 as f32,
            }
        );
        
        pokedex.insert(
            "aromatisse".to_string(),
            PokedexPokemon {
                species: "aromatisse".to_string(),
                base_stats: BaseStats {
                    hp: 101,
                    attack: 72,
                    defense: 72,
                    special_attack: 99,
                    special_defense: 89,
                    speed: 29
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "none".to_string(),
                    hidden: "aromaveil".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 15.5 as f32,
            }
        );
        
        pokedex.insert(
            "swirlix".to_string(),
            PokedexPokemon {
                species: "swirlix".to_string(),
                base_stats: BaseStats {
                    hp: 62,
                    attack: 48,
                    defense: 66,
                    special_attack: 59,
                    special_defense: 57,
                    speed: 49
                },
                abilities: Abilities {
                    first: "sweetveil".to_string(),
                    second: "none".to_string(),
                    hidden: "unburden".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 3.5 as f32,
            }
        );
        
        pokedex.insert(
            "slurpuff".to_string(),
            PokedexPokemon {
                species: "slurpuff".to_string(),
                base_stats: BaseStats {
                    hp: 82,
                    attack: 80,
                    defense: 86,
                    special_attack: 85,
                    special_defense: 75,
                    speed: 72
                },
                abilities: Abilities {
                    first: "sweetveil".to_string(),
                    second: "none".to_string(),
                    hidden: "unburden".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "inkay".to_string(),
            PokedexPokemon {
                species: "inkay".to_string(),
                base_stats: BaseStats {
                    hp: 53,
                    attack: 54,
                    defense: 53,
                    special_attack: 37,
                    special_defense: 46,
                    speed: 45
                },
                abilities: Abilities {
                    first: "contrary".to_string(),
                    second: "suctioncups".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Psychic
                ),
                weight: 3.5 as f32,
            }
        );
        
        pokedex.insert(
            "malamar".to_string(),
            PokedexPokemon {
                species: "malamar".to_string(),
                base_stats: BaseStats {
                    hp: 86,
                    attack: 92,
                    defense: 88,
                    special_attack: 68,
                    special_defense: 75,
                    speed: 73
                },
                abilities: Abilities {
                    first: "contrary".to_string(),
                    second: "suctioncups".to_string(),
                    hidden: "infiltrator".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Psychic
                ),
                weight: 47 as f32,
            }
        );
        
        pokedex.insert(
            "binacle".to_string(),
            PokedexPokemon {
                species: "binacle".to_string(),
                base_stats: BaseStats {
                    hp: 42,
                    attack: 52,
                    defense: 67,
                    special_attack: 39,
                    special_defense: 56,
                    speed: 50
                },
                abilities: Abilities {
                    first: "toughclaws".to_string(),
                    second: "sniper".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Water
                ),
                weight: 31 as f32,
            }
        );
        
        pokedex.insert(
            "barbaracle".to_string(),
            PokedexPokemon {
                species: "barbaracle".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 105,
                    defense: 115,
                    special_attack: 54,
                    special_defense: 86,
                    speed: 68
                },
                abilities: Abilities {
                    first: "toughclaws".to_string(),
                    second: "sniper".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Water
                ),
                weight: 96 as f32,
            }
        );
        
        pokedex.insert(
            "skrelp".to_string(),
            PokedexPokemon {
                species: "skrelp".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 60,
                    defense: 60,
                    special_attack: 60,
                    special_defense: 60,
                    speed: 30
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "poisontouch".to_string(),
                    hidden: "adaptability".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Water
                ),
                weight: 7.3 as f32,
            }
        );
        
        pokedex.insert(
            "dragalge".to_string(),
            PokedexPokemon {
                species: "dragalge".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 75,
                    defense: 90,
                    special_attack: 97,
                    special_defense: 123,
                    speed: 44
                },
                abilities: Abilities {
                    first: "poisonpoint".to_string(),
                    second: "poisontouch".to_string(),
                    hidden: "adaptability".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Dragon
                ),
                weight: 81.5 as f32,
            }
        );
        
        pokedex.insert(
            "clauncher".to_string(),
            PokedexPokemon {
                species: "clauncher".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 53,
                    defense: 62,
                    special_attack: 58,
                    special_defense: 63,
                    speed: 44
                },
                abilities: Abilities {
                    first: "megalauncher".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 8.3 as f32,
            }
        );
        
        pokedex.insert(
            "clawitzer".to_string(),
            PokedexPokemon {
                species: "clawitzer".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 73,
                    defense: 88,
                    special_attack: 120,
                    special_defense: 89,
                    speed: 59
                },
                abilities: Abilities {
                    first: "megalauncher".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 35.3 as f32,
            }
        );
        
        pokedex.insert(
            "helioptile".to_string(),
            PokedexPokemon {
                species: "helioptile".to_string(),
                base_stats: BaseStats {
                    hp: 44,
                    attack: 38,
                    defense: 33,
                    special_attack: 61,
                    special_defense: 43,
                    speed: 70
                },
                abilities: Abilities {
                    first: "dryskin".to_string(),
                    second: "sandveil".to_string(),
                    hidden: "solarpower".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Normal
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "heliolisk".to_string(),
            PokedexPokemon {
                species: "heliolisk".to_string(),
                base_stats: BaseStats {
                    hp: 62,
                    attack: 55,
                    defense: 52,
                    special_attack: 109,
                    special_defense: 94,
                    speed: 109
                },
                abilities: Abilities {
                    first: "dryskin".to_string(),
                    second: "sandveil".to_string(),
                    hidden: "solarpower".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Normal
                ),
                weight: 21 as f32,
            }
        );
        
        pokedex.insert(
            "tyrunt".to_string(),
            PokedexPokemon {
                species: "tyrunt".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 89,
                    defense: 77,
                    special_attack: 45,
                    special_defense: 45,
                    speed: 48
                },
                abilities: Abilities {
                    first: "strongjaw".to_string(),
                    second: "none".to_string(),
                    hidden: "sturdy".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Dragon
                ),
                weight: 26 as f32,
            }
        );
        
        pokedex.insert(
            "tyrantrum".to_string(),
            PokedexPokemon {
                species: "tyrantrum".to_string(),
                base_stats: BaseStats {
                    hp: 82,
                    attack: 121,
                    defense: 119,
                    special_attack: 69,
                    special_defense: 59,
                    speed: 71
                },
                abilities: Abilities {
                    first: "strongjaw".to_string(),
                    second: "none".to_string(),
                    hidden: "rockhead".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Dragon
                ),
                weight: 270 as f32,
            }
        );
        
        pokedex.insert(
            "amaura".to_string(),
            PokedexPokemon {
                species: "amaura".to_string(),
                base_stats: BaseStats {
                    hp: 77,
                    attack: 59,
                    defense: 50,
                    special_attack: 67,
                    special_defense: 63,
                    speed: 46
                },
                abilities: Abilities {
                    first: "refrigerate".to_string(),
                    second: "none".to_string(),
                    hidden: "snowwarning".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Ice
                ),
                weight: 25.2 as f32,
            }
        );
        
        pokedex.insert(
            "aurorus".to_string(),
            PokedexPokemon {
                species: "aurorus".to_string(),
                base_stats: BaseStats {
                    hp: 123,
                    attack: 77,
                    defense: 72,
                    special_attack: 99,
                    special_defense: 92,
                    speed: 58
                },
                abilities: Abilities {
                    first: "refrigerate".to_string(),
                    second: "none".to_string(),
                    hidden: "snowwarning".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Ice
                ),
                weight: 225 as f32,
            }
        );
        
        pokedex.insert(
            "sylveon".to_string(),
            PokedexPokemon {
                species: "sylveon".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 65,
                    defense: 65,
                    special_attack: 110,
                    special_defense: 130,
                    speed: 60
                },
                abilities: Abilities {
                    first: "cutecharm".to_string(),
                    second: "none".to_string(),
                    hidden: "pixilate".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 23.5 as f32,
            }
        );
        
        pokedex.insert(
            "hawlucha".to_string(),
            PokedexPokemon {
                species: "hawlucha".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 92,
                    defense: 75,
                    special_attack: 74,
                    special_defense: 63,
                    speed: 118
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "unburden".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Flying
                ),
                weight: 21.5 as f32,
            }
        );
        
        pokedex.insert(
            "dedenne".to_string(),
            PokedexPokemon {
                species: "dedenne".to_string(),
                base_stats: BaseStats {
                    hp: 67,
                    attack: 58,
                    defense: 57,
                    special_attack: 81,
                    special_defense: 67,
                    speed: 101
                },
                abilities: Abilities {
                    first: "cheekpouch".to_string(),
                    second: "pickup".to_string(),
                    hidden: "plus".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Fairy
                ),
                weight: 2.2 as f32,
            }
        );
        
        pokedex.insert(
            "carbink".to_string(),
            PokedexPokemon {
                species: "carbink".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 50,
                    defense: 150,
                    special_attack: 50,
                    special_defense: 150,
                    speed: 50
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "none".to_string(),
                    hidden: "sturdy".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Fairy
                ),
                weight: 5.7 as f32,
            }
        );
        
        pokedex.insert(
            "goomy".to_string(),
            PokedexPokemon {
                species: "goomy".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 50,
                    defense: 35,
                    special_attack: 55,
                    special_defense: 75,
                    speed: 40
                },
                abilities: Abilities {
                    first: "sapsipper".to_string(),
                    second: "hydration".to_string(),
                    hidden: "gooey".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 2.8 as f32,
            }
        );
        
        pokedex.insert(
            "sliggoo".to_string(),
            PokedexPokemon {
                species: "sliggoo".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 75,
                    defense: 53,
                    special_attack: 83,
                    special_defense: 113,
                    speed: 60
                },
                abilities: Abilities {
                    first: "sapsipper".to_string(),
                    second: "hydration".to_string(),
                    hidden: "gooey".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 17.5 as f32,
            }
        );
        
        pokedex.insert(
            "goodra".to_string(),
            PokedexPokemon {
                species: "goodra".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 100,
                    defense: 70,
                    special_attack: 110,
                    special_defense: 150,
                    speed: 80
                },
                abilities: Abilities {
                    first: "sapsipper".to_string(),
                    second: "hydration".to_string(),
                    hidden: "gooey".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 150.5 as f32,
            }
        );
        
        pokedex.insert(
            "klefki".to_string(),
            PokedexPokemon {
                species: "klefki".to_string(),
                base_stats: BaseStats {
                    hp: 57,
                    attack: 80,
                    defense: 91,
                    special_attack: 80,
                    special_defense: 87,
                    speed: 75
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "none".to_string(),
                    hidden: "magician".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Fairy
                ),
                weight: 3 as f32,
            }
        );
        
        pokedex.insert(
            "phantump".to_string(),
            PokedexPokemon {
                species: "phantump".to_string(),
                base_stats: BaseStats {
                    hp: 43,
                    attack: 70,
                    defense: 48,
                    special_attack: 50,
                    special_defense: 60,
                    speed: 38
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "frisk".to_string(),
                    hidden: "harvest".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 7 as f32,
            }
        );
        
        pokedex.insert(
            "trevenant".to_string(),
            PokedexPokemon {
                species: "trevenant".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 110,
                    defense: 76,
                    special_attack: 65,
                    special_defense: 82,
                    speed: 56
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "frisk".to_string(),
                    hidden: "harvest".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 71 as f32,
            }
        );
        
        pokedex.insert(
            "pumpkaboo".to_string(),
            PokedexPokemon {
                species: "pumpkaboo".to_string(),
                base_stats: BaseStats {
                    hp: 49,
                    attack: 66,
                    defense: 70,
                    special_attack: 44,
                    special_defense: 55,
                    speed: 51
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "frisk".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "pumpkaboosmall".to_string(),
            PokedexPokemon {
                species: "pumpkaboosmall".to_string(),
                base_stats: BaseStats {
                    hp: 44,
                    attack: 66,
                    defense: 70,
                    special_attack: 44,
                    special_defense: 55,
                    speed: 56
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "frisk".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 3.5 as f32,
            }
        );
        
        pokedex.insert(
            "pumpkaboolarge".to_string(),
            PokedexPokemon {
                species: "pumpkaboolarge".to_string(),
                base_stats: BaseStats {
                    hp: 54,
                    attack: 66,
                    defense: 70,
                    special_attack: 44,
                    special_defense: 55,
                    speed: 46
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "frisk".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 7.5 as f32,
            }
        );
        
        pokedex.insert(
            "pumpkaboosuper".to_string(),
            PokedexPokemon {
                species: "pumpkaboosuper".to_string(),
                base_stats: BaseStats {
                    hp: 59,
                    attack: 66,
                    defense: 70,
                    special_attack: 44,
                    special_defense: 55,
                    speed: 41
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "frisk".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "gourgeist".to_string(),
            PokedexPokemon {
                species: "gourgeist".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 90,
                    defense: 122,
                    special_attack: 58,
                    special_defense: 75,
                    speed: 84
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "frisk".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 12.5 as f32,
            }
        );
        
        pokedex.insert(
            "gourgeistsmall".to_string(),
            PokedexPokemon {
                species: "gourgeistsmall".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 85,
                    defense: 122,
                    special_attack: 58,
                    special_defense: 75,
                    speed: 99
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "frisk".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 9.5 as f32,
            }
        );
        
        pokedex.insert(
            "gourgeistlarge".to_string(),
            PokedexPokemon {
                species: "gourgeistlarge".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 95,
                    defense: 122,
                    special_attack: 58,
                    special_defense: 75,
                    speed: 69
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "frisk".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 14 as f32,
            }
        );
        
        pokedex.insert(
            "gourgeistsuper".to_string(),
            PokedexPokemon {
                species: "gourgeistsuper".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 100,
                    defense: 122,
                    special_attack: 58,
                    special_defense: 75,
                    speed: 54
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "frisk".to_string(),
                    hidden: "insomnia".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 39 as f32,
            }
        );
        
        pokedex.insert(
            "bergmite".to_string(),
            PokedexPokemon {
                species: "bergmite".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 69,
                    defense: 85,
                    special_attack: 32,
                    special_defense: 35,
                    speed: 28
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "icebody".to_string(),
                    hidden: "sturdy".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 99.5 as f32,
            }
        );
        
        pokedex.insert(
            "avalugg".to_string(),
            PokedexPokemon {
                species: "avalugg".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 117,
                    defense: 184,
                    special_attack: 44,
                    special_defense: 46,
                    speed: 28
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "icebody".to_string(),
                    hidden: "sturdy".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 505 as f32,
            }
        );
        
        pokedex.insert(
            "noibat".to_string(),
            PokedexPokemon {
                species: "noibat".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 30,
                    defense: 35,
                    special_attack: 45,
                    special_defense: 40,
                    speed: 55
                },
                abilities: Abilities {
                    first: "frisk".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Dragon
                ),
                weight: 8 as f32,
            }
        );
        
        pokedex.insert(
            "noivern".to_string(),
            PokedexPokemon {
                species: "noivern".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 70,
                    defense: 80,
                    special_attack: 97,
                    special_defense: 80,
                    speed: 123
                },
                abilities: Abilities {
                    first: "frisk".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Dragon
                ),
                weight: 85 as f32,
            }
        );
        
        pokedex.insert(
            "xerneas".to_string(),
            PokedexPokemon {
                species: "xerneas".to_string(),
                base_stats: BaseStats {
                    hp: 126,
                    attack: 131,
                    defense: 95,
                    special_attack: 131,
                    special_defense: 98,
                    speed: 99
                },
                abilities: Abilities {
                    first: "fairyaura".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 215 as f32,
            }
        );
        
        pokedex.insert(
            "yveltal".to_string(),
            PokedexPokemon {
                species: "yveltal".to_string(),
                base_stats: BaseStats {
                    hp: 126,
                    attack: 131,
                    defense: 95,
                    special_attack: 131,
                    special_defense: 98,
                    speed: 99
                },
                abilities: Abilities {
                    first: "darkaura".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Flying
                ),
                weight: 203 as f32,
            }
        );
        
        pokedex.insert(
            "zygarde".to_string(),
            PokedexPokemon {
                species: "zygarde".to_string(),
                base_stats: BaseStats {
                    hp: 108,
                    attack: 100,
                    defense: 121,
                    special_attack: 81,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "aurabreak".to_string(),
                    second: "powerconstruct".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ground
                ),
                weight: 305 as f32,
            }
        );
        
        pokedex.insert(
            "zygarde10".to_string(),
            PokedexPokemon {
                species: "zygarde10".to_string(),
                base_stats: BaseStats {
                    hp: 54,
                    attack: 100,
                    defense: 71,
                    special_attack: 61,
                    special_defense: 85,
                    speed: 115
                },
                abilities: Abilities {
                    first: "aurabreak".to_string(),
                    second: "powerconstruct".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ground
                ),
                weight: 33.5 as f32,
            }
        );
        
        pokedex.insert(
            "zygardecomplete".to_string(),
            PokedexPokemon {
                species: "zygardecomplete".to_string(),
                base_stats: BaseStats {
                    hp: 216,
                    attack: 100,
                    defense: 121,
                    special_attack: 91,
                    special_defense: 95,
                    speed: 85
                },
                abilities: Abilities {
                    first: "powerconstruct".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ground
                ),
                weight: 610 as f32,
            }
        );
        
        pokedex.insert(
            "diancie".to_string(),
            PokedexPokemon {
                species: "diancie".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 100,
                    defense: 150,
                    special_attack: 100,
                    special_defense: 150,
                    speed: 50
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Fairy
                ),
                weight: 8.8 as f32,
            }
        );
        
        pokedex.insert(
            "dianciemega".to_string(),
            PokedexPokemon {
                species: "dianciemega".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 160,
                    defense: 110,
                    special_attack: 160,
                    special_defense: 110,
                    speed: 110
                },
                abilities: Abilities {
                    first: "magicbounce".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Fairy
                ),
                weight: 27.8 as f32,
            }
        );
        
        pokedex.insert(
            "hoopa".to_string(),
            PokedexPokemon {
                species: "hoopa".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 110,
                    defense: 60,
                    special_attack: 150,
                    special_defense: 130,
                    speed: 70
                },
                abilities: Abilities {
                    first: "magician".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Ghost
                ),
                weight: 9 as f32,
            }
        );
        
        pokedex.insert(
            "hoopaunbound".to_string(),
            PokedexPokemon {
                species: "hoopaunbound".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 160,
                    defense: 60,
                    special_attack: 170,
                    special_defense: 130,
                    speed: 80
                },
                abilities: Abilities {
                    first: "magician".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Dark
                ),
                weight: 490 as f32,
            }
        );
        
        pokedex.insert(
            "volcanion".to_string(),
            PokedexPokemon {
                species: "volcanion".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 110,
                    defense: 120,
                    special_attack: 130,
                    special_defense: 90,
                    speed: 70
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Water
                ),
                weight: 195 as f32,
            }
        );
        
        pokedex.insert(
            "rowlet".to_string(),
            PokedexPokemon {
                species: "rowlet".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 55,
                    defense: 55,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 42
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "longreach".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Flying
                ),
                weight: 1.5 as f32,
            }
        );
        
        pokedex.insert(
            "dartrix".to_string(),
            PokedexPokemon {
                species: "dartrix".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 75,
                    defense: 75,
                    special_attack: 70,
                    special_defense: 70,
                    speed: 52
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "longreach".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Flying
                ),
                weight: 16 as f32,
            }
        );
        
        pokedex.insert(
            "decidueye".to_string(),
            PokedexPokemon {
                species: "decidueye".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 107,
                    defense: 75,
                    special_attack: 100,
                    special_defense: 100,
                    speed: 70
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "longreach".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Ghost
                ),
                weight: 36.6 as f32,
            }
        );
        
        pokedex.insert(
            "litten".to_string(),
            PokedexPokemon {
                species: "litten".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 65,
                    defense: 40,
                    special_attack: 60,
                    special_defense: 40,
                    speed: 70
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "intimidate".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 4.3 as f32,
            }
        );
        
        pokedex.insert(
            "torracat".to_string(),
            PokedexPokemon {
                species: "torracat".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 85,
                    defense: 50,
                    special_attack: 80,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "intimidate".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 25 as f32,
            }
        );
        
        pokedex.insert(
            "incineroar".to_string(),
            PokedexPokemon {
                species: "incineroar".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 115,
                    defense: 90,
                    special_attack: 80,
                    special_defense: 90,
                    speed: 60
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "intimidate".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Dark
                ),
                weight: 83 as f32,
            }
        );
        
        pokedex.insert(
            "popplio".to_string(),
            PokedexPokemon {
                species: "popplio".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 54,
                    defense: 54,
                    special_attack: 66,
                    special_defense: 56,
                    speed: 40
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "liquidvoice".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 7.5 as f32,
            }
        );
        
        pokedex.insert(
            "brionne".to_string(),
            PokedexPokemon {
                species: "brionne".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 69,
                    defense: 69,
                    special_attack: 91,
                    special_defense: 81,
                    speed: 50
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "liquidvoice".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 17.5 as f32,
            }
        );
        
        pokedex.insert(
            "primarina".to_string(),
            PokedexPokemon {
                species: "primarina".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 74,
                    defense: 74,
                    special_attack: 126,
                    special_defense: 116,
                    speed: 60
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "liquidvoice".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fairy
                ),
                weight: 44 as f32,
            }
        );
        
        pokedex.insert(
            "pikipek".to_string(),
            PokedexPokemon {
                species: "pikipek".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 75,
                    defense: 30,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 65
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "skilllink".to_string(),
                    hidden: "pickup".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 1.2 as f32,
            }
        );
        
        pokedex.insert(
            "trumbeak".to_string(),
            PokedexPokemon {
                species: "trumbeak".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 85,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 75
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "skilllink".to_string(),
                    hidden: "pickup".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 14.8 as f32,
            }
        );
        
        pokedex.insert(
            "toucannon".to_string(),
            PokedexPokemon {
                species: "toucannon".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 120,
                    defense: 75,
                    special_attack: 75,
                    special_defense: 75,
                    speed: 60
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "skilllink".to_string(),
                    hidden: "sheerforce".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Flying
                ),
                weight: 26 as f32,
            }
        );
        
        pokedex.insert(
            "yungoos".to_string(),
            PokedexPokemon {
                species: "yungoos".to_string(),
                base_stats: BaseStats {
                    hp: 48,
                    attack: 70,
                    defense: 30,
                    special_attack: 30,
                    special_defense: 30,
                    speed: 45
                },
                abilities: Abilities {
                    first: "stakeout".to_string(),
                    second: "strongjaw".to_string(),
                    hidden: "adaptability".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "gumshoos".to_string(),
            PokedexPokemon {
                species: "gumshoos".to_string(),
                base_stats: BaseStats {
                    hp: 88,
                    attack: 110,
                    defense: 60,
                    special_attack: 55,
                    special_defense: 60,
                    speed: 45
                },
                abilities: Abilities {
                    first: "stakeout".to_string(),
                    second: "strongjaw".to_string(),
                    hidden: "adaptability".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 14.2 as f32,
            }
        );
        
        pokedex.insert(
            "grubbin".to_string(),
            PokedexPokemon {
                species: "grubbin".to_string(),
                base_stats: BaseStats {
                    hp: 47,
                    attack: 62,
                    defense: 45,
                    special_attack: 55,
                    special_defense: 45,
                    speed: 46
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 4.4 as f32,
            }
        );
        
        pokedex.insert(
            "charjabug".to_string(),
            PokedexPokemon {
                species: "charjabug".to_string(),
                base_stats: BaseStats {
                    hp: 57,
                    attack: 82,
                    defense: 95,
                    special_attack: 55,
                    special_defense: 75,
                    speed: 36
                },
                abilities: Abilities {
                    first: "battery".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Electric
                ),
                weight: 10.5 as f32,
            }
        );
        
        pokedex.insert(
            "vikavolt".to_string(),
            PokedexPokemon {
                species: "vikavolt".to_string(),
                base_stats: BaseStats {
                    hp: 77,
                    attack: 70,
                    defense: 90,
                    special_attack: 145,
                    special_defense: 75,
                    speed: 43
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Electric
                ),
                weight: 45 as f32,
            }
        );
        
        pokedex.insert(
            "crabrawler".to_string(),
            PokedexPokemon {
                species: "crabrawler".to_string(),
                base_stats: BaseStats {
                    hp: 47,
                    attack: 82,
                    defense: 57,
                    special_attack: 42,
                    special_defense: 47,
                    speed: 63
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "ironfist".to_string(),
                    hidden: "angerpoint".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 7 as f32,
            }
        );
        
        pokedex.insert(
            "crabominable".to_string(),
            PokedexPokemon {
                species: "crabominable".to_string(),
                base_stats: BaseStats {
                    hp: 97,
                    attack: 132,
                    defense: 77,
                    special_attack: 62,
                    special_defense: 67,
                    speed: 43
                },
                abilities: Abilities {
                    first: "hypercutter".to_string(),
                    second: "ironfist".to_string(),
                    hidden: "angerpoint".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Ice
                ),
                weight: 180 as f32,
            }
        );
        
        pokedex.insert(
            "oricorio".to_string(),
            PokedexPokemon {
                species: "oricorio".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 70,
                    defense: 70,
                    special_attack: 98,
                    special_defense: 70,
                    speed: 93
                },
                abilities: Abilities {
                    first: "dancer".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Flying
                ),
                weight: 3.4 as f32,
            }
        );
        
        pokedex.insert(
            "oricoriopompom".to_string(),
            PokedexPokemon {
                species: "oricoriopompom".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 70,
                    defense: 70,
                    special_attack: 98,
                    special_defense: 70,
                    speed: 93
                },
                abilities: Abilities {
                    first: "dancer".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Flying
                ),
                weight: 3.4 as f32,
            }
        );
        
        pokedex.insert(
            "oricoriopau".to_string(),
            PokedexPokemon {
                species: "oricoriopau".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 70,
                    defense: 70,
                    special_attack: 98,
                    special_defense: 70,
                    speed: 93
                },
                abilities: Abilities {
                    first: "dancer".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Flying
                ),
                weight: 3.4 as f32,
            }
        );
        
        pokedex.insert(
            "oricoriosensu".to_string(),
            PokedexPokemon {
                species: "oricoriosensu".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 70,
                    defense: 70,
                    special_attack: 98,
                    special_defense: 70,
                    speed: 93
                },
                abilities: Abilities {
                    first: "dancer".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Flying
                ),
                weight: 3.4 as f32,
            }
        );
        
        pokedex.insert(
            "cutiefly".to_string(),
            PokedexPokemon {
                species: "cutiefly".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 40,
                    special_attack: 55,
                    special_defense: 40,
                    speed: 84
                },
                abilities: Abilities {
                    first: "honeygather".to_string(),
                    second: "shielddust".to_string(),
                    hidden: "sweetveil".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fairy
                ),
                weight: 0.2 as f32,
            }
        );
        
        pokedex.insert(
            "ribombee".to_string(),
            PokedexPokemon {
                species: "ribombee".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 55,
                    defense: 60,
                    special_attack: 95,
                    special_defense: 70,
                    speed: 124
                },
                abilities: Abilities {
                    first: "honeygather".to_string(),
                    second: "shielddust".to_string(),
                    hidden: "sweetveil".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fairy
                ),
                weight: 0.5 as f32,
            }
        );
        
        pokedex.insert(
            "rockruff".to_string(),
            PokedexPokemon {
                species: "rockruff".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 65,
                    defense: 40,
                    special_attack: 30,
                    special_defense: 40,
                    speed: 60
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "vitalspirit".to_string(),
                    hidden: "steadfast".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 9.2 as f32,
            }
        );
        
        pokedex.insert(
            "lycanroc".to_string(),
            PokedexPokemon {
                species: "lycanroc".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 115,
                    defense: 65,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 112
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "sandrush".to_string(),
                    hidden: "steadfast".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 25 as f32,
            }
        );
        
        pokedex.insert(
            "lycanrocmidnight".to_string(),
            PokedexPokemon {
                species: "lycanrocmidnight".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 115,
                    defense: 75,
                    special_attack: 55,
                    special_defense: 75,
                    speed: 82
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "vitalspirit".to_string(),
                    hidden: "noguard".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 25 as f32,
            }
        );
        
        pokedex.insert(
            "lycanrocdusk".to_string(),
            PokedexPokemon {
                species: "lycanrocdusk".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 117,
                    defense: 65,
                    special_attack: 55,
                    special_defense: 65,
                    speed: 110
                },
                abilities: Abilities {
                    first: "toughclaws".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 25 as f32,
            }
        );
        
        pokedex.insert(
            "wishiwashi".to_string(),
            PokedexPokemon {
                species: "wishiwashi".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 20,
                    defense: 20,
                    special_attack: 25,
                    special_defense: 25,
                    speed: 40
                },
                abilities: Abilities {
                    first: "schooling".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "wishiwashischool".to_string(),
            PokedexPokemon {
                species: "wishiwashischool".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 140,
                    defense: 130,
                    special_attack: 140,
                    special_defense: 135,
                    speed: 30
                },
                abilities: Abilities {
                    first: "schooling".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 78.6 as f32,
            }
        );
        
        pokedex.insert(
            "mareanie".to_string(),
            PokedexPokemon {
                species: "mareanie".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 53,
                    defense: 62,
                    special_attack: 43,
                    special_defense: 52,
                    speed: 45
                },
                abilities: Abilities {
                    first: "merciless".to_string(),
                    second: "limber".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Water
                ),
                weight: 8 as f32,
            }
        );
        
        pokedex.insert(
            "toxapex".to_string(),
            PokedexPokemon {
                species: "toxapex".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 63,
                    defense: 152,
                    special_attack: 53,
                    special_defense: 142,
                    speed: 35
                },
                abilities: Abilities {
                    first: "merciless".to_string(),
                    second: "limber".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Water
                ),
                weight: 14.5 as f32,
            }
        );
        
        pokedex.insert(
            "mudbray".to_string(),
            PokedexPokemon {
                species: "mudbray".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 100,
                    defense: 70,
                    special_attack: 45,
                    special_defense: 55,
                    speed: 45
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "stamina".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 110 as f32,
            }
        );
        
        pokedex.insert(
            "mudsdale".to_string(),
            PokedexPokemon {
                species: "mudsdale".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 125,
                    defense: 100,
                    special_attack: 55,
                    special_defense: 85,
                    speed: 35
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "stamina".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 920 as f32,
            }
        );
        
        pokedex.insert(
            "dewpider".to_string(),
            PokedexPokemon {
                species: "dewpider".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 40,
                    defense: 52,
                    special_attack: 40,
                    special_defense: 72,
                    speed: 27
                },
                abilities: Abilities {
                    first: "waterbubble".to_string(),
                    second: "none".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Bug
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "araquanid".to_string(),
            PokedexPokemon {
                species: "araquanid".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 70,
                    defense: 92,
                    special_attack: 50,
                    special_defense: 132,
                    speed: 42
                },
                abilities: Abilities {
                    first: "waterbubble".to_string(),
                    second: "none".to_string(),
                    hidden: "waterabsorb".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Bug
                ),
                weight: 82 as f32,
            }
        );
        
        pokedex.insert(
            "fomantis".to_string(),
            PokedexPokemon {
                species: "fomantis".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 55,
                    defense: 35,
                    special_attack: 50,
                    special_defense: 35,
                    speed: 35
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "none".to_string(),
                    hidden: "contrary".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 1.5 as f32,
            }
        );
        
        pokedex.insert(
            "lurantis".to_string(),
            PokedexPokemon {
                species: "lurantis".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 105,
                    defense: 90,
                    special_attack: 80,
                    special_defense: 90,
                    speed: 45
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "none".to_string(),
                    hidden: "contrary".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 18.5 as f32,
            }
        );
        
        pokedex.insert(
            "morelull".to_string(),
            PokedexPokemon {
                species: "morelull".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 35,
                    defense: 55,
                    special_attack: 65,
                    special_defense: 75,
                    speed: 15
                },
                abilities: Abilities {
                    first: "illuminate".to_string(),
                    second: "effectspore".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Fairy
                ),
                weight: 1.5 as f32,
            }
        );
        
        pokedex.insert(
            "shiinotic".to_string(),
            PokedexPokemon {
                species: "shiinotic".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 45,
                    defense: 80,
                    special_attack: 90,
                    special_defense: 100,
                    speed: 30
                },
                abilities: Abilities {
                    first: "illuminate".to_string(),
                    second: "effectspore".to_string(),
                    hidden: "raindish".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Fairy
                ),
                weight: 11.5 as f32,
            }
        );
        
        pokedex.insert(
            "salandit".to_string(),
            PokedexPokemon {
                species: "salandit".to_string(),
                base_stats: BaseStats {
                    hp: 48,
                    attack: 44,
                    defense: 40,
                    special_attack: 71,
                    special_defense: 40,
                    speed: 77
                },
                abilities: Abilities {
                    first: "corrosion".to_string(),
                    second: "none".to_string(),
                    hidden: "oblivious".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Fire
                ),
                weight: 4.8 as f32,
            }
        );
        
        pokedex.insert(
            "salazzle".to_string(),
            PokedexPokemon {
                species: "salazzle".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 64,
                    defense: 60,
                    special_attack: 111,
                    special_defense: 60,
                    speed: 117
                },
                abilities: Abilities {
                    first: "corrosion".to_string(),
                    second: "none".to_string(),
                    hidden: "oblivious".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Fire
                ),
                weight: 22.2 as f32,
            }
        );
        
        pokedex.insert(
            "stufful".to_string(),
            PokedexPokemon {
                species: "stufful".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 75,
                    defense: 50,
                    special_attack: 45,
                    special_defense: 50,
                    speed: 50
                },
                abilities: Abilities {
                    first: "fluffy".to_string(),
                    second: "klutz".to_string(),
                    hidden: "cutecharm".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fighting
                ),
                weight: 6.8 as f32,
            }
        );
        
        pokedex.insert(
            "bewear".to_string(),
            PokedexPokemon {
                species: "bewear".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 125,
                    defense: 80,
                    special_attack: 55,
                    special_defense: 60,
                    speed: 60
                },
                abilities: Abilities {
                    first: "fluffy".to_string(),
                    second: "klutz".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Fighting
                ),
                weight: 135 as f32,
            }
        );
        
        pokedex.insert(
            "bounsweet".to_string(),
            PokedexPokemon {
                species: "bounsweet".to_string(),
                base_stats: BaseStats {
                    hp: 42,
                    attack: 30,
                    defense: 38,
                    special_attack: 30,
                    special_defense: 38,
                    speed: 32
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "oblivious".to_string(),
                    hidden: "sweetveil".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 3.2 as f32,
            }
        );
        
        pokedex.insert(
            "steenee".to_string(),
            PokedexPokemon {
                species: "steenee".to_string(),
                base_stats: BaseStats {
                    hp: 52,
                    attack: 40,
                    defense: 48,
                    special_attack: 40,
                    special_defense: 48,
                    speed: 62
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "oblivious".to_string(),
                    hidden: "sweetveil".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 8.2 as f32,
            }
        );
        
        pokedex.insert(
            "tsareena".to_string(),
            PokedexPokemon {
                species: "tsareena".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 120,
                    defense: 98,
                    special_attack: 50,
                    special_defense: 98,
                    speed: 72
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "queenlymajesty".to_string(),
                    hidden: "sweetveil".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 21.4 as f32,
            }
        );
        
        pokedex.insert(
            "comfey".to_string(),
            PokedexPokemon {
                species: "comfey".to_string(),
                base_stats: BaseStats {
                    hp: 51,
                    attack: 52,
                    defense: 90,
                    special_attack: 82,
                    special_defense: 110,
                    speed: 100
                },
                abilities: Abilities {
                    first: "flowerveil".to_string(),
                    second: "triage".to_string(),
                    hidden: "naturalcure".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "oranguru".to_string(),
            PokedexPokemon {
                species: "oranguru".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 60,
                    defense: 80,
                    special_attack: 90,
                    special_defense: 110,
                    speed: 60
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "telepathy".to_string(),
                    hidden: "symbiosis".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Psychic
                ),
                weight: 76 as f32,
            }
        );
        
        pokedex.insert(
            "passimian".to_string(),
            PokedexPokemon {
                species: "passimian".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 120,
                    defense: 90,
                    special_attack: 40,
                    special_defense: 60,
                    speed: 80
                },
                abilities: Abilities {
                    first: "receiver".to_string(),
                    second: "none".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 82.8 as f32,
            }
        );
        
        pokedex.insert(
            "wimpod".to_string(),
            PokedexPokemon {
                species: "wimpod".to_string(),
                base_stats: BaseStats {
                    hp: 25,
                    attack: 35,
                    defense: 40,
                    special_attack: 20,
                    special_defense: 30,
                    speed: 80
                },
                abilities: Abilities {
                    first: "wimpout".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Water
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "golisopod".to_string(),
            PokedexPokemon {
                species: "golisopod".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 125,
                    defense: 140,
                    special_attack: 60,
                    special_defense: 90,
                    speed: 40
                },
                abilities: Abilities {
                    first: "emergencyexit".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Water
                ),
                weight: 108 as f32,
            }
        );
        
        pokedex.insert(
            "sandygast".to_string(),
            PokedexPokemon {
                species: "sandygast".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 55,
                    defense: 80,
                    special_attack: 70,
                    special_defense: 45,
                    speed: 15
                },
                abilities: Abilities {
                    first: "watercompaction".to_string(),
                    second: "none".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Ground
                ),
                weight: 70 as f32,
            }
        );
        
        pokedex.insert(
            "palossand".to_string(),
            PokedexPokemon {
                species: "palossand".to_string(),
                base_stats: BaseStats {
                    hp: 85,
                    attack: 75,
                    defense: 110,
                    special_attack: 100,
                    special_defense: 75,
                    speed: 35
                },
                abilities: Abilities {
                    first: "watercompaction".to_string(),
                    second: "none".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Ground
                ),
                weight: 250 as f32,
            }
        );
        
        pokedex.insert(
            "pyukumuku".to_string(),
            PokedexPokemon {
                species: "pyukumuku".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 60,
                    defense: 130,
                    special_attack: 30,
                    special_defense: 130,
                    speed: 5
                },
                abilities: Abilities {
                    first: "innardsout".to_string(),
                    second: "none".to_string(),
                    hidden: "unaware".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 1.2 as f32,
            }
        );
        
        pokedex.insert(
            "typenull".to_string(),
            PokedexPokemon {
                species: "typenull".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 59
                },
                abilities: Abilities {
                    first: "battlearmor".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 120.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvally".to_string(),
            PokedexPokemon {
                species: "silvally".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallybug".to_string(),
            PokedexPokemon {
                species: "silvallybug".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallydark".to_string(),
            PokedexPokemon {
                species: "silvallydark".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallydragon".to_string(),
            PokedexPokemon {
                species: "silvallydragon".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyelectric".to_string(),
            PokedexPokemon {
                species: "silvallyelectric".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyfairy".to_string(),
            PokedexPokemon {
                species: "silvallyfairy".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyfighting".to_string(),
            PokedexPokemon {
                species: "silvallyfighting".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyfire".to_string(),
            PokedexPokemon {
                species: "silvallyfire".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyflying".to_string(),
            PokedexPokemon {
                species: "silvallyflying".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyghost".to_string(),
            PokedexPokemon {
                species: "silvallyghost".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallygrass".to_string(),
            PokedexPokemon {
                species: "silvallygrass".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyground".to_string(),
            PokedexPokemon {
                species: "silvallyground".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyice".to_string(),
            PokedexPokemon {
                species: "silvallyice".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallypoison".to_string(),
            PokedexPokemon {
                species: "silvallypoison".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallypsychic".to_string(),
            PokedexPokemon {
                species: "silvallypsychic".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallyrock".to_string(),
            PokedexPokemon {
                species: "silvallyrock".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallysteel".to_string(),
            PokedexPokemon {
                species: "silvallysteel".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "silvallywater".to_string(),
            PokedexPokemon {
                species: "silvallywater".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 95,
                    defense: 95,
                    special_attack: 95,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rkssystem".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 100.5 as f32,
            }
        );
        
        pokedex.insert(
            "minior".to_string(),
            PokedexPokemon {
                species: "minior".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 100,
                    defense: 60,
                    special_attack: 100,
                    special_defense: 60,
                    speed: 120
                },
                abilities: Abilities {
                    first: "shieldsdown".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Flying
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "miniormeteor".to_string(),
            PokedexPokemon {
                species: "miniormeteor".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 60,
                    defense: 100,
                    special_attack: 60,
                    special_defense: 100,
                    speed: 60
                },
                abilities: Abilities {
                    first: "shieldsdown".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Flying
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "komala".to_string(),
            PokedexPokemon {
                species: "komala".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 115,
                    defense: 65,
                    special_attack: 75,
                    special_defense: 95,
                    speed: 65
                },
                abilities: Abilities {
                    first: "comatose".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 19.9 as f32,
            }
        );
        
        pokedex.insert(
            "turtonator".to_string(),
            PokedexPokemon {
                species: "turtonator".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 78,
                    defense: 135,
                    special_attack: 91,
                    special_defense: 85,
                    speed: 36
                },
                abilities: Abilities {
                    first: "shellarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Dragon
                ),
                weight: 212 as f32,
            }
        );
        
        pokedex.insert(
            "togedemaru".to_string(),
            PokedexPokemon {
                species: "togedemaru".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 98,
                    defense: 63,
                    special_attack: 40,
                    special_defense: 73,
                    speed: 96
                },
                abilities: Abilities {
                    first: "ironbarbs".to_string(),
                    second: "lightningrod".to_string(),
                    hidden: "sturdy".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Steel
                ),
                weight: 3.3 as f32,
            }
        );
        
        pokedex.insert(
            "mimikyu".to_string(),
            PokedexPokemon {
                species: "mimikyu".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 90,
                    defense: 80,
                    special_attack: 50,
                    special_defense: 105,
                    speed: 96
                },
                abilities: Abilities {
                    first: "disguise".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Fairy
                ),
                weight: 0.7 as f32,
            }
        );
        
        pokedex.insert(
            "mimikyubusted".to_string(),
            PokedexPokemon {
                species: "mimikyubusted".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 90,
                    defense: 80,
                    special_attack: 50,
                    special_defense: 105,
                    speed: 96
                },
                abilities: Abilities {
                    first: "disguise".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Fairy
                ),
                weight: 0.7 as f32,
            }
        );
        
        pokedex.insert(
            "bruxish".to_string(),
            PokedexPokemon {
                species: "bruxish".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 105,
                    defense: 70,
                    special_attack: 70,
                    special_defense: 70,
                    speed: 92
                },
                abilities: Abilities {
                    first: "dazzling".to_string(),
                    second: "strongjaw".to_string(),
                    hidden: "wonderskin".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Psychic
                ),
                weight: 19 as f32,
            }
        );
        
        pokedex.insert(
            "drampa".to_string(),
            PokedexPokemon {
                species: "drampa".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 60,
                    defense: 85,
                    special_attack: 135,
                    special_defense: 91,
                    speed: 36
                },
                abilities: Abilities {
                    first: "berserk".to_string(),
                    second: "sapsipper".to_string(),
                    hidden: "cloudnine".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Dragon
                ),
                weight: 185 as f32,
            }
        );
        
        pokedex.insert(
            "dhelmise".to_string(),
            PokedexPokemon {
                species: "dhelmise".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 131,
                    defense: 100,
                    special_attack: 86,
                    special_defense: 90,
                    speed: 40
                },
                abilities: Abilities {
                    first: "steelworker".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Grass
                ),
                weight: 210 as f32,
            }
        );
        
        pokedex.insert(
            "jangmoo".to_string(),
            PokedexPokemon {
                species: "jangmoo".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 55,
                    defense: 65,
                    special_attack: 45,
                    special_defense: 45,
                    speed: 45
                },
                abilities: Abilities {
                    first: "bulletproof".to_string(),
                    second: "soundproof".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 29.7 as f32,
            }
        );
        
        pokedex.insert(
            "hakamoo".to_string(),
            PokedexPokemon {
                species: "hakamoo".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 75,
                    defense: 90,
                    special_attack: 65,
                    special_defense: 70,
                    speed: 65
                },
                abilities: Abilities {
                    first: "bulletproof".to_string(),
                    second: "soundproof".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Fighting
                ),
                weight: 47 as f32,
            }
        );
        
        pokedex.insert(
            "kommoo".to_string(),
            PokedexPokemon {
                species: "kommoo".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 110,
                    defense: 125,
                    special_attack: 100,
                    special_defense: 105,
                    speed: 85
                },
                abilities: Abilities {
                    first: "bulletproof".to_string(),
                    second: "soundproof".to_string(),
                    hidden: "overcoat".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Fighting
                ),
                weight: 78.2 as f32,
            }
        );
        
        pokedex.insert(
            "tapukoko".to_string(),
            PokedexPokemon {
                species: "tapukoko".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 115,
                    defense: 85,
                    special_attack: 95,
                    special_defense: 75,
                    speed: 130
                },
                abilities: Abilities {
                    first: "electricsurge".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Fairy
                ),
                weight: 20.5 as f32,
            }
        );
        
        pokedex.insert(
            "tapulele".to_string(),
            PokedexPokemon {
                species: "tapulele".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 75,
                    special_attack: 130,
                    special_defense: 115,
                    speed: 95
                },
                abilities: Abilities {
                    first: "psychicsurge".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 18.6 as f32,
            }
        );
        
        pokedex.insert(
            "tapubulu".to_string(),
            PokedexPokemon {
                species: "tapubulu".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 130,
                    defense: 115,
                    special_attack: 85,
                    special_defense: 95,
                    speed: 75
                },
                abilities: Abilities {
                    first: "grassysurge".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Fairy
                ),
                weight: 45.5 as f32,
            }
        );
        
        pokedex.insert(
            "tapufini".to_string(),
            PokedexPokemon {
                species: "tapufini".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 75,
                    defense: 115,
                    special_attack: 95,
                    special_defense: 130,
                    speed: 85
                },
                abilities: Abilities {
                    first: "mistysurge".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fairy
                ),
                weight: 21.2 as f32,
            }
        );
        
        pokedex.insert(
            "cosmog".to_string(),
            PokedexPokemon {
                species: "cosmog".to_string(),
                base_stats: BaseStats {
                    hp: 43,
                    attack: 29,
                    defense: 31,
                    special_attack: 29,
                    special_defense: 31,
                    speed: 37
                },
                abilities: Abilities {
                    first: "unaware".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 0.1 as f32,
            }
        );
        
        pokedex.insert(
            "cosmoem".to_string(),
            PokedexPokemon {
                species: "cosmoem".to_string(),
                base_stats: BaseStats {
                    hp: 43,
                    attack: 29,
                    defense: 131,
                    special_attack: 29,
                    special_defense: 131,
                    speed: 37
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 999.9 as f32,
            }
        );
        
        pokedex.insert(
            "solgaleo".to_string(),
            PokedexPokemon {
                species: "solgaleo".to_string(),
                base_stats: BaseStats {
                    hp: 137,
                    attack: 137,
                    defense: 107,
                    special_attack: 113,
                    special_defense: 89,
                    speed: 97
                },
                abilities: Abilities {
                    first: "fullmetalbody".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Steel
                ),
                weight: 230 as f32,
            }
        );
        
        pokedex.insert(
            "lunala".to_string(),
            PokedexPokemon {
                species: "lunala".to_string(),
                base_stats: BaseStats {
                    hp: 137,
                    attack: 113,
                    defense: 89,
                    special_attack: 137,
                    special_defense: 107,
                    speed: 97
                },
                abilities: Abilities {
                    first: "shadowshield".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Ghost
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "nihilego".to_string(),
            PokedexPokemon {
                species: "nihilego".to_string(),
                base_stats: BaseStats {
                    hp: 109,
                    attack: 53,
                    defense: 47,
                    special_attack: 127,
                    special_defense: 131,
                    speed: 103
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Poison
                ),
                weight: 55.5 as f32,
            }
        );
        
        pokedex.insert(
            "buzzwole".to_string(),
            PokedexPokemon {
                species: "buzzwole".to_string(),
                base_stats: BaseStats {
                    hp: 107,
                    attack: 139,
                    defense: 139,
                    special_attack: 53,
                    special_defense: 53,
                    speed: 79
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fighting
                ),
                weight: 333.6 as f32,
            }
        );
        
        pokedex.insert(
            "pheromosa".to_string(),
            PokedexPokemon {
                species: "pheromosa".to_string(),
                base_stats: BaseStats {
                    hp: 71,
                    attack: 137,
                    defense: 37,
                    special_attack: 137,
                    special_defense: 37,
                    speed: 151
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fighting
                ),
                weight: 25 as f32,
            }
        );
        
        pokedex.insert(
            "xurkitree".to_string(),
            PokedexPokemon {
                species: "xurkitree".to_string(),
                base_stats: BaseStats {
                    hp: 83,
                    attack: 89,
                    defense: 71,
                    special_attack: 173,
                    special_defense: 71,
                    speed: 83
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 100 as f32,
            }
        );
        
        pokedex.insert(
            "celesteela".to_string(),
            PokedexPokemon {
                species: "celesteela".to_string(),
                base_stats: BaseStats {
                    hp: 97,
                    attack: 101,
                    defense: 103,
                    special_attack: 107,
                    special_defense: 101,
                    speed: 61
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Flying
                ),
                weight: 999.9 as f32,
            }
        );
        
        pokedex.insert(
            "kartana".to_string(),
            PokedexPokemon {
                species: "kartana".to_string(),
                base_stats: BaseStats {
                    hp: 59,
                    attack: 181,
                    defense: 131,
                    special_attack: 59,
                    special_defense: 31,
                    speed: 109
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Steel
                ),
                weight: 0.1 as f32,
            }
        );
        
        pokedex.insert(
            "guzzlord".to_string(),
            PokedexPokemon {
                species: "guzzlord".to_string(),
                base_stats: BaseStats {
                    hp: 223,
                    attack: 101,
                    defense: 53,
                    special_attack: 97,
                    special_defense: 53,
                    speed: 43
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Dragon
                ),
                weight: 888 as f32,
            }
        );
        
        pokedex.insert(
            "necrozma".to_string(),
            PokedexPokemon {
                species: "necrozma".to_string(),
                base_stats: BaseStats {
                    hp: 97,
                    attack: 107,
                    defense: 101,
                    special_attack: 127,
                    special_defense: 89,
                    speed: 79
                },
                abilities: Abilities {
                    first: "prismarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 230 as f32,
            }
        );
        
        pokedex.insert(
            "magearna".to_string(),
            PokedexPokemon {
                species: "magearna".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 95,
                    defense: 115,
                    special_attack: 130,
                    special_defense: 115,
                    speed: 65
                },
                abilities: Abilities {
                    first: "soulheart".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Fairy
                ),
                weight: 80.5 as f32,
            }
        );
        
        pokedex.insert(
            "marshadow".to_string(),
            PokedexPokemon {
                species: "marshadow".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 125,
                    defense: 80,
                    special_attack: 90,
                    special_defense: 90,
                    speed: 125
                },
                abilities: Abilities {
                    first: "technician".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Ghost
                ),
                weight: 22.2 as f32,
            }
        );
        
        pokedex.insert(
            "missingno".to_string(),
            PokedexPokemon {
                species: "missingno".to_string(),
                base_stats: BaseStats {
                    hp: 33,
                    attack: 136,
                    defense: 0,
                    special_attack: 6,
                    special_defense: 6,
                    speed: 29
                },
                abilities: Abilities {
                    first: "".to_string(),
                    second: "none".to_string(),
                    hidden: "".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 1590.8 as f32,
            }
        );
        
        pokedex.insert(
            "tomohawk".to_string(),
            PokedexPokemon {
                species: "tomohawk".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 60,
                    defense: 90,
                    special_attack: 115,
                    special_defense: 80,
                    speed: 85
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "prankster".to_string(),
                    hidden: "justified".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Fighting
                ),
                weight: 37.2 as f32,
            }
        );
        
        pokedex.insert(
            "necturna".to_string(),
            PokedexPokemon {
                species: "necturna".to_string(),
                base_stats: BaseStats {
                    hp: 64,
                    attack: 120,
                    defense: 100,
                    special_attack: 85,
                    special_defense: 120,
                    speed: 81
                },
                abilities: Abilities {
                    first: "forewarn".to_string(),
                    second: "none".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Ghost
                ),
                weight: 49.6 as f32,
            }
        );
        
        pokedex.insert(
            "mollux".to_string(),
            PokedexPokemon {
                species: "mollux".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 45,
                    defense: 83,
                    special_attack: 131,
                    special_defense: 105,
                    speed: 76
                },
                abilities: Abilities {
                    first: "dryskin".to_string(),
                    second: "none".to_string(),
                    hidden: "illuminate".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Poison
                ),
                weight: 41 as f32,
            }
        );
        
        pokedex.insert(
            "aurumoth".to_string(),
            PokedexPokemon {
                species: "aurumoth".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 120,
                    defense: 99,
                    special_attack: 117,
                    special_defense: 60,
                    speed: 94
                },
                abilities: Abilities {
                    first: "weakarmor".to_string(),
                    second: "noguard".to_string(),
                    hidden: "illusion".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Psychic
                ),
                weight: 193 as f32,
            }
        );
        
        pokedex.insert(
            "malaconda".to_string(),
            PokedexPokemon {
                species: "malaconda".to_string(),
                base_stats: BaseStats {
                    hp: 115,
                    attack: 100,
                    defense: 60,
                    special_attack: 40,
                    special_defense: 130,
                    speed: 55
                },
                abilities: Abilities {
                    first: "harvest".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Grass
                ),
                weight: 108.8 as f32,
            }
        );
        
        pokedex.insert(
            "cawmodore".to_string(),
            PokedexPokemon {
                species: "cawmodore".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 92,
                    defense: 130,
                    special_attack: 65,
                    special_defense: 75,
                    speed: 118
                },
                abilities: Abilities {
                    first: "intimidate".to_string(),
                    second: "voltabsorb".to_string(),
                    hidden: "bigpecks".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Flying
                ),
                weight: 37 as f32,
            }
        );
        
        pokedex.insert(
            "volkraken".to_string(),
            PokedexPokemon {
                species: "volkraken".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 45,
                    defense: 80,
                    special_attack: 135,
                    special_defense: 100,
                    speed: 95
                },
                abilities: Abilities {
                    first: "analytic".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "pressure".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fire
                ),
                weight: 44.5 as f32,
            }
        );
        
        pokedex.insert(
            "plasmanta".to_string(),
            PokedexPokemon {
                species: "plasmanta".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 57,
                    defense: 119,
                    special_attack: 131,
                    special_defense: 98,
                    speed: 100
                },
                abilities: Abilities {
                    first: "stormdrain".to_string(),
                    second: "vitalspirit".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Poison
                ),
                weight: 460 as f32,
            }
        );
        
        pokedex.insert(
            "naviathan".to_string(),
            PokedexPokemon {
                species: "naviathan".to_string(),
                base_stats: BaseStats {
                    hp: 103,
                    attack: 110,
                    defense: 90,
                    special_attack: 95,
                    special_defense: 65,
                    speed: 97
                },
                abilities: Abilities {
                    first: "waterveil".to_string(),
                    second: "heatproof".to_string(),
                    hidden: "lightmetal".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Steel
                ),
                weight: 510 as f32,
            }
        );
        
        pokedex.insert(
            "crucibelle".to_string(),
            PokedexPokemon {
                species: "crucibelle".to_string(),
                base_stats: BaseStats {
                    hp: 106,
                    attack: 105,
                    defense: 65,
                    special_attack: 75,
                    special_defense: 85,
                    speed: 104
                },
                abilities: Abilities {
                    first: "regenerator".to_string(),
                    second: "moldbreaker".to_string(),
                    hidden: "liquidooze".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Poison
                ),
                weight: 23.6 as f32,
            }
        );
        
        pokedex.insert(
            "crucibellemega".to_string(),
            PokedexPokemon {
                species: "crucibellemega".to_string(),
                base_stats: BaseStats {
                    hp: 106,
                    attack: 135,
                    defense: 75,
                    special_attack: 85,
                    special_defense: 125,
                    speed: 114
                },
                abilities: Abilities {
                    first: "magicguard".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Poison
                ),
                weight: 22.5 as f32,
            }
        );
        
        pokedex.insert(
            "kerfluffle".to_string(),
            PokedexPokemon {
                species: "kerfluffle".to_string(),
                base_stats: BaseStats {
                    hp: 84,
                    attack: 78,
                    defense: 86,
                    special_attack: 115,
                    special_defense: 88,
                    speed: 119
                },
                abilities: Abilities {
                    first: "naturalcure".to_string(),
                    second: "aromaveil".to_string(),
                    hidden: "friendguard".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Fighting
                ),
                weight: 24.2 as f32,
            }
        );
        
        pokedex.insert(
            "syclant".to_string(),
            PokedexPokemon {
                species: "syclant".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 116,
                    defense: 70,
                    special_attack: 114,
                    special_defense: 64,
                    speed: 121
                },
                abilities: Abilities {
                    first: "compoundeyes".to_string(),
                    second: "mountaineer".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Bug
                ),
                weight: 52 as f32,
            }
        );
        
        pokedex.insert(
            "revenankh".to_string(),
            PokedexPokemon {
                species: "revenankh".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 105,
                    defense: 90,
                    special_attack: 65,
                    special_defense: 110,
                    speed: 65
                },
                abilities: Abilities {
                    first: "shedskin".to_string(),
                    second: "airlock".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Fighting
                ),
                weight: 44 as f32,
            }
        );
        
        pokedex.insert(
            "pyroak".to_string(),
            PokedexPokemon {
                species: "pyroak".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 70,
                    defense: 105,
                    special_attack: 95,
                    special_defense: 90,
                    speed: 60
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "battlearmor".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Grass
                ),
                weight: 168 as f32,
            }
        );
        
        pokedex.insert(
            "fidgit".to_string(),
            PokedexPokemon {
                species: "fidgit".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 76,
                    defense: 109,
                    special_attack: 90,
                    special_defense: 80,
                    speed: 105
                },
                abilities: Abilities {
                    first: "persistent".to_string(),
                    second: "vitalspirit".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Ground
                ),
                weight: 53 as f32,
            }
        );
        
        pokedex.insert(
            "stratagem".to_string(),
            PokedexPokemon {
                species: "stratagem".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 60,
                    defense: 65,
                    special_attack: 120,
                    special_defense: 70,
                    speed: 130
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "technician".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 45 as f32,
            }
        );
        
        pokedex.insert(
            "arghonaut".to_string(),
            PokedexPokemon {
                species: "arghonaut".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 110,
                    defense: 95,
                    special_attack: 70,
                    special_defense: 100,
                    speed: 75
                },
                abilities: Abilities {
                    first: "unaware".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Fighting
                ),
                weight: 151 as f32,
            }
        );
        
        pokedex.insert(
            "kitsunoh".to_string(),
            PokedexPokemon {
                species: "kitsunoh".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 103,
                    defense: 85,
                    special_attack: 55,
                    special_defense: 80,
                    speed: 110
                },
                abilities: Abilities {
                    first: "frisk".to_string(),
                    second: "limber".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Ghost
                ),
                weight: 51 as f32,
            }
        );
        
        pokedex.insert(
            "cyclohm".to_string(),
            PokedexPokemon {
                species: "cyclohm".to_string(),
                base_stats: BaseStats {
                    hp: 108,
                    attack: 60,
                    defense: 118,
                    special_attack: 112,
                    special_defense: 70,
                    speed: 80
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "static".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Dragon
                ),
                weight: 59 as f32,
            }
        );
        
        pokedex.insert(
            "colossoil".to_string(),
            PokedexPokemon {
                species: "colossoil".to_string(),
                base_stats: BaseStats {
                    hp: 133,
                    attack: 122,
                    defense: 72,
                    special_attack: 71,
                    special_defense: 72,
                    speed: 95
                },
                abilities: Abilities {
                    first: "rebound".to_string(),
                    second: "guts".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Ground
                ),
                weight: 683.6 as f32,
            }
        );
        
        pokedex.insert(
            "krilowatt".to_string(),
            PokedexPokemon {
                species: "krilowatt".to_string(),
                base_stats: BaseStats {
                    hp: 151,
                    attack: 84,
                    defense: 73,
                    special_attack: 83,
                    special_defense: 74,
                    speed: 105
                },
                abilities: Abilities {
                    first: "trace".to_string(),
                    second: "magicguard".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Water
                ),
                weight: 10.6 as f32,
            }
        );
        
        pokedex.insert(
            "voodoom".to_string(),
            PokedexPokemon {
                species: "voodoom".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 85,
                    defense: 80,
                    special_attack: 105,
                    special_defense: 80,
                    speed: 110
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "lightningrod".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Dark
                ),
                weight: 75.5 as f32,
            }
        );
        
        pokedex.insert(
            "basculinbluestripe".to_string(),
            PokedexPokemon {
                species: "basculinbluestripe".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 92,
                    defense: 65,
                    special_attack: 80,
                    special_defense: 55,
                    speed: 98
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "adaptability".to_string(),
                    hidden: "moldbreaker".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "zeraora".to_string(),
            PokedexPokemon {
                species: "zeraora".to_string(),
                base_stats: BaseStats {
                    hp: 88,
                    attack: 112,
                    defense: 75,
                    special_attack: 102,
                    special_defense: 80,
                    speed: 143
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 44.5 as f32,
            }
        );
        
        pokedex.insert(
            "naganadel".to_string(),
            PokedexPokemon {
                species: "naganadel".to_string(),
                base_stats: BaseStats {
                    hp: 73,
                    attack: 73,
                    defense: 73,
                    special_attack: 127,
                    special_defense: 73,
                    speed: 121
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Dragon
                ),
                weight: 150 as f32,
            }
        );
        
        pokedex.insert(
            "stakataka".to_string(),
            PokedexPokemon {
                species: "stakataka".to_string(),
                base_stats: BaseStats {
                    hp: 61,
                    attack: 131,
                    defense: 211,
                    special_attack: 53,
                    special_defense: 101,
                    speed: 13
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Steel
                ),
                weight: 820 as f32,
            }
        );
        
        pokedex.insert(
            "necrozmadawnwings".to_string(),
            PokedexPokemon {
                species: "necrozmadawnwings".to_string(),
                base_stats: BaseStats {
                    hp: 97,
                    attack: 113,
                    defense: 109,
                    special_attack: 157,
                    special_defense: 127,
                    speed: 77
                },
                abilities: Abilities {
                    first: "prismarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Ghost
                ),
                weight: 350 as f32,
            }
        );
        
        pokedex.insert(
            "blacephalon".to_string(),
            PokedexPokemon {
                species: "blacephalon".to_string(),
                base_stats: BaseStats {
                    hp: 53,
                    attack: 127,
                    defense: 53,
                    special_attack: 151,
                    special_defense: 79,
                    speed: 107
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Ghost
                ),
                weight: 13 as f32,
            }
        );
        
        pokedex.insert(
            "necrozmaduskmane".to_string(),
            PokedexPokemon {
                species: "necrozmaduskmane".to_string(),
                base_stats: BaseStats {
                    hp: 97,
                    attack: 157,
                    defense: 127,
                    special_attack: 113,
                    special_defense: 109,
                    speed: 77
                },
                abilities: Abilities {
                    first: "prismarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Steel
                ),
                weight: 460 as f32,
            }
        );
        
        pokedex.insert(
            "necrozmaultra".to_string(),
            PokedexPokemon {
                species: "necrozmaultra".to_string(),
                base_stats: BaseStats {
                    hp: 97,
                    attack: 167,
                    defense: 97,
                    special_attack: 167,
                    special_defense: 97,
                    speed: 129
                },
                abilities: Abilities {
                    first: "neuroforce".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Dragon
                ),
                weight: 230 as f32,
            }
        );
        
        pokedex.insert(
            "raticatealolatotem".to_string(),
            PokedexPokemon {
                species: "raticatealolatotem".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 71,
                    defense: 70,
                    special_attack: 40,
                    special_defense: 80,
                    speed: 77
                },
                abilities: Abilities {
                    first: "thickfat".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Normal
                ),
                weight: 105 as f32,
            }
        );
        
        pokedex.insert(
            "pikachuoriginal".to_string(),
            PokedexPokemon {
                species: "pikachuoriginal".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachuhoenn".to_string(),
            PokedexPokemon {
                species: "pikachuhoenn".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachusinnoh".to_string(),
            PokedexPokemon {
                species: "pikachusinnoh".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachuunova".to_string(),
            PokedexPokemon {
                species: "pikachuunova".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachukalos".to_string(),
            PokedexPokemon {
                species: "pikachukalos".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachualola".to_string(),
            PokedexPokemon {
                species: "pikachualola".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachupartner".to_string(),
            PokedexPokemon {
                species: "pikachupartner".to_string(),
                base_stats: BaseStats {
                    hp: 35,
                    attack: 55,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 90
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "pikachustarter".to_string(),
            PokedexPokemon {
                species: "pikachustarter".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 80,
                    defense: 50,
                    special_attack: 75,
                    special_defense: 60,
                    speed: 120
                },
                abilities: Abilities {
                    first: "static".to_string(),
                    second: "none".to_string(),
                    hidden: "lightningrod".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "marowakalolatotem".to_string(),
            PokedexPokemon {
                species: "marowakalolatotem".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 80,
                    defense: 110,
                    special_attack: 50,
                    special_defense: 80,
                    speed: 45
                },
                abilities: Abilities {
                    first: "rockhead".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Ghost
                ),
                weight: 98 as f32,
            }
        );
        
        pokedex.insert(
            "eeveestarter".to_string(),
            PokedexPokemon {
                species: "eeveestarter".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 75,
                    defense: 70,
                    special_attack: 65,
                    special_defense: 85,
                    speed: 75
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "adaptability".to_string(),
                    hidden: "anticipation".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 6.5 as f32,
            }
        );
        
        pokedex.insert(
            "gumshoostotem".to_string(),
            PokedexPokemon {
                species: "gumshoostotem".to_string(),
                base_stats: BaseStats {
                    hp: 88,
                    attack: 110,
                    defense: 60,
                    special_attack: 55,
                    special_defense: 60,
                    speed: 45
                },
                abilities: Abilities {
                    first: "adaptability".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 60 as f32,
            }
        );
        
        pokedex.insert(
            "vikavolttotem".to_string(),
            PokedexPokemon {
                species: "vikavolttotem".to_string(),
                base_stats: BaseStats {
                    hp: 77,
                    attack: 70,
                    defense: 90,
                    special_attack: 145,
                    special_defense: 75,
                    speed: 43
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Electric
                ),
                weight: 147.5 as f32,
            }
        );
        
        pokedex.insert(
            "ribombeetotem".to_string(),
            PokedexPokemon {
                species: "ribombeetotem".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 55,
                    defense: 60,
                    special_attack: 95,
                    special_defense: 70,
                    speed: 124
                },
                abilities: Abilities {
                    first: "sweetveil".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Fairy
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "araquanidtotem".to_string(),
            PokedexPokemon {
                species: "araquanidtotem".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 70,
                    defense: 92,
                    special_attack: 50,
                    special_defense: 132,
                    speed: 42
                },
                abilities: Abilities {
                    first: "waterbubble".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Bug
                ),
                weight: 217.5 as f32,
            }
        );
        
        pokedex.insert(
            "lurantistotem".to_string(),
            PokedexPokemon {
                species: "lurantistotem".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 105,
                    defense: 90,
                    special_attack: 80,
                    special_defense: 90,
                    speed: 45
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 58 as f32,
            }
        );
        
        pokedex.insert(
            "salazzletotem".to_string(),
            PokedexPokemon {
                species: "salazzletotem".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 64,
                    defense: 60,
                    special_attack: 111,
                    special_defense: 60,
                    speed: 117
                },
                abilities: Abilities {
                    first: "corrosion".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Fire
                ),
                weight: 81 as f32,
            }
        );
        
        pokedex.insert(
            "togedemarutotem".to_string(),
            PokedexPokemon {
                species: "togedemarutotem".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 98,
                    defense: 63,
                    special_attack: 40,
                    special_defense: 73,
                    speed: 96
                },
                abilities: Abilities {
                    first: "sturdy".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Steel
                ),
                weight: 13 as f32,
            }
        );
        
        pokedex.insert(
            "mimikyutotem".to_string(),
            PokedexPokemon {
                species: "mimikyutotem".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 90,
                    defense: 80,
                    special_attack: 50,
                    special_defense: 105,
                    speed: 96
                },
                abilities: Abilities {
                    first: "disguise".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Fairy
                ),
                weight: 2.8 as f32,
            }
        );
        
        pokedex.insert(
            "mimikyubustedtotem".to_string(),
            PokedexPokemon {
                species: "mimikyubustedtotem".to_string(),
                base_stats: BaseStats {
                    hp: 55,
                    attack: 90,
                    defense: 80,
                    special_attack: 50,
                    special_defense: 105,
                    speed: 96
                },
                abilities: Abilities {
                    first: "disguise".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Fairy
                ),
                weight: 2.8 as f32,
            }
        );
        
        pokedex.insert(
            "kommoototem".to_string(),
            PokedexPokemon {
                species: "kommoototem".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 110,
                    defense: 125,
                    special_attack: 100,
                    special_defense: 105,
                    speed: 85
                },
                abilities: Abilities {
                    first: "overcoat".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Fighting
                ),
                weight: 207.5 as f32,
            }
        );
        
        pokedex.insert(
            "magearnaoriginal".to_string(),
            PokedexPokemon {
                species: "magearnaoriginal".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 95,
                    defense: 115,
                    special_attack: 130,
                    special_defense: 115,
                    speed: 65
                },
                abilities: Abilities {
                    first: "soulheart".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Fairy
                ),
                weight: 80.5 as f32,
            }
        );
        
        pokedex.insert(
            "poipole".to_string(),
            PokedexPokemon {
                species: "poipole".to_string(),
                base_stats: BaseStats {
                    hp: 67,
                    attack: 73,
                    defense: 67,
                    special_attack: 73,
                    special_defense: 67,
                    speed: 73
                },
                abilities: Abilities {
                    first: "beastboost".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Typeless
                ),
                weight: 1.8 as f32,
            }
        );
        
        pokedex.insert(
            "meltan".to_string(),
            PokedexPokemon {
                species: "meltan".to_string(),
                base_stats: BaseStats {
                    hp: 46,
                    attack: 65,
                    defense: 65,
                    special_attack: 55,
                    special_defense: 35,
                    speed: 34
                },
                abilities: Abilities {
                    first: "magnetpull".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 8 as f32,
            }
        );
        
        pokedex.insert(
            "melmetal".to_string(),
            PokedexPokemon {
                species: "melmetal".to_string(),
                base_stats: BaseStats {
                    hp: 135,
                    attack: 143,
                    defense: 143,
                    special_attack: 80,
                    special_defense: 65,
                    speed: 34
                },
                abilities: Abilities {
                    first: "ironfist".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 800 as f32,
            }
        );
        
        pokedex.insert(
            "grookey".to_string(),
            PokedexPokemon {
                species: "grookey".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 65
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "grassysurge".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 5 as f32,
            }
        );
        
        pokedex.insert(
            "thwackey".to_string(),
            PokedexPokemon {
                species: "thwackey".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 70,
                    special_attack: 55,
                    special_defense: 60,
                    speed: 80
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "grassysurge".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 14 as f32,
            }
        );
        
        pokedex.insert(
            "rillaboom".to_string(),
            PokedexPokemon {
                species: "rillaboom".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 125,
                    defense: 90,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 85
                },
                abilities: Abilities {
                    first: "overgrow".to_string(),
                    second: "none".to_string(),
                    hidden: "grassysurge".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 90 as f32,
            }
        );
        
        pokedex.insert(
            "scorbunny".to_string(),
            PokedexPokemon {
                species: "scorbunny".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 71,
                    defense: 40,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 69
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "libero".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 4.5 as f32,
            }
        );
        
        pokedex.insert(
            "raboot".to_string(),
            PokedexPokemon {
                species: "raboot".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 86,
                    defense: 60,
                    special_attack: 55,
                    special_defense: 60,
                    speed: 94
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "libero".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 9 as f32,
            }
        );
        
        pokedex.insert(
            "cinderace".to_string(),
            PokedexPokemon {
                species: "cinderace".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 116,
                    defense: 75,
                    special_attack: 65,
                    special_defense: 75,
                    speed: 119
                },
                abilities: Abilities {
                    first: "blaze".to_string(),
                    second: "none".to_string(),
                    hidden: "libero".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Typeless
                ),
                weight: 33 as f32,
            }
        );
        
        pokedex.insert(
            "sobble".to_string(),
            PokedexPokemon {
                species: "sobble".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 40,
                    defense: 40,
                    special_attack: 70,
                    special_defense: 40,
                    speed: 70
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "sniper".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "drizzile".to_string(),
            PokedexPokemon {
                species: "drizzile".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 60,
                    defense: 55,
                    special_attack: 95,
                    special_defense: 55,
                    speed: 90
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "sniper".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 11.5 as f32,
            }
        );
        
        pokedex.insert(
            "inteleon".to_string(),
            PokedexPokemon {
                species: "inteleon".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 65,
                    special_attack: 125,
                    special_defense: 65,
                    speed: 120
                },
                abilities: Abilities {
                    first: "torrent".to_string(),
                    second: "none".to_string(),
                    hidden: "sniper".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 45.2 as f32,
            }
        );
        
        pokedex.insert(
            "skwovet".to_string(),
            PokedexPokemon {
                species: "skwovet".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 55,
                    defense: 55,
                    special_attack: 35,
                    special_defense: 35,
                    speed: 25
                },
                abilities: Abilities {
                    first: "cheekpouch".to_string(),
                    second: "none".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 2.5 as f32,
            }
        );
        
        pokedex.insert(
            "greedent".to_string(),
            PokedexPokemon {
                species: "greedent".to_string(),
                base_stats: BaseStats {
                    hp: 120,
                    attack: 95,
                    defense: 95,
                    special_attack: 55,
                    special_defense: 75,
                    speed: 20
                },
                abilities: Abilities {
                    first: "cheekpouch".to_string(),
                    second: "none".to_string(),
                    hidden: "gluttony".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "rookidee".to_string(),
            PokedexPokemon {
                species: "rookidee".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 47,
                    defense: 35,
                    special_attack: 33,
                    special_defense: 35,
                    speed: 57
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "unnerve".to_string(),
                    hidden: "bigpecks".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Typeless
                ),
                weight: 1.8 as f32,
            }
        );
        
        pokedex.insert(
            "corvisquire".to_string(),
            PokedexPokemon {
                species: "corvisquire".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 67,
                    defense: 55,
                    special_attack: 43,
                    special_defense: 55,
                    speed: 77
                },
                abilities: Abilities {
                    first: "keeneye".to_string(),
                    second: "unnerve".to_string(),
                    hidden: "bigpecks".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Typeless
                ),
                weight: 16 as f32,
            }
        );
        
        pokedex.insert(
            "corviknight".to_string(),
            PokedexPokemon {
                species: "corviknight".to_string(),
                base_stats: BaseStats {
                    hp: 98,
                    attack: 87,
                    defense: 105,
                    special_attack: 53,
                    special_defense: 85,
                    speed: 67
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "unnerve".to_string(),
                    hidden: "mirrorarmor".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Steel
                ),
                weight: 75 as f32,
            }
        );
        
        pokedex.insert(
            "corviknightgmax".to_string(),
            PokedexPokemon {
                species: "corviknightgmax".to_string(),
                base_stats: BaseStats {
                    hp: 98,
                    attack: 87,
                    defense: 105,
                    special_attack: 53,
                    special_defense: 85,
                    speed: 67
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "unnerve".to_string(),
                    hidden: "mirrorarmor".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Steel
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "blipbug".to_string(),
            PokedexPokemon {
                species: "blipbug".to_string(),
                base_stats: BaseStats {
                    hp: 25,
                    attack: 20,
                    defense: 20,
                    special_attack: 25,
                    special_defense: 45,
                    speed: 45
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "compoundeyes".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Typeless
                ),
                weight: 8 as f32,
            }
        );
        
        pokedex.insert(
            "dottler".to_string(),
            PokedexPokemon {
                species: "dottler".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 35,
                    defense: 80,
                    special_attack: 50,
                    special_defense: 90,
                    speed: 30
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "compoundeyes".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Psychic
                ),
                weight: 19.5 as f32,
            }
        );
        
        pokedex.insert(
            "orbeetle".to_string(),
            PokedexPokemon {
                species: "orbeetle".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 45,
                    defense: 110,
                    special_attack: 80,
                    special_defense: 120,
                    speed: 90
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "frisk".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Psychic
                ),
                weight: 40.8 as f32,
            }
        );
        
        pokedex.insert(
            "orbeetlegmax".to_string(),
            PokedexPokemon {
                species: "orbeetlegmax".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 45,
                    defense: 110,
                    special_attack: 80,
                    special_defense: 120,
                    speed: 90
                },
                abilities: Abilities {
                    first: "swarm".to_string(),
                    second: "frisk".to_string(),
                    hidden: "telepathy".to_string()
                },
                types: (
                    PokemonTypes::Bug,
                    PokemonTypes::Psychic
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "nickit".to_string(),
            PokedexPokemon {
                species: "nickit".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 28,
                    defense: 28,
                    special_attack: 47,
                    special_defense: 52,
                    speed: 50
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "unburden".to_string(),
                    hidden: "stakeout".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 8.9 as f32,
            }
        );
        
        pokedex.insert(
            "thievul".to_string(),
            PokedexPokemon {
                species: "thievul".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 58,
                    defense: 58,
                    special_attack: 87,
                    special_defense: 92,
                    speed: 90
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "unburden".to_string(),
                    hidden: "stakeout".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Typeless
                ),
                weight: 19.9 as f32,
            }
        );
        
        pokedex.insert(
            "gossifleur".to_string(),
            PokedexPokemon {
                species: "gossifleur".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 40,
                    defense: 60,
                    special_attack: 40,
                    special_defense: 60,
                    speed: 10
                },
                abilities: Abilities {
                    first: "cottondown".to_string(),
                    second: "regenerator".to_string(),
                    hidden: "effectspore".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 2.2 as f32,
            }
        );
        
        pokedex.insert(
            "eldegoss".to_string(),
            PokedexPokemon {
                species: "eldegoss".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 50,
                    defense: 90,
                    special_attack: 80,
                    special_defense: 120,
                    speed: 60
                },
                abilities: Abilities {
                    first: "cottondown".to_string(),
                    second: "regenerator".to_string(),
                    hidden: "effectspore".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Typeless
                ),
                weight: 2.5 as f32,
            }
        );
        
        pokedex.insert(
            "wooloo".to_string(),
            PokedexPokemon {
                species: "wooloo".to_string(),
                base_stats: BaseStats {
                    hp: 42,
                    attack: 40,
                    defense: 55,
                    special_attack: 40,
                    special_defense: 45,
                    speed: 48
                },
                abilities: Abilities {
                    first: "fluffy".to_string(),
                    second: "runaway".to_string(),
                    hidden: "bulletproof".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 6 as f32,
            }
        );
        
        pokedex.insert(
            "dubwool".to_string(),
            PokedexPokemon {
                species: "dubwool".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 80,
                    defense: 100,
                    special_attack: 60,
                    special_defense: 90,
                    speed: 88
                },
                abilities: Abilities {
                    first: "fluffy".to_string(),
                    second: "steadfast".to_string(),
                    hidden: "bulletproof".to_string()
                },
                types: (
                    PokemonTypes::Normal,
                    PokemonTypes::Typeless
                ),
                weight: 43 as f32,
            }
        );
        
        pokedex.insert(
            "chewtle".to_string(),
            PokedexPokemon {
                species: "chewtle".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 64,
                    defense: 50,
                    special_attack: 38,
                    special_defense: 38,
                    speed: 44
                },
                abilities: Abilities {
                    first: "strongjaw".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 8.5 as f32,
            }
        );
        
        pokedex.insert(
            "drednaw".to_string(),
            PokedexPokemon {
                species: "drednaw".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 115,
                    defense: 90,
                    special_attack: 48,
                    special_defense: 68,
                    speed: 74
                },
                abilities: Abilities {
                    first: "strongjaw".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Rock
                ),
                weight: 115.5 as f32,
            }
        );
        
        pokedex.insert(
            "drednawgmax".to_string(),
            PokedexPokemon {
                species: "drednawgmax".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 115,
                    defense: 90,
                    special_attack: 48,
                    special_defense: 68,
                    speed: 74
                },
                abilities: Abilities {
                    first: "strongjaw".to_string(),
                    second: "shellarmor".to_string(),
                    hidden: "swiftswim".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Rock
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "yamper".to_string(),
            PokedexPokemon {
                species: "yamper".to_string(),
                base_stats: BaseStats {
                    hp: 59,
                    attack: 45,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 26
                },
                abilities: Abilities {
                    first: "ballfetch".to_string(),
                    second: "none".to_string(),
                    hidden: "rattled".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 13.5 as f32,
            }
        );
        
        pokedex.insert(
            "boltund".to_string(),
            PokedexPokemon {
                species: "boltund".to_string(),
                base_stats: BaseStats {
                    hp: 69,
                    attack: 90,
                    defense: 60,
                    special_attack: 90,
                    special_defense: 60,
                    speed: 121
                },
                abilities: Abilities {
                    first: "strongjaw".to_string(),
                    second: "none".to_string(),
                    hidden: "competitive".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 34 as f32,
            }
        );
        
        pokedex.insert(
            "rolycoly".to_string(),
            PokedexPokemon {
                species: "rolycoly".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 40,
                    defense: 50,
                    special_attack: 40,
                    special_defense: 50,
                    speed: 30
                },
                abilities: Abilities {
                    first: "steamengine".to_string(),
                    second: "heatproof".to_string(),
                    hidden: "flashfire".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "carkol".to_string(),
            PokedexPokemon {
                species: "carkol".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 60,
                    defense: 90,
                    special_attack: 60,
                    special_defense: 70,
                    speed: 50
                },
                abilities: Abilities {
                    first: "steamengine".to_string(),
                    second: "flamebody".to_string(),
                    hidden: "flashfire".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Fire
                ),
                weight: 78 as f32,
            }
        );
        
        pokedex.insert(
            "coalossal".to_string(),
            PokedexPokemon {
                species: "coalossal".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 80,
                    defense: 120,
                    special_attack: 80,
                    special_defense: 90,
                    speed: 30
                },
                abilities: Abilities {
                    first: "steamengine".to_string(),
                    second: "flamebody".to_string(),
                    hidden: "flashfire".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Fire
                ),
                weight: 310.5 as f32,
            }
        );
        
        pokedex.insert(
            "coalossalgmax".to_string(),
            PokedexPokemon {
                species: "coalossalgmax".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 80,
                    defense: 120,
                    special_attack: 80,
                    special_defense: 90,
                    speed: 30
                },
                abilities: Abilities {
                    first: "steamengine".to_string(),
                    second: "flamebody".to_string(),
                    hidden: "flashfire".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Fire
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "applin".to_string(),
            PokedexPokemon {
                species: "applin".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 40,
                    defense: 80,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 20
                },
                abilities: Abilities {
                    first: "ripen".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "bulletproof".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dragon
                ),
                weight: 0.5 as f32,
            }
        );
        
        pokedex.insert(
            "flapple".to_string(),
            PokedexPokemon {
                species: "flapple".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 110,
                    defense: 80,
                    special_attack: 95,
                    special_defense: 60,
                    speed: 70
                },
                abilities: Abilities {
                    first: "ripen".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dragon
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "flapplegmax".to_string(),
            PokedexPokemon {
                species: "flapplegmax".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 110,
                    defense: 80,
                    special_attack: 95,
                    special_defense: 60,
                    speed: 70
                },
                abilities: Abilities {
                    first: "ripen".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "hustle".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dragon
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "appletun".to_string(),
            PokedexPokemon {
                species: "appletun".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 85,
                    defense: 80,
                    special_attack: 100,
                    special_defense: 80,
                    speed: 30
                },
                abilities: Abilities {
                    first: "ripen".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dragon
                ),
                weight: 13 as f32,
            }
        );
        
        pokedex.insert(
            "appletungmax".to_string(),
            PokedexPokemon {
                species: "appletungmax".to_string(),
                base_stats: BaseStats {
                    hp: 110,
                    attack: 85,
                    defense: 80,
                    special_attack: 100,
                    special_defense: 80,
                    speed: 30
                },
                abilities: Abilities {
                    first: "ripen".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "thickfat".to_string()
                },
                types: (
                    PokemonTypes::Grass,
                    PokemonTypes::Dragon
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "silicobra".to_string(),
            PokedexPokemon {
                species: "silicobra".to_string(),
                base_stats: BaseStats {
                    hp: 52,
                    attack: 57,
                    defense: 75,
                    special_attack: 35,
                    special_defense: 50,
                    speed: 46
                },
                abilities: Abilities {
                    first: "sandspit".to_string(),
                    second: "shedskin".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 7.6 as f32,
            }
        );
        
        pokedex.insert(
            "sandaconda".to_string(),
            PokedexPokemon {
                species: "sandaconda".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 107,
                    defense: 125,
                    special_attack: 65,
                    special_defense: 70,
                    speed: 71
                },
                abilities: Abilities {
                    first: "sandspit".to_string(),
                    second: "shedskin".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 65.5 as f32,
            }
        );
        
        pokedex.insert(
            "sandacondagmax".to_string(),
            PokedexPokemon {
                species: "sandacondagmax".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 107,
                    defense: 125,
                    special_attack: 65,
                    special_defense: 70,
                    speed: 71
                },
                abilities: Abilities {
                    first: "sandspit".to_string(),
                    second: "shedskin".to_string(),
                    hidden: "sandveil".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Typeless
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "cramorant".to_string(),
            PokedexPokemon {
                species: "cramorant".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 55,
                    special_attack: 85,
                    special_defense: 95,
                    speed: 85
                },
                abilities: Abilities {
                    first: "gulpmissile".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Water
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "cramorantgulping".to_string(),
            PokedexPokemon {
                species: "cramorantgulping".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 55,
                    special_attack: 85,
                    special_defense: 95,
                    speed: 85
                },
                abilities: Abilities {
                    first: "gulpmissile".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Water
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "cramorantgorging".to_string(),
            PokedexPokemon {
                species: "cramorantgorging".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 85,
                    defense: 55,
                    special_attack: 85,
                    special_defense: 95,
                    speed: 85
                },
                abilities: Abilities {
                    first: "gulpmissile".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Flying,
                    PokemonTypes::Water
                ),
                weight: 18 as f32,
            }
        );
        
        pokedex.insert(
            "arrokuda".to_string(),
            PokedexPokemon {
                species: "arrokuda".to_string(),
                base_stats: BaseStats {
                    hp: 41,
                    attack: 63,
                    defense: 40,
                    special_attack: 40,
                    special_defense: 30,
                    speed: 66
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "propellertail".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "barraskewda".to_string(),
            PokedexPokemon {
                species: "barraskewda".to_string(),
                base_stats: BaseStats {
                    hp: 61,
                    attack: 123,
                    defense: 60,
                    special_attack: 60,
                    special_defense: 50,
                    speed: 136
                },
                abilities: Abilities {
                    first: "swiftswim".to_string(),
                    second: "none".to_string(),
                    hidden: "propellertail".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Typeless
                ),
                weight: 30 as f32,
            }
        );
        
        pokedex.insert(
            "toxel".to_string(),
            PokedexPokemon {
                species: "toxel".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 38,
                    defense: 35,
                    special_attack: 54,
                    special_defense: 35,
                    speed: 40
                },
                abilities: Abilities {
                    first: "rattled".to_string(),
                    second: "static".to_string(),
                    hidden: "klutz".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Poison
                ),
                weight: 11 as f32,
            }
        );
        
        pokedex.insert(
            "toxtricity".to_string(),
            PokedexPokemon {
                species: "toxtricity".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 98,
                    defense: 70,
                    special_attack: 114,
                    special_defense: 70,
                    speed: 75
                },
                abilities: Abilities {
                    first: "punkrock".to_string(),
                    second: "plus".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Poison
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "toxtricitylowkey".to_string(),
            PokedexPokemon {
                species: "toxtricitylowkey".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 98,
                    defense: 70,
                    special_attack: 114,
                    special_defense: 70,
                    speed: 75
                },
                abilities: Abilities {
                    first: "punkrock".to_string(),
                    second: "minus".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Poison
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "toxtricitygmax".to_string(),
            PokedexPokemon {
                species: "toxtricitygmax".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 98,
                    defense: 70,
                    special_attack: 114,
                    special_defense: 70,
                    speed: 75
                },
                abilities: Abilities {
                    first: "punkrock".to_string(),
                    second: "plus".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Poison
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "sizzlipede".to_string(),
            PokedexPokemon {
                species: "sizzlipede".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 45,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 45
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "whitesmoke".to_string(),
                    hidden: "flamebody".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Bug
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "centiskorch".to_string(),
            PokedexPokemon {
                species: "centiskorch".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 115,
                    defense: 65,
                    special_attack: 90,
                    special_defense: 90,
                    speed: 65
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "whitesmoke".to_string(),
                    hidden: "flamebody".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Bug
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "centiskorchgmax".to_string(),
            PokedexPokemon {
                species: "centiskorchgmax".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 115,
                    defense: 65,
                    special_attack: 90,
                    special_defense: 90,
                    speed: 65
                },
                abilities: Abilities {
                    first: "flashfire".to_string(),
                    second: "whitesmoke".to_string(),
                    hidden: "flamebody".to_string()
                },
                types: (
                    PokemonTypes::Fire,
                    PokemonTypes::Bug
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "clobbopus".to_string(),
            PokedexPokemon {
                species: "clobbopus".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 68,
                    defense: 60,
                    special_attack: 50,
                    special_defense: 50,
                    speed: 32
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "none".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 4 as f32,
            }
        );
        
        pokedex.insert(
            "grapploct".to_string(),
            PokedexPokemon {
                species: "grapploct".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 118,
                    defense: 90,
                    special_attack: 70,
                    special_defense: 80,
                    speed: 42
                },
                abilities: Abilities {
                    first: "limber".to_string(),
                    second: "none".to_string(),
                    hidden: "technician".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 39 as f32,
            }
        );
        
        pokedex.insert(
            "sinistea".to_string(),
            PokedexPokemon {
                species: "sinistea".to_string(),
                base_stats: BaseStats {
                    hp: 40,
                    attack: 45,
                    defense: 45,
                    special_attack: 74,
                    special_defense: 54,
                    speed: 50
                },
                abilities: Abilities {
                    first: "weakarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 0.2 as f32,
            }
        );
        
        pokedex.insert(
            "polteageist".to_string(),
            PokedexPokemon {
                species: "polteageist".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 65,
                    defense: 65,
                    special_attack: 134,
                    special_defense: 114,
                    speed: 70
                },
                abilities: Abilities {
                    first: "weakarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 0.4 as f32,
            }
        );
        
        pokedex.insert(
            "hatenna".to_string(),
            PokedexPokemon {
                species: "hatenna".to_string(),
                base_stats: BaseStats {
                    hp: 42,
                    attack: 30,
                    defense: 45,
                    special_attack: 56,
                    special_defense: 53,
                    speed: 39
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "anticipation".to_string(),
                    hidden: "magicbounce".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 3.4 as f32,
            }
        );
        
        pokedex.insert(
            "hattrem".to_string(),
            PokedexPokemon {
                species: "hattrem".to_string(),
                base_stats: BaseStats {
                    hp: 57,
                    attack: 40,
                    defense: 65,
                    special_attack: 86,
                    special_defense: 73,
                    speed: 49
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "anticipation".to_string(),
                    hidden: "magicbounce".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 4.8 as f32,
            }
        );
        
        pokedex.insert(
            "hatterene".to_string(),
            PokedexPokemon {
                species: "hatterene".to_string(),
                base_stats: BaseStats {
                    hp: 57,
                    attack: 90,
                    defense: 95,
                    special_attack: 136,
                    special_defense: 103,
                    speed: 29
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "anticipation".to_string(),
                    hidden: "magicbounce".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 5.1 as f32,
            }
        );
        
        pokedex.insert(
            "hatterenegmax".to_string(),
            PokedexPokemon {
                species: "hatterenegmax".to_string(),
                base_stats: BaseStats {
                    hp: 57,
                    attack: 90,
                    defense: 95,
                    special_attack: 136,
                    special_defense: 103,
                    speed: 29
                },
                abilities: Abilities {
                    first: "healer".to_string(),
                    second: "anticipation".to_string(),
                    hidden: "magicbounce".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "impidimp".to_string(),
            PokedexPokemon {
                species: "impidimp".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 45,
                    defense: 30,
                    special_attack: 55,
                    special_defense: 40,
                    speed: 50
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "frisk".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fairy
                ),
                weight: 5.5 as f32,
            }
        );
        
        pokedex.insert(
            "morgrem".to_string(),
            PokedexPokemon {
                species: "morgrem".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 60,
                    defense: 45,
                    special_attack: 75,
                    special_defense: 55,
                    speed: 70
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "frisk".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fairy
                ),
                weight: 12.5 as f32,
            }
        );
        
        pokedex.insert(
            "grimmsnarl".to_string(),
            PokedexPokemon {
                species: "grimmsnarl".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 120,
                    defense: 65,
                    special_attack: 95,
                    special_defense: 75,
                    speed: 60
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "frisk".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fairy
                ),
                weight: 61 as f32,
            }
        );
        
        pokedex.insert(
            "grimmsnarlgmax".to_string(),
            PokedexPokemon {
                species: "grimmsnarlgmax".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 120,
                    defense: 65,
                    special_attack: 95,
                    special_defense: 75,
                    speed: 60
                },
                abilities: Abilities {
                    first: "prankster".to_string(),
                    second: "frisk".to_string(),
                    hidden: "pickpocket".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Fairy
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "zigzagoongalar".to_string(),
            PokedexPokemon {
                species: "zigzagoongalar".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 30,
                    defense: 41,
                    special_attack: 30,
                    special_defense: 41,
                    speed: 60
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "quickfeet".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Normal
                ),
                weight: 17.5 as f32,
            }
        );
        
        pokedex.insert(
            "obstagoon".to_string(),
            PokedexPokemon {
                species: "obstagoon".to_string(),
                base_stats: BaseStats {
                    hp: 93,
                    attack: 90,
                    defense: 101,
                    special_attack: 60,
                    special_defense: 81,
                    speed: 95
                },
                abilities: Abilities {
                    first: "reckless".to_string(),
                    second: "guts".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Normal
                ),
                weight: 46 as f32,
            }
        );
        
        pokedex.insert(
            "perrserker".to_string(),
            PokedexPokemon {
                species: "perrserker".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 110,
                    defense: 100,
                    special_attack: 50,
                    special_defense: 60,
                    speed: 50
                },
                abilities: Abilities {
                    first: "battlearmor".to_string(),
                    second: "toughclaws".to_string(),
                    hidden: "steelyspirit".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "cursola".to_string(),
            PokedexPokemon {
                species: "cursola".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 95,
                    defense: 50,
                    special_attack: 145,
                    special_defense: 130,
                    speed: 30
                },
                abilities: Abilities {
                    first: "weakarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "perishbody".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 0.4 as f32,
            }
        );
        
        pokedex.insert(
            "sirfetchd".to_string(),
            PokedexPokemon {
                species: "sirfetchd".to_string(),
                base_stats: BaseStats {
                    hp: 62,
                    attack: 135,
                    defense: 95,
                    special_attack: 68,
                    special_defense: 82,
                    speed: 65
                },
                abilities: Abilities {
                    first: "steadfast".to_string(),
                    second: "none".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 117 as f32,
            }
        );
        
        pokedex.insert(
            "mrrime".to_string(),
            PokedexPokemon {
                species: "mrrime".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 85,
                    defense: 75,
                    special_attack: 110,
                    special_defense: 100,
                    speed: 70
                },
                abilities: Abilities {
                    first: "tangledfeet".to_string(),
                    second: "screencleaner".to_string(),
                    hidden: "icebody".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Psychic
                ),
                weight: 58.2 as f32,
            }
        );
        
        pokedex.insert(
            "runerigus".to_string(),
            PokedexPokemon {
                species: "runerigus".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 95,
                    defense: 145,
                    special_attack: 50,
                    special_defense: 105,
                    speed: 30
                },
                abilities: Abilities {
                    first: "wanderingspirit".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Ghost
                ),
                weight: 66.6 as f32,
            }
        );
        
        pokedex.insert(
            "milcery".to_string(),
            PokedexPokemon {
                species: "milcery".to_string(),
                base_stats: BaseStats {
                    hp: 45,
                    attack: 40,
                    defense: 40,
                    special_attack: 50,
                    special_defense: 61,
                    speed: 34
                },
                abilities: Abilities {
                    first: "sweetveil".to_string(),
                    second: "none".to_string(),
                    hidden: "aromaveil".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 0.3 as f32,
            }
        );
        
        pokedex.insert(
            "alcremie".to_string(),
            PokedexPokemon {
                species: "alcremie".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 60,
                    defense: 75,
                    special_attack: 110,
                    special_defense: 121,
                    speed: 64
                },
                abilities: Abilities {
                    first: "sweetveil".to_string(),
                    second: "none".to_string(),
                    hidden: "aromaveil".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 0.5 as f32,
            }
        );
        
        pokedex.insert(
            "alcremiegmax".to_string(),
            PokedexPokemon {
                species: "alcremiegmax".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 60,
                    defense: 75,
                    special_attack: 110,
                    special_defense: 121,
                    speed: 64
                },
                abilities: Abilities {
                    first: "sweetveil".to_string(),
                    second: "none".to_string(),
                    hidden: "aromaveil".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "falinks".to_string(),
            PokedexPokemon {
                species: "falinks".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 100,
                    defense: 100,
                    special_attack: 70,
                    special_defense: 60,
                    speed: 75
                },
                abilities: Abilities {
                    first: "battlearmor".to_string(),
                    second: "none".to_string(),
                    hidden: "defiant".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 62 as f32,
            }
        );
        
        pokedex.insert(
            "pincurchin".to_string(),
            PokedexPokemon {
                species: "pincurchin".to_string(),
                base_stats: BaseStats {
                    hp: 48,
                    attack: 101,
                    defense: 95,
                    special_attack: 91,
                    special_defense: 85,
                    speed: 15
                },
                abilities: Abilities {
                    first: "lightningrod".to_string(),
                    second: "none".to_string(),
                    hidden: "electricsurge".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 1 as f32,
            }
        );
        
        pokedex.insert(
            "snom".to_string(),
            PokedexPokemon {
                species: "snom".to_string(),
                base_stats: BaseStats {
                    hp: 30,
                    attack: 25,
                    defense: 35,
                    special_attack: 45,
                    special_defense: 30,
                    speed: 20
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "none".to_string(),
                    hidden: "icescales".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Bug
                ),
                weight: 3.8 as f32,
            }
        );
        
        pokedex.insert(
            "frosmoth".to_string(),
            PokedexPokemon {
                species: "frosmoth".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 65,
                    defense: 60,
                    special_attack: 125,
                    special_defense: 90,
                    speed: 65
                },
                abilities: Abilities {
                    first: "shielddust".to_string(),
                    second: "none".to_string(),
                    hidden: "icescales".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Bug
                ),
                weight: 42 as f32,
            }
        );
        
        pokedex.insert(
            "stonjourner".to_string(),
            PokedexPokemon {
                species: "stonjourner".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 125,
                    defense: 135,
                    special_attack: 20,
                    special_defense: 20,
                    speed: 70
                },
                abilities: Abilities {
                    first: "powerspot".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Rock,
                    PokemonTypes::Typeless
                ),
                weight: 520 as f32,
            }
        );
        
        pokedex.insert(
            "eiscue".to_string(),
            PokedexPokemon {
                species: "eiscue".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 80,
                    defense: 110,
                    special_attack: 65,
                    special_defense: 90,
                    speed: 50
                },
                abilities: Abilities {
                    first: "iceface".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 89 as f32,
            }
        );
        
        pokedex.insert(
            "eiscuenoice".to_string(),
            PokedexPokemon {
                species: "eiscuenoice".to_string(),
                base_stats: BaseStats {
                    hp: 75,
                    attack: 80,
                    defense: 70,
                    special_attack: 65,
                    special_defense: 50,
                    speed: 130
                },
                abilities: Abilities {
                    first: "iceface".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 89 as f32,
            }
        );
        
        pokedex.insert(
            "indeedee".to_string(),
            PokedexPokemon {
                species: "indeedee".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 65,
                    defense: 55,
                    special_attack: 105,
                    special_defense: 95,
                    speed: 95
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "synchronize".to_string(),
                    hidden: "psychicsurge".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Normal
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "indeedeef".to_string(),
            PokedexPokemon {
                species: "indeedeef".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 55,
                    defense: 65,
                    special_attack: 95,
                    special_defense: 105,
                    speed: 85
                },
                abilities: Abilities {
                    first: "owntempo".to_string(),
                    second: "synchronize".to_string(),
                    hidden: "psychicsurge".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Normal
                ),
                weight: 28 as f32,
            }
        );
        
        pokedex.insert(
            "morpeko".to_string(),
            PokedexPokemon {
                species: "morpeko".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 95,
                    defense: 58,
                    special_attack: 70,
                    special_defense: 58,
                    speed: 97
                },
                abilities: Abilities {
                    first: "hungerswitch".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Dark
                ),
                weight: 3 as f32,
            }
        );
        
        pokedex.insert(
            "morpekohangry".to_string(),
            PokedexPokemon {
                species: "morpekohangry".to_string(),
                base_stats: BaseStats {
                    hp: 58,
                    attack: 95,
                    defense: 58,
                    special_attack: 70,
                    special_defense: 58,
                    speed: 97
                },
                abilities: Abilities {
                    first: "hungerswitch".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Dark
                ),
                weight: 3 as f32,
            }
        );
        
        pokedex.insert(
            "cufant".to_string(),
            PokedexPokemon {
                species: "cufant".to_string(),
                base_stats: BaseStats {
                    hp: 72,
                    attack: 80,
                    defense: 49,
                    special_attack: 40,
                    special_defense: 49,
                    speed: 40
                },
                abilities: Abilities {
                    first: "sheerforce".to_string(),
                    second: "none".to_string(),
                    hidden: "heavymetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 100 as f32,
            }
        );
        
        pokedex.insert(
            "copperajah".to_string(),
            PokedexPokemon {
                species: "copperajah".to_string(),
                base_stats: BaseStats {
                    hp: 122,
                    attack: 130,
                    defense: 69,
                    special_attack: 80,
                    special_defense: 69,
                    speed: 30
                },
                abilities: Abilities {
                    first: "sheerforce".to_string(),
                    second: "none".to_string(),
                    hidden: "heavymetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 650 as f32,
            }
        );
        
        pokedex.insert(
            "copperajahgmax".to_string(),
            PokedexPokemon {
                species: "copperajahgmax".to_string(),
                base_stats: BaseStats {
                    hp: 122,
                    attack: 130,
                    defense: 69,
                    special_attack: 80,
                    special_defense: 69,
                    speed: 30
                },
                abilities: Abilities {
                    first: "sheerforce".to_string(),
                    second: "none".to_string(),
                    hidden: "heavymetal".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "dracozolt".to_string(),
            PokedexPokemon {
                species: "dracozolt".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 100,
                    defense: 90,
                    special_attack: 80,
                    special_defense: 70,
                    speed: 75
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "hustle".to_string(),
                    hidden: "sandrush".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Dragon
                ),
                weight: 190 as f32,
            }
        );
        
        pokedex.insert(
            "arctozolt".to_string(),
            PokedexPokemon {
                species: "arctozolt".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 100,
                    defense: 90,
                    special_attack: 90,
                    special_defense: 80,
                    speed: 55
                },
                abilities: Abilities {
                    first: "voltabsorb".to_string(),
                    second: "static".to_string(),
                    hidden: "slushrush".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Ice
                ),
                weight: 150 as f32,
            }
        );
        
        pokedex.insert(
            "dracovish".to_string(),
            PokedexPokemon {
                species: "dracovish".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 90,
                    defense: 100,
                    special_attack: 70,
                    special_defense: 80,
                    speed: 75
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "strongjaw".to_string(),
                    hidden: "sandrush".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Dragon
                ),
                weight: 215 as f32,
            }
        );
        
        pokedex.insert(
            "arctovish".to_string(),
            PokedexPokemon {
                species: "arctovish".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 90,
                    defense: 100,
                    special_attack: 80,
                    special_defense: 90,
                    speed: 55
                },
                abilities: Abilities {
                    first: "waterabsorb".to_string(),
                    second: "icebody".to_string(),
                    hidden: "slushrush".to_string()
                },
                types: (
                    PokemonTypes::Water,
                    PokemonTypes::Ice
                ),
                weight: 175 as f32,
            }
        );
        
        pokedex.insert(
            "duraludon".to_string(),
            PokedexPokemon {
                species: "duraludon".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 95,
                    defense: 115,
                    special_attack: 120,
                    special_defense: 50,
                    speed: 85
                },
                abilities: Abilities {
                    first: "lightmetal".to_string(),
                    second: "heavymetal".to_string(),
                    hidden: "stalwart".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Dragon
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "duraludongmax".to_string(),
            PokedexPokemon {
                species: "duraludongmax".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 95,
                    defense: 115,
                    special_attack: 120,
                    special_defense: 50,
                    speed: 85
                },
                abilities: Abilities {
                    first: "lightmetal".to_string(),
                    second: "heavymetal".to_string(),
                    hidden: "stalwart".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Dragon
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "dreepy".to_string(),
            PokedexPokemon {
                species: "dreepy".to_string(),
                base_stats: BaseStats {
                    hp: 28,
                    attack: 60,
                    defense: 30,
                    special_attack: 40,
                    special_defense: 30,
                    speed: 82
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ghost
                ),
                weight: 2 as f32,
            }
        );
        
        pokedex.insert(
            "drakloak".to_string(),
            PokedexPokemon {
                species: "drakloak".to_string(),
                base_stats: BaseStats {
                    hp: 68,
                    attack: 80,
                    defense: 50,
                    special_attack: 60,
                    special_defense: 50,
                    speed: 102
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ghost
                ),
                weight: 11 as f32,
            }
        );
        
        pokedex.insert(
            "dragapult".to_string(),
            PokedexPokemon {
                species: "dragapult".to_string(),
                base_stats: BaseStats {
                    hp: 88,
                    attack: 120,
                    defense: 75,
                    special_attack: 100,
                    special_defense: 75,
                    speed: 142
                },
                abilities: Abilities {
                    first: "clearbody".to_string(),
                    second: "infiltrator".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Ghost
                ),
                weight: 50 as f32,
            }
        );
        
        pokedex.insert(
            "zacian".to_string(),
            PokedexPokemon {
                species: "zacian".to_string(),
                base_stats: BaseStats {
                    hp: 92,
                    attack: 130,
                    defense: 115,
                    special_attack: 80,
                    special_defense: 115,
                    speed: 138
                },
                abilities: Abilities {
                    first: "intrepidsword".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Typeless
                ),
                weight: 110 as f32,
            }
        );
        
        pokedex.insert(
            "zaciancrowned".to_string(),
            PokedexPokemon {
                species: "zaciancrowned".to_string(),
                base_stats: BaseStats {
                    hp: 92,
                    attack: 170,
                    defense: 115,
                    special_attack: 80,
                    special_defense: 115,
                    speed: 148
                },
                abilities: Abilities {
                    first: "intrepidsword".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fairy,
                    PokemonTypes::Steel
                ),
                weight: 355 as f32,
            }
        );
        
        pokedex.insert(
            "zamazenta".to_string(),
            PokedexPokemon {
                species: "zamazenta".to_string(),
                base_stats: BaseStats {
                    hp: 92,
                    attack: 130,
                    defense: 115,
                    special_attack: 80,
                    special_defense: 115,
                    speed: 138
                },
                abilities: Abilities {
                    first: "dauntlessshield".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 210 as f32,
            }
        );
        
        pokedex.insert(
            "zamazentacrowned".to_string(),
            PokedexPokemon {
                species: "zamazentacrowned".to_string(),
                base_stats: BaseStats {
                    hp: 92,
                    attack: 130,
                    defense: 145,
                    special_attack: 80,
                    special_defense: 145,
                    speed: 128
                },
                abilities: Abilities {
                    first: "dauntlessshield".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Steel
                ),
                weight: 785 as f32,
            }
        );
        
        pokedex.insert(
            "eternatus".to_string(),
            PokedexPokemon {
                species: "eternatus".to_string(),
                base_stats: BaseStats {
                    hp: 140,
                    attack: 85,
                    defense: 95,
                    special_attack: 145,
                    special_defense: 95,
                    speed: 130
                },
                abilities: Abilities {
                    first: "pressure".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Dragon
                ),
                weight: 950 as f32,
            }
        );
        
        pokedex.insert(
            "meowthgalar".to_string(),
            PokedexPokemon {
                species: "meowthgalar".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 55,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 40
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "toughclaws".to_string(),
                    hidden: "unnerve".to_string()
                },
                types: (
                    PokemonTypes::Steel,
                    PokemonTypes::Typeless
                ),
                weight: 7.5 as f32,
            }
        );
        
        pokedex.insert(
            "ponytagalar".to_string(),
            PokedexPokemon {
                species: "ponytagalar".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 85,
                    defense: 22,
                    special_attack: 65,
                    special_defense: 65,
                    speed: 90
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "pastelveil".to_string(),
                    hidden: "anticipation".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 24 as f32,
            }
        );
        
        pokedex.insert(
            "rapidashgalar".to_string(),
            PokedexPokemon {
                species: "rapidashgalar".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 100,
                    defense: 70,
                    special_attack: 80,
                    special_defense: 80,
                    speed: 105
                },
                abilities: Abilities {
                    first: "runaway".to_string(),
                    second: "pastelveil".to_string(),
                    hidden: "anticipation".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Fairy
                ),
                weight: 80 as f32,
            }
        );
        
        pokedex.insert(
            "farfetchdgalar".to_string(),
            PokedexPokemon {
                species: "farfetchdgalar".to_string(),
                base_stats: BaseStats {
                    hp: 52,
                    attack: 95,
                    defense: 55,
                    special_attack: 58,
                    special_defense: 62,
                    speed: 55
                },
                abilities: Abilities {
                    first: "steadfast".to_string(),
                    second: "none".to_string(),
                    hidden: "scrappy".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 15 as f32,
            }
        );
        
        pokedex.insert(
            "weezinggalar".to_string(),
            PokedexPokemon {
                species: "weezinggalar".to_string(),
                base_stats: BaseStats {
                    hp: 65,
                    attack: 90,
                    defense: 120,
                    special_attack: 85,
                    special_defense: 70,
                    speed: 60
                },
                abilities: Abilities {
                    first: "levitate".to_string(),
                    second: "neutralizinggas".to_string(),
                    hidden: "mistysurge".to_string()
                },
                types: (
                    PokemonTypes::Poison,
                    PokemonTypes::Fairy
                ),
                weight: 16 as f32,
            }
        );
        
        pokedex.insert(
            "mrmimegalar".to_string(),
            PokedexPokemon {
                species: "mrmimegalar".to_string(),
                base_stats: BaseStats {
                    hp: 50,
                    attack: 65,
                    defense: 65,
                    special_attack: 90,
                    special_defense: 90,
                    speed: 100
                },
                abilities: Abilities {
                    first: "vitalspirit".to_string(),
                    second: "screencleaner".to_string(),
                    hidden: "icebody".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Psychic
                ),
                weight: 56.8 as f32,
            }
        );
        
        pokedex.insert(
            "corsolagalar".to_string(),
            PokedexPokemon {
                species: "corsolagalar".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 55,
                    defense: 100,
                    special_attack: 65,
                    special_defense: 100,
                    speed: 30
                },
                abilities: Abilities {
                    first: "weakarmor".to_string(),
                    second: "none".to_string(),
                    hidden: "cursedbody".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 0.5 as f32,
            }
        );
        
        pokedex.insert(
            "linoonegalar".to_string(),
            PokedexPokemon {
                species: "linoonegalar".to_string(),
                base_stats: BaseStats {
                    hp: 78,
                    attack: 70,
                    defense: 61,
                    special_attack: 50,
                    special_defense: 61,
                    speed: 100
                },
                abilities: Abilities {
                    first: "pickup".to_string(),
                    second: "gluttony".to_string(),
                    hidden: "quickfeet".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Normal
                ),
                weight: 32.5 as f32,
            }
        );
        
        pokedex.insert(
            "darumakagalar".to_string(),
            PokedexPokemon {
                species: "darumakagalar".to_string(),
                base_stats: BaseStats {
                    hp: 70,
                    attack: 90,
                    defense: 45,
                    special_attack: 15,
                    special_defense: 45,
                    speed: 50
                },
                abilities: Abilities {
                    first: "hustle".to_string(),
                    second: "none".to_string(),
                    hidden: "innerfocus".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 40 as f32,
            }
        );
        
        pokedex.insert(
            "darmanitangalar".to_string(),
            PokedexPokemon {
                species: "darmanitangalar".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 140,
                    defense: 55,
                    special_attack: 30,
                    special_defense: 55,
                    speed: 95
                },
                abilities: Abilities {
                    first: "gorillatactics".to_string(),
                    second: "none".to_string(),
                    hidden: "zenmode".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 120 as f32,
            }
        );
        
        pokedex.insert(
            "yamaskgalar".to_string(),
            PokedexPokemon {
                species: "yamaskgalar".to_string(),
                base_stats: BaseStats {
                    hp: 38,
                    attack: 55,
                    defense: 85,
                    special_attack: 30,
                    special_defense: 65,
                    speed: 30
                },
                abilities: Abilities {
                    first: "wanderingspirit".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Ghost
                ),
                weight: 1.5 as f32,
            }
        );
        
        pokedex.insert(
            "stunfiskgalar".to_string(),
            PokedexPokemon {
                species: "stunfiskgalar".to_string(),
                base_stats: BaseStats {
                    hp: 109,
                    attack: 81,
                    defense: 99,
                    special_attack: 66,
                    special_defense: 84,
                    speed: 32
                },
                abilities: Abilities {
                    first: "mimicry".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ground,
                    PokemonTypes::Steel
                ),
                weight: 20.5 as f32,
            }
        );
        
        pokedex.insert(
            "urshifu".to_string(),
            PokedexPokemon {
                species: "urshifu".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 130,
                    defense: 100,
                    special_attack: 63,
                    special_defense: 60,
                    speed: 97
                },
                abilities: Abilities {
                    first: "unseenfist".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Dark
                ),
                weight: 105 as f32,
            }
        );
        
        pokedex.insert(
            "urshifurapidstrike".to_string(),
            PokedexPokemon {
                species: "urshifurapidstrike".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 130,
                    defense: 100,
                    special_attack: 63,
                    special_defense: 60,
                    speed: 97
                },
                abilities: Abilities {
                    first: "unseenfist".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Water
                ),
                weight: 105 as f32,
            }
        );
        
        pokedex.insert(
            "urshifugmax".to_string(),
            PokedexPokemon {
                species: "urshifugmax".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 130,
                    defense: 100,
                    special_attack: 63,
                    special_defense: 60,
                    speed: 97
                },
                abilities: Abilities {
                    first: "unseenfist".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Dark
                ),
                weight: 0 as f32,
            }
        );
        
        pokedex.insert(
            "urshifurapidstrikegmax".to_string(),
            PokedexPokemon {
                species: "urshifurapidstrikegmax".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 130,
                    defense: 100,
                    special_attack: 63,
                    special_defense: 60,
                    speed: 97
                },
                abilities: Abilities {
                    first: "unseenfist".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Water
                ),
                weight: 105 as f32,
            }
        );
        
        pokedex.insert(
            "zarude".to_string(),
            PokedexPokemon {
                species: "zarude".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 120,
                    defense: 105,
                    special_attack: 70,
                    special_defense: 95,
                    speed: 105
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Grass
                ),
                weight: 70 as f32,
            }
        );
        
        pokedex.insert(
            "zarudedada".to_string(),
            PokedexPokemon {
                species: "zarudedada".to_string(),
                base_stats: BaseStats {
                    hp: 105,
                    attack: 120,
                    defense: 105,
                    special_attack: 70,
                    special_defense: 95,
                    speed: 105
                },
                abilities: Abilities {
                    first: "leafguard".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dark,
                    PokemonTypes::Grass
                ),
                weight: 70 as f32,
            }
        );
        
        pokedex.insert(
            "slowpokegalar".to_string(),
            PokedexPokemon {
                species: "slowpokegalar".to_string(),
                base_stats: BaseStats {
                    hp: 90,
                    attack: 65,
                    defense: 65,
                    special_attack: 40,
                    special_defense: 40,
                    speed: 15
                },
                abilities: Abilities {
                    first: "gluttony".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Typeless
                ),
                weight: 36 as f32,
            }
        );
        
        pokedex.insert(
            "slowbrogalar".to_string(),
            PokedexPokemon {
                species: "slowbrogalar".to_string(),
                base_stats: BaseStats {
                    hp: 95,
                    attack: 100,
                    defense: 95,
                    special_attack: 100,
                    special_defense: 70,
                    speed: 30
                },
                abilities: Abilities {
                    first: "quickdraw".to_string(),
                    second: "owntempo".to_string(),
                    hidden: "regenerator".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Poison
                ),
                weight: 70.5 as f32,
            }
        );
        
        pokedex.insert(
            "kubfu".to_string(),
            PokedexPokemon {
                species: "kubfu".to_string(),
                base_stats: BaseStats {
                    hp: 60,
                    attack: 90,
                    defense: 60,
                    special_attack: 53,
                    special_defense: 50,
                    speed: 72
                },
                abilities: Abilities {
                    first: "innerfocus".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Fighting,
                    PokemonTypes::Typeless
                ),
                weight: 12 as f32,
            }
        );
        
        pokedex.insert(
            "regieleki".to_string(),
            PokedexPokemon {
                species: "regieleki".to_string(),
                base_stats: BaseStats {
                    hp: 80,
                    attack: 100,
                    defense: 50,
                    special_attack: 100,
                    special_defense: 50,
                    speed: 200
                },
                abilities: Abilities {
                    first: "transistor".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Electric,
                    PokemonTypes::Typeless
                ),
                weight: 145 as f32,
            }
        );
        
        pokedex.insert(
            "regidrago".to_string(),
            PokedexPokemon {
                species: "regidrago".to_string(),
                base_stats: BaseStats {
                    hp: 200,
                    attack: 100,
                    defense: 50,
                    special_attack: 100,
                    special_defense: 50,
                    speed: 80
                },
                abilities: Abilities {
                    first: "dragonsmaw".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Dragon,
                    PokemonTypes::Typeless
                ),
                weight: 200 as f32,
            }
        );
        
        pokedex.insert(
            "glastrier".to_string(),
            PokedexPokemon {
                species: "glastrier".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 145,
                    defense: 130,
                    special_attack: 65,
                    special_defense: 110,
                    speed: 30
                },
                abilities: Abilities {
                    first: "chillingneigh".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ice,
                    PokemonTypes::Typeless
                ),
                weight: 800 as f32,
            }
        );
        
        pokedex.insert(
            "spectrier".to_string(),
            PokedexPokemon {
                species: "spectrier".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 65,
                    defense: 60,
                    special_attack: 145,
                    special_defense: 80,
                    speed: 130
                },
                abilities: Abilities {
                    first: "grimneigh".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Ghost,
                    PokemonTypes::Typeless
                ),
                weight: 44.5 as f32,
            }
        );
        
        pokedex.insert(
            "calyrex".to_string(),
            PokedexPokemon {
                species: "calyrex".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 80,
                    defense: 80,
                    special_attack: 80,
                    special_defense: 80,
                    speed: 80
                },
                abilities: Abilities {
                    first: "unnerve".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Grass
                ),
                weight: 7.7 as f32,
            }
        );
        
        pokedex.insert(
            "calyrexice".to_string(),
            PokedexPokemon {
                species: "calyrexice".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 165,
                    defense: 150,
                    special_attack: 85,
                    special_defense: 130,
                    speed: 50
                },
                abilities: Abilities {
                    first: "asone(glastrier)".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Ice
                ),
                weight: 809.1 as f32,
            }
        );
        
        pokedex.insert(
            "calyrexshadow".to_string(),
            PokedexPokemon {
                species: "calyrexshadow".to_string(),
                base_stats: BaseStats {
                    hp: 100,
                    attack: 85,
                    defense: 80,
                    special_attack: 165,
                    special_defense: 100,
                    speed: 150
                },
                abilities: Abilities {
                    first: "asone(spectrier)".to_string(),
                    second: "none".to_string(),
                    hidden: "none".to_string()
                },
                types: (
                    PokemonTypes::Psychic,
                    PokemonTypes::Ghost
                ),
                weight: 56.3 as f32,
            }
        );

        pokedex
    };
}

pub fn get_pkmn(pkmn_name: &str) -> &'static PokedexPokemon {
    return POKEDEX.get(pkmn_name).unwrap_or_else(
        || panic!("Could not get pkmn for {}", pkmn_name)
    )
}

pub struct Abilities {
    pub first: String,
    pub second: String,
    pub hidden: String,
}

pub struct BaseStats {
    pub hp: i16,
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
}

pub struct PokedexPokemon {
    pub species: String,
    pub weight: f32,
    pub types: (PokemonTypes, PokemonTypes),
    pub base_stats: BaseStats,
    pub abilities: Abilities,
}
