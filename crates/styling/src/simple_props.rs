use crate::{attribute::Attribute, AttributeGetter, PreBaseState, StyleBaseState};

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

            impl Style<StyleBaseState<()>> {
                pub fn $name(self) -> Style<StyleBaseState<AttributeGetter<[<$name:camel>]>>> {
                    self.into_prebase(Box::new(ToAttribute::attribute))
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
    color_scheme:
    clear:
    caption_side:
    break_inside:
    break_before:
    break_after:
    box_decoration_break:
    border_style:
    border_left_style:
    border_right_style:
    border_top_style:
    border_bottom_style:
    border_inline_style:
    border_inline_start_style:
    border_inline_end_style:
    border_collapse:
    border_block_start_style:
    border_block_end_style:
    backface_visibility:
    animation_play_state:
    animation_fill_mode:
    animation_direction:
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
    align_content:stretch|center|flex_start|flex_end|space_between|space_around|space_evenly|initial|inherit);
simple_property!(
    align_items:stretch|center|flex_start|flex_end|start|end|baseline|initial|inherit);
simple_property!(
    align_self:auto|stretch|center|flex_start|flex_end|baseline|initial|inherit);
simple_property!(
    all:initial|inherit|unset);
simple_property!(
    position:static_|relative|fixed|absolute|sticky|initial|inherit);
simple_property!(
    box_decoration_break:slice|clone|initial|inherit|unset);
simple_property!(
    box_sizing:content_box|border_box|initial|inherit);

simple_property!(
    background_repeat:
    repeat_x|repeat_y|no_repeat|space|round|initial|inherit);
simple_property!(
    background_origin:padding_box|border_box|content_box|initial|inherit);
simple_property!(
    background_clip:padding_box|border_box|content_box|initial|inherit);
simple_property!(
    background_blend_mode:normal|multiply|screen|overlay|darken|lighten|color_dodge|saturation|color|luminosity);
simple_property!(
    background_attachment:scroll|fixed|local|initial|inherit);
simple_property!(
    animation_direction: normal|reverse|alternate|alternate_reverse|initial|inherit);
simple_property!(
    animation_fill_mode: none|forwards|backwards|both|initial|inherit);
simple_property!(
    animation_play_state: paused|running|initial|inherit);
simple_property!(
    backface_visibility: visible|hidden|initial|inherit);
simple_property!(
    border_block_end_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_block_start_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_collapse: separate|collapse|initial|inherit);
simple_property!(
    border_inline_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_inline_start_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_inline_end_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_left_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_right_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_top_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    border_bottom_style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit);
simple_property!(
    break_after: auto|all|always|avoid|avoid_column|avoid_page|avoid_region|column|left|page|recto|region|right|verso|initial|inherit);
simple_property!(
    break_before: auto|all|always|avoid|avoid_column|avoid_page|avoid_region|column|left|page|recto|region|right|verso|initial|inherit);
simple_property!(
    break_inside: auto|all|always|avoid|avoid_column|avoid_page|avoid_region|column|left|page|recto|region|right|verso|initial|inherit);
simple_property!(
    caption_side: top|bottom|initial|inherit);
simple_property!(
    clear: none|left|right|both|initial|inherit);
simple_property!(
    color_scheme: normal|light|dark|only);
