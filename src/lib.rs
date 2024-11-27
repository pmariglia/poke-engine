mod damage_calc;

pub mod abilities;
pub mod choice_effects;
pub mod choices;
pub mod evaluate;
pub mod generate_instructions;
pub mod instruction;
pub mod io;
pub mod items;
pub mod mcts;
pub mod pokemon;
pub mod search;
pub mod serialize;
pub mod state;

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
