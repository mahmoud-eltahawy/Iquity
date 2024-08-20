use super::{MedAttribute, Style};
use std::fmt::Display;

#[derive(Hash, Eq, PartialEq)]
pub enum CssPosition {
    Static,
    Relative,
    Fixed,
    Absolute,
    Sticky,
}

impl MedAttribute<CssPosition> {
    pub fn fixed(self) -> Style {
        self.inner(CssPosition::Fixed)
    }

    pub fn relative(self) -> Style {
        self.inner(CssPosition::Relative)
    }

    pub fn static_(self) -> Style {
        self.inner(CssPosition::Static)
    }

    pub fn absolute(self) -> Style {
        self.inner(CssPosition::Absolute)
    }

    pub fn sticky(self) -> Style {
        self.inner(CssPosition::Sticky)
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
