#[cfg(feature = "gen2")]
#[path = "gen2/abilities.rs"]
pub mod abilities;
#[cfg(feature = "gen2")]
#[path = "gen2/choice_effects.rs"]
pub mod choice_effects;
#[cfg(feature = "gen2")]
#[path = "gen2/damage_calc.rs"]
mod damage_calc;
#[cfg(feature = "gen2")]
#[path = "gen2/evaluate.rs"]
pub mod evaluate;
#[cfg(feature = "gen2")]
#[path = "gen2/generate_instructions.rs"]
pub mod generate_instructions;
#[cfg(feature = "gen2")]
#[path = "gen2/items.rs"]
pub mod items;
#[cfg(feature = "gen2")]
#[path = "gen2/state.rs"]
pub mod state;

#[cfg(not(feature = "gen2"))]
pub mod abilities;
#[cfg(not(feature = "gen2"))]
pub mod choice_effects;
#[cfg(not(feature = "gen2"))]
mod damage_calc;
#[cfg(not(feature = "gen2"))]
pub mod evaluate;
#[cfg(not(feature = "gen2"))]
pub mod generate_instructions;
#[cfg(not(feature = "gen2"))]
pub mod items;
#[cfg(not(feature = "gen2"))]
pub mod state;

pub mod choices;
pub mod instruction;
pub mod io;
pub mod mcts;
pub mod pokemon;
pub mod search;
pub mod serialize;

#[macro_export]
macro_rules! assert_unique_feature {
    () => {};
    ($first:tt $(,$rest:tt)*) => {
        $(
            #[cfg(all(feature = $first, feature = $rest))]
            compile_error!(concat!("features \"", $first, "\" and \"", $rest, "\" cannot be used together"));
        )*
        assert_unique_feature!($($rest),*);
    }
}

#[macro_export]
macro_rules! define_enum_with_from_str {
    // Case when a default variant is provided
    (
        $(#[$meta:meta])*
        $name:ident {
            $($variant:ident),+ $(,)?
        },
        default = $default_variant:ident
    ) => {
        $(#[$meta])*
        pub enum $name {
            $($variant),+
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input.to_uppercase().as_str() {
                    $(
                        stringify!($variant) => Ok($name::$variant),
                    )+
                    _ => Ok($name::$default_variant),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };

    // Case when no default variant is provided
    (
        $(#[$meta:meta])*
        $name:ident {
            $($variant:ident),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        pub enum $name {
            $($variant),+
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input.to_uppercase().as_str() {
                    $(
                        stringify!($variant) => Ok($name::$variant),
                    )+
                    _ => panic!("Invalid {}: {}", stringify!($name), input.to_uppercase().as_str()),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

assert_unique_feature!("gen2", "gen3", "gen4", "gen5", "gen6", "gen7", "gen8", "gen9");
