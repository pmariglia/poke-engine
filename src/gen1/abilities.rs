#![allow(unused_variables)]
use crate::define_enum_with_from_str;

define_enum_with_from_str! {
    #[repr(u8)]
    #[derive(PartialEq, Debug, Clone)]
    Abilities {
        NONE,
        NOABILITY,
    },
    default = NOABILITY
}
