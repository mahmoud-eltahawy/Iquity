use super::{MedAttribute, Style};

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

impl Color {
    pub(crate) fn css(&self) -> String {
        match self {
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
        }
    }
}

impl MedAttribute<Color> {
    pub fn hex(self, hex: u32) -> Style {
        self.inner(Color::Hex(hex))
    }

    pub fn t_hex(self, hex: u32) -> Style {
        self.inner(Color::THex(hex))
    }

    pub fn rgb(self, red: u8, green: u8, blue: u8) -> Style {
        self.inner(Color::Rgb(red, green, blue))
    }

    pub fn rgba(self, red: u8, green: u8, blue: u8, opacity: u8) -> Style {
        debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
        self.inner(Color::Rgba(red, green, blue, opacity))
    }

    pub fn hsl(self, hue: u16, saturation: u8, lightness: u8) -> Style {
        debug_assert!(hue <= 360, "hue should be from 0 to 360");
        debug_assert!(saturation <= 100, "saturation should be from 0 to 100");
        debug_assert!(lightness <= 100, "lightness should be from 0 to 100");
        self.inner(Color::Hsl(hue, saturation, lightness))
    }

    pub fn hsla(self, hue: u16, saturation: u8, lightness: u8, opacity: u8) -> Style {
        debug_assert!(hue <= 360, "hue should be from 0 to 360");
        debug_assert!(saturation <= 100, "saturation should be from 0 to 100");
        debug_assert!(lightness <= 100, "lightness should be from 0 to 100");
        debug_assert!(opacity <= 100, "opacity is from 0 to 100 not from 0 to 1");
        self.inner(Color::Hsla(hue, saturation, lightness, opacity))
    }

    pub fn alice_blue(self) -> Style {
        self.inner(Color::AliceBlue)
    }
    pub fn antique_white(self) -> Style {
        self.inner(Color::AntiqueWhite)
    }
    pub fn aqua(self) -> Style {
        self.inner(Color::Aqua)
    }
    pub fn aquamarine(self) -> Style {
        self.inner(Color::Aquamarine)
    }
    pub fn azure(self) -> Style {
        self.inner(Color::Azure)
    }
    pub fn beige(self) -> Style {
        self.inner(Color::Beige)
    }
    pub fn bisque(self) -> Style {
        self.inner(Color::Bisque)
    }
    pub fn black(self) -> Style {
        self.inner(Color::Black)
    }
    pub fn blanched_almond(self) -> Style {
        self.inner(Color::BlanchedAlmond)
    }
    pub fn blue(self) -> Style {
        self.inner(Color::Blue)
    }
    pub fn blue_violet(self) -> Style {
        self.inner(Color::BlueViolet)
    }
    pub fn brown(self) -> Style {
        self.inner(Color::Brown)
    }
    pub fn burly_wood(self) -> Style {
        self.inner(Color::BurlyWood)
    }
    pub fn cadet_blue(self) -> Style {
        self.inner(Color::CadetBlue)
    }
    pub fn chartreuse(self) -> Style {
        self.inner(Color::Chartreuse)
    }
    pub fn chocolate(self) -> Style {
        self.inner(Color::Chocolate)
    }
    pub fn coral(self) -> Style {
        self.inner(Color::Coral)
    }
    pub fn cornflower_blue(self) -> Style {
        self.inner(Color::CornflowerBlue)
    }
    pub fn cornsilk(self) -> Style {
        self.inner(Color::Cornsilk)
    }
    pub fn crimson(self) -> Style {
        self.inner(Color::Crimson)
    }
    pub fn cyan(self) -> Style {
        self.inner(Color::Cyan)
    }
    pub fn dark_blue(self) -> Style {
        self.inner(Color::DarkBlue)
    }
    pub fn dark_cyan(self) -> Style {
        self.inner(Color::DarkCyan)
    }
    pub fn dark_golden_rod(self) -> Style {
        self.inner(Color::DarkGoldenRod)
    }
    pub fn dark_grey(self) -> Style {
        self.inner(Color::DarkGrey)
    }
    pub fn dark_green(self) -> Style {
        self.inner(Color::DarkGreen)
    }
    pub fn dark_khaki(self) -> Style {
        self.inner(Color::DarkKhaki)
    }
    pub fn dark_magenta(self) -> Style {
        self.inner(Color::DarkMagenta)
    }
    pub fn dark_olive_green(self) -> Style {
        self.inner(Color::DarkOliveGreen)
    }
    pub fn darkorange(self) -> Style {
        self.inner(Color::Darkorange)
    }
    pub fn dark_orchid(self) -> Style {
        self.inner(Color::DarkOrchid)
    }
    pub fn dark_red(self) -> Style {
        self.inner(Color::DarkRed)
    }
    pub fn dark_salmon(self) -> Style {
        self.inner(Color::DarkSalmon)
    }
    pub fn dark_sea_green(self) -> Style {
        self.inner(Color::DarkSeaGreen)
    }
    pub fn dark_slate_blue(self) -> Style {
        self.inner(Color::DarkSlateBlue)
    }
    pub fn dark_slate_grey(self) -> Style {
        self.inner(Color::DarkSlateGrey)
    }
    pub fn dark_turquoise(self) -> Style {
        self.inner(Color::DarkTurquoise)
    }
    pub fn dark_violet(self) -> Style {
        self.inner(Color::DarkViolet)
    }
    pub fn deep_pink(self) -> Style {
        self.inner(Color::DeepPink)
    }
    pub fn deep_sky_blue(self) -> Style {
        self.inner(Color::DeepSkyBlue)
    }
    pub fn dim_gray(self) -> Style {
        self.inner(Color::DimGray)
    }
    pub fn dodger_blue(self) -> Style {
        self.inner(Color::DodgerBlue)
    }
    pub fn fire_brick(self) -> Style {
        self.inner(Color::FireBrick)
    }
    pub fn floral_white(self) -> Style {
        self.inner(Color::FloralWhite)
    }
    pub fn forest_green(self) -> Style {
        self.inner(Color::ForestGreen)
    }
    pub fn fuchsia(self) -> Style {
        self.inner(Color::Fuchsia)
    }
    pub fn gainsboro(self) -> Style {
        self.inner(Color::Gainsboro)
    }
    pub fn ghost_white(self) -> Style {
        self.inner(Color::GhostWhite)
    }
    pub fn gold(self) -> Style {
        self.inner(Color::Gold)
    }
    pub fn golden_rod(self) -> Style {
        self.inner(Color::GoldenRod)
    }
    pub fn grey(self) -> Style {
        self.inner(Color::Grey)
    }
    pub fn green(self) -> Style {
        self.inner(Color::Green)
    }
    pub fn green_yellow(self) -> Style {
        self.inner(Color::GreenYellow)
    }
    pub fn honey_dew(self) -> Style {
        self.inner(Color::HoneyDew)
    }
    pub fn hot_pink(self) -> Style {
        self.inner(Color::HotPink)
    }
    pub fn indian_red(self) -> Style {
        self.inner(Color::IndianRed)
    }
    pub fn indigo(self) -> Style {
        self.inner(Color::Indigo)
    }
    pub fn ivory(self) -> Style {
        self.inner(Color::Ivory)
    }
    pub fn khaki(self) -> Style {
        self.inner(Color::Khaki)
    }
    pub fn lavender(self) -> Style {
        self.inner(Color::Lavender)
    }
    pub fn lavender_blush(self) -> Style {
        self.inner(Color::LavenderBlush)
    }
    pub fn lawn_green(self) -> Style {
        self.inner(Color::LawnGreen)
    }
    pub fn lemon_chiffon(self) -> Style {
        self.inner(Color::LemonChiffon)
    }
    pub fn light_blue(self) -> Style {
        self.inner(Color::LightBlue)
    }
    pub fn light_coral(self) -> Style {
        self.inner(Color::LightCoral)
    }
    pub fn light_cyan(self) -> Style {
        self.inner(Color::LightCyan)
    }
    pub fn light_golden_rod_yellow(self) -> Style {
        self.inner(Color::LightGoldenRodYellow)
    }
    pub fn light_grey(self) -> Style {
        self.inner(Color::LightGrey)
    }
    pub fn light_green(self) -> Style {
        self.inner(Color::LightGreen)
    }
    pub fn light_pink(self) -> Style {
        self.inner(Color::LightPink)
    }
    pub fn light_salmon(self) -> Style {
        self.inner(Color::LightSalmon)
    }
    pub fn light_sea_green(self) -> Style {
        self.inner(Color::LightSeaGreen)
    }
    pub fn light_sky_blue(self) -> Style {
        self.inner(Color::LightSkyBlue)
    }
    pub fn light_slate_grey(self) -> Style {
        self.inner(Color::LightSlateGrey)
    }
    pub fn light_steel_blue(self) -> Style {
        self.inner(Color::LightSteelBlue)
    }
    pub fn light_yellow(self) -> Style {
        self.inner(Color::LightYellow)
    }
    pub fn lime(self) -> Style {
        self.inner(Color::Lime)
    }
    pub fn lime_green(self) -> Style {
        self.inner(Color::LimeGreen)
    }
    pub fn linen(self) -> Style {
        self.inner(Color::Linen)
    }
    pub fn magenta(self) -> Style {
        self.inner(Color::Magenta)
    }
    pub fn maroon(self) -> Style {
        self.inner(Color::Maroon)
    }
    pub fn medium_aqua_marine(self) -> Style {
        self.inner(Color::MediumAquaMarine)
    }
    pub fn medium_blue(self) -> Style {
        self.inner(Color::MediumBlue)
    }
    pub fn medium_orchid(self) -> Style {
        self.inner(Color::MediumOrchid)
    }
    pub fn medium_purple(self) -> Style {
        self.inner(Color::MediumPurple)
    }
    pub fn medium_sea_green(self) -> Style {
        self.inner(Color::MediumSeaGreen)
    }
    pub fn medium_slate_blue(self) -> Style {
        self.inner(Color::MediumSlateBlue)
    }
    pub fn medium_spring_green(self) -> Style {
        self.inner(Color::MediumSpringGreen)
    }
    pub fn medium_turquoise(self) -> Style {
        self.inner(Color::MediumTurquoise)
    }
    pub fn medium_violet_red(self) -> Style {
        self.inner(Color::MediumVioletRed)
    }
    pub fn midnight_blue(self) -> Style {
        self.inner(Color::MidnightBlue)
    }
    pub fn mint_cream(self) -> Style {
        self.inner(Color::MintCream)
    }
    pub fn misty_rose(self) -> Style {
        self.inner(Color::MistyRose)
    }
    pub fn moccasin(self) -> Style {
        self.inner(Color::Moccasin)
    }
    pub fn navajo_white(self) -> Style {
        self.inner(Color::NavajoWhite)
    }
    pub fn navy(self) -> Style {
        self.inner(Color::Navy)
    }
    pub fn old_lace(self) -> Style {
        self.inner(Color::OldLace)
    }
    pub fn olive(self) -> Style {
        self.inner(Color::Olive)
    }
    pub fn olive_drab(self) -> Style {
        self.inner(Color::OliveDrab)
    }
    pub fn orange(self) -> Style {
        self.inner(Color::Orange)
    }
    pub fn orange_red(self) -> Style {
        self.inner(Color::OrangeRed)
    }
    pub fn orchid(self) -> Style {
        self.inner(Color::Orchid)
    }
    pub fn pale_golden_rod(self) -> Style {
        self.inner(Color::PaleGoldenRod)
    }
    pub fn pale_green(self) -> Style {
        self.inner(Color::PaleGreen)
    }
    pub fn pale_turquoise(self) -> Style {
        self.inner(Color::PaleTurquoise)
    }
    pub fn pale_violet_red(self) -> Style {
        self.inner(Color::PaleVioletRed)
    }
    pub fn papaya_whip(self) -> Style {
        self.inner(Color::PapayaWhip)
    }
    pub fn peach_puff(self) -> Style {
        self.inner(Color::PeachPuff)
    }
    pub fn peru(self) -> Style {
        self.inner(Color::Peru)
    }
    pub fn pink(self) -> Style {
        self.inner(Color::Pink)
    }
    pub fn plum(self) -> Style {
        self.inner(Color::Plum)
    }
    pub fn powder_blue(self) -> Style {
        self.inner(Color::PowderBlue)
    }
    pub fn purple(self) -> Style {
        self.inner(Color::Purple)
    }
    pub fn red(self) -> Style {
        self.inner(Color::Red)
    }
    pub fn rosy_brown(self) -> Style {
        self.inner(Color::RosyBrown)
    }
    pub fn royal_blue(self) -> Style {
        self.inner(Color::RoyalBlue)
    }
    pub fn saddle_brown(self) -> Style {
        self.inner(Color::SaddleBrown)
    }
    pub fn salmon(self) -> Style {
        self.inner(Color::Salmon)
    }
    pub fn sandy_brown(self) -> Style {
        self.inner(Color::SandyBrown)
    }
    pub fn sea_green(self) -> Style {
        self.inner(Color::SeaGreen)
    }
    pub fn sea_shell(self) -> Style {
        self.inner(Color::SeaShell)
    }
    pub fn sienna(self) -> Style {
        self.inner(Color::Sienna)
    }
    pub fn silver(self) -> Style {
        self.inner(Color::Silver)
    }
    pub fn sky_blue(self) -> Style {
        self.inner(Color::SkyBlue)
    }
    pub fn slate_blue(self) -> Style {
        self.inner(Color::SlateBlue)
    }
    pub fn slate_grey(self) -> Style {
        self.inner(Color::SlateGrey)
    }
    pub fn snow(self) -> Style {
        self.inner(Color::Snow)
    }
    pub fn spring_green(self) -> Style {
        self.inner(Color::SpringGreen)
    }
    pub fn steel_blue(self) -> Style {
        self.inner(Color::SteelBlue)
    }
    pub fn tan(self) -> Style {
        self.inner(Color::Tan)
    }
    pub fn teal(self) -> Style {
        self.inner(Color::Teal)
    }
    pub fn thistle(self) -> Style {
        self.inner(Color::Thistle)
    }
    pub fn tomato(self) -> Style {
        self.inner(Color::Tomato)
    }
    pub fn turquoise(self) -> Style {
        self.inner(Color::Turquoise)
    }
    pub fn violet(self) -> Style {
        self.inner(Color::Violet)
    }
    pub fn wheat(self) -> Style {
        self.inner(Color::Wheat)
    }
    pub fn white(self) -> Style {
        self.inner(Color::White)
    }
    pub fn white_smoke(self) -> Style {
        self.inner(Color::WhiteSmoke)
    }
    pub fn yellow(self) -> Style {
        self.inner(Color::Yellow)
    }

    pub fn yellow_green(self) -> Style {
        self.inner(Color::YellowGreen)
    }
}
