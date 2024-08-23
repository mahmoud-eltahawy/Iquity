use crate::{PreBaseState, PreStyleBase, StyleBaseState};

use super::Style;
use std::fmt::Display;

use paste::paste;

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
                            [<$name:camel>]::[<$varient:camel>] => stringify!([<$varient:camel:lower>]),
                        )*
                    };
                    write!(f, stringify!($name:{};),result)
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

simple_property!(position, static_, relative, fixed, absolute, sticky);
