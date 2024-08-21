use crate::{PreState, PreStyleBase, StyleBaseState};

use super::Style;
use std::fmt::Display;

#[derive(Hash, Eq, PartialEq)]
pub enum CssPosition {
    Static,
    Relative,
    Fixed,
    Absolute,
    Sticky,
}

impl Style<PreStyleBase<CssPosition>> {
    pub fn fixed(self) -> Style<StyleBaseState> {
        self.base(CssPosition::Fixed)
    }

    pub fn relative(self) -> Style<StyleBaseState> {
        self.base(CssPosition::Relative)
    }

    pub fn static_(self) -> Style<StyleBaseState> {
        self.base(CssPosition::Static)
    }

    pub fn absolute(self) -> Style<StyleBaseState> {
        self.base(CssPosition::Absolute)
    }

    pub fn sticky(self) -> Style<StyleBaseState> {
        self.base(CssPosition::Sticky)
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
