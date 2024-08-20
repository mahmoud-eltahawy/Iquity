use std::collections::HashSet;

mod color;

pub struct Style(HashSet<CssAttribute>);

pub fn styling() -> Style {
    Style(HashSet::new())
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

    pub fn background_color(self) -> MedAttribute<color::Color> {
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

impl<T> MedAttribute<T> {
    fn inner(self, position: T) -> Style {
        let Self { mut style, fun } = self;
        let attr = fun(position);
        let success = style.0.insert(attr);
        debug_assert!(success, "value already exists");
        style
    }
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

#[derive(Hash, Eq, PartialEq)]
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
