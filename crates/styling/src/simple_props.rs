use crate::{PreBaseState, PreStyleBase, StyleBaseState};

use super::Style;
use std::fmt::Display;

use paste::paste;

use ident_case::RenameRule::KebabCase;

#[derive(Hash, Eq, PartialEq)]
pub enum SimpleAttribute {
    BackgroundBlendMode(BlendMode),
    BackgroundRepeat(Repeat),
    BackgroundAttachment(Attachment),
    BackgroundOrigin(Origin),
    BackgroundClip(Origin),
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    AlignSelf(AlignSelf),
    All(All),
    Position(Position),
    BoxDecorationBreak(BoxDecorationBreak),
    BoxSizing(BoxSizing),
}

impl Display for SimpleAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::BackgroundBlendMode(x) => {
                format!("background-blend-mode:{x};")
            }
            Self::AlignContent(x) => format!("align-content:{x};"),
            Self::AlignItems(x) => format!("align-items:{x};"),
            Self::AlignSelf(x) => format!("align-self:{x};"),
            Self::All(x) => format!("all:{x};"),
            Self::BoxDecorationBreak(x) => format!("box-decoration-break:{x};"),
            Self::Position(x) => format!("position:{x};"),
            Self::BoxSizing(x) => format!("box-sizing:{x};"),
            Self::BackgroundRepeat(x) => format!("background-repeat:{x};"),
            Self::BackgroundAttachment(x) => format!("background-attachment:{x};"),
            Self::BackgroundOrigin(x) => format!("background-origin:{x};"),
            Self::BackgroundClip(x) => format!("background-clip:{x};"),
        };
        write!(f, "{}", result)
    }
}

macro_rules! simple_property {
    ($name:ident:$($varient:ident)|+) => {
        paste! {
            #[allow(clippy::enum_variant_names)]
            #[derive(Hash, Eq, PartialEq)]
            pub enum [<$name:camel>] {
                $([<$varient:camel>],)*
            }

            impl Display for [<$name:camel>]{
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let result = match self {
                        $(
                            [<$name:camel>]::[<$varient:camel>] => KebabCase.apply_to_variant(stringify!([<$varient:camel>])),
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
simple_property!(
    align_content:
    stretch
        | center
        | flex_start
        | flex_end
        | space_between
        | space_around
        | space_evenly
        | initial
        | inherit
);
simple_property!(
    align_items:
    stretch | center | flex_start | flex_end | start | end | baseline | initial | inherit
);
simple_property!(
    align_self:
    auto | stretch | center | flex_start | flex_end | baseline | initial | inherit
);
simple_property!(all: initial | inherit | unset);
simple_property!(
    position:
    static_ | relative | fixed | absolute | sticky | initial | inherit
);
simple_property!(
    repeat:
    repeat_x | repeat_y | no_repeat | space | round | initial | inherit
);
simple_property!(attachment: scroll | fixed | local | initial | inherit);
simple_property!(
    origin:
    padding_box | border_box | content_box | initial | inherit
);
simple_property!(
    blend_mode:
    normal
        | multiply
        | screen
        | overlay
        | darken
        | lighten
        | color_dodge
        | saturation
        | color
        | luminosity
);
simple_property!(
    box_decoration_break:
    slice | clone | initial | inherit | unset
);
simple_property!(box_sizing:content_box|border_box|initial|inherit);
