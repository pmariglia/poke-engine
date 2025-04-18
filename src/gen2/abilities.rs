#![allow(unused_variables)]
use crate::define_enum_with_from_str;

define_enum_with_from_str! {
    #[repr(i16)]
    #[derive(PartialEq, Debug, Clone, Copy)]
    Abilities {
        NONE,
        NOABILITY,
    },
    default = NOABILITY
}
