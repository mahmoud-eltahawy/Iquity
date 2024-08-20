use super::{CssAttribute, MedAttribute, Style};
use crate::color::Color;
use std::fmt::Display;

pub struct MedBackground {
    pub(crate) style: Style,
}

pub struct MedPosition {
    style: Style,
}

pub struct MedPositionX {
    style: Style,
    x: PositionX,
}

#[derive(Hash, Eq, PartialEq)]
pub struct DuetPosition(PositionX, PositionY);

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

impl MedBackground {
    pub fn med_attr<T>(self, fun: Box<dyn FnOnce(T) -> CssAttribute>) -> MedAttribute<T> {
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

    pub fn position_x(self) -> MedAttribute<PositionX> {
        let Self { style } = self;
        MedAttribute {
            core: style,
            fun: Box::new(CssAttribute::BackgroundPositionX),
        }
    }

    pub fn position_y(self) -> MedAttribute<PositionY> {
        let Self { style } = self;
        MedAttribute {
            core: style,
            fun: Box::new(CssAttribute::BackgroundPositionY),
        }
    }
}

impl MedAttribute<PositionX> {
    pub fn left(self) -> Style {
        self.inner(PositionX::Left)
    }

    pub fn right(self) -> Style {
        self.inner(PositionX::Right)
    }

    pub fn center(self) -> Style {
        self.inner(PositionX::Center)
    }
}

impl MedAttribute<PositionY> {
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

impl MedPosition {
    fn inner(self, x: PositionX) -> MedPositionX {
        let Self { style } = self;
        MedPositionX { style, x }
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

impl MedPositionX {
    fn inner(self, position: PositionY) -> Style {
        let Self { mut style, x: left } = self;
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
impl Display for DuetPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DuetPosition(x, y) = self;
        write!(f, "{x} {y}",)
    }
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
