use std::fmt::Display;

use crate::{
    background::{BackgroundBaseState, PreBackgroundBase},
    PreStyleBase, StyleBaseState,
};

use super::Style;

#[derive(Hash, Eq, PartialEq)]
pub enum Color {
    Hex(u32),
    THex(u32),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
    Hsl(u16, u8, u8),
    Hsla(u16, u8, u8, u8),
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
    DarkGrey,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    Darkorange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
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
    LightGrey,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
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
    YellowGreen,
}

impl Style<PreStyleBase<Color>> {
    pub fn hex(self, hex: u32) -> Style<StyleBaseState> {
        self.base(Color::Hex(hex))
    }

    pub fn t_hex(self, hex: u32) -> Style<StyleBaseState> {
        self.base(Color::THex(hex))
    }

    pub fn rgb(self, red: u8, green: u8, blue: u8) -> Style<StyleBaseState> {
        self.base(Color::Rgb(red, green, blue))
    }

    pub fn rgba(self, red: u8, green: u8, blue: u8, opacity: u8) -> Style<StyleBaseState> {
        debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
        self.base(Color::Rgba(red, green, blue, opacity))
    }

    pub fn hsl(self, hue: u16, saturation: u8, lightness: u8) -> Style<StyleBaseState> {
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
    ) -> Style<StyleBaseState> {
        debug_assert!(hue <= 360, "hue should be from 0 to 360");
        debug_assert!(saturation <= 100, "saturation should be from 0 to 100");
        debug_assert!(lightness <= 100, "lightness should be from 0 to 100");
        debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
        self.base(Color::Hsla(hue, saturation, lightness, opacity))
    }

    pub fn alice_blue(self) -> Style<StyleBaseState> {
        self.base(Color::AliceBlue)
    }
    pub fn antique_white(self) -> Style<StyleBaseState> {
        self.base(Color::AntiqueWhite)
    }
    pub fn aqua(self) -> Style<StyleBaseState> {
        self.base(Color::Aqua)
    }
    pub fn aquamarine(self) -> Style<StyleBaseState> {
        self.base(Color::Aquamarine)
    }
    pub fn azure(self) -> Style<StyleBaseState> {
        self.base(Color::Azure)
    }
    pub fn beige(self) -> Style<StyleBaseState> {
        self.base(Color::Beige)
    }
    pub fn bisque(self) -> Style<StyleBaseState> {
        self.base(Color::Bisque)
    }
    pub fn black(self) -> Style<StyleBaseState> {
        self.base(Color::Black)
    }
    pub fn blanched_almond(self) -> Style<StyleBaseState> {
        self.base(Color::BlanchedAlmond)
    }
    pub fn blue(self) -> Style<StyleBaseState> {
        self.base(Color::Blue)
    }
    pub fn blue_violet(self) -> Style<StyleBaseState> {
        self.base(Color::BlueViolet)
    }
    pub fn brown(self) -> Style<StyleBaseState> {
        self.base(Color::Brown)
    }
    pub fn burly_wood(self) -> Style<StyleBaseState> {
        self.base(Color::BurlyWood)
    }
    pub fn cadet_blue(self) -> Style<StyleBaseState> {
        self.base(Color::CadetBlue)
    }
    pub fn chartreuse(self) -> Style<StyleBaseState> {
        self.base(Color::Chartreuse)
    }
    pub fn chocolate(self) -> Style<StyleBaseState> {
        self.base(Color::Chocolate)
    }
    pub fn coral(self) -> Style<StyleBaseState> {
        self.base(Color::Coral)
    }
    pub fn cornflower_blue(self) -> Style<StyleBaseState> {
        self.base(Color::CornflowerBlue)
    }
    pub fn cornsilk(self) -> Style<StyleBaseState> {
        self.base(Color::Cornsilk)
    }
    pub fn crimson(self) -> Style<StyleBaseState> {
        self.base(Color::Crimson)
    }
    pub fn cyan(self) -> Style<StyleBaseState> {
        self.base(Color::Cyan)
    }
    pub fn dark_blue(self) -> Style<StyleBaseState> {
        self.base(Color::DarkBlue)
    }
    pub fn dark_cyan(self) -> Style<StyleBaseState> {
        self.base(Color::DarkCyan)
    }
    pub fn dark_golden_rod(self) -> Style<StyleBaseState> {
        self.base(Color::DarkGoldenRod)
    }
    pub fn dark_grey(self) -> Style<StyleBaseState> {
        self.base(Color::DarkGrey)
    }
    pub fn dark_green(self) -> Style<StyleBaseState> {
        self.base(Color::DarkGreen)
    }
    pub fn dark_khaki(self) -> Style<StyleBaseState> {
        self.base(Color::DarkKhaki)
    }
    pub fn dark_magenta(self) -> Style<StyleBaseState> {
        self.base(Color::DarkMagenta)
    }
    pub fn dark_olive_green(self) -> Style<StyleBaseState> {
        self.base(Color::DarkOliveGreen)
    }
    pub fn darkorange(self) -> Style<StyleBaseState> {
        self.base(Color::Darkorange)
    }
    pub fn dark_orchid(self) -> Style<StyleBaseState> {
        self.base(Color::DarkOrchid)
    }
    pub fn dark_red(self) -> Style<StyleBaseState> {
        self.base(Color::DarkRed)
    }
    pub fn dark_salmon(self) -> Style<StyleBaseState> {
        self.base(Color::DarkSalmon)
    }
    pub fn dark_sea_green(self) -> Style<StyleBaseState> {
        self.base(Color::DarkSeaGreen)
    }
    pub fn dark_slate_blue(self) -> Style<StyleBaseState> {
        self.base(Color::DarkSlateBlue)
    }
    pub fn dark_slate_grey(self) -> Style<StyleBaseState> {
        self.base(Color::DarkSlateGrey)
    }
    pub fn dark_turquoise(self) -> Style<StyleBaseState> {
        self.base(Color::DarkTurquoise)
    }
    pub fn dark_violet(self) -> Style<StyleBaseState> {
        self.base(Color::DarkViolet)
    }
    pub fn deep_pink(self) -> Style<StyleBaseState> {
        self.base(Color::DeepPink)
    }
    pub fn deep_sky_blue(self) -> Style<StyleBaseState> {
        self.base(Color::DeepSkyBlue)
    }
    pub fn dim_gray(self) -> Style<StyleBaseState> {
        self.base(Color::DimGray)
    }
    pub fn dodger_blue(self) -> Style<StyleBaseState> {
        self.base(Color::DodgerBlue)
    }
    pub fn fire_brick(self) -> Style<StyleBaseState> {
        self.base(Color::FireBrick)
    }
    pub fn floral_white(self) -> Style<StyleBaseState> {
        self.base(Color::FloralWhite)
    }
    pub fn forest_green(self) -> Style<StyleBaseState> {
        self.base(Color::ForestGreen)
    }
    pub fn fuchsia(self) -> Style<StyleBaseState> {
        self.base(Color::Fuchsia)
    }
    pub fn gainsboro(self) -> Style<StyleBaseState> {
        self.base(Color::Gainsboro)
    }
    pub fn ghost_white(self) -> Style<StyleBaseState> {
        self.base(Color::GhostWhite)
    }
    pub fn gold(self) -> Style<StyleBaseState> {
        self.base(Color::Gold)
    }
    pub fn golden_rod(self) -> Style<StyleBaseState> {
        self.base(Color::GoldenRod)
    }
    pub fn grey(self) -> Style<StyleBaseState> {
        self.base(Color::Grey)
    }
    pub fn green(self) -> Style<StyleBaseState> {
        self.base(Color::Green)
    }
    pub fn green_yellow(self) -> Style<StyleBaseState> {
        self.base(Color::GreenYellow)
    }
    pub fn honey_dew(self) -> Style<StyleBaseState> {
        self.base(Color::HoneyDew)
    }
    pub fn hot_pink(self) -> Style<StyleBaseState> {
        self.base(Color::HotPink)
    }
    pub fn indian_red(self) -> Style<StyleBaseState> {
        self.base(Color::IndianRed)
    }
    pub fn indigo(self) -> Style<StyleBaseState> {
        self.base(Color::Indigo)
    }
    pub fn ivory(self) -> Style<StyleBaseState> {
        self.base(Color::Ivory)
    }
    pub fn khaki(self) -> Style<StyleBaseState> {
        self.base(Color::Khaki)
    }
    pub fn lavender(self) -> Style<StyleBaseState> {
        self.base(Color::Lavender)
    }
    pub fn lavender_blush(self) -> Style<StyleBaseState> {
        self.base(Color::LavenderBlush)
    }
    pub fn lawn_green(self) -> Style<StyleBaseState> {
        self.base(Color::LawnGreen)
    }
    pub fn lemon_chiffon(self) -> Style<StyleBaseState> {
        self.base(Color::LemonChiffon)
    }
    pub fn light_blue(self) -> Style<StyleBaseState> {
        self.base(Color::LightBlue)
    }
    pub fn light_coral(self) -> Style<StyleBaseState> {
        self.base(Color::LightCoral)
    }
    pub fn light_cyan(self) -> Style<StyleBaseState> {
        self.base(Color::LightCyan)
    }
    pub fn light_golden_rod_yellow(self) -> Style<StyleBaseState> {
        self.base(Color::LightGoldenRodYellow)
    }
    pub fn light_grey(self) -> Style<StyleBaseState> {
        self.base(Color::LightGrey)
    }
    pub fn light_green(self) -> Style<StyleBaseState> {
        self.base(Color::LightGreen)
    }
    pub fn light_pink(self) -> Style<StyleBaseState> {
        self.base(Color::LightPink)
    }
    pub fn light_salmon(self) -> Style<StyleBaseState> {
        self.base(Color::LightSalmon)
    }
    pub fn light_sea_green(self) -> Style<StyleBaseState> {
        self.base(Color::LightSeaGreen)
    }
    pub fn light_sky_blue(self) -> Style<StyleBaseState> {
        self.base(Color::LightSkyBlue)
    }
    pub fn light_slate_grey(self) -> Style<StyleBaseState> {
        self.base(Color::LightSlateGrey)
    }
    pub fn light_steel_blue(self) -> Style<StyleBaseState> {
        self.base(Color::LightSteelBlue)
    }
    pub fn light_yellow(self) -> Style<StyleBaseState> {
        self.base(Color::LightYellow)
    }
    pub fn lime(self) -> Style<StyleBaseState> {
        self.base(Color::Lime)
    }
    pub fn lime_green(self) -> Style<StyleBaseState> {
        self.base(Color::LimeGreen)
    }
    pub fn linen(self) -> Style<StyleBaseState> {
        self.base(Color::Linen)
    }
    pub fn magenta(self) -> Style<StyleBaseState> {
        self.base(Color::Magenta)
    }
    pub fn maroon(self) -> Style<StyleBaseState> {
        self.base(Color::Maroon)
    }
    pub fn medium_aqua_marine(self) -> Style<StyleBaseState> {
        self.base(Color::MediumAquaMarine)
    }
    pub fn medium_blue(self) -> Style<StyleBaseState> {
        self.base(Color::MediumBlue)
    }
    pub fn medium_orchid(self) -> Style<StyleBaseState> {
        self.base(Color::MediumOrchid)
    }
    pub fn medium_purple(self) -> Style<StyleBaseState> {
        self.base(Color::MediumPurple)
    }
    pub fn medium_sea_green(self) -> Style<StyleBaseState> {
        self.base(Color::MediumSeaGreen)
    }
    pub fn medium_slate_blue(self) -> Style<StyleBaseState> {
        self.base(Color::MediumSlateBlue)
    }
    pub fn medium_spring_green(self) -> Style<StyleBaseState> {
        self.base(Color::MediumSpringGreen)
    }
    pub fn medium_turquoise(self) -> Style<StyleBaseState> {
        self.base(Color::MediumTurquoise)
    }
    pub fn medium_violet_red(self) -> Style<StyleBaseState> {
        self.base(Color::MediumVioletRed)
    }
    pub fn midnight_blue(self) -> Style<StyleBaseState> {
        self.base(Color::MidnightBlue)
    }
    pub fn mint_cream(self) -> Style<StyleBaseState> {
        self.base(Color::MintCream)
    }
    pub fn misty_rose(self) -> Style<StyleBaseState> {
        self.base(Color::MistyRose)
    }
    pub fn moccasin(self) -> Style<StyleBaseState> {
        self.base(Color::Moccasin)
    }
    pub fn navajo_white(self) -> Style<StyleBaseState> {
        self.base(Color::NavajoWhite)
    }
    pub fn navy(self) -> Style<StyleBaseState> {
        self.base(Color::Navy)
    }
    pub fn old_lace(self) -> Style<StyleBaseState> {
        self.base(Color::OldLace)
    }
    pub fn olive(self) -> Style<StyleBaseState> {
        self.base(Color::Olive)
    }
    pub fn olive_drab(self) -> Style<StyleBaseState> {
        self.base(Color::OliveDrab)
    }
    pub fn orange(self) -> Style<StyleBaseState> {
        self.base(Color::Orange)
    }
    pub fn orange_red(self) -> Style<StyleBaseState> {
        self.base(Color::OrangeRed)
    }
    pub fn orchid(self) -> Style<StyleBaseState> {
        self.base(Color::Orchid)
    }
    pub fn pale_golden_rod(self) -> Style<StyleBaseState> {
        self.base(Color::PaleGoldenRod)
    }
    pub fn pale_green(self) -> Style<StyleBaseState> {
        self.base(Color::PaleGreen)
    }
    pub fn pale_turquoise(self) -> Style<StyleBaseState> {
        self.base(Color::PaleTurquoise)
    }
    pub fn pale_violet_red(self) -> Style<StyleBaseState> {
        self.base(Color::PaleVioletRed)
    }
    pub fn papaya_whip(self) -> Style<StyleBaseState> {
        self.base(Color::PapayaWhip)
    }
    pub fn peach_puff(self) -> Style<StyleBaseState> {
        self.base(Color::PeachPuff)
    }
    pub fn peru(self) -> Style<StyleBaseState> {
        self.base(Color::Peru)
    }
    pub fn pink(self) -> Style<StyleBaseState> {
        self.base(Color::Pink)
    }
    pub fn plum(self) -> Style<StyleBaseState> {
        self.base(Color::Plum)
    }
    pub fn powder_blue(self) -> Style<StyleBaseState> {
        self.base(Color::PowderBlue)
    }
    pub fn purple(self) -> Style<StyleBaseState> {
        self.base(Color::Purple)
    }
    pub fn red(self) -> Style<StyleBaseState> {
        self.base(Color::Red)
    }
    pub fn rosy_brown(self) -> Style<StyleBaseState> {
        self.base(Color::RosyBrown)
    }
    pub fn royal_blue(self) -> Style<StyleBaseState> {
        self.base(Color::RoyalBlue)
    }
    pub fn saddle_brown(self) -> Style<StyleBaseState> {
        self.base(Color::SaddleBrown)
    }
    pub fn salmon(self) -> Style<StyleBaseState> {
        self.base(Color::Salmon)
    }
    pub fn sandy_brown(self) -> Style<StyleBaseState> {
        self.base(Color::SandyBrown)
    }
    pub fn sea_green(self) -> Style<StyleBaseState> {
        self.base(Color::SeaGreen)
    }
    pub fn sea_shell(self) -> Style<StyleBaseState> {
        self.base(Color::SeaShell)
    }
    pub fn sienna(self) -> Style<StyleBaseState> {
        self.base(Color::Sienna)
    }
    pub fn silver(self) -> Style<StyleBaseState> {
        self.base(Color::Silver)
    }
    pub fn sky_blue(self) -> Style<StyleBaseState> {
        self.base(Color::SkyBlue)
    }
    pub fn slate_blue(self) -> Style<StyleBaseState> {
        self.base(Color::SlateBlue)
    }
    pub fn slate_grey(self) -> Style<StyleBaseState> {
        self.base(Color::SlateGrey)
    }
    pub fn snow(self) -> Style<StyleBaseState> {
        self.base(Color::Snow)
    }
    pub fn spring_green(self) -> Style<StyleBaseState> {
        self.base(Color::SpringGreen)
    }
    pub fn steel_blue(self) -> Style<StyleBaseState> {
        self.base(Color::SteelBlue)
    }
    pub fn tan(self) -> Style<StyleBaseState> {
        self.base(Color::Tan)
    }
    pub fn teal(self) -> Style<StyleBaseState> {
        self.base(Color::Teal)
    }
    pub fn thistle(self) -> Style<StyleBaseState> {
        self.base(Color::Thistle)
    }
    pub fn tomato(self) -> Style<StyleBaseState> {
        self.base(Color::Tomato)
    }
    pub fn turquoise(self) -> Style<StyleBaseState> {
        self.base(Color::Turquoise)
    }
    pub fn violet(self) -> Style<StyleBaseState> {
        self.base(Color::Violet)
    }
    pub fn wheat(self) -> Style<StyleBaseState> {
        self.base(Color::Wheat)
    }
    pub fn white(self) -> Style<StyleBaseState> {
        self.base(Color::White)
    }
    pub fn white_smoke(self) -> Style<StyleBaseState> {
        self.base(Color::WhiteSmoke)
    }
    pub fn yellow(self) -> Style<StyleBaseState> {
        self.base(Color::Yellow)
    }

    pub fn yellow_green(self) -> Style<StyleBaseState> {
        self.base(Color::YellowGreen)
    }
}

impl Style<PreBackgroundBase<Color>> {
    pub fn hex(self, hex: u32) -> Style<BackgroundBaseState> {
        self.base(Color::Hex(hex))
    }

    pub fn t_hex(self, hex: u32) -> Style<BackgroundBaseState> {
        self.base(Color::THex(hex))
    }

    pub fn rgb(self, red: u8, green: u8, blue: u8) -> Style<BackgroundBaseState> {
        self.base(Color::Rgb(red, green, blue))
    }

    pub fn rgba(self, red: u8, green: u8, blue: u8, opacity: u8) -> Style<BackgroundBaseState> {
        debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
        self.base(Color::Rgba(red, green, blue, opacity))
    }

    pub fn hsl(self, hue: u16, saturation: u8, lightness: u8) -> Style<BackgroundBaseState> {
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
    ) -> Style<BackgroundBaseState> {
        debug_assert!(hue <= 360, "hue should be from 0 to 360");
        debug_assert!(saturation <= 100, "saturation should be from 0 to 100");
        debug_assert!(lightness <= 100, "lightness should be from 0 to 100");
        debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
        self.base(Color::Hsla(hue, saturation, lightness, opacity))
    }

    pub fn alice_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::AliceBlue)
    }
    pub fn antique_white(self) -> Style<BackgroundBaseState> {
        self.base(Color::AntiqueWhite)
    }
    pub fn aqua(self) -> Style<BackgroundBaseState> {
        self.base(Color::Aqua)
    }
    pub fn aquamarine(self) -> Style<BackgroundBaseState> {
        self.base(Color::Aquamarine)
    }
    pub fn azure(self) -> Style<BackgroundBaseState> {
        self.base(Color::Azure)
    }
    pub fn beige(self) -> Style<BackgroundBaseState> {
        self.base(Color::Beige)
    }
    pub fn bisque(self) -> Style<BackgroundBaseState> {
        self.base(Color::Bisque)
    }
    pub fn black(self) -> Style<BackgroundBaseState> {
        self.base(Color::Black)
    }
    pub fn blanched_almond(self) -> Style<BackgroundBaseState> {
        self.base(Color::BlanchedAlmond)
    }
    pub fn blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::Blue)
    }
    pub fn blue_violet(self) -> Style<BackgroundBaseState> {
        self.base(Color::BlueViolet)
    }
    pub fn brown(self) -> Style<BackgroundBaseState> {
        self.base(Color::Brown)
    }
    pub fn burly_wood(self) -> Style<BackgroundBaseState> {
        self.base(Color::BurlyWood)
    }
    pub fn cadet_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::CadetBlue)
    }
    pub fn chartreuse(self) -> Style<BackgroundBaseState> {
        self.base(Color::Chartreuse)
    }
    pub fn chocolate(self) -> Style<BackgroundBaseState> {
        self.base(Color::Chocolate)
    }
    pub fn coral(self) -> Style<BackgroundBaseState> {
        self.base(Color::Coral)
    }
    pub fn cornflower_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::CornflowerBlue)
    }
    pub fn cornsilk(self) -> Style<BackgroundBaseState> {
        self.base(Color::Cornsilk)
    }
    pub fn crimson(self) -> Style<BackgroundBaseState> {
        self.base(Color::Crimson)
    }
    pub fn cyan(self) -> Style<BackgroundBaseState> {
        self.base(Color::Cyan)
    }
    pub fn dark_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkBlue)
    }
    pub fn dark_cyan(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkCyan)
    }
    pub fn dark_golden_rod(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkGoldenRod)
    }
    pub fn dark_grey(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkGrey)
    }
    pub fn dark_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkGreen)
    }
    pub fn dark_khaki(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkKhaki)
    }
    pub fn dark_magenta(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkMagenta)
    }
    pub fn dark_olive_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkOliveGreen)
    }
    pub fn darkorange(self) -> Style<BackgroundBaseState> {
        self.base(Color::Darkorange)
    }
    pub fn dark_orchid(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkOrchid)
    }
    pub fn dark_red(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkRed)
    }
    pub fn dark_salmon(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkSalmon)
    }
    pub fn dark_sea_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkSeaGreen)
    }
    pub fn dark_slate_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkSlateBlue)
    }
    pub fn dark_slate_grey(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkSlateGrey)
    }
    pub fn dark_turquoise(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkTurquoise)
    }
    pub fn dark_violet(self) -> Style<BackgroundBaseState> {
        self.base(Color::DarkViolet)
    }
    pub fn deep_pink(self) -> Style<BackgroundBaseState> {
        self.base(Color::DeepPink)
    }
    pub fn deep_sky_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::DeepSkyBlue)
    }
    pub fn dim_gray(self) -> Style<BackgroundBaseState> {
        self.base(Color::DimGray)
    }
    pub fn dodger_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::DodgerBlue)
    }
    pub fn fire_brick(self) -> Style<BackgroundBaseState> {
        self.base(Color::FireBrick)
    }
    pub fn floral_white(self) -> Style<BackgroundBaseState> {
        self.base(Color::FloralWhite)
    }
    pub fn forest_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::ForestGreen)
    }
    pub fn fuchsia(self) -> Style<BackgroundBaseState> {
        self.base(Color::Fuchsia)
    }
    pub fn gainsboro(self) -> Style<BackgroundBaseState> {
        self.base(Color::Gainsboro)
    }
    pub fn ghost_white(self) -> Style<BackgroundBaseState> {
        self.base(Color::GhostWhite)
    }
    pub fn gold(self) -> Style<BackgroundBaseState> {
        self.base(Color::Gold)
    }
    pub fn golden_rod(self) -> Style<BackgroundBaseState> {
        self.base(Color::GoldenRod)
    }
    pub fn grey(self) -> Style<BackgroundBaseState> {
        self.base(Color::Grey)
    }
    pub fn green(self) -> Style<BackgroundBaseState> {
        self.base(Color::Green)
    }
    pub fn green_yellow(self) -> Style<BackgroundBaseState> {
        self.base(Color::GreenYellow)
    }
    pub fn honey_dew(self) -> Style<BackgroundBaseState> {
        self.base(Color::HoneyDew)
    }
    pub fn hot_pink(self) -> Style<BackgroundBaseState> {
        self.base(Color::HotPink)
    }
    pub fn indian_red(self) -> Style<BackgroundBaseState> {
        self.base(Color::IndianRed)
    }
    pub fn indigo(self) -> Style<BackgroundBaseState> {
        self.base(Color::Indigo)
    }
    pub fn ivory(self) -> Style<BackgroundBaseState> {
        self.base(Color::Ivory)
    }
    pub fn khaki(self) -> Style<BackgroundBaseState> {
        self.base(Color::Khaki)
    }
    pub fn lavender(self) -> Style<BackgroundBaseState> {
        self.base(Color::Lavender)
    }
    pub fn lavender_blush(self) -> Style<BackgroundBaseState> {
        self.base(Color::LavenderBlush)
    }
    pub fn lawn_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::LawnGreen)
    }
    pub fn lemon_chiffon(self) -> Style<BackgroundBaseState> {
        self.base(Color::LemonChiffon)
    }
    pub fn light_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightBlue)
    }
    pub fn light_coral(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightCoral)
    }
    pub fn light_cyan(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightCyan)
    }
    pub fn light_golden_rod_yellow(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightGoldenRodYellow)
    }
    pub fn light_grey(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightGrey)
    }
    pub fn light_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightGreen)
    }
    pub fn light_pink(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightPink)
    }
    pub fn light_salmon(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightSalmon)
    }
    pub fn light_sea_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightSeaGreen)
    }
    pub fn light_sky_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightSkyBlue)
    }
    pub fn light_slate_grey(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightSlateGrey)
    }
    pub fn light_steel_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightSteelBlue)
    }
    pub fn light_yellow(self) -> Style<BackgroundBaseState> {
        self.base(Color::LightYellow)
    }
    pub fn lime(self) -> Style<BackgroundBaseState> {
        self.base(Color::Lime)
    }
    pub fn lime_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::LimeGreen)
    }
    pub fn linen(self) -> Style<BackgroundBaseState> {
        self.base(Color::Linen)
    }
    pub fn magenta(self) -> Style<BackgroundBaseState> {
        self.base(Color::Magenta)
    }
    pub fn maroon(self) -> Style<BackgroundBaseState> {
        self.base(Color::Maroon)
    }
    pub fn medium_aqua_marine(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumAquaMarine)
    }
    pub fn medium_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumBlue)
    }
    pub fn medium_orchid(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumOrchid)
    }
    pub fn medium_purple(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumPurple)
    }
    pub fn medium_sea_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumSeaGreen)
    }
    pub fn medium_slate_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumSlateBlue)
    }
    pub fn medium_spring_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumSpringGreen)
    }
    pub fn medium_turquoise(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumTurquoise)
    }
    pub fn medium_violet_red(self) -> Style<BackgroundBaseState> {
        self.base(Color::MediumVioletRed)
    }
    pub fn midnight_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::MidnightBlue)
    }
    pub fn mint_cream(self) -> Style<BackgroundBaseState> {
        self.base(Color::MintCream)
    }
    pub fn misty_rose(self) -> Style<BackgroundBaseState> {
        self.base(Color::MistyRose)
    }
    pub fn moccasin(self) -> Style<BackgroundBaseState> {
        self.base(Color::Moccasin)
    }
    pub fn navajo_white(self) -> Style<BackgroundBaseState> {
        self.base(Color::NavajoWhite)
    }
    pub fn navy(self) -> Style<BackgroundBaseState> {
        self.base(Color::Navy)
    }
    pub fn old_lace(self) -> Style<BackgroundBaseState> {
        self.base(Color::OldLace)
    }
    pub fn olive(self) -> Style<BackgroundBaseState> {
        self.base(Color::Olive)
    }
    pub fn olive_drab(self) -> Style<BackgroundBaseState> {
        self.base(Color::OliveDrab)
    }
    pub fn orange(self) -> Style<BackgroundBaseState> {
        self.base(Color::Orange)
    }
    pub fn orange_red(self) -> Style<BackgroundBaseState> {
        self.base(Color::OrangeRed)
    }
    pub fn orchid(self) -> Style<BackgroundBaseState> {
        self.base(Color::Orchid)
    }
    pub fn pale_golden_rod(self) -> Style<BackgroundBaseState> {
        self.base(Color::PaleGoldenRod)
    }
    pub fn pale_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::PaleGreen)
    }
    pub fn pale_turquoise(self) -> Style<BackgroundBaseState> {
        self.base(Color::PaleTurquoise)
    }
    pub fn pale_violet_red(self) -> Style<BackgroundBaseState> {
        self.base(Color::PaleVioletRed)
    }
    pub fn papaya_whip(self) -> Style<BackgroundBaseState> {
        self.base(Color::PapayaWhip)
    }
    pub fn peach_puff(self) -> Style<BackgroundBaseState> {
        self.base(Color::PeachPuff)
    }
    pub fn peru(self) -> Style<BackgroundBaseState> {
        self.base(Color::Peru)
    }
    pub fn pink(self) -> Style<BackgroundBaseState> {
        self.base(Color::Pink)
    }
    pub fn plum(self) -> Style<BackgroundBaseState> {
        self.base(Color::Plum)
    }
    pub fn powder_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::PowderBlue)
    }
    pub fn purple(self) -> Style<BackgroundBaseState> {
        self.base(Color::Purple)
    }
    pub fn red(self) -> Style<BackgroundBaseState> {
        self.base(Color::Red)
    }
    pub fn rosy_brown(self) -> Style<BackgroundBaseState> {
        self.base(Color::RosyBrown)
    }
    pub fn royal_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::RoyalBlue)
    }
    pub fn saddle_brown(self) -> Style<BackgroundBaseState> {
        self.base(Color::SaddleBrown)
    }
    pub fn salmon(self) -> Style<BackgroundBaseState> {
        self.base(Color::Salmon)
    }
    pub fn sandy_brown(self) -> Style<BackgroundBaseState> {
        self.base(Color::SandyBrown)
    }
    pub fn sea_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::SeaGreen)
    }
    pub fn sea_shell(self) -> Style<BackgroundBaseState> {
        self.base(Color::SeaShell)
    }
    pub fn sienna(self) -> Style<BackgroundBaseState> {
        self.base(Color::Sienna)
    }
    pub fn silver(self) -> Style<BackgroundBaseState> {
        self.base(Color::Silver)
    }
    pub fn sky_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::SkyBlue)
    }
    pub fn slate_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::SlateBlue)
    }
    pub fn slate_grey(self) -> Style<BackgroundBaseState> {
        self.base(Color::SlateGrey)
    }
    pub fn snow(self) -> Style<BackgroundBaseState> {
        self.base(Color::Snow)
    }
    pub fn spring_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::SpringGreen)
    }
    pub fn steel_blue(self) -> Style<BackgroundBaseState> {
        self.base(Color::SteelBlue)
    }
    pub fn tan(self) -> Style<BackgroundBaseState> {
        self.base(Color::Tan)
    }
    pub fn teal(self) -> Style<BackgroundBaseState> {
        self.base(Color::Teal)
    }
    pub fn thistle(self) -> Style<BackgroundBaseState> {
        self.base(Color::Thistle)
    }
    pub fn tomato(self) -> Style<BackgroundBaseState> {
        self.base(Color::Tomato)
    }
    pub fn turquoise(self) -> Style<BackgroundBaseState> {
        self.base(Color::Turquoise)
    }
    pub fn violet(self) -> Style<BackgroundBaseState> {
        self.base(Color::Violet)
    }
    pub fn wheat(self) -> Style<BackgroundBaseState> {
        self.base(Color::Wheat)
    }
    pub fn white(self) -> Style<BackgroundBaseState> {
        self.base(Color::White)
    }
    pub fn white_smoke(self) -> Style<BackgroundBaseState> {
        self.base(Color::WhiteSmoke)
    }
    pub fn yellow(self) -> Style<BackgroundBaseState> {
        self.base(Color::Yellow)
    }

    pub fn yellow_green(self) -> Style<BackgroundBaseState> {
        self.base(Color::YellowGreen)
    }
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
            Color::AliceBlue => "AliceBlue".to_string(),
            Color::AntiqueWhite => "AntiqueWhite".to_string(),
            Color::Aqua => "Aqua".to_string(),
            Color::Aquamarine => "Aquamarine".to_string(),
            Color::Azure => "Azure".to_string(),
            Color::Beige => "Beige".to_string(),
            Color::Bisque => "Bisque".to_string(),
            Color::Black => "Black".to_string(),
            Color::BlanchedAlmond => "BlanchedAlmond".to_string(),
            Color::Blue => "Blue".to_string(),
            Color::BlueViolet => "BlueViolet".to_string(),
            Color::Brown => "Brown".to_string(),
            Color::BurlyWood => "BurlyWood".to_string(),
            Color::CadetBlue => "CadetBlue".to_string(),
            Color::Chartreuse => "Chartreuse".to_string(),
            Color::Chocolate => "Chocolate".to_string(),
            Color::Coral => "Coral".to_string(),
            Color::CornflowerBlue => "CornflowerBlue".to_string(),
            Color::Cornsilk => "Cornsilk".to_string(),
            Color::Crimson => "Crimson".to_string(),
            Color::Cyan => "Cyan".to_string(),
            Color::DarkBlue => "DarkBlue".to_string(),
            Color::DarkCyan => "DarkCyan".to_string(),
            Color::DarkGoldenRod => "DarkGoldenRod".to_string(),
            Color::DarkGrey => "DarkGrey".to_string(),
            Color::DarkGreen => "DarkGreen".to_string(),
            Color::DarkKhaki => "DarkKhaki".to_string(),
            Color::DarkMagenta => "DarkMagenta".to_string(),
            Color::DarkOliveGreen => "DarkOliveGreen".to_string(),
            Color::Darkorange => "Darkorange".to_string(),
            Color::DarkOrchid => "DarkOrchid".to_string(),
            Color::DarkRed => "DarkRed".to_string(),
            Color::DarkSalmon => "DarkSalmon".to_string(),
            Color::DarkSeaGreen => "DarkSeaGreen".to_string(),
            Color::DarkSlateBlue => "DarkSlateBlue".to_string(),
            Color::DarkSlateGrey => "DarkSlateGrey".to_string(),
            Color::DarkTurquoise => "DarkTurquoise".to_string(),
            Color::DarkViolet => "DarkViolet".to_string(),
            Color::DeepPink => "DeepPink".to_string(),
            Color::DeepSkyBlue => "DeepSkyBlue".to_string(),
            Color::DimGray => "DimGray".to_string(),
            Color::DodgerBlue => "DodgerBlue".to_string(),
            Color::FireBrick => "FireBrick".to_string(),
            Color::FloralWhite => "FloralWhite".to_string(),
            Color::ForestGreen => "ForestGreen".to_string(),
            Color::Fuchsia => "Fuchsia".to_string(),
            Color::Gainsboro => "Gainsboro".to_string(),
            Color::GhostWhite => "GhostWhite".to_string(),
            Color::Gold => "Gold".to_string(),
            Color::GoldenRod => "GoldenRod".to_string(),
            Color::Grey => "Grey".to_string(),
            Color::Green => "Green".to_string(),
            Color::GreenYellow => "GreenYellow".to_string(),
            Color::HoneyDew => "HoneyDew".to_string(),
            Color::HotPink => "HotPink".to_string(),
            Color::IndianRed => "IndianRed".to_string(),
            Color::Indigo => "Indigo".to_string(),
            Color::Ivory => "Ivory".to_string(),
            Color::Khaki => "Khaki".to_string(),
            Color::Lavender => "Lavender".to_string(),
            Color::LavenderBlush => "LavenderBlush".to_string(),
            Color::LawnGreen => "LawnGreen".to_string(),
            Color::LemonChiffon => "LemonChiffon".to_string(),
            Color::LightBlue => "LightBlue".to_string(),
            Color::LightCoral => "LightCoral".to_string(),
            Color::LightCyan => "LightCyan".to_string(),
            Color::LightGoldenRodYellow => "LightGoldenRodYellow".to_string(),
            Color::LightGrey => "LightGrey".to_string(),
            Color::LightGreen => "LightGreen".to_string(),
            Color::LightPink => "LightPink".to_string(),
            Color::LightSalmon => "LightSalmon".to_string(),
            Color::LightSeaGreen => "LightSeaGreen".to_string(),
            Color::LightSkyBlue => "LightSkyBlue".to_string(),
            Color::LightSlateGrey => "LightSlateGrey".to_string(),
            Color::LightSteelBlue => "LightSteelBlue".to_string(),
            Color::LightYellow => "LightYellow".to_string(),
            Color::Lime => "Lime".to_string(),
            Color::LimeGreen => "LimeGreen".to_string(),
            Color::Linen => "Linen".to_string(),
            Color::Magenta => "Magenta".to_string(),
            Color::Maroon => "Maroon".to_string(),
            Color::MediumAquaMarine => "MediumAquaMarine".to_string(),
            Color::MediumBlue => "MediumBlue".to_string(),
            Color::MediumOrchid => "MediumOrchid".to_string(),
            Color::MediumPurple => "MediumPurple".to_string(),
            Color::MediumSeaGreen => "MediumSeaGreen".to_string(),
            Color::MediumSlateBlue => "MediumSlateBlue".to_string(),
            Color::MediumSpringGreen => "MediumSpringGreen".to_string(),
            Color::MediumTurquoise => "MediumTurquoise".to_string(),
            Color::MediumVioletRed => "MediumVioletRed".to_string(),
            Color::MidnightBlue => "MidnightBlue".to_string(),
            Color::MintCream => "MintCream".to_string(),
            Color::MistyRose => "MistyRose".to_string(),
            Color::Moccasin => "Moccasin".to_string(),
            Color::NavajoWhite => "NavajoWhite".to_string(),
            Color::Navy => "Navy".to_string(),
            Color::OldLace => "OldLace".to_string(),
            Color::Olive => "Olive".to_string(),
            Color::OliveDrab => "OliveDrab".to_string(),
            Color::Orange => "Orange".to_string(),
            Color::OrangeRed => "OrangeRed".to_string(),
            Color::Orchid => "Orchid".to_string(),
            Color::PaleGoldenRod => "PaleGoldenRod".to_string(),
            Color::PaleGreen => "PaleGreen".to_string(),
            Color::PaleTurquoise => "PaleTurquoise".to_string(),
            Color::PaleVioletRed => "PaleVioletRed".to_string(),
            Color::PapayaWhip => "PapayaWhip".to_string(),
            Color::PeachPuff => "PeachPuff".to_string(),
            Color::Peru => "Peru".to_string(),
            Color::Pink => "Pink".to_string(),
            Color::Plum => "Plum".to_string(),
            Color::PowderBlue => "PowderBlue".to_string(),
            Color::Purple => "Purple".to_string(),
            Color::Red => "Red".to_string(),
            Color::RosyBrown => "RosyBrown".to_string(),
            Color::RoyalBlue => "RoyalBlue".to_string(),
            Color::SaddleBrown => "SaddleBrown".to_string(),
            Color::Salmon => "Salmon".to_string(),
            Color::SandyBrown => "SandyBrown".to_string(),
            Color::SeaGreen => "SeaGreen".to_string(),
            Color::SeaShell => "SeaShell".to_string(),
            Color::Sienna => "Sienna".to_string(),
            Color::Silver => "Silver".to_string(),
            Color::SkyBlue => "SkyBlue".to_string(),
            Color::SlateBlue => "SlateBlue".to_string(),
            Color::SlateGrey => "SlateGrey".to_string(),
            Color::Snow => "Snow".to_string(),
            Color::SpringGreen => "SpringGreen".to_string(),
            Color::SteelBlue => "SteelBlue".to_string(),
            Color::Tan => "Tan".to_string(),
            Color::Teal => "Teal".to_string(),
            Color::Thistle => "Thistle".to_string(),
            Color::Tomato => "Tomato".to_string(),
            Color::Turquoise => "Turquoise".to_string(),
            Color::Violet => "Violet".to_string(),
            Color::Wheat => "Wheat".to_string(),
            Color::White => "White".to_string(),
            Color::WhiteSmoke => "WhiteSmoke".to_string(),
            Color::Yellow => "Yellow".to_string(),
            Color::YellowGreen => "YellowGreen".to_string(),
        };
        write!(f, "{}", result)
    }
}
