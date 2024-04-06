#![allow(dead_code)]
use poke_engine::io;
use std::process::exit;

extern crate lazy_static;

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

fn main() {
    io::main();
    exit(1);
}
