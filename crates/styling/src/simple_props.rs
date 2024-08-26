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

macro_rules! simple_property {
    ($target:ident,$name:ident:$($varient:ident)|+) => {
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

            impl Style<$target<()>> {
                pub fn $name(self) -> Style<$target<AttributeGetter<[<$name:camel>]>>> {
                    self.into_prebase(Box::new(ToAttribute::attribute))
                }
            }


            impl Style<$target<AttributeGetter<[<$name:camel>]>>> {
                $(
                    pub fn $varient(self) -> Style<$target<()>> {
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
    StyleBaseState,
    align_content:stretch|center|flex_start|flex_end|space_between|space_around|space_evenly|initial|inherit);
simple_property!(
    StyleBaseState,
    align_items:stretch|center|flex_start|flex_end|start|end|baseline|initial|inherit);
simple_property!(
    StyleBaseState,
    align_self:auto|stretch|center|flex_start|flex_end|baseline|initial|inherit);
simple_property!(
    StyleBaseState,
    all:initial|inherit|unset);
simple_property!(
    StyleBaseState,
    position:static_|relative|fixed|absolute|sticky|initial|inherit);
simple_property!(
    StyleBaseState,
    box_decoration_break:slice|clone|initial|inherit|unset);
simple_property!(
    StyleBaseState,
    box_sizing:content_box|border_box|initial|inherit);

simple_property!(
    BackgroundBaseState,
    background_repeat:
    repeat_x|repeat_y|no_repeat|space|round|initial|inherit);
simple_property!(
    BackgroundBaseState,
    background_origin:padding_box|border_box|content_box|initial|inherit);
simple_property!(
    BackgroundBaseState,
    background_clip:padding_box|border_box|content_box|initial|inherit);
simple_property!(
    BackgroundBaseState,
    background_blend_mode:normal|multiply|screen|overlay|darken|lighten|color_dodge|saturation|color|luminosity);
simple_property!(
    BackgroundBaseState,
    background_attachment:scroll|fixed|local|initial|inherit);
