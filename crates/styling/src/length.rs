use crate::{AttributeGetter, PreBaseState, Style, StyleBaseState};
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

macro_rules! style_impl {
    ($output:ident) => {
        impl Style<$output<AttributeGetter<Length>>> {
            pub fn px(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Px(num))
            }

            pub fn cm(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Cm(num))
            }

            pub fn percent(self, num: u8) -> Style<$output<()>> {
                debug_assert!(num <= 100, "percent number should be from 0 to 100");
                self.base(Length::Percent(num))
            }

            pub fn mm(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Mm(num))
            }
            pub fn inch(self, num: u8) -> Style<$output<()>> {
                self.base(Length::In(num))
            }
            pub fn pt(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Pt(num))
            }
            pub fn pc(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Pc(num))
            }
            pub fn em(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Em(num))
            }
            pub fn ex(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Ex(num))
            }
            pub fn ch(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Ch(num))
            }
            pub fn rem_(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Rem(num))
            }
            pub fn vw(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Vw(num))
            }
            pub fn vh(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Vh(num))
            }
            pub fn vmin(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Vmin(num))
            }
            pub fn vmax(self, num: u8) -> Style<$output<()>> {
                self.base(Length::Vmax(num))
            }
        }
    };
}

style_impl!(StyleBaseState);
