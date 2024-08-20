use std::{collections::HashSet, fmt::Display};

mod color;

pub struct Style(HashSet<CssAttribute>);

pub fn styling() -> Style {
    Style(HashSet::new())
}

impl Style {
    pub fn fontsize(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::FontSize),
        }
    }

    pub fn margin(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Margin),
        }
    }

    pub fn padding(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Padding),
        }
    }

    pub fn bottom(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Bottom),
        }
    }

    pub fn height(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Height),
        }
    }

    pub fn width(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Width),
        }
    }

    pub fn position(self) -> MedAttribute<CssPosition> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Position),
        }
    }

    pub fn background_color(self) -> MedAttribute<color::Color> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::BackgroundColor),
        }
    }
}

pub struct MedAttribute<T> {
    style: Style,
    fun: Box<dyn Fn(T) -> CssAttribute>,
}

impl<T> MedAttribute<T> {
    fn inner(self, position: T) -> Style {
        let Self { mut style, fun } = self;
        let attr = fun(position);
        let success = style.0.insert(attr);
        debug_assert!(success, "value already exists");
        style
    }
}

#[derive(Hash, Eq, PartialEq)]
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

impl MedAttribute<Length> {
    pub fn px(self, num: u8) -> Style {
        self.inner(Length::Px(num))
    }

    pub fn cm(self, num: u8) -> Style {
        self.inner(Length::Cm(num))
    }

    pub fn percent(self, num: u8) -> Style {
        debug_assert!(num <= 100, "percent number should be from 0 to 100");
        self.inner(Length::Percent(num))
    }
}

impl MedAttribute<CssPosition> {
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

#[derive(Hash, Eq, PartialEq)]
pub enum CssAttribute {
    FontSize(Length),
    Position(CssPosition),
    BackgroundColor(color::Color),
    Top(Length),
    Bottom(Length),
    Right(Length),
    Left(Length),
    Height(Length),
    Width(Length),
    Margin(Length),
    Padding(Length),
}

#[derive(Hash, Eq, PartialEq)]
pub enum CssPosition {
    Static,
    Relative,
    Fixed,
    Absolute,
    Sticky,
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Cm(num) => write!(f, "{num}cm"),
            Length::Mm(num) => write!(f, "{num}mm"),
            Length::In(num) => write!(f, "{num}in"),
            Length::Px(num) => write!(f, "{num}px"),
            Length::Pt(num) => write!(f, "{num}pt"),
            Length::Pc(num) => write!(f, "{num}pc"),
            Length::Em(num) => write!(f, "{num}em"),
            Length::Ex(num) => write!(f, "{num}ex"),
            Length::Ch(num) => write!(f, "{num}ch"),
            Length::Rem(num) => write!(f, "{num}rem"),
            Length::Vw(num) => write!(f, "{num}vw"),
            Length::Vh(num) => write!(f, "{num}vh"),
            Length::Vmin(num) => write!(f, "{num}vmin"),
            Length::Vmax(num) => write!(f, "{num}vmax"),
            Length::Percent(num) => write!(f, "{num}%"),
        }
    }
}

impl Display for CssPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            CssPosition::Static => "static",
            CssPosition::Relative => "relative",
            CssPosition::Fixed => "fixed",
            CssPosition::Absolute => "absolute",
            CssPosition::Sticky => "sticky",
        };
        write!(f, "position:{result};")
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .map(|x| match x {
                CssAttribute::FontSize(distance) => format!("font-size:{};", distance),
                CssAttribute::Position(position) => position.to_string(),
                CssAttribute::BackgroundColor(color) => {
                    format!("background-color:{};", color)
                }
                CssAttribute::Top(distance) => format!("top:{};", distance),
                CssAttribute::Bottom(distance) => format!("bottom:{};", distance),
                CssAttribute::Right(distance) => format!("right:{};", distance),
                CssAttribute::Left(distance) => format!("left:{};", distance),
                CssAttribute::Height(distance) => format!("height:{};", distance),
                CssAttribute::Width(distance) => format!("width:{};", distance),
                CssAttribute::Margin(distance) => format!("margin:{};", distance),
                CssAttribute::Padding(distance) => format!("padding:{};", distance),
            })
            .fold(String::new(), move |acc, x| acc + &x);
        write!(f, "{}", result)
    }
}
