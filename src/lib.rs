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

assert_unique_feature!("gen4", "gen5", "gen6", "gen7", "gen8", "gen9");
