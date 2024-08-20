use super::{CssAttribute, MedAttribute, Style};
use crate::color::Color;
use std::fmt::Display;

pub struct MedBackground {
    pub(crate) style: Style,
}

impl MedBackground {
    pub(crate) fn med_attr<T>(self, fun: Box<dyn FnOnce(T) -> CssAttribute>) -> MedAttribute<T> {
        let Self { style } = self;
        MedAttribute { core: style, fun }
    }

    pub fn color(self) -> MedAttribute<Color> {
        self.med_attr(Box::new(CssAttribute::BackgroundColor))
    }

    pub fn image(self, source: String) -> Style {
        let Self { mut style } = self;
        style.0.insert(CssAttribute::BackgroundImage(source));
        style
    }

    pub fn repeat(self) -> MedAttribute<Repeat> {
        self.med_attr(Box::new(CssAttribute::BackgroundRepeat))
    }

    pub fn attachment(self) -> MedAttribute<Attachment> {
        self.med_attr(Box::new(CssAttribute::BackgroundAttachment))
    }

    pub fn position(self) -> MedPosition {
        let Self { style } = self;
        MedPosition { style }
    }
}

pub struct MedPosition {
    style: Style,
}

impl MedPosition {
    fn inner(self, position: PositionX) -> MedPositionX {
        let Self { style } = self;
        MedPositionX {
            style,
            left: position,
        }
    }

    pub fn left(self) -> MedPositionX {
        self.inner(PositionX::Left)
    }

    pub fn right(self) -> MedPositionX {
        self.inner(PositionX::Right)
    }

    pub fn center(self) -> MedPositionX {
        self.inner(PositionX::Center)
    }
}

pub struct MedPositionX {
    style: Style,
    left: PositionX,
}

impl MedPositionX {
    fn inner(self, position: PositionY) -> Style {
        let Self { mut style, left } = self;
        style
            .0
            .insert(CssAttribute::BackgroundPosition(DuetPosition(
                left, position,
            )));
        style
    }

    pub fn top(self) -> Style {
        self.inner(PositionY::Top)
    }

    pub fn bottom(self) -> Style {
        self.inner(PositionY::Bottom)
    }

    pub fn center(self) -> Style {
        self.inner(PositionY::Center)
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum PositionY {
    Top,
    Bottom,
    Center,
}

#[derive(Hash, Eq, PartialEq)]
pub enum PositionX {
    Left,
    Right,
    Center,
}

impl Display for PositionX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            PositionX::Right => "right",
            PositionX::Left => "left",
            PositionX::Center => "center",
        };
        write!(f, "{result}")
    }
}

impl Display for PositionY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            PositionY::Top => "top",
            PositionY::Bottom => "bottom",
            PositionY::Center => "center",
        };
        write!(f, "{result}")
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct DuetPosition(PositionX, PositionY);

impl Display for DuetPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DuetPosition(x, y) = self;
        write!(f, "{x} {y}",)
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum Repeat {
    X,
    Y,
    None,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Attachment {
    Fixed,
    Scroll,
}

impl MedAttribute<Attachment> {
    pub fn scroll(self) -> Style {
        self.inner(Attachment::Scroll)
    }

    pub fn fixed(self) -> Style {
        self.inner(Attachment::Fixed)
    }
}

impl Display for Attachment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Attachment::Fixed => "fixed",
            Attachment::Scroll => "scroll",
        };
        write!(f, "{result}")
    }
}

impl MedAttribute<Repeat> {
    pub fn x(self) -> Style {
        self.inner(Repeat::X)
    }
    pub fn y(self) -> Style {
        self.inner(Repeat::Y)
    }
    pub fn none(self) -> Style {
        self.inner(Repeat::None)
    }
}

impl Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Repeat::X => "repeat-x",
            Repeat::Y => "repeat-y",
            Repeat::None => "no-repeat",
        };
        write!(f, "{result}")
    }
}
