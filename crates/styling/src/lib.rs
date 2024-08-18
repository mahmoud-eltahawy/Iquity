pub struct Style(Vec<CssAttribute>);

pub fn styling() -> Style {
    Style(vec![])
}

impl Style {
    pub fn fontsize(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(|x| CssAttribute::FontSize(x)),
        }
    }

    pub fn margin(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(|x| CssAttribute::Margin(x)),
        }
    }

    pub fn padding(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(|x| CssAttribute::Padding(x)),
        }
    }

    pub fn bottom(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(|x| CssAttribute::Bottom(x)),
        }
    }

    pub fn height(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(|x| CssAttribute::Height(x)),
        }
    }

    pub fn width(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(|x| CssAttribute::Width(x)),
        }
    }

    pub fn position(self) -> MedAttribute<CssPosition> {
        MedAttribute {
            style: self,
            fun: Box::new(|x| CssAttribute::Position(x)),
        }
    }

    pub fn background_color(self) -> MedAttribute<Color> {
        MedAttribute {
            style: self,
            fun: Box::new(|x| CssAttribute::BackgroundColor(x)),
        }
    }
}

pub struct MedAttribute<T> {
    style: Style,
    fun: Box<dyn Fn(T) -> CssAttribute>,
}

impl MedAttribute<Color> {
    pub fn hex(self, hex: u32) -> Style {
        let Self { mut style, fun } = self;
        let attr = fun(Color::Hex(hex));
        style.0.push(attr);
        style
    }
}

impl MedAttribute<Length> {
    pub fn inner(self, length: Length) -> Style {
        let Self { mut style, fun } = self;
        let attr = fun(length);
        style.0.push(attr);
        style
    }

    pub fn px(self, num: u8) -> Style {
        self.inner(Length::Px(num))
    }

    pub fn cm(self, num: u8) -> Style {
        self.inner(Length::Cm(num))
    }

    pub fn percent(self, num: u8) -> Style {
        assert!(num <= 100, "percent number should be from 0 to 100");
        self.inner(Length::Percent(num))
    }
}

impl MedAttribute<CssPosition> {
    fn inner(self, position: CssPosition) -> Style {
        let Self { mut style, fun } = self;
        let attr = fun(position);
        style.0.push(attr);
        style
    }

    pub fn fixed(self) -> Style {
        self.inner(CssPosition::Fixed)
    }

    pub fn relative(self) -> Style {
        self.inner(CssPosition::Relative)
    }

    pub fn static_p(self) -> Style {
        self.inner(CssPosition::Static)
    }

    pub fn absolute(self) -> Style {
        self.inner(CssPosition::Absolute)
    }

    pub fn sticky(self) -> Style {
        self.inner(CssPosition::Sticky)
    }
}

pub enum CssAttribute {
    FontSize(Length),
    Position(CssPosition),
    BackgroundColor(Color),
    Top(Length),
    Bottom(Length),
    Right(Length),
    Left(Length),
    Height(Length),
    Width(Length),
    Margin(Length),
    Padding(Length),
}

pub enum Length {
    //absolute
    Cm(u8),
    Mm(u8),
    In(u8),
    Px(u8),
    Pt(u8),
    Pc(u8),
    //relative
    Em(u8),
    Ex(u8),
    Ch(u8),
    Rem(u8),
    Vw(u8),
    Vh(u8),
    Vmin(u8),
    Vmax(u8),
    Percent(u8),
}

impl Length {
    fn get(&self) -> String {
        match self {
            Length::Cm(num) => format!("{num}cm"),
            Length::Mm(num) => format!("{num}mm"),
            Length::In(num) => format!("{num}in"),
            Length::Px(num) => format!("{num}px"),
            Length::Pt(num) => format!("{num}pt"),
            Length::Pc(num) => format!("{num}pc"),
            Length::Em(num) => format!("{num}em"),
            Length::Ex(num) => format!("{num}ex"),
            Length::Ch(num) => format!("{num}ch"),
            Length::Rem(num) => format!("{num}rem"),
            Length::Vw(num) => format!("{num}vw"),
            Length::Vh(num) => format!("{num}vh"),
            Length::Vmin(num) => format!("{num}vmin"),
            Length::Vmax(num) => format!("{num}vmax"),
            Length::Percent(num) => format!("{num}%"),
        }
    }
}

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
    fn css(&self) -> String {
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
                assert!(*opacity <= 100, "opacity should be from 0 to 100");
                let opacity = *opacity as f32 / 100.;
                // let i: u32 = 0xffffff;
                format!("rgba({red},{green},{blue},{opacity})")
            }
            Color::Hsl(hue, saturation, lightness) => {
                assert!(hue <= &360, "hue should be from 0 to 360");
                assert!(saturation <= &100, "saturation should be from 0 to 100");
                assert!(lightness <= &100, "lightness should be from 0 to 100");
                format!("hsl({hue},{saturation}%,{lightness}%)")
            }
            Color::Hsla(hue, saturation, lightness, opacity) => {
                assert!(hue <= &360, "hue should be from 0 to 360");
                assert!(saturation <= &100, "saturation should be from 0 to 100");
                assert!(lightness <= &100, "lightness should be from 0 to 100");
                assert!(opacity <= &100, "opacity should be from 0 to 100");
                let opacity = *opacity as f32 / 100.;
                format!("hsl({hue},{saturation}%,{lightness}%,{opacity})")
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

pub enum CssPosition {
    Static,
    Relative,
    Fixed,
    Absolute,
    Sticky,
}

impl CssPosition {
    fn get(&self) -> String {
        let result = match self {
            CssPosition::Static => "static",
            CssPosition::Relative => "relative",
            CssPosition::Fixed => "fixed",
            CssPosition::Absolute => "absolute",
            CssPosition::Sticky => "sticky",
        };
        format!("position:{result};")
    }
}

impl Style {
    pub fn build(&self) -> String {
        self.0
            .iter()
            .map(|x| match x {
                CssAttribute::FontSize(distance) => format!("font-size:{};", distance.get()),
                CssAttribute::Position(position) => position.get(),
                CssAttribute::BackgroundColor(color) => {
                    format!("background-color:{};", color.css())
                }
                CssAttribute::Top(distance) => format!("top:{};", distance.get()),
                CssAttribute::Bottom(distance) => format!("bottom:{};", distance.get()),
                CssAttribute::Right(distance) => format!("right:{};", distance.get()),
                CssAttribute::Left(distance) => format!("left:{};", distance.get()),
                CssAttribute::Height(distance) => format!("height:{};", distance.get()),
                CssAttribute::Width(distance) => format!("width:{};", distance.get()),
                CssAttribute::Margin(distance) => format!("margin:{};", distance.get()),
                CssAttribute::Padding(distance) => format!("padding:{};", distance.get()),
            })
            .fold(String::new(), move |acc, x| acc + &x)
    }
}
