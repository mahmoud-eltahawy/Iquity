use super::{MedAttribute, Style};
use std::fmt::Display;

#[derive(Hash, Eq, PartialEq)]
pub enum Position {
    Static,
    Relative,
    Fixed,
    Absolute,
    Sticky,
}

impl MedAttribute<Position> {
    pub fn fixed(self) -> Style {
        self.inner(Position::Fixed)
    }

    pub fn relative(self) -> Style {
        self.inner(Position::Relative)
    }

    pub fn static_(self) -> Style {
        self.inner(Position::Static)
    }

    pub fn absolute(self) -> Style {
        self.inner(Position::Absolute)
    }

    pub fn sticky(self) -> Style {
        self.inner(Position::Sticky)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Position::Static => "static",
            Position::Relative => "relative",
            Position::Fixed => "fixed",
            Position::Absolute => "absolute",
            Position::Sticky => "sticky",
        };
        write!(f, "position:{result};")
    }
}
