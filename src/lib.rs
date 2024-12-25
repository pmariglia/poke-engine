#[cfg(feature = "gen1")]
#[path = "gen1/abilities.rs"]
pub mod abilities;
#[cfg(feature = "gen1")]
#[path = "gen1/base_stats.rs"]
pub mod base_stats;
#[cfg(feature = "gen1")]
#[path = "gen1/choice_effects.rs"]
pub mod choice_effects;
#[cfg(feature = "gen1")]
#[path = "gen1/damage_calc.rs"]
pub mod damage_calc;
#[cfg(feature = "gen1")]
#[path = "gen1/evaluate.rs"]
pub mod evaluate;
#[cfg(feature = "gen1")]
#[path = "gen1/generate_instructions.rs"]
pub mod generate_instructions;
#[cfg(feature = "gen1")]
#[path = "gen1/items.rs"]
pub mod items;
#[cfg(feature = "gen1")]
#[path = "gen1/state.rs"]
pub mod state;

#[cfg(feature = "gen2")]
#[path = "gen2/abilities.rs"]
pub mod abilities;
#[cfg(feature = "gen2")]
#[path = "gen2/choice_effects.rs"]
pub mod choice_effects;
#[cfg(feature = "gen2")]
#[path = "gen2/damage_calc.rs"]
pub mod damage_calc;
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

#[cfg(not(any(feature = "gen2", feature = "gen1")))]
pub mod abilities;
#[cfg(not(any(feature = "gen2", feature = "gen1")))]
pub mod choice_effects;
#[cfg(not(any(feature = "gen2", feature = "gen1")))]
pub mod damage_calc;
#[cfg(not(any(feature = "gen2", feature = "gen1")))]
pub mod evaluate;
#[cfg(not(any(feature = "gen2", feature = "gen1")))]
pub mod generate_instructions;
#[cfg(not(any(feature = "gen2", feature = "gen1")))]
pub mod items;
#[cfg(not(any(feature = "gen2", feature = "gen1")))]
pub mod state;

pub mod choices;
pub mod instruction;
pub mod io;
pub mod mcts;
pub mod pokemon;
pub mod search;

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
        #[repr($repr:ident)]
        $(#[$meta:meta])*
        $name:ident {
            $($variant:ident),+ $(,)?
        },
        default = $default_variant:ident
    ) => {
        #[repr($repr)]
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

        impl From<$repr> for $name {
            fn from(value: $repr) -> $name {
                match value {
                    $(
                        x if x == $name::$variant as $repr => $name::$variant,
                    )+
                    _ => $name::$default_variant,
                }
            }
        }
        impl Into<$repr> for $name {
            fn into(self) -> $repr {
                self as $repr
            }
        }
    };

    // Case when no default variant is provided
    (
        #[repr($repr:ident)]
        $(#[$meta:meta])*
        $name:ident {
            $($variant:ident),+ $(,)?
        }
    ) => {
        #[repr($repr)]
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

        impl From<$repr> for $name {
            fn from(value: $repr) -> $name {
                match value {
                    $(
                        x if x == $name::$variant as $repr => $name::$variant,
                    )+
                    _ => panic!("Invalid {}: {}", stringify!($name), value),
                }
            }
        }
        impl Into<$repr> for $name {
            fn into(self) -> $repr {
                self as $repr
            }
        }
    };
}

assert_unique_feature!("gen2", "gen3", "gen4", "gen5", "gen6", "gen7", "gen8", "gen9");
