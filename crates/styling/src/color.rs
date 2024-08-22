use std::fmt::Display;

use crate::{
    background::{BackgroundBaseState, PreBackgroundBase},
    PreBaseState, PreStyleBase, StyleBaseState,
};
use std::stringify;

use super::Style;

use paste::paste;

macro_rules! color_define {
    ($($color:ident),+) => {
        paste! {
        #[derive(Hash, Eq, PartialEq)]
        pub enum Color {
            Hex(u32),
            THex(u32),
            Rgb(u8, u8, u8),
            Rgba(u8, u8, u8, u8),
            Hsl(u16, u8, u8),
            Hsla(u16, u8, u8, u8)
            $(,$color)*
        }

        impl Display for Color {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let result = match self {
                    Color::Hex(c) => {
                        let result = format!("{c:#06x}")[2..].to_string();
                        format!("#{result}")
                    }
                    Color::THex(c) => {
                        let result = format!("{c:#08x}")[2..].to_string();
                        format!("#{result}")
                    }
                    Color::Rgb(red, green, blue) => format!("rgb({red},{green},{blue})"),
                    Color::Rgba(red, green, blue, opacity) => {
                        format!("rgba({red},{green},{blue},{})", *opacity as f32 / 100.)
                    }
                    Color::Hsl(hue, saturation, lightness) => {
                        format!("hsl({hue},{saturation}%,{lightness}%)")
                    }
                    Color::Hsla(hue, saturation, lightness, opacity) => {
                        format!(
                            "hsl({hue},{saturation}%,{lightness}%,{})",
                            *opacity as f32 / 100.
                        )
                    }
                    $(
                        Color::$color =>  stringify!([<$color:lower>]).to_string(),
                    )*
                    };
                write!(f, "{}", result)
            }
        }
        }
    };
}

macro_rules! color_impl {
    ($target:ident,$output:ident,$($color:ident),+) => {
        paste! {
        impl Style<$target<Color>> {
            pub fn hex(self, hex: u32) -> Style<$output> {
                self.base(Color::Hex(hex))
            }

            pub fn t_hex(self, hex: u32) -> Style<$output> {
                self.base(Color::THex(hex))
            }

            pub fn rgb(self, red: u8, green: u8, blue: u8) -> Style<$output> {
                self.base(Color::Rgb(red, green, blue))
            }

            pub fn rgba(self, red: u8, green: u8, blue: u8, opacity: u8) -> Style<$output> {
                debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
                self.base(Color::Rgba(red, green, blue, opacity))
            }

            pub fn hsl(self, hue: u16, saturation: u8, lightness: u8) -> Style<$output> {
                debug_assert!(hue <= 360, "hue should be from 0 to 360");
                debug_assert!(saturation <= 100, "saturation should be from 0 to 100");
                debug_assert!(lightness <= 100, "lightness should be from 0 to 100");
                self.base(Color::Hsl(hue, saturation, lightness))
            }

            pub fn hsla(
                self,
                hue: u16,
                saturation: u8,
                lightness: u8,
                opacity: u8,
            ) -> Style<$output> {
                debug_assert!(hue <= 360, "hue should be from 0 to 360");
                debug_assert!(saturation <= 100, "saturation should be from 0 to 100");
                debug_assert!(lightness <= 100, "lightness should be from 0 to 100");
                debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
                self.base(Color::Hsla(hue, saturation, lightness, opacity))
            }

            $(
                pub fn [<$color:lower>](self) -> Style<$output> {
                    self.base(Color::[<$color>])
                }
            )*
        }
        }
    };
}

// PreStyleBase,
// StyleBaseState,
color_define!(
    ALICEBLUE,
    ANTIQUEWHITE,
    AQUA,
    AQUAMARINE,
    AZURE,
    BEIGE,
    BISQUE,
    BLACK,
    BLANCHEDALMOND,
    BLUE,
    BLUEVIOLET,
    BROWN,
    BURLYWOOD,
    CADETBLUE,
    CHARTREUSE,
    CHOCOLATE,
    CORAL,
    CORNFLOWERBLUE,
    CORNSILK,
    CRIMSON,
    CYAN,
    DARKBLUE,
    DARKCYAN,
    DARKGOLDENROD,
    DARKGREY,
    DARKGREEN,
    DARKKHAKI,
    DARKMAGENTA,
    DARKOLIVEGREEN,
    DARKORANGE,
    DARKORCHID,
    DARKRED,
    DARKSALMON,
    DARKSEAGREEN,
    DARKSLATEBLUE,
    DARKSLATEGREY,
    DARKTURQUOISE,
    DARKVIOLET,
    DEEPPINK,
    DEEPSKYBLUE,
    DIMGRAY,
    DODGERBLUE,
    FIREBRICK,
    FLORALWHITE,
    FORESTGREEN,
    FUCHSIA,
    GAINSBORO,
    GHOSTWHITE,
    GOLD,
    GOLDENROD,
    GREY,
    GREEN,
    GREENYELLOW,
    HONEYDEW,
    HOTPINK,
    INDIANRED,
    INDIGO,
    IVORY,
    KHAKI,
    LAVENDER,
    LAVENDERBLUSH,
    LAWNGREEN,
    LEMONCHIFFON,
    LIGHTBLUE,
    LIGHTCORAL,
    LIGHTCYAN,
    LIGHTGOLDENRODYELLOW,
    LIGHTGREY,
    LIGHTGREEN,
    LIGHTPINK,
    LIGHTSALMON,
    LIGHTSEAGREEN,
    LIGHTSKYBLUE,
    LIGHTSLATEGREY,
    LIGHTSTEELBLUE,
    LIGHTYELLOW,
    LIME,
    LIMEGREEN,
    LINEN,
    MAGENTA,
    MAROON,
    MEDIUMAQUAMARINE,
    MEDIUMBLUE,
    MEDIUMORCHID,
    MEDIUMPURPLE,
    MEDIUMSEAGREEN,
    MEDIUMSLATEBLUE,
    MEDIUMSPRINGGREEN,
    MEDIUMTURQUOISE,
    MEDIUMVIOLETRED,
    MIDNIGHTBLUE,
    MINTCREAM,
    MISTYROSE,
    MOCCASIN,
    NAVAJOWHITE,
    NAVY,
    OLDLACE,
    OLIVE,
    OLIVEDRAB,
    ORANGE,
    ORANGERED,
    ORCHID,
    PALEGOLDENROD,
    PALEGREEN,
    PALETURQUOISE,
    PALEVIOLETRED,
    PAPAYAWHIP,
    PEACHPUFF,
    PERU,
    PINK,
    PLUM,
    POWDERBLUE,
    PURPLE,
    RED,
    ROSYBROWN,
    ROYALBLUE,
    SADDLEBROWN,
    SALMON,
    SANDYBROWN,
    SEAGREEN,
    SEASHELL,
    SIENNA,
    SILVER,
    SKYBLUE,
    SLATEBLUE,
    SLATEGREY,
    SNOW,
    SPRINGGREEN,
    STEELBLUE,
    TAN,
    TEAL,
    THISTLE,
    TOMATO,
    TURQUOISE,
    VIOLET,
    WHEAT,
    WHITE,
    WHITESMOKE,
    YELLOW,
    YELLOWGREEN
);

macro_rules! colors_impls {
    ($($color:ident),+) => {
        color_impl!(
            PreBackgroundBase,
            BackgroundBaseState
            $(,$color)*
        );
        color_impl!(
            PreStyleBase,
            StyleBaseState
            $(,$color)*
        );
    };
}

colors_impls!(
    ALICEBLUE,
    ANTIQUEWHITE,
    AQUA,
    AQUAMARINE,
    AZURE,
    BEIGE,
    BISQUE,
    BLACK,
    BLANCHEDALMOND,
    BLUE,
    BLUEVIOLET,
    BROWN,
    BURLYWOOD,
    CADETBLUE,
    CHARTREUSE,
    CHOCOLATE,
    CORAL,
    CORNFLOWERBLUE,
    CORNSILK,
    CRIMSON,
    CYAN,
    DARKBLUE,
    DARKCYAN,
    DARKGOLDENROD,
    DARKGREY,
    DARKGREEN,
    DARKKHAKI,
    DARKMAGENTA,
    DARKOLIVEGREEN,
    DARKORANGE,
    DARKORCHID,
    DARKRED,
    DARKSALMON,
    DARKSEAGREEN,
    DARKSLATEBLUE,
    DARKSLATEGREY,
    DARKTURQUOISE,
    DARKVIOLET,
    DEEPPINK,
    DEEPSKYBLUE,
    DIMGRAY,
    DODGERBLUE,
    FIREBRICK,
    FLORALWHITE,
    FORESTGREEN,
    FUCHSIA,
    GAINSBORO,
    GHOSTWHITE,
    GOLD,
    GOLDENROD,
    GREY,
    GREEN,
    GREENYELLOW,
    HONEYDEW,
    HOTPINK,
    INDIANRED,
    INDIGO,
    IVORY,
    KHAKI,
    LAVENDER,
    LAVENDERBLUSH,
    LAWNGREEN,
    LEMONCHIFFON,
    LIGHTBLUE,
    LIGHTCORAL,
    LIGHTCYAN,
    LIGHTGOLDENRODYELLOW,
    LIGHTGREY,
    LIGHTGREEN,
    LIGHTPINK,
    LIGHTSALMON,
    LIGHTSEAGREEN,
    LIGHTSKYBLUE,
    LIGHTSLATEGREY,
    LIGHTSTEELBLUE,
    LIGHTYELLOW,
    LIME,
    LIMEGREEN,
    LINEN,
    MAGENTA,
    MAROON,
    MEDIUMAQUAMARINE,
    MEDIUMBLUE,
    MEDIUMORCHID,
    MEDIUMPURPLE,
    MEDIUMSEAGREEN,
    MEDIUMSLATEBLUE,
    MEDIUMSPRINGGREEN,
    MEDIUMTURQUOISE,
    MEDIUMVIOLETRED,
    MIDNIGHTBLUE,
    MINTCREAM,
    MISTYROSE,
    MOCCASIN,
    NAVAJOWHITE,
    NAVY,
    OLDLACE,
    OLIVE,
    OLIVEDRAB,
    ORANGE,
    ORANGERED,
    ORCHID,
    PALEGOLDENROD,
    PALEGREEN,
    PALETURQUOISE,
    PALEVIOLETRED,
    PAPAYAWHIP,
    PEACHPUFF,
    PERU,
    PINK,
    PLUM,
    POWDERBLUE,
    PURPLE,
    RED,
    ROSYBROWN,
    ROYALBLUE,
    SADDLEBROWN,
    SALMON,
    SANDYBROWN,
    SEAGREEN,
    SEASHELL,
    SIENNA,
    SILVER,
    SKYBLUE,
    SLATEBLUE,
    SLATEGREY,
    SNOW,
    SPRINGGREEN,
    STEELBLUE,
    TAN,
    TEAL,
    THISTLE,
    TOMATO,
    TURQUOISE,
    VIOLET,
    WHEAT,
    WHITE,
    WHITESMOKE,
    YELLOW,
    YELLOWGREEN
);
