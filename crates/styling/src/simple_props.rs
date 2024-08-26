use crate::{
    attribute::Attribute, background::BackgroundBaseState, AttributeGetter, PreBaseState,
    StyleBaseState,
};

use super::Style;
use crate::attribute::ToAttribute;
use std::fmt::Display;

use paste::paste;

use ident_case::RenameRule::KebabCase;

macro_rules! define_properties {
    ($($name:ident):+) => {
        paste!{
            #[derive(Hash, Eq, PartialEq)]
            pub enum SimpleAttribute {
                $(
                    [<$name:camel>]([<$name:camel>]),
                )*
            }

            impl Display for SimpleAttribute {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let result = match self {
                        $(
                            Self::[<$name:camel>](x) => format!("{}:{};",KebabCase.apply_to_variant(stringify!($name)),x),
                        )*
                    };
                    write!(f, "{}", result)
                }

            }
        }
    };
}

impl Style<StyleBaseState<()>> {
    pub fn align_content(self) -> Style<StyleBaseState<AttributeGetter<AlignContent>>> {
        self.help(Box::new(ToAttribute::attribute))
    }

    pub fn align_items(self) -> Style<StyleBaseState<AttributeGetter<AlignItems>>> {
        self.help(Box::new(ToAttribute::attribute))
    }

    pub fn align_self(self) -> Style<StyleBaseState<AttributeGetter<AlignSelf>>> {
        self.help(Box::new(ToAttribute::attribute))
    }

    pub fn all(self) -> Style<StyleBaseState<AttributeGetter<All>>> {
        self.help(Box::new(ToAttribute::attribute))
    }

    pub fn position(self) -> Style<StyleBaseState<AttributeGetter<Position>>> {
        self.help(Box::new(ToAttribute::attribute))
    }

    pub fn box_decoration_break(
        self,
    ) -> Style<StyleBaseState<AttributeGetter<BoxDecorationBreak>>> {
        self.help(Box::new(ToAttribute::attribute))
    }

    pub fn box_sizing(self) -> Style<StyleBaseState<AttributeGetter<BoxSizing>>> {
        self.help(Box::new(ToAttribute::attribute))
    }
}

impl Style<BackgroundBaseState<()>> {
    pub fn repeat(self) -> Style<BackgroundBaseState<AttributeGetter<BackgroundRepeat>>> {
        self.pre_base(Box::new(ToAttribute::attribute))
    }

    pub fn origin(self) -> Style<BackgroundBaseState<AttributeGetter<BackgroundOrigin>>> {
        self.pre_base(Box::new(ToAttribute::attribute))
    }

    pub fn clip(self) -> Style<BackgroundBaseState<AttributeGetter<BackgroundClip>>> {
        self.pre_base(Box::new(ToAttribute::attribute))
    }

    pub fn blend_mode(self) -> Style<BackgroundBaseState<AttributeGetter<BackgroundBlendMode>>> {
        self.pre_base(Box::new(ToAttribute::attribute))
    }

    pub fn attachment(self) -> Style<BackgroundBaseState<AttributeGetter<BackgroundAttachment>>> {
        self.pre_base(Box::new(ToAttribute::attribute))
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

            impl ToAttribute for [<$name:camel>] {
                fn attribute(self) -> Attribute {
                    Attribute::SimpleAttribute(SimpleAttribute::[<$name:camel>](self))
                }
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


            impl Style<StyleBaseState<AttributeGetter<[<$name:camel>]>>> {
                $(
                    pub fn $varient(self) -> Style<StyleBaseState<()>> {
                        self.base([<$name:camel>]::[<$varient:camel>])
                    }
                )*
            }

        }
    };
}

define_properties!(
    box_decoration_break:
    box_sizing:
    align_content:
    align_items:
    align_self:
    all:
    background_attachment:
    background_repeat:
    background_origin:
    background_clip:
    background_blend_mode:
    position
);

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
    background_repeat:
    repeat_x | repeat_y | no_repeat | space | round | initial | inherit
);
simple_property!(background_attachment: scroll | fixed | local | initial | inherit);
simple_property!(
    background_origin:
    padding_box | border_box | content_box | initial | inherit
);
simple_property!(
    background_clip:
    padding_box | border_box | content_box | initial | inherit
);
simple_property!(
    background_blend_mode:
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
