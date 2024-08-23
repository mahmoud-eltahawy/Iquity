use crate::{PreBaseState, PreStyleBase, StyleBaseState};

use super::Style;
use std::fmt::Display;

use paste::paste;

use ident_case::RenameRule::KebabCase;

macro_rules! simple_property {
    ($name:ident,$($varient:ident),+) => {
        paste! {
            #[derive(Hash, Eq, PartialEq)]
            pub enum [<$name:camel>] {
                $([<$varient:camel>],)*
            }

            impl Display for [<$name:camel>]{
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let result = match self {
                        $(
                            [<$name:camel>]::[<$varient:camel>] => KebabCase.apply_to_variant(stringify!([<$varient>])),
                        )*
                    };
                    write!(f, "{}",result)
                }
            }


            impl Style<PreStyleBase<[<$name:camel>]>> {
                $(
                    pub fn $varient(self) -> Style<StyleBaseState> {
                        self.base([<$name:camel>]::[<$varient:camel>])
                    }
                )*
            }

        }
    };
}

simple_property!(position, static_, relative, fixed, absolute, sticky, initial, inherit);
simple_property!(
    align_content,
    stretch,
    center,
    flex_start,
    flex_end,
    space_between,
    space_around,
    space_evenly,
    initial,
    inherit
);
