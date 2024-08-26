use std::fmt::Display;

use crate::{AttributeGetter, PreBaseState, StyleBaseState};
use std::stringify;

use super::Style;

use paste::paste;

macro_rules! color_impl {
    ($($color:ident),+) => {
        paste! {
        impl Style<StyleBaseState<AttributeGetter<Color>>> {
            pub fn hex(self, hex: u32) -> Style<StyleBaseState<()>> {
                self.base(Color::Hex(hex))
            }

            pub fn t_hex(self, hex: u32) -> Style<StyleBaseState<()>> {
                self.base(Color::THex(hex))
            }

            pub fn rgb(self, red: u8, green: u8, blue: u8) -> Style<StyleBaseState<()>> {
                self.base(Color::Rgb(red, green, blue))
            }

            pub fn rgba(self, red: u8, green: u8, blue: u8, opacity: u8) -> Style<StyleBaseState<()>> {
                debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
                self.base(Color::Rgba(red, green, blue, opacity))
            }

            pub fn hsl(self, hue: u16, saturation: u8, lightness: u8) -> Style<StyleBaseState<()>> {
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
            ) -> Style<StyleBaseState<()>> {
                debug_assert!(hue <= 360, "hue should be from 0 to 360");
                debug_assert!(saturation <= 100, "saturation should be from 0 to 100");
                debug_assert!(lightness <= 100, "lightness should be from 0 to 100");
                debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
                self.base(Color::Hsla(hue, saturation, lightness, opacity))
            }

            $(
                pub fn [<$color:snake>](self) -> Style<StyleBaseState<()>> {
                    self.base(Color::[<$color>])
                }
            )*
        }
        }
    };
}

macro_rules! color_define {
    ($($color:ident),+) => {
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
                        Color::$color => stringify!($color).to_string(),
                    )*
                };
                write!(f, "{}", result)
            }
        }
        color_impl!($($color),*);
    };
}

color_define!(
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenRod,
    DarkGray,
    DarkGrey,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
    Gray,
    Grey,
    Green,
    GreenYellow,
    HoneyDew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGray,
    LightGrey,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    RebeccaPurple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    SlateGrey,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen
);
