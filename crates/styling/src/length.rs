use super::{MedAttribute, Style};
use std::fmt::Display;

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

    pub fn mm(self, num: u8) -> Style {
        self.inner(Length::Mm(num))
    }
    pub fn inch(self, num: u8) -> Style {
        self.inner(Length::In(num))
    }
    pub fn pt(self, num: u8) -> Style {
        self.inner(Length::Pt(num))
    }
    pub fn pc(self, num: u8) -> Style {
        self.inner(Length::Pc(num))
    }
    pub fn em(self, num: u8) -> Style {
        self.inner(Length::Em(num))
    }
    pub fn ex(self, num: u8) -> Style {
        self.inner(Length::Ex(num))
    }
    pub fn ch(self, num: u8) -> Style {
        self.inner(Length::Ch(num))
    }
    pub fn rem(self, num: u8) -> Style {
        self.inner(Length::Rem(num))
    }
    pub fn vw(self, num: u8) -> Style {
        self.inner(Length::Vw(num))
    }
    pub fn vh(self, num: u8) -> Style {
        self.inner(Length::Vh(num))
    }
    pub fn vmin(self, num: u8) -> Style {
        self.inner(Length::Vmin(num))
    }
    pub fn vmax(self, num: u8) -> Style {
        self.inner(Length::Vmax(num))
    }
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
